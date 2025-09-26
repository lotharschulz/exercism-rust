# Troubleshooting Failing Tests

Use this ONLY after completing the standard solve workflow in `solve-exercise.md`.
If you have not yet: enabled tests, iteratively implemented, and run validation—do that first.

When encountering persistent failing tests, follow these steps to identify and resolve the issues:

## Common Issues

1. **Incorrect Implementation**: Review the logic in your implementation. Ensure that it adheres to the problem requirements outlined in the exercise description.

2. **Test Cases**: Check if the failing tests are based on specific edge cases or conditions that you may have overlooked. Ensure that your implementation handles all scenarios.

3. **Ignored Tests**: Make sure that all relevant tests are enabled. Look for `#[ignore]` attributes in the test files and remove them if necessary.

4. **Dependencies**: Verify that all dependencies in `Cargo.toml` are correctly specified and up to date. Run `cargo update` to ensure you have the latest versions.

## Debugging Steps

1. **Run Tests Verbosely**: Use the command `cargo test -- --nocapture` to see detailed output from the tests. This can help identify where the failure occurs.

2. **Check Test Output**: Review the output of the failing tests to understand what is expected versus what is being returned. This can provide clues on what needs to be fixed.

3. **Use Debugging Tools**: Utilize debugging tools such as `println!` statements to output variable values at different stages of your implementation. This can help trace the flow of data and identify where things go wrong.

4. **Consult Documentation**: Refer to the Rust documentation or the Exercism exercise README for additional context or requirements that may not be immediately clear.

5. **Seek Help**: If you're still stuck, consider reaching out for help. You can ask for assistance on forums, chat groups, or the Exercism community.

## Final Steps

After making changes to your implementation:

- Rerun the tests using `cargo test` to verify that the issues have been resolved.
- Ensure that all tests pass before considering the exercise complete.
- If new issues arise, repeat the troubleshooting process as needed.