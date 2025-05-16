# Plugin Development Guide

This guide explains how to create plugins for Rust Editor.

## Plugin Types

Rust Editor supports two types of plugins:

1. Native Plugins (shared libraries)
2. WebAssembly Plugins

## Creating a Native Plugin

### 1. Create a new library project

```bash
cargo new --lib my-plugin
cd my-plugin
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
editor-plugin = { path = "../editor-plugin" }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. Implement the Plugin Trait

```rust
use editor_plugin::{Plugin, PluginMetadata, Result};
use async_trait::async_trait;

#[derive(Default)]
pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata {
        // Return plugin metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        // Initialize your plugin
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        // Cleanup
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        // Handle commands
        Ok(serde_json::json!({"status": "ok"}))
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin::default()))
}
```

## Creating a WebAssembly Plugin

### 1. Create a new library project

```bash
cargo new --lib my-wasm-plugin
cd my-wasm-plugin
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-wasm-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
editor-plugin = { path = "../editor-plugin" }
wasm-bindgen = "0.2"
```

### 3. Implement the Plugin

```rust
use wasm_bindgen::prelude::*;
use editor_plugin::{Plugin, PluginMetadata, Result};

#[wasm_bindgen]
pub struct MyWasmPlugin;

#[wasm_bindgen]
impl MyWasmPlugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Plugin for MyWasmPlugin {
    // Implementation similar to native plugin
}
```

## Plugin Manifest

Every plugin needs a `plugin.json` manifest file:

```json
{
    "name": "my-plugin",
    "version": "0.1.0",
    "description": "My awesome plugin",
    "author": "Your Name",
    "license": "MIT",
    "entry_point": "lib",
    "plugin_type": "Native",
    "dependencies": [],
    "permissions": [
        {
            "FileSystem": {
                "paths": ["/tmp"],
                "read_only": true
            }
        }
    ]
}
```

## Plugin Permissions

Plugins run in a sandboxed environment and need explicit permissions:

- FileSystem: Access to specific paths
- Network: Access to specific hosts/ports
- Process: Ability to execute specific commands

## Plugin API

### Events

Plugins can subscribe to editor events:

```rust
use editor_plugin::event::EventHandler;

impl EventHandler for MyPlugin {
    async fn handle_event(&self, event: EditorEvent) {
        match event {
            EditorEvent::DocumentOpened(doc) => {
                // Handle document open
            }
            EditorEvent::DocumentClosed(doc) => {
                // Handle document close
            }
            // ...
        }
    }
}
```

### Commands

Plugins can register commands:

```rust
impl MyPlugin {
    fn register_commands(&self, registry: &mut CommandRegistry) {
        registry.register("my-command", |args| {
            // Command implementation
        });
    }
}
```

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin() {
        let mut plugin = MyPlugin::default();
        assert!(plugin.initialize().await.is_ok());
        // More tests...
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use editor_plugin::testing::TestHarness;

    #[tokio::test]
    async fn test_plugin_integration() {
        let harness = TestHarness::new();
        harness.load_plugin("path/to/plugin").await;
        // Test plugin functionality
    }
}
```

## Best Practices

1. **Error Handling**
   - Use proper error types
   - Provide meaningful error messages
   - Handle all error cases

2. **Performance**
   - Avoid blocking operations
   - Use async/await for I/O
   - Minimize memory usage

3. **Security**
   - Request minimal permissions
   - Validate all inputs
   - Handle sensitive data carefully

4. **Documentation**
   - Document public API
   - Provide usage examples
   - Include setup instructions

## Distribution

1. Build your plugin:
   ```bash
   cargo build --release
   ```

2. Package the plugin:
   ```bash
   tar czf my-plugin.tar.gz \
       target/release/libmy_plugin.* \
       plugin.json \
       README.md \
       LICENSE
   ```

## Debugging

1. Enable debug logging:
   ```rust
   log::debug!("Debug message");
   ```

2. Run with debug logging:
   ```bash
   RUST_LOG=debug cargo test
   ```

## Common Issues

1. **Loading Failures**
   - Check plugin manifest
   - Verify permissions
   - Check dependencies

2. **Runtime Errors**
   - Check error logs
   - Verify API usage
   - Test in isolation

## Resources

- API Documentation
- Example Plugins
- Plugin Registry
- Community Plugins
