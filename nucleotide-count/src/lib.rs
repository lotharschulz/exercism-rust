use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // Check if the nucleotide is valid
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }
    // Get the nucleotide counts
    let counts = nucleotide_counts(dna)?;
    // Return the count for the specified nucleotide
    match counts.get(&nucleotide) {
        Some(&count) => Ok(count),
        None => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    // Initialize counts for each nucleotide
    for nucleotide in "ACGT".chars() {
        counts.insert(nucleotide, 0);
    }
    // Count the occurrences of each nucleotide in the DNA string
    for c in dna.chars() {
        if !"ACGT".contains(c) {
            return Err(c);
        }
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        }
    }
    Ok(counts)
}
