pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string()], // return a vector with an empty string if length is 0
        l if l > digits.len() => vec![], // eeturn an empty vector if length is greater than the number of digits
        _ => digits
            .chars() // convert the string to a vector of characters
            .collect::<Vec<_>>()
            .windows(len) // create sliding windows of the specified length
            .map(|window| window.iter().collect()) // convert each window to a string
            .collect(), // collect all strings into a vector
    }
}
