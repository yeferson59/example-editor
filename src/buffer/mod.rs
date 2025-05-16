//! Buffer management system for text editing

mod rope;
mod history;
mod operations;
mod markers;

pub use rope::Buffer;
pub use history::{History, HistoryEntry};
pub use operations::{Operation, TextOperation};
pub use markers::{Marker, MarkerSet};

use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

/// A text buffer with undo/redo support and markers
pub struct TextBuffer {
    /// The underlying text buffer
    buffer: Arc<RwLock<Buffer>>,
    /// Undo/redo history
    history: Arc<RwLock<History>>,
    /// Text markers
    markers: Arc<RwLock<MarkerSet>>,
    /// Line ending style
    line_ending: LineEnding,
    /// Indentation settings
    indentation: IndentationSettings,
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix style (\n)
    Unix,
    /// Windows style (\r\n)
    Windows,
    /// Mac style (\r)
    Mac,
}

/// Indentation settings
#[derive(Debug, Clone)]
pub struct IndentationSettings {
    /// Use spaces for indentation
    pub use_spaces: bool,
    /// Tab size in spaces
    pub tab_size: u8,
    /// Auto-indent
    pub auto_indent: bool,
}

impl TextBuffer {
    /// Creates a new text buffer
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(RwLock::new(Buffer::new())),
            history: Arc::new(RwLock::new(History::new())),
            markers: Arc::new(RwLock::new(MarkerSet::new())),
            line_ending: LineEnding::Unix,
            indentation: IndentationSettings::default(),
        }
    }

    /// Creates a buffer from existing text
    pub fn from_text(text: &str) -> Self {
        let mut buffer = Self::new();
        buffer.buffer = Arc::new(RwLock::new(Buffer::from_text(text)));
        buffer
    }

    /// Returns the current text content
    pub async fn text(&self) -> String {
        self.buffer.read().await.text()
    }

    /// Returns the length in bytes
    pub async fn len(&self) -> usize {
        self.buffer.read().await.len()
    }

    /// Returns true if the buffer is empty
    pub async fn is_empty(&self) -> bool {
        self.buffer.read().await.is_empty()
    }

    /// Inserts text at the specified position
    pub async fn insert(&mut self, position: usize, text: &str) -> Result<()> {
        let operation = TextOperation::Insert {
            position,
            text: text.to_string(),
        };

        // Apply the operation
        {
            let mut buffer = self.buffer.write().await;
            buffer.apply_operation(&operation)?;
        }

        // Record in history
        self.history.write().await.push(operation);

        Ok(())
    }

    /// Deletes text in the specified range
    pub async fn delete(&mut self, start: usize, end: usize) -> Result<()> {
        let text = {
            let buffer = self.buffer.read().await;
            buffer.slice(start..end).to_string()
        };

        let operation = TextOperation::Delete {
            start,
            end,
            text,
        };

        // Apply the operation
        {
            let mut buffer = self.buffer.write().await;
            buffer.apply_operation(&operation)?;
        }

        // Record in history
        self.history.write().await.push(operation);

        Ok(())
    }

    /// Undoes the last operation
    pub async fn undo(&mut self) -> Result<()> {
        if let Some(operation) = self.history.write().await.undo() {
            let mut buffer = self.buffer.write().await;
            buffer.apply_operation(&operation.invert())?;
        }
        Ok(())
    }

    /// Redoes the last undone operation
    pub async fn redo(&mut self) -> Result<()> {
        if let Some(operation) = self.history.write().await.redo() {
            let mut buffer = self.buffer.write().await;
            buffer.apply_operation(&operation)?;
        }
        Ok(())
    }

    /// Sets a marker at the specified position
    pub async fn set_marker(&mut self, name: &str, position: usize) {
        self.markers.write().await.set(name, position);
    }

    /// Gets the position of a marker
    pub async fn get_marker(&self, name: &str) -> Option<usize> {
        self.markers.read().await.get(name)
    }

    /// Removes a marker
    pub async fn remove_marker(&mut self, name: &str) {
        self.markers.write().await.remove(name);
    }

    /// Sets the line ending style
    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    /// Sets the indentation settings
    pub fn set_indentation(&mut self, settings: IndentationSettings) {
        self.indentation = settings;
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for IndentationSettings {
    fn default() -> Self {
        Self {
            use_spaces: true,
            tab_size: 4,
            auto_indent: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_operations() {
        let mut buffer = TextBuffer::new();

        // Test insert
        buffer.insert(0, "Hello").await.unwrap();
        assert_eq!(buffer.text().await, "Hello");

        buffer.insert(5, ", World!").await.unwrap();
        assert_eq!(buffer.text().await, "Hello, World!");

        // Test delete
        buffer.delete(5, 7).await.unwrap();
        assert_eq!(buffer.text().await, "HelloWorld!");

        // Test undo/redo
        buffer.undo().await.unwrap();
        assert_eq!(buffer.text().await, "Hello, World!");

        buffer.redo().await.unwrap();
        assert_eq!(buffer.text().await, "HelloWorld!");
    }

    #[tokio::test]
    async fn test_markers() {
        let mut buffer = TextBuffer::new();
        buffer.insert(0, "Hello, World!").await.unwrap();

        // Test marker operations
        buffer.set_marker("start", 0).await;
        buffer.set_marker("end", 5).await;

        assert_eq!(buffer.get_marker("start").await, Some(0));
        assert_eq!(buffer.get_marker("end").await, Some(5));

        buffer.remove_marker("start").await;
        assert_eq!(buffer.get_marker("start").await, None);
    }
}
