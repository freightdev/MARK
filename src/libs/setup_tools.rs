// source: src/libs/setup_tools.rs
// src/libs/setup_tools.rs

use std::fs::{self, create_dir_all};
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::io::Write;

pub async fn run(dir: &str) {
    println!("\n🛠️ --setup-tools: Tool builder starting at {dir}\n");

    let theme = ColorfulTheme::default();
    let mut tools = Vec::new();

    loop {
        let tool_name: String = Input::with_theme(&theme)
            .with_prompt("🔧 Tool name")
            .interact_text()
            .unwrap();

        let tool_description: String = Input::with_theme(&theme)
            .with_prompt("📘 What does this tool do?")
            .interact_text()
            .unwrap();

        let tool_output: String = Input::with_theme(&theme)
            .with_prompt("📤 What kind of output does it produce?")
            .interact_text()
            .unwrap();

        let marker_interaction: String = Input::with_theme(&theme)
            .with_prompt("🧠 What markers or agents does it interact with (comma-separated)?")
            .interact_text()
            .unwrap();

        // Register the tool metadata
        tools.push((tool_name.clone(), tool_description.clone(), tool_output.clone(), marker_interaction.clone()));

        // Build the file structure
        let tool_path = Path::new(dir)
            .join(".mark/tools")
            .join(&tool_name)
            .join("marks");

        create_dir_all(tool_path.join("workspace")).unwrap();
        create_dir_all(tool_path.join("notebook")).unwrap();

        // Write md.[tool]
        let md_path = tool_path.join(format!("md.{}", tool_name));
        let md_content = format!(
            r#"# Tool: {tool_name}

## Description
{tool_description}

## Output
{tool_output}

## Interactions
{marker_interaction}
"#
        );
        fs::write(&md_path, md_content).unwrap();

        // Empty starter marks
        let marks_path = tool_path.join(format!("marks.{}", tool_name));
        let markers_path = tool_path.join(format!("markers.{}", tool_name));
        fs::write(&marks_path, "# Marks Placeholder\n").unwrap();
        fs::write(&markers_path, "# Markers Placeholder\n").unwrap();

        // Add to tool.marks index
        let index_path = Path::new(dir).join(".mark/tools/tool.marks");
        let entry = format!("- {tool_name}: {}
", tool_description);
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(index_path)
            .unwrap()
            .write_all(entry.as_bytes())
            .unwrap();

        let continue_prompt = Confirm::with_theme(&theme)
            .with_prompt("Add another tool?")
            .interact()
            .unwrap();

        if !continue_prompt {
            break;
        }
    }

    println!("\n✅ All tools initialized.");
}