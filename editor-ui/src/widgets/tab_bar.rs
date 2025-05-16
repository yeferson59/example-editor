//! Tab bar widget for managing multiple open files

use crate::Widget;
use eframe::egui;
use editor_core::Document;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Tab state
#[derive(Clone)]
struct Tab {
    /// Document reference
    document: Arc<RwLock<Document>>,
    /// Is modified
    is_modified: bool,
    /// Is active
    is_active: bool,
    /// Is pinned
    is_pinned: bool,
    /// Preview only (close after losing focus)
    is_preview: bool,
}

/// Tab bar widget
pub struct TabBar {
    /// Open tabs
    tabs: Vec<Tab>,
    /// Active tab index
    active_tab: Option<usize>,
    /// Drag state
    drag_state: Option<DragState>,
    /// Context menu state
    context_menu: Option<ContextMenuState>,
}

/// Tab drag state
struct DragState {
    /// Source tab index
    source: usize,
    /// Current position
    current_pos: egui::Pos2,
}

/// Context menu state
struct ContextMenuState {
    /// Tab index
    tab: usize,
    /// Menu position
    pos: egui::Pos2,
}

impl TabBar {
    /// Creates a new tab bar
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            drag_state: None,
            context_menu: None,
        }
    }

    /// Adds a new tab
    pub fn add_tab(&mut self, document: Arc<RwLock<Document>>) {
        let tab = Tab {
            document,
            is_modified: false,
            is_active: false,
            is_pinned: false,
            is_preview: false,
        };

        self.tabs.push(tab);
        if self.active_tab.is_none() {
            self.active_tab = Some(self.tabs.len() - 1);
        }
    }

    /// Closes a tab
    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if let Some(active) = self.active_tab {
                if active >= self.tabs.len() {
                    self.active_tab = if self.tabs.is_empty() {
                        None
                    } else {
                        Some(self.tabs.len() - 1)
                    };
                }
            }
        }
    }

    /// Returns the active document
    pub fn active_document(&self) -> Option<Arc<RwLock<Document>>> {
        self.active_tab
            .and_then(|idx| self.tabs.get(idx))
            .map(|tab| tab.document.clone())
    }

    /// Shows a single tab
    fn show_tab(&mut self, ui: &mut egui::Ui, idx: usize, tab: &mut Tab) {
        let mut style = ui.style_mut();
        if tab.is_active {
            style.visuals.widgets.active.bg_fill = ui.visuals().selection.bg_fill;
        }

        let mut label_text = {
            let doc = tab.document.blocking_read();
            doc.name().to_string()
        };

        if tab.is_modified {
            label_text.push('*');
        }

        if tab.is_pinned {
            label_text.insert(0, '📌');
        }

        let response = ui.add(egui::SelectableLabel::new(
            tab.is_active,
            label_text,
        ));

        // Handle interactions
        if response.clicked() {
            self.active_tab = Some(idx);
            for (i, t) in self.tabs.iter_mut().enumerate() {
                t.is_active = i == idx;
            }
        }

        if response.secondary_clicked() {
            self.context_menu = Some(ContextMenuState {
                tab: idx,
                pos: response.rect.center(),
            });
        }

        // Close button
        if !tab.is_pinned {
            ui.add_space(4.0);
            if ui.small_button("×").clicked() {
                self.close_tab(idx);
            }
        }

        // Drag and drop
        if response.dragged() {
            self.drag_state = Some(DragState {
                source: idx,
                current_pos: response.rect.center(),
            });
        }
    }

    /// Shows the context menu
    fn show_context_menu(&mut self, ui: &mut egui::Ui) {
        if let Some(ctx_menu) = &self.context_menu {
            let tab = &mut self.tabs[ctx_menu.tab];
            
            egui::Window::new("Tab Options")
                .fixed_pos(ctx_menu.pos)
                .show(ui.ctx(), |ui| {
                    if ui.button(if tab.is_pinned { "Unpin" } else { "Pin" }).clicked() {
                        tab.is_pinned = !tab.is_pinned;
                        self.context_menu = None;
                    }

                    if !tab.is_pinned {
                        if ui.button("Close").clicked() {
                            self.close_tab(ctx_menu.tab);
                            self.context_menu = None;
                        }

                        if ui.button("Close Others").clicked() {
                            let doc = tab.document.clone();
                            self.tabs.retain(|t| Arc::ptr_eq(&t.document, &doc) || t.is_pinned);
                            self.context_menu = None;
                        }

                        if ui.button("Close All").clicked() {
                            self.tabs.retain(|t| t.is_pinned);
                            self.context_menu = None;
                        }
                    }
                });
        }
    }

    /// Handles tab dragging
    fn handle_drag(&mut self, ui: &mut egui::Ui) {
        if let Some(drag_state) = &self.drag_state {
            // Draw drag indicator
            ui.painter().circle_filled(
                drag_state.current_pos,
                4.0,
                ui.visuals().selection.bg_fill,
            );

            // Find drop target
            for (idx, tab) in self.tabs.iter().enumerate() {
                if idx != drag_state.source {
                    // TODO: Implement drop logic
                }
            }
        }
    }
}

impl Widget for TabBar {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            // Show tabs
            for idx in 0..self.tabs.len() {
                let mut tab = self.tabs[idx].clone();
                self.show_tab(ui, idx, &mut tab);
                self.tabs[idx] = tab;
            }

            // Add button
            ui.add_space(8.0);
            if ui.button("+").clicked() {
                // TODO: Create new document
            }
        });

        // Show context menu
        self.show_context_menu(ui);

        // Handle drag and drop
        self.handle_drag(ui);

        // Update drag state
        if !ui.input().pointer.primary_down() {
            self.drag_state = None;
        }
    }

    fn update(&mut self) {
        // Update tab states
        for tab in &mut self.tabs {
            let doc = tab.document.blocking_read();
            tab.is_modified = doc.is_modified();
        }
    }

    fn handle_input(&mut self, event: &editor_core::input::KeyEvent) {
        use editor_core::input::KeyCode;
        
        // Handle keyboard shortcuts
        match event.code {
            KeyCode::Tab if event.modifiers.contains(&editor_core::input::Modifier::Ctrl) => {
                // Switch to next tab
                if let Some(active) = self.active_tab {
                    let next = (active + 1) % self.tabs.len();
                    self.active_tab = Some(next);
                }
            }
            KeyCode::Tab if event.modifiers.contains(&editor_core::input::Modifier::CtrlShift) => {
                // Switch to previous tab
                if let Some(active) = self.active_tab {
                    let prev = if active == 0 {
                        self.tabs.len() - 1
                    } else {
                        active - 1
                    };
                    self.active_tab = Some(prev);
                }
            }
            _ => {}
        }
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}
