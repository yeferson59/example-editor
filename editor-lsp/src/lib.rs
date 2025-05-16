//! Language Server Protocol support for rust-editor
//!
//! Provides LSP client implementation for code intelligence features

mod client;
mod config;
mod server;
mod types;

pub use client::LspClient;
pub use config::LspConfig;
pub use types::{Error, LspError, Result};
pub use types::{
    CompletionItem,
    CompletionItemKind,
    CompletionResponse,
    Position,
    Range,
    TextDocumentContentChangeEvent,
    TextDocumentItem,
    Url,
};

/// Initializes LSP support
pub async fn init() -> Result<()> {
    // Register default language servers
    config::register_default_servers()?;
    Ok(())
}
