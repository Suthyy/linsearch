# LinSearch

A fast command-line tool to search Linear issues for specific terms across titles, descriptions, and comments.

## Features

- 🔍 Search across issue titles, descriptions, and comments
- 🚀 Fast parallel API requests with rate limiting
- 📊 Interactive team selection
- 💾 Environment variable support for API keys
- 🎯 Flexible search options

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Suthyy/linsearch.git
cd linsearch

# Install using cargo
cargo install --path .

# Or build manually
cargo build --release
# Binary will be at: target/release/linsearch
```

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/Suthyy/linsearch/releases) page.

## Usage

### Basic Usage

```bash
# Search with interactive prompts
linsearch "search term"

# Search with all options specified
linsearch "search term" -a YOUR_API_KEY -t TEAM_ID -d -c
```

### Options

- `SEARCH_TERM` - The term to search for (required)
- `-a, --api-key <API_KEY>` - Linear API key (or set `LINEAR_API_KEY` env var)
- `-t, --team-id <TEAM_ID>` - Team ID to search within
- `-d, --descriptions` - Search in issue descriptions
- `-c, --comments` - Search in issue comments

### Environment Variables

Set your Linear API key as an environment variable to avoid entering it each time:

```bash
export LINEAR_API_KEY="your_api_key_here"
```

Add this to your `~/.zshrc` or `~/.bashrc` to make it permanent.

### Examples

```bash
# Search for "authentication" in descriptions and comments
linsearch "authentication" -d -c

# Search with API key from environment
export LINEAR_API_KEY="lin_api_..."
linsearch "bug" -d

# Search specific team
linsearch "feature" -t "team_id_here" -d -c
```

## Development

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Optional: [just](https://github.com/casey/just) command runner

### Quick Start

```bash
# Run in development mode
cargo run -- "search term" -d -c

# Or using make
make dev ARGS='"search term" -d -c'

# Or using just
just dev "search term" -d -c
```

### Development Commands

#### Using Make

```bash
make build          # Build debug binary
make build-release  # Build optimized release binary
make dev            # Run in development mode
make clean          # Clean build artifacts
make install        # Install to ~/.cargo/bin
make test           # Run tests
make check          # Check code without building
make fmt            # Format code
make lint           # Run clippy linter
make dist           # Create distribution package
```

#### Using Just

```bash
just build          # Build debug binary
just build-release  # Build optimized release binary
just dev            # Run in development mode
just clean          # Clean build artifacts
just install        # Install to ~/.cargo/bin
just test           # Run tests
just check          # Check code without building
just fmt            # Format code
just lint           # Run clippy linter
just fix            # Format and auto-fix lints
just dist           # Create distribution package
just watch          # Watch and rebuild on changes
just size           # Show binary size
```

### Project Structure

```
linsearch/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── cli.rs            # CLI argument parsing
│   ├── search.rs         # Search logic
│   ├── ui.rs             # User interaction & display
│   └── api/
│       ├── mod.rs        # API client
│       ├── types.rs      # Data structures
│       └── queries.rs    # GraphQL queries
├── Cargo.toml            # Dependencies
├── Makefile              # Make commands
├── justfile              # Just commands
└── README.md             # This file
```

## Building for Release

### Single Platform

```bash
# Build optimized binary
cargo build --release

# Binary will be at: target/release/linsearch
# Size: ~5-10MB (stripped)
```

### Multiple Platforms

For cross-compilation, install [cross](https://github.com/cross-rs/cross):

```bash
cargo install cross

# Build for different platforms
cross build --release --target x86_64-apple-darwin      # macOS Intel
cross build --release --target aarch64-apple-darwin     # macOS Apple Silicon
cross build --release --target x86_64-unknown-linux-gnu # Linux
cross build --release --target x86_64-pc-windows-msvc   # Windows
```

## Sharing with Colleagues

### Option 1: Share the Binary

1. Build the release binary:

   ```bash
   make build-release
   # or
   just build-release
   ```

2. Share the binary from `target/release/linear-finder`

3. Recipients can place it in their PATH:
   ```bash
   # macOS/Linux
   sudo cp linear-finder /usr/local/bin/
   # or
   cp linear-finder ~/.local/bin/
   ```

### Option 2: Install from Source

Share the repository and have colleagues run:

```bash
cargo install --path .
```

### Option 3: GitHub Releases

Use the included GitHub Actions workflow to automatically build and publish releases for multiple platforms.

## API Rate Limiting

The tool has a built-in rate limit of 1,500 API requests per run to prevent hitting Linear's API limits. If you need to search larger datasets, consider:

- Narrowing your search scope with team filters
- Running multiple searches with different terms
- Increasing the `MAX_REQUESTS` constant in `src/api/mod.rs`

## Acknowledgments

This project was developed with assistance from AI pair programming tools (Claude/Cursor). The implementation, architecture, and tooling were created through collaborative AI-assisted development.

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
