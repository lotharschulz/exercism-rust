use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    for c in candidate.chars().filter(|c| c.is_alphabetic()) {
        let lower_c = c.to_lowercase().next().unwrap();
        if seen.contains(&lower_c) {
            return false;
        }
        seen.insert(lower_c);
    }
    true
}
