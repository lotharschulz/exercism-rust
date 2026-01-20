use std::collections::HashMap;

/// Count occurrences of words in a string.
///
/// This function processes the input string through the following steps:
/// 1. Splits on any non-alphanumeric character except apostrophes (')
/// 2. Removes leading and trailing apostrophes from each word
/// 3. Filters out empty strings
/// 4. Converts all words to lowercase for case-insensitive counting
/// 5. Returns a HashMap with word counts
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// let counts = word_count::word_count("Hello, hello!");
/// assert_eq!(counts.get("hello"), Some(&2));
/// ```
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'') // `is_punctuation()` as in https://doc.rust-lang.org/nightly/nightly-rustc/src/rustfmt_nightly/string.rs.html#366-370 may be useful in the future
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('\''))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
