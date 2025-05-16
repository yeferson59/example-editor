//! Document management module
//!
//! Provides document abstraction that manages buffers and maintains document metadata

use crate::buffer::Buffer;
use crate::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use std::path::{Path, PathBuf};

/// Represents metadata about a document
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    /// The name of the document
    pub name: String,
    /// The file path, if the document is associated with a file
    pub path: Option<PathBuf>,
    /// The line ending style (Unix, Windows, etc.)
    pub line_ending: LineEnding,
    /// The document's language/file type
    pub language: Option<String>,
}

/// Represents different line ending styles
#[derive(Debug, Clone, PartialEq)]
pub enum LineEnding {
    Unix,    // \n
    Windows, // \r\n
    Mac,     // \r
}

impl LineEnding {
    /// Detects the line ending used in a string
    pub fn detect(text: &str) -> Self {
        // Quick check for common patterns
        if text.contains("\r\n") {
            return LineEnding::Windows;
        }
        
        if text.contains('\r') && !text.contains('\n') {
            return LineEnding::Mac;
        }
        
        // Default to Unix line endings
        LineEnding::Unix
    }
    
    /// Converts the line ending to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Mac => "\r",
        }
    }
    
    /// Normalizes text to use this line ending style
    pub fn normalize(&self, text: &str) -> String {
        // First normalize all line endings to Unix style
        let unix_text = text
            .replace("\r\n", "\n")
            .replace("\r", "\n");
            
        // Then convert to the target line ending if needed
        match self {
            LineEnding::Unix => unix_text,
            LineEnding::Windows => unix_text.replace("\n", "\r\n"),
            LineEnding::Mac => unix_text.replace("\n", "\r"),
        }
    }
}

impl Default for LineEnding {
    fn default() -> Self {
        #[cfg(windows)]
        return LineEnding::Windows;
        #[cfg(not(windows))]
        return LineEnding::Unix;
    }
}

/// Represents a document in the editor
pub struct Document {
    /// The document's buffer containing the actual text
    buffer: Arc<RwLock<Buffer>>,
    /// Document metadata
    metadata: DocumentMetadata,
    /// Version number for change tracking
    version: u64,
}

impl Document {
    /// Creates a new empty document
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let language = Path::new(&name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string());

        Self {
            buffer: Arc::new(RwLock::new(Buffer::new())),
            metadata: DocumentMetadata {
                name,
                path: None,
                line_ending: LineEnding::default(),
                language,
            },
            version: 0,
        }
    }

    /// Creates a new document from a file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let buffer = Buffer::from_file(path)?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let language = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string());
            
        // Detect the line ending from the buffer content
        let content = buffer.text();
        let line_ending = LineEnding::detect(&content);

        Ok(Self {
            buffer: Arc::new(RwLock::new(buffer)),
            metadata: DocumentMetadata {
                name,
                path: Some(path.to_path_buf()),
                line_ending,
                language,
            },
            version: 0,
        })
    }

    /// Returns the document's name
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Returns the document's path, if any
    pub fn path(&self) -> Option<&Path> {
        self.metadata.path.as_deref()
    }

    /// Returns the document's language, if any
    pub fn language(&self) -> Option<&str> {
        self.metadata.language.as_deref()
    }
    
    /// Returns the line ending style used by this document
    pub fn line_ending(&self) -> &LineEnding {
        &self.metadata.line_ending
    }
    
    /// Sets the line ending style for this document
    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.metadata.line_ending = line_ending;
    }

    /// Returns the current version number
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Returns the document's content as a string
    pub fn text(&self) -> String {
        self.buffer.read().text()
    }

    /// Inserts text at the specified position
    pub fn insert(&mut self, position: usize, text: &str) -> Result<()> {
        self.buffer.write().insert(position, text)?;
        self.version += 1;
        Ok(())
    }

    /// Deletes text in the specified range
    pub fn delete(&mut self, start: usize, end: usize) -> Result<()> {
        self.buffer.write().delete(start, end)?;
        self.version += 1;
        Ok(())
    }

    /// Saves the document to its file
    pub fn save(&mut self) -> Result<()> {
        // Before saving, normalize line endings if needed
        if let Some(_path) = &self.metadata.path {
            let text = self.text();
            let normalized_text = self.metadata.line_ending.normalize(&text);
            
            // Only rewrite if line endings changed
            if normalized_text != text {
                let mut buffer = self.buffer.write();
                buffer.delete(0, text.len())?;  // Clear existing content
                buffer.insert(0, &normalized_text)?;  // Insert normalized content
            }
        }
        
        self.buffer.write().save()
    }
    
    /// Normalizes the document's line endings to the specified style
    pub fn normalize_line_endings(&mut self, line_ending: LineEnding) -> Result<()> {
        let text = self.text();
        let normalized_text = line_ending.normalize(&text);
        
        // Only update if there were changes
        if normalized_text != text {
            let mut buffer = self.buffer.write();
            buffer.delete(0, text.len())?;  // Clear existing content
            buffer.insert(0, &normalized_text)?;  // Insert normalized content
            self.metadata.line_ending = line_ending;
            self.version += 1;
        }
        
        Ok(())
    }

    /// Returns true if the document has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.buffer.read().is_dirty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_operations() {
        let mut doc = Document::new("test.txt");
        
        // Test insert
        doc.insert(0, "Hello").unwrap();
        assert_eq!(doc.text(), "Hello");
        
        // Test version increment
        assert_eq!(doc.version(), 1);
        
        // Test delete
        doc.delete(0, 1).unwrap();
        assert_eq!(doc.text(), "ello");
        assert_eq!(doc.version(), 2);
        
        // Test metadata
        assert_eq!(doc.name(), "test.txt");
        assert!(doc.is_dirty());
    }
    
    #[test]
    fn test_line_ending_detection() {
        assert_eq!(LineEnding::detect("hello\nworld"), LineEnding::Unix);
        assert_eq!(LineEnding::detect("hello\r\nworld"), LineEnding::Windows);
        assert_eq!(LineEnding::detect("hello\rworld"), LineEnding::Mac);
        assert_eq!(LineEnding::detect("no newlines"), LineEnding::Unix);
        
        // Mixed line endings should prioritize Windows (CRLF) as it's most specific
        assert_eq!(LineEnding::detect("hello\r\nworld\ntest"), LineEnding::Windows);
    }
    
    #[test]
    fn test_line_ending_normalization() {
        let unix_text = "line1\nline2\nline3";
        let windows_text = "line1\r\nline2\r\nline3";
        let mac_text = "line1\rline2\rline3";
        
        // Test normalization to different line ending styles
        assert_eq!(LineEnding::Unix.normalize(windows_text), unix_text);
        assert_eq!(LineEnding::Windows.normalize(unix_text), windows_text);
        assert_eq!(LineEnding::Mac.normalize(unix_text), mac_text);
        
        // Test normalization with mixed line endings
        let mixed_text = "line1\nline2\r\nline3\rline4";
        assert_eq!(LineEnding::Unix.normalize(mixed_text), "line1\nline2\nline3\nline4");
        assert_eq!(LineEnding::Windows.normalize(mixed_text), "line1\r\nline2\r\nline3\r\nline4");
        assert_eq!(LineEnding::Mac.normalize(mixed_text), "line1\rline2\rline3\rline4");
    }
}
