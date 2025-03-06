pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    
    // Handle empty matrix
    if input.is_empty() || input[0].is_empty() {
        return result;
    }

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            // Check if value is maximum in its row
            let is_row_max = row.iter().all(|&x| x <= value);
            
            // Check if value is minimum in its column
            let is_col_min = input.iter().all(|r| r[col_idx] >= value);

            if is_row_max && is_col_min {
                result.push((row_idx, col_idx));
            }
        }
    }
    
    result
}
