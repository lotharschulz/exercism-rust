# Agents Configuration

## Test Runner Agent
- Purpose: Automatically run tests after code changes
- Trigger: On file save in src/
- Command: cargo test

## Formatter Agent
- Purpose: Format code according to rustfmt
- Trigger: Before commit
- Command: cargo fmt

## Benchmark Agent
- Purpose: Run performance benchmarks
- Trigger: Manual or after optimization
- Command: cargo bench