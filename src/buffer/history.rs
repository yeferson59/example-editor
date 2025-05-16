//! History management for undo/redo functionality

use super::operations::TextOperation;
use std::collections::VecDeque;

/// Maximum number of history entries
const MAX_HISTORY_SIZE: usize = 1000;

/// History entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The operation performed
    pub operation: TextOperation,
    /// The timestamp of the operation
    pub timestamp: std::time::SystemTime,
    /// Group ID for combining operations
    pub group_id: Option<u64>,
}

/// Undo/redo history
#[derive(Debug)]
pub struct History {
    /// Undo stack
    undo_stack: VecDeque<HistoryEntry>,
    /// Redo stack
    redo_stack: VecDeque<HistoryEntry>,
    /// Current group ID
    current_group: Option<u64>,
    /// Next group ID
    next_group_id: u64,
}

impl History {
    /// Creates a new history
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            current_group: None,
            next_group_id: 1,
        }
    }

    /// Pushes an operation onto the history
    pub fn push(&mut self, operation: TextOperation) {
        let entry = HistoryEntry {
            operation,
            timestamp: std::time::SystemTime::now(),
            group_id: self.current_group,
        };

        self.undo_stack.push_back(entry);
        self.redo_stack.clear();

        // Limit history size
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }

    /// Starts a new operation group
    pub fn start_group(&mut self) {
        self.current_group = Some(self.next_group_id);
        self.next_group_id += 1;
    }

    /// Ends the current operation group
    pub fn end_group(&mut self) {
        self.current_group = None;
    }

    /// Undoes an operation
    pub fn undo(&mut self) -> Option<TextOperation> {
        if let Some(entry) = self.undo_stack.pop_back() {
            // If this operation is part of a group, undo all operations in the group
            let group_id = entry.group_id;
            let mut operations = vec![entry.operation.clone()];
            
            // Keep undoing operations in the same group
            while let Some(last) = self.undo_stack.back() {
                if last.group_id != group_id {
                    break;
                }
                if let Some(entry) = self.undo_stack.pop_back() {
                    operations.push(entry.operation.clone());
                }
            }

            // Push operations to redo stack
            for operation in operations.iter().rev() {
                self.redo_stack.push_back(HistoryEntry {
                    operation: operation.clone(),
                    timestamp: std::time::SystemTime::now(),
                    group_id,
                });
            }

            // Return combined operation
            Some(TextOperation::combine(&operations))
        } else {
            None
        }
    }

    /// Redoes an operation
    pub fn redo(&mut self) -> Option<TextOperation> {
        if let Some(entry) = self.redo_stack.pop_back() {
            // If this operation is part of a group, redo all operations in the group
            let group_id = entry.group_id;
            let mut operations = vec![entry.operation.clone()];
            
            // Keep redoing operations in the same group
            while let Some(last) = self.redo_stack.back() {
                if last.group_id != group_id {
                    break;
                }
                if let Some(entry) = self.redo_stack.pop_back() {
                    operations.push(entry.operation.clone());
                }
            }

            // Push operations to undo stack
            for operation in operations.iter().rev() {
                self.undo_stack.push_back(HistoryEntry {
                    operation: operation.clone(),
                    timestamp: std::time::SystemTime::now(),
                    group_id,
                });
            }

            // Return combined operation
            Some(TextOperation::combine(&operations))
        } else {
            None
        }
    }

    /// Returns true if there are operations to undo
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if there are operations to redo
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears the history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_group = None;
    }

    /// Returns the number of operations in the undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the number of operations in the redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_history() {
        let mut history = History::new();
        
        // Test pushing operations
        history.push(TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        });
        
        history.push(TextOperation::Insert {
            position: 5,
            text: ", World!".to_string(),
        });

        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 0);

        // Test undo
        let undo_op = history.undo().unwrap();
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 1);

        // Test redo
        let redo_op = history.redo().unwrap();
        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_operation_groups() {
        let mut history = History::new();
        
        // Start a group
        history.start_group();
        
        // Push multiple operations in the same group
        history.push(TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        });
        
        history.push(TextOperation::Insert {
            position: 5,
            text: ", ".to_string(),
        });
        
        history.push(TextOperation::Insert {
            position: 7,
            text: "World!".to_string(),
        });
        
        history.end_group();

        // All operations should be undone together
        let undo_op = history.undo().unwrap();
        assert_eq!(history.undo_count(), 0);
        
        // Redo should restore all operations
        let redo_op = history.redo().unwrap();
        assert_eq!(history.undo_count(), 3);
    }

    #[test]
    fn test_history_limit() {
        let mut history = History::new();
        
        // Push more than MAX_HISTORY_SIZE operations
        for i in 0..MAX_HISTORY_SIZE + 10 {
            history.push(TextOperation::Insert {
                position: i,
                text: "x".to_string(),
            });
        }

        assert_eq!(history.undo_count(), MAX_HISTORY_SIZE);
    }
}
