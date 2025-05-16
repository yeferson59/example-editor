//! Text operations module
//!
//! Provides text manipulation operations and transformations

use std::ops::Range;
use serde::{Serialize, Deserialize};
use crate::Result;

/// Represents a position in the text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
}

/// Represents a range in the text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRange {
    /// Start position
    pub start: Position,
    /// End position
    pub end: Position,
}

/// Represents a text operation that can be applied to a buffer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextOperation {
    /// Insert text at a position
    Insert {
        /// Position to insert at
        position: Position,
        /// Text to insert
        text: String,
    },
    /// Delete text in a range
    Delete {
        /// Range to delete
        range: TextRange,
    },
    /// Replace text in a range
    Replace {
        /// Range to replace
        range: TextRange,
        /// New text
        text: String,
    },
}

/// Text utility functions
pub struct TextUtil;

impl TextUtil {
    /// Converts a line-column position to a byte offset
    pub fn position_to_offset(text: &str, position: Position) -> Option<usize> {
        let mut current_line = 0;
        let mut current_column = 0;
        let mut byte_offset = 0;

        for (idx, c) in text.char_indices() {
            if current_line == position.line && current_column == position.column {
                return Some(byte_offset);
            }

            if c == '\n' {
                current_line += 1;
                current_column = 0;
            } else {
                current_column += 1;
            }

            byte_offset = idx + c.len_utf8();
        }

        // Handle position at end of text
        if current_line == position.line && current_column == position.column {
            Some(byte_offset)
        } else {
            None
        }
    }

    /// Converts a byte offset to a line-column position
    pub fn offset_to_position(text: &str, offset: usize) -> Option<Position> {
        if offset > text.len() {
            return None;
        }

        let mut line = 0;
        let mut column = 0;
        let mut current_offset = 0;

        for c in text.chars() {
            if current_offset == offset {
                return Some(Position { line, column });
            }

            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }

            current_offset += c.len_utf8();
        }

        // Handle position at end of text
        if current_offset == offset {
            Some(Position { line, column })
        } else {
            None
        }
    }

    /// Gets the line at the specified index
    pub fn get_line(text: &str, line_index: usize) -> Option<&str> {
        text.lines().nth(line_index)
    }

    /// Gets the line range containing the given byte range
    pub fn get_line_range(text: &str, range: Range<usize>) -> Range<usize> {
        let start_pos = Self::offset_to_position(text, range.start)
            .map(|pos| pos.line)
            .unwrap_or(0);
        let end_pos = Self::offset_to_position(text, range.end)
            .map(|pos| pos.line + 1)
            .unwrap_or_else(|| text.lines().count());
        start_pos..end_pos
    }

    /// Counts the number of lines in the text
    pub fn line_count(text: &str) -> usize {
        text.lines().count()
    }

    /// Returns the length of the specified line
    pub fn line_length(text: &str, line_index: usize) -> Option<usize> {
        Self::get_line(text, line_index).map(|line| line.chars().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_conversions() {
        let text = "Hello\nWorld\nRust";
        
        // Test position to offset
        let pos = Position { line: 1, column: 2 };
        let offset = TextUtil::position_to_offset(text, pos).unwrap();
        assert_eq!(offset, 8); // "Hello\nWo" -> 8 bytes
        
        // Test offset to position
        let pos2 = TextUtil::offset_to_position(text, offset).unwrap();
        assert_eq!(pos, pos2);
    }

    #[test]
    fn test_line_operations() {
        let text = "Hello\nWorld\nRust";
        
        // Test get_line
        assert_eq!(TextUtil::get_line(text, 1), Some("World"));
        
        // Test line_count
        assert_eq!(TextUtil::line_count(text), 3);
        
        // Test line_length
        assert_eq!(TextUtil::line_length(text, 1), Some(5)); // "World"
    }

    #[test]
    fn test_line_range() {
        let text = "Hello\nWorld\nRust";
        
        // Test range covering multiple lines
        let range = 4..8; // "o\nWo"
        let line_range = TextUtil::get_line_range(text, range);
        assert_eq!(line_range, 0..2);
    }
}
