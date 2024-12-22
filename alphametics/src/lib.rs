use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the input into words and the result
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }
    let result_word = parts[1];
    let words: Vec<&str> = parts[0].split(" + ").collect();

    // Collect all unique characters
    let mut chars: Vec<char> = words.iter().flat_map(|word| word.chars()).collect();
    chars.extend(result_word.chars());
    chars.sort_unstable();
    chars.dedup();

    // Ensure there are no more than 10 unique characters
    if chars.len() > 10 {
        return None;
    }

    // Helper function to convert a word to a number based on the current mapping
    fn word_to_number(word: &str, map: &HashMap<char, u8>) -> Option<u64> {
        word.chars()
            .map(|c| map.get(&c).copied())
            .collect::<Option<Vec<u8>>>()
            .map(|digits| digits.into_iter().fold(0, |acc, d| acc * 10 + d as u64))
    }

    // Recursive function to try all possible mappings
    fn solve_recursive(
        chars: &[char],
        map: &mut HashMap<char, u8>,
        used: &mut [bool],
        words: &[&str],
        result_word: &str,
    ) -> bool {
        if chars.is_empty() {
            let sum: u64 = words
                .iter()
                .map(|&word| word_to_number(word, map).unwrap())
                .sum();
            return sum == word_to_number(result_word, map).unwrap();
        }

        let c = chars[0];
        for digit in 0..=9 {
            if !used[digit as usize] {
                // Ensure no leading zeros
                if digit == 0
                    && (words.iter().any(|&word| word.starts_with(c)) || result_word.starts_with(c))
                {
                    continue;
                }
                map.insert(c, digit);
                used[digit as usize] = true;
                if solve_recursive(&chars[1..], map, used, words, result_word) {
                    return true;
                }
                used[digit as usize] = false;
                map.remove(&c);
            }
        }
        return false;
    }

    let mut map = HashMap::new();
    let mut used = [false; 10];
    if solve_recursive(&chars, &mut map, &mut used, &words, result_word) {
        return Some(map);
    } else {
        return None;
    }
}
