pub trait Luhn {
    #[must_use]
    fn valid_luhn(&self) -> bool;
}

/// Helper function to check if a string is a valid Luhn number.
/// Returns Some(true) if valid, Some(false) if invalid but only contains digits/spaces,
/// None if contains invalid characters.
fn is_valid_luhn_str(input: &str) -> Option<bool> {
    let mut digits = Vec::new();
    let mut has_valid_chars = true;

    for ch in input.chars() {
        match ch {
            '0'..='9' => digits.push(ch as u8 - b'0'),
            ' ' => {}
            _ => {
                has_valid_chars = false;
            }
        }
    }

    if !has_valid_chars {
        return None;
    }

    if digits.len() <= 1 {
        return Some(false);
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &d)| {
            let mut val = u32::from(d);
            if idx % 2 == 1 {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            val
        })
        .sum();

    Some(sum.is_multiple_of(10))
}

impl Luhn for &str {
    fn valid_luhn(&self) -> bool {
        is_valid_luhn_str(self).unwrap_or(false)
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}

macro_rules! impl_luhn_for_uint {
    ($($t:ty),* $(,)?) => {
        $(
            impl Luhn for $t {
                fn valid_luhn(&self) -> bool {
                    let s = self.to_string();
                    s.valid_luhn()
                }
            }
        )*
    }
}

impl_luhn_for_uint!(u8, u16, u32, u64, usize);
