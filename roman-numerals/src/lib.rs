use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            value: to_roman_numeral(num),
        }
    }
}

/// Converts an Arabic numeral (1-3999) to its Roman numeral representation.
/// Uses a greedy algorithm that repeatedly subtracts the largest possible
/// value-numeral pair from the number, building the result string.
fn to_roman_numeral(mut num: u32) -> String {
    const PAIRS: &[(u32, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();

    for &(value, numeral) in PAIRS {
        while num >= value {
            result.push_str(numeral);
            num -= value;
        }
    }

    result
}
