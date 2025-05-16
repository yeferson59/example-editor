//! Syntax highlighting implementation using tree-sitter

use std::collections::HashMap;
use tree_sitter::{Parser as TSParser, Query, QueryCursor};
use crate::{Language, Theme, Style, Result, SyntaxError};

/// Represents a highlighting event
#[derive(Debug, Clone)]
pub enum HighlightEvent {
    /// Source text with style information
    Source {
        start: usize,
        end: usize,
        style: Style,
    },
    /// A highlighting error occurred
    Error(String),
}

/// Syntax highlighter
pub struct Highlighter {
    /// Tree-sitter parser
    parser: TSParser,
    /// Current language
    language: Option<Language>,
    /// Current theme
    theme: Theme,
    /// Highlight queries by language
    queries: HashMap<String, Query>,
}

impl Highlighter {
    /// Creates a new highlighter
    pub fn new() -> Self {
        Self {
            parser: TSParser::new(),
            language: None,
            theme: Theme::default(),
            queries: HashMap::new(),
        }
    }

    /// Sets the current language
    pub fn set_language(&mut self, language: Language) -> Result<()> {
        // Use the owned TSLanguage that the parser requires
        let ts_lang = language.ts_language();
        self.parser.set_language(ts_lang)
            .map_err(|e| SyntaxError::ParserError(e.to_string()))?;
        
        // Load highlight query if not already loaded
        if !self.queries.contains_key(&language.config().name) {
            let query_source = self.get_highlight_query(&language)?;
            let query = Query::new(language.ts_language(), &query_source)
                .map_err(|e| SyntaxError::ParserError(e.to_string()))?;
            self.queries.insert(language.config().name.clone(), query);
        }

        self.language = Some(language);
        Ok(())
    }

    /// Sets the highlighting theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Highlights the given text
    pub fn highlight(&mut self, text: &str) -> Result<Vec<HighlightEvent>> {
        let language = self.language.as_ref()
            .ok_or_else(|| SyntaxError::HighlightError("No language set".to_string()))?;

        // Parse the text
        let tree = self.parser.parse(text, None)
            .ok_or_else(|| SyntaxError::ParserError("Failed to parse text".to_string()))?;

        let query = self.queries.get(&language.config().name)
            .ok_or_else(|| SyntaxError::HighlightError("No highlight query found".to_string()))?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(query, tree.root_node(), text.as_bytes());

        let mut events = Vec::new();
        for match_ in matches {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];
                
                if let Some(style) = self.theme.get_style(capture_name) {
                    events.push(HighlightEvent::Source {
                        start: node.start_byte(),
                        end: node.end_byte(),
                        style: style.clone(),
                    });
                }
            }
        }

        // Sort events by start position
        events.sort_by_key(|event| match event {
            HighlightEvent::Source { start, .. } => *start,
            HighlightEvent::Error(_) => 0,
        });

        Ok(events)
    }

    /// Returns the highlight query for a language
    fn get_highlight_query(&self, language: &Language) -> Result<String> {
        // In a real implementation, this would load language-specific queries
        // For now, return a basic query for demonstration
        match language.config().name.as_str() {
            "Rust" => Ok(r#"
                (identifier) @variable
                (type_identifier) @type
                (string_literal) @string
                (integer_literal) @number
                (line_comment) @comment
                (block_comment) @comment
                (attribute_item) @attribute
                (macro_invocation) @macro
                (keyword) @keyword
            "#.to_string()),
            "Python" => Ok(r#"
                (identifier) @variable
                (string) @string
                (integer) @number
                (comment) @comment
                (decorator) @attribute
                (keyword) @keyword
            "#.to_string()),
            "JavaScript" => Ok(r#"
                (identifier) @variable
                (string) @string
                (number) @number
                (comment) @comment
                (jsx_element) @jsx
                (keyword) @keyword
            "#.to_string()),
            _ => Err(SyntaxError::UnsupportedLanguage(
                language.config().name.clone()
            )),
        }
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language;

    #[test]
    fn test_highlighter_initialization() {
        language::register_default_languages().unwrap();
        let mut highlighter = Highlighter::new();
        
        let rust_lang = language::get_language("rust").unwrap();
        assert!(highlighter.set_language(rust_lang).is_ok());
    }

    #[test]
    fn test_rust_highlighting() {
        language::register_default_languages().unwrap();
        let mut highlighter = Highlighter::new();
        
        let rust_lang = language::get_language("rust").unwrap();
        highlighter.set_language(rust_lang).unwrap();

        let source = r#"
            fn main() {
                println!("Hello, world!");
            }
        "#;

        let events = highlighter.highlight(source).unwrap();
        assert!(!events.is_empty());
    }
}
