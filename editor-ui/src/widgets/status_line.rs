//! Status line widget

use crate::Widget;
use eframe::egui;
use std::time::{Duration, Instant};

/// Status message
#[derive(Clone)]
struct StatusMessage {
    /// Message text
    text: String,
    /// Message type
    message_type: MessageType,
    /// Timestamp
    timestamp: Instant,
    /// Duration to show
    duration: Option<Duration>,
}

/// Message type
#[derive(Clone, Copy, PartialEq)]
enum MessageType {
    /// Information
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
}

/// Status line widget
pub struct StatusLine {
    /// Current messages
    messages: Vec<StatusMessage>,
    /// Editor mode
    mode: String,
    /// Cursor position
    cursor_position: (usize, usize),
    /// File encoding
    encoding: String,
    /// Line ending style
    line_ending: String,
    /// Git branch
    git_branch: Option<String>,
    /// LSP status
    lsp_status: String,
}

impl StatusLine {
    /// Creates a new status line
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            mode: "NORMAL".to_string(),
            cursor_position: (1, 1),
            encoding: "UTF-8".to_string(),
            line_ending: "LF".to_string(),
            git_branch: None,
            lsp_status: String::new(),
        }
    }

    /// Sets the editor mode
    pub fn set_mode(&mut self, mode: impl Into<String>) {
        self.mode = mode.into();
    }

    /// Sets the cursor position
    pub fn set_cursor_position(&mut self, line: usize, column: usize) {
        self.cursor_position = (line + 1, column + 1);
    }

    /// Sets the file encoding
    pub fn set_encoding(&mut self, encoding: impl Into<String>) {
        self.encoding = encoding.into();
    }

    /// Sets the line ending style
    pub fn set_line_ending(&mut self, line_ending: impl Into<String>) {
        self.line_ending = line_ending.into();
    }

    /// Sets the Git branch
    pub fn set_git_branch(&mut self, branch: Option<String>) {
        self.git_branch = branch;
    }

    /// Sets the LSP status
    pub fn set_lsp_status(&mut self, status: impl Into<String>) {
        self.lsp_status = status.into();
    }

    /// Shows an info message
    pub fn info(&mut self, text: impl Into<String>, duration: Option<Duration>) {
        self.show_message(text.into(), MessageType::Info, duration);
    }

    /// Shows a warning message
    pub fn warning(&mut self, text: impl Into<String>, duration: Option<Duration>) {
        self.show_message(text.into(), MessageType::Warning, duration);
    }

    /// Shows an error message
    pub fn error(&mut self, text: impl Into<String>, duration: Option<Duration>) {
        self.show_message(text.into(), MessageType::Error, duration);
    }

    /// Shows a message
    fn show_message(&mut self, text: String, message_type: MessageType, duration: Option<Duration>) {
        self.messages.push(StatusMessage {
            text,
            message_type,
            timestamp: Instant::now(),
            duration,
        });
    }

    /// Updates messages
    fn update_messages(&mut self) {
        self.messages.retain(|msg| {
            if let Some(duration) = msg.duration {
                msg.timestamp.elapsed() < duration
            } else {
                true
            }
        });
    }

    /// Gets the color for a message type
    fn message_color(&self, message_type: MessageType, ui: &egui::Ui) -> egui::Color32 {
        match message_type {
            MessageType::Info => ui.style().visuals.text_color(),
            MessageType::Warning => egui::Color32::YELLOW,
            MessageType::Error => egui::Color32::RED,
        }
    }
}

impl Widget for StatusLine {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left side
            ui.label(&self.mode);
            ui.separator();

            // Messages
            if !self.messages.is_empty() {
                let message = &self.messages[self.messages.len() - 1];
                ui.colored_label(
                    self.message_color(message.message_type, ui),
                    &message.text,
                );
                ui.separator();
            }

            // Git branch
            if let Some(branch) = &self.git_branch {
                ui.label(format!("⎇ {}", branch));
                ui.separator();
            }

            // LSP status
            if !self.lsp_status.is_empty() {
                ui.label(&self.lsp_status);
                ui.separator();
            }

            // Right side
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(&self.encoding);
                ui.separator();
                ui.label(&self.line_ending);
                ui.separator();
                ui.label(format!("{}:{}", self.cursor_position.0, self.cursor_position.1));
            });
        });
    }

    fn update(&mut self) {
        self.update_messages();
    }

    fn handle_input(&mut self, _event: &editor_core::input::KeyEvent) {
        // Handle keyboard input
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self::new()
    }
}
