pub fn private_key(p: u64) -> u64 {
    number_between(2, p - 1)
}

fn number_between(min: u64, max: u64) -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
