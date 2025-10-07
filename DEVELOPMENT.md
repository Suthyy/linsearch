# Development Guide

This guide covers development workflows and best practices for the LinSearch project.

## Project Structure

```
linsearch/
├── .cargo/
│   └── config.toml          # Cargo build configuration
├── .github/
│   └── workflows/
│       ├── ci.yml           # Continuous Integration
│       └── release.yml      # Release automation
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # CLI argument parsing (clap)
│   ├── search.rs            # Search logic and algorithms
│   ├── ui.rs                # User interaction & display
│   └── api/
│       ├── mod.rs           # Linear API client
│       ├── types.rs         # Data structures & types
│       └── queries.rs       # GraphQL query strings
├── Cargo.toml               # Project manifest & dependencies
├── Makefile                 # Make automation
├── justfile                 # Just automation (modern alternative)
├── README.md                # User documentation
└── DEVELOPMENT.md           # This file
```

## Module Overview

### `main.rs`

- Application entry point
- Orchestrates the flow: CLI parsing → API calls → Search → Display
- Minimal logic, delegates to other modules

### `lib.rs`

- Public library interface
- Exports modules for potential reuse as a library

### `cli.rs`

- Command-line argument parsing using `clap`
- Defines the CLI structure and help text

### `api/mod.rs`

- `LinearClient` struct that wraps the HTTP client
- Handles authentication and rate limiting
- Methods: `fetch_teams()`, `fetch_issues()`, `fetch_comments()`

### `api/types.rs`

- All data structures for API responses
- GraphQL response types
- Search result types (`Match`, `CommentHit`)

### `api/queries.rs`

- GraphQL query string constants
- Keeps queries separate from logic for easy modification

### `search.rs`

- Search logic and filtering
- `SearchOptions` configuration
- `search_issues()` function that processes issues

### `ui.rs`

- User interaction functions (`prompt`)
- Display functions for teams, results, etc.
- Keeps presentation logic separate from business logic

## Development Workflow

### Quick Start

```bash
# Clone and setup
git clone https://github.com/Suthyy/linsearch.git
cd linsearch

# Run in dev mode
cargo run -- "search term" -d -c

# Or with make
make dev ARGS='"search term" -d -c'

# Or with just
just dev "search term" -d -c
```

### Development Loop

1. **Make changes** to source files
2. **Check compilation**: `cargo check`
3. **Format code**: `cargo fmt`
4. **Run linter**: `cargo clippy`
5. **Test changes**: `cargo run -- <args>`
6. **Commit changes**

### Quick Commands

```bash
# Fast compilation check
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Run with arguments
cargo run -- "test" -d -c

# Build release binary
cargo build --release
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Adding Tests

Add tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Building for Release

### Local Release Build

```bash
# Build optimized binary
cargo build --release

# Binary location
ls -lh target/release/linear-finder

# Test the release binary
./target/release/linear-finder "test" -d -c
```

### Cross-Platform Builds

Install `cross` for easy cross-compilation:

```bash
cargo install cross

# Build for macOS Intel
cross build --release --target x86_64-apple-darwin

# Build for macOS Apple Silicon
cross build --release --target aarch64-apple-darwin

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for Windows
cross build --release --target x86_64-pc-windows-msvc
```

## Release Process

### Manual Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if you create one)
3. Commit changes
4. Create and push tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
5. GitHub Actions will automatically build and create a release

### Automated Release (GitHub Actions)

The project includes two workflows:

1. **CI Workflow** (`.github/workflows/ci.yml`)

   - Runs on every push/PR
   - Checks formatting, linting, tests
   - Builds on Linux, macOS, Windows

2. **Release Workflow** (`.github/workflows/release.yml`)
   - Triggered by version tags (`v*`)
   - Builds binaries for all platforms
   - Creates GitHub release with binaries
   - Generates checksums

## Code Style

### Formatting

Use `rustfmt` for consistent formatting:

```bash
cargo fmt
```

Configuration is in `rustfmt.toml` (if you add one).

### Linting

Use `clippy` for catching common mistakes:

```bash
cargo clippy -- -D warnings
```

### Naming Conventions

- **Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Best Practices

1. **Early returns**: Use early returns to reduce nesting
2. **Error handling**: Use `Result` and `?` operator
3. **Documentation**: Add doc comments for public APIs
4. **Modularity**: Keep functions small and focused
5. **Type safety**: Leverage Rust's type system

## Debugging

### Print Debugging

```rust
println!("Debug: {:?}", variable);
eprintln!("Error: {:?}", error);
dbg!(variable);
```

### Environment Variables

```bash
# Enable debug logging for reqwest
RUST_LOG=reqwest=debug cargo run -- "test"

# Enable all debug logging
RUST_LOG=debug cargo run -- "test"
```

### Using a Debugger

With VS Code and rust-analyzer:

1. Install "CodeLLDB" extension
2. Set breakpoints
3. Press F5 to debug

## Performance Optimization

### Profiling

```bash
# Build with debug info
cargo build --release --profile release-with-debug

# Profile with instruments (macOS)
instruments -t "Time Profiler" target/release/linsearch

# Profile with perf (Linux)
perf record target/release/linsearch "test"
perf report
```

### Binary Size

Check binary size:

```bash
cargo build --release
ls -lh target/release/linsearch
```

Reduce binary size (already configured in `Cargo.toml`):

```toml
[profile.release]
opt-level = 3      # Maximum optimization
lto = true         # Link-time optimization
codegen-units = 1  # Better optimization
strip = true       # Strip symbols
```

## Dependencies

### Current Dependencies

- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` - Serialization
- `serde_json` - JSON handling
- `anyhow` - Error handling

### Adding Dependencies

```bash
# Add a dependency
cargo add dependency-name

# Add with features
cargo add tokio --features full

# Add dev dependency
cargo add --dev criterion
```

## Troubleshooting

### Common Issues

**Issue**: Compilation errors after pulling changes

```bash
# Solution: Clean and rebuild
cargo clean
cargo build
```

**Issue**: Outdated dependencies

```bash
# Solution: Update dependencies
cargo update
```

**Issue**: Binary not found after install

```bash
# Solution: Ensure ~/.cargo/bin is in PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

### Pre-commit Checklist

- [ ] Code compiles: `cargo check`
- [ ] Tests pass: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation updated if needed

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Linear API Documentation](https://developers.linear.app/docs/graphql/working-with-the-graphql-api)
