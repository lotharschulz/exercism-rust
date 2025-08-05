pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, &row_str)| annotate_row(garden, row, row_str))
        .collect()
}

fn annotate_row(garden: &[&str], row: usize, row_str: &str) -> String {
    row_str
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(col, &byte)| annotate_cell(garden, row, col, byte as char))
        .collect()
}

fn annotate_cell(garden: &[&str], row: usize, col: usize, cell: char) -> char {
    match cell {
        '*' => '*',
        _ => count_adjacent_flowers(garden, row, col)
            .map_or(' ', |count| char::from_digit(count, 10).unwrap()),
    }
}

fn count_adjacent_flowers(garden: &[&str], row: usize, col: usize) -> Option<u32> {
    let count = get_neighbors(row, col)
        .into_iter()
        .filter_map(|(nr, nc)| get_cell_at(garden, nr, nc))
        .filter(|&cell| cell == '*')
        .count() as u32;

    (count > 0).then_some(count)
}

fn get_neighbors(row: usize, col: usize) -> Vec<(i32, i32)> {
    (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| !(dr == 0 && dc == 0))
        .map(|(dr, dc)| (row as i32 + dr, col as i32 + dc))
        .collect()
}

fn get_cell_at(garden: &[&str], row: i32, col: i32) -> Option<char> {
    if row >= 0 && col >= 0 {
        garden
            .get(row as usize)?
            .as_bytes()
            .get(col as usize)
            .map(|&byte| byte as char)
    } else {
        None
    }
}
