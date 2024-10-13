pub fn nth(n: u32) -> u32 {
    let &result = (2..)
        .filter(|&i| is_prime(i))
        .skip(n as usize)
        .take(1)
        .collect::<Vec<u32>>()
        .first()
        .unwrap();

    result
}

fn is_prime(n: u32) -> bool {
    (2..=f32::sqrt(n as f32) as u32)
        .all(|i| n % i != 0)
}
