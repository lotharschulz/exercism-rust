pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let rows = garden.len();
    let cols = if rows > 0 { garden[0].len() } else { 0 };

    let mut result = Vec::with_capacity(rows);

    for row in 0..rows {
        let mut row_string = String::with_capacity(cols);

        for col in 0..cols {
            let current_char = garden[row].as_bytes()[col] as char;

            if current_char == '*' {
                row_string.push('*');
            } else {
                // Count adjacent flowers
                let mut count = 0;

                // Check all 8 directions: up, up-right, right, down-right, down, down-left, left, up-left
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue; // Skip the current cell
                        }

                        let nr = row as i32 + dr;
                        let nc = col as i32 + dc;

                        // Check bounds
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let neighbor_char = garden[nr as usize].as_bytes()[nc as usize] as char;
                            if neighbor_char == '*' {
                                count += 1;
                            }
                        }
                    }
                }

                if count == 0 {
                    row_string.push(' ');
                } else {
                    row_string.push(char::from_digit(count, 10).unwrap());
                }
            }
        }

        result.push(row_string);
    }

    result
}
