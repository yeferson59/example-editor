# Word Count Plugin

A simple plugin for Rust Editor that counts words in the current document.

## Features

- Word counting with whitespace-based tokenization
- Status bar integration
- Command palette integration

## Installation

1. Build the plugin:
   ```bash
   cd examples/plugins/word-count
   cargo build --release
   ```

2. Install the plugin:
   ```bash
   mkdir -p ~/.config/rust-editor/plugins/word-count
   cp target/release/libword_count_plugin.* ~/.config/rust-editor/plugins/word-count/
   cp plugin.json ~/.config/rust-editor/plugins/word-count/
   ```

## Usage

The plugin adds the following commands to the editor:

- `Word Count: Count Words` - Counts words in the current document
- `Word Count: Show Statistics` - Shows detailed statistics about the document

You can access these commands through:

1. Command Palette (Ctrl+P)
2. Status Bar (click on word count)
3. Context Menu (right-click in editor)

## Configuration

Add to your `~/.config/rust-editor/config.toml`:

```toml
[plugins.word-count]
enabled = true
show_in_status_bar = true
count_comments = false
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Creating a Release

```bash
cargo build --release
```

## License

MIT License - see the [LICENSE](LICENSE) file for details.
