/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // Remove hyphens from the ISBN string
    let isbn = isbn.replace("-", "");

    // Check if the length of the ISBN is exactly 10 characters
    if isbn.len() != 10 {
        return false;
    }

    // Calculate the checksum using the ISBN-10 formula
    let mut sum = 0;
    for (i, c) in isbn.chars().enumerate() {
        let digit = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'X' if i == 9 => 10, // 'X' is only valid as the last character
            _ => return false,   // Invalid character
        };
        sum += digit * (10 - i as u32);
    }

    // The ISBN is valid if the checksum is divisible by 11
    sum % 11 == 0
}