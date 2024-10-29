pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut factor = 2;
    while n > 1 {
        while n % factor == 0 {
            factors.push(factor);
            n /= factor;
        }
        factor += 1;
    }
    factors
}
