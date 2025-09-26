use std::collections::HashSet;

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let sorted_word: Vec<char> = sorted_chars(word);
    for anagram in possible_anagrams {
        if word.to_lowercase() != anagram.to_lowercase() && sorted_word == sorted_chars(anagram) {
            result.insert(*anagram);
        }
    }
    result
}
