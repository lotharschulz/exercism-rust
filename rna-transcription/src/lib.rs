#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, nucleotide) in dna.chars().enumerate() {
            match nucleotide {
                'A' | 'C' | 'G' | 'T' => continue,
                _ => return Err(i),
            }
        }
        Ok(Dna {
            sequence: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna_sequence: String = self
            .sequence
            .chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna {
            sequence: rna_sequence,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, nucleotide) in rna.chars().enumerate() {
            match nucleotide {
                'A' | 'C' | 'G' | 'U' => continue,
                _ => return Err(i),
            }
        }
        Ok(Rna {
            sequence: rna.to_string(),
        })
    }
}
