pub struct Luhn {
    digits: Vec<u8>,
    only_digits_and_spaces: bool,
}

impl Luhn {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // must contain only digits/spaces in the original input
        if !self.only_digits_and_spaces {
            return false;
        }

        // must be at least two digits
        if self.digits.len() <= 1 {
            return false;
        }

        // Luhn checksum: from right, double every second digit; if >9, subtract 9
        let sum: u32 = self
            .digits
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

        sum.is_multiple_of(10)
    }
}

/// Build a Luhn from a string slice, allowing spaces and rejecting any other non-digit chars.
impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        let mut only_digits_and_spaces = true;
        let mut digits = Vec::new();

        for ch in input.chars() {
            match ch {
                '0'..='9' => digits.push(ch as u8 - b'0'),
                ' ' => {}
                _ => {
                    only_digits_and_spaces = false;
                }
            }
        }

        Luhn {
            digits,
            only_digits_and_spaces,
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}

macro_rules! impl_from_uint_for_luhn {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for Luhn {
                fn from(value: $t) -> Self {
                    // Convert to string and reuse &str implementation.
                    // No spaces, only digits.
                    let s = value.to_string();
                    let digits = s.bytes().map(|b| b - b'0').collect();
                    Luhn { digits, only_digits_and_spaces: true }
                }
            }
        )*
    }
}

impl_from_uint_for_luhn!(u8, u16, u32, u64, usize);
