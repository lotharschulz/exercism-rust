/// Rotates each alphabetic character in the input string by the specified key.
///
/// This implements a Caesar cipher (rotational cipher) that:
/// - Shifts lowercase letters within 'a'-'z'
/// - Shifts uppercase letters within 'A'-'Z'  
/// - Preserves all non-alphabetic characters (spaces, numbers, punctuation)
/// - Uses modular arithmetic so key values of 0 and 26 produce identical output
///
/// # Arguments
/// * `input` - The text to transform
/// * `key` - The number of positions to shift (0-25, though any u8 value works due to modulo)
///
/// # Examples
/// ```
/// # use rotational_cipher::rotate;
/// assert_eq!(rotate("abc", 1), "bcd");
/// assert_eq!(rotate("Hello, World!", 13), "Uryyb, Jbeyq!");
/// ```
pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(|c| rotate_char(c, key)).collect()
}

/// Rotates a single character by the specified key if it's alphabetic.
/// Non-alphabetic characters are returned unchanged.
fn rotate_char(c: char, key: u8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let offset = (c as u8 - base + key) % 26;
    (base + offset) as char
}
