//! Plugin manager implementation

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{Plugin, PluginMetadata, Result};

/// Plugin event types
#[derive(Debug, Clone)]
pub enum PluginEvent {
    /// Plugin was loaded
    Loaded(PluginMetadata),
    /// Plugin was unloaded
    Unloaded(PluginMetadata),
    /// Plugin state changed
    StateChanged {
        /// Plugin metadata
        metadata: PluginMetadata,
        /// New state
        state: PluginState,
    },
    /// Plugin error occurred
    Error {
        /// Plugin metadata
        metadata: PluginMetadata,
        /// Error message
        error: String,
    },
}

/// Plugin state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginState {
    /// Plugin is loaded but not initialized
    Loaded,
    /// Plugin is initialized and running
    Running,
    /// Plugin is disabled
    Disabled,
    /// Plugin encountered an error
    Error,
}

/// Plugin manager
pub struct PluginManager {
    /// Active plugins
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    /// Plugin states
    states: Arc<RwLock<HashMap<String, PluginState>>>,
    /// Event subscribers
    subscribers: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<PluginEvent>>>>,
}

impl PluginManager {
    /// Creates a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            states: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Registers a plugin
    pub async fn register_plugin(&self, plugin: Box<dyn Plugin>) -> Result<()> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        self.plugins.write().await.insert(name.clone(), plugin);
        self.states.write().await.insert(name.clone(), PluginState::Loaded);

        self.emit_event(PluginEvent::Loaded(metadata)).await;
        Ok(())
    }

    /// Unregisters a plugin
    pub async fn unregister_plugin(&self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.write().await.remove(name) {
            let metadata = plugin.metadata().clone();
            self.states.write().await.remove(name);
            self.emit_event(PluginEvent::Unloaded(metadata)).await;
        }
        Ok(())
    }

    /// Initializes a plugin
    pub async fn initialize_plugin(&self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.write().await.get_mut(name) {
            plugin.initialize().await?;
            self.states.write().await.insert(name.to_string(), PluginState::Running);
            
            self.emit_event(PluginEvent::StateChanged {
                metadata: plugin.metadata().clone(),
                state: PluginState::Running,
            }).await;
        }
        Ok(())
    }

    /// Shuts down a plugin
    pub async fn shutdown_plugin(&self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.write().await.get_mut(name) {
            plugin.shutdown().await?;
            self.states.write().await.insert(name.to_string(), PluginState::Disabled);
            
            self.emit_event(PluginEvent::StateChanged {
                metadata: plugin.metadata().clone(),
                state: PluginState::Disabled,
            }).await;
        }
        Ok(())
    }

    /// Executes a plugin command
    pub async fn execute_command(&self, name: &str, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        if let Some(plugin) = self.plugins.read().await.get(name) {
            plugin.execute(command, args).await
        } else {
            Err(crate::PluginError::ExecutionError(format!("Plugin {} not found", name)))
        }
    }

    /// Subscribes to plugin events
    pub async fn subscribe(&self) -> tokio::sync::mpsc::Receiver<PluginEvent> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        self.subscribers.write().await.push(tx);
        rx
    }

    /// Emits a plugin event
    async fn emit_event(&self, event: PluginEvent) {
        let subscribers = self.subscribers.read().await;
        for subscriber in subscribers.iter() {
            let _ = subscriber.send(event.clone()).await;
        }
    }

    /// Returns the state of a plugin
    pub async fn get_plugin_state(&self, name: &str) -> Option<PluginState> {
        self.states.read().await.get(name).cloned()
    }

    /// Returns all registered plugins
    pub async fn get_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.read().await
            .values()
            .map(|p| p.metadata().clone())
            .collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginMetadata;

    struct TestPlugin {
        metadata: PluginMetadata,
    }

    #[async_trait::async_trait]
    impl Plugin for TestPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        async fn initialize(&mut self) -> Result<()> {
            Ok(())
        }

        async fn shutdown(&mut self) -> Result<()> {
            Ok(())
        }

        async fn execute(&self, _command: &str, _args: serde_json::Value) -> Result<serde_json::Value> {
            Ok(serde_json::json!({"status": "ok"}))
        }
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        let manager = PluginManager::new();

        let plugin = TestPlugin {
            metadata: PluginMetadata {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
                description: "Test plugin".to_string(),
            },
        };

        manager.register_plugin(Box::new(plugin)).await.unwrap();
        assert_eq!(manager.get_plugin_state("test").await, Some(PluginState::Loaded));

        manager.initialize_plugin("test").await.unwrap();
        assert_eq!(manager.get_plugin_state("test").await, Some(PluginState::Running));

        manager.shutdown_plugin("test").await.unwrap();
        assert_eq!(manager.get_plugin_state("test").await, Some(PluginState::Disabled));

        manager.unregister_plugin("test").await.unwrap();
        assert_eq!(manager.get_plugin_state("test").await, None);
    }
}
