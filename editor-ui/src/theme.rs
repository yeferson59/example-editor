//! Theme settings for the editor UI

use eframe::egui::{self, Visuals};
use serde::{Serialize, Deserialize};

/// Internal theme type used for system theme detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeType {
    Light,
    Dark,
}

/// Theme for the editor UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Use system theme
    System,
}

/// UI colors for theming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
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

/// UI element colors
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

/// Syntax highlighting colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxColors {
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

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

impl Theme {
    /// Apply the theme to the egui context
    pub fn apply(&self, ctx: &egui::Context) {
        let theme_type = match self {
            Theme::Light => ThemeType::Light,
            Theme::Dark => ThemeType::Dark,
            Theme::System => self.detect_system_theme(),
        };
        
        // Apply egui visuals
        match theme_type {
            ThemeType::Light => ctx.set_visuals(Visuals::light()),
            ThemeType::Dark => ctx.set_visuals(Visuals::dark()),
        }
    }
    
    /// Get colors for the current theme
    pub fn get_colors(&self) -> Colors {
        match self {
            Theme::Light => Self::light_colors(),
            Theme::Dark => Self::dark_colors(),
            Theme::System => {
                match self.detect_system_theme() {
                    ThemeType::Light => Self::light_colors(),
                    ThemeType::Dark => Self::dark_colors(),
                }
            }
        }
    }
    
    /// Get syntax colors for the current theme
    pub fn get_syntax_colors(&self) -> SyntaxColors {
        match self {
            Theme::Light => Self::light_syntax(),
            Theme::Dark => Self::dark_syntax(),
            Theme::System => {
                match self.detect_system_theme() {
                    ThemeType::Light => Self::light_syntax(),
                    ThemeType::Dark => Self::dark_syntax(),
                }
            }
        }
    }
    
    /// Detect the system theme
    fn detect_system_theme(&self) -> ThemeType {
        #[cfg(target_os = "macos")]
        {
            // Use dark-light crate to detect macOS dark mode
            match dark_light::detect() {
                dark_light::Mode::Dark => return ThemeType::Dark,
                dark_light::Mode::Light => return ThemeType::Light,
                _ => return ThemeType::Light,
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // Default to light theme on unsupported platforms
            ThemeType::Light
        }
    }
    
    /// Get light theme colors
    fn light_colors() -> Colors {
        Colors {
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
        }
    }
    
    /// Get dark theme colors
    fn dark_colors() -> Colors {
        Colors {
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
        }
    }
    
    /// Get light theme syntax colors
    fn light_syntax() -> SyntaxColors {
        SyntaxColors {
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
        }
    }
    
    /// Get dark theme syntax colors
    fn dark_syntax() -> SyntaxColors {
        SyntaxColors {
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
        }
    }
}
