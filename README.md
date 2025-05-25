[My](https://exercism.io/profiles/lotharschulz) [exercism rust](https://exercism.org/tracks/rust/exercises/) track solutions


I use this also to experiment with prompts to solve the tasks

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

This setup created all changes of [044f6c3](https://github.com/lotharschulz/exercism-rust/commit/044f6c3640b75a14d3108f3160a1150f5c1c1a19) that passes all tests.


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

This setup created all changes of [987c7cc](https://github.com/lotharschulz/exercism-rust/commit/987c7ccf03b71a029a5ae2389c2cb97f57f859b5) that passes all tests and resulted in formatted code.



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

This setup created all changes of [98ebce4](https://github.com/lotharschulz/exercism-rust/commit/98ebce4d36b06e36ee0f0fa75367eff968995258) that passes all tests.

### model: GPT 4o

##### Context

 `lib.rs`

##### Prompt

```
Make also sure that all tests in 'tests' folder are active - no tests ignored. 
Implement the todos in libr.rs. 
When implementing make sure all tests in 'tests' folder pass succesfully. 
```

This setup created all changes of [df4e27c](https://github.com/lotharschulz/exercism-rust/commit/df4e27c10cff8a3e650e3c853c51bf72b58b65b0) that passes all tests.

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

This setup with Claude 3.7 Sonnet Thinking created all changes for [4d45abe](https://github.com/lotharschulz/exercism-rust/commit/4d45abee64e399763c8b6d23b355ddf996d9872b) that passes all tests.

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

created all changes of [b091d88](https://github.com/lotharschulz/exercism-rust/commit/b091d88b74cd2a18fc4e581422726b29263bb286) that passes all tests.

### model: Gemini 2.0 Flash - NOT WORKING so far

##### Context 
- `xxx.rs` test file 
- `lib.rs` task file
- the source folder

##### Prompt

```
Perform all changes in the src/lib.rs and 'tests' folder.
Ensure all tests in the 'tests' folder are active and none are ignored. 
Implement the TODOs in 'src/lib.rs'. 
After the implementation of the TODOs 'src/lib.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully. 
In case tests fail, adapt the implementation.
```

This best setup I found so far with Gemini 2.0 Flash created an implementation, however the tests did not pass.
(implementation (https://github.com/lotharschulz/exercism-rust/commit/cc2dc3e70a70fe45799161249a93c0a1ca552b99) done with Claude 3.7)
