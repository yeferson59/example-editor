//! Plugin loader implementation

use std::path::{Path, PathBuf};
use crate::{Plugin, PluginConfig, PluginError, Result, PluginType};
use crate::native::NativePlugin;
use crate::wasm::WasmPlugin;

/// Plugin loader error
#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("Failed to load plugin: {0}")]
    LoadFailed(String),

    #[error("Invalid plugin format: {0}")]
    InvalidFormat(String),

    #[error("Missing plugin manifest")]
    MissingManifest,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Plugin loader
pub struct PluginLoader {
    /// Plugin search paths
    search_paths: Vec<PathBuf>,
}

impl PluginLoader {
    /// Creates a new plugin loader
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
        }
    }

    /// Adds a search path
    pub fn add_search_path(&mut self, path: impl AsRef<Path>) {
        self.search_paths.push(path.as_ref().to_path_buf());
    }

    /// Loads a plugin from a path
    pub async fn load(&self, path: impl AsRef<Path>) -> Result<Box<dyn Plugin>> {
        let path = path.as_ref();
        let config = self.load_config(path)?;

        match config.manifest.plugin_type {
            PluginType::Native => {
                let plugin = NativePlugin::load(path, config).await?;
                Ok(Box::new(plugin))
            }
            PluginType::Wasm => {
                let plugin = WasmPlugin::load(path, config).await?;
                Ok(Box::new(plugin))
            }
        }
    }

    /// Loads plugin configuration from a path
    fn load_config(&self, path: &Path) -> Result<PluginConfig> {
        let manifest_path = path.join("plugin.json");
        if !manifest_path.exists() {
            return Err(PluginError::ManifestError("Missing plugin.json".to_string()));
        }

        let manifest_contents = std::fs::read_to_string(&manifest_path)?;
        let manifest = serde_json::from_str(&manifest_contents)
            .map_err(|e| PluginError::ManifestError(e.to_string()))?;

        Ok(PluginConfig::new(manifest))
    }

    /// Discovers plugins in search paths
    pub async fn discover(&self) -> Result<Vec<Box<dyn Plugin>>> {
        let mut plugins = Vec::new();

        for path in &self.search_paths {
            if !path.exists() || !path.is_dir() {
                continue;
            }

            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    if let Ok(plugin) = self.load(&path).await {
                        plugins.push(plugin);
                    }
                }
            }
        }

        Ok(plugins)
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_plugin_loading() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path().join("test-plugin");
        fs::create_dir(&plugin_dir).unwrap();

        // Create a test plugin manifest
        let manifest = serde_json::json!({
            "name": "test-plugin",
            "version": "0.1.0",
            "description": "Test plugin",
            "author": "Test Author",
            "license": "MIT",
            "entry_point": "lib.rs",
            "plugin_type": "Native",
            "dependencies": [],
            "permissions": []
        });

        fs::write(
            plugin_dir.join("plugin.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        ).unwrap();

        let mut loader = PluginLoader::new();
        loader.add_search_path(temp_dir.path());

        let plugins = loader.discover().await.unwrap();
        assert_eq!(plugins.len(), 1);
    }
}
