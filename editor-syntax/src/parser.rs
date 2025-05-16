//! Code parsing functionality using tree-sitter

use tree_sitter::{Parser as TSParser, Tree, Node, TreeCursor};
use crate::{Language, Result, SyntaxError};

/// Code parser using tree-sitter
pub struct Parser {
    /// Tree-sitter parser
    parser: TSParser,
    /// Current language
    language: Option<Language>,
    /// Current syntax tree
    tree: Option<Tree>,
}

impl Parser {
    /// Creates a new parser
    pub fn new() -> Self {
        Self {
            parser: TSParser::new(),
            language: None,
            tree: None,
        }
    }

    /// Sets the current language
    pub fn set_language(&mut self, language: Language) -> Result<()> {
        self.parser.set_language(language.ts_language())
            .map_err(|e| SyntaxError::ParserError(e.to_string()))?;
        self.language = Some(language);
        Ok(())
    }

    /// Parses the given text
    pub fn parse(&mut self, text: &str, old_tree: Option<&Tree>) -> Result<Tree> {
        if self.language.is_none() {
            return Err(SyntaxError::ParserError("No language set".to_string()));
        }

        let tree = self.parser.parse(text, old_tree)
            .ok_or_else(|| SyntaxError::ParserError("Failed to parse text".to_string()))?;
        
        self.tree = Some(tree.clone());
        Ok(tree)
    }

    /// Returns a syntax error for the given node, if any
    pub fn get_error(&self, node: &Node) -> Option<String> {
        if node.is_error() {
            Some(format!("Syntax error at byte offset {}", node.start_byte()))
        } else if node.is_missing() {
            Some(format!("Missing syntax at byte offset {}", node.start_byte()))
        } else {
            None
        }
    }

    /// Returns an iterator over the syntax errors in the tree
    pub fn iter_errors<'a>(&self, tree: &'a Tree) -> impl Iterator<Item = Node<'a>> {
        ErrorIterator::new(tree)
    }

    /// Returns the current syntax tree
    pub fn tree(&self) -> Option<&Tree> {
        self.tree.as_ref()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over syntax errors in a tree
struct ErrorIterator<'a> {
    cursor: TreeCursor<'a>,
}

impl<'a> ErrorIterator<'a> {
    fn new(tree: &'a Tree) -> Self {
        Self {
            cursor: tree.walk(),
        }
    }
}

impl<'a> Iterator for ErrorIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.cursor.node();
            
            if node.is_error() || node.is_missing() {
                // Go to next sibling or parent\'s next sibling
                while !self.cursor.goto_next_sibling() {
                    if !self.cursor.goto_parent() {
                        return Some(node);
                    }
                }
                return Some(node);
            }

            // Try to go to first child
            if !self.cursor.goto_first_child() {
                // No children, try next sibling
                while !self.cursor.goto_next_sibling() {
                    if !self.cursor.goto_parent() {
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language;

    #[test]
    fn test_parser_initialization() {
        language::register_default_languages().unwrap();
        let mut parser = Parser::new();
        
        let rust_lang = language::get_language("rust").unwrap();
        assert!(parser.set_language(rust_lang).is_ok());
    }

    #[test]
    fn test_rust_parsing() {
        language::register_default_languages().unwrap();
        let mut parser = Parser::new();
        
        let rust_lang = language::get_language("rust").unwrap();
        parser.set_language(rust_lang).unwrap();

        let source = r#"
            fn main() {
                println!("Hello, world!");
            }
        "#;

        let tree = parser.parse(source, None).unwrap();
        assert!(!tree.root_node().is_error());
    }

    #[test]
    fn test_error_detection() {
        language::register_default_languages().unwrap();
        let mut parser = Parser::new();
        
        let rust_lang = language::get_language("rust").unwrap();
        parser.set_language(rust_lang).unwrap();

        // Invalid Rust code
        let source = r#"
            fn main() {
                println!("Hello, world!"
            }
        "#;

        let tree = parser.parse(source, None).unwrap();
        let errors: Vec<_> = parser.iter_errors(&tree).collect();
        assert!(!errors.is_empty());
    }
}
