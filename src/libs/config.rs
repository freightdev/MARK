// source: src/libs/config.rs
// src/libs/config.rs

use std::fs;
use std::path::Path;
use dialoguer::{Input, Confirm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MarkConfig {
    llm_model: String,
    default_agent: String,
    tool_path: String,
    cache_enabled: bool,
    bookmark_depth: usize,
}

pub async fn run(dir: &str) {
    println!("⚙️  --config: Creating/editing .markrc in `{dir}`");

    let config_path = Path::new(dir).join(".markrc");

    let default_config = MarkConfig {
        llm_model: "gpt-4".to_string(),
        default_agent: "packetpilot".to_string(),
        tool_path: ".mark/tools/".to_string(),
        cache_enabled: true,
        bookmark_depth: 3,
    };

    let llm_model: String = Input::new()
        .with_prompt("🧠 Default LLM Model")
        .default(default_config.llm_model.clone())
        .interact_text()
        .unwrap();

    let default_agent: String = Input::new()
        .with_prompt("🤖 Default Agent")
        .default(default_config.default_agent.clone())
        .interact_text()
        .unwrap();

    let tool_path: String = Input::new()
        .with_prompt("🧰 Tool Directory Path")
        .default(default_config.tool_path.clone())
        .interact_text()
        .unwrap();

    let cache_enabled: bool = Confirm::new()
        .with_prompt("💾 Enable Cache?")
        .default(default_config.cache_enabled)
        .interact()
        .unwrap();

    let bookmark_depth: usize = Input::new()
        .with_prompt("📚 Max Bookmark Depth (for --bookmarks)")
        .default(default_config.bookmark_depth)
        .interact_text()
        .unwrap();

    let updated = MarkConfig {
        llm_model,
        default_agent,
        tool_path,
        cache_enabled,
        bookmark_depth,
    };

    let toml = toml::to_string_pretty(&updated).unwrap();
    fs::write(config_path, toml).unwrap();

    println!("✅ .markrc configuration saved.");
}