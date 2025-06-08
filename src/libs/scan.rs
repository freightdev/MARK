// source: src/libs/scan.rs
// src/libs/scan.rs

use std::fs;
use std::path::{Path, PathBuf};

pub async fn run(dir: &str) {
    println!("🔍 --scan: Validating MARK structure at {}\n", dir);

    let base = Path::new(dir).join(".mark");
    let mstp_path = base.join("mark.mstp");
    let Ok(mstp) = fs::read_to_string(&mstp_path) else {
        eprintln!("❌ Could not read mark.mstp");
        return;
    };

    let paths: Vec<PathBuf> = mstp
        .lines()
        .filter_map(|l| l.strip_prefix("- ").map(str::trim))
        .map(|p| base.join(p))
        .collect();

    let mut issues = 0;

    for path in paths {
        if path.ends_with("agent.marks") {
            issues += check_index(&path, true);
        } else if path.ends_with("tool.marks") {
            issues += check_index(&path, false);
        }
    }

    if issues == 0 {
        println!("\n✅ Structure looks good. All references valid.");
    } else {
        println!("\n❗ Scan complete with {} issue(s).", issues);
    }
}

fn check_index(index_path: &Path, is_agent: bool) -> usize {
    let mut issues = 0;

    let Ok(content) = fs::read_to_string(index_path) else {
        eprintln!("❌ Missing index file: {}", index_path.display());
        return 1;
    };

    for line in content.lines().filter_map(|l| l.strip_prefix("- ").map(str::trim)) {
        let full = index_path.parent().unwrap().join(line);
        let dir = full.parent().unwrap();
        let name = dir.file_name().unwrap().to_str().unwrap();

        let required = [
            dir.join(format!("marks.{}", name)),
            dir.join(format!("markers.{}", name)),
            dir.join(format!("md.{}", name)),
        ];

        for file in &required {
            if !file.exists() {
                println!("❌ Missing: {}", file.display());
                issues += 1;
            }
        }

        // Bonus: warn on empty files
        for file in &required {
            if file.exists() {
                if let Ok(meta) = fs::metadata(file) {
                    if meta.len() == 0 {
                        println!("⚠️  Empty file: {}", file.display());
                    }
                }
            }
        }
    }

    issues
}