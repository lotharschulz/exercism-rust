# Rust Idioms Refinement Prompt

Refactor the existing solution toward idiomatic Rust.

## Targets
- Prefer iterators over indexed loops when clearer.
- Use pattern matching instead of chains of if/else where it improves clarity.
- Replace manual error handling with `?` where suitable.
- Remove dead code / unused imports.
- Simplify types (e.g., &str vs String where ownership not needed).
- Use `From` / `Into` / `AsRef` traits for ergonomic conversions when helpful.
- Add doc comments (///) for public items.
- Prefer functional coding style

## Do Not
- Introduce complexity for stylistic purity.
- Use unsafe.
- Over-abstract small problems.

## Deliverable
Provide a bullet list of refactors + rationale.
