use std::collections::HashSet;

// Represents a palindrome number with its factors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,                    // The palindrome number itself
    factors: HashSet<(u64, u64)>,  // Set of factor pairs that produce this palindrome
}

impl Palindrome {
    // Creates a new Palindrome instance with empty factors
    pub fn new(value: u64) -> Self {
        Palindrome {
            value,
            factors: HashSet::new(),
        }
    }

    // Returns the palindrome value
    pub fn value(&self) -> u64 {
        self.value
    }

    // Consumes the palindrome and returns its factors
    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    // Adds a factor pair to the palindrome
    pub fn add_factor(&mut self, factor1: u64, factor2: u64) {
        self.factors.insert((factor1, factor2));
    }
}

// Checks if a number is a palindrome by converting to string and comparing with its reverse
fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}

// Finds the smallest and largest palindrome products within the given range
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // Return None if range is invalid
    if min > max {
        return None;
    }

    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;

    // Iterate through all possible factor pairs
    for i in min..=max {
        for j in i..=max {  // Start from i to avoid duplicate factor pairs
            let product = i * j;
            if is_palindrome(product) {
                // Handle minimum palindrome
                if let Some(p) = &mut min_palindrome {
                    if product < p.value() {
                        // Found new minimum
                        *p = Palindrome::new(product);
                        p.add_factor(i, j);
                    } else if product == p.value() {
                        // Found additional factors for minimum
                        p.add_factor(i, j);
                    }
                } else {
                    // First palindrome found
                    let mut p = Palindrome::new(product);
                    p.add_factor(i, j);
                    min_palindrome = Some(p);
                }

                // Handle maximum palindrome (similar logic as minimum)
                if let Some(p) = &mut max_palindrome {
                    if product > p.value() {
                        // Found new maximum
                        *p = Palindrome::new(product);
                        p.add_factor(i, j);
                    } else if product == p.value() {
                        // Found additional factors for maximum
                        p.add_factor(i, j);
                    }
                } else {
                    // First palindrome found
                    let mut p = Palindrome::new(product);
                    p.add_factor(i, j);
                    max_palindrome = Some(p);
                }
            }
        }
    }

    // Return tuple of min and max palindromes if both exist, None otherwise
    match (min_palindrome, max_palindrome) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    }
}
