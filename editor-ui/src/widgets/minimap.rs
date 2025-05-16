//! Minimap widget

use crate::Widget;
use eframe::egui;
use editor_core::Buffer;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Minimap widget
pub struct Minimap {
    /// Buffer reference
    buffer: Option<Arc<RwLock<Buffer>>>,
    /// Scale factor
    scale: f32,
    /// Visible region (start_line, end_line)
    visible_region: (usize, usize),
    /// Total number of lines
    total_lines: usize,
    /// Line height in pixels
    line_height: f32,
}

impl Minimap {
    /// Creates a new minimap
    pub fn new() -> Self {
        Self {
            buffer: None,
            scale: 0.5,
            visible_region: (0, 0),
            total_lines: 0,
            line_height: 2.0,
        }
    }

    /// Sets the buffer to display
    pub fn set_buffer(&mut self, buffer: Option<Arc<RwLock<Buffer>>>) {
        self.buffer = buffer;
    }

    /// Sets the visible region
    pub fn set_visible_region(&mut self, start: usize, end: usize) {
        self.visible_region = (start, end);
    }

    /// Sets the scale factor
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(0.1, 1.0);
    }

    /// Renders a line of text
    fn render_line(&self, ui: &mut egui::Ui, text: &str, y: f32) {
        let rect = egui::Rect::from_min_size(
            egui::pos2(ui.min_rect().left(), y),
            egui::vec2(ui.available_width(), self.line_height),
        );

        // Calculate line density (ratio of non-whitespace characters)
        let density = text.chars().filter(|c| !c.is_whitespace()).count() as f32 / text.len().max(1) as f32;

        // Use density to determine color intensity
        let color = ui.style().visuals.text_color().linear_multiply(density);

        ui.painter().rect_filled(rect, 0.0, color);
    }
}

impl Widget for Minimap {
    fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(buffer) = &self.buffer {
            let buffer = buffer.blocking_read();
            let text = buffer.text();
            let lines: Vec<&str> = text.lines().collect();
            self.total_lines = lines.len();

            let total_height = self.total_lines as f32 * self.line_height;
            let visible_height = ui.available_height();

            // Calculate visible range
            let visible_ratio = visible_height / total_height;
            let visible_start = (self.visible_region.0 as f32 * visible_ratio) * visible_height;
            let visible_end = (self.visible_region.1 as f32 * visible_ratio) * visible_height;

            // Draw visible region indicator
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(ui.min_rect().left(), visible_start),
                    egui::pos2(ui.max_rect().right(), visible_end),
                ),
                0.0,
                ui.style().visuals.selection.bg_fill.linear_multiply(0.3),
            );

            // Draw lines
            for (i, line) in lines.iter().enumerate() {
                let y = i as f32 * self.line_height;
                self.render_line(ui, line, y);
            }

            // Handle clicks
            if let Some(click_pos) = ui.input().pointer.press_origin() {
                let ratio = (click_pos.y - ui.min_rect().top()) / ui.available_height();
                let line = (ratio * self.total_lines as f32) as usize;
                // TODO: Emit event to scroll main editor to this line
            }
        }
    }

    fn update(&mut self) {
        // Update minimap state
    }

    fn handle_input(&mut self, _event: &editor_core::input::KeyEvent) {
        // Handle keyboard input
    }
}

impl Default for Minimap {
    fn default() -> Self {
        Self::new()
    }
}
