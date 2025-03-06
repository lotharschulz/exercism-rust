pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Initialize an empty vector to store the saddle points
    let mut saddle_points = Vec::new();
    // Handle empty matrix
    if input.is_empty() || input[0].is_empty() {
        // Return empty saddle_points if the input matrix is empty
        return saddle_points;
    }

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            // Check if value is maximum in its row
            let is_row_max = row.iter().all(|&x| x <= value);
            // Check if value is minimum in its column
            let is_col_min = input.iter().all(|r| r[col_idx] >= value);

            // Add the position to the saddle_points if it's a saddle point
            if is_row_max && is_col_min {
                saddle_points.push((row_idx, col_idx));
            }
        }
    }
    saddle_points
}
