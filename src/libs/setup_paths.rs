// source: src/libs/setup_paths.rs
// src/libs/setup_paths.rs

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Input};

pub async fn run(dir: &str) {
    println!("\n🧭 --setup-paths: Creating MARK tree at {dir}\n");

    let theme = ColorfulTheme::default();

    let mstp_name: String = Input::with_theme(&theme)
        .with_prompt("📘 Name for your master path file (e.g., mark.mstp)")
        .default("mark.mstp".into())
        .interact_text()
        .unwrap();

    let mstp_path = Path::new(dir).join(".mark").join(&mstp_name);
    create_dir_all(mstp_path.parent().unwrap()).unwrap();

    let mut mstp_file = File::create(&mstp_path).unwrap();

    let root_story: String = Input::with_theme(&theme)
        .with_prompt("📖 Describe your MARK root story/mission")
        .interact_text()
        .unwrap();

    writeln!(mstp_file, "# {}
", root_story).unwrap();

    let mut step_count = 1;
    loop {
        let step: String = Input::with_theme(&theme)
            .with_prompt(format!("🔹 Step {}: Path to .marks file (e.g., agents/echo/marks/marks.echo)", step_count))
            .interact_text()
            .unwrap();

        writeln!(mstp_file, "{}: {}", step_count, step).unwrap();
        step_count += 1;

        let done: bool = dialoguer::Confirm::with_theme(&theme)
            .with_prompt("Add another path step?")
            .interact()
            .unwrap();

        if !done {
            break;
        }
    }

    println!("\n✅ Path setup written to {}", mstp_path.display());
}