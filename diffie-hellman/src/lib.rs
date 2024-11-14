use std::time::{SystemTime, UNIX_EPOCH};

/// generates a pseudo-random private key for cryptographic purposes
pub fn private_key(p: u64) -> u64 {
    // get the current time in nanoseconds since the UNIX epoch
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = duration.as_nanos() as u64;

    // use a simple linear congruential generator (LCG) for pseudo-random number generation
    // constants are from Numerical Recipes
    let a: u64 = 1664525; // multiplier constant
    let c: u64 = 1013904223; // increment constant
    let m: u64 = 2u64.pow(32); // modulus constant (2^32)

    // calculate the pseudo-random number using the LCG formula
    let random_number = (a.wrapping_mul(nanos).wrapping_add(c)) % m;

    // ensure the random number is within the range [2, p - 2]
    2 + (random_number % (p - 2))
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

/// function calculates (base^exp) % modulus efficiently, called modular exponentiation
/// even for large values of exp,
/// by using an iterative method that reduces the number of multiplications required.
/// makes it suitable for use in cryptographic applications
fn modular_exponentiation(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus;
    }
    result
}
