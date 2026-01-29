/// Represents a matrix of integers stored as a string.
pub struct Matrix {
    /// Raw matrix data as a string, with rows separated by newlines
    data: String,
}

impl Matrix {
    /// Creates a new Matrix from a string representation.
    ///
    /// # Arguments
    /// * `input` - A string containing matrix values, either single number or space/newline separated
    pub fn new(input: &str) -> Self {
        Matrix {
            data: input.to_string(),
        }
    }

    /// Extracts a row from the matrix (1-indexed).
    ///
    /// # Arguments
    /// * `row_no` - The row number to extract (1-indexed)
    ///
    /// # Returns
    /// * `Some(Vec<u32>)` if the row exists, `None` if row_no is out of bounds
    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        // Handle single-number matrices (no newlines)
        if !self.data.contains('\n') {
            return self.data.parse::<u32>().ok().map(|n| vec![n]);
        }

        // Split by newlines and find the requested row (converting 1-indexed to 0-indexed)
        self.data
            .split('\n')
            .nth(row_no.saturating_sub(1))
            .and_then(|row_data| {
                // Parse each whitespace-separated value to u32
                let numbers: Vec<u32> = row_data
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();
                // Return Some if numbers exist, None if row was empty
                if numbers.is_empty() {
                    None
                } else {
                    Some(numbers)
                }
            })
    }

    /// Extracts a column from the matrix (1-indexed).
    ///
    /// # Arguments
    /// * `col_no` - The column number to extract (1-indexed)
    ///
    /// # Returns
    /// * `Some(Vec<u32>)` if the column exists, `None` if col_no is out of bounds
    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        // Handle single-number matrices (no newlines)
        if !self.data.contains('\n') {
            return self.data.parse::<u32>().ok().map(|n| vec![n]);
        }

        let mut column_data = Vec::new();

        // Iterate through each row
        for row in self.data.split('\n') {
            // Parse each whitespace-separated value to u32
            let numbers: Vec<u32> = row
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            // Return None if the requested column index is out of bounds for this row
            if col_no > numbers.len() {
                return None;
            }

            // Add the value at the requested column (converting 1-indexed to 0-indexed)
            column_data.push(numbers[col_no.saturating_sub(1)]);
        }

        // Return Some if we collected any values, None if the matrix was empty
        if column_data.is_empty() {
            None
        } else {
            Some(column_data)
        }
    }
}
