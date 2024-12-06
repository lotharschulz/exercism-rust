pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut new_word = true;
    let mut prev_uppercase = false;

    for c in phrase.chars() {
        if c.is_alphabetic() {
            // Add the character to the acronym if it's the start of a new word
            // or an uppercase character that follows a lowercase character
            if new_word || (c.is_uppercase() && !prev_uppercase) {
                acronym.push(c.to_uppercase().next().unwrap());
                new_word = false;
            }
            // Track if the current character is uppercase
            prev_uppercase = c.is_uppercase();
        } else if c == '\'' {
            // Ignore apostrophes
            continue;
        } else {
            // Mark the start of a new word for non-alphabetic characters
            new_word = true;
            prev_uppercase = false;
        }
    }

    acronym
}
