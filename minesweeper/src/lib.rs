fn count_mines(minefield: &[&str], x: usize, y: usize, width: usize, height: usize) -> u32 {
    // Iterate over the 3x3 grid centered at (x, y)
    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
        .filter_map(|(dy, dx)| {
            if dy == 0 && dx == 0 {
                // Skip the center cell
                None
            } else {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                // Check if the neighboring cell is within bounds
                if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                    // Return true if the neighboring cell contains a mine
                    Some(minefield[ny as usize].as_bytes()[nx as usize] == b'*')
                } else {
                    None
                }
            }
        })
        // Count the number of mines in the neighboring cells
        .filter(|&is_mine| is_mine)
        .count() as u32
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };
    let mut result = vec![String::new(); height];

    for y in 0..height {
        for x in 0..width {
            if minefield[y].as_bytes()[x] == b'*' {
                // If the cell contains a mine, add '*' to the result
                result[y].push('*');
            } else {
                // Count the number of mines surrounding the cell
                let count = count_mines(minefield, x, y, width, height);
                if count > 0 {
                    // If there are surrounding mines, add the count to the result
                    result[y].push(char::from_digit(count, 10).unwrap());
                } else {
                    // If there are no surrounding mines, add a space to the result
                    result[y].push(' ');
                }
            }
        }
    }
    result
}
