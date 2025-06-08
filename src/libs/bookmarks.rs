// source: src/libs/bookmarks.rs
// src/libs/bookmarks.rs

use std::fs;
use std::path::Path;

pub async fn run(dir: &str) {
    let path = Path::new(dir).join(".mark/bookmarks.mstp");

    println!("\n🔖 --bookmarks: Reading {}\n", path.display());

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            println!("❌ bookmarks.mstp not found.");
            return;
        }
    };

    let mut lines = content.lines();
    while let Some(line) = lines.next() {
        if line.trim().starts_with("#") {
            println!("\x1b[1;34m{}\x1b[0m", line); // Blue header
        } else if line.trim().starts_with("-") {
            println!("🧭  {}", line.trim().trim_start_matches("-").trim());
        }
    }

    println!();
}