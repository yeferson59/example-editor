//! File explorer widget

use crate::Widget;
use eframe::egui;
use std::path::{Path, PathBuf};

/// File tree node
#[derive(Debug)]
struct FileNode {
    /// Node name
    name: String,
    /// Full path
    path: PathBuf,
    /// Is directory
    is_dir: bool,
    /// Child nodes
    children: Vec<FileNode>,
    /// Is expanded
    expanded: bool,
}

impl FileNode {
    /// Creates a new file node
    fn new(path: PathBuf) -> std::io::Result<Self> {
        let metadata = path.metadata()?;
        let name = path.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut node = Self {
            name,
            path: path.clone(),
            is_dir: metadata.is_dir(),
            children: Vec::new(),
            expanded: false,
        };

        if metadata.is_dir() {
            node.load_children()?;
        }

        Ok(node)
    }

    /// Loads child nodes
    fn load_children(&mut self) -> std::io::Result<()> {
        self.children.clear();
        
        for entry in std::fs::read_dir(&self.path)? {
            let entry = entry?;
            let node = FileNode::new(entry.path())?;
            self.children.push(node);
        }

        self.children.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        Ok(())
    }
}

/// File explorer widget
pub struct FileExplorer {
    /// Root path
    root: Option<PathBuf>,
    /// Root node
    root_node: Option<FileNode>,
    /// Selected path
    selected: Option<PathBuf>,
    /// Scroll position
    scroll: egui::Vec2,
}

impl FileExplorer {
    /// Creates a new file explorer
    pub fn new() -> Self {
        Self {
            root: None,
            root_node: None,
            selected: None,
            scroll: egui::Vec2::ZERO,
        }
    }

    /// Sets the root path
    pub fn set_root(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let path = path.as_ref().to_path_buf();
        self.root = Some(path.clone());
        self.root_node = Some(FileNode::new(path)?);
        Ok(())
    }

    /// Shows a file node
    fn show_node(&mut self, ui: &mut egui::Ui, node: &mut FileNode) {
        let icon = if node.is_dir {
            if node.expanded { "📂" } else { "📁" }
        } else {
            "📄"
        };

        let mut label = format!("{} {}", icon, node.name);
        if node.is_dir {
            label.push_str("/");
        }

        let response = ui.selectable_label(
            Some(&node.path) == self.selected.as_ref(),
            label,
        );

        if response.clicked() {
            self.selected = Some(node.path.clone());
        }

        if response.double_clicked() && node.is_dir {
            node.expanded = !node.expanded;
            if node.expanded {
                if let Err(e) = node.load_children() {
                    log::error!("Failed to load directory: {}", e);
                }
            }
        }

        if node.expanded && node.is_dir {
            ui.indent("children", |ui| {
                for child in &mut node.children {
                    self.show_node(ui, child);
                }
            });
        }
    }
}

impl Widget for FileExplorer {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Files");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟳").clicked() {
                        if let Some(root) = &self.root {
                            if let Err(e) = self.set_root(root) {
                                log::error!("Failed to refresh: {}", e);
                            }
                        }
                    }
                });
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .id_source("file_explorer")
                .show(ui, |ui| {
                    if let Some(root) = self.root_node.as_mut() {
                        self.show_node(ui, root);
                    } else {
                        ui.label("No folder open");
                    }
                });
        });
    }

    fn update(&mut self) {
        // Update file explorer state
    }

    fn handle_input(&mut self, _event: &editor_core::input::KeyEvent) {
        // Handle keyboard navigation
    }
}

impl Default for FileExplorer {
    fn default() -> Self {
        Self::new()
    }
}
