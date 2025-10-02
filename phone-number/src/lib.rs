#[must_use]
pub fn number(user_number: &str) -> Option<String> {
    // Keep only digits
    let mut digits = String::with_capacity(user_number.len());
    for ch in user_number.chars() {
        match ch {
            '0'..='9' => digits.push(ch),
            '+' | '(' | ')' | ' ' | '-' | '.' => {
                // allowed punctuation; ignore
            }
            _ => return None, // invalid character (letters or other punctuation)
        }
    }

    // Handle country code
    if digits.len() == 11 {
        if digits.starts_with('1') {
            digits.remove(0); // drop leading country code
        } else {
            return None;
        }
    }

    // Must be exactly 10 digits now
    if digits.len() != 10 {
        return None;
    }

    // NANP rules: area code and exchange code (first of each trio) must be 2-9
    let bytes = digits.as_bytes();
    let area_first = bytes[0];
    let exch_first = bytes[3];
    if !(b'2'..=b'9').contains(&area_first) {
        return None;
    }
    if !(b'2'..=b'9').contains(&exch_first) {
        return None;
    }

    Some(digits)
}
