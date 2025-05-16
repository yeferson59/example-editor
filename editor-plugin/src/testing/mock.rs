//! Mock implementations for testing

use std::sync::Arc;
use async_trait::async_trait;
use parking_lot::RwLock;
use crate::{Plugin, PluginMetadata, Result, PluginEvent};

/// Mock plugin for testing
#[derive(Default)]
pub struct MockPlugin {
    /// Plugin metadata
    metadata: PluginMetadata,
    /// Initialize call count
    initialize_count: Arc<RwLock<usize>>,
    /// Shutdown call count
    shutdown_count: Arc<RwLock<usize>>,
    /// Execute call count and history
    execute_history: Arc<RwLock<Vec<(String, serde_json::Value)>>>,
}

impl MockPlugin {
    /// Creates a new mock plugin
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: PluginMetadata {
                name: name.into(),
                version: "0.1.0".to_string(),
                description: "Mock plugin for testing".to_string(),
            },
            initialize_count: Arc::new(RwLock::new(0)),
            shutdown_count: Arc::new(RwLock::new(0)),
            execute_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Returns the number of times initialize was called
    pub fn initialize_count(&self) -> usize {
        *self.initialize_count.read()
    }

    /// Returns the number of times shutdown was called
    pub fn shutdown_count(&self) -> usize {
        *self.shutdown_count.read()
    }

    /// Returns the execution history
    pub fn execute_history(&self) -> Vec<(String, serde_json::Value)> {
        self.execute_history.read().clone()
    }
}

#[async_trait]
impl Plugin for MockPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        *self.initialize_count.write() += 1;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        *self.shutdown_count.write() += 1;
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        self.execute_history.write().push((command.to_string(), args.clone()));
        Ok(serde_json::json!({
            "status": "ok",
            "command": command,
            "args": args
        }))
    }
}

/// Mock event handler for testing
#[derive(Default)]
pub struct MockEventHandler {
    /// Received events
    events: Arc<RwLock<Vec<PluginEvent>>>,
}

impl MockEventHandler {
    /// Creates a new mock event handler
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Returns the received events
    pub fn events(&self) -> Vec<PluginEvent> {
        self.events.read().clone()
    }
}

#[async_trait]
impl crate::event::EventHandler for MockEventHandler {
    async fn handle_event(&self, event: PluginEvent) {
        self.events.write().push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_plugin() {
        let mut plugin = MockPlugin::new("test");
        
        // Test initialization
        assert_eq!(plugin.initialize_count(), 0);
        plugin.initialize().await.unwrap();
        assert_eq!(plugin.initialize_count(), 1);

        // Test command execution
        let result = plugin.execute(
            "test_command",
            serde_json::json!({"arg": "value"}),
        ).await.unwrap();

        assert_eq!(result["status"], "ok");
        assert_eq!(result["command"], "test_command");
        
        let history = plugin.execute_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].0, "test_command");

        // Test shutdown
        assert_eq!(plugin.shutdown_count(), 0);
        plugin.shutdown().await.unwrap();
        assert_eq!(plugin.shutdown_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_event_handler() {
        let handler = MockEventHandler::new();
        
        let event = PluginEvent::Loaded(PluginMetadata {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
            description: "Test plugin".to_string(),
        });

        handler.handle_event(event.clone()).await;
        
        let events = handler.events();
        assert_eq!(events.len(), 1);
        // Compare the events based on their debug representation
        assert_eq!(format!("{:?}", events[0]), format!("{:?}", event));
    }
}
