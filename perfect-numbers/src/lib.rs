#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

// Function to classify a number as Abundant, Perfect, or Deficient
pub fn classify(num: u64) -> Option<Classification> {
    // Return None if the number is 0, as it cannot be classified
    if num == 0 {
        return None;
    }

    // Calculate the sum of all divisors of the number (excluding the number itself)
    let sum: u64 = (1..num).filter(|&x| num % x == 0).sum();

    // Use pattern matching to return the appropriate classification
    match sum {
        s if s == num => Some(Classification::Perfect), // Perfect if the sum equals the number
        s if s > num => Some(Classification::Abundant), // Abundant if the sum is greater than the number
        _ => Some(Classification::Deficient), // Deficient if the sum is less than the number
    }
}
