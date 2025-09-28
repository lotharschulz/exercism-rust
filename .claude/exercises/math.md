# Math Exercise Patterns

## Core Principles for Math Exercises

### Numeric Type Selection
- Use `u32` for counts and indices
- Use `u64` for large numbers that might overflow
- Use `i32/i64` when negative values are possible
- Use `usize` for array indices and sizes
- Consider `u128/i128` for very large numbers
- Use `f64` for floating-point calculations

### Overflow Handling
- Use `checked_*` methods for safe arithmetic
- Use `saturating_*` for clamping to min/max
- Use `wrapping_*` for intentional wraparound
- Consider using `num` crate for big integers

## Common Math Patterns

### Prime Numbers
```rust
/// Check if a number is prime
pub fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        n => {
            let sqrt = (n as f64).sqrt() as u64;
            (3..=sqrt).step_by(2).all(|i| n % i != 0)
        }
    }
}

/// Generate primes using Sieve of Eratosthenes
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 {
        is_prime[1] = false;
    }
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &prime)| prime.then_some(num))
        .collect()
}

/// Find nth prime number
pub fn nth_prime(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;
    
    loop {
        if is_prime(candidate as u64) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += if candidate == 2 { 1 } else { 2 };
    }
}
```

### Factorial and Combinations
```rust
/// Calculate factorial with overflow checking
pub fn factorial(n: u32) -> Option<u64> {
    (1..=n as u64).try_fold(1u64, |acc, x| acc.checked_mul(x))
}

/// Iterative factorial with memoization
pub fn factorial_memo(n: u32) -> u64 {
    static mut CACHE: Vec<u64> = Vec::new();
    
    unsafe {
        if CACHE.is_empty() {
            CACHE.push(1); // 0! = 1
        }
        
        while CACHE.len() <= n as usize {
            let next = CACHE.last().unwrap() * CACHE.len() as u64;
            CACHE.push(next);
        }
        
        CACHE[n as usize]
    }
}

/// Calculate binomial coefficient (n choose k)
pub fn binomial(n: u32, k: u32) -> u64 {
    if k > n {
        return 0;
    }
    
    let k = k.min(n - k); // Optimization: C(n,k) = C(n,n-k)
    
    (0..k).fold(1, |acc, i| {
        acc * (n - i) as u64 / (i + 1) as u64
    })
}
```

### Number Properties
```rust
/// Check if number is perfect
pub fn is_perfect(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    
    let sum: u64 = (1..n)
        .filter(|i| n % i == 0)
        .sum();
    
    sum == n
}

/// Classify numbers as perfect, abundant, or deficient
#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(n: u64) -> Option<Classification> {
    if n == 0 {
        return None;
    }
    
    let aliquot_sum: u64 = (1..n)
        .filter(|i| n % i == 0)
        .sum();
    
    Some(match aliquot_sum.cmp(&n) {
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Abundant,
        std::cmp::Ordering::Less => Classification::Deficient,
    })
}

/// Check if number is Armstrong number
pub fn is_armstrong_number(n: u32) -> bool {
    let digits: Vec<u32> = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    let power = digits.len() as u32;
    let sum: u32 = digits.iter()
        .map(|&d| d.pow(power))
        .sum();
    
    sum == n
}
```

### Sequences
```rust
/// Generate Fibonacci sequence
pub fn fibonacci(n: usize) -> Vec<u64> {
    match n {
        0 => vec![],
        1 => vec![0],
        _ => {
            let mut fib = vec![0, 1];
            for i in 2..n {
                let next = fib[i - 1] + fib[i - 2];
                fib.push(next);
            }
            fib
        }
    }
}

/// Fibonacci with iterator
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current.checked_add(self.next)?;
        Some(current)
    }
}

/// Collatz conjecture
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    
    let mut n = n;
    let mut steps = 0;
    
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            n.checked_mul(3)?.checked_add(1)?
        };
        steps += 1;
    }
    
    Some(steps)
}
```

### Divisors and GCD/LCM
```rust
/// Find all divisors
pub fn divisors(n: u64) -> Vec<u64> {
    let sqrt = (n as f64).sqrt() as u64;
    let mut divs = Vec::new();
    
    for i in 1..=sqrt {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }
    
    divs.sort_unstable();
    divs
}

/// Calculate GCD using Euclidean algorithm
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculate LCM
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b  // Avoid overflow by dividing first
    }
}

/// Prime factorization
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    // Handle 2 separately
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    // Check odd numbers
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    // If n is still > 1, it's a prime factor
    if n > 1 {
        factors.push(n);
    }
    
    factors
}
```

## Exercise-Specific Templates

### nth-prime
```rust
pub fn nth(n: u32) -> u32 {
    // Using iterator approach
    (2u32..)
        .filter(|&x| is_prime(x as u64))
        .nth(n as usize)
        .expect("There are infinite primes")
}

// Helper function
fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        n => {
            let sqrt = (n as f64).sqrt() as u64;
            (3..=sqrt).step_by(2).all(|i| n % i != 0)
        }
    }
}
```

### sum-of-multiples
```rust
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let valid_factors: Vec<_> = factors.iter()
        .filter(|&&f| f != 0 && f < limit)
        .collect();
    
    if valid_factors.is_empty() {
        return 0;
    }
    
    (1..limit)
        .filter(|n| valid_factors.iter().any(|&&f| n % f == 0))
        .sum()
}

// Optimized using inclusion-exclusion principle for two factors
pub fn sum_of_multiples_optimized(limit: u32, a: u32, b: u32) -> u32 {
    fn sum_multiples(n: u32, limit: u32) -> u32 {
        if n == 0 || n >= limit {
            return 0;
        }
        let count = (limit - 1) / n;
        n * count * (count + 1) / 2
    }
    
    let lcm = lcm(a as u64, b as u64) as u32;
    sum_multiples(a, limit) + sum_multiples(b, limit) - sum_multiples(lcm, limit)
}
```

### perfect-numbers
```rust
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(n: u64) -> Option<Classification> {
    if n == 0 {
        return None;
    }
    
    // Calculate sum of proper divisors
    let sum = (1..=n/2)
        .filter(|i| n % i == 0)
        .sum::<u64>();
    
    Some(match sum.cmp(&n) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
        Ordering::Less => Classification::Deficient,
    })
}
```

### collatz-conjecture
```rust
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    
    Some(
        std::iter::successors(Some(n), |&n| {
            if n == 1 {
                None
            } else if n % 2 == 0 {
                Some(n / 2)
            } else {
                n.checked_mul(3)?.checked_add(1)
            }
        })
        .count() as u64 - 1  // Subtract 1 because we don't count the starting number
    )
}
```

## Optimization Strategies

### Memoization
```rust
use std::collections::HashMap;

pub struct Memoized<F> {
    cache: HashMap<u32, u64>,
    func: F,
}

impl<F> Memoized<F>
where
    F: Fn(&mut Memoized<F>, u32) -> u64,
{
    pub fn new(func: F) -> Self {
        Memoized {
            cache: HashMap::new(),
            func,
        }
    }
    
    pub fn call(&mut self, n: u32) -> u64 {
        if let Some(&result) = self.cache.get(&n) {
            return result;
        }
        
        let result = (self.func)(self, n);
        self.cache.insert(n, result);
        result
    }
}
```

### Integer Square Root
```rust
/// Fast integer square root
pub fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    
    x
}
```

## Common Pitfalls and Solutions

### Integer Overflow
```rust
// Wrong: Can overflow
let factorial = (1..=20u32).product::<u32>();

// Right: Check for overflow
let factorial = (1..=20u32).try_fold(1u32, |acc, x| acc.checked_mul(x));

// Alternative: Use larger type
let factorial = (1..=20u64).product::<u64>();
```

### Floating Point Precision
```rust
// Wrong: Direct floating point comparison
if (0.1 + 0.2) == 0.3 {  // May fail due to precision

// Right: Use epsilon comparison
const EPSILON: f64 = 1e-10;
if ((0.1 + 0.2) - 0.3).abs() < EPSILON {
```

### Division by Zero
```rust
// Wrong: Potential division by zero
let result = n / divisor;

// Right: Check first
let result = if divisor != 0 {
    Some(n / divisor)
} else {
    None
};
```

## Testing Math Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_input() {
        assert_eq!(function(0), expected);
    }

    #[test]
    fn test_one_input() {
        assert_eq!(function(1), expected);
    }

    #[test]
    fn test_prime_numbers() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for prime in primes {
            assert!(is_prime(prime));
        }
    }

    #[test]
    fn test_large_numbers() {
        // Test with u32::MAX or u64::MAX
        assert_eq!(function(u32::MAX), expected);
    }

    #[test]
    fn test_overflow_handling() {
        // Should not panic
        let result = factorial(100);
        assert!(result.is_none());
    }

    #[test]
    fn test_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        let _ = nth_prime(10_000);
        let duration = start.elapsed();
        
        assert!(duration.as_secs() < 1, "Should complete in under 1 second");
    }

    #[test]
    #[should_panic]
    fn test_panic_conditions() {
        // Test conditions that should panic
        function_that_panics(invalid_input);
    }
}
```

## Performance Tips

1. **Use integer arithmetic when possible** - Much faster than floating point
2. **Pre-compute when feasible** - Store primes, factorials, etc.
3. **Short-circuit evaluations** - Return early when possible
4. **Use bit operations** - `n & 1` is faster than `n % 2` for odd/even check
5. **Avoid unnecessary allocations** - Reuse vectors when possible
6. **Consider approximations** - Sometimes "good enough" is sufficient

## Exercise Checklist

- [ ] Choose appropriate integer types
- [ ] Handle overflow conditions
- [ ] Check for division by zero
- [ ] Test with boundary values (0, 1, MAX)
- [ ] Optimize algorithm complexity
- [ ] Consider memoization for repeated calculations
- [ ] Validate inputs (negative numbers, zero)
- [ ] Document mathematical formulas used
- [ ] Include performance benchmarks
- [ ] Test with known mathematical properties