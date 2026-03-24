pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }
    let height = lines.len();
    let width = lines[0].len();
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut rectangle_count = 0;
    for top in 0..height {
        for bottom in (top + 1)..height {
            let valid_columns: Vec<usize> = (0..width)
                .filter(|&j| grid[top][j] == '+' && grid[bottom][j] == '+')
                .filter(|&j| {
                    (top + 1..bottom).all(|k| {
                        let cell = grid[k][j];
                        cell == '|' || cell == '+'
                    })
                })
                .collect();
            for i in 0..valid_columns.len() {
                for j in (i + 1)..valid_columns.len() {
                    let left = valid_columns[i];
                    let right = valid_columns[j];
                    let horizontal_ok = (left + 1..right).all(|jj| {
                        let top_cell = grid[top][jj];
                        let bottom_cell = grid[bottom][jj];
                        (top_cell == '-' || top_cell == '+') && (bottom_cell == '-' || bottom_cell == '+')
                    });
                    if horizontal_ok {
                        rectangle_count += 1;
                    }
                }
            }
        }
    }
    rectangle_count
}
