//! Editor view component

use eframe::egui;
use editor_core::{Buffer, Document};
use crate::{Widget, Theme};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Editor view state
pub struct EditorView {
    /// Current document
    document: Option<Arc<RwLock<Document>>>,
    /// Editor theme
    theme: Theme,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// Scroll position (lines)
    scroll: f32,
    /// Line numbers visible
    show_line_numbers: bool,
    /// Minimap visible
    show_minimap: bool,
    /// Font size
    font_size: f32,
}

impl EditorView {
    /// Creates a new editor view
    pub fn new() -> Self {
        Self {
            document: None,
            theme: Theme::default(),
            cursor: (0, 0),
            scroll: 0.0,
            show_line_numbers: true,
            show_minimap: true,
            font_size: 14.0,
        }
    }

    /// Sets the current document
    pub fn set_document(&mut self, document: Option<Arc<RwLock<Document>>>) {
        self.document = document;
    }

    /// Sets the editor theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Shows line numbers
    fn show_line_numbers(&self, ui: &mut egui::Ui, total_lines: usize) {
        let line_number_width = (total_lines.to_string().len() * 8) as f32;
        
        egui::SidePanel::left("line_numbers")
            .exact_width(line_number_width)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    for line in 0..total_lines {
                        ui.label(format!("{}", line + 1));
                    }
                });
            });
    }

    /// Shows the minimap
    fn show_minimap(&self, ui: &mut egui::Ui, text: &str) {
        egui::SidePanel::right("minimap")
            .exact_width(100.0)
            .resizable(false)
            .show_inside(ui, |ui| {
                // TODO: Implement minimap rendering
            });
    }

    /// Handles text input at current cursor position
    fn handle_text_input(&mut self, text: &str) {
        if let Some(doc) = &self.document {
            let mut doc = doc.write();
            let (line, col) = self.cursor;
            // Calculate the actual position in the buffer
            let position = doc.text().lines()
                .take(line)
                .map(|l| l.len() + 1) // +1 for newline
                .sum::<usize>() + col;

            if let Err(e) = doc.insert(position, text) {
                log::error!("Failed to insert text: {}", e);
                return;
            }

            // Update cursor position
            if text == "\n" {
                self.cursor = (line + 1, 0);
            } else {
                self.cursor = (line, col + text.len());
            }
        }
    }

    /// Renders text content
    fn render_text(&mut self, ui: &mut egui::Ui) {
        if let Some(doc) = &self.document {
            let doc = doc.read();
            let text = doc.text();
            
            let text_edit = egui::TextEdit::multiline(&mut text.to_string())
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .lock_focus(true);

            let response = ui.add(text_edit);

            // Handle keyboard input
            if response.has_focus() {
                let input = ui.input();
                
                // Handle regular text input
                if let Some(text) = input.events.iter().find_map(|event| {
                    if let egui::Event::Text(text) = event {
                        Some(text)
                    } else {
                        None
                    }
                }) {
                    self.handle_text_input(text);
                }

                // Handle special keys
                for event in &input.events {
                    match event {
                        egui::Event::Key {
                            key: egui::Key::Enter,
                            pressed: true,
                            ..
                        } => {
                            self.handle_text_input("\n");
                        }
                        egui::Event::Key {
                            key: egui::Key::Backspace,
                            pressed: true,
                            ..
                        } => {
                            if let Some(doc) = &self.document {
                                let mut doc = doc.write();
                                let (line, col) = self.cursor;
                                if col > 0 {
                                    // Delete previous character
                                    let position = doc.text().lines()
                                        .take(line)
                                        .map(|l| l.len() + 1)
                                        .sum::<usize>() + col - 1;
                                    
                                    if let Err(e) = doc.delete(position, position + 1) {
                                        log::error!("Failed to delete text: {}", e);
                                    } else {
                                        self.cursor = (line, col - 1);
                                    }
                                } else if line > 0 {
                                    // Move to end of previous line
                                    let prev_line_len = doc.text().lines()
                                        .nth(line - 1)
                                        .map(|l| l.len())
                                        .unwrap_or(0);
                                    self.cursor = (line - 1, prev_line_len);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Widget for EditorView {
    fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(doc) = &self.document {
            let doc = doc.read();
            let text = doc.text();
            let total_lines = text.lines().count();

            ui.horizontal(|ui| {
                if self.show_line_numbers {
                    self.show_line_numbers(ui, total_lines);
                }

                self.render_text(ui);

                if self.show_minimap {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.label(&text);
                    });
                }
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.heading("No file open - Press Ctrl+N to create a new file");
            });
        }
    }

    fn update(&mut self) {
        // Update editor state
    }

    fn handle_input(&mut self, event: &editor_core::input::KeyEvent) {
        // Handle keyboard input
    }
}

impl Default for EditorView {
    fn default() -> Self {
        Self::new()
    }
}
