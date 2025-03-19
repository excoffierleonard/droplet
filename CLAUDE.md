# Droplet Project Guidelines

## Build Commands
- Build: `cargo build`
- Run: `cargo run`
- Release build: `cargo build --release`
- Check: `cargo check`

## Test Commands
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests with output: `cargo test -- --nocapture`

## Lint Commands
- Format code: `cargo fmt`
- Lint check: `cargo clippy`
- Fix lints: `cargo clippy --fix`

## Code Style
- Use 4-space indentation
- Follow Rust naming conventions (snake_case for variables/functions, CamelCase for types)
- Group imports by std, external, and internal
- Use meaningful error messages with error propagation (`?` operator)
- Prefer pattern matching over if/else chains
- Document public functions with /// comments
- Prefer Result<T, E> over panics for error handling