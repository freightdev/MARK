// source: src/libs/fix.rs
// src/libs/fix.rs

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub async fn run(dir: &str) {
    println!("🔧 --fix: Auditing source headers and mod structure at {}", dir);
    let root = Path::new(dir).join("src");
    let mut patched = 0;

    for entry in WalkDir::new(&root) {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }

        let mut contents = String::new();
        fs::File::open(&path).unwrap().read_to_string(&mut contents).unwrap();

        let rel = path.strip_prefix(dir).unwrap();
        let comment_path = format!("// source: {}", rel.display());

        let mut lines: Vec<&str> = contents.lines().collect();
        let needs_header = !lines.get(0).map(|l| l.contains("// source:")).unwrap_or(false);

        if needs_header {
            lines.insert(0, &comment_path);
            let new_content = lines.join("\n");
            fs::write(path, new_content).unwrap();
            patched += 1;
            println!("🧼 Patched header: {}", rel.display());
        }
    }

    println!("✅ --fix complete. {} files patched.", patched);
}

