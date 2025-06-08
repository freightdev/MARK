// source: src/libs/scan_tree.rs
// /src/libs/scan_tree.rs

fn walk_tree(base: &Path) -> Vec<TreeNode> {
    let mut nodes = Vec::new();

    for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let depth = path.components().count();

        // skip hidden junk
        if path.file_name().unwrap_or_default().to_string_lossy().starts_with('.') {
            continue;
        }

        // Build tree structure with path + file info
        nodes.push(TreeNode {
            path: path.to_path_buf(),
            depth,
            is_dir: path.is_dir(),
            file_type: detect_type(path),
        });
    }

    nodes
}

fn detect_type(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_string_lossy();

    if name.starts_with("marks.") {
        Some("agent_mark".to_string())
    } else if name.starts_with("markers.") {
        Some("marker_file".to_string())
    } else if name.starts_with("md.") {
        Some("agent_or_tool_md".to_string())
    } else if name == "agent.md" {
        Some("legacy_agent_file".to_string())
    } else if name == "tool.md" {
        Some("legacy_tool_file".to_string())
    } else {
        None
    }
}