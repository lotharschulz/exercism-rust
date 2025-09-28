# Instructions for Exercism Rust Project

## Quick Start Guide

### For AI Assistants (Claude Code, Cursor, Copilot)
To solve a new exercise, say: "Implement the [exercise-name] exercise"

Example commands:
- "Implement the accumulate exercise"
- "Solve reverse-string with optimization"
- "Fix failing tests in nth-prime"
- "Refactor all exercises to use iterators"

### For Human Developers
1. Choose an exercise from the list below
2. Navigate to its directory: `cd exercise-name/`
3. Read the README.md for requirements
4. Implement solution in `src/lib.rs`
5. Run tests with `cargo test`
6. Enable ignored tests by removing `#[ignore]`
7. Iterate until all tests pass

## Project Structure

```
exercism-rust/
├── .github/
│   ├── copilot-instructions.md  # GitHub Copilot config
│   └── prompts/                 # Reusable prompts
├── .claude/
│   ├── config.json              # Claude Code config (if supported)
│   ├── prompts.md               # Pre-defined prompts
│   └── exercises/               # Exercise-specific patterns
├── exercise-name/
│   ├── src/
│   │   └── lib.rs              # Your implementation goes here
│   ├── tests/
│   │   └── exercise-name.rs    # Test suite (don't modify)
│   ├── Cargo.toml              # Dependencies and metadata
│   └── README.md               # Exercise description
├── CLAUDE.md                   # Claude Code instructions
├── .instructions.md            # This file
└── README.md                   # Project overview
```

## Development Workflow

### Standard Implementation Process

1. **Understand Requirements**
   ```bash
   cd exercise-name/
   cat README.md
   cargo test  # See what needs to be implemented
   ```

2. **Implement Solution**
   ```bash
   $EDITOR src/lib.rs
   # Write your implementation
   ```

3. **Enable Tests**
   ```bash
   # Remove #[ignore] attributes
   sed -i 's/#\[ignore\]//g' tests/*.rs
   ```

4. **Test Driven Development**
   ```bash
   cargo test                   # Run all tests
   cargo test test_specific     # Run specific test
   cargo test -- --nocapture    # See println! output
   ```

5. **Code Quality**
   ```bash
   cargo fmt                            # Format code
   cargo clippy                         # Basic linting
   cargo clippy -- -W clippy::pedantic  # Strict linting
   ```

6. **Documentation**
   ```bash
   cargo doc --open  # Generate and view documentation
   cargo test --doc  # Test documentation examples
   ```

## Available Commands

### Essential Commands
```bash
cargo test                  # Run tests
cargo fmt                   # Format code
cargo clippy                # Lint code
cargo check                 # Quick compilation check
cargo build --release       # Optimized build
cargo doc                   # Generate documentation
```

### Advanced Commands
```bash
cargo bench                 # Run benchmarks
cargo test -- --ignored     # Run only ignored tests
cargo expand                # Expand macros (requires cargo-expand)
cargo tree                  # Show dependency tree
cargo audit                 # Check for security vulnerabilities
```

### Useful Aliases
Add these to your shell configuration:
```bash
alias ct='cargo test'
alias cf='cargo fmt'
alias cc='cargo clippy -- -W clippy::pedantic'
alias cta='cargo test && cargo fmt && cargo clippy'
alias ctv='cargo test -- --nocapture'
```

## Exercise Categories

### String Manipulation
- `reverse-string` - Reverse a string (Unicode aware)
- `acronym` - Generate acronyms from phrases
- `bob` - Teenager response generator
- `beer-song` - Generate the beer song lyrics
- `raindrops` - Raindrops game (Pling, Plang, Plong)

### Collections & Iterators
- `accumulate` - Implement map function
- `flatten-array` - Flatten nested arrays
- `sublist` - Determine list relationships
- `grade-school` - Student grade management

### Math & Algorithms
- `nth-prime` - Find the nth prime number
- `collatz-conjecture` - Calculate Collatz sequence length
- `prime-factors` - Prime factorization
- `sum-of-multiples` - Sum multiples below a number
- `perfect-numbers` - Classify perfect numbers

### Parsing & Validation
- `phone-number` - Parse and validate phone numbers
- `roman-numerals` - Convert to/from Roman numerals
- `allergies` - Allergy scoring system
- `isbn-verifier` - Validate ISBN-10 numbers

### Data Structures
- `linked-list` - Implement a doubly linked list
- `binary-search` - Implement binary search
- `robot-simulator` - Robot movement simulation
- `circular-buffer` - Implement circular buffer

### Concurrency (Advanced)
- `parallel-letter-frequency` - Parallel character counting
- `bank-account` - Thread-safe bank operations

## Rust-Specific Guidelines

### Memory Management
- Prefer borrowing (`&T`) over cloning
- Use `String` for owned strings, `&str` for borrowed
- Allocate with capacity when size is known: `Vec::with_capacity(n)`
- Consider `Cow<'_, str>` for conditional ownership

### Error Handling
```rust
// Good: Return Result for fallible operations
pub fn parse(input: &str) -> Result<Value, ParseError> {
    // ...
}

// Bad: Panic in library code
pub fn parse(input: &str) -> Value {
    input.parse().unwrap()  // Don't do this!
}
```

### Iterators vs Loops
```rust
// Preferred: Iterator chains
let result: Vec<_> = items
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect();

// Avoid: Manual loops
let mut result = Vec::new();
for item in items {
    if item.is_valid() {
        result.push(item.transform());
    }
}
```

### Pattern Matching
```rust
// Use exhaustive matching
match value {
    Some(x) if x > 0 => process_positive(x),
    Some(x) => process_negative(x),
    None => handle_none(),
}

// Use if-let for single patterns
if let Some(x) = optional_value {
    process(x);
}
```

## Testing Best Practices

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Test the happy path
    }

    #[test]
    fn test_edge_cases() {
        // Test boundaries and special cases
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_conditions() {
        // Test error handling
    }
}
```

### Property-Based Testing
```rust
#[cfg(test)]
mod properties {
    use quickcheck::quickcheck;
    
    quickcheck! {
        fn prop_reverse_twice_is_identity(xs: Vec<i32>) -> bool {
            reverse(&reverse(&xs)) == xs
        }
    }
}
```

## Performance Optimization

### Benchmarking
```rust
#![feature(test)]
extern crate test;

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input = prepare_input();
        b.iter(|| solution(&input));
    }
}
```

### Common Optimizations
1. Use `&str` instead of `String` for parameters
2. Avoid unnecessary allocations
3. Pre-allocate collections when possible
4. Use `lazy_static` for compile-time constants
5. Consider `SmallVec` for small collections
6. Use appropriate data structures (HashMap vs BTreeMap)

## Documentation Standards

### Function Documentation
```rust
/// Calculates the nth prime number.
///
/// # Arguments
/// * `n` - The index of the prime to find (0-based)
///
/// # Returns
/// The nth prime number
///
/// # Examples
/// ```
/// use exercism::nth_prime;
/// 
/// assert_eq!(nth_prime(0), 2);
/// assert_eq!(nth_prime(5), 13);
/// ```
///
/// # Panics
/// Panics if n is greater than 10,000
pub fn nth_prime(n: u32) -> u32 {
    // Implementation
}
```

## Troubleshooting

### Common Issues and Solutions

#### Compilation Errors
- **Lifetime errors**: Add explicit lifetime annotations
- **Type mismatch**: Check test expectations vs implementation
- **Trait not implemented**: Add required trait implementations

#### Test Failures
- **Off-by-one errors**: Check loop bounds and indices
- **UTF-8 issues**: Use `.chars()` instead of `.bytes()` for strings
- **Empty input**: Handle edge cases explicitly

#### Performance Issues
- **Slow tests**: Use `--release` flag for optimized builds
- **Memory usage**: Profile with `valgrind` or `heaptrack`
- **Algorithm complexity**: Review Big-O complexity

## Contributing

### Adding New Exercises
1. Create directory structure
2. Add Cargo.toml with dependencies
3. Create stub in src/lib.rs
4. Add comprehensive tests
5. Write clear README.md
6. Test the exercise setup

### Improving Existing Solutions
1. Ensure all tests still pass
2. Improve performance if possible
3. Add documentation
4. Consider generic implementations
5. Add property-based tests

## Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

### Exercism Specific
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Rust Track Syllabus](https://exercism.org/tracks/rust/concepts)
- [Mentoring Notes](https://exercism.org/docs/mentoring)

### Tools and Utilities
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Rustfmt Configuration](https://rust-lang.github.io/rustfmt/)

## AI Assistant Integration

### For Claude Code
- Primary config: `CLAUDE.md` in repository root
- Prompts library: `.claude/prompts.md`
- Say: "Follow CLAUDE.md instructions"

### For GitHub Copilot
- Config: `.github/copilot-instructions.md`
- Use comments to guide suggestions
- Reference this file in issues for Copilot Agent

### For Cursor
- Rules: `.cursor/rules/instructions.md`
- Use Composer (Cmd/Ctrl + Shift + I)
- Reference: `@file:.instructions.md`

## Exercise Completion Checklist

For each exercise, ensure:
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is complete
- [ ] Examples in docs compile
- [ ] Edge cases are handled
- [ ] Performance is reasonable
- [ ] Code is idiomatic Rust

## Quick Reference Card

```bash
# Start new exercise
cd exercise-name && cargo test

# Development cycle
cargo test && cargo fmt && cargo clippy

# Full check
cargo test && cargo fmt && cargo clippy -- -W clippy::pedantic && cargo doc

# Debug test failures
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Benchmark
cargo bench

# View documentation
cargo doc --open
```
