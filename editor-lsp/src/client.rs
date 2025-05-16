use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};
use lsp_types::*;
use crate::{Result, LspConfig, server};

/// LSP client for communicating with language servers
#[allow(dead_code)]
pub struct LspClient {
    /// The LSP service
    service: Arc<Mutex<LspService<server::LanguageServer>>>,
    /// Client configuration
    config: LspConfig,
    /// Server capabilities
    capabilities: Arc<Mutex<ServerCapabilities>>,
    /// Initialization status
    initialized: bool,
}

impl LspClient {
    /// Creates a new LSP client
    pub async fn new(config: LspConfig) -> Result<Self> {
        let (service, _socket) = LspService::build(|client| {
            server::LanguageServer::new(client)
        }).finish();
        
        let service_arc = Arc::new(Mutex::new(service));
        
        // Create a new service instance for the background task
        let (background_service, background_socket) = LspService::build(|client| {
            server::LanguageServer::new(client)
        }).finish();
        
        // Start LSP server in background using stdio
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        
        tokio::spawn(async move {
            let server = Server::new(stdin, stdout, background_socket);
            server.serve(background_service).await;
            log::info!("LSP server stopped");
        });

        Ok(Self {
            service: service_arc,
            config,
            capabilities: Arc::new(Mutex::new(ServerCapabilities::default())),
            initialized: false,
        })
    }

    // ... rest of implementation stays the same...
}