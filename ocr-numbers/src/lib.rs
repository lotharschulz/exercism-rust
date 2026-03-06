#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    // Split the input string into lines
    let lines: Vec<&str> = input.lines().collect();
    // Check if the number of lines is valid (must be multiple of 4, since each digit row is 4 lines high)
    if lines.is_empty() || !lines.len().is_multiple_of(4) {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    // Process each group of 4 lines (one row of digits)
    let rows: Vec<String> = lines
        .chunks(4) // Split lines into chunks of 4
        .map(|chunk| {
            // Take the first 3 lines (the digit patterns; the 4th is always blank)
            let height_lines = &chunk[0..3];
            // Get the length of the first line
            let len = height_lines[0].len();
            // Validate: all lines must have the same length, and length must be multiple of 3 (each digit is 3 columns wide)
            if !height_lines.iter().all(|l| l.len() == len) || len % 3 != 0 {
                return Err(Error::InvalidColumnCount(len));
            }
            // Calculate number of digits in this row
            let num_digits = len / 3;
            // Build the string of recognized digits for this row
            let digits: String = (0..num_digits)
                .map(|startup_position| {
                    // Extract the 3x3 cell for this digit position
                    let cell: Vec<&str> = height_lines
                        .iter()
                        .map(|l| &l[startup_position * 3..startup_position * 3 + 3])
                        .collect();
                    // Recognize the digit from the cell pattern
                    recognize(&cell)
                })
                .collect();
            Ok(digits)
        })
        // Collect results, propagating any errors
        .collect::<Result<Vec<_>, _>>()?;

    // Join the rows with commas and return
    Ok(rows.join(","))
}

fn recognize(cell: &[&str]) -> char {
    // Match the 3x3 cell pattern to a digit (0-9) or '?' for unrecognized
    match cell {
        [" _ ", "| |", "|_|"] => '0',
        ["   ", "  |", "  |"] => '1',
        [" _ ", " _|", "|_ "] => '2',
        [" _ ", " _|", " _|"] => '3',
        ["   ", "|_|", "  |"] => '4',
        [" _ ", "|_ ", " _|"] => '5',
        [" _ ", "|_ ", "|_|"] => '6',
        [" _ ", "  |", "  |"] => '7',
        [" _ ", "|_|", "|_|"] => '8',
        [" _ ", "|_|", " _|"] => '9',
        _ => '?',
    }
}
