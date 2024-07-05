pub fn is_pangram(sentence: &str) -> bool {
    ('a'..='z').all(|c| sentence.to_lowercase().contains(c))
}
