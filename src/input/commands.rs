//! Command system implementation

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use parking_lot::RwLock;

/// Represents a command that can be executed
pub struct Command {
    /// Command name
    name: String,
    /// Command description
    description: String,
    /// Command handler
    handler: Arc<dyn Fn() -> Result<()> + Send + Sync>,
}

impl Command {
    /// Creates a new command
    pub fn new<F>(name: impl Into<String>, description: impl Into<String>, handler: F) -> Self
    where
        F: Fn() -> Result<()> + Send + Sync + "static,
    {
        Self {
            name: name.into(),
            description: description.into(),
            handler: Arc::new(handler),
        }
    }

    /// Returns the command name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Executes the command
    pub fn execute(&self) -> Result<()> {
        (self.handler)()
    }
}

/// Registry of available commands
pub struct CommandRegistry {
    /// Registered commands
    commands: Arc<RwLock<HashMap<String, Command>>>,
}

impl CommandRegistry {
    /// Creates a new command registry
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a new command
    pub fn register<F>(&mut self, name: &str, description: &str, handler: F)
    where
        F: Fn() -> Result<()> + Send + Sync + "static,
    {
        let command = Command::new(name, description, handler);
        self.commands.write().insert(name.to_string(), command);
    }

    /// Returns a command by name
    pub fn get(&self, name: &str) -> Option<Command> {
        self.commands.read().get(name).cloned()
    }

    /// Returns all registered commands
    pub fn list(&self) -> HashMap<String, String> {
        self.commands
            .read()
            .iter()
            .map(|(name, cmd)| (name.clone(), cmd.description().to_string()))
            .collect()
    }

    /// Removes a command
    pub fn unregister(&mut self, name: &str) {
        self.commands.write().remove(name);
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
