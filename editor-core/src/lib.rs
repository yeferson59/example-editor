//! Core text editing engine for rust-editor

mod buffer;
mod document;
pub mod editor;
mod event;

pub use buffer::Buffer;
pub use document::Document;
pub use editor::Editor;
pub use event::{Event, EventHandler};

/// Result type for editor operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for editor operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Buffer error: {0}")]
    Buffer(String),
    
    #[error("Document error: {0}")]
    Document(String),
    
    #[error("Event error: {0}")]
    Event(String),
}

/// Creates a new buffer with the given text
pub fn create_buffer(text: &str) -> Buffer {
    Buffer::from_text(text)
}

/// Creates a new document with the given name
pub fn create_document(name: &str) -> Document {
    Document::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_buffer() {
        let buffer = create_buffer("Hello, World!");
        assert_eq!(buffer.text(), "Hello, World!");
    }

    #[test]
    fn test_create_document() {
        let doc = create_document("test.txt");
        assert_eq!(doc.name(), "test.txt");
    }
}
