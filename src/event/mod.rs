//! Event system for editor components communication

mod dispatcher;
mod handler;
mod queue;

pub use dispatcher::EventDispatcher;
pub use handler::{EventHandler, HandlerId};
pub use queue::EventQueue;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/// Editor events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorEvent {
    /// Buffer events
    Buffer(BufferEvent),
    /// Document events
    Document(DocumentEvent),
    /// UI events
    Ui(UiEvent),
    /// Input events
    Input(InputEvent),
    /// Plugin events
    Plugin(PluginEvent),
}

/// Buffer-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferEvent {
    /// Text was inserted
    Insert {
        /// Position where text was inserted
        position: usize,
        /// Inserted text
        text: String,
    },
    /// Text was deleted
    Delete {
        /// Start position of deletion
        start: usize,
        /// End position of deletion
        end: usize,
        /// Deleted text
        text: String,
    },
    /// Buffer was modified
    Modified {
        /// Whether the buffer is now modified
        modified: bool,
    },
}

/// Document-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentEvent {
    /// Document was opened
    Open {
        /// Path to the document
        path: PathBuf,
    },
    /// Document was saved
    Save {
        /// Path where the document was saved
        path: PathBuf,
    },
    /// Document was closed
    Close {
        /// Path to the closed document
        path: PathBuf,
    },
    /// Language was changed
    LanguageChange {
        /// New language identifier
        language: String,
    },
}

/// UI-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiEvent {
    /// Theme was changed
    ThemeChange {
        /// New theme name
        theme: String,
    },
    /// Font was changed
    FontChange {
        /// New font name
        font: String,
        /// New font size
        size: f32,
    },
    /// Layout was changed
    LayoutChange {
        /// New layout configuration
        layout: String,
    },
    /// Status message was updated
    StatusMessage {
        /// Message text
        text: String,
        /// Message duration in seconds (None for persistent)
        duration: Option<f32>,
    },
}

/// Input-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    /// Key was pressed
    KeyPress {
        /// Key code
        key: crate::input::KeyCode,
        /// Key modifiers
        modifiers: Vec<crate::input::Modifier>,
    },
    /// Mouse button was pressed
    MousePress {
        /// Button number
        button: u8,
        /// X coordinate
        x: f32,
        /// Y coordinate
        y: f32,
    },
    /// Mouse was moved
    MouseMove {
        /// X coordinate
        x: f32,
        /// Y coordinate
        y: f32,
    },
    /// Mouse wheel was scrolled
    MouseScroll {
        /// Horizontal delta
        delta_x: f32,
        /// Vertical delta
        delta_y: f32,
    },
}

/// Plugin-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Plugin was loaded
    Load {
        /// Plugin name
        name: String,
    },
    /// Plugin was unloaded
    Unload {
        /// Plugin name
        name: String,
    },
    /// Plugin command was executed
    Command {
        /// Plugin name
        plugin: String,
        /// Command name
        command: String,
        /// Command result
        result: CommandResult,
    },
}

/// Result of a plugin command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
    /// Command succeeded
    Success,
    /// Command failed
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = EditorEvent::Buffer(BufferEvent::Insert {
            position: 0,
            text: "Hello".to_string(),
        });

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: EditorEvent = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            EditorEvent::Buffer(BufferEvent::Insert { position, text }) => {
                assert_eq!(position, 0);
                assert_eq!(text, "Hello");
            }
            _ => panic!("Wrong event type after deserialization"),
        }
    }
}
