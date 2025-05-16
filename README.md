# Rust Editor

A modern, extensible text editor written in Rust, featuring syntax highlighting, LSP support, and a plugin system.

## Features

- 🚀 High-performance text editing engine
- 🎨 Syntax highlighting with tree-sitter
- 🧠 Language Server Protocol (LSP) support
- 🔌 Plugin system with native and WebAssembly support
- 🎯 Multi-cursor editing
- 📦 Built-in package manager
- 🔍 Fast search and replace
- 🌳 File explorer
- 🔄 Git integration
- 🎹 Customizable keybindings
- 🎨 Theme support
- 📝 Multiple document editing

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-editor.git
cd rust-editor

# Build the project
cargo build --release

# Install the editor
cargo install --path .
```

### Dependencies

- Rust 1.70.0 or later
- tree-sitter
- Node.js and npm (for some language servers)

## Usage

```bash
# Open a file
rust-editor path/to/file.rs

# Open multiple files
rust-editor file1.rs file2.rs

# Open with a specific theme
rust-editor --theme dark file.rs
```

## Development

### Project Structure

- `editor-core`: Core text editing engine
- `editor-ui`: GUI implementation using egui
- `editor-syntax`: Syntax highlighting and parsing
- `editor-lsp`: Language Server Protocol support
- `editor-plugin`: Plugin system
- `rust-editor`: Main application

### Building

```bash
# Build all components
cargo build --workspace

# Run tests
cargo test --workspace

# Run with debug logging
RUST_LOG=debug cargo run
```

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Configuration

Configuration is stored in `~/.config/rust-editor/config.toml`:

```toml
[editor]
theme = "dark"
tab_size = 4
insert_spaces = true

[keybindings]
"ctrl+p" = "command_palette"
"ctrl+b" = "toggle_sidebar"
"ctrl+s" = "save"

[language.rust]
formatter = "rustfmt"
lsp = "rust-analyzer"
```

## Plugins

Rust Editor supports both native and WebAssembly plugins. See the [Plugin Development Guide](docs/PLUGINS.md) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [egui](https://github.com/emilk/egui) - GUI framework
- [tree-sitter](https://tree-sitter.github.io/tree-sitter/) - Parsing system
- [tower-lsp](https://github.com/ebkalderon/tower-lsp) - LSP implementation
