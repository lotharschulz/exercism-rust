# Shared Core Solve Flow (Reference)

Canonical minimal sequence for solving an Exercism Rust exercise.

1. Inspect `README.md` for requirements.
2. Run `cargo test` to observe current failures / TODO footprint.
3. Remove all `#[ignore]` from tests.
4. Implement smallest missing piece (replace one `todo!()` or failing logic region).
5. Re-run `cargo test`.
6. Repeat steps 4–5 until all tests pass.
7. Run `cargo fmt --all`.
8. Run `cargo clippy --all-targets -- -D warnings`.
9. (If enforced) ensure public items have `///` docs.
10. Summarize changes + propose next refinement (idioms / optimization / docs).

Keep functions cohesive, avoid premature optimization, and resist adding dependencies.
