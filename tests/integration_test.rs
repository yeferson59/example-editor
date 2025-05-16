//! Integration tests for Rust Editor

use editor_core::{Buffer, Document, Event};
use editor_plugin::{Plugin, PluginManager};
use editor_syntax::Highlighter;
use editor_lsp::LspClient;
use tokio;

mod common;

#[tokio::test]
async fn test_editor_initialization() {
    // Create test environment
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file_path = temp_dir.path().join("test.rs");
    std::fs::write(&test_file_path, "fn main() {\n    println!(\"Hello, World!\");\n}\n").unwrap();

    // Initialize components
    let mut buffer = Buffer::new();
    buffer.load_file(&test_file_path).await.unwrap();

    let mut document = Document::new("test.rs");
    document.set_buffer(buffer);

    let mut highlighter = Highlighter::new();
    highlighter.set_language("rust").unwrap();

    // Verify initial state
    assert_eq!(document.name(), "test.rs");
    assert!(!document.is_dirty());

    // Test syntax highlighting
    let highlights = highlighter.highlight(document.text()).unwrap();
    assert!(!highlights.is_empty());
}

#[tokio::test]
async fn test_plugin_integration() {
    // Initialize plugin manager
    let plugin_manager = PluginManager::new();
    
    // Load test plugin
    let plugin_path = std::path::Path::new("examples/plugins/hello-world");
    plugin_manager.load_plugin(plugin_path).await.unwrap();

    // Execute plugin command
    let result = plugin_manager.execute_command(
        "hello-world",
        "greet",
        serde_json::json!({"name": "Test"}),
    ).await.unwrap();

    assert_eq!(
        result.get("message").unwrap().as_str().unwrap(),
        "Hello, Test!"
    );
}

#[tokio::test]
async fn test_lsp_integration() {
    // Initialize LSP client
    let mut lsp_client = LspClient::new().await.unwrap();

    // Test file content
    let content = r#"
        fn main() {
            println!("Hello, World!");
        }
    "#;

    // Create test file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    std::fs::write(&test_file, content).unwrap();

    // Initialize LSP for Rust
    lsp_client.initialize_for_file(&test_file).await.unwrap();

    // Get completions
    let completions = lsp_client.get_completions(&test_file, (1, 0)).await.unwrap();
    assert!(!completions.is_empty());
}

#[tokio::test]
async fn test_document_changes() {
    let mut document = Document::new("test.txt");
    
    // Test insertions
    document.insert(0, "Hello").unwrap();
    assert_eq!(document.text(), "Hello");
    
    document.insert(5, ", World!").unwrap();
    assert_eq!(document.text(), "Hello, World!");
    
    // Test deletions
    document.delete(5, 7).unwrap();
    assert_eq!(document.text(), "HelloWorld!");
    
    // Test undo/redo
    document.undo().unwrap();
    assert_eq!(document.text(), "Hello, World!");
    
    document.redo().unwrap();
    assert_eq!(document.text(), "HelloWorld!");
}

#[tokio::test]
async fn test_syntax_highlighting_with_changes() {
    let mut highlighter = Highlighter::new();
    highlighter.set_language("rust").unwrap();

    let content = r#"
        fn test() {
            let x = 42;
            println!("{}", x);
        }
    "#;

    // Initial highlighting
    let highlights = highlighter.highlight(content).unwrap();
    let initial_count = highlights.len();

    // Modified content
    let modified = r#"
        fn test() {
            let x = 42;
            let y = "Hello";
            println!("{}, {}", x, y);
        }
    "#;

    // Highlight modified content
    let new_highlights = highlighter.highlight(modified).unwrap();
    assert!(new_highlights.len() > initial_count);
}

#[tokio::test]
async fn test_multiple_buffers() {
    let mut buffer1 = Buffer::new();
    buffer1.insert(0, "Buffer 1 content").unwrap();

    let mut buffer2 = Buffer::new();
    buffer2.insert(0, "Buffer 2 content").unwrap();

    let mut doc1 = Document::new("doc1.txt");
    doc1.set_buffer(buffer1);

    let mut doc2 = Document::new("doc2.txt");
    doc2.set_buffer(buffer2);

    assert_eq!(doc1.text(), "Buffer 1 content");
    assert_eq!(doc2.text(), "Buffer 2 content");
}

#[tokio::test]
async fn test_file_operations() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Create and save document
    let mut document = Document::new("test.txt");
    document.insert(0, "Hello, World!").unwrap();
    document.save_as(&test_file).await.unwrap();

    // Read back the file
    let content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "Hello, World!");

    // Modify and save
    document.insert(5, " beautiful").unwrap();
    document.save().await.unwrap();

    // Verify changes
    let updated_content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(updated_content, "Hello beautiful, World!");
}

#[tokio::test]
async fn test_event_handling() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    // Create event handler
    let handler = common::TestEventHandler::new(tx);
    
    // Create document with event handler
    let mut document = Document::with_event_handler(
        "test.txt",
        Box::new(handler),
    );

    // Modify document
    document.insert(0, "Hello").unwrap();

    // Verify event was received
    let event = rx.recv().await.unwrap();
    match event {
        Event::TextChanged { .. } => (),
        _ => panic!("Unexpected event type"),
    }
}
