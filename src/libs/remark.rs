// source: src/libs/remark.rs
// src/libs/remark.rs

use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

pub async fn run(dir: &str) {
    println!("♻️ --remark: Auditing .marks against MSTP...");

    let base = Path::new(dir).join(".mark");
    let remark_path = base.join("cache/remark.patch");
    create_dir_all(remark_path.parent().unwrap()).unwrap();

    let mut patches = String::new();
    let mstp_path = base.join("mark.mstp");

    let Ok(mstp) = fs::read_to_string(&mstp_path) else {
        eprintln!("❌ No mark.mstp found at {}", mstp_path.display());
        return;
    };

    let paths: Vec<PathBuf> = mstp
        .lines()
        .filter_map(|l| l.strip_prefix("- ").map(str::trim))
        .map(|p| base.join(p))
        .collect();

    for index in paths {
        if index.ends_with("agent.marks") {
            patches.push_str(&audit_marks(&index, true));
        } else if index.ends_with("tool.marks") {
            patches.push_str(&audit_marks(&index, false));
        }
    }

    fs::write(&remark_path, patches).unwrap();
    println!("✅ Remark audit complete.\n📄 Patch written to: {}\n", remark_path.display());
}

fn audit_marks(index: &Path, is_agent: bool) -> String {
    let mut out = String::new();

    let Ok(content) = fs::read_to_string(index) else {
        return format!("❌ MISSING INDEX: {}\n", index.display());
    };

    for line in content.lines().filter_map(|l| l.strip_prefix("- ").map(str::trim)) {
        let full = index.parent().unwrap().join(line);
        let dir = full.parent().unwrap();
        let name = dir.file_name().unwrap().to_str().unwrap();

        let files = [
            dir.join(format!("marks.{}", name)),
            dir.join(format!("markers.{}", name)),
            dir.join(format!("md.{}", name)),
        ];

        for file in files {
            if !file.exists() {
                let msg = format!("🔧 PATCH: Missing {}\n", file.display());
                out.push_str(&msg);
            } else if let Ok(meta) = fs::metadata(&file) {
                if meta.len() == 0 {
                    let msg = format!("⚠️  EMPTY: {}\n", file.display());
                    out.push_str(&msg);
                }
            }
        }
    }

    out
}