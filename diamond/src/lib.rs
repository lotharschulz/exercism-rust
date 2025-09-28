#[must_use]
pub fn get_diamond(c: char) -> Vec<String> {
    let size = (c as u8 - b'A') as usize;

    let ascending = (0..=size).map(|i| build_line(i, size));
    let descending = (0..size).rev().map(|i| build_line(i, size));

    ascending.chain(descending).collect()
}

#[allow(clippy::cast_possible_truncation)]
fn build_line(row: usize, max_row: usize) -> String {
    let width = 2 * max_row + 1;
    let letter = (b'A' + row as u8) as char;
    let outer_spaces = max_row - row;

    if row == 0 {
        format!("{letter:^width$}")
    } else {
        let inner_spaces = 2 * row - 1;
        format!(
            "{}{}{}{}{}",
            " ".repeat(outer_spaces),
            letter,
            " ".repeat(inner_spaces),
            letter,
            " ".repeat(outer_spaces)
        )
    }
}
