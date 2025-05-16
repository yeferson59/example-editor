//! Text markers and annotations system

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A marker in the text buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    /// Marker name
    pub name: String,
    /// Position in the buffer
    pub position: usize,
    /// Marker type
    pub marker_type: MarkerType,
    /// Marker data
    pub data: Option<serde_json::Value>,
}

/// Marker type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerType {
    /// Cursor position
    Cursor,
    /// Selection
    Selection,
    /// Bookmark
    Bookmark,
    /// Search result
    SearchResult,
    /// Error or warning
    Diagnostic {
        /// Severity level
        severity: DiagnosticSeverity,
        /// Source of the diagnostic
        source: String,
    },
    /// Custom marker type
    Custom(String),
}

/// Diagnostic severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    /// Error
    Error,
    /// Warning
    Warning,
    /// Information
    Info,
    /// Hint
    Hint,
}

/// Set of markers in a buffer
#[derive(Debug, Default)]
pub struct MarkerSet {
    /// Markers by name
    markers: HashMap<String, Marker>,
    /// Markers by type
    markers_by_type: HashMap<MarkerType, Vec<String>>,
}

impl MarkerSet {
    /// Creates a new marker set
    pub fn new() -> Self {
        Self {
            markers: HashMap::new(),
            markers_by_type: HashMap::new(),
        }
    }

    /// Sets a marker
    pub fn set(&mut self, name: &str, position: usize) {
        self.set_with_type(name, position, MarkerType::Bookmark);
    }

    /// Sets a marker with a specific type
    pub fn set_with_type(&mut self, name: &str, position: usize, marker_type: MarkerType) {
        let marker = Marker {
            name: name.to_string(),
            position,
            marker_type: marker_type.clone(),
            data: None,
        };

        // Remove from old type group if exists
        if let Some(old_marker) = self.markers.get(name) {
            if let Some(type_markers) = self.markers_by_type.get_mut(&old_marker.marker_type) {
                type_markers.retain(|n| n != name);
            }
        }

        // Add to new type group
        self.markers_by_type
            .entry(marker_type)
            .or_insert_with(Vec::new)
            .push(name.to_string());

        self.markers.insert(name.to_string(), marker);
    }

    /// Sets a marker with data
    pub fn set_with_data(&mut self, name: &str, position: usize, marker_type: MarkerType, data: serde_json::Value) {
        let marker = Marker {
            name: name.to_string(),
            position,
            marker_type: marker_type.clone(),
            data: Some(data),
        };

        // Remove from old type group if exists
        if let Some(old_marker) = self.markers.get(name) {
            if let Some(type_markers) = self.markers_by_type.get_mut(&old_marker.marker_type) {
                type_markers.retain(|n| n != name);
            }
        }

        // Add to new type group
        self.markers_by_type
            .entry(marker_type)
            .or_insert_with(Vec::new)
            .push(name.to_string());

        self.markers.insert(name.to_string(), marker);
    }

    /// Gets a marker\'s position
    pub fn get(&self, name: &str) -> Option<usize> {
        self.markers.get(name).map(|m| m.position)
    }

    /// Gets a marker\'s data
    pub fn get_data(&self, name: &str) -> Option<&serde_json::Value> {
        self.markers.get(name).and_then(|m| m.data.as_ref())
    }

    /// Gets all markers of a specific type
    pub fn get_by_type(&self, marker_type: &MarkerType) -> Vec<&Marker> {
        self.markers_by_type
            .get(marker_type)
            .map(|names| {
                names.iter()
                    .filter_map(|name| self.markers.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Removes a marker
    pub fn remove(&mut self, name: &str) {
        if let Some(marker) = self.markers.remove(name) {
            if let Some(type_markers) = self.markers_by_type.get_mut(&marker.marker_type) {
                type_markers.retain(|n| n != name);
            }
        }
    }

    /// Updates marker positions after text changes
    pub fn update_positions(&mut self, position: usize, offset: isize) {
        for marker in self.markers.values_mut() {
            if marker.position >= position {
                marker.position = (marker.position as isize + offset).max(0) as usize;
            }
        }
    }

    /// Returns all markers in a range
    pub fn in_range(&self, start: usize, end: usize) -> Vec<&Marker> {
        self.markers
            .values()
            .filter(|m| m.position >= start && m.position < end)
            .collect()
    }

    /// Returns the number of markers
    pub fn len(&self) -> usize {
        self.markers.len()
    }

    /// Returns true if there are no markers
    pub fn is_empty(&self) -> bool {
        self.markers.is_empty()
    }

    /// Clears all markers
    pub fn clear(&mut self) {
        self.markers.clear();
        self.markers_by_type.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_operations() {
        let mut markers = MarkerSet::new();
        
        // Test basic marker operations
        markers.set("bookmark1", 10);
        assert_eq!(markers.get("bookmark1"), Some(10));
        
        markers.set("bookmark2", 20);
        assert_eq!(markers.len(), 2);
        
        markers.remove("bookmark1");
        assert_eq!(markers.len(), 1);
        assert_eq!(markers.get("bookmark1"), None);
    }

    #[test]
    fn test_marker_types() {
        let mut markers = MarkerSet::new();
        
        // Add markers of different types
        markers.set_with_type("cursor", 5, MarkerType::Cursor);
        markers.set_with_type("error", 10, MarkerType::Diagnostic {
            severity: DiagnosticSeverity::Error,
            source: "linter".to_string(),
        });
        
        // Test getting markers by type
        let cursors = markers.get_by_type(&MarkerType::Cursor);
        assert_eq!(cursors.len(), 1);
        assert_eq!(cursors[0].position, 5);
    }

    #[test]
    fn test_marker_data() {
        let mut markers = MarkerSet::new();
        
        // Add marker with data
        markers.set_with_data(
            "diagnostic",
            15,
            MarkerType::Diagnostic {
                severity: DiagnosticSeverity::Warning,
                source: "analyzer".to_string(),
            },
            serde_json::json!({
                "message": "Unused variable",
                "code": "W001"
            }),
        );

        // Verify data
        let data = markers.get_data("diagnostic").unwrap();
        assert_eq!(data["message"], "Unused variable");
        assert_eq!(data["code"], "W001");
    }

    #[test]
    fn test_position_updates() {
        let mut markers = MarkerSet::new();
        
        markers.set("m1", 10);
        markers.set("m2", 20);
        markers.set("m3", 30);

        // Test position updates after insertion
        markers.update_positions(15, 5);
        assert_eq!(markers.get("m1"), Some(10)); // Before insertion point
        assert_eq!(markers.get("m2"), Some(25)); // After insertion point
        assert_eq!(markers.get("m3"), Some(35)); // After insertion point

        // Test position updates after deletion
        markers.update_positions(15, -5);
        assert_eq!(markers.get("m1"), Some(10)); // Before deletion point
        assert_eq!(markers.get("m2"), Some(20)); // After deletion point
        assert_eq!(markers.get("m3"), Some(30)); // After deletion point
    }

    #[test]
    fn test_range_queries() {
        let mut markers = MarkerSet::new();
        
        markers.set("m1", 10);
        markers.set("m2", 20);
        markers.set("m3", 30);
        markers.set("m4", 40);

        let range_markers = markers.in_range(15, 35);
        assert_eq!(range_markers.len(), 2);
        assert!(range_markers.iter().any(|m| m.position == 20));
        assert!(range_markers.iter().any(|m| m.position == 30));
    }
}
