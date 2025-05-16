//! Syntax highlighting and parsing module for rust-editor
//!
//! Provides syntax highlighting and code analysis using tree-sitter

mod highlighter;
mod language;
mod parser;
mod theme;

pub use highlighter::{Highlighter, HighlightEvent};
pub use language::{Language, LanguageConfig, get_language_by_extension};
pub use parser::Parser;
pub use theme::{Theme, Style};

use thiserror::Error;

/// Error type for syntax-related operations
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Parser error: {0}")]
    ParserError(String),
    
    #[error("Highlighting error: {0}")]
    HighlightError(String),
    
    #[error("Theme error: {0}")]
    ThemeError(String),
}

/// Result type for syntax operations
pub type Result<T> = std::result::Result<T, SyntaxError>;

/// Initializes syntax highlighting support
pub fn init() -> Result<()> {
    language::register_default_languages()?;
    Ok(())
}
