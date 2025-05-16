//! User preferences

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use anyhow::Result;

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// Current theme
    pub theme: Theme,
    /// Key bindings
    pub keybindings: KeyBindings,
    /// Plugin settings
    pub plugins: PluginSettings,
    /// Editor behaviors
    pub editor: EditorBehavior,
    /// Auto-completion settings
    pub completion: CompletionSettings,
}

/// Theme settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Is dark theme
    pub is_dark: bool,
    /// Color scheme
    pub colors: ColorScheme,
    /// Token colors
    pub tokens: TokenColors,
}

/// Color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Background color
    pub background: String,
    /// Foreground color
    pub foreground: String,
    /// Selection color
    pub selection: String,
    /// Line number color
    pub line_numbers: String,
    /// Current line color
    pub current_line: String,
    /// UI element colors
    pub ui: UiColors,
}

/// UI colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiColors {
    /// Status bar background
    pub status_bar_background: String,
    /// Status bar foreground
    pub status_bar_foreground: String,
    /// Tab active background
    pub tab_active_background: String,
    /// Tab inactive background
    pub tab_inactive_background: String,
    /// Panel background
    pub panel_background: String,
    /// Button background
    pub button_background: String,
    /// Button foreground
    pub button_foreground: String,
}

/// Token colors for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenColors {
    /// Keywords
    pub keyword: String,
    /// Strings
    pub string: String,
    /// Numbers
    pub number: String,
    /// Comments
    pub comment: String,
    /// Functions
    pub function: String,
    /// Types
    pub type_name: String,
    /// Variables
    pub variable: String,
    /// Constants
    pub constant: String,
    /// Operators
    pub operator: String,
    /// Parameters
    pub parameter: String,
}

/// Key bindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    /// Custom key bindings
    pub custom: std::collections::HashMap<String, String>,
    /// Disabled default bindings
    pub disabled: Vec<String>,
    /// Multi-key timeout (ms)
    pub timeout: u32,
}

/// Plugin settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettings {
    /// Enabled plugins
    pub enabled: Vec<String>,
    /// Plugin-specific settings
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    /// Auto-update plugins
    pub auto_update: bool,
    /// Plugin installation directory
    pub install_dir: Option<PathBuf>,
}

/// Editor behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorBehavior {
    /// Auto-save enabled
    pub auto_save: bool,
    /// Auto-save interval (seconds)
    pub auto_save_interval: u32,
    /// Format on save
    pub format_on_save: bool,
    /// Trim trailing whitespace on save
    pub trim_whitespace: bool,
    /// Ensure final newline
    pub ensure_final_newline: bool,
    /// Word wrap mode
    pub word_wrap: WrapMode,
    /// Scroll past end
    pub scroll_past_end: bool,
    /// Minimap enabled
    pub show_minimap: bool,
    /// Smart indent
    pub smart_indent: bool,
}

/// Word wrap mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WrapMode {
    /// No wrap
    None,
    /// Wrap at view width
    View,
    /// Wrap at specific column
    Column(u32),
}

/// Auto-completion settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSettings {
    /// Enable auto-completion
    pub enabled: bool,
    /// Trigger on characters
    pub trigger_characters: Vec<char>,
    /// Minimum word length
    pub min_word_length: u32,
    /// Suggestion delay (ms)
    pub delay: u32,
    /// Show parameter hints
    pub parameter_hints: bool,
    /// Show documentation
    pub show_documentation: bool,
}

impl Preferences {
    /// Loads preferences from disk
    pub fn load() -> Result<Self> {
        let prefs_path = Self::preferences_path()?;
        if prefs_path.exists() {
            let contents = std::fs::read_to_string(prefs_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    /// Saves preferences to disk
    pub fn save(&self) -> Result<()> {
        let prefs_path = Self::preferences_path()?;
        if let Some(parent) = prefs_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(prefs_path, contents)?;
        Ok(())
    }

    /// Returns the preferences file path
    fn preferences_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("rust-editor");
        path.push("preferences.toml");
        Ok(path)
    }

    /// Returns the default dark theme
    pub fn dark_theme() -> Theme {
        Theme {
            name: "Dark+".to_string(),
            is_dark: true,
            colors: ColorScheme {
                background: "#1e1e1e".to_string(),
                foreground: "#d4d4d4".to_string(),
                selection: "#264f78".to_string(),
                line_numbers: "#858585".to_string(),
                current_line: "#282828".to_string(),
                ui: UiColors {
                    status_bar_background: "#007acc".to_string(),
                    status_bar_foreground: "#ffffff".to_string(),
                    tab_active_background: "#1e1e1e".to_string(),
                    tab_inactive_background: "#2d2d2d".to_string(),
                    panel_background: "#252526".to_string(),
                    button_background: "#0e639c".to_string(),
                    button_foreground: "#ffffff".to_string(),
                },
            },
            tokens: TokenColors {
                keyword: "#569cd6".to_string(),
                string: "#ce9178".to_string(),
                number: "#b5cea8".to_string(),
                comment: "#6a9955".to_string(),
                function: "#dcdcaa".to_string(),
                type_name: "#4ec9b0".to_string(),
                variable: "#9cdcfe".to_string(),
                constant: "#4fc1ff".to_string(),
                operator: "#d4d4d4".to_string(),
                parameter: "#9cdcfe".to_string(),
            },
        }
    }

    /// Returns the default light theme
    pub fn light_theme() -> Theme {
        Theme {
            name: "Light+".to_string(),
            is_dark: false,
            colors: ColorScheme {
                background: "#ffffff".to_string(),
                foreground: "#000000".to_string(),
                selection: "#add6ff".to_string(),
                line_numbers: "#237893".to_string(),
                current_line: "#f3f3f3".to_string(),
                ui: UiColors {
                    status_bar_background: "#007acc".to_string(),
                    status_bar_foreground: "#ffffff".to_string(),
                    tab_active_background: "#ffffff".to_string(),
                    tab_inactive_background: "#ececec".to_string(),
                    panel_background: "#f3f3f3".to_string(),
                    button_background: "#007acc".to_string(),
                    button_foreground: "#ffffff".to_string(),
                },
            },
            tokens: TokenColors {
                keyword: "#0000ff".to_string(),
                string: "#a31515".to_string(),
                number: "#098658".to_string(),
                comment: "#008000".to_string(),
                function: "#795e26".to_string(),
                type_name: "#267f99".to_string(),
                variable: "#001080".to_string(),
                constant: "#0070c1".to_string(),
                operator: "#000000".to_string(),
                parameter: "#001080".to_string(),
            },
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: Self::dark_theme(),
            keybindings: KeyBindings {
                custom: std::collections::HashMap::new(),
                disabled: Vec::new(),
                timeout: 1000,
            },
            plugins: PluginSettings {
                enabled: Vec::new(),
                settings: std::collections::HashMap::new(),
                auto_update: true,
                install_dir: None,
            },
            editor: EditorBehavior {
                auto_save: true,
                auto_save_interval: 300,
                format_on_save: true,
                trim_whitespace: true,
                ensure_final_newline: true,
                word_wrap: WrapMode::View,
                scroll_past_end: true,
                show_minimap: true,
                smart_indent: true,
            },
            completion: CompletionSettings {
                enabled: true,
                trigger_characters: vec!['.', ':', '>', '(', '"', '\''],
                min_word_length: 2,
                delay: 100,
                parameter_hints: true,
                show_documentation: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_serialization() {
        let prefs = Preferences::default();
        let serialized = toml::to_string_pretty(&prefs).unwrap();
        let deserialized: Preferences = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.theme.name, prefs.theme.name);
        assert_eq!(deserialized.theme.colors.background, prefs.theme.colors.background);
    }

    #[test]
    fn test_theme_switching() {
        let mut prefs = Preferences::default();
        assert!(prefs.theme.is_dark);

        prefs.theme = Preferences::light_theme();
        assert!(!prefs.theme.is_dark);
    }
}
