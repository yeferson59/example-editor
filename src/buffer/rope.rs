//! Rope data structure for efficient text manipulation

use std::ops::{Range, Index};
use std::cmp::{min, max};
use std::fmt;

const CHUNK_SIZE: usize = 1024;

/// A node in the rope tree
#[derive(Clone)]
enum Node {
    /// Leaf node containing text
    Leaf {
        /// Text content
        text: String,
        /// Length in bytes
        len: usize,
        /// Number of lines
        lines: usize,
    },
    /// Internal node
    Internal {
        /// Left child
        left: Box<Node>,
        /// Right child
        right: Box<Node>,
        /// Total length in bytes
        len: usize,
        /// Total number of lines
        lines: usize,
    },
}

/// Rope data structure for text storage
#[derive(Clone)]
pub struct Buffer {
    /// Root node of the rope tree
    root: Node,
}

impl Node {
    /// Creates a new leaf node
    fn leaf(text: String) -> Self {
        let len = text.len();
        let lines = text.chars().filter(|&c| c == '\n').count();
        Node::Leaf { text, len, lines }
    }

    /// Creates a new internal node
    fn internal(left: Node, right: Node) -> Self {
        let len = left.len() + right.len();
        let lines = left.lines() + right.lines();
        Node::Internal {
            left: Box::new(left),
            right: Box::new(right),
            len,
            lines,
        }
    }

    /// Returns the length in bytes
    fn len(&self) -> usize {
        match self {
            Node::Leaf { len, .. } => *len,
            Node::Internal { len, .. } => *len,
        }
    }

    /// Returns the number of lines
    fn lines(&self) -> usize {
        match self {
            Node::Leaf { lines, .. } => *lines,
            Node::Internal { lines, .. } => *lines,
        }
    }

    /// Splits the node at the given offset
    fn split(&self, offset: usize) -> (Node, Node) {
        match self {
            Node::Leaf { text, .. } => {
                let (left, right) = text.split_at(offset);
                (Node::leaf(left.to_string()), Node::leaf(right.to_string()))
            }
            Node::Internal { left, right, .. } => {
                let left_len = left.len();
                if offset <= left_len {
                    let (left_left, left_right) = left.split(offset);
                    (left_left, Node::internal(left_right, (**right).clone()))
                } else {
                    let (right_left, right_right) = right.split(offset - left_len);
                    (Node::internal((**left).clone(), right_left), right_right)
                }
            }
        }
    }

    /// Concatenates two nodes
    fn concat(left: Node, right: Node) -> Node {
        match (left, right) {
            (Node::Leaf { text: left_text, .. }, Node::Leaf { text: right_text, .. }) => {
                if left_text.len() + right_text.len() <= CHUNK_SIZE {
                    let mut text = left_text;
                    text.push_str(&right_text);
                    Node::leaf(text)
                } else {
                    Node::internal(Node::leaf(left_text), Node::leaf(right_text))
                }
            }
            (left, right) => Node::internal(left, right),
        }
    }
}

impl Buffer {
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Self {
            root: Node::leaf(String::new()),
        }
    }

    /// Creates a buffer from existing text
    pub fn from_text(text: &str) -> Self {
        Self {
            root: Node::leaf(text.to_string()),
        }
    }

    /// Returns the length in bytes
    pub fn len(&self) -> usize {
        self.root.len()
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of lines
    pub fn lines(&self) -> usize {
        self.root.lines()
    }

    /// Returns the text content
    pub fn text(&self) -> String {
        self.slice(0..self.len())
    }

    /// Returns a slice of the text
    pub fn slice(&self, range: Range<usize>) -> String {
        let mut result = String::new();
        self.slice_into(&self.root, range, &mut result);
        result
    }

    /// Slices text into the given string
    fn slice_into(&self, node: &Node, range: Range<usize>, result: &mut String) {
        match node {
            Node::Leaf { text, .. } => {
                let start = min(range.start, text.len());
                let end = min(range.end, text.len());
                result.push_str(&text[start..end]);
            }
            Node::Internal { left, right, .. } => {
                let left_len = left.len();
                if range.start < left_len {
                    self.slice_into(left, range.start..min(range.end, left_len), result);
                }
                if range.end > left_len {
                    self.slice_into(right, max(range.start, left_len) - left_len..range.end - left_len, result);
                }
            }
        }
    }

    /// Inserts text at the specified position
    pub fn insert(&mut self, position: usize, text: &str) {
        if position > self.len() {
            return;
        }

        if text.is_empty() {
            return;
        }

        let (left, right) = self.root.split(position);
        let middle = Node::leaf(text.to_string());
        self.root = Node::concat(Node::concat(left, middle), right);
    }

    /// Deletes text in the specified range
    pub fn delete(&mut self, range: Range<usize>) {
        if range.start >= self.len() || range.start >= range.end {
            return;
        }

        let (left, temp) = self.root.split(range.start);
        let (_, right) = temp.split(range.end - range.start);
        self.root = Node::concat(left, right);
    }

    /// Returns an iterator over the lines
    pub fn lines_iter(&self) -> LinesIterator {
        LinesIterator {
            text: self.text(),
            position: 0,
        }
    }
}

/// Iterator over lines in the buffer
pub struct LinesIterator {
    text: String,
    position: usize,
}

impl Iterator for LinesIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.text.len() {
            return None;
        }

        let start = self.position;
        while self.position < self.text.len() && self.text.as_bytes()[self.position] != b'\n' {
            self.position += 1;
        }

        let line = self.text[start..self.position].to_string();
        if self.position < self.text.len() {
            self.position += 1; // Skip newline
        }

        Some(line)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer {{ len: {}, lines: {} }}", self.len(), self.lines())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_operations() {
        let mut buffer = Buffer::new();
        assert!(buffer.is_empty());

        // Test insert
        buffer.insert(0, "Hello");
        assert_eq!(buffer.text(), "Hello");
        assert_eq!(buffer.len(), 5);

        buffer.insert(5, ", World!");
        assert_eq!(buffer.text(), "Hello, World!");
        assert_eq!(buffer.len(), 12);

        // Test delete
        buffer.delete(5..7);
        assert_eq!(buffer.text(), "HelloWorld!");

        // Test slice
        assert_eq!(buffer.slice(0..5), "Hello");
        assert_eq!(buffer.slice(5..10), "World");
    }

    #[test]
    fn test_line_operations() {
        let mut buffer = Buffer::new();
        buffer.insert(0, "Line 1\nLine 2\nLine 3");

        assert_eq!(buffer.lines(), 2); // Number of newlines
        
        let lines: Vec<String> = buffer.lines_iter().collect();
        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_large_operations() {
        let mut buffer = Buffer::new();
        let large_text = "x".repeat(CHUNK_SIZE * 2);
        
        buffer.insert(0, &large_text);
        assert_eq!(buffer.len(), CHUNK_SIZE * 2);

        buffer.delete(CHUNK_SIZE/2..CHUNK_SIZE*3/2);
        assert_eq!(buffer.len(), CHUNK_SIZE);
    }
}
