# Getting Started with Rust Editor

This guide will help you get started with using and developing Rust Editor.

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-editor.git
   cd rust-editor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the editor:
   ```bash
   cargo install --path .
   ```

### Using Pre-built Binaries

Download the appropriate binary for your platform from the [releases page](https://github.com/yourusername/rust-editor/releases).

## Basic Usage

### Opening Files

```bash
# Open a single file
rust-editor file.txt

# Open multiple files
rust-editor file1.txt file2.txt

# Open a directory
rust-editor /path/to/project
```

### Key Bindings

Default key bindings:

- `Ctrl+S`: Save file
- `Ctrl+O`: Open file
- `Ctrl+P`: Command palette
- `Ctrl+F`: Find
- `Ctrl+H`: Replace
- `Ctrl+B`: Toggle sidebar
- `Ctrl+\`: Split editor
- `Ctrl+Tab`: Switch tab
- `Ctrl+W`: Close tab

### Commands

Access the command palette with `Ctrl+P` to see all available commands.

Common commands:
- `File: New File`
- `File: Save`
- `View: Toggle File Explorer`
- `Edit: Format Document`
- `Go to Definition`
- `Find References`

## Configuration

The editor can be configured through `~/.config/rust-editor/config.toml`:

```toml
[editor]
theme = "dark"
font = "JetBrains Mono"
font_size = 14
tab_size = 4
insert_spaces = true
line_numbers = true
word_wrap = true

[ui]
minimap = true
status_bar = true
file_tree = true

[keybindings]
"ctrl+s" = "save"
"ctrl+p" = "command_palette"
"ctrl+/" = "toggle_comment"
```

## Language Support

The editor supports various programming languages through LSP:

1. Rust:
   ```toml
   [language.rust]
   lsp = "rust-analyzer"
   formatter = "rustfmt"
   ```

2. Python:
   ```toml
   [language.python]
   lsp = "pyright"
   formatter = "black"
   ```

3. JavaScript/TypeScript:
   ```toml
   [language.javascript]
   lsp = "typescript-language-server"
   formatter = "prettier"
   ```

## Plugins

### Installing Plugins

1. From the command palette:
   - Press `Ctrl+P`
   - Type `Plugins: Install Plugin`
   - Select the plugin to install

2. Manual installation:
   ```bash
   mkdir -p ~/.config/rust-editor/plugins/plugin-name
   cp plugin-files ~/.config/rust-editor/plugins/plugin-name/
   ```

### Writing Plugins

See the [Plugin Development Guide](PLUGINS.md) for details on creating plugins.

## Customization

### Themes

Create custom themes in `~/.config/rust-editor/themes/`:

```toml
# mytheme.toml
[theme]
name = "My Theme"
dark = true

[colors]
background = "#1e1e1e"
foreground = "#d4d4d4"
selection = "#264f78"
cursor = "#ffffff"

[syntax]
keyword = "#569cd6"
string = "#ce9178"
comment = "#6a9955"
function = "#dcdcaa"
```

### Snippets

Add code snippets in `~/.config/rust-editor/snippets/`:

```json
{
  "println": {
    "prefix": "println",
    "body": "println!(\"$1\");",
    "description": "Print line macro"
  }
}
```

## Development

### Setting Up Development Environment

1. Install dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

   # macOS
   brew install tree-sitter

   # Windows
   # Install Visual Studio Build Tools
   ```

2. Install development tools:
   ```bash
   cargo install cargo-watch cargo-edit cargo-audit
   ```

### Development Commands

Use the development script:

```bash
# Build the project
./scripts/dev.sh build

# Run tests
./scripts/dev.sh test

# Format code
./scripts/dev.sh fmt

# Check code
./scripts/dev.sh check

# Run linter
./scripts/dev.sh lint

# Generate docs
./scripts/dev.sh doc
```

### Using Docker

```bash
# Start development environment
docker-compose up dev

# Run tests
docker-compose run test
```

## Troubleshooting

### Common Issues

1. LSP not working:
   - Check LSP server installation
   - Verify language configuration
   - Check LSP logs in Output panel

2. Performance issues:
   - Disable minimap for large files
   - Check system resources
   - Update editor and plugins

3. Plugin problems:
   - Verify plugin compatibility
   - Check plugin logs
   - Try reinstalling the plugin

### Getting Help

- Check the [FAQ](FAQ.md)
- Search [issues](https://github.com/yourusername/rust-editor/issues)
- Join our [Discord server](https://discord.gg/your-server)
- Read the [documentation](https://docs.rust-editor.dev)

## Next Steps

- Read the [User Guide](USER_GUIDE.md) for detailed information
- Learn about [extending the editor](EXTENDING.md)
- Contribute to the project
