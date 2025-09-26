## Validation Checklist (Post-Implementation)

Use this after completing the core development loop (`solve-exercise.md`). Do NOT re-implement here.

### Commands to Run
1. `cargo test` (expect: all green, no ignored tests)
2. `cargo fmt --all -- --check` (expect: no diffs)
3. `cargo clippy --all-targets -- -D warnings` (expect: clean)
4. (Optional) `cargo doc --no-deps` (expect: zero warnings if docs enforced)

### Pass Criteria
- All tests pass (including previously ignored ones).
- No `#[ignore]` left in any test file.
- No Clippy warnings.
- Code formatted.
- No stray `dbg!` / `println!` left in solution logic.

### If Failing
- For test failures: re-enter solve loop.
- For style/lint failures: fix minimal surface first, then reconsider refactors.
- For persistent doc gaps (if enforced): add concise `///` doc comments, not prose novels.

### Output
Provide a short summary block:
```
Status: PASS
Tests: <n passed>
Clippy: clean
Docs enforced: <yes/no>
Next suggestion: <small idiom/perf/doc improvement>
```

Avoid re-describing implementation details here; focus on final readiness.