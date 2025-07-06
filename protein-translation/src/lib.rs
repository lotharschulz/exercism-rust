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
    if rna.len() % 3 != 0 {
        // Handle cases where the RNA string length is not a multiple of 3 but a stop codon is present
        let stop_pos = rna
            .find("UAA")
            .or_else(|| rna.find("UAG"))
            .or_else(|| rna.find("UGA"));
        if let Some(pos) = stop_pos {
            if pos % 3 != 0 {
                return Err(TranslationError::InvalidCodon);
            }
            let rna_substr = &rna[..pos];
            if rna_substr.len() % 3 != 0 {
                return Err(TranslationError::IncompleteSequence);
            }
        } else {
            return Err(TranslationError::IncompleteSequence);
        }
    }

    rna.as_bytes()
        .chunks(3)
        .map(|codon_bytes| {
            std::str::from_utf8(codon_bytes)
                .map_err(|_| TranslationError::InvalidCodon)
                .and_then(protein_from_codon)
        })
        .take_while(|res| res.as_ref().map(|opt| opt.is_some()).unwrap_or(true))
        .filter_map(|res| res.transpose())
        .collect()
}
