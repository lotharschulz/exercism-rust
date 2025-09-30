## [My](https://exercism.io/profiles/lotharschulz) [exercism rust](https://exercism.org/tracks/rust/exercises/) track solutions

I use this repository also to experiment with AI/LLM prompts to solve the exercises.

## [Claude Code](https://claudecode.io/)

I used implementing the [diamond challenge](https://exercism.org/tracks/rust/exercises/diamond) and the [largest-series-product](https://exercism.org/tracks/rust/exercises/largest-series-product) exercises. The [.claude](https://github.com/lotharschulz/exercism-rust/tree/main/.claude) folder contains instructions that enable resolved exercises using both [concise](https://github.com/lotharschulz/exercism-rust/commit/7de48659e33c3a8416ee872e5f5f1851e3946f86) and [detailed](https://github.com/lotharschulz/exercism-rust/commit/8c4b24b6b67fd7538d4e8bad9cdd5c35a9952bd0) prompts.

## [Copilot coding agent](https://docs.github.com/en/copilot/concepts/coding-agent/coding-agent)

I used implementing the [Crypto Square](https://github.com/lotharschulz/exercism-rust/issues/7) exercise. This issue is notably brief compared to others, as the [.github](https://github.com/lotharschulz/exercism-rust/tree/main/.github) folder already includes Copilot [prompts](https://github.com/lotharschulz/exercism-rust/tree/main/.github/copilot/prompts) and [agent](https://github.com/lotharschulz/exercism-rust/blob/main/.github/copilot/agent.yml) instructions.

An earlier attempt was implementing the [Wordy Exercise](https://github.com/lotharschulz/exercism-rust/issues/1). During this implementation, the Copilot agent detects that no Copilot instructions were present in the repository and flagged this through a [PR comment](https://github.com/lotharschulz/exercism-rust/pull/2#issuecomment-3261479075). This led to the creation of a dedicated issue:: [✨ Set up Copilot instructions](https://github.com/lotharschulz/exercism-rust/issues/3).

With those instructions in place, I proceeded to implement the [custom-set exercise](https://github.com/lotharschulz/exercism-rust/issues/5). Interestingly, the existing Exercism tests and instructions were already so comprehensive and well-structured that adding Copilot instructions didn’t significantly enhance the development experience. In both the Wordy and Custom Set exercises, the quality of the Exercism framework proved to be the dominant factor.

## [Windsurf IDE](https://windsurf.com/editor)

Windsurf IDE successfully solved the [atbash-cipher](https://github.com/lotharschulz/exercism-rust/tree/main/atbash-cipher) exercise using the following prompt:

```
# Code Implementation

Implement all _instructions_ defined in `atbash-cipher/README.md` in the rust file:`atbash-cipher/src/lib.rs`.

## Enable all tests
Ensure all tests in the `atbash-cipher/tests` folder are NOT ignored and enabled. 

## Test the code

Run all tests in the `atbash-cipher/tests` folder, after the implementation of the all _instructions_ in `atbash-cipher/src/lib.rs` is done.


## Ensure all tests pass

Make sure all tests pass successfully. 
In case test(s) fail, adapt the code implementation and rerun all tests until all tests pass.

## Improve

Do better. Improve the code implementation and increase readability and reduce code complexity.
```

## [Cursor IDE](docs.cursor.com)

Cursor IDE successfully solved the [accumulate](https://github.com/lotharschulz/exercism-rust/tree/main/accumulate) exercise using its agent mode with the following structured prompt:

```
# Code Implementation

Implement all _instructions_ defined in `accumulate/README.md` in the rust file:`accumulate/src/lib.rs`.

## Enable all tests
Ensure all tests in the `accumulate/tests` folder are NOT ignored and enabled. 

## Test the code

Run all tests in the `accumulate/tests` folder, after the implementation of the all _instructions_ in `accumulate/src/lib.rs` is done.


## Ensure all tests pass

Make sure all tests pass successfully. 
In case test(s) fail, adapt the code implementation and rerun all tests until all tests pass.

## Improve

Do better. Improve the code implementation and increase readability and reduce code complexity.
```

The result worked and was good code. Cursor ran multiple commands (`cargo` and others) and as a user, I had to confirm every of the requested commands.

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

## [copilot agent mode](https://github.blog/news-insights/product-news/github-copilot-the-agent-awakens/)

### model: GPT 4.1

##### Context

- exercism exercise folder

##### Prompt

```
Implement all TODOs in 'src/lib.rs'.
Ensure all tests in the 'tests' folder are NOT ignored and enabled. 
After the implementation of the all TODOs in 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully. 
In case tests fail, adapt the implementation and continue with running tests and subsequent code implementation in case tests fail until all tests pass.
```

This setup created all changes of [044f6c3](https://github.com/lotharschulz/exercism-rust/commit/044f6c3640b75a14d3108f3160a1150f5c1c1a19) that passed all tests.


### model: Claude 3.7 Sonnet

##### Context

- exercism exercise folder

##### Prompt

```
Implement all TODOs in 'src/lib.rs'.
Ensure all tests in the 'tests' folder are NOT ignored and enabled. 
After the implementation of the all TODOs in 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully.

Run test like so "cargo test --manifest-path Cargo.toml && cargo fmt --all -- --check"

In case tests fail, adapt the implementation and continue with running tests and subsequent code implementation in case tests fail until all tests pass.
```

This setup created all changes of [987c7cc](https://github.com/lotharschulz/exercism-rust/commit/987c7ccf03b71a029a5ae2389c2cb97f57f859b5) that passed all tests and resulted in formatted code.



## [copilot edits](https://code.visualstudio.com/docs/copilot/copilot-edits)

### model: Claude Sonnet 4

##### Context

- exercism exercise folder

##### Prompt

```
Implement all TODOs in 'src/lib.rs'.
Ensure all tests in the 'tests' folder are NOT ignored and enabled.
After the implementation of the all TODOs in 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully.
In case tests fail, adapt the implementation and continue with running tests and subsequent code implementation in case tests fail until all tests pass.
```

This setup created all changes of [98ebce4](https://github.com/lotharschulz/exercism-rust/commit/98ebce4d36b06e36ee0f0fa75367eff968995258) that passed all tests.

### model: GPT 4o

##### Context

 `lib.rs`

##### Prompt

```
Make also sure that all tests in 'tests' folder are active - no tests ignored. 
Implement the todos in libr.rs. 
When implementing make sure all tests in 'tests' folder pass succesfully. 
```

This setup created all changes of [df4e27c](https://github.com/lotharschulz/exercism-rust/commit/df4e27c10cff8a3e650e3c853c51bf72b58b65b0) that passed all tests.

### models: Claude 3.7 Sonnet & Claude 3.7 Sonnet Thinking

##### Context 
- `xxx.rs` test file 
   and 
- `lib.rs` task file

##### Prompt

```
Ensure all tests in the 'tests' folder are active and none are ignored. 
Implement the TODOs in 'src/lib.rs'. 
After the implementation of the TODOs 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully. 
In case tests fail, adapt the implementation.
```

This setup with Claude 3.7 Sonnet created all changes for [7cfd77c](https://github.com/lotharschulz/exercism-rust/commit/7cfd77cd51645e1bfe96fae48ed026856678ab46) that passes all tests.

This setup with Claude 3.7 Sonnet Thinking created all changes of [4d45abe](https://github.com/lotharschulz/exercism-rust/commit/4d45abee64e399763c8b6d23b355ddf996d9872b) that passed all tests.

### model: Claude 3.5 Sonnet

The following prompt with the folder and lib.rs added to the context:

##### Context

`lib.rs` plus the whole exercism task folder 

##### Prompt

```
Ensure all tests in the 'tests' folder are active and none are ignored. 
Implement the TODOs in 'src/lib.rs'. 
After the implementation of the TODOs 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully. 
In case tests fail, adapt the implementation.
```

created all changes of [b091d88](https://github.com/lotharschulz/exercism-rust/commit/b091d88b74cd2a18fc4e581422726b29263bb286) that passed all tests.

### model: Gemini 2.5 Pro (Preview)

- exercism exercise folder

##### Prompt

```
Implement all TODOs in 'src/lib.rs'.
Ensure all tests in the 'tests' folder are NOT ignored and enabled.
After the implementation of the all TODOs in 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully.
In case tests fail, adapt the implementation and continue with running tests and subsequent code implementation in case tests fail until all tests pass.
```

This setup created all changes of [b388327](https://github.com/lotharschulz/exercism-rust/commit/b3883279594d6269e43581729610954d6e9cccd6) that passed all tests.
