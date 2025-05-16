//! Plugin system for rust-editor
//!
//! Provides plugin loading and management functionality

mod loader;
mod manager;
mod native;
mod wasm;
mod registry;
mod sandbox;

pub use loader::{PluginLoader, LoaderError};
pub use manager::{PluginManager, PluginEvent};
pub use native::NativePlugin;
pub use wasm::WasmPlugin;
pub use registry::{PluginRegistry, PluginMetadata};
pub use sandbox::{Sandbox, SandboxConfig};

use thiserror::Error;
use std::path::PathBuf;

/// Plugin-related errors
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Failed to load plugin: {0}")]
    LoadError(String),

    #[error("Plugin initialization failed: {0}")]
    InitError(String),

    #[error("Plugin execution error: {0}")]
    ExecutionError(String),

    #[error("Invalid plugin manifest: {0}")]
    ManifestError(String),

    #[error("Sandbox error: {0}")]
    SandboxError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Result type for plugin operations
pub type Result<T> = std::result::Result<T, PluginError>;

/// Plugin interface that all plugins must implement
#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    /// Returns the plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initializes the plugin
    async fn initialize(&mut self) -> Result<()>;

    /// Shuts down the plugin
    async fn shutdown(&mut self) -> Result<()>;

    /// Executes a plugin command
    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value>;
}

/// Plugin manifest format
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginManifest {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin license
    pub license: String,
    /// Plugin entry point
    pub entry_point: String,
    /// Plugin type (native or wasm)
    pub plugin_type: PluginType,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Plugin permissions
    pub permissions: Vec<Permission>,
}

/// Plugin types
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PluginType {
    /// Native plugin (shared library)
    Native,
    /// WebAssembly plugin
    Wasm,
}

/// Plugin dependency
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginDependency {
    /// Dependency name
    pub name: String,
    /// Version requirement
    pub version_req: String,
}

/// Plugin permissions
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Permission {
    /// File system access
    FileSystem {
        /// Allowed paths
        paths: Vec<PathBuf>,
        /// Read-only access
        read_only: bool,
    },
    /// Network access
    Network {
        /// Allowed hosts
        hosts: Vec<String>,
        /// Allowed ports
        ports: Vec<u16>,
    },
    /// Process execution
    Process {
        /// Allowed commands
        commands: Vec<String>,
    },
}

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Plugin sandbox configuration
    pub sandbox: SandboxConfig,
}

impl PluginConfig {
    /// Creates a new plugin configuration
    pub fn new(manifest: PluginManifest) -> Self {
        Self {
            manifest,
            sandbox: SandboxConfig::default(),
        }
    }

    /// Sets the sandbox configuration
    pub fn with_sandbox(mut self, config: SandboxConfig) -> Self {
        self.sandbox = config;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest() {
        let manifest = PluginManifest {
            name: "test-plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            entry_point: "lib.rs".to_string(),
            plugin_type: PluginType::Native,
            dependencies: vec![],
            permissions: vec![
                Permission::FileSystem {
                    paths: vec![PathBuf::from("/tmp")],
                    read_only: true,
                },
            ],
        };

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: PluginManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test-plugin");
    }
}
