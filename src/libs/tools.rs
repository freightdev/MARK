// source: src/libs/tools.rs
// src/libs/tools.rs

use std::fs;
use std::path::Path;

pub async fn run(dir: &str) {
    let tool_index = Path::new(dir).join(".mark/tools/tool.marks");

    println!("🧰 --tools: Scanning registered tools\n");

    let Ok(contents) = fs::read_to_string(&tool_index) else {
        eprintln!("❌ Could not read tool.marks at {}", tool_index.display());
        return;
    };

    for line in contents.lines().filter(|l| l.trim().starts_with("- ")) {
        let tool_path = line.trim().trim_start_matches("- ").trim();
        let full = Path::new(dir).join(".mark").join(tool_path);
        let tool_dir = full.parent().unwrap();
        let tool_name = tool_dir.file_name().unwrap().to_str().unwrap();

        let md_path = tool_dir.join(format!("md.{}", tool_name));
        println!("🔧 Tool: {}", tool_name);

        if md_path.exists() {
            if let Ok(data) = fs::read_to_string(&md_path) {
                for line in data.lines() {
                    if line.starts_with("## Intent") || line.starts_with("## Example Output") {
                        println!("  {}", line);
                    }
                }
            } else {
                println!("  ⚠️ Could not read metadata.");
            }
        } else {
            println!("  ❌ Missing md.{}", tool_name);
        }

        println!();
    }
}