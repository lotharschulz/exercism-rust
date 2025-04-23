pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // If upper_bound is less than 2, there are no primes to return
    if upper_bound < 2 {
        return vec![];
    }

    // Create a vector to track which numbers are prime
    // Initially assume all numbers from 2 to upper_bound are prime
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    
    // 0 and 1 are not prime
    is_prime[0] = false;
    is_prime[1] = false;
    
    // Apply the Sieve of Eratosthenes algorithm
    // For each number from 2 to sqrt(upper_bound)
    let sqrt_bound = (upper_bound as f64).sqrt() as u64 + 1;
    for i in 2..=sqrt_bound {
        if is_prime[i as usize] {
            // Mark all multiples of i as not prime, starting from i²
            let mut j = i * i;
            while j <= upper_bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    
    // Collect all the prime numbers into a Vec
    (2..=upper_bound)
        .filter(|&i| is_prime[i as usize])
        .collect()
}
