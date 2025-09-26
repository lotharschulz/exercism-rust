# Optimize Solution Prompt

Analyze current implementation for performance hotspots or needless allocations.

## Steps
1. Benchmark (if meaningful) using Criterion (see benchmark template). If no harness exists:
	`.github/scripts/inject_benchmark.sh <exercise-dir> <function>` then run `cargo bench`.
2. Identify critical paths using reasoning (iteration counts, cloning, allocations).
3. Eliminate avoidable clones / allocations.
4. Replace naive loops with iterator adapters when clearer & not slower.
5. Consider algorithmic improvements (e.g., HashSet vs Vec search, precomputation).
6. Re-run tests + benchmarks after each change.
7. Ensure readability is not sacrificed for marginal gains.

## Checklist
- [ ] No needless clones
- [ ] Avoid repeated parsing / computation inside loops
- [ ] Appropriate data structures chosen
- [ ] Benchmarks show neutral or improved performance
- [ ] No regression in correctness

## Output
Summarize: changes, trade-offs, before/after notes (qualitative if no bench numbers).
