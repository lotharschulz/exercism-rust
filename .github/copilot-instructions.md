# Exercism Rust Track Solutions

This repository contains solutions to Rust exercises from the Exercism programming platform. It serves as both a learning resource and experimentation ground for AI/LLM prompts to solve coding exercises.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Repository Structure
- 65 individual Rust exercise directories (e.g., `hello-world`, `bowling`, `alphametics`)
- Each exercise contains:
  - `src/lib.rs` - Main implementation file (may contain `todo!()` macros to implement)
  - `tests/` - Test files (some tests may be `#[ignore]` and need to be enabled)
  - `Cargo.toml` - Package configuration
  - `README.md` - Exercise description
  - `HELP.md` - Exercism platform help
- No workspace configuration - each exercise is independent

### Environment Setup
Rust and Cargo are pre-installed:
- Rust: 1.89.0
- Cargo: 1.89.0
- All standard Rust toolchain commands are available

### Build and Test Commands
Always run commands from within an exercise directory:

**Single Exercise Testing:**
```bash
cd <exercise-directory>
cargo test
```
- Time: 0.1-16 seconds per exercise
- NEVER CANCEL: Some exercises (alphametics, palindrome-products) take 15-16 seconds due to computational complexity
- Set timeout to 30+ seconds for individual tests

**Code Quality Checks:**
```bash
cargo fmt --all              # Format code (0.1-0.4 seconds)
cargo fmt --all -- --check   # Check formatting without changing files
cargo clippy                 # Lint code (0.5-0.7 seconds)  
cargo check                  # Fast compilation check (0.1-0.5 seconds)
```

**Full Repository Testing:**
```bash
# Test all exercises (from repository root)
for dir in $(find . -maxdepth 1 -type d -name "*" | grep -v "^\.$" | grep -v "^\.git" | sort); do
    echo "Testing $dir"
    (cd "$dir" && cargo test) || echo "Failed: $dir"
done
```
- Time: ~62 seconds for all 65 exercises
- NEVER CANCEL: Full test suite takes approximately 1 minute. Set timeout to 90+ seconds.

## Validation

### Standard Workflow
When working on exercises:
1. Navigate to the exercise directory: `cd <exercise-name>`
2. Run initial tests to see current state: `cargo test`
3. Implement TODOs in `src/lib.rs` if present
4. Enable ignored tests by removing `#[ignore]` attributes in test files
5. Run tests frequently: `cargo test` 
6. Format code: `cargo fmt --all`
7. Check with linter: `cargo clippy`
8. Final validation: `cargo test && cargo fmt --all -- --check && cargo clippy`

### Exercise States
Exercises may be in different states:
- **Complete**: All tests pass, no TODOs
- **Incomplete**: Contains `todo!()` macros in `src/lib.rs`
- **Partial**: Some tests ignored with `#[ignore]` attribute
- **Failing**: Implementation exists but tests fail (needs debugging)

### Common Issues
Some exercises are known to have failing tests:
- `pig-latin`: 4 failing tests related to specific phonetic rules
- `wordy`: Contains unimplemented `todo!()` macro

These are normal and represent exercises in various stages of completion.

### Manual Testing
Always exercise the implemented functionality:
- For simple exercises: Verify the function returns expected output
- For complex exercises: Test edge cases and performance
- Use `cargo test <test_name>` to run specific tests
- Use `cargo test -- --include-ignored` to run all tests including ignored ones

## Timing Expectations and Warnings

**Individual Operations:**
- `cargo check`: 0.1-0.5 seconds - NEVER CANCEL, set 10+ second timeout
- `cargo test`: 0.1-16 seconds - NEVER CANCEL, set 30+ second timeout  
- `cargo fmt`: 0.1-0.4 seconds - NEVER CANCEL, set 10+ second timeout
- `cargo clippy`: 0.5-0.7 seconds - NEVER CANCEL, set 15+ second timeout

**Computationally Intensive Exercises:**
- `alphametics`: 15-16 seconds for tests
- `palindrome-products`: 15-16 seconds for tests
- NEVER CANCEL these - they are solving complex mathematical problems

**Full Repository Operations:**
- All exercise tests: ~62 seconds - NEVER CANCEL, set 90+ second timeout
- All exercise formatting: ~30 seconds - NEVER CANCEL, set 60+ second timeout

## Key Projects and Focus Areas

### Exercise Categories
- **Basic**: `hello-world`, `leap`, `raindrops`, `reverse-string`
- **String Processing**: `acronym`, `pig-latin`, `pangram`, `isogram`
- **Algorithms**: `binary-search`, `sieve`, `nth-prime`, `alphametics`
- **Data Structures**: `bowling`, `clock`, `robot-simulator`, `simple-linked-list`
- **Mathematics**: `grains`, `prime-factors`, `perfect-numbers`, `armstrong-numbers`

### AI/LLM Experimentation
The repository includes examples of using different AI models (GPT-4, Claude, Gemini) to solve exercises. Reference commit history for examples of successful AI-generated solutions.

## Common Commands Reference

### Navigation
```bash
# List all exercises
ls -1 | grep -v -E "^\.|README" | head -10

# Find exercises with TODOs
find . -name "*.rs" -exec grep -l "todo!" {} \;

# Find exercises with ignored tests  
find . -name "*.rs" -exec grep -l "#\[ignore\]" {} \;
```

### Testing Patterns
```bash
# Test specific exercise
cd hello-world && cargo test

# Test with verbose output
cargo test -- --nocapture

# Test only non-ignored tests
cargo test

# Test including ignored tests
cargo test -- --include-ignored

# Test specific test function
cargo test test_name
```

### Development Workflow
```bash
# Complete development cycle for an exercise
cd <exercise-name>
cargo test                    # See initial state
# Edit src/lib.rs and test files
cargo test                    # Verify implementation
cargo fmt --all              # Format code
cargo clippy                 # Check for issues
cargo test                    # Final verification
```

Always run the complete validation cycle before considering an exercise complete. The goal is working, well-formatted, lint-free code that passes all tests.