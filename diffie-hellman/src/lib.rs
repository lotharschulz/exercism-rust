use std::time::{SystemTime, UNIX_EPOCH};

pub fn private_key(p: u64) -> u64 {
    // Get the current time in nanoseconds since the UNIX epoch
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = duration.as_nanos() as u64;

    // Use a simple linear congruential generator (LCG) for pseudo-random number generation
    // Constants are from Numerical Recipes
    let a: u64 = 1664525;
    let c: u64 = 1013904223;
    let m: u64 = 2u64.pow(32);

    let random_number = (a.wrapping_mul(nanos).wrapping_add(c)) % m;

    2 + (random_number % (p - 2))
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
