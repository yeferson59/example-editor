# File Stats Plugin

A plugin for Rust Editor that provides detailed file and directory statistics with UI integration.

## Features

- Counts files and directories
- Calculates total size
- Shows file type distribution
- Counts lines of code
- Provides formatted output
- UI integration for statistics display

## Usage

1. Build the plugin:
   ```bash
   cargo build --release
   ```

2. Install the plugin:
   ```bash
   cp target/release/libfile_stats_plugin.* ~/.config/rust-editor/plugins/
   cp plugin.json ~/.config/rust-editor/plugins/file-stats/
   ```

3. Use the plugin in Rust Editor:
   ```rust
   // Analyze a directory
   editor.execute_plugin_command(
       "file-stats",
       "analyze",
       json!({"path": "/path/to/directory"})
   );

   // Get current statistics
   editor.execute_plugin_command("file-stats", "get_stats", json!({}));
   ```

## Commands

- `analyze`: Analyzes a directory
  ```json
  {
      "path": "/path/to/analyze"
  }
  ```

- `get_stats`: Returns current statistics
  ```json
  {}
  ```

## Permissions

This plugin requires read-only file system access to analyze directories.

## Development

To modify the plugin:

1. Update the plugin code in `src/lib.rs`
2. Add new statistics or analysis features
3. Run tests: `cargo test`
4. Build: `cargo build --release`

## Testing

Run the tests:
```bash
cargo test
```

## License

MIT License
