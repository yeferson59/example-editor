//! Common types for LSP functionality

use thiserror::Error;
use tower_lsp::jsonrpc;

/// LSP-related errors
#[derive(Error, Debug)]
pub enum Error {
    /// Error during LSP initialization
    #[error("LSP initialization failed: {0}")]
    InitializationError(String),
    
    /// Connection-related errors
    #[error("LSP connection error: {0}")]
    ConnectionError(String),
    
    /// Request-related errors
    #[error("LSP request failed: {0}")]
    RequestError(String),
    
    /// Server-side errors
    #[error("LSP server error: {0}")]
    ServerError(String),
    
    /// JSON-RPC errors
    #[error("JSON-RPC error: {0}")]
    JsonRpcError(#[from] jsonrpc::Error),
    
    /// IO errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Result type for LSP operations
pub type Result<T> = std::result::Result<T, Error>;

/// LSP-specific error type alias
pub type LspError = Error;

// Re-export commonly used LSP types
pub use tower_lsp::lsp_types::{
    CompletionItem,
    CompletionItemKind,
    CompletionResponse,
    Position,
    Range,
    TextDocumentContentChangeEvent,
    TextDocumentItem,
    Url,
};
