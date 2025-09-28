#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/// Finds the largest product of consecutive digits in a string
///
/// # Arguments
/// * `string_digits` - A string containing only digit characters
/// * `span` - The number of consecutive digits to multiply
///
/// # Errors
///
/// Returns an error if:
/// * `Error::SpanTooLong` - The span exceeds the string length
/// * `Error::InvalidDigit(char)` - The string contains non-digit characters
///
/// # Examples
/// ```
/// use largest_series_product::lsp;
/// assert_eq!(lsp("63915", 3), Ok(162));
/// ```
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => return Ok(1),
        s if s > string_digits.len() => return Err(Error::SpanTooLong),
        _ => {}
    }

    let digits = parse_digits(string_digits)?;

    digits
        .windows(span)
        .map(calculate_product)
        .max()
        .ok_or(Error::SpanTooLong)
}

/// Parses a string into a vector of u64 digits
fn parse_digits(input: &str) -> Result<Vec<u64>, Error> {
    input
        .chars()
        .map(|c| c.to_digit(10).map(u64::from).ok_or(Error::InvalidDigit(c)))
        .collect()
}

/// Calculates the product of a slice of numbers
fn calculate_product(window: &[u64]) -> u64 {
    window.iter().product()
}
