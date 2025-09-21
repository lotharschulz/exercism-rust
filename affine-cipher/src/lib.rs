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
    if gcd(a.rem_euclid(26), 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let a = a.rem_euclid(26);
    let b = b.rem_euclid(26);

    // Build encoded characters (letters transformed, digits preserved),
    // then group into chunks of 5 characters separated by spaces
    let mut out = String::new();
    let mut count = 0usize; // number of emitted non-space chars

    for ch in plaintext.chars() {
        if ch.is_ascii_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            let x = (lower as u8 - b'a') as i32;
            let y = (a * x + b).rem_euclid(26) as u8;
            let enc = (b'a' + y) as char;
            if count > 0 && count % 5 == 0 {
                out.push(' ');
            }
            out.push(enc);
            count += 1;
        } else if ch.is_ascii_digit() {
            if count > 0 && count % 5 == 0 {
                out.push(' ');
            }
            out.push(ch);
            count += 1;
        } else {
            // ignore punctuation, spaces, etc.
        }
    }

    Ok(out)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Validate that a and 26 are coprime
    if gcd(a.rem_euclid(26), 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let a = a.rem_euclid(26);
    let b = b.rem_euclid(26);
    let a_inv = mod_inverse(a, 26).expect("a and 26 are coprime, inverse must exist");

    let mut out = String::new();
    for ch in ciphertext.chars() {
        if ch.is_ascii_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            let y = (lower as u8 - b'a') as i32;
            let x = (a_inv * (y - b)).rem_euclid(26) as u8;
            let dec = (b'a' + x) as char;
            out.push(dec);
        } else if ch.is_ascii_digit() {
            out.push(ch);
        } else {
            // ignore spaces, punctuation, etc.
        }
    }

    Ok(out)
}

// Compute greatest common divisor using Euclidean algorithm (non-negative result)
fn gcd(mut a: i32, mut b: i32) -> i32 {
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

    if r > 1 { return None; }
    if t < 0 { t += m; }
    Some(t)
}
