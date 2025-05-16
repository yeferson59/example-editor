//! Common test utilities

use std::sync::Arc;
use tokio::sync::mpsc;
use editor_core::Event;
use async_trait::async_trait;

/// Test event handler
pub struct TestEventHandler {
    sender: mpsc::Sender<Event>,
}

impl TestEventHandler {
    /// Creates a new test event handler
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl editor_core::EventHandler for TestEventHandler {
    async fn handle(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}

/// Creates a test rust file
pub fn create_test_rust_file(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), content).unwrap();
    file
}

/// Creates a test workspace
pub struct TestWorkspace {
    /// Temporary directory
    pub dir: tempfile::TempDir,
}

impl TestWorkspace {
    /// Creates a new test workspace
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
        }
    }

    /// Creates a file in the workspace
    pub fn create_file(&self, name: &str, content: &str) -> std::path::PathBuf {
        let path = self.dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}

impl Default for TestWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

/// Test utilities for documents
pub trait TestDocumentExt {
    /// Creates a test document with content
    fn create_test(name: &str, content: &str) -> Self;
}

impl TestDocumentExt for editor_core::Document {
    fn create_test(name: &str, content: &str) -> Self {
        let mut doc = Self::new(name);
        doc.insert(0, content).unwrap();
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let workspace = TestWorkspace::new();
        let file_path = workspace.create_file("test.txt", "Hello");
        assert!(file_path.exists());
        assert_eq!(std::fs::read_to_string(file_path).unwrap(), "Hello");
    }

    #[test]
    fn test_document_creation() {
        let doc = editor_core::Document::create_test("test.txt", "Hello");
        assert_eq!(doc.text(), "Hello");
    }
}
