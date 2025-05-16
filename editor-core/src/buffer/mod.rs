//! Buffer management module
//!
//! Provides efficient text buffer implementation using rope data structure

use ropey::Rope;
use parking_lot::RwLock;
use std::sync::Arc;
use crate::Result;

/// Represents a text buffer with efficient manipulation capabilities
pub struct Buffer {
    /// The underlying rope data structure for text storage
    content: Arc<RwLock<Rope>>,
    /// Path to the file associated with this buffer, if any
    path: Option<std::path::PathBuf>,
    /// Flag indicating if the buffer has unsaved changes
    dirty: bool,
}

impl Buffer {
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Self {
            content: Arc::new(RwLock::new(Rope::new())),
            path: None,
            dirty: false,
        }
    }

    /// Creates a new buffer from existing text
    pub fn from_text(text: &str) -> Self {
        Self {
            content: Arc::new(RwLock::new(Rope::from_str(text))),
            path: None,
            dirty: false,
        }
    }

    /// Creates a new buffer from a file
    pub fn from_file(path: impl Into<std::path::PathBuf>) -> Result<Self> {
        let path = path.into();
        let text = std::fs::read_to_string(&path)?;
        Ok(Self {
            content: Arc::new(RwLock::new(Rope::from_str(&text))),
            path: Some(path),
            dirty: false,
        })
    }

    /// Returns the current content of the buffer as a string
    pub fn text(&self) -> String {
        self.content.read().to_string()
    }

    /// Inserts text at the specified byte offset
    pub fn insert(&mut self, offset: usize, text: &str) -> Result<()> {
        let mut content = self.content.write();
        content.insert(offset, text);
        self.dirty = true;
        Ok(())
    }

    /// Deletes text in the specified byte range
    pub fn delete(&mut self, start: usize, end: usize) -> Result<()> {
        let mut content = self.content.write();
        content.remove(start..end);
        self.dirty = true;
        Ok(())
    }

    /// Returns the length of the buffer in bytes
    pub fn len(&self) -> usize {
        self.content.read().len_bytes()
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the buffer has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Saves the buffer content to its associated file
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.path {
            std::fs::write(path, self.text())?;
            self.dirty = false;
        }
        Ok(())
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_operations() {
        let mut buffer = Buffer::new();
        
        // Test insert
        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.text(), "Hello");
        
        // Test append
        buffer.insert(5, " World").unwrap();
        assert_eq!(buffer.text(), "Hello World");
        
        // Test delete
        buffer.delete(5, 6).unwrap();  // Delete space
        assert_eq!(buffer.text(), "HelloWorld");
        
        // Test length
        assert_eq!(buffer.len(), 10);
        
        // Test dirty flag
        assert!(buffer.is_dirty());
    }
}
