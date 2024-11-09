// 1) Declare a character stack brack as vector.
// 2) Traverse the string expression.
//     a) push char to stack, if the current character is a starting bracket (‘(‘ or ‘{‘ or ‘[‘).
//     b) pop char from stack, if the current character is a closing bracket (‘)’ or ‘}’ or ‘]’),
//            and if the popped character does not matches the starting bracket: then “not balanced”.
// 3) After complete traversal, if there is some starting bracket left in stack: then “not balanced”.

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => brack.push(c),
            ')' => {
                if brack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if brack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if brack.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }
    brack.is_empty()
}
