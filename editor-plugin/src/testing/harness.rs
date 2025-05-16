//! Test harness for plugin testing

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use async_trait::async_trait;

use crate::{
    Plugin, PluginConfig, PluginEvent, PluginMetadata,
    loader::PluginLoader, Result,
};

/// Test harness for plugin testing
pub struct TestHarness {
    /// Active plugins
    plugins: RwLock<HashMap<String, Box<dyn Plugin>>>,
    /// Plugin loader
    loader: PluginLoader,
    /// Event sender
    event_tx: mpsc::Sender<PluginEvent>,
    /// Event receiver
    event_rx: mpsc::Receiver<PluginEvent>,
    /// Temporary test directory
    temp_dir: tempfile::TempDir,
}

impl TestHarness {
    /// Creates a new test harness
    pub fn new() -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(100);
        let temp_dir = tempfile::TempDir::new()?;
        
        Ok(Self {
            plugins: RwLock::new(HashMap::new()),
            loader: PluginLoader::new(),
            event_tx,
            event_rx,
            temp_dir,
        })
    }

    /// Loads a plugin for testing
    pub async fn load_plugin(&self, path: impl AsRef<Path>) -> Result<()> {
        let plugin = self.loader.load(path).await?;
        let name = plugin.metadata().name.clone();
        self.plugins.write().await.insert(name, plugin);
        Ok(())
    }

    /// Returns the path to the temporary directory
    pub fn temp_path(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }

    /// Creates a test file
    pub async fn create_test_file(&self, name: &str, content: &str) -> Result<PathBuf> {
        let path = self.temp_dir.path().join(name);
        tokio::fs::write(&path, content).await?;
        Ok(path)
    }

    /// Executes a plugin command
    pub async fn execute_command(
        &self,
        plugin_name: &str,
        command: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(plugin_name) {
            plugin.execute(command, args).await
        } else {
            Err(crate::PluginError::ExecutionError(
                format!("Plugin {} not found", plugin_name)
            ))
        }
    }

    /// Waits for a specific event
    pub async fn wait_for_event(&mut self) -> Option<PluginEvent> {
        self.event_rx.recv().await
    }

    /// Sends a test event
    pub async fn send_event(&self, event: PluginEvent) -> Result<()> {
        self.event_tx.send(event).await.map_err(|e| {
            crate::PluginError::ExecutionError(format!("Failed to send event: {}", e))
        })
    }
}

/// Test plugin for the harness
#[derive(Default)]
pub struct TestPlugin {
    metadata: PluginMetadata,
    initialized: bool,
}

#[async_trait]
impl Plugin for TestPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        self.initialized = true;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.initialized = false;
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "command": command,
            "args": args,
            "initialized": self.initialized,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_harness_basics() {
        let harness = TestHarness::new().unwrap();
        
        // Create test file
        let file_path = harness.create_test_file("test.txt", "Hello, World!")
            .await
            .unwrap();
        
        assert!(file_path.exists());
        let content = tokio::fs::read_to_string(file_path).await.unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[tokio::test]
    async fn test_plugin_execution() {
        let harness = TestHarness::new().unwrap();
        let mut plugins = harness.plugins.write().await;
        
        let plugin = TestPlugin {
            metadata: PluginMetadata {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
                description: "Test plugin".to_string(),
            },
            initialized: false,
        };

        plugins.insert("test".to_string(), Box::new(plugin));
        drop(plugins);

        let result = harness.execute_command(
            "test",
            "test_command",
            serde_json::json!({"arg": "value"}),
        ).await.unwrap();

        assert_eq!(result["command"], "test_command");
        assert_eq!(result["args"]["arg"], "value");
    }
}
