# LinSearch - Just Commands
# Install just: cargo install just
# Usage: just <recipe>

# List all available recipes
default:
    @just --list

# Build debug binary
build:
    cargo build

# Build optimized release binary
build-release:
    cargo build --release
    @echo ""
    @echo "✅ Release binary created at: target/release/linsearch"
    @du -h target/release/linsearch

# Run in development mode with arguments
dev *ARGS:
    cargo run -- {{ARGS}}

# Clean build artifacts
clean:
    cargo clean

# Install to local cargo bin
install:
    cargo install --path .

# Run tests
test:
    cargo test

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Format and lint
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

# Create distribution package for current platform
dist: build-release
    #!/usr/bin/env bash
    mkdir -p dist
    cp target/release/linsearch dist/
    echo "✅ Distribution package created in dist/"
    ls -lh dist/

# Build for multiple platforms (requires cross)
build-all:
    #!/usr/bin/env bash
    echo "Building for multiple platforms..."
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    echo "✅ Builds complete"

# Watch and rebuild on changes
watch:
    cargo watch -x check -x test -x build

# Show binary size
size:
    @du -h target/release/linsearch 2>/dev/null || echo "Release binary not found. Run 'just build-release' first."

# Run with example search
example:
    cargo run -- "example" -d -c
