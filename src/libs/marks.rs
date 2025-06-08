// source: src/libs/marks.rs
// src/libs/marks.rs

use std::fs;
use std::path::{Path, PathBuf};

pub async fn run(dir: &str) {
    println!("📄 --marks: Listing all .marks under `{dir}`\n");

    let base = Path::new(dir).join(".mark");

    let agents_index = base.join("agents/agent.marks");
    let tools_index = base.join("tools/tool.marks");

    println!("--- Agents ---");
    print_mark_index(&agents_index, true);

    println!("\n--- Tools ---");
    print_mark_index(&tools_index, false);
}

fn print_mark_index(index: &Path, is_agent: bool) {
    if let Ok(lines) = fs::read_to_string(index) {
        for line in lines.lines().filter_map(|l| l.strip_prefix("- ").map(str::trim)) {
            let mark_path = index.parent().unwrap().join(line);
            let meta = mark_path.display().to_string();

            println!("📁 {}", meta);

            if let Ok(contents) = fs::read_to_string(&mark_path) {
                for l in contents.lines().filter(|l| l.starts_with("#") || l.starts_with("##")) {
                    println!("  {}", l);
                }
            }

            let prefix = if is_agent { "md" } else { "md" };
            let id = mark_path.parent().unwrap().file_name().unwrap().to_str().unwrap();
            let meta_path = mark_path.parent().unwrap().join(format!("{prefix}.{id}"));

            if meta_path.exists() {
                if let Ok(contents) = fs::read_to_string(&meta_path) {
                    for l in contents.lines().filter(|l| l.contains("Capabilities") || l.contains("Intent")) {
                        println!("  🔧 {l}");
                    }
                }
            }
        }
    } else {
        println!("❌ Could not read: {}", index.display());
    }
}