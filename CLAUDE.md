# Claude Code Instructions for Exercism Rust

## Project Overview
This repository contains Rust solutions for Exercism exercises. Your role is to implement clean, idiomatic Rust solutions following test-driven development principles.

## Core Workflow for New Exercises

When asked to solve an exercise (e.g., "solve accumulate"):

1. **Navigate to exercise directory**
   ```bash
   cd accumulate/
   ```

2. **Read and analyze requirements**
   - Parse `README.md` for specifications
   - Examine test file in `tests/` directory
   - Identify input/output types from test signatures

3. **Implement solution in `src/lib.rs`**
   - Start with minimal working implementation
   - Focus on passing tests first, optimization second

4. **Enable and run tests**
   ```bash
   # Remove #[ignore] attributes from tests
   sed -i 's/#\[ignore\]//g' tests/*.rs
   
   # Run tests
   cargo test
   ```

5. **Iterate until all tests pass**
   - Fix failures one by one
   - Re-run tests after each change

6. **Polish the solution**
   ```bash
   cargo fmt
   cargo clippy -- -W clippy::pedantic
   ```

## Rust Implementation Guidelines

### Must Follow
- Use iterator methods over manual loops
- Prefer `&str` over `String` for parameters
- Handle errors with `Result<T, E>` instead of `panic!`
- Implement `From` traits for type conversions
- Use pattern matching exhaustively

### Code Patterns

```rust
// Prefer functional style
input.iter()
    .filter(|&x| predicate(x))
    .map(transform)
    .collect()

// Over imperative
let mut result = Vec::new();
for x in input {
    if predicate(x) {
        result.push(transform(x));
    }
}
```

### Memory Efficiency
- Borrow when possible: `&T` over `T`
- Use `Cow<'_, str>` for flexible string handling
- Allocate with capacity: `Vec::with_capacity(n)`
- Return iterators when appropriate

## Exercise-Specific Patterns

### String Exercises
```rust
// Unicode-aware operations
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
```

### Collection Exercises
```rust
// Generic implementations
pub fn map<T, F, U>(input: Vec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    input.into_iter().map(f).collect()
}
```

### Math Exercises
```rust
// Use appropriate integer types
pub fn nth(n: u32) -> u32 {
    // Consider overflow with checked_* methods
    (2..)
        .filter(|&x| is_prime(x))
        .nth(n as usize)
        .expect("prime exists")
}
```

## Testing Strategy

1. **Run specific test first**
   ```bash
   cargo test test_name
   ```

2. **Then run all tests**
   ```bash
   cargo test
   ```

3. **Add edge case tests**
   ```rust
   #[test]
   fn test_empty_input() {
       assert_eq!(function(""), expected);
   }
   ```

## Documentation Requirements

Every public function needs:
```rust
/// Brief description
/// 
/// # Examples
/// ```
/// use crate::module;
/// assert_eq!(module::function(input), expected);
/// ```
/// 
/// # Panics
/// Panics if [condition]
/// 
/// # Errors
/// Returns `Err` if [condition]
pub fn function_name() -> Result<T, E> {
    // implementation
}
```

## Command Sequences

### Full implementation sequence:
```bash
# Navigate to exercise
cd exercise-name/

# Implement solution
$EDITOR src/lib.rs

# Enable tests
sed -i 's/#\[ignore\]//g' tests/*.rs

# Test cycle
cargo test
cargo fmt --all -- --check
cargo clippy -- -W clippy::pedantic

# Final verification
cargo test --release
```

## Error Handling

When encountering common issues:

### Compilation Errors
- Check type signatures match test expectations
- Verify lifetime annotations
- Ensure trait bounds are satisfied

### Test Failures
- Print debug information: `dbg!(&value);`
- Use `cargo test -- --nocapture` for output
- Check edge cases (empty, single element, maximum values)

### Performance Issues
- Profile with `cargo bench`
- Use `--release` flag for optimization
- Consider algorithmic complexity

## Continuous Improvement

After passing all tests:
1. Refactor for clarity
2. Extract magic numbers as constants
3. Add type aliases for complex types
4. Consider generic implementations
5. Add benchmarks for performance-critical code

## DO NOT

- Use `unsafe` rust keyword
- Use `unwrap()` in production code (use `expect()` with message or proper error handling)
- Ignore compiler warnings
- Leave commented-out code
- Use `clone()` unnecessarily
- Forget to handle UTF-8 in string operations

## Success Criteria

A solution is complete when:
- [ ] All tests pass
- [ ] `cargo fmt` produces no changes
- [ ] `cargo clippy` shows no warnings
- [ ] Documentation is complete
- [ ] Code is idiomatic Rust