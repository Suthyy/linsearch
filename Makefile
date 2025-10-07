.PHONY: help build build-release dev clean install test check fmt lint

# Default target
help:
	@echo "LinSearch - Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build          - Build debug binary"
	@echo "  build-release  - Build optimized release binary"
	@echo "  dev            - Run in development mode with arguments (use ARGS='...')"
	@echo "  clean          - Clean build artifacts"
	@echo "  install        - Install binary to ~/.cargo/bin"
	@echo "  test           - Run tests"
	@echo "  check          - Check code without building"
	@echo "  fmt            - Format code"
	@echo "  lint           - Run clippy linter"
	@echo "  dist           - Create distribution package"
	@echo ""
	@echo "Examples:"
	@echo "  make dev ARGS='searchterm -d -c'"
	@echo "  make build-release"

# Build debug binary
build:
	cargo build

# Build optimized release binary
build-release:
	cargo build --release
	@echo ""
	@echo "✅ Release binary created at: target/release/linsearch"
	@echo "   Size: $$(du -h target/release/linsearch | cut -f1)"

# Run in development mode
dev:
	cargo run -- $(ARGS)

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

# Create distribution package
dist: build-release
	@mkdir -p dist
	@cp target/release/linsearch dist/
	@echo "✅ Distribution package created in dist/"
	@echo "   Binary: dist/linsearch"
