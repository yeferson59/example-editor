//! Native plugin implementation

use std::path::Path;
use libloading::{Library, Symbol};
use crate::{Plugin, PluginConfig, PluginMetadata, Result, PluginError};

/// Native plugin
#[allow(dead_code)]
pub struct NativePlugin {
    /// Plugin library
    library: Library,
    /// Plugin metadata
    metadata: PluginMetadata,
    /// Plugin interface
    interface: Box<dyn PluginInterface>,
}

/// Plugin interface for native plugins
pub trait PluginInterface: Send + Sync {
    /// Initializes the plugin
    fn initialize(&mut self) -> Result<()>;
    /// Shuts down the plugin
    fn shutdown(&mut self) -> Result<()>;
    /// Executes a command
    fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value>;
}

impl NativePlugin {
    /// Loads a native plugin from a path
    pub async fn load(path: impl AsRef<Path>, config: PluginConfig) -> Result<Self> {
        let path = path.as_ref();
        let library_path = if cfg!(target_os = "windows") {
            path.join(&config.manifest.entry_point).with_extension("dll")
        } else if cfg!(target_os = "macos") {
            path.join(&config.manifest.entry_point).with_extension("dylib")
        } else {
            path.join(&config.manifest.entry_point).with_extension("so")
        };

        unsafe {
            let library = Library::new(library_path)
                .map_err(|e| PluginError::LoadError(e.to_string()))?;

            // Load plugin factory function
            let factory: Symbol<unsafe extern "C" fn() -> *mut dyn PluginInterface> = 
                library.get(b"create_plugin")
                .map_err(|e| PluginError::LoadError(e.to_string()))?;

            // Create plugin instance
            let interface = Box::from_raw(factory());

            Ok(Self {
                library,
                metadata: PluginMetadata {
                    name: config.manifest.name,
                    version: config.manifest.version,
                    description: config.manifest.description,
                },
                interface,
            })
        }
    }
}

#[async_trait::async_trait]
impl Plugin for NativePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        self.interface.initialize()
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.interface.shutdown()
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        self.interface.execute(command, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestInterface;

    impl PluginInterface for TestInterface {
        fn initialize(&mut self) -> Result<()> {
            Ok(())
        }

        fn shutdown(&mut self) -> Result<()> {
            Ok(())
        }

        fn execute(&self, _command: &str, _args: serde_json::Value) -> Result<serde_json::Value> {
            Ok(serde_json::json!({"status": "ok"}))
        }
    }

    // Note: This test requires a real plugin library to work
    // #[tokio::test]
    // async fn test_native_plugin_loading() {
    //     // Test implementation
    // }
}
