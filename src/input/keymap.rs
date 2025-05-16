//! Keymap implementation for managing keybindings

use std::collections::HashMap;
use super::{Keybinding, KeyEvent, KeyCode, KeyPattern, Modifier};

/// Manages keybindings and their associated commands
pub struct KeyMap {
    /// Registered keybindings
    bindings: HashMap<Keybinding, String>,
}

impl KeyMap {
    /// Creates a new keymap
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Creates a new keymap with default bindings
    pub fn with_defaults() -> Self {
        let mut keymap = Self::new();
        keymap.add_default_bindings();
        keymap
    }

    /// Adds default keybindings
    fn add_default_bindings(&mut self) {
        // File operations
        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("s"),
                vec![Modifier::Ctrl],
            )]),
            "save".to_string(),
        );

        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("o"),
                vec![Modifier::Ctrl],
            )]),
            "open".to_string(),
        );

        // Edit operations
        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("z"),
                vec![Modifier::Ctrl],
            )]),
            "undo".to_string(),
        );

        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("z"),
                vec![Modifier::Ctrl, Modifier::Shift],
            )]),
            "redo".to_string(),
        );

        // Search operations
        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("f"),
                vec![Modifier::Ctrl],
            )]),
            "find".to_string(),
        );

        // Navigation
        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("g"),
                vec![Modifier::Ctrl],
            )]),
            "goto_line".to_string(),
        );

        // UI operations
        self.add_binding(
            Keybinding::new(vec![KeyPattern::new(
                KeyCode::Char("p"),
                vec![Modifier::Ctrl],
            )]),
            "command_palette".to_string(),
        );
    }

    /// Adds a keybinding
    pub fn add_binding(&mut self, binding: Keybinding, command: String) {
        self.bindings.insert(binding, command);
    }

    /// Removes a keybinding
    pub fn remove_binding(&mut self, binding: &Keybinding) {
        self.bindings.remove(binding);
    }

    /// Gets the command for a key sequence
    pub fn get_command(&self, sequence: &[KeyEvent]) -> Option<String> {
        for (binding, command) in &self.bindings {
            if binding.matches(sequence) {
                return Some(command.clone());
            }
        }
        None
    }

    /// Checks if a sequence could potentially match any binding
    pub fn is_prefix(&self, sequence: &[KeyEvent]) -> bool {
        self.bindings.keys().any(|binding| binding.is_prefix(sequence))
    }

    /// Returns all registered bindings
    pub fn bindings(&self) -> HashMap<String, Keybinding> {
        self.bindings
            .iter()
            .map(|(binding, command)| (command.clone(), binding.clone()))
            .collect()
    }

    /// Returns a binding for a command
    pub fn get_binding(&self, command: &str) -> Option<Keybinding> {
        self.bindings
            .iter()
            .find(|(_, cmd)| cmd == &command)
            .map(|(binding, _)| binding.clone())
    }

    /// Clears all bindings
    pub fn clear(&mut self) {
        self.bindings.clear();
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::with_defaults()
    }
}
