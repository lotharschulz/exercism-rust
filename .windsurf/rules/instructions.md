# Windsurf Instructions for Exercism Rust

## Project Context
Rust solutions for Exercism exercises using idiomatic patterns and TDD.

## Cascade AI Behavior
- Analyze test requirements before implementing
- Use iterator chains over loops
- Handle errors with Result<T, E>
- Apply clippy pedantic linting
- Generate comprehensive documentation

## Development Flow
1. Read exercise README.md
2. Implement minimal passing solution
3. Enable all tests (remove #[ignore])
4. Refactor for idiomatic Rust
5. Optimize for readability

## Rust Patterns
- Prefer borrowing over cloning
- Use pattern matching exhaustively
- Implement standard traits (Debug, Clone, PartialEq)
- Follow Rust API Guidelines