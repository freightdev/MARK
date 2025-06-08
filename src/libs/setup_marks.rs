// source: src/libs/setup_marks.rs
// src/libs/setup_marks.rs

use std::fs::{self, create_dir_all};
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub async fn run(dir: &str) {
    println!("\n🧠 --setup-marks: Mark file generator starting at {dir}\n");

    let theme = ColorfulTheme::default();
    let mut created = Vec::new();

    loop {
        let mark_name: String = Input::with_theme(&theme)
            .with_prompt("📝 Mark name (e.g., packetpilot)")
            .interact_text()
            .unwrap();

        let intent: String = Input::with_theme(&theme)
            .with_prompt("🎯 What is the intent of this mark?")
            .interact_text()
            .unwrap();

        let agent: String = Input::with_theme(&theme)
            .with_prompt("🤖 Which agent will run this mark?")
            .interact_text()
            .unwrap();

        let tools: String = Input::with_theme(&theme)
            .with_prompt("🧰 Which tools will be used (comma-separated)?")
            .interact_text()
            .unwrap();

        let task: String = Input::with_theme(&theme)
            .with_prompt("🛠️ Describe the exact task this mark performs")
            .interact_text()
            .unwrap();

        let next: String = Input::with_theme(&theme)
            .with_prompt("➡️ Next mark to run (optional)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let marks_path = Path::new(dir).join(".mark/agents").join(&agent).join("marks");
        create_dir_all(&marks_path).unwrap();

        let mark_file = marks_path.join(format!("marks.{}", mark_name));
        let content = format!(
            r#"# MARK: {mark_name}

## Intent
{intent}

## Agent
{agent}

## Tools
- {}

## Task
{task}

{}"#,
            tools.replace(",", "\n- "),
            if next.trim().is_empty() {
                String::from("## Next\n(none)")
            } else {
                format!("## Next\n{}", next)
            }
        );

        fs::write(&mark_file, content).unwrap();
        created.push(mark_file);

        let continue_prompt = Confirm::with_theme(&theme)
            .with_prompt("Create another mark?")
            .interact()
            .unwrap();

        if !continue_prompt {
            break;
        }
    }

    println!("\n✅ Created marks:");
    for path in created {
        println!(" - {}", path.display());
    }
}