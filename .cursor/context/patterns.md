# Common Rust Patterns for Exercism

## Iterator Patterns
```rust
// Collect with type annotation
let numbers: Vec<i32> = input.iter().map(|x| x * 2).collect();

// Filter and map
let valid: Vec<_> = items.iter()
    .filter_map(|x| x.parse().ok())
    .collect();

// Fold for accumulation
let sum = numbers.iter().fold(0, |acc, x| acc + x);

// Partition
let (evens, odds): (Vec<_>, Vec<_>) = numbers.iter()
    .partition(|&&x| x % 2 == 0);