# Rust Editor Configuration Guide

This directory contains the configuration files for Rust Editor. The main configuration file is `editor.toml`, which controls various aspects of the editor's behavior.

## Configuration Files

- `editor.toml`: Main configuration file
- `keybindings.toml`: Custom key bindings (optional)
- `snippets.toml`: Code snippets (optional)

## Configuration Sections

### Editor Settings

```toml
[editor]
theme = "dark"          # Editor theme (dark/light)
font = "JetBrains Mono" # Editor font
font_size = 14         # Font size in points
line_numbers = true    # Show line numbers
tab_size = 4          # Tab width in spaces
insert_spaces = true  # Convert tabs to spaces
```

### UI Settings

```toml
[ui]
command_palette = true # Show command palette
minimap = true        # Show minimap
status_line = true    # Show status line
file_tree = true     # Show file tree
```

### Keybindings

```toml
[keybindings]
"ctrl+s" = "save"
"ctrl+p" = "command_palette"
```

### Language Settings

```toml
[language.rust]
formatter = "rustfmt"
auto_format = true
lsp = "rust-analyzer"
```

## Theme Customization

```toml
[theme]
background = "#282c34"
foreground = "#abb2bf"
selection = "#3e4451"
```

## Plugin Configuration

```toml
[plugin]
enabled = true
auto_update = true
safe_mode = true
```

## Environment Variables

The editor respects the following environment variables:

- `RUST_EDITOR_CONFIG`: Path to config directory
- `RUST_EDITOR_PLUGINS`: Path to plugins directory
- `RUST_EDITOR_THEME`: Override theme setting
- `RUST_EDITOR_LOG`: Set log level (error, warn, info, debug, trace)

## Additional Configuration

### LSP Servers

The editor automatically manages LSP servers for supported languages. You can configure additional LSP servers in the language section:

```toml
[language.custom]
lsp = "custom-language-server"
lsp_args = ["--stdio"]
```

### Formatters

Configure code formatters for different languages:

```toml
[language.python]
formatter = "black"
formatter_args = ["--line-length", "88"]
```

### File Associations

Associate file extensions with languages:

```toml
[file_types]
".rs" = "rust"
".py" = "python"
".jsx" = "javascript"
```

## Debugging

To troubleshoot configuration issues:

1. Run with debug logging:
   ```bash
   RUST_EDITOR_LOG=debug rust-editor
   ```

2. Check the configuration location:
   ```bash
   rust-editor --config-path
   ```

3. Validate configuration:
   ```bash
   rust-editor --validate-config
   ```

## Examples

### Custom Theme

```toml
[theme]
name = "My Theme"
background = "#1e1e1e"
foreground = "#d4d4d4"
selection = "#264f78"
cursor = "#528bff"
```

### Custom Keybindings

```toml
[keybindings]
"ctrl+shift+b" = "build"
"f5" = "debug"
"ctrl+r" = "run"
```

### Language-specific Settings

```toml
[language.rust]
tab_size = 4
auto_format = true
format_on_save = true

[language.python]
tab_size = 4
auto_format = true
format_on_save = true
```

## Configuration Precedence

Configuration settings are applied in the following order (highest to lowest priority):

1. Command-line arguments
2. Environment variables
3. User configuration file
4. Workspace configuration
5. Default settings

## Updating Configuration

Configuration changes are applied:

- Immediately for UI settings
- On next file open for language settings
- On editor restart for some core settings

## Further Resources

- [Configuration Reference](docs/configuration.md)
- [Theme Guide](docs/themes.md)
- [Keybindings Reference](docs/keybindings.md)
- [Plugin Development](docs/plugins.md)
