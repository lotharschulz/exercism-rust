pub fn encrypt(input: &str) -> String {
    // Step 1: Normalize the input (remove spaces/punctuation, lowercase)
    let normalized: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    if normalized.is_empty() {
        return String::new();
    }

    let len = normalized.len();

    // Step 2: Calculate rectangle dimensions
    // Find smallest c such that: r * c >= len, c >= r, c - r <= 1
    let c = (len as f64).sqrt().ceil() as usize;
    let r = len.div_ceil(c);

    // Step 3: Fill rectangle row by row
    let mut rectangle: Vec<Vec<char>> = vec![vec![' '; c]; r];
    for (i, ch) in normalized.chars().enumerate() {
        let row = i / c;
        let col = i % c;
        if row < r {
            rectangle[row][col] = ch;
        }
    }

    // Step 4: Read columns to create cipher text
    let columns: Vec<String> = (0..c)
        .map(|col| rectangle.iter().map(|row| row[col]).collect())
        .collect();

    // Step 5: Join columns with spaces
    columns.join(" ")
}
