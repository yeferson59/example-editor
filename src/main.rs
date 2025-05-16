//! Rust Editor - A modern text editor written in Rust

mod buffer;
mod input;
mod state;
mod event;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to open
    #[arg(name = "FILE")]
    files: Vec<PathBuf>,

    /// Start in read-only mode
    #[arg(short = 'R', long)]
    readonly: bool,

    /// Configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Theme to use
    #[arg(long)]
    theme: Option<String>,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: log::LevelFilter,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    env_logger::Builder::new()
        .filter_level(args.log_level)
        .init();

    log::info!("Starting Rust Editor...");

    // Initialize state
    let state = state::EditorState::new()?;

    // Load configuration
    if let Some(config_path) = args.config {
        log::info!("Loading configuration from: {}", config_path.display());
        // TODO: Load custom configuration
    }

    // Set theme if specified
    if let Some(theme_name) = args.theme {
        log::info!("Setting theme: {}", theme_name);
        let mut prefs = state.preferences().write().await;
        // TODO: Load and set theme
    }

    // Create editor instance
    let mut editor = Editor::new(state);

    // Open initial files
    for path in args.files {
        if let Err(e) = editor.open_file(&path).await {
            log::error!("Failed to open {}: {}", path.display(), e);
        }
    }

    // Initialize UI
    editor_ui::run(editor)?;

    Ok(())
}

/// Main editor struct
pub struct Editor {
    /// Editor state
    state: state::EditorState,
    /// Event dispatcher
    events: event::EventDispatcher,
    /// Input handler
    input: input::InputHandler,
}

impl Editor {
    /// Creates a new editor instance
    pub fn new(state: state::EditorState) -> Self {
        Self {
            state,
            events: event::EventDispatcher::new(),
            input: input::InputHandler::new(),
        }
    }

    /// Opens a file
    pub async fn open_file(&mut self, path: &PathBuf) -> Result<()> {
        log::info!("Opening file: {}", path.display());

        // Create new buffer
        let mut buffer = buffer::TextBuffer::new();

        // Read file content
        let content = tokio::fs::read_to_string(path).await?;
        buffer.insert(0, &content).await?;

        // Create document
        let mut session = self.state.session().write().await;
        session.add_file(path.clone());

        // Update cursor position if file was previously open
        if let Some(pos) = session.cursor_positions.get(path) {
            buffer.set_marker("cursor", pos.line).await;
        }

        Ok(())
    }

    /// Saves the current file
    pub async fn save_file(&mut self) -> Result<()> {
        let session = self.state.session().read().await;
        if let Some(path) = &session.active_file {
            log::info!("Saving file: {}", path.display());
            
            // Get buffer content
            // TODO: Get active buffer content
            
            // Save file
            // TODO: Save file content
            
            // Update state
            self.events.dispatch(event::EditorEvent::Document(
                event::DocumentEvent::Save {
                    path: path.clone(),
                }
            )).await;
        }
        Ok(())
    }

    /// Handles input events
    pub async fn handle_input(&mut self, event: input::KeyEvent) -> Result<()> {
        if let Some(command) = self.input.handle_key(event) {
            // Execute command
            command.execute().await?;
        }
        Ok(())
    }

    /// Returns the editor state
    pub fn state(&self) -> &state::EditorState {
        &self.state
    }
}
