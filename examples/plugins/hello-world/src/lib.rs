//! Example hello world plugin for Rust Editor

use editor_plugin::{Plugin, PluginMetadata, Result};
use async_trait::async_trait;
use serde_json::json;

/// Hello world plugin
#[derive(Default)]
pub struct HelloWorldPlugin {
    /// Plugin metadata
    metadata: PluginMetadata,
}

impl HelloWorldPlugin {
    /// Creates a new hello world plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "hello-world".to_string(),
                version: "0.1.0".to_string(),
                description: "A simple hello world plugin".to_string(),
            },
        }
    }
}

#[async_trait]
impl Plugin for HelloWorldPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        log::info!("Hello World plugin initialized!");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        log::info!("Hello World plugin shutting down!");
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        match command {
            "greet" => {
                let name = args.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("World");
                
                Ok(json!({
                    "message": format!("Hello, {}!", name)
                }))
            }
            _ => Ok(json!({
                "error": format!("Unknown command: {}", command)
            }))
        }
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(HelloWorldPlugin::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin() {
        let mut plugin = HelloWorldPlugin::new();
        
        // Test initialization
        assert!(plugin.initialize().await.is_ok());
        
        // Test command execution
        let result = plugin.execute(
            "greet",
            json!({"name": "Rust"}),
        ).await.unwrap();
        
        assert_eq!(
            result.get("message").unwrap().as_str().unwrap(),
            "Hello, Rust!"
        );
        
        // Test shutdown
        assert!(plugin.shutdown().await.is_ok());
    }
}
