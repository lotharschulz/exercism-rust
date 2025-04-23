pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new(); // Return empty vector for size 0
    }

    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size]; // Initialize with zeros
    let mut value = 1; // Start with 1

    let mut row_start = 0;
    let mut row_end = size - 1;
    let mut col_start = 0;
    let mut col_end = size - 1;

    while row_start <= row_end && col_start <= col_end {
        // Fill top row left to right
        for col in col_start..=col_end {
            matrix[row_start][col] = value;
            value += 1;
        }
        row_start += 1;

        // Fill right column top to bottom
        for row in row_start..=row_end {
            matrix[row][col_end] = value;
            value += 1;
        }

        if col_end > 0 {
            col_end -= 1;
        } else {
            break; // Avoid underflow
        }

        // Fill bottom row right to left
        if row_start <= row_end {
            for col in (col_start..=col_end).rev() {
                matrix[row_end][col] = value;
                value += 1;
            }

            if row_end > 0 {
                row_end -= 1;
            } else {
                break; // Avoid underflow
            }
        }

        // Fill left column bottom to top
        if col_start <= col_end {
            for row in (row_start..=row_end).rev() {
                matrix[row][col_start] = value;
                value += 1;
            }
            col_start += 1;
        }
    }

    matrix
}
