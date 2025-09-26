/// "Encipher" with the Atbash cipher.
fn atbash(c: char) -> char {
    if c.is_ascii_lowercase() {
        (b'a' + (b'z' - c as u8)) as char
    } else {
        c
    }
}

pub fn encode(plain: &str) -> String {
    let transformed: String = plain
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                atbash(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .collect();
    let chars: Vec<char> = transformed.chars().collect();
    chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                atbash(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .collect()
}
