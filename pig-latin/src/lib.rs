pub fn translate(input: &str) -> String {
    input.split_whitespace().map(|word| {
        let first_vowel_pos = word.find(|c: char| "aeiou".contains(c));
        match first_vowel_pos {
            Some(0) => format!("{}ay", word),
            Some(pos) => {
                if word[..pos].ends_with('q') && word[pos..].starts_with('u') {
                    format!("{}{}ay", &word[pos + 1..], &word[..pos + 1])
                } else {
                    format!("{}{}ay", &word[pos..], &word[..pos])
                }
            }
            None => format!("{}ay", word),
        }
    }).collect::<Vec<_>>().join(" ")
}
