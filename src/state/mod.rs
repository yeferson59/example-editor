//! State management system for the editor

mod config;
mod session;
mod preferences;

pub use config::{Config, EditorConfig};
pub use session::{Session, SessionState};
pub use preferences::{Preferences, Theme};

use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// Global editor state
pub struct EditorState {
    /// Current configuration
    config: Arc<RwLock<Config>>,
    /// Current session
    session: Arc<RwLock<Session>>,
    /// User preferences
    preferences: Arc<RwLock<Preferences>>,
}

impl EditorState {
    /// Creates a new editor state
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Arc::new(RwLock::new(Config::load()?)),
            session: Arc::new(RwLock::new(Session::new())),
            preferences: Arc::new(RwLock::new(Preferences::load()?)),
        })
    }

    /// Returns the current configuration
    pub fn config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }

    /// Returns the current session
    pub fn session(&self) -> Arc<RwLock<Session>> {
        self.session.clone()
    }

    /// Returns the user preferences
    pub fn preferences(&self) -> Arc<RwLock<Preferences>> {
        self.preferences.clone()
    }

    /// Saves the current state
    pub async fn save(&self) -> Result<()> {
        self.config.read().await.save()?;
        self.session.read().await.save()?;
        self.preferences.read().await.save()?;
        Ok(())
    }

    /// Resets the state to defaults
    pub async fn reset(&self) -> Result<()> {
        *self.config.write().await = Config::default();
        *self.session.write().await = Session::new();
        *self.preferences.write().await = Preferences::default();
        self.save().await
    }
}
