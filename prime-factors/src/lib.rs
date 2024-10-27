pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut remaining = n;
    let mut factor = 2;

    while remaining > 1 {
        while remaining % factor == 0 {
            result.push(factor);
            remaining %= factor;
        }
        factor += 1;
    }
    result
}
