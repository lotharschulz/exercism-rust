pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

use std::collections::HashMap;
use std::sync::Arc;

pub struct Forth {
    // Runtime value stack used by all operations.
    stack: Vec<Value>,
    // Storage for user-defined words.
    words: Vec<WordDefinition>,
    // Case-insensitive lookup from word name to index in `words`.
    dictionary: HashMap<String, usize>,
}

#[derive(Clone)]
struct WordDefinition {
    // Compiled operations for the word body.
    // Arc avoids cloning large bodies during calls.
    ops: Arc<[Op]>,
}

#[derive(Clone)]
enum Op {
    // Push literal number.
    Push(Value),
    // Execute one built-in operation.
    Builtin(Builtin),
    // Call a previously defined user word by id.
    Call(usize),
}

#[derive(Clone, Copy)]
enum Builtin {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: Vec::new(),
            dictionary: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        // Forth words are case-insensitive, so normalize once up front.
        let tokens = input
            .split_whitespace()
            .map(|token| token.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let mut index = 0;
        while index < tokens.len() {
            if tokens[index] == ":" {
                // Parse and compile a full definition: `: name ... ;`.
                self.define_word(&tokens, &mut index)?;
            } else {
                // Compile and execute one token immediately.
                let op = self.compile_token(&tokens[index])?;
                self.execute_op(op)?;
                index += 1;
            }
        }

        Ok(())
    }

    fn define_word(&mut self, tokens: &[String], index: &mut usize) -> Result {
        // Skip ':'.
        *index += 1;

        // Next token must be the new word name.
        let Some(name) = tokens.get(*index) else {
            return Err(Error::InvalidWord);
        };
        // Numbers cannot be redefined.
        if parse_number(name).is_some() {
            return Err(Error::InvalidWord);
        }

        *index += 1;
        let mut ops = Vec::new();

        while *index < tokens.len() {
            let token = &tokens[*index];
            if token == ";" {
                // Save compiled body and update dictionary to this newest definition.
                let word_id = self.words.len();
                self.words.push(WordDefinition {
                    ops: Arc::from(ops),
                });
                self.dictionary.insert(name.clone(), word_id);
                *index += 1;
                return Ok(());
            }

            // Compile definition body token-by-token.
            ops.push(self.compile_token(token)?);
            *index += 1;
        }

        // Missing terminating ';'.
        Err(Error::InvalidWord)
    }

    fn compile_token(&self, token: &str) -> std::result::Result<Op, Error> {
        // Numeric literal.
        if let Some(value) = parse_number(token) {
            return Ok(Op::Push(value));
        }

        // User-defined word (resolved to its current id at compile time).
        if let Some(&word_id) = self.dictionary.get(token) {
            return Ok(Op::Call(word_id));
        }

        // Built-in word/operator.
        if let Some(builtin) = parse_builtin(token) {
            return Ok(Op::Builtin(builtin));
        }

        // Not a number, built-in, or known user word.
        Err(Error::UnknownWord)
    }

    fn execute_op(&mut self, op: Op) -> Result {
        match op {
            Op::Push(value) => {
                self.stack.push(value);
                Ok(())
            }
            Op::Builtin(builtin) => self.execute_builtin(builtin),
            Op::Call(word_id) => {
                // Clone the Arc handle so recursive execution doesn't borrow `self.words`.
                let ops = Arc::clone(&self.words[word_id].ops);
                for nested_op in ops.iter().cloned() {
                    self.execute_op(nested_op)?;
                }
                Ok(())
            }
        }
    }

    fn execute_builtin(&mut self, builtin: Builtin) -> Result {
        match builtin {
            Builtin::Add => self.apply_binary_op(|left, right| Ok(left + right))?,
            Builtin::Sub => self.apply_binary_op(|left, right| Ok(left - right))?,
            Builtin::Mul => self.apply_binary_op(|left, right| Ok(left * right))?,
            Builtin::Div => self.apply_binary_op(|left, right| {
                if right == 0 {
                    return Err(Error::DivisionByZero);
                }
                Ok(left / right)
            })?,
            Builtin::Dup => {
                let value = *self.stack.last().ok_or(Error::StackUnderflow)?;
                self.stack.push(value);
            }
            Builtin::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            Builtin::Swap => {
                let len = self.stack.len();
                if len < 2 {
                    return Err(Error::StackUnderflow);
                }
                self.stack.swap(len - 1, len - 2);
            }
            Builtin::Over => {
                let len = self.stack.len();
                if len < 2 {
                    return Err(Error::StackUnderflow);
                }
                let value = self.stack[len - 2];
                self.stack.push(value);
            }
        }

        Ok(())
    }

    fn apply_binary_op<F>(&mut self, op: F) -> Result
    where
        F: FnOnce(Value, Value) -> std::result::Result<Value, Error>,
    {
        // Forth binary operators consume the top two stack values.
        let (left, right) = self.pop_two()?;
        let value = op(left, right)?;
        self.stack.push(value);
        Ok(())
    }

    fn pop_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        // Note order: `right` is the top item, `left` is below it.
        let right = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let left = self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok((left, right))
    }
}

fn parse_number(token: &str) -> Option<Value> {
    // Accept signed integer literals (e.g. `-42`).
    token.parse::<Value>().ok()
}

fn parse_builtin(token: &str) -> Option<Builtin> {
    // Built-ins are matched after lowercasing input.
    match token {
        "+" => Some(Builtin::Add),
        "-" => Some(Builtin::Sub),
        "*" => Some(Builtin::Mul),
        "/" => Some(Builtin::Div),
        "dup" => Some(Builtin::Dup),
        "drop" => Some(Builtin::Drop),
        "swap" => Some(Builtin::Swap),
        "over" => Some(Builtin::Over),
        _ => None,
    }
}
