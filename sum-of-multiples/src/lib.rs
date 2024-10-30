pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    for n in 1..limit {
        for &factor in factors {
            if factor != 0 && n % factor == 0 {
                multiples.push(n);
            }
        }
    }
    multiples.sort_unstable();
    multiples.dedup();
    multiples.iter().sum()
}
