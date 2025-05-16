//! Theme configuration for syntax highlighting

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Creates a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a color from a hex string (e.g., "#FF0000")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(Self::new(r, g, b))
    }
}

/// Text style for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// Foreground color
    pub foreground: Option<Color>,
    /// Background color
    pub background: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
}

impl Style {
    /// Creates a new style
    pub fn new() -> Self {
        Self {
            foreground: None,
            background: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }

    /// Sets the foreground color
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets the background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets bold style
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Sets italic style
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Sets underline style
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Syntax highlighting theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Whether this is a dark theme
    pub dark: bool,
    /// Default text style
    pub default_style: Style,
    /// Styles for different syntax elements
    styles: HashMap<String, Style>,
}

impl Theme {
    /// Creates a new theme
    pub fn new(name: impl Into<String>, dark: bool) -> Self {
        Self {
            name: name.into(),
            dark,
            default_style: Style::default(),
            styles: HashMap::new(),
        }
    }

    /// Sets a style for a syntax element
    pub fn set_style(&mut self, element: impl Into<String>, style: Style) {
        self.styles.insert(element.into(), style);
    }

    /// Gets the style for a syntax element
    pub fn get_style(&self, element: &str) -> Option<&Style> {
        self.styles.get(element)
    }

    /// Creates a default dark theme
    pub fn dark() -> Self {
        let mut theme = Self::new("Dark", true);
        
        // Configure default styles
        theme.set_style("keyword", Style::new()
            .with_foreground(Color::from_hex("#C678DD").unwrap())
            .with_bold(true));
        
        theme.set_style("type", Style::new()
            .with_foreground(Color::from_hex("#E5C07B").unwrap()));
        
        theme.set_style("function", Style::new()
            .with_foreground(Color::from_hex("#61AFEF").unwrap()));
        
        theme.set_style("variable", Style::new()
            .with_foreground(Color::from_hex("#ABB2BF").unwrap()));
        
        theme.set_style("string", Style::new()
            .with_foreground(Color::from_hex("#98C379").unwrap()));
        
        theme.set_style("number", Style::new()
            .with_foreground(Color::from_hex("#D19A66").unwrap()));
        
        theme.set_style("comment", Style::new()
            .with_foreground(Color::from_hex("#5C6370").unwrap())
            .with_italic(true));
        
        theme.set_style("operator", Style::new()
            .with_foreground(Color::from_hex("#56B6C2").unwrap()));

        theme
    }

    /// Creates a default light theme
    pub fn light() -> Self {
        let mut theme = Self::new("Light", false);
        
        // Configure default styles
        theme.set_style("keyword", Style::new()
            .with_foreground(Color::from_hex("#A626A4").unwrap())
            .with_bold(true));
        
        theme.set_style("type", Style::new()
            .with_foreground(Color::from_hex("#C18401").unwrap()));
        
        theme.set_style("function", Style::new()
            .with_foreground(Color::from_hex("#4078F2").unwrap()));
        
        theme.set_style("variable", Style::new()
            .with_foreground(Color::from_hex("#383A42").unwrap()));
        
        theme.set_style("string", Style::new()
            .with_foreground(Color::from_hex("#50A14F").unwrap()));
        
        theme.set_style("number", Style::new()
            .with_foreground(Color::from_hex("#986801").unwrap()));
        
        theme.set_style("comment", Style::new()
            .with_foreground(Color::from_hex("#A0A1A7").unwrap())
            .with_italic(true));
        
        theme.set_style("operator", Style::new()
            .with_foreground(Color::from_hex("#0184BC").unwrap()));

        theme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .with_foreground(Color::new(255, 0, 0))
            .with_bold(true)
            .with_italic(true);

        assert_eq!(style.foreground.unwrap().r, 255);
        assert!(style.bold);
        assert!(style.italic);
    }

    #[test]
    fn test_theme_styles() {
        let theme = Theme::dark();
        
        // Test keyword style
        let keyword_style = theme.get_style("keyword").unwrap();
        assert!(keyword_style.bold);
        
        // Test comment style
        let comment_style = theme.get_style("comment").unwrap();
        assert!(comment_style.italic);
    }
}
