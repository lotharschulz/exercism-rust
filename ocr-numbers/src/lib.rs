// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() || !lines.len().is_multiple_of(4) {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    let rows: Vec<String> = lines
        .chunks(4)
        .map(|chunk| {
            let height_lines = &chunk[0..3];
            let len = height_lines[0].len();
            if !height_lines.iter().all(|l| l.len() == len) || len % 3 != 0 {
                return Err(Error::InvalidColumnCount(len));
            }
            let num_digits = len / 3;
            let digits: String = (0..num_digits)
                .map(|j| {
                    let cell: Vec<&str> =
                        height_lines.iter().map(|l| &l[j * 3..j * 3 + 3]).collect();
                    recognize(&cell)
                })
                .collect();
            Ok(digits)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows.join(","))
}

fn recognize(cell: &[&str]) -> char {
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
