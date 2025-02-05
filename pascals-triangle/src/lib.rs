pub struct PascalsTriangle {
    row_count: u32, // Store the number of rows in the triangle
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // Initialize a new Pascal's Triangle with the given number of rows
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle = Vec::new(); // Initialize the vector to store the rows of the triangle
        for row_idx in 0..self.row_count {
            let mut row = Vec::with_capacity(row_idx as usize + 1); // Create a new row with the appropriate capacity
            for col_idx in 0..=row_idx {
                if col_idx == 0 || col_idx == row_idx {
                    // The first and last elements of each row are always 1
                    row.push(1);
                } else {
                    // Calculate the value based on the sum of the two elements above it
                    let prev_row: &Vec<u32> = &triangle[row_idx as usize - 1];
                    let value = prev_row[col_idx as usize - 1] + prev_row[col_idx as usize];
                    row.push(value);
                }
            }
            triangle.push(row); // Add the completed row to the triangle
        }
        triangle // Return the completed triangle
    }
}
