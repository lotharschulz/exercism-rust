#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // input validation
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }

    // Check if any digit in the number vector is smaller than the from_base
    if number.iter().any(|&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(
            *number.iter().find(|&&digit| digit >= from_base).unwrap(),
        ));
    }

    // Initialize decimal_value to store the intermediate decimal representation
    let mut decimal_value = 0;
    // Iterate over each digit in the input number
    for &digit in number {
        // Check if the digit is valid in the given from_base
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        // Convert the number to its decimal representation
        decimal_value = decimal_value * from_base + digit;
    }
    // If the decimal value is zero, return a vector containing zero
    if decimal_value == 0 {
        return Ok(vec![0]);
    }
    // Initialize result vector to store the final converted number
    let mut result = Vec::new();
    // Convert the decimal value to the desired to_base
    let mut current = decimal_value;
    while current > 0 {
        // Append the remainder of current divided by to_base to the result vector
        result.push(current % to_base);
        // Update current by dividing it by to_base
        current /= to_base;
    }
    // Reverse the result vector to get the correct order of digits
    result.reverse();
    Ok(result)
}
