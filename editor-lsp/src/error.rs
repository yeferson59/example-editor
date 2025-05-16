//! Error types for LSP functionality

use thiserror::Error;
use tower_lsp::jsonrpc;

/// LSP error types
#[derive(Debug, Error)]
pub enum Error {
    /// Error during LSP initialization
    #[error("LSP initialization error: {0}")]
    InitializationError(String),

    /// Error from the LSP server
    #[error("LSP server error: {0}")]
    ServerError(String),

    /// Error during an LSP request
    #[error("LSP request error: {0}")]
    RequestError(String),

    /// JSON-RPC error
    #[error("JSON-RPC error: {0}")]
    JsonRpcError(#[from] jsonrpc::Error),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for LSP operations
pub type Result<T> = std::result::Result<T, Error>;

