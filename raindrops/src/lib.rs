pub fn raindrops(input: u32) -> String {
    let mut result = String::new();

    if input % 3 == 0 {
        result += "Pling";
    }
    if input % 5 == 0 {
        result += "Plang";
    }
    if input % 7 == 0 {
        result += "Plong";
    }

    if result.is_empty() {
        result = input.to_string();
    }
    result
}

/*

modulo ifs above in pattern matching

match (input % 3, input % 5, input % 7) {
    (0, 0, 0) => "PlingPlangPlong".to_string(),
    (0, 0, _) => "PlingPlang".to_string(),
    (0, _, 0) => "PlingPlong".to_string(),
    (_, 0, 0) => "PlangPlong".to_string(),
    (0, _, _) => "Pling".to_string(),
    (_, 0, _) => "Plang".to_string(),
    (_, _, 0) => "Plong".to_string(),
    _ => input.to_string(),
}

*/
