// source: src/main.rs
// src/main.rs

mod cli;
mod libs;

#[tokio::main]
async fn main() {
    let matches = cli::build().get_matches();

    if let Some(dir) = matches.get_one::<String>("setup") {
        libs::setup::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("marker") {
        libs::marker::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("bookmarks") {
        libs::bookmarks::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("marks") {
        libs::marks::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("agents") {
        libs::agents::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("tools") {
        libs::tools::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("scan") {
        libs::scan::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("config") {
        libs::config::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("test") {
        libs::test::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("remark") {
        libs::remark::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("clean") {
        libs::clean::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("clean-mark") {
        libs::clean_mark::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("chat") {
        libs::chat::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("setup-tests") {
        libs::setup_tests::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("setup-paths") {
        libs::setup_paths::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("setup-marks") {
        libs::setup_marks::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("setup-markers") {
        libs::setup_markers::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("setup-tools") {
        libs::setup_tools::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("fix-src") {
        libs::fix_src::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("fix-mark") {
        libs::fix::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("learn") {
        libs::learn::run(dir).await;
    } else if let Some(dir) = matches.get_one::<String>("train") {
        libs::train::run(dir).await;
    } else {
        eprintln!("❌ No valid flag provided. Use --help for options.");
    }
}