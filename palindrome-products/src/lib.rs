use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64) -> Self {
        Palindrome {
            value,
            factors: HashSet::new(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    pub fn add_factor(&mut self, factor1: u64, factor2: u64) {
        self.factors.insert((factor1, factor2));
    }
}

fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                if let Some(p) = &mut min_palindrome {
                    if product < p.value() {
                        *p = Palindrome::new(product);
                        p.add_factor(i, j);
                    } else if product == p.value() {
                        p.add_factor(i, j);
                    }
                } else {
                    let mut p = Palindrome::new(product);
                    p.add_factor(i, j);
                    min_palindrome = Some(p);
                }

                if let Some(p) = &mut max_palindrome {
                    if product > p.value() {
                        *p = Palindrome::new(product);
                        p.add_factor(i, j);
                    } else if product == p.value() {
                        p.add_factor(i, j);
                    }
                } else {
                    let mut p = Palindrome::new(product);
                    p.add_factor(i, j);
                    max_palindrome = Some(p);
                }
            }
        }
    }

    match (min_palindrome, max_palindrome) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    }
}
