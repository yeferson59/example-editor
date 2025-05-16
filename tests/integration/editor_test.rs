//! Integration tests for the editor

use rust_editor::{Editor, Config};
use editor_core::{Buffer, Document};
use editor_syntax::Highlighter;
use editor_lsp::LspClient;
use editor_plugin::PluginManager;
use tokio;

/// Test fixture
struct TestFixture {
    /// Editor instance
    editor: Editor,
    /// Temporary directory
    temp_dir: tempfile::TempDir,
}

impl TestFixture {
    /// Creates a new test fixture
    async fn new() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = Config::default();
        let editor = Editor::new(config).await.unwrap();
        
        Self {
            editor,
            temp_dir,
        }
    }

    /// Creates a test file
    fn create_test_file(&self, name: &str, content: &str) -> std::path::PathBuf {
        let path = self.temp_dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}

#[tokio::test]
async fn test_file_operations() {
    let fixture = TestFixture::new().await;
    
    // Create test file
    let file_path = fixture.create_test_file(
        "test.rs",
        "fn main() {\n    println!(\"Hello, World!\");\n}\n"
    );

    // Open file
    fixture.editor.open_file(&file_path).await.unwrap();
    
    // Verify content
    let doc = fixture.editor.current_document().await.unwrap();
    let text = doc.text().await;
    assert!(text.contains("Hello, World!"));

    // Make changes
    doc.insert(19, "Rust").await.unwrap();
    assert!(doc.text().await.contains("Hello, Rust!"));

    // Save changes
    fixture.editor.save_file().await.unwrap();
    
    // Verify file content
    let saved_content = std::fs::read_to_string(&file_path).unwrap();
    assert!(saved_content.contains("Hello, Rust!"));
}

#[tokio::test]
async fn test_syntax_highlighting() {
    let fixture = TestFixture::new().await;
    
    // Create Rust file
    let file_path = fixture.create_test_file(
        "syntax.rs",
        r#"
        fn test_function() -> Result<(), Error> {
            let x = 42;
            let s = "Hello";
            println!("{} {}", s, x);
            Ok(())
        }
        "#
    );

    // Open file
    fixture.editor.open_file(&file_path).await.unwrap();
    
    // Get syntax highlighting
    let doc = fixture.editor.current_document().await.unwrap();
    let highlights = doc.get_highlights().await.unwrap();
    
    // Verify highlighting
    assert!(highlights.iter().any(|h| h.style == "keyword")); // fn, let
    assert!(highlights.iter().any(|h| h.style == "number")); // 42
    assert!(highlights.iter().any(|h| h.style == "string")); // "Hello"
}

#[tokio::test]
async fn test_lsp_integration() {
    let fixture = TestFixture::new().await;
    
    // Create Rust file with error
    let file_path = fixture.create_test_file(
        "lsp.rs",
        r#"
        fn main() {
            let x: i32 = "not a number";
        }
        "#
    );

    // Open file
    fixture.editor.open_file(&file_path).await.unwrap();
    
    // Wait for LSP diagnostics
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    // Verify diagnostics
    let doc = fixture.editor.current_document().await.unwrap();
    let diagnostics = doc.get_diagnostics().await.unwrap();
    
    assert!(!diagnostics.is_empty());
    assert!(diagnostics[0].message.contains("mismatched types"));
}

#[tokio::test]
async fn test_plugin_system() {
    let fixture = TestFixture::new().await;
    
    // Load test plugin
    let plugin_path = fixture.temp_dir.path().join("plugins").join("test-plugin");
    std::fs::create_dir_all(&plugin_path).unwrap();
    
    // Create plugin manifest
    let manifest = r#"
    {
        "name": "test-plugin",
        "version": "0.1.0",
        "description": "Test plugin",
        "entry_point": "lib",
        "plugin_type": "Native",
        "permissions": []
    }
    "#;
    std::fs::write(plugin_path.join("plugin.json"), manifest).unwrap();

    // Load plugin
    fixture.editor.load_plugin(&plugin_path).await.unwrap();
    
    // Verify plugin loaded
    let plugins = fixture.editor.plugin_manager().plugins().await;
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].name(), "test-plugin");
}

#[tokio::test]
async fn test_multiple_documents() {
    let fixture = TestFixture::new().await;
    
    // Create test files
    let file1 = fixture.create_test_file("doc1.txt", "Document 1");
    let file2 = fixture.create_test_file("doc2.txt", "Document 2");

    // Open files
    fixture.editor.open_file(&file1).await.unwrap();
    fixture.editor.open_file(&file2).await.unwrap();
    
    // Verify document management
    assert_eq!(fixture.editor.document_count().await, 2);
    
    // Switch between documents
    fixture.editor.switch_to_document(0).await.unwrap();
    assert!(fixture.editor.current_document().await.unwrap().text().await.contains("Document 1"));
    
    fixture.editor.switch_to_document(1).await.unwrap();
    assert!(fixture.editor.current_document().await.unwrap().text().await.contains("Document 2"));
}

#[tokio::test]
async fn test_undo_redo() {
    let fixture = TestFixture::new().await;
    
    // Create test file
    let file_path = fixture.create_test_file("undo.txt", "Initial text");
    
    // Open file
    fixture.editor.open_file(&file_path).await.unwrap();
    let doc = fixture.editor.current_document().await.unwrap();
    
    // Make changes
    doc.insert(0, "Hello ").await.unwrap();
    assert_eq!(doc.text().await, "Hello Initial text");
    
    // Undo
    doc.undo().await.unwrap();
    assert_eq!(doc.text().await, "Initial text");
    
    // Redo
    doc.redo().await.unwrap();
    assert_eq!(doc.text().await, "Hello Initial text");
}
