/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Validate that a and 26 are coprime
    if greatest_common_divisor(a.rem_euclid(26), 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let a = a.rem_euclid(26);
    let b = b.rem_euclid(26);

    // Closure to encode a single character; returns None for ignored characters
    let encode_char = |ch: char| -> Option<char> {
        if ch.is_ascii_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            let x = (lower as u8 - b'a') as i32;
            let y = (a * x + b).rem_euclid(26) as u8;
            Some((b'a' + y) as char)
        } else if ch.is_ascii_digit() {
            Some(ch)
        } else {
            None
        }
    };

    // Build a flat string of encoded chars (letters transformed, digits preserved)
    let flat: Vec<char> = plaintext.chars().filter_map(encode_char).collect();

    // Group into chunks of 5 characters separated by spaces
    let grouped = flat
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(grouped)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Validate that a and 26 are coprime
    if greatest_common_divisor(a.rem_euclid(26), 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let a = a.rem_euclid(26);
    let b = b.rem_euclid(26);
    let a_inv = mod_inverse(a, 26).expect("a and 26 are coprime, inverse must exist");

    // Closure to decode a single character; returns None for ignored characters
    let decode_char = |ch: char| -> Option<char> {
        if ch.is_ascii_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            let y = (lower as u8 - b'a') as i32;
            let x = (a_inv * (y - b)).rem_euclid(26) as u8;
            Some((b'a' + x) as char)
        } else if ch.is_ascii_digit() {
            Some(ch)
        } else {
            None
        }
    };

    let decoded: String = ciphertext.chars().filter_map(decode_char).collect();
    Ok(decoded)
}

// Compute greatest common divisor using Euclidean algorithm (non-negative result)
fn greatest_common_divisor(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

// Compute modular multiplicative inverse of a modulo m, if it exists
fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    // Extended Euclidean algorithm
    let (mut t, mut new_t) = (0i32, 1i32);
    let (mut r, mut new_r) = (m, a.rem_euclid(m));

    while new_r != 0 {
        let q = r / new_r;
        let tmp_t = t - q * new_t;
        t = new_t;
        new_t = tmp_t;

        let tmp_r = r - q * new_r;
        r = new_r;
        new_r = tmp_r;
    }

    if r > 1 {
        return None;
    }
    if t < 0 {
        t += m;
    }
    Some(t)
}
