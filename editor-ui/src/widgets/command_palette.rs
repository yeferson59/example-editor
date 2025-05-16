//! Command palette widget

use crate::Widget;
use eframe::egui;
use std::collections::HashMap;

/// Command palette entry
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command shortcut
    pub shortcut: Option<String>,
    /// Command handler
    pub handler: Box<dyn Fn() + Send + Sync>,
}

/// Command palette widget
pub struct CommandPalette {
    /// Available commands
    commands: HashMap<String, CommandEntry>,
    /// Search query
    query: String,
    /// Selected index
    selected: usize,
    /// Visible state
    visible: bool,
}

impl CommandPalette {
    /// Creates a new command palette
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            query: String::new(),
            selected: 0,
            visible: false,
        }
    }

    /// Registers a command
    pub fn register_command<F>(&mut self, name: &str, description: &str, shortcut: Option<&str>, handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.commands.insert(
            name.to_string(),
            CommandEntry {
                name: name.to_string(),
                description: description.to_string(),
                shortcut: shortcut.map(String::from),
                handler: Box::new(handler),
            },
        );
    }

    /// Shows or hides the command palette
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        if !visible {
            self.query.clear();
            self.selected = 0;
        }
    }

    /// Toggles visibility
    pub fn toggle(&mut self) {
        self.set_visible(!self.visible);
    }

    /// Filters commands based on query
    fn filtered_commands(&self) -> Vec<&CommandEntry> {
        let query = self.query.to_lowercase();
        let mut commands: Vec<_> = self.commands
            .values()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query)
            })
            .collect();
        
        commands.sort_by(|a, b| a.name.cmp(&b.name));
        commands
    }

    /// Executes the selected command
    fn execute_selected(&self) {
        if let Some(command) = self.filtered_commands().get(self.selected) {
            (command.handler)();
        }
    }
}

impl Widget for CommandPalette {
    fn show(&mut self, ui: &mut egui::Ui) {
        if !self.visible {
            return;
        }

        let frame = egui::Frame::popup(ui.style())
            .stroke(egui::Stroke::none())
            .rounding(4.0)
            .shadow(ui.style().visuals.popup_shadow);

        egui::Area::new("command_palette")
            .fixed_pos(ui.ctx().screen_rect().center())
            .show(ui.ctx(), |ui| {
                frame.show(ui, |ui| {
                    ui.set_min_width(400.0);
                    ui.set_max_width(600.0);

                    // Search input
                    let response = ui.text_edit_singleline(&mut self.query)
                        .on_hover_text("Type to search commands");

                    if response.lost_focus() {
                        if ui.input().key_pressed(egui::Key::Escape) {
                            self.set_visible(false);
                            return;
                        }
                        if ui.input().key_pressed(egui::Key::Enter) {
                            self.execute_selected();
                            self.set_visible(false);
                            return;
                        }
                    }

                    ui.separator();

                    // Command list
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            let filtered = self.filtered_commands();
                            self.selected = self.selected.min(filtered.len().saturating_sub(1));

                            for (idx, command) in filtered.iter().enumerate() {
                                let selected = idx == self.selected;

                                let response = ui.selectable_label(
                                    selected,
                                    format!(
                                        "{}\t{}\t{}",
                                        command.name,
                                        command.description,
                                        command.shortcut.as_deref().unwrap_or("")
                                    ),
                                );

                                if response.clicked() {
                                    self.selected = idx;
                                    self.execute_selected();
                                    self.set_visible(false);
                                    break;
                                }

                                if response.hovered() {
                                    self.selected = idx;
                                }
                            }
                        });
                });
            });
    }

    fn update(&mut self) {
        // Update command palette state
    }

    fn handle_input(&mut self, event: &editor_core::input::KeyEvent) {
        // Handle keyboard navigation
        use editor_core::input::KeyCode;
        match event.code {
            KeyCode::Enter => {
                self.execute_selected();
                self.set_visible(false);
            }
            KeyCode::Esc => {
                self.set_visible(false);
            }
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
            }
            KeyCode::Down => {
                let max = self.filtered_commands().len().saturating_sub(1);
                self.selected = (self.selected + 1).min(max);
            }
            _ => {}
        }
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}
