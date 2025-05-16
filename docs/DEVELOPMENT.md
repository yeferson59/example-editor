# Development Guide

This guide explains how to set up your development environment and contribute to Rust Editor.

## Setting Up Development Environment

1. Install Rust and Cargo
2. Install required dependencies:
   ```bash
   # macOS
   brew install tree-sitter

   # Ubuntu/Debian
   sudo apt-get install tree-sitter-cli

   # Windows
   cargo install tree-sitter-cli
   ```

3. Clone and build the project:
   ```bash
   git clone https://github.com/yourusername/rust-editor.git
   cd rust-editor
   cargo build --workspace
   ```

## Architecture Overview

### Editor Core

The core editing engine handles text manipulation, document management, and event dispatching:

```rust
editor-core/
├── src/
│   ├── buffer/       # Text buffer implementation
│   ├── document/     # Document management
│   ├── event/        # Event system
│   └── text/         # Text operations
```

### UI Layer

The UI layer is built with egui and handles all user interactions:

```rust
editor-ui/
├── src/
│   ├── app.rs        # Main application
│   ├── editor.rs     # Editor view
│   └── widgets/      # UI components
```

### Syntax Highlighting

Syntax highlighting is implemented using tree-sitter:

```rust
editor-syntax/
├── src/
│   ├── highlighter/  # Syntax highlighter
│   ├── language/     # Language definitions
│   └── parser/       # Code parser
```

### LSP Support

Language Server Protocol support for code intelligence:

```rust
editor-lsp/
├── src/
│   ├── client/      # LSP client
│   ├── server/      # Language server
│   └── config/      # LSP configuration
```

### Plugin System

The plugin system supports both native and WebAssembly plugins:

```rust
editor-plugin/
├── src/
│   ├── loader/      # Plugin loading
│   ├── manager/     # Plugin management
│   ├── native/      # Native plugin support
│   └── wasm/        # WebAssembly support
```

## Building and Testing

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific component
cargo build -p editor-core
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cargo test -p editor-core

# Run with logging
RUST_LOG=debug cargo test
```

### Documentation

```bash
# Generate documentation
cargo doc --workspace --no-deps

# Open documentation
cargo doc --workspace --no-deps --open
```

## Code Style

We follow the standard Rust style guide with some additional rules:

- Use `rustfmt` for code formatting
- Follow the Rust API guidelines
- Write documentation for public APIs
- Include tests for new functionality

## Debugging

1. Use logging:
   ```rust
   log::debug!("Debug message");
   log::info!("Info message");
   log::error!("Error message");
   ```

2. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run
   ```

3. Use VS Code with rust-analyzer:
   - Install rust-analyzer extension
   - Configure launch.json for debugging

## Performance Profiling

1. Install perf tools:
   ```bash
   cargo install cargo-flamegraph
   ```

2. Generate flamegraph:
   ```bash
   cargo flamegraph
   ```

## Creating Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and lints
5. Submit a pull request

Please ensure your PR:
- Includes tests
- Updates documentation
- Follows code style
- Has a clear description
- References any related issues

## Release Process

1. Update version in Cargo.toml files
2. Update CHANGELOG.md
3. Create git tag
4. Build release artifacts
5. Publish to crates.io

## Getting Help

- Join our Discord server
- Check the issue tracker
- Read the documentation
- Contact the maintainers
