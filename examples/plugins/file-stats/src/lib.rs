//! File statistics plugin for Rust Editor
//!
//! This plugin provides file and directory statistics with UI integration.

use editor_plugin::{Plugin, PluginMetadata, Result, Permission};
use async_trait::async_trait;
use serde_json::json;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

/// File statistics plugin
pub struct FileStatsPlugin {
    /// Plugin metadata
    metadata: PluginMetadata,
    /// Current file statistics
    stats: FileStats,
}

/// File statistics
#[derive(Debug, Default, serde::Serialize)]
struct FileStats {
    /// Total number of files
    file_count: usize,
    /// Total number of directories
    dir_count: usize,
    /// Total size in bytes
    total_size: u64,
    /// File type distribution
    file_types: std::collections::HashMap<String, usize>,
    /// Lines of code (for text files)
    lines_of_code: usize,
}

impl FileStatsPlugin {
    /// Creates a new file statistics plugin
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "file-stats".to_string(),
                version: "0.1.0".to_string(),
                description: "File and directory statistics plugin".to_string(),
            },
            stats: FileStats::default(),
        }
    }

    /// Analyzes a directory
    async fn analyze_directory(&mut self, path: &Path) -> Result<()> {
        let mut stats = FileStats::default();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                stats.file_count += 1;
                stats.total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);

                // Count file types
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_string();
                    *stats.file_types.entry(ext).or_insert(0) += 1;
                }

                // Count lines of code for text files
                if let Ok(content) = fs::read_to_string(path).await {
                    stats.lines_of_code += content.lines().count();
                }
            } else if path.is_dir() {
                stats.dir_count += 1;
            }
        }

        self.stats = stats;
        Ok(())
    }

    /// Formats the statistics as a human-readable string
    fn format_stats(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Files: {}\n", self.stats.file_count));
        output.push_str(&format!("Directories: {}\n", self.stats.dir_count));
        output.push_str(&format!("Total size: {} bytes\n", self.stats.total_size));
        output.push_str(&format!("Lines of code: {}\n", self.stats.lines_of_code));
        
        output.push_str("\nFile types:\n");
        for (ext, count) in &self.stats.file_types {
            output.push_str(&format!("  .{}: {}\n", ext, count));
        }

        output
    }
}

#[async_trait]
impl Plugin for FileStatsPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        log::info!("File Stats plugin initialized!");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        log::info!("File Stats plugin shutting down!");
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        match command {
            "analyze" => {
                if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
                    self.analyze_directory(Path::new(path)).await?;
                    Ok(json!({
                        "stats": self.stats,
                        "formatted": self.format_stats()
                    }))
                } else {
                    Ok(json!({
                        "error": "Path argument required"
                    }))
                }
            }
            "get_stats" => {
                Ok(json!({
                    "stats": self.stats,
                    "formatted": self.format_stats()
                }))
            }
            _ => Ok(json!({
                "error": format!("Unknown command: {}", command)
            }))
        }
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(FileStatsPlugin::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn test_file_analysis() {
        // Create a temporary directory with some test files
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create some test files
        File::create(base_path.join("test.rs"))
            .unwrap()
            .write_all(b"fn main() {\n    println!(\"Hello\");\n}\n")
            .unwrap();

        File::create(base_path.join("data.json"))
            .unwrap()
            .write_all(b"{\"test\": true}")
            .unwrap();

        std::fs::create_dir(base_path.join("src")).unwrap();
        File::create(base_path.join("src/lib.rs"))
            .unwrap()
            .write_all(b"pub fn test() {}\n")
            .unwrap();

        // Test the plugin
        let mut plugin = FileStatsPlugin::new();
        
        // Initialize plugin
        assert!(plugin.initialize().await.is_ok());

        // Analyze directory
        let result = plugin.execute(
            "analyze",
            json!({"path": base_path.to_str().unwrap()}),
        ).await.unwrap();

        // Verify results
        let stats = result.get("stats").unwrap();
        assert_eq!(stats.get("file_count").unwrap().as_u64().unwrap(), 3);
        assert_eq!(stats.get("dir_count").unwrap().as_u64().unwrap(), 1);

        // Shutdown plugin
        assert!(plugin.shutdown().await.is_ok());
    }
}
