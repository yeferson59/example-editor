//! Keybinding implementation

use serde::{Serialize, Deserialize};
use super::{KeyCode, KeyEvent};

/// Key modifier flags
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
}

/// Pattern for matching key events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyPattern {
    /// The key code to match
    pub code: KeyCode,
    /// Required modifiers
    pub modifiers: Vec<Modifier>,
}

impl KeyPattern {
    /// Creates a new key pattern
    pub fn new(code: KeyCode, modifiers: Vec<Modifier>) -> Self {
        Self { code, modifiers }
    }

    /// Checks if this pattern matches a key event
    pub fn matches(&self, event: &KeyEvent) -> bool {
        if self.code != event.code {
            return false;
        }

        // Check that all required modifiers are present
        for required in &self.modifiers {
            if !event.modifiers.contains(required) {
                return false;
            }
        }

        // Check that no extra modifiers are present
        for present in &event.modifiers {
            if !self.modifiers.contains(present) {
                return false;
            }
        }

        true
    }
}

/// Represents a complete keybinding
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Keybinding {
    /// Sequence of key patterns
    patterns: Vec<KeyPattern>,
}

impl Keybinding {
    /// Creates a new keybinding
    pub fn new(patterns: Vec<KeyPattern>) -> Self {
        Self { patterns }
    }

    /// Checks if this binding matches a sequence of key events
    pub fn matches(&self, events: &[KeyEvent]) -> bool {
        if events.len() != self.patterns.len() {
            return false;
        }

        self.patterns.iter().zip(events).all(|(pattern, event)| {
            pattern.matches(event)
        })
    }

    /// Checks if this binding could potentially match a sequence
    pub fn is_prefix(&self, events: &[KeyEvent]) -> bool {
        if events.len() > self.patterns.len() {
            return false;
        }

        self.patterns.iter().zip(events).all(|(pattern, event)| {
            pattern.matches(event)
        })
    }

    /// Returns the key patterns
    pub fn patterns(&self) -> &[KeyPattern] {
        &self.patterns
    }
}
