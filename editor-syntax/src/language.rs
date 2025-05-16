//! Language configuration and management

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use tree_sitter::Language as TSLanguage;

use crate::Result;

lazy_static::lazy_static! {
    static ref LANGUAGES: Arc<RwLock<HashMap<String, Language>>> = Arc::new(RwLock::new(HashMap::new()));
}

/// Language configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Language name
    pub name: String,
    /// File extensions associated with this language
    pub extensions: Vec<String>,
    /// Comment tokens
    pub comments: Comments,
    /// Brackets configuration
    pub brackets: Brackets,
    /// Indentation rules
    pub indentation: IndentationRules,
}

/// Comment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comments {
    /// Line comment token
    pub line: Option<String>,
    /// Block comment start token
    pub block_start: Option<String>,
    /// Block comment end token
    pub block_end: Option<String>,
}

/// Bracket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brackets {
    /// Opening brackets and their corresponding closing brackets
    pub pairs: Vec<(char, char)>,
}

/// Indentation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndentationRules {
    /// Increase indent after these patterns
    pub increase_indent: Vec<String>,
    /// Decrease indent after these patterns
    pub decrease_indent: Vec<String>,
}

/// Represents a programming language
#[derive(Clone)]
pub struct Language {
    /// Language configuration
    config: LanguageConfig,
    /// Tree-sitter language
    ts_language: TSLanguage,
}

impl Language {
    /// Creates a new language
    pub fn new(config: LanguageConfig, ts_language: TSLanguage) -> Self {
        Self {
            config,
            ts_language,
        }
    }

    /// Returns the language configuration
    pub fn config(&self) -> &LanguageConfig {
        &self.config
    }

    /// Returns the tree-sitter language
    pub fn ts_language(&self) -> TSLanguage {
        // Return a clone since tree-sitter expects an owned TSLanguage
        self.ts_language.clone()
    }
}

/// Registers the default supported languages
pub fn register_default_languages() -> Result<()> {
    let mut languages = LANGUAGES.write();

    // Register Rust
    languages.insert(
        "rust".to_string(),
        Language::new(
            LanguageConfig {
                name: "Rust".to_string(),
                extensions: vec![".rs".to_string()],
                comments: Comments {
                    line: Some("//".to_string()),
                    block_start: Some("/*".to_string()),
                    block_end: Some("*/".to_string()),
                },
                brackets: Brackets {
                    pairs: vec![
                        ('(', ')'),
                        ('[', ']'),
                        ('{', '}'),
                        ('<', '>'),
                    ],
                },
                indentation: IndentationRules {
                    increase_indent: vec![
                        "{".to_string(),
                        "(".to_string(),
                        "[".to_string(),
                    ],
                    decrease_indent: vec![
                        "}".to_string(),
                        ")".to_string(),
                        "]".to_string(),
                    ],
                },
            },
            tree_sitter_rust::language(),
        ),
    );

    // Register Python
    languages.insert(
        "python".to_string(),
        Language::new(
            LanguageConfig {
                name: "Python".to_string(),
                extensions: vec![".py".to_string()],
                comments: Comments {
                    line: Some("#".to_string()),
                    block_start: Some("\"\"\"".to_string()),
                    block_end: Some("\"\"\"".to_string()),
                },
                brackets: Brackets {
                    pairs: vec![
                        ('(', ')'),
                        ('[', ']'),
                        ('{', '}'),
                    ],
                },
                indentation: IndentationRules {
                    increase_indent: vec![
                        ":".to_string(),
                    ],
                    decrease_indent: vec![],
                },
            },
            tree_sitter_python::language(),
        ),
    );

    // Register JavaScript
    languages.insert(
        "javascript".to_string(),
        Language::new(
            LanguageConfig {
                name: "JavaScript".to_string(),
                extensions: vec![".js".to_string(), ".jsx".to_string()],
                comments: Comments {
                    line: Some("//".to_string()),
                    block_start: Some("/*".to_string()),
                    block_end: Some("*/".to_string()),
                },
                brackets: Brackets {
                    pairs: vec![
                        ('(', ')'),
                        ('[', ']'),
                        ('{', '}'),
                    ],
                },
                indentation: IndentationRules {
                    increase_indent: vec![
                        "{".to_string(),
                        "(".to_string(),
                        "[".to_string(),
                    ],
                    decrease_indent: vec![
                        "}".to_string(),
                        ")".to_string(),
                        "]".to_string(),
                    ],
                },
            },
            tree_sitter_javascript::language(),
        ),
    );

    Ok(())
}

/// Gets a language by name
#[allow(dead_code)]
pub fn get_language(name: &str) -> Option<Language> {
    LANGUAGES.read().get(name).cloned()
}

/// Gets a language by file extension
#[allow(dead_code)]
pub fn get_language_by_extension(ext: &str) -> Option<Language> {
    LANGUAGES.read()
        .iter()
        .find(|(_, lang)| lang.config.extensions.contains(&ext.to_string()))
        .map(|(_, lang)| lang.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_registration() {
        register_default_languages().unwrap();
        
        assert!(get_language("rust").is_some());
        assert!(get_language("python").is_some());
        assert!(get_language("javascript").is_some());
    }

    #[test]
    fn test_language_by_extension() {
        register_default_languages().unwrap();
        
        let rust_lang = get_language_by_extension(".rs").unwrap();
        assert_eq!(rust_lang.config().name, "Rust");
        
        let py_lang = get_language_by_extension(".py").unwrap();
        assert_eq!(py_lang.config().name, "Python");
    }
}
