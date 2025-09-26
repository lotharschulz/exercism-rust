# Solve Exercise Prompt

Implement all TODOs in `src/lib.rs` for the current Exercism Rust exercise.

Reference: see `shared-core-flow.md` for the canonical minimal sequence.

## Required Steps
1. Read `README.md` in the exercise folder.
2. If any tests are ignored (#[ignore]), remove the attribute.
3. Run `cargo test` and capture failing cases.
4. Implement features iteratively until tests pass.
5. Ensure formatting (`cargo fmt`) and lint cleanliness (`cargo clippy -- -D warnings`).
6. Avoid adding dependencies unless absolutely necessary.
7. Provide a short summary of changes (bullet list) at end (keep concise; validation phase will format final status).

## Constraints
- Prefer clear, idiomatic solutions over micro-optimizations initially.
- Include simple doc comments for public functions.

## Output
Return: implementation summary + next improvement suggestion (choose: idiomatic refinements, documentation polish, or performance exploration).

## Next Steps After Success
1. Run `validate-solution.md` prompt to enforce quality gates.
2. Optionally run `rust-idioms.md` for stylistic improvements.
3. Optionally run `documentation.md` if adding public API docs.
4. For performance-sensitive tasks, proceed to `optimize-solution.md` (add benchmark harness first if meaningful).
