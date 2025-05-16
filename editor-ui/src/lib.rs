//! GUI implementation for rust-editor

mod app;
mod theme;

pub use crate::app::run;
pub use crate::theme::Theme;

use editor_core::Error as CoreError;
use std::io;

/// UI error type
#[derive(thiserror::Error, Debug)]
pub enum UiError {
    #[error("Failed to initialize UI: {0}")]
    InitializationError(String),

    /// Core editor errors
    #[error("Editor error: {0}")]
    EditorError(#[from] CoreError),

    /// File system errors
    #[error("File system error: {0}")]
    IoError(#[from] io::Error),
}

// Allow conversion from eframe error to UiError
impl From<eframe::Error> for UiError {
    fn from(err: eframe::Error) -> Self {
        UiError::InitializationError(err.to_string())
    }
}
