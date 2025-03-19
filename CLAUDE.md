# Droplet Project Guidelines

## Build Commands

- Build: `cargo build`
- Run: `cargo run`
- Release build: `cargo build --release`
- Check: `cargo check`
- Docker build: `docker build -t droplet .`
- Docker run: `docker-compose up`

## Test Commands

- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests with output: `cargo test -- --nocapture`
- Run with log output: `RUST_LOG=debug cargo run`

## Lint Commands

- Format code: `cargo fmt`
- Lint check: `cargo clippy`
- Fix lints: `cargo clippy --fix`

## Code Style

- Use 4-space indentation
- Follow Rust naming conventions (snake_case for variables/functions, CamelCase for types)
- Group imports in this order: std, external crates, internal modules
- Use meaningful error messages with error propagation (`?` operator)
- Prefer pattern matching over if/else chains
- Document public functions with /// comments
- Prefer Result<T, E> over panics for error handling
- Use environment variables with dotenvy for configuration
- Apply actix-web middleware consistently (Logger, Compress)
- Keep main.rs focused on application setup, with business logic in lib.rs
- Use cargo-chef for optimized Docker builds