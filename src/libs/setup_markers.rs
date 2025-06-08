// source: src/libs/setup_markers.rs
// src/libs/setup_markers.rs

use std::fs::{self, create_dir_all};
use std::io::Write;
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub async fn run(dir: &str) {
    println!("\n📍 --setup-markers: Marker generator starting at {dir}\n");

    let theme = ColorfulTheme::default();

    loop {
        let agent: String = Input::with_theme(&theme)
            .with_prompt("🤖 Target agent for this marker")
            .interact_text()
            .unwrap();

        let marker_name: String = Input::with_theme(&theme)
            .with_prompt("📛 Marker name")
            .interact_text()
            .unwrap();

        let trigger: String = Input::with_theme(&theme)
            .with_prompt("🎯 When should this marker be used?")
            .interact_text()
            .unwrap();

        let effect: String = Input::with_theme(&theme)
            .with_prompt("🌀 What should the marker trigger or assist with?")
            .interact_text()
            .unwrap();

        // Build paths
        let marks_path = Path::new(dir).join(".mark/agents").join(&agent).join("marks");
        create_dir_all(&marks_path).unwrap();

        let marker_path = marks_path.join(format!("markers.{}", agent));

        // Append or create marker file
        let content = format!(
            r#"\n## Marker: {marker_name}

### Trigger
{trigger}

### Effect
{effect}
"#
        );
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&marker_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        let continue_prompt = Confirm::with_theme(&theme)
            .with_prompt("Add another marker?")
            .interact()
            .unwrap();

        if !continue_prompt {
            break;
        }
    }

    println!("\n✅ Marker setup complete.");
}