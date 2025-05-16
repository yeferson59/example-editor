use crate::{Document, Result, Error};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main editor type that coordinates documents and editing operations
pub struct Editor {
    /// Currently open documents
    documents: HashMap<String, Document>,
    /// Currently active document
    active_document: Option<String>,
}

impl Editor {
    /// Creates a new empty editor
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            active_document: None,
        }
    }

    /// Opens a document from a file.
    ///
    /// This method loads a document from the specified file path and adds it to the editor.
    /// The newly opened document will be set as the active document.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to open. This can be any type that can be converted
    ///            into a PathBuf and referenced as a Path.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the file was successfully opened
    /// * `Err(_)` if the file could not be opened or read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// // Open a file using a string path
    /// editor.open_file("path/to/file.txt").unwrap();
    ///
    /// // Or using a PathBuf
    /// let path = std::path::PathBuf::from("path/to/another_file.txt");
    /// editor.open_file(path).unwrap();
    /// ```
    pub fn open_file(&mut self, path: impl Into<PathBuf> + AsRef<std::path::Path>) -> Result<()> {
        let doc = Document::from_file(path)?;
        let name = doc.name().to_string();
        self.documents.insert(name.clone(), doc);
        self.active_document = Some(name);
        Ok(())
    }

    /// Creates a new empty document with the given name.
    ///
    /// This method creates a new empty document and adds it to the editor.
    /// The newly created document will be set as the active document.
    /// If a document with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the new document.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the document was successfully created
    /// * `Err(_)` if there was an error creating the document
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// editor.new_document("new_file.txt").unwrap();
    /// ```
    pub fn new_document(&mut self, name: &str) -> Result<()> {
        if self.documents.contains_key(name) {
            log::info!("Document with name '{}' already exists and will be replaced", name);
        }
        
        log::info!("Creating new document '{}'", name);
        let doc = Document::new(name);
        self.documents.insert(name.to_string(), doc);
        self.active_document = Some(name.to_string());
        Ok(())
    }

    /// Returns a reference to the active document, if any.
    ///
    /// # Returns
    ///
    /// * `Some(&Document)` if there is an active document
    /// * `None` if there is no active document
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("test.txt").unwrap();
    /// if let Some(doc) = editor.active_document() {
    ///     println!("Active document: {}", doc.name());
    /// }
    /// ```
    pub fn active_document(&self) -> Option<&Document> {
        self.active_document
            .as_ref()
            .and_then(|name| self.documents.get(name))
    }

    /// Returns a mutable reference to the active document, if any.
    ///
    /// # Returns
    ///
    /// * `Some(&mut Document)` if there is an active document
    /// * `None` if there is no active document
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("test.txt").unwrap();
    /// if let Some(doc) = editor.active_document_mut() {
    ///     // Perform some operation on the document
    ///     // doc.insert_text(...);
    /// }
    /// ```
    pub fn active_document_mut(&mut self) -> Option<&mut Document> {
        // Avoid cloning the string by using as_deref
        let name = self.active_document.as_deref()?;
        self.documents.get_mut(name)
    }

    /// Sets the specified document as the active document.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the document to set as active.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the document was successfully set as active
    /// * `Err(_)` if no document with the given name exists
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("doc1.txt").unwrap();
    /// # editor.new_document("doc2.txt").unwrap();
    /// // Switch to a different document
    /// editor.set_active_document("doc1.txt").unwrap();
    /// ```
    pub fn set_active_document(&mut self, name: &str) -> Result<()> {
        if !self.documents.contains_key(name) {
            return Err(Error::Document(format!("Document not found: {}", name)));
        }
        self.active_document = Some(name.to_string());
        Ok(())
    }

    /// Returns a list of names of all open documents.
    ///
    /// # Returns
    ///
    /// A vector containing the names of all currently open documents.
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("doc1.txt").unwrap();
    /// # editor.new_document("doc2.txt").unwrap();
    /// let names = editor.document_names();
    /// assert!(names.contains(&"doc1.txt".to_string()));
    /// assert!(names.contains(&"doc2.txt".to_string()));
    /// ```
    pub fn document_names(&self) -> Vec<String> {
        self.documents.keys().cloned().collect()
    }

    /// Checks if a document with the given name exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the document to check.
    ///
    /// # Returns
    ///
    /// `true` if a document with the given name exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("doc1.txt").unwrap();
    /// assert!(editor.has_document("doc1.txt"));
    /// assert!(!editor.has_document("nonexistent.txt"));
    /// ```
    pub fn has_document(&self, name: &str) -> bool {
        self.documents.contains_key(name)
    }

    /// Closes the document with the given name.
    ///
    /// If the document being closed is the active document, the active document
    /// will be set to None.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the document to close.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the document was successfully closed
    /// * `Err(_)` if no document with the given name exists
    ///
    /// # Examples
    ///
    /// ```
    /// # use editor_core::Editor;
    /// # let mut editor = Editor::new();
    /// # editor.new_document("doc1.txt").unwrap();
    /// editor.close_document("doc1.txt").unwrap();
    /// assert!(!editor.has_document("doc1.txt"));
    /// ```
    pub fn close_document(&mut self, name: &str) -> Result<()> {
        if !self.documents.contains_key(name) {
            return Err(Error::Document(format!("Cannot close document: {} not found", name)));
        }
        
        self.documents.remove(name);
        
        // If the closed document was the active one, set active to None
        if self.active_document.as_deref() == Some(name) {
            self.active_document = None;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    /// Creates a temporary file with the given content and name
    fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    #[test]
    /// Test creating a new empty editor
    fn test_new_editor() {
        let editor = Editor::new();
        assert!(editor.documents.is_empty());
        assert!(editor.active_document.is_none());
    }

    #[test]
    /// Test basic editor operations
    fn test_editor_operations() {
        let mut editor = Editor::new();
        
        // Test new document
        editor.new_document("test.txt").unwrap();
        assert_eq!(editor.document_names(), vec!["test.txt"]);
        
        // Test active document
        assert!(editor.active_document().is_some());
        assert_eq!(editor.active_document().unwrap().name(), "test.txt");
    }

    #[test]
    /// Test close document functionality
    fn test_close_document() {
        let mut editor = Editor::new();
        
        // Create and then close a document
        editor.new_document("doc1.txt").unwrap();
        assert!(editor.has_document("doc1.txt"));
        assert_eq!(editor.active_document().unwrap().name(), "doc1.txt");
        
        editor.close_document("doc1.txt").unwrap();
        assert!(!editor.has_document("doc1.txt"));
        assert!(editor.active_document().is_none());
    }
}

