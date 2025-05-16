//! Text operations implementation

use serde::{Serialize, Deserialize};

/// A text operation that can be performed on a buffer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextOperation {
    /// Insert text at a position
    Insert {
        /// Position to insert at
        position: usize,
        /// Text to insert
        text: String,
    },
    /// Delete text in a range
    Delete {
        /// Start position
        start: usize,
        /// End position
        end: usize,
        /// Deleted text (for undo)
        text: String,
    },
    /// Replace text in a range
    Replace {
        /// Start position
        start: usize,
        /// End position
        end: usize,
        /// Old text (for undo)
        old_text: String,
        /// New text
        new_text: String,
    },
    /// Compound operation
    Compound {
        /// List of operations to perform
        operations: Vec<TextOperation>,
    },
}

/// Operation traits
pub trait Operation {
    /// Applies the operation
    fn apply(&self, buffer: &mut super::rope::Buffer) -> anyhow::Result<()>;
    /// Returns the inverse of this operation
    fn invert(&self) -> Self;
    /// Combines this operation with another if possible
    fn combine(&self, other: &Self) -> Option<Self> where Self: Sized;
}

impl TextOperation {
    /// Combines multiple operations into one
    pub fn combine(operations: &[TextOperation]) -> TextOperation {
        if operations.len() == 1 {
            operations[0].clone()
        } else {
            TextOperation::Compound {
                operations: operations.to_vec(),
            }
        }
    }

    /// Returns true if this operation can be combined with another
    pub fn can_combine(&self, other: &TextOperation) -> bool {
        match (self, other) {
            // Adjacent insertions at the same position
            (
                TextOperation::Insert { position: pos1, .. },
                TextOperation::Insert { position: pos2, .. }
            ) => pos1 + 1 == *pos2,

            // Adjacent deletions
            (
                TextOperation::Delete { start: s1, end: e1, .. },
                TextOperation::Delete { start: s2, end: e2, .. }
            ) => e1 == *s2 || s1 == *e2,

            // Consecutive replacements at the same position
            (
                TextOperation::Replace { start: s1, end: e1, .. },
                TextOperation::Replace { start: s2, end: e2, .. }
            ) => e1 == *s2,

            _ => false,
        }
    }
}

impl Operation for TextOperation {
    fn apply(&self, buffer: &mut super::rope::Buffer) -> anyhow::Result<()> {
        match self {
            TextOperation::Insert { position, text } => {
                buffer.insert(*position, text);
                Ok(())
            }
            TextOperation::Delete { start, end, .. } => {
                buffer.delete(*start..*end);
                Ok(())
            }
            TextOperation::Replace { start, end, new_text, .. } => {
                buffer.delete(*start..*end);
                buffer.insert(*start, new_text);
                Ok(())
            }
            TextOperation::Compound { operations } => {
                for op in operations {
                    op.apply(buffer)?;
                }
                Ok(())
            }
        }
    }

    fn invert(&self) -> Self {
        match self {
            TextOperation::Insert { position, text } => {
                TextOperation::Delete {
                    start: *position,
                    end: *position + text.len(),
                    text: text.clone(),
                }
            }
            TextOperation::Delete { start, end, text } => {
                TextOperation::Insert {
                    position: *start,
                    text: text.clone(),
                }
            }
            TextOperation::Replace { start, end, old_text, new_text } => {
                TextOperation::Replace {
                    start: *start,
                    end: *start + new_text.len(),
                    old_text: new_text.clone(),
                    new_text: old_text.clone(),
                }
            }
            TextOperation::Compound { operations } => {
                TextOperation::Compound {
                    operations: operations.iter()
                        .rev()
                        .map(|op| op.invert())
                        .collect(),
                }
            }
        }
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        if !self.can_combine(other) {
            return None;
        }

        match (self, other) {
            // Combine adjacent insertions
            (
                TextOperation::Insert { position: pos1, text: text1 },
                TextOperation::Insert { text: text2, .. }
            ) => {
                let mut combined_text = text1.clone();
                combined_text.push_str(text2);
                Some(TextOperation::Insert {
                    position: *pos1,
                    text: combined_text,
                })
            }

            // Combine adjacent deletions
            (
                TextOperation::Delete { start: s1, text: text1, .. },
                TextOperation::Delete { text: text2, .. }
            ) => {
                let mut combined_text = text1.clone();
                combined_text.push_str(text2);
                Some(TextOperation::Delete {
                    start: *s1,
                    end: *s1 + combined_text.len(),
                    text: combined_text,
                })
            }

            // Combine consecutive replacements
            (
                TextOperation::Replace { start: s1, old_text: old1, new_text: new1, .. },
                TextOperation::Replace { old_text: old2, new_text: new2, .. }
            ) => {
                let mut combined_old = old1.clone();
                combined_old.push_str(old2);
                let mut combined_new = new1.clone();
                combined_new.push_str(new2);
                Some(TextOperation::Replace {
                    start: *s1,
                    end: *s1 + combined_old.len(),
                    old_text: combined_old,
                    new_text: combined_new,
                })
            }

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::rope::Buffer;

    #[test]
    fn test_insert_operation() {
        let mut buffer = Buffer::new();
        
        let op = TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        };

        op.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "Hello");

        // Test inversion
        let inverse = op.invert();
        inverse.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "");
    }

    #[test]
    fn test_delete_operation() {
        let mut buffer = Buffer::from_text("Hello, World!");
        
        let op = TextOperation::Delete {
            start: 5,
            end: 7,
            text: ", ".to_string(),
        };

        op.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "HelloWorld!");

        // Test inversion
        let inverse = op.invert();
        inverse.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "Hello, World!");
    }

    #[test]
    fn test_replace_operation() {
        let mut buffer = Buffer::from_text("Hello, World!");
        
        let op = TextOperation::Replace {
            start: 0,
            end: 5,
            old_text: "Hello".to_string(),
            new_text: "Hi".to_string(),
        };

        op.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "Hi, World!");

        // Test inversion
        let inverse = op.invert();
        inverse.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "Hello, World!");
    }

    #[test]
    fn test_compound_operation() {
        let mut buffer = Buffer::new();
        
        let ops = TextOperation::Compound {
            operations: vec![
                TextOperation::Insert {
                    position: 0,
                    text: "Hello".to_string(),
                },
                TextOperation::Insert {
                    position: 5,
                    text: ", ".to_string(),
                },
                TextOperation::Insert {
                    position: 7,
                    text: "World!".to_string(),
                },
            ],
        };

        ops.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "Hello, World!");

        // Test inversion
        let inverse = ops.invert();
        inverse.apply(&mut buffer).unwrap();
        assert_eq!(buffer.text(), "");
    }

    #[test]
    fn test_operation_combining() {
        let op1 = TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        };

        let op2 = TextOperation::Insert {
            position: 5,
            text: ", World!".to_string(),
        };

        // Operations should not combine (not adjacent)
        assert!(op1.combine(&op2).is_none());

        let op3 = TextOperation::Insert {
            position: 1,
            text: "i".to_string(),
        };

        // Operations should combine (adjacent)
        let combined = op1.combine(&op3);
        assert!(combined.is_some());
        match combined.unwrap() {
            TextOperation::Insert { text, .. } => {
                assert_eq!(text, "Hiello");
            }
            _ => panic!("Wrong operation type"),
        }
    }
}
