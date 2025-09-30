# Debug Failing Tests

Help debug and fix the failing tests:

## Debugging Process
1. Run tests with output: cargo test -- --nocapture
2. Identify failing assertions
3. Add debug prints: dbg!(&value)
4. Check edge cases:
   - Empty input
   - Single element
   - Maximum values
   - Unicode/UTF-8 issues
5. Verify type signatures
6. Fix issues iteratively
7. Clean up debug code

## Common Issues to Check
- Off-by-one errors
- Integer overflow
- Lifetime issues
- Borrow checker violations
- Pattern matching exhaustiveness

Provide clear explanation of the issue and fix.