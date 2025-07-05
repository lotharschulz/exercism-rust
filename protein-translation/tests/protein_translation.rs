use protein_translation::*;

#[test]
fn empty_rna_sequence_results_in_no_proteins() {
    assert_eq!(translate(""), Ok(vec![]),);
}

#[test]
fn methionine_rna_sequence() {
    assert_eq!(translate("AUG"), Ok(vec!["Methionine"]),);
}

#[test]
fn phenylalanine_rna_sequence_1() {
    assert_eq!(translate("UUU"), Ok(vec!["Phenylalanine"]),);
}

#[test]
fn phenylalanine_rna_sequence_2() {
    assert_eq!(translate("UUC"), Ok(vec!["Phenylalanine"]),);
}

#[test]
fn leucine_rna_sequence_1() {
    assert_eq!(translate("UUA"), Ok(vec!["Leucine"]),);
}

#[test]
fn leucine_rna_sequence_2() {
    assert_eq!(translate("UUG"), Ok(vec!["Leucine"]),);
}

#[test]
fn serine_rna_sequence_1() {
    assert_eq!(translate("UCU"), Ok(vec!["Serine"]),);
}

#[test]
fn serine_rna_sequence_2() {
    assert_eq!(translate("UCC"), Ok(vec!["Serine"]),);
}

#[test]
fn serine_rna_sequence_3() {
    assert_eq!(translate("UCA"), Ok(vec!["Serine"]),);
}

#[test]
fn serine_rna_sequence_4() {
    assert_eq!(translate("UCG"), Ok(vec!["Serine"]),);
}

#[test]
fn tyrosine_rna_sequence_1() {
    assert_eq!(translate("UAU"), Ok(vec!["Tyrosine"]),);
}

#[test]
fn tyrosine_rna_sequence_2() {
    assert_eq!(translate("UAC"), Ok(vec!["Tyrosine"]),);
}

#[test]
fn cysteine_rna_sequence_1() {
    assert_eq!(translate("UGU"), Ok(vec!["Cysteine"]),);
}

#[test]
fn cysteine_rna_sequence_2() {
    assert_eq!(translate("UGC"), Ok(vec!["Cysteine"]),);
}

#[test]
fn tryptophan_rna_sequence() {
    assert_eq!(translate("UGG"), Ok(vec!["Tryptophan"]),);
}

#[test]
fn stop_codon_rna_sequence_1() {
    assert_eq!(translate("UAA"), Ok(vec![]),);
}

#[test]
fn stop_codon_rna_sequence_2() {
    assert_eq!(translate("UAG"), Ok(vec![]),);
}

#[test]
fn stop_codon_rna_sequence_3() {
    assert_eq!(translate("UGA"), Ok(vec![]),);
}

#[test]
fn sequence_of_two_protein_codons_translates_into_proteins() {
    assert_eq!(
        translate("UUUUUU"),
        Ok(vec!["Phenylalanine", "Phenylalanine"]),
    );
}

#[test]
fn sequence_of_two_different_protein_codons_translates_into_proteins() {
    assert_eq!(translate("UUAUUG"), Ok(vec!["Leucine", "Leucine"]),);
}

#[test]
fn translate_rna_strand_into_correct_protein_list() {
    assert_eq!(
        translate("AUGUUUUGG"),
        Ok(vec!["Methionine", "Phenylalanine", "Tryptophan"]),
    );
}

#[test]
fn translation_stops_if_stop_codon_at_beginning_of_sequence() {
    assert_eq!(translate("UAGUGG"), Ok(vec![]),);
}

#[test]
fn translation_stops_if_stop_codon_at_end_of_two_codon_sequence() {
    assert_eq!(translate("UGGUAG"), Ok(vec!["Tryptophan"]),);
}

#[test]
fn translation_stops_if_stop_codon_at_end_of_three_codon_sequence() {
    assert_eq!(
        translate("AUGUUUUAA"),
        Ok(vec!["Methionine", "Phenylalanine"]),
    );
}

#[test]
fn translation_stops_if_stop_codon_in_middle_of_three_codon_sequence() {
    assert_eq!(translate("UGGUAGUGG"), Ok(vec!["Tryptophan"]),);
}

#[test]
fn translation_stops_if_stop_codon_in_middle_of_six_codon_sequence() {
    assert_eq!(
        translate("UGGUGUUAUUAAUGGUUU"),
        Ok(vec!["Tryptophan", "Cysteine", "Tyrosine"]),
    );
}

#[test]
fn sequence_of_two_non_stop_codons_does_not_translate_to_a_stop_codon() {
    assert_eq!(translate("AUGAUG"), Ok(vec!["Methionine", "Methionine"]),);
}

#[test]
fn unknown_amino_acids_not_part_of_a_codon_can_t_translate() {
    assert_eq!(translate("XYZ"), Err(TranslationError::InvalidCodon),);
}

#[test]
fn incomplete_rna_sequence_can_t_translate() {
    assert_eq!(translate("AUGU"), Err(TranslationError::IncompleteSequence),);
}

#[test]
fn incomplete_rna_sequence_can_translate_if_valid_until_a_stop_codon() {
    assert_eq!(
        translate("UUCUUCUAAUGGU"),
        Ok(vec!["Phenylalanine", "Phenylalanine"]),
    );
}
