# Hello World Plugin

This is a simple example plugin for Rust Editor that demonstrates the basic plugin API.

## Features

- Implements a simple greeting command
- Shows basic plugin lifecycle (initialize/shutdown)
- Demonstrates command handling

## Usage

1. Build the plugin:
   ```bash
   cargo build --release
   ```

2. Install the plugin:
   ```bash
   cp target/release/libhello_world_plugin.* ~/.config/rust-editor/plugins/
   cp plugin.json ~/.config/rust-editor/plugins/hello-world/
   ```

3. Use the plugin in Rust Editor:
   ```rust
   // Execute the greeting command
   editor.execute_plugin_command("hello-world", "greet", json!({"name": "User"}));
   ```

## Development

To modify the plugin:

1. Update the plugin code in `src/lib.rs`
2. Add new commands in the `execute` function
3. Run tests: `cargo test`
4. Build: `cargo build --release`

## Testing

Run the tests:
```bash
cargo test
```

## License

MIT License
