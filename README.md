## [My](https://exercism.io/profiles/lotharschulz) [exercism rust](https://exercism.org/tracks/rust/exercises/) track solutions

I use this repository also to experiment with AI/LLM prompts to solve the exercises.

## [Copilot coding agent](https://docs.github.com/en/copilot/concepts/coding-agent/coding-agent)

### model GPT 5

#### Context

I exclusively used GitHub Issues for this project. The workflow began with implementing the [Implement Wordy Exercise](https://github.com/lotharschulz/exercism-rust/issues/1). During this implementation, the Copilot agent detected that no Copilot instructions were present in the repository and flagged this through a [PR comment](https://github.com/lotharschulz/exercism-rust/pull/2#issuecomment-3261479075). This discovery prompted me to set up Copilot instructions as a separate issue: [✨ Set up Copilot instructions](https://github.com/lotharschulz/exercism-rust/issues/3).
With the Copilot instructions now in place, I proceeded to implement the [implement custom-set exercise](https://github.com/lotharschulz/exercism-rust/issues/5). Interestingly, the Exercism tests and instructions were already so comprehensive and well-structured that adding Copilot instructions didn't yield a significantly improved development experience compared to the initial Wordy Exercise implementation. The quality of the existing Exercism framework proved to be the dominant factor in both cases.

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
