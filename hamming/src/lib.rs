/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // Check if the lengths of the two strings are equal
    if s1.len() != s2.len() {
        return None; // Return None if lengths are mismatched
    }

    // Calculate the Hamming distance by comparing characters at each position
    let distance = s1
        .chars() // Convert the first string into an iterator of characters
        .zip(s2.chars()) // Pair up characters from both strings
        .filter(|(c1, c2)| c1 != c2) // Filter out pairs where characters are different
        .count(); // Count the number of differing pairs

    Some(distance) // Return the Hamming distance wrapped in Some
}