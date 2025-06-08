// source: src/libs/learn.rs
// src/libs/learn.rs

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub async fn run(dir: &str) {
    println!("\n🧠 --learn: Scanning and caching structure of `{}`", dir);

    let root = Path::new(dir);
    let learn_path = root.join(".mark/cache/learned.md");
    let mut buffer = BufWriter::new(File::create(&learn_path).unwrap());

    writeln!(buffer, "# Learned Project Structure\n").unwrap();
    writeln!(buffer, "Root: `{}`\n", dir).unwrap();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        let display = path.strip_prefix(root).unwrap();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        writeln!(buffer, "- [{}]({})", ext, display.display()).unwrap();
    }

    writeln!(
        buffer,
        "\n✅ Learning complete. Structure saved to `{}`\n",
        learn_path.display()
    )
    .unwrap();

    println!("📘 Learned structure written to .mark/cache/learned.md");
}