pub fn egg_count(display_value: u32) -> usize {
    // Count the number of 1 bits in the binary representation of display_value
    let ones_count = display_value.count_ones();
    // Try to convert the count to usize, return 0 if conversion fails
    ones_count.try_into().unwrap_or(0)
}
