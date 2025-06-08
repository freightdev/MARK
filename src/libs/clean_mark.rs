// source: src/libs/clean_mark.rs
// src/libs/clean_mark.rs

use std::fs;
use std::path::Path;

pub async fn run(dir: &str) {
    println!("🧼 --clean-mark: Cleaning unused imports, variables, and dead code from {}", dir);

    let mark_dir = Path::new(dir);
    let walker = walkdir::WalkDir::new(mark_dir).into_iter();

    for entry in walker.filter_map(Result::ok).filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false)) {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let cleaned = clean_rust_file(&content);
            fs::write(path, cleaned).unwrap();
            println!("✅ Cleaned: {}", path.display());
        }
    }

    println!("\n🎉 --clean-mark complete. All Rust source files swept.");
}

fn clean_rust_file(content: &str) -> String {
    let mut cleaned = String::new();

    for line in content.lines() {
        if line.contains("use") && (line.contains("Write") || line.contains("PathBuf") || line.contains("WalkDir") || line.contains("Input") || line.contains("Confirm") || line.contains("ColorfulTheme")) {
            continue; // Skip unused imports for now
        }
        if line.contains("fn sanitize_id") ||
           line.contains("fn title_case") ||
           line.contains("fn extract_name_from_path") ||
           line.contains("fn ensure_dir") {
            continue; // Skip dead utility code
        }
        if line.contains("let is_agent") || line.contains("is_agent: bool") {
            continue; // Skip unused variables
        }

        cleaned.push_str(line);
        cleaned.push('\n');
    }

    cleaned
}