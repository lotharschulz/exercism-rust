pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let exponent = num_str.len() as u32;

    let sum: u64 = num_str.chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .map(|x| x.pow(exponent))
        .sum();

    sum == num as u64
}
