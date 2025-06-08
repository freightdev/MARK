// source: src/libs/agents.rs
// src/libs/agents.rs

use std::fs;
use std::path::Path;

pub async fn run(dir: &str) {
    println!("🧬 --agents: Scanning registered agents in {}", dir);

    let index_path = Path::new(dir).join(".mark/agents/agent.marks");

    let Ok(contents) = fs::read_to_string(&index_path) else {
        eprintln!("❌ Could not read agent.marks at {}", index_path.display());
        return;
    };

    for line in contents.lines().filter(|l| l.trim().starts_with("- ")) {
        let rel_path = line.trim().trim_start_matches("- ").trim();
        let full = Path::new(dir).join(".mark").join(rel_path);
        let agent_dir = full.parent().unwrap();
        let agent_name = agent_dir.file_name().unwrap().to_str().unwrap();

        println!("🤖 Agent: {}", agent_name);

        let md_path = agent_dir.join(format!("md.{}", agent_name));
        if md_path.exists() {
            if let Ok(data) = fs::read_to_string(&md_path) {
                for line in data.lines() {
                    if line.starts_with("## Capabilities") || line.starts_with("- ") {
                        println!("  {}", line);
                    }
                }
            } else {
                println!("  ⚠️ Could not read md.{}", agent_name);
            }
        } else {
            println!("  ❌ Missing md.{}", agent_name);
        }

        println!();
    }
}