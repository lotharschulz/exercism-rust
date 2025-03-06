[My](https://exercism.io/profiles/lotharschulz) [exercism rust](https://exercism.org/tracks/rust/exercises/) track Solutions


I use this also to experiment with prompts to solve the tasks

## [copilot edits](https://code.visualstudio.com/docs/copilot/copilot-edits)

#### model: GPT 4o

Context: `lib.rs`
Prompt:

```
Make also sure that all tests in 'tests' folder are active - no tests ignored. 
Implement the todos in libr.rs. 
When implementing make sure all tests in 'tests' folder pass succesfully. 
```

This setup created all changes for [df4e27c](https://github.com/lotharschulz/exercism-rust/commit/df4e27c10cff8a3e650e3c853c51bf72b58b65b0) that passes all tests.

#### model: Claude 3.5 Sonnet (Preview)

The following prompt with the folder and libr.rs added to the context:

Context: `lib.rs` plus the whole exercism task folder 
Prompt:

```
Ensure all tests in the 'tests' folder are active and none are ignored. 
Implement the TODOs in 'src/libr.rs'. 
After the implementation of the TODOs 'src/libr.rs', run the tests in the 'tests' folder.
Make sure all tests in the 'tests' folder pass successfully. 
In case tests fail, adapt the implementation.
```

This setup created all changes for [b091d88](https://github.com/lotharschulz/exercism-rust/commit/b091d88b74cd2a18fc4e581422726b29263bb286) that passes all tests.
