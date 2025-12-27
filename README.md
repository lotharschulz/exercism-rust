## [My](https://exercism.io/profiles/lotharschulz) [exercism rust](https://exercism.org/tracks/rust/exercises/) track solutions

I use this repository also to experiment with AI/LLM prompts to solve the exercises.

Prompt within exercis root folder:

```
# Code Implementation

Implement all _instructions_ defined in `README.md` in the rust file:`src/lib.rs`.

## Enable all tests
Ensure all tests in the `tests` folder are NOT ignored and enabled. 

## Test the code

Run all tests in the `tests` folder, after the implementation of the all _instructions_ in `src/lib.rs` is done.


## Ensure all tests pass

Make sure all tests pass successfully. 
In case test(s) fail, adapt the code implementation and rerun all tests until all tests pass.

## Improve

Do better. Improve the code implementation and increase readability and reduce code complexity.
```

## [Claude Code](https://claudecode.io/)

I used implementing the [diamond challenge](https://exercism.org/tracks/rust/exercises/diamond) and the [largest-series-product](https://exercism.org/tracks/rust/exercises/largest-series-product) exercises. The [.claude](https://github.com/lotharschulz/exercism-rust/tree/main/.claude) folder contains instructions that enable resolved exercises.

## [Copilot coding agent](https://docs.github.com/en/copilot/concepts/coding-agent/coding-agent)

I used implementing the [Crypto Square](https://github.com/lotharschulz/exercism-rust/issues/7) exercise. This issue is notably brief compared to others, as the [.github](https://github.com/lotharschulz/exercism-rust/tree/main/.github) folder already includes Copilot [prompts](https://github.com/lotharschulz/exercism-rust/tree/main/.github/copilot/prompts) and [agent](https://github.com/lotharschulz/exercism-rust/blob/main/.github/copilot/agent.yml) instructions.

An earlier attempt was implementing the [Wordy Exercise](https://github.com/lotharschulz/exercism-rust/issues/1). During this implementation, the Copilot agent detects that no Copilot instructions were present in the repository and flagged this through a [PR comment](https://github.com/lotharschulz/exercism-rust/pull/2#issuecomment-3261479075). This led to the creation of a dedicated issue:: [✨ Set up Copilot instructions](https://github.com/lotharschulz/exercism-rust/issues/3).

With those instructions in place, I proceeded to implement the [custom-set exercise](https://github.com/lotharschulz/exercism-rust/issues/5). Interestingly, the existing exercism tests and instructions were already so comprehensive and well-structured that adding Copilot instructions didn’t significantly enhance the development experience. In both the Wordy and Custom Set exercises, the quality of the Exercism framework proved to be the dominant factor.

## [Windsurf IDE](https://windsurf.com/editor)

Windsurf IDE successfully solved the [luhn-from](https://exercism.org/tracks/rust/exercises/luhn-from) and [atbash-cipher](https://github.com/lotharschulz/exercism-rust/tree/main/atbash-cipher) exercise.
Also Windsurf solved the [luhn-trait](https://exercism.org/tracks/rust/exercises/luhn-trait) exercise with a much smaller previous version of the prompt.

## [Cursor IDE](docs.cursor.com)

Cursor IDE successfully solved the [accumulate](https://github.com/lotharschulz/exercism-rust/tree/main/accumulate) and [list-ops](https://exercism.org/tracks/rust/exercises/list-ops) exercise using its agent.

The result worked and was good code, still in one case clippy caused warnings ([ba72c13](https://github.com/lotharschulz/exercism-rust/commit/ba72c13b4d45ca0a7babab5c5212511a9b4d851c)).

A much smaller prompt:
```
Implement all _instructions_ defined in `phone-number/README.md` in the rust file:`phone-number/src/lib.rs`.
```
resolved the [phone-number](https://exercism.org/tracks/rust/exercises/phone-number). Similar to above, the first implementation included clippy warnings ([5c2175a](https://github.com/lotharschulz/exercism-rust/commit/5c2175a46016cbc6ca762ad0b10051ce65d3fd0e))

Cursor ran multiple commands (`cargo` and others) and as a user, I had to confirm every of the requested commands.

#### Execution Process
Cursor's agent mode performed the following steps autonomously:

- Analyzed the exercise requirements from README.md
- Generated the implementation plan
- Implemented the solution in src/lib.rs
- Enabled all tests by removing #[ignore] attributes
- Executed cargo test to verify the solution
- Refactored the code for improved readability
- Re-tested to ensure refactoring didn't break functionality

#### User Interaction

- Approval Required: Each command execution (cargo test, file modifications) required user confirmation
- Transparency: All planned changes were shown before execution
- Control: User maintained full control over the process

## [Gemini CLI](https://blog.google/technology/developers/introducing-gemini-cli-open-source-ai-agent/)

Gemini cli is a terminal coding assitant that can solve exercism exercises: [https://www.lotharschulz.info/2025/06/25/getting-started-with-google-gemini-cli-complete-setup-guide-and-rust-testing-experience/](https://www.lotharschulz.info/2025/06/25/getting-started-with-google-gemini-cli-complete-setup-guide-and-rust-testing-experience/) 

### Benchmark Harness Automation

You can auto-generate a Criterion benchmark scaffold for an exercise:

```
chmod +x .github/scripts/inject_benchmark.sh
.github/scripts/inject_benchmark.sh <exercise-dir> <function-name>
# Example:
.github/scripts/inject_benchmark.sh accumulate accumulate
```

This will:
- Copy the benchmark template
- Insert a benchmark calling the specified function
- Add `criterion` to `[dev-dependencies]` if missing
- Append a `[[bench]]` section

Run benchmarks with:

```
cd <exercise-dir>
cargo bench
```

## Copilot Prompt Taxonomy

| Role | File | Purpose |
|------|------|---------|
| Core implementation | `solve-exercise.md` | Main iterative solve loop (enable tests → implement → iterate) |
| Legacy alias | `implement-exercise.md` | Deprecated; forwards to `solve-exercise.md` |
| Validation | `validate-solution.md` | Post-implementation quality gates (tests, fmt, clippy, docs) |
| Idiomatic refinement | `rust-idioms.md` | Refactor toward clear, idiomatic Rust |
| Performance | `optimize-solution.md` | Benchmark & improve critical paths |
| Documentation | `documentation.md` | Add/verify `///` docs and README improvements |
| Troubleshooting | `troubleshoot-tests.md` | Deep dive when failures persist after solve loop |
| TDD mode | `test-driven.md` | Red → Green → Refactor workflow |
| Shared reference | `shared-core-flow.md` | Minimal canonical sequence (referenced by others) |

### Doc Enforcement
If `agent.yml` sets `metadata.default.enforce_doc_on_public: true`, CI will fail when
public items lack doc comments. To temporarily relax this, set the flag to `false` in
`.github/copilot/agent.yml` (not recommended for long-term hygiene).

### Optimization Benchmarks
The optimize prompt assumes (but does not force) criterion usage. Use the injector script
to scaffold when exploring performance changes:

```
.github/scripts/inject_benchmark.sh <exercise> <function>
```