//! Main application window

use eframe::egui;
use editor_core::Editor;
use crate::{UiError, theme::Theme};
use std::sync::Arc;
use tokio::sync::RwLock;
use editor_syntax::{Highlighter, HighlightEvent, get_language_by_extension};
use rfd::FileDialog;
use std::fs;

/// Main application state
#[allow(dead_code)]
pub struct EditorApp {
    /// Editor instance
    editor: Arc<RwLock<Editor>>,
    /// Current theme
    theme: Theme,
    /// UI state
    ui_state: UiState,
    /// Current document content
    current_document_content: String,
    /// Cursor position (line, column)
    cursor_position: (usize, usize),
}

/// UI state
#[derive(Default)]
#[allow(dead_code)]
struct UiState {
    /// Show command palette
    show_command_palette: bool,
    /// Show file explorer
    show_file_explorer: bool,
    /// Show search panel
    show_search: bool,
    /// Show settings panel
    show_settings: bool,
    /// Panel sizes
    panel_sizes: PanelSizes,
    /// Current file name
    file_name: String,
}

/// Panel sizes
#[derive(Default)]
#[allow(dead_code)]
struct PanelSizes {
    /// Left panel width - TODO: Implement panel resizing
    left_panel: f32,
    /// Right panel width - TODO: Implement panel resizing
    right_panel: f32,
    /// Bottom panel height - TODO: Implement panel resizing
    bottom_panel: f32,
}

impl EditorApp {
    /// Creates a new editor application
    pub fn new(editor: Editor) -> Self {
        Self {
            editor: Arc::new(RwLock::new(editor)),
            theme: Theme::default(),
            ui_state: UiState::default(),
            current_document_content: String::new(),
            cursor_position: (0, 0),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme.apply(ctx);

        // Show menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });

        // Show editor
        self.show_editor(ctx);

        // Bottom panel (output, search results)
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(100.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.show_bottom_panel(ui);
            });

        // File explorer panel
        if self.ui_state.show_file_explorer {
            egui::SidePanel::left("file_explorer")
                .min_width(200.0)
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_file_explorer(ui);
                });
        }

        // Handle keyboard shortcuts
        self.handle_shortcuts(ctx);
    }
}

impl EditorApp {
    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.current_document_content.clear();
                    self.cursor_position = (0, 0);
                    self.ui_state.file_name = "untitled".to_string();
                }
                if ui.button("Open...").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.current_document_content = content;
                            self.cursor_position = (0, 0);
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                self.ui_state.file_name = name.to_string();
                            }
                        }
                    }
                }
                if ui.button("Save").clicked() {
                    // TODO: Save current file
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    // TODO: Exit application
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // TODO: Undo
                }
                if ui.button("Redo").clicked() {
                    // TODO: Redo
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    // TODO: Cut
                }
                if ui.button("Copy").clicked() {
                    // TODO: Copy
                }
                if ui.button("Paste").clicked() {
                    // TODO: Paste
                }
            });

            ui.menu_button("View", |ui| {
                if ui.checkbox(&mut self.ui_state.show_file_explorer, "File Explorer").clicked() {
                    // Toggle file explorer
                }
                if ui.checkbox(&mut self.ui_state.show_search, "Search").clicked() {
                    // Toggle search panel
                }
            });
        });
    }

    fn show_file_explorer(&mut self, ui: &mut egui::Ui) {
        ui.heading("Files");
        // TODO: Show file tree
    }

    fn show_outline(&mut self, ui: &mut egui::Ui) {
        ui.heading("Outline");
        // TODO: Show document outline
    }

    fn show_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let _search_selected = ui.selectable_value(&mut self.ui_state.show_search, true, "Search").clicked();
            let problems_selected = ui.selectable_label(true, "Problems").clicked();
            let output_selected = ui.selectable_label(true, "Output").clicked();
            
            if problems_selected {
                // TODO: Handle problems panel selection
            }
            
            if output_selected {
                // TODO: Handle output panel selection
            }
        });

        ui.separator();

        if self.ui_state.show_search {
            // TODO: Show search results
        }
    }

    fn show_editor(&mut self, ctx: &egui::Context) {
        use egui::{TextStyle, text::TextFormat};
    
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.ui_state.file_name);
            });
        });
    
        egui::CentralPanel::default().show(ctx, |ui| {
            let file_name = &self.ui_state.file_name;
            let lang = file_name.split('.').last().and_then(get_language_by_extension);
            let mut highlighter = Highlighter::new();
            if let Some(language) = lang {
                let _ = highlighter.set_language(language);
            }
    
            let mut layouter = move |ui: &egui::Ui, text: &str, _wrap_width: f32| {
                let mut layout_job = egui::text::LayoutJob::default();
                if let Ok(events) = highlighter.highlight(text) {
                    for event in events {
                        if let HighlightEvent::Source { start, end, style } = event {
                            let part = &text[start..end];
                            let mut format = TextFormat::default();
                            format.font_id = TextStyle::Monospace.resolve(ui.style());
                            format.color = style.foreground
                                .map(|fg| egui::Color32::from_rgb(fg.r, fg.g, fg.b))
                                .unwrap_or_else(|| ui.visuals().text_color());
                            layout_job.append(part, 0.0, format);
                        }
                    }
                } else {
                    let mut format = TextFormat::default();
                    format.font_id = TextStyle::Monospace.resolve(ui.style());
                    format.color = ui.visuals().text_color();
                    layout_job.append(text, 0.0, format);
                }
                ui.fonts(|f| f.layout_job(layout_job))
            };
    
            egui::TextEdit::multiline(&mut self.current_document_content)
                .font(TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .desired_rows(30)
                .layouter(&mut layouter)
                .show(ui);
        });
    }
    
    fn show_command_palette(&mut self, ctx: &egui::Context) {
        egui::Window::new("Command Palette")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |_ui| {
                // TODO: Show command palette
            });
    }

    fn show_settings(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(true)
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |_ui| {
                // TODO: Show settings
            });
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::P) && i.modifiers.command {
                self.ui_state.show_command_palette = true;
            }
            // Ctrl+N: New file
            if i.modifiers.command && i.key_pressed(egui::Key::N) {
                self.current_document_content.clear();
                self.cursor_position = (0, 0);
                self.ui_state.file_name = "untitled".to_string();
            }
            // Ctrl+O: Open file
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        self.current_document_content = content;
                        self.cursor_position = (0, 0);
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            self.ui_state.file_name = name.to_string();
                        }
                    }
                }
            }
            // Ctrl+S: Save file
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                let mut save_path = None;
                if self.ui_state.file_name == "untitled" || self.ui_state.file_name.is_empty() {
                    if let Some(path) = FileDialog::new().set_title("Save File").save_file() {
                        save_path = Some(path);
                    }
                } else {
                    if let Some(path) = FileDialog::new().set_file_name(&self.ui_state.file_name).save_file() {
                        save_path = Some(path);
                    }
                }
                if let Some(path) = save_path {
                    if let Err(e) = fs::write(&path, &self.current_document_content) {
                        eprintln!("Error saving file: {}", e);
                    } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        self.ui_state.file_name = name.to_string();
                    }
                }
            }
            // Ctrl+W: Close file
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                self.current_document_content.clear();
                self.cursor_position = (0, 0);
                self.ui_state.file_name = "untitled".to_string();
            }
        });
    }
}

/// Runs the editor application
pub fn run(editor: Editor) -> std::result::Result<(), UiError> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Editor",
        options,
        Box::new(|_cc| Box::new(EditorApp::new(editor))),
    )?;

    Ok(())
}
