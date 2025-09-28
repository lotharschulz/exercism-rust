# Claude Code Prompts for Exercism Rust

## Standard Exercise Implementation

### Basic Implementation
"Implement the [EXERCISE] exercise following TDD:
1. Read README.md for requirements
2. Write minimal passing implementation
3. Enable all tests
4. Refactor for idiomatic Rust
5. Ensure clippy and fmt pass"

### Full Workflow
"Complete the [EXERCISE] exercise with full workflow:
1. Navigate to exercise directory
2. Analyze README.md and test requirements
3. Implement solution in src/lib.rs
4. Remove #[ignore] from all tests
5. Run cargo test iteratively until all pass
6. Run cargo fmt --all
7. Run cargo clippy -- -W clippy::pedantic
8. Add documentation comments
9. Verify with cargo test --release
10. Final tests with cargo test && cargo fmt --all -- --check && cargo clippy -- -W clippy::pedantic"

### Quick Implementation
"Quickly solve [EXERCISE]:
- Read requirements
- Implement working solution
- Enable and pass all tests
- Format code"

## Optimization Requests

### Performance Optimization
"Optimize the current solution for [EXERCISE]:
1. Benchmark current performance with cargo bench
2. Identify bottlenecks (allocations, algorithms)
3. Apply Rust performance idioms:
   - Use iterators instead of loops
   - Avoid unnecessary clones
   - Use &str instead of String where possible
   - Pre-allocate collections with with_capacity
4. Maintain readability
5. Document complexity improvements
6. Re-benchmark and compare"

### Memory Optimization
"Reduce memory usage in [EXERCISE]:
1. Profile current memory usage
2. Replace owned values with borrows where possible
3. Use Cow<'_, str> for conditional ownership
4. Consider using SmallVec for small collections
5. Implement zero-copy parsing if applicable
6. Verify tests still pass"

### Algorithm Optimization
"Improve algorithm efficiency in [EXERCISE]:
1. Analyze current time complexity
2. Research optimal algorithms for this problem
3. Implement improved algorithm
4. Compare benchmark results
5. Document the complexity change"

## Debugging and Fixing

### Debug Failing Tests
"Debug and fix failing tests in [EXERCISE]:
1. Run cargo test -- --nocapture to see output
2. Add dbg!() statements for inspection
3. Check edge cases:
   - Empty input
   - Single element
   - Maximum values
   - Unicode/UTF-8 for strings
4. Verify type signatures match test expectations
5. Fix issues and re-run tests
6. Remove debug statements"

### Fix Compilation Errors
"Fix compilation errors in [EXERCISE]:
1. Run cargo check to see all errors
2. Fix type mismatches
3. Add missing lifetime annotations
4. Implement required traits
5. Handle all match arms
6. Resolve borrow checker issues"

### Fix Clippy Warnings
"Resolve all clippy warnings in [EXERCISE]:
1. Run cargo clippy
2. Fix each warning:
   - Replace .ok().expect() with .expect()
   - Use if let instead of match for single pattern
   - Remove redundant clones
   - Simplify boolean expressions
3. Run cargo clippy -- -W clippy::pedantic
4. Fix pedantic warnings where reasonable
5. Verify tests still pass"

## Documentation Tasks

### Add Comprehensive Documentation
"Add documentation to [EXERCISE]:
1. Add module-level documentation:
   ```rust
   //! Solution for [EXERCISE] exercise
   //! 
   //! This module implements...
   ```
2. Document all public functions:
   ```rust
   /// Brief description
   /// 
   /// # Arguments
   /// * `input` - Description
   /// 
   /// # Returns
   /// Description of return value
   /// 
   /// # Examples
   /// ```
   /// use crate::function;
   /// assert_eq!(function(input), expected);
   /// ```
   /// 
   /// # Panics
   /// Panics if...
   /// 
   /// # Errors
   /// Returns `Err` if...
   ```
3. Add inline comments for complex logic
4. Generate docs with cargo doc
5. Verify examples compile with cargo test --doc"

### Add Examples
"Add usage examples to [EXERCISE]:
1. Add examples in doc comments
2. Create examples/ directory if needed
3. Write practical usage examples
4. Ensure examples are tested
5. Include edge case examples"

## Refactoring

### Refactor for Idioms
"Refactor [EXERCISE] to be more idiomatic:
1. Replace loops with iterator chains
2. Use pattern matching instead of if-else chains
3. Implement standard traits (From, Into, Display)
4. Apply RAII patterns
5. Use type system effectively (NewType, PhantomData)
6. Extract magic numbers as constants
7. Use type aliases for complex types"

### Extract Generic Implementation
"Make [EXERCISE] generic:
1. Identify type-specific code
2. Add generic type parameters
3. Add necessary trait bounds
4. Update function signatures
5. Ensure tests work with generic implementation
6. Add tests for multiple types"

### Simplify Code
"Simplify the implementation of [EXERCISE]:
1. Remove unnecessary complexity
2. Combine similar functions
3. Eliminate redundant variables
4. Simplify nested conditions
5. Use standard library functions where applicable"

## Testing Enhancements

### Add Property-Based Tests
"Add property-based tests to [EXERCISE]:
1. Add quickcheck or proptest dependency
2. Identify properties to test
3. Implement property tests:
   ```rust
   #[quickcheck]
   fn prop_reversing_twice_is_identity(input: String) -> bool {
       reverse(&reverse(&input)) == input
   }
   ```
4. Run with many random inputs
5. Fix any discovered issues"

### Add Benchmark Tests
"Add benchmarks to [EXERCISE]:
1. Add benches/ directory
2. Create benchmark file:
   ```rust
   #![feature(test)]
   extern crate test;
   use test::Bencher;
   
   #[bench]
   fn bench_solution(b: &mut Bencher) {
       b.iter(|| function(test_input()));
   }
   ```
3. Run cargo bench
4. Document performance characteristics"

### Add Edge Case Tests
"Add edge case tests to [EXERCISE]:
1. Test empty input
2. Test single element
3. Test maximum/minimum values
4. Test Unicode strings (if applicable)
5. Test boundary conditions
6. Test error cases"

## Specific Exercise Patterns

### String Exercise
"Implement [STRING_EXERCISE] with Unicode support:
1. Use .chars() for character iteration
2. Handle UTF-8 properly
3. Consider grapheme clusters if needed
4. Avoid string allocation where possible
5. Test with emoji and non-ASCII characters"

### Collection Exercise
"Implement [COLLECTION_EXERCISE] efficiently:
1. Use iterator methods
2. Avoid intermediate collections
3. Consider lazy evaluation
4. Pre-allocate with capacity when size is known
5. Use appropriate collection type (Vec, HashSet, etc.)"

### Math Exercise
"Implement [MATH_EXERCISE] with proper numerics:
1. Choose appropriate integer types (u32, i64, etc.)
2. Handle overflow with checked_* methods
3. Consider floating-point precision issues
4. Implement efficient algorithms (e.g., Sieve for primes)
5. Add tests for boundary values"

### Parsing Exercise
"Implement [PARSING_EXERCISE] robustly:
1. Return Result for fallible parsing
2. Provide clear error messages
3. Handle malformed input gracefully
4. Consider using nom or regex for complex parsing
5. Test with invalid inputs"

## Multi-Exercise Tasks

### Implement Multiple Exercises
"Implement all exercises in [CATEGORY]:
1. List all exercises in category
2. For each exercise:
   - Navigate to directory
   - Implement solution
   - Pass all tests
   - Format and lint
3. Create summary of approaches used
4. Identify common patterns"

### Update All Exercises
"Update all exercises to latest Rust edition:
1. Update Cargo.toml edition field
2. Run cargo fix --edition
3. Update to new syntax/features
4. Ensure all tests pass
5. Update documentation"

## Special Commands

### Create Exercise From Scratch
"Create a new exercise called [NAME]:
1. Create directory structure
2. Create Cargo.toml
3. Create src/lib.rs with stub
4. Create tests/[NAME].rs
5. Create README.md
6. Implement solution"

### Compare Solutions
"Compare different approaches for [EXERCISE]:
1. Implement iterative version
2. Implement recursive version
3. Implement functional version
4. Benchmark all versions
5. Document trade-offs"

### Explain Current Implementation
"Explain the current implementation of [EXERCISE]:
1. Describe the algorithm used
2. Explain time/space complexity
3. Identify key Rust features used
4. Discuss trade-offs made
5. Suggest possible improvements"

## Quick Commands

### One-Liners
- "Solve [EXERCISE]"
- "Fix tests in [EXERCISE]"
- "Optimize [EXERCISE]"
- "Document [EXERCISE]"
- "Refactor [EXERCISE]"
- "Debug [EXERCISE]"
- "Benchmark [EXERCISE]"
- "Format all exercises"
- "Run all tests"
- "Check all exercises with clippy"

## Workflow Combinations

### Full Development Cycle
"Complete full development cycle for [EXERCISE]:
1. Implement using TDD
2. Optimize for performance
3. Add comprehensive documentation
4. Add property-based tests
5. Add benchmarks
6. Create alternative implementations
7. Write comparison report"

### Competition-Style Implementation
"Implement [EXERCISE] competition-style:
1. Focus on passing tests quickly
2. Optimize for runtime speed
3. Minimize memory usage
4. Keep code concise
5. Skip extensive documentation"

### Learning-Focused Implementation
"Implement [EXERCISE] for learning:
1. Start with naive approach
2. Add extensive comments explaining logic
3. Implement multiple versions
4. Compare different approaches
5. Document what was learned
6. Add links to relevant Rust documentation"