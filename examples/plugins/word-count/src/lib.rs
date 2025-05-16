//! Word count plugin example

use editor_plugin::{Plugin, PluginMetadata, Result};
use async_trait::async_trait;
use serde_json::json;

/// Word count plugin
pub struct WordCountPlugin {
    /// Plugin metadata
    metadata: PluginMetadata,
    /// Word count
    count: usize,
}

impl WordCountPlugin {
    /// Creates a new word count plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "word-count".to_string(),
                version: "0.1.0".to_string(),
                description: "Counts words in the current document".to_string(),
            },
            count: 0,
        }
    }

    /// Counts words in text
    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }
}

#[async_trait]
impl Plugin for WordCountPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        log::info!("Word count plugin initialized");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        log::info!("Word count plugin shutting down");
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        match command {
            "count" => {
                let text = args["text"].as_str().unwrap_or("");
                let count = self.count_words(text);
                Ok(json!({
                    "count": count,
                    "message": format!("Word count: {}", count)
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
    Box::into_raw(Box::new(WordCountPlugin::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_word_count() {
        let plugin = WordCountPlugin::new();
        
        let result = plugin.execute(
            "count",
            json!({
                "text": "Hello World! This is a test."
            }),
        ).await.unwrap();

        assert_eq!(result["count"], 6);
    }
}
