//! Input handling module

mod keybinding;
mod commands;
mod keymap;

pub use keybinding::{Keybinding, KeyPattern, Modifier};
pub use commands::{Command, CommandRegistry};
pub use keymap::KeyMap;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents a key event
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    /// The key code
    pub code: KeyCode,
    /// Key modifiers (Ctrl, Alt, Shift)
    pub modifiers: Vec<Modifier>,
}

/// Represents a key code
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Function(u8),
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Backspace,
    Enter,
    Esc,
}

/// Input handler for the editor
pub struct InputHandler {
    /// Keybinding registry
    keymap: KeyMap,
    /// Command registry
    commands: CommandRegistry,
    /// Current key sequence
    sequence: Vec<KeyEvent>,
}

impl InputHandler {
    /// Creates a new input handler
    pub fn new() -> Self {
        Self {
            keymap: KeyMap::new(),
            commands: CommandRegistry::new(),
            sequence: Vec::new(),
        }
    }

    /// Registers a keybinding
    pub fn register_keybinding(&mut self, binding: Keybinding, command: String) {
        self.keymap.add_binding(binding, command);
    }

    /// Registers a command
    pub fn register_command<F>(&mut self, name: &str, description: &str, handler: F)
    where
        F: Fn() -> anyhow::Result<()> + Send + Sync + "static,
    {
        self.commands.register(name, description, handler);
    }

    /// Handles a key event
    pub fn handle_key(&mut self, event: KeyEvent) -> Option<Command> {
        self.sequence.push(event.clone());

        // Check if the current sequence matches any keybinding
        if let Some(command) = self.keymap.get_command(&self.sequence) {
            self.sequence.clear();
            self.commands.get(&command)
        } else {
            // Check if the sequence could potentially match a binding
            if !self.keymap.is_prefix(&self.sequence) {
                self.sequence.clear();
            }
            None
        }
    }

    /// Returns the current key sequence
    pub fn current_sequence(&self) -> &[KeyEvent] {
        &self.sequence
    }

    /// Clears the current key sequence
    pub fn clear_sequence(&mut self) {
        self.sequence.clear();
    }

    /// Returns all registered keybindings
    pub fn keybindings(&self) -> HashMap<String, Keybinding> {
        self.keymap.bindings()
    }

    /// Returns all registered commands
    pub fn commands(&self) -> HashMap<String, String> {
        self.commands.list()
    }

    /// Creates a new input handler with default bindings and commands
    pub fn with_defaults() -> Self {
        let mut handler = Self::new();
        handler.register_default_commands();
        handler
    }

    /// Registers default commands
    fn register_default_commands(&mut self) {
        // File operations
        self.register_command("save", "Save current file", || {
            // TODO: Implement save functionality
            Ok(())
        });

        self.register_command("open", "Open file", || {
            // TODO: Implement open functionality
            Ok(())
        });

        // Edit operations
        self.register_command("undo", "Undo last action", || {
            // TODO: Implement undo functionality
            Ok(())
        });

        self.register_command("redo", "Redo last action", || {
            // TODO: Implement redo functionality
            Ok(())
        });

        // Search operations
        self.register_command("find", "Find text", || {
            // TODO: Implement find functionality
            Ok(())
        });

        // UI operations
        self.register_command("command_palette", "Open command palette", || {
            // TODO: Implement command palette functionality
            Ok(())
        });
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_handling() {
        let mut handler = InputHandler::new();
        
        // Register a test command
        handler.register_command("test", "Test command", || Ok(()));
        
        // Register a keybinding
        let binding = Keybinding::new(vec![
            KeyPattern::new(KeyCode::Char("t"), vec![Modifier::Ctrl]),
        ]);
        handler.register_keybinding(binding, "test".to_string());

        // Test key event handling
        let event = KeyEvent {
            code: KeyCode::Char("t"),
            modifiers: vec![Modifier::Ctrl],
        };
        
        let command = handler.handle_key(event);
        assert!(command.is_some());
        
        // Test sequence clearing
        assert!(handler.current_sequence().is_empty());
    }

    #[test]
    fn test_default_commands() {
        let handler = InputHandler::with_defaults();
        let commands = handler.commands();
        
        // Check for some default commands
        assert!(commands.contains_key("save"));
        assert!(commands.contains_key("open"));
        assert!(commands.contains_key("undo"));
        assert!(commands.contains_key("redo"));
    }
}
