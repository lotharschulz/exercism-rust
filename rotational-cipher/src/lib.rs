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


/// Explanation of &str and String usage:
/// ```rust
/// pub fn rotate(input: &str, key: u8) -> String
/// ```
/// &str (String Slice)
/// - Borrowed reference to string data stored elsewhere
/// - Immutable and fixed-size view into a string
/// - Doesn't own the data it points to
/// - Very lightweight to pass around (just a pointer + length)
/// - Why used for input:
///   - Accepts any string type: String, &String, &str, or string literals ("hello")
///   - No ownership transfer needed - we just read the data
///   - Efficient - no copying or allocation
/// String (Owned String)
/// - Owned, growable, heap-allocated string
/// - Can be modified and resized
/// - Responsible for freeing its memory when dropped
/// - Heavier weight than &str
/// - Why used for return value:
///   - We're creating new data (the rotated text)
///   - Caller needs to own this new string
///   - The transformed characters are built using .collect() which creates a new String
/// In This Code
/// ```rust
/// input.chars().map(|c| rotate_char(c, key)).collect()
/// ```
/// input is &str - we just iterate over it without taking ownership
/// .collect() builds a new String from the mapped characters
/// Returning String gives the caller ownership of this newly created data
/// This pattern (&str → String) is idiomatic Rust for string transformation functions.
