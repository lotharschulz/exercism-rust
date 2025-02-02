use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // Check if the nucleotide is valid
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }
    // Check if the DNA string contains only valid nucleotides
    if !dna.chars().all(|c| "ACGT".contains(c)) {
        return Err(dna.chars().find(|&c| !"ACGT".contains(c)).unwrap());
    }
    // Count the occurrences of the nucleotide in the DNA string
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
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
        *counts.get_mut(&c).unwrap() += 1;
    }
    Ok(counts)
}
