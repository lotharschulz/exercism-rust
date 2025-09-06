pub fn answer(command: &str) -> Option<i32> {
    // Remove question mark and normalize whitespace
    let command = command.trim_end_matches('?').trim();

    // Check if command starts with "What is"
    if !command.starts_with("What is ") {
        return None;
    }

    // Extract the expression part after "What is "
    let expression = &command[8..]; // "What is ".len() == 8

    // Parse the expression
    parse_expression(expression)
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Operation {
    fn apply(&self, left: i32, right: i32) -> i32 {
        match self {
            Operation::Add => left + right,
            Operation::Subtract => left - right,
            Operation::Multiply => left * right,
            Operation::Divide => left / right,
            Operation::Power => left.pow(right as u32),
        }
    }
}

fn parse_expression(expr: &str) -> Option<i32> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();

    if tokens.is_empty() {
        return None;
    }

    // Handle simple number case
    if tokens.len() == 1 {
        return tokens[0].parse::<i32>().ok();
    }

    // Handle operations - evaluate left to right
    let mut result = tokens[0].parse::<i32>().ok()?;
    let mut i = 1;

    while i < tokens.len() {
        let (operation, tokens_consumed) = parse_operation(&tokens[i..])?;
        i += tokens_consumed;

        let operand = if operation == Operation::Power {
            // For exponentiation, parse the ordinal number (e.g., "5th" -> 5)
            let power_str = tokens.get(i)?;
            parse_ordinal(power_str)?
        } else {
            tokens.get(i)?.parse::<i32>().ok()?
        };

        result = operation.apply(result, operand);

        // Skip past the operand (and "power" for exponentiation)
        i += if operation == Operation::Power { 2 } else { 1 };
    }

    Some(result)
}

fn parse_operation(tokens: &[&str]) -> Option<(Operation, usize)> {
    match *(tokens.first()?) {
        "plus" => Some((Operation::Add, 1)),
        "minus" => Some((Operation::Subtract, 1)),
        "multiplied" => {
            if tokens.get(1) == Some(&"by") {
                Some((Operation::Multiply, 2))
            } else {
                None
            }
        }
        "divided" => {
            if tokens.get(1) == Some(&"by") {
                Some((Operation::Divide, 2))
            } else {
                None
            }
        }
        "raised" => {
            if tokens.get(1) == Some(&"to")
                && tokens.get(2) == Some(&"the")
                && tokens.len() > 4
                && tokens.get(4) == Some(&"power")
            {
                Some((Operation::Power, 3)) // Skip "raised", "to", "the"
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_ordinal(ordinal: &str) -> Option<i32> {
    // Remove ordinal suffix (st, nd, rd, th) and parse the number
    let num_str = if ordinal.ends_with("st")
        || ordinal.ends_with("nd")
        || ordinal.ends_with("rd")
        || ordinal.ends_with("th")
    {
        &ordinal[..ordinal.len() - 2]
    } else {
        ordinal
    };

    num_str.parse::<i32>().ok()
}
