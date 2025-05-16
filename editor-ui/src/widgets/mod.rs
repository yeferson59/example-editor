//! UI widgets module

mod command_palette;
mod file_explorer;
mod minimap;
mod status_line;
mod tab_bar;

pub use command_palette::CommandPalette;
pub use file_explorer::FileExplorer;
pub use minimap::Minimap;
pub use status_line::StatusLine;
pub use tab_bar::TabBar;

use crate::Widget;
use eframe::egui;

/// Widget container trait
pub trait Container: Widget {
    /// Adds a widget to the container
    fn add<W: Widget>(&mut self, widget: W);
    /// Removes a widget
    fn remove(&mut self, id: &str);
    /// Gets a widget by id
    fn get(&self, id: &str) -> Option<&dyn Widget>;
    /// Gets a mutable widget by id
    fn get_mut(&mut self, id: &str) -> Option<&mut dyn Widget>;
}

/// Base widget state
#[derive(Default)]
pub struct WidgetState {
    /// Widget is visible
    pub visible: bool,
    /// Widget is enabled
    pub enabled: bool,
    /// Widget position
    pub position: (f32, f32),
    /// Widget size
    pub size: (f32, f32),
}

/// Base widget implementation
pub struct BaseWidget {
    /// Widget ID
    id: String,
    /// Widget state
    state: WidgetState,
}

impl BaseWidget {
    /// Creates a new base widget
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            state: WidgetState::default(),
        }
    }

    /// Returns the widget ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the widget state
    pub fn state(&self) -> &WidgetState {
        &self.state
    }

    /// Returns mutable widget state
    pub fn state_mut(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

impl Widget for BaseWidget {
    fn show(&mut self, _ui: &mut egui::Ui) {
        // Base widget has no visual representation
    }

    fn update(&mut self) {
        // Base widget has no state to update
    }

    fn handle_input(&mut self, _event: &editor_core::input::KeyEvent) {
        // Base widget does not handle input
    }
}
