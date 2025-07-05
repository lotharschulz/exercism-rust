pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    let codons = rna.as_bytes().chunks(3);

    for codon_bytes in codons {
        let codon = match std::str::from_utf8(codon_bytes) {
            Ok(c) => c,
            Err(_) => return None,
        };

        if codon.len() != 3 {
            return None;
        }

        match codon {
            "AUG" => proteins.push("Methionine"),
            "UUU" | "UUC" => proteins.push("Phenylalanine"),
            "UUA" | "UUG" => proteins.push("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => proteins.push("Serine"),
            "UAU" | "UAC" => proteins.push("Tyrosine"),
            "UGU" | "UGC" => proteins.push("Cysteine"),
            "UGG" => proteins.push("Tryptophan"),
            "UAA" | "UAG" | "UGA" => return Some(proteins),
            _ => return None,
        }
    }

    Some(proteins)
}
