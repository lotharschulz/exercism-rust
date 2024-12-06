pub fn is_valid(code: &str) -> bool {
    // Remove all spaces from the input string
    let code = code.replace(" ", "");

    // Check if the length is less than or equal to 1 or if any character is not a digit
    if code.len() <= 1 || !code.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    // Calculate the Luhn checksum
    let sum = code
        .chars()
        .rev() // Reverse the characters
        .enumerate() // Enumerate to get index and character
        .map(|(i, c)| {
            let mut n = c.to_digit(10).unwrap(); // Convert character to digit
            if i % 2 == 1 {
                // Double every second digit
                n *= 2;
                if n > 9 {
                    // Subtract 9 if the result is greater than 9
                    n -= 9;
                }
            }
            n
        })
        .sum::<u32>(); // Sum all the digits

    // Check if the sum is divisible by 10
    sum % 10 == 0
}
