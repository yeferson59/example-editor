//! Editor configuration

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use anyhow::Result;

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Editor settings
    pub editor: EditorConfig,
    /// UI settings
    pub ui: UiConfig,
    /// Language settings
    pub language: LanguageConfig,
}

/// Editor-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab size in spaces
    pub tab_size: u8,
    /// Use spaces for indentation
    pub use_spaces: bool,
    /// Line ending style
    pub line_ending: LineEnding,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Wrap long lines
    pub wrap_lines: bool,
    /// Auto-save interval in seconds (0 to disable)
    pub auto_save_interval: u32,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Show minimap
    pub show_minimap: bool,
    /// Show status bar
    pub show_status_bar: bool,
    /// Show file tree
    pub show_file_tree: bool,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: f32,
}

/// Language-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Default language settings
    pub default: LanguageSettings,
    /// Language-specific overrides
    pub overrides: std::collections::HashMap<String, LanguageSettings>,
}

/// Settings for a specific language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSettings {
    /// Tab size override
    pub tab_size: Option<u8>,
    /// Use spaces override
    pub use_spaces: Option<bool>,
    /// Formatter command
    pub formatter: Option<String>,
    /// LSP server command
    pub lsp_server: Option<String>,
}

/// Line ending style
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineEnding {
    /// Unix style (
)
    Unix,
    /// Windows style (
)
    Windows,
    /// Mac style ()
    Mac,
}

impl Config {
    /// Loads configuration from disk
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    /// Saves configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    /// Returns the configuration file path
    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("rust-editor");
        path.push("config.toml");
        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig {
                tab_size: 4,
                use_spaces: true,
                line_ending: LineEnding::Unix,
                show_line_numbers: true,
                wrap_lines: false,
                auto_save_interval: 300,
            },
            ui: UiConfig {
                show_minimap: true,
                show_status_bar: true,
                show_file_tree: true,
                font_family: "JetBrains Mono".to_string(),
                font_size: 14.0,
            },
            language: LanguageConfig {
                default: LanguageSettings {
                    tab_size: None,
                    use_spaces: None,
                    formatter: None,
                    lsp_server: None,
                },
                overrides: std::collections::HashMap::new(),
            },
        }
    }
}
