use rand::Rng;

/// Validates that a key contains only lowercase ASCII letters and is not empty.
fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

/// Shifts a character by the given amount, wrapping around the alphabet.
///
/// # Arguments
/// * `c` - The character to shift
/// * `shift` - The shift amount (0-25)
/// * `encode` - If true, shifts forward; if false, shifts backward (decode)
fn shift_char(c: char, shift: u8, encode: bool) -> char {
    const BASE: u8 = b'a';
    const ALPHABET_SIZE: u8 = 26;

    let c_pos = (c as u8) - BASE;
    let new_pos = if encode {
        (c_pos + shift) % ALPHABET_SIZE
    } else {
        (c_pos + ALPHABET_SIZE - shift) % ALPHABET_SIZE
    };
    (BASE + new_pos) as char
}

/// Applies a cipher operation (encode or decode) using the given key.
///
/// # How it works
/// 1. Validates the key contains only lowercase letters and is not empty
/// 2. Pairs each character in the input string with the corresponding key character
/// 3. If the key is shorter than the string, it cycles/repeats (using `.cycle()`)
/// 4. For each pair, converts the key character to a shift amount (a=0, b=1, c=2, etc.)
/// 5. Applies the shift to the input character using `shift_char()`
/// 6. Collects all shifted characters into a new String
///
/// # Arguments
/// * `key` - The cipher key (lowercase letters only)
/// * `s` - The string to encode or decode
/// * `encode` - If true, encodes; if false, decodes
///
/// # Returns
/// * `Some(String)` - The transformed text if the key is valid
/// * `None` - If the key is invalid (empty or contains non-lowercase letters)
///
/// # Example
/// * `apply_cipher("abc", "hello", true)` → `Some("hfnlp")`
/// * Key cycles: h+a, e+b, l+c, l+a, o+b
fn apply_cipher(key: &str, s: &str, encode: bool) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(text_char, key_char)| {
                let shift = (key_char as u8) - b'a';
                shift_char(text_char, shift, encode)
            })
            .collect(),
    )
}

/// Encodes plaintext using a Vigenère cipher with the given key.
///
/// # Returns
/// * `Some(String)` - The encoded text if the key is valid
/// * `None` - If the key is empty or contains non-lowercase letters
pub fn encode(key: &str, s: &str) -> Option<String> {
    apply_cipher(key, s, true)
}

/// Decodes ciphertext using a Vigenère cipher with the given key.
///
/// # Returns
/// * `Some(String)` - The decoded text if the key is valid
/// * `None` - If the key is empty or contains non-lowercase letters
pub fn decode(key: &str, s: &str) -> Option<String> {
    apply_cipher(key, s, false)
}

/// Generates a random key of 100 lowercase letters and encodes the plaintext.
///
/// # Returns
/// A tuple containing:
/// * The randomly generated key (100 characters)
/// * The encoded text
pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();
    let key: String = (0..100)
        .map(|_| rng.random_range(b'a'..=b'z') as char)
        .collect();
    let encoded = encode(&key, s).expect("Generated key should always be valid");
    (key, encoded)
}
