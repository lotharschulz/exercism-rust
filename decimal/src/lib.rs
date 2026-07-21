use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};

const BASE: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

impl Sign {
    fn flipped(self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

/// Arbitrary-precision decimal number.
///
/// The value is `digits / BASE^scale`, negated when `sign` is negative.
/// `digits` stores decimal digits least-significant first, kept in canonical
/// form: no trailing fractional zeros, no leading integer zeros, and zero is
/// always positive with scale 0. Canonical form makes derived equality valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    sign: Sign,
    digits: Vec<u8>,
    scale: usize,
}

impl Decimal {
    /// Parses a decimal number such as `"-12.34"` or `"+5"`.
    ///
    /// # Examples
    /// ```
    /// use decimal::Decimal;
    /// assert!(Decimal::try_from("3.14").is_some());
    /// assert!(Decimal::try_from("not a number").is_none());
    /// ```
    #[must_use]
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (sign, unsigned) = match input.strip_prefix('-') {
            Some(rest) => (Sign::Negative, rest),
            None => (Sign::Positive, input.strip_prefix('+').unwrap_or(input)),
        };
        let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        if integer.is_empty() && fraction.is_empty() {
            return None;
        }
        let digits = integer
            .chars()
            .chain(fraction.chars())
            .rev()
            .map(|c| {
                c.to_digit(BASE.into())
                    .and_then(|digit| u8::try_from(digit).ok())
            })
            .collect::<Option<Vec<u8>>>()?;
        Some(Decimal::new(sign, digits, fraction.len()))
    }

    /// Builds a decimal and brings it into canonical form.
    fn new(sign: Sign, mut digits: Vec<u8>, mut scale: usize) -> Decimal {
        let redundant_fraction_zeros = digits
            .iter()
            .take(scale)
            .take_while(|&&digit| digit == 0)
            .count();
        digits.drain(..redundant_fraction_zeros);
        scale -= redundant_fraction_zeros;
        while digits.last() == Some(&0) {
            digits.pop();
        }
        if digits.is_empty() {
            return Decimal {
                sign: Sign::Positive,
                digits: vec![0],
                scale: 0,
            };
        }
        Decimal {
            sign,
            digits,
            scale,
        }
    }

    /// Returns the digits shifted so they represent the value at `scale`
    /// fractional places. `scale` must be at least `self.scale`.
    fn upscaled(&self, scale: usize) -> Vec<u8> {
        let mut digits = vec![0; scale - self.scale];
        digits.extend(&self.digits);
        digits
    }
}

fn digit_at(digits: &[u8], index: usize) -> u8 {
    digits.get(index).copied().unwrap_or(0)
}

/// Compares little-endian digit sequences of possibly different lengths.
fn compare_digits(a: &[u8], b: &[u8]) -> Ordering {
    (0..a.len().max(b.len()))
        .rev()
        .map(|i| digit_at(a, i).cmp(&digit_at(b, i)))
        .find(|&ordering| ordering != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

fn add_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let longest = a.len().max(b.len());
    let mut sum = Vec::with_capacity(longest + 1);
    let mut carry = 0;
    for i in 0..longest {
        let total = digit_at(a, i) + digit_at(b, i) + carry;
        sum.push(total % BASE);
        carry = total / BASE;
    }
    if carry > 0 {
        sum.push(carry);
    }
    sum
}

/// Subtracts `b` from `a`. Requires `a >= b`.
fn sub_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut difference = Vec::with_capacity(a.len());
    let mut borrow = 0;
    for (i, &digit) in a.iter().enumerate() {
        let subtrahend = digit_at(b, i) + borrow;
        let (result, borrowed) = if digit >= subtrahend {
            (digit - subtrahend, 0)
        } else {
            (digit + BASE - subtrahend, 1)
        };
        difference.push(result);
        borrow = borrowed;
    }
    difference
}

/// Schoolbook multiplication of little-endian digit sequences.
fn mul_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut product = vec![0; a.len() + b.len()];
    for (i, &digit_a) in a.iter().enumerate() {
        let mut carry = 0;
        for (j, &digit_b) in b.iter().enumerate() {
            let total = product[i + j] + digit_a * digit_b + carry;
            product[i + j] = total % BASE;
            carry = total / BASE;
        }
        product[i + b.len()] += carry;
    }
    product
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let scale = self.scale.max(other.scale);
        let a = self.upscaled(scale);
        let b = other.upscaled(scale);
        if self.sign == other.sign {
            return Decimal::new(self.sign, add_digits(&a, &b), scale);
        }
        match compare_digits(&a, &b) {
            Ordering::Less => Decimal::new(other.sign, sub_digits(&b, &a), scale),
            _ => Decimal::new(self.sign, sub_digits(&a, &b), scale),
        }
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        Decimal::new(self.sign.flipped(), self.digits, self.scale)
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        self + -other
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        let sign = if self.sign == other.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };
        Decimal::new(
            sign,
            mul_digits(&self.digits, &other.digits),
            self.scale + other.scale,
        )
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (sign, _) => {
                let scale = self.scale.max(other.scale);
                let by_magnitude = compare_digits(&self.upscaled(scale), &other.upscaled(scale));
                match sign {
                    Sign::Positive => by_magnitude,
                    Sign::Negative => by_magnitude.reverse(),
                }
            }
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
