//! Plugin registry implementation

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
}

/// Plugin registry
pub struct PluginRegistry {
    /// Registered plugins
    plugins: Arc<RwLock<HashMap<String, PluginMetadata>>>,
}

impl PluginRegistry {
    /// Creates a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a plugin
    pub async fn register(&self, metadata: PluginMetadata) {
        self.plugins.write().await.insert(metadata.name.clone(), metadata);
    }

    /// Unregisters a plugin
    pub async fn unregister(&self, name: &str) {
        self.plugins.write().await.remove(name);
    }

    /// Returns plugin metadata by name
    pub async fn get(&self, name: &str) -> Option<PluginMetadata> {
        self.plugins.read().await.get(name).cloned()
    }

    /// Returns all registered plugins
    pub async fn list(&self) -> Vec<PluginMetadata> {
        self.plugins.read().await.values().cloned().collect()
    }

    /// Checks if a plugin is registered
    pub async fn is_registered(&self, name: &str) -> bool {
        self.plugins.read().await.contains_key(name)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_registry() {
        let registry = PluginRegistry::new();

        let metadata = PluginMetadata {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
            description: "Test plugin".to_string(),
        };

        registry.register(metadata.clone()).await;
        assert!(registry.is_registered("test").await);

        let retrieved = registry.get("test").await.unwrap();
        assert_eq!(retrieved.name, "test");
        assert_eq!(retrieved.version, "0.1.0");

        registry.unregister("test").await;
        assert!(!registry.is_registered("test").await);
    }
}
