// source: src/libs/utils.rs
// src/libs/utils.rs

use std::path::{Path, PathBuf};

/// Converts a raw string to a MARK-safe ID (snake_case).
pub fn sanitize_id(raw: &str) -> String {
    raw.to_lowercase()
        .replace([' ', '-'], "_")
        .replace("__", "_")
}

/// Converts a snake_case ID to Title Case (for human-readable names).
pub fn title_case(id: &str) -> String {
    id.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Given a `.marks` file path, returns the agent/tool name (from its parent).
pub fn extract_name_from_path(path: &Path) -> Option<String> {
    path.parent()
        .and_then(|parent| parent.file_name())
        .and_then(|os| os.to_str())
        .map(|s| s.to_string())
}

/// Ensures a folder path exists (with proper error logging).
pub fn ensure_dir(path: &PathBuf) {
    if let Err(e) = std::fs::create_dir_all(path) {
        eprintln!("❌ Failed to create directory {}: {}", path.display(), e);
    }
}