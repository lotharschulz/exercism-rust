pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };
    let mut result = vec![String::new(); height];

    for y in 0..height {
        for x in 0..width {
            if minefield[y].as_bytes()[x] == b'*' {
                result[y].push('*');
            } else {
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                            if minefield[ny as usize].as_bytes()[nx as usize] == b'*' {
                                count += 1;
                            }
                        }
                    }
                }
                if count > 0 {
                    result[y].push(char::from_digit(count, 10).unwrap());
                } else {
                    result[y].push(' ');
                }
            }
        }
    }

    result
}
