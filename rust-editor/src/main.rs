use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use editor_core::editor::Editor;

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

    // Create a new editor instance
    let mut editor = Editor::new();

    // If no files were specified, create an "untitled" document
    if args.files.is_empty() {
        editor.new_document("untitled-1")?;
    } else {
        // Open any specified files
        for path in args.files {
            editor.open_file(&path)?;
        }
    }

    // Start the editor UI
    editor_ui::run(editor)?;

    Ok(())
}
