//! LSP server implementation

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer as LspServer};

/// Language server implementation
pub struct LanguageServer {
    /// Client instance for communicating with the editor
    client: Client,
    /// Document storage
    documents: Arc<RwLock<HashMap<Url, DocumentState>>>,
    /// Server state
    state: Arc<Mutex<ServerState>>,
}

/// Document state
#[derive(Debug, Clone)]
struct DocumentState {
    /// Document version
    version: i32,
    /// Document text content
    content: String,
    /// Document language ID
    language_id: String,
}

/// Server state
struct ServerState {
    /// Root workspace URI
    root_uri: Option<Url>,
}

impl LanguageServer {
    /// Creates a new language server
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(Mutex::new(ServerState { root_uri: None })),
        }
    }
}

#[tower_lsp::async_trait]
impl LspServer for LanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Log initialization
        let root_uri = params
            .root_uri
            .clone()
            .map_or_else(|| String::from("None"), |uri| uri.to_string());

        self.client
            .log_message(
                MessageType::INFO,
                format!("Initializing language server with root URI: {}", root_uri),
            )
            .await;

        let mut state = self.state.lock().await;
        state.root_uri = params.root_uri;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                definition_provider: Some(OneOf::Left(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "rust-editor-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = DocumentState {
            version: params.text_document.version,
            content: params.text_document.text,
            language_id: params.text_document.language_id,
        };

        let uri = params.text_document.uri.clone();

        // Scope the lock to drop it before await
        {
            self.documents.write().insert(uri.clone(), document);
        }

        // Now we can await without holding the lock
        self.client
            .log_message(MessageType::INFO, format!("Document opened: {}", uri))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;
        let changes = params.content_changes;
        let mut changed = false;

        // Scope the lock to drop it before await
        {
            let mut documents = self.documents.write();

            if let Some(doc_state) = documents.get_mut(&uri) {
                // Update version
                doc_state.version = version;

                // Apply changes - simplified to always use the last change text
                if let Some(last_change) = changes.last() {
                    doc_state.content = last_change.text.clone();
                    changed = true;
                }
            }
        }

        // Now we can await without holding the lock
        if changed {
            self.client
                .log_message(MessageType::INFO, format!("Document changed: {}", uri))
                .await;
        } else {
            self.client
                .log_message(
                    MessageType::WARNING,
                    format!("Changed unknown document: {}", uri),
                )
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        // Scope the lock to drop it before await
        {
            self.documents.write().remove(&uri);
        }

        // Now we can await without holding the lock
        self.client
            .log_message(MessageType::INFO, format!("Document closed: {}", uri))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.clone();
        let _position = params.text_document_position.position;

        let documents = self.documents.read();
        if let Some(doc) = documents.get(&uri) {
            // Provide basic completion items based on language ID
            let items = match doc.language_id.as_str() {
                "rust" => {
                    vec![
                        CompletionItem {
                            label: "fn".to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            detail: Some("fn - Function definition".to_string()),
                            insert_text: Some("fn $1($2) {\n    $0\n}".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                        CompletionItem {
                            label: "struct".to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            detail: Some("struct - Structure definition".to_string()),
                            insert_text: Some("struct $1 {\n    $0\n}".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                    ]
                }
                _ => Vec::new(),
            };

            return Ok(Some(CompletionResponse::Array(items)));
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let position = params.text_document_position_params.position;

        let documents = self.documents.read();
        if let Some(doc) = documents.get(&uri) {
            // Basic hover information based on language ID
            match doc.language_id.as_str() {
                "rust" => {
                    // For demonstration, return simple hover info
                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: format!(
                                "**Position**: line {}, character {}\n\n**Language**: Rust",
                                position.line, position.character
                            ),
                        }),
                        range: None,
                    }));
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let _position = params.text_document_position_params.position;
        let context = params.context.as_ref();

        let documents = self.documents.read();
        if let Some(doc) = documents.get(&uri) {
            match doc.language_id.as_str() {
                "rust" => {
                    // Basic signature help for demonstration
                    return Ok(Some(SignatureHelp {
                        signatures: vec![SignatureInformation {
                            label: "fn example(param1: i32, param2: &str) -> bool".to_string(),
                            documentation: Some(Documentation::MarkupContent(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: "Example function signature".to_string(),
                            })),
                            parameters: Some(vec![
                                ParameterInformation {
                                    label: ParameterLabel::Simple("param1: i32".to_string()),
                                    documentation: Some(Documentation::MarkupContent(
                                        MarkupContent {
                                            kind: MarkupKind::Markdown,
                                            value: "First parameter".to_string(),
                                        },
                                    )),
                                },
                                ParameterInformation {
                                    label: ParameterLabel::Simple("param2: &str".to_string()),
                                    documentation: Some(Documentation::MarkupContent(
                                        MarkupContent {
                                            kind: MarkupKind::Markdown,
                                            value: "Second parameter".to_string(),
                                        },
                                    )),
                                },
                            ]),
                            active_parameter: context
                                .and_then(|ctx| ctx.active_signature_help.as_ref())
                                .and_then(|help| help.active_parameter),
                        }],
                        active_signature: Some(0),
                        active_parameter: context
                            .and_then(|ctx| ctx.active_signature_help.as_ref())
                            .and_then(|help| help.active_parameter),
                    }));
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let position = params.text_document_position_params.position;

        let documents = self.documents.read();
        if let Some(_doc) = documents.get(&uri) {
            // For demonstration, return a mock definition location
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: position.line,
                        character: 0,
                    },
                    end: Position {
                        line: position.line + 1,
                        character: 0,
                    },
                },
            })));
        }

        Ok(None)
    }
}
