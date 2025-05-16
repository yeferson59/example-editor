//! LSP configuration

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use lsp_types::Url;
use crate::Result;

lazy_static::lazy_static! {
    static ref SERVERS: Arc<RwLock<HashMap<String, LspConfig>>> = Arc::new(RwLock::new(HashMap::new()));
}

/// LSP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// Server name
    pub name: String,
    /// Language ID
    pub language_id: String,
    /// Command to start the server
    pub command: String,
    /// Root URI for the workspace
    pub root_uri: Url,
    /// Server-specific initialization options
    pub initialization_options: Option<serde_json::Value>,
}

impl LspConfig {
    /// Creates a new LSP configuration
    pub fn new(
        name: impl Into<String>,
        language_id: impl Into<String>,
        command: impl Into<String>,
        root_uri: Url,
    ) -> Self {
        Self {
            name: name.into(),
            language_id: language_id.into(),
            command: command.into(),
            root_uri,
            initialization_options: None,
        }
    }

    /// Sets initialization options
    pub fn with_initialization_options(mut self, options: serde_json::Value) -> Self {
        self.initialization_options = Some(options);
        self
    }
}

/// Registers a language server configuration
pub fn register_server(config: LspConfig) {
    SERVERS.write().insert(config.language_id.clone(), config);
}

/// Gets a language server configuration by language ID
#[allow(dead_code)]
pub fn get_server(language_id: &str) -> Option<LspConfig> {
    SERVERS.read().get(language_id).cloned()
}

/// Registers default language server configurations
pub fn register_default_servers() -> Result<()> {
    // Example root URI - in practice, this would be set per-project
    let root_uri = Url::from_file_path("/").unwrap();

    // Rust
    register_server(LspConfig::new(
        "rust-analyzer",
        "rust",
        "rust-analyzer",
        root_uri.clone(),
    ).with_initialization_options(serde_json::json!({
        "checkOnSave": {
            "command": "clippy"
        }
    })));

    // Python
    register_server(LspConfig::new(
        "python-language-server",
        "python",
        "pylsp",
        root_uri.clone(),
    ));

    // JavaScript/TypeScript
    register_server(LspConfig::new(
        "typescript-language-server",
        "typescript",
        "typescript-language-server --stdio",
        root_uri.clone(),
    ));

    register_server(LspConfig::new(
        "typescript-language-server",
        "javascript",
        "typescript-language-server --stdio",
        root_uri.clone(),
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_registration() {
        let root_uri = Url::from_file_path("/tmp").unwrap();
        let config = LspConfig::new(
            "test-server",
            "test-lang",
            "test-command",
            root_uri,
        );

        register_server(config.clone());
        
        let retrieved = get_server("test-lang").unwrap();
        assert_eq!(retrieved.name, "test-server");
        assert_eq!(retrieved.command, "test-command");
    }

    #[test]
    fn test_default_servers() {
        register_default_servers().unwrap();
        
        assert!(get_server("rust").is_some());
        assert!(get_server("python").is_some());
        assert!(get_server("typescript").is_some());
        assert!(get_server("javascript").is_some());
    }
}
