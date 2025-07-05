#[derive(Debug, PartialEq)]
pub enum TranslationError {
    InvalidCodon,
    IncompleteSequence,
}

fn protein_from_codon(codon: &str) -> Result<Option<&str>, TranslationError> {
    match codon {
        "AUG" => Ok(Some("Methionine")),
        "UUU" | "UUC" => Ok(Some("Phenylalanine")),
        "UUA" | "UUG" => Ok(Some("Leucine")),
        "UCU" | "UCC" | "UCA" | "UCG" => Ok(Some("Serine")),
        "UAU" | "UAC" => Ok(Some("Tyrosine")),
        "UGU" | "UGC" => Ok(Some("Cysteine")),
        "UGG" => Ok(Some("Tryptophan")),
        "UAA" | "UAG" | "UGA" => Ok(None), // STOP codon
        _ => Err(TranslationError::InvalidCodon),
    }
}

pub fn translate(rna: &str) -> Result<Vec<&str>, TranslationError> {
    rna.as_bytes()
        .chunks(3)
        .map(|codon_bytes| {
            if codon_bytes.len() != 3 {
                return Err(TranslationError::IncompleteSequence);
            }
            std::str::from_utf8(codon_bytes)
                .map_err(|_| TranslationError::InvalidCodon)
                .and_then(protein_from_codon)
        })
        .take_while(|res| res.as_ref().map(|opt| opt.is_some()).unwrap_or(true))
        .filter_map(|res| res.transpose())
        .collect()
}
