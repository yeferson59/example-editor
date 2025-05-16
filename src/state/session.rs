//! Editor session state

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use anyhow::Result;

/// Editor session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Currently open files
    pub open_files: Vec<PathBuf>,
    /// Currently active file
    pub active_file: Option<PathBuf>,
    /// Window state
    pub window: WindowState,
    /// Cursor positions for open files
    pub cursor_positions: std::collections::HashMap<PathBuf, CursorPosition>,
    /// Scroll positions for open files
    pub scroll_positions: std::collections::HashMap<PathBuf, ScrollPosition>,
    /// Current workspace
    pub workspace: Option<WorkspaceState>,
}

/// Window state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window position
    pub position: (i32, i32),
    /// Window size
    pub size: (u32, u32),
    /// Is window maximized
    pub maximized: bool,
    /// Active panels
    pub panels: PanelState,
}

/// Panel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    /// File explorer visible
    pub explorer_visible: bool,
    /// Explorer width
    pub explorer_width: u32,
    /// Search panel visible
    pub search_visible: bool,
    /// Search panel height
    pub search_height: u32,
}

/// Cursor position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
    /// Selection start (if any)
    pub selection_start: Option<(usize, usize)>,
    /// Selection end (if any)
    pub selection_end: Option<(usize, usize)>,
}

/// Scroll position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollPosition {
    /// Vertical scroll offset
    pub vertical: f32,
    /// Horizontal scroll offset
    pub horizontal: f32,
    /// First visible line
    pub first_visible_line: usize,
}

/// Workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    /// Workspace root path
    pub root: PathBuf,
    /// Git branch (if applicable)
    pub git_branch: Option<String>,
    /// Open folders in workspace
    pub open_folders: Vec<PathBuf>,
    /// Excluded patterns
    pub excluded_patterns: Vec<String>,
}

impl Session {
    /// Creates a new session
    pub fn new() -> Self {
        Self {
            open_files: Vec::new(),
            active_file: None,
            window: WindowState::default(),
            cursor_positions: std::collections::HashMap::new(),
            scroll_positions: std::collections::HashMap::new(),
            workspace: None,
        }
    }

    /// Saves session state to disk
    pub fn save(&self) -> Result<()> {
        let session_path = Self::session_path()?;
        if let Some(parent) = session_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(session_path, contents)?;
        Ok(())
    }

    /// Loads session state from disk
    pub fn load() -> Result<Self> {
        let session_path = Self::session_path()?;
        if session_path.exists() {
            let contents = std::fs::read_to_string(session_path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::new())
        }
    }

    /// Returns the session file path
    fn session_path() -> Result<PathBuf> {
        let mut path = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
        path.push("rust-editor");
        path.push("session.json");
        Ok(path)
    }

    /// Adds a file to the session
    pub fn add_file(&mut self, path: PathBuf) {
        if !self.open_files.contains(&path) {
            self.open_files.push(path.clone());
        }
        self.active_file = Some(path);
    }

    /// Removes a file from the session
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.open_files.retain(|p| p != path);
        self.cursor_positions.remove(path);
        self.scroll_positions.remove(path);
        if Some(path) == self.active_file.as_ref() {
            self.active_file = self.open_files.last().cloned();
        }
    }

    /// Sets the workspace
    pub fn set_workspace(&mut self, root: PathBuf) {
        self.workspace = Some(WorkspaceState {
            root,
            git_branch: None,
            open_folders: Vec::new(),
            excluded_patterns: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
            ],
        });
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            position: (100, 100),
            size: (1200, 800),
            maximized: false,
            panels: PanelState {
                explorer_visible: true,
                explorer_width: 250,
                search_visible: false,
                search_height: 200,
            },
        }
    }
}
