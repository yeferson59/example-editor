//! Testing utilities for plugin development

mod harness;
mod mock;

pub use harness::TestHarness;
pub use mock::{MockPlugin, MockEventHandler};

use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::{Plugin, PluginEvent, Result};

/// Test context for plugin testing
pub struct TestContext {
    /// Temporary directory for test files
    pub temp_dir: tempfile::TempDir,
    /// Event receiver
    pub event_rx: mpsc::Receiver<PluginEvent>,
}

impl TestContext {
    /// Creates a new test context
    pub fn new() -> Result<Self> {
        let temp_dir = tempfile::TempDir::new()?;
        let (_, event_rx) = mpsc::channel(100);

        Ok(Self {
            temp_dir,
            event_rx,
        })
    }

    /// Returns the path to the temporary directory
    pub fn temp_path(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }

    /// Creates a test file with content
    pub async fn create_test_file(&self, name: &str, content: &str) -> Result<PathBuf> {
        let path = self.temp_dir.path().join(name);
        tokio::fs::write(&path, content).await?;
        Ok(path)
    }

    /// Waits for a specific event
    pub async fn wait_for_event(&mut self) -> Option<PluginEvent> {
        self.event_rx.recv().await
    }
}

/// Test utilities for plugins
pub trait PluginTestExt {
    /// Sets up the plugin for testing
    async fn setup_test(&mut self) -> Result<()>;
    /// Tears down the plugin after testing
    async fn teardown_test(&mut self) -> Result<()>;
}

impl<T: Plugin> PluginTestExt for T {
    async fn setup_test(&mut self) -> Result<()> {
        self.initialize().await
    }

    async fn teardown_test(&mut self) -> Result<()> {
        self.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginMetadata;

    #[tokio::test]
    async fn test_context_creation() {
        let context = TestContext::new().unwrap();
        assert!(context.temp_dir.path().exists());
    }

    #[tokio::test]
    async fn test_file_creation() {
        let context = TestContext::new().unwrap();
        let file_path = context.create_test_file("test.txt", "Hello, World!")
            .await
            .unwrap();
        
        assert!(file_path.exists());
        let content = tokio::fs::read_to_string(file_path).await.unwrap();
        assert_eq!(content, "Hello, World!");
    }
}
