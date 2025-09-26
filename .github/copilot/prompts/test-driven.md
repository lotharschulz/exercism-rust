# Test-Driven Development Prompt

Adopt a TDD loop for this exercise.

## Loop Structure
1. Pick the smallest unimplemented requirement.
2. Write or enable (un-ignore) a focused test expressing the requirement.
3. Run `cargo test` to see it fail (RED).
4. Implement the minimal code to satisfy the test (GREEN).
5. Refactor for readability / idioms (REFACTOR).

Repeat until all requirements are satisfied and no ignored tests remain.

## Additional Guidelines
- Keep functions small and cohesive.
- Extract helper functions when duplication appears 2+ times.
- Preserve test clarity; do not over-generalize early.

## Exit Criteria
- All tests pass.
- No ignored tests.
- Clippy clean, formatted.
- Code reviewed for clarity and idiomatic style.
