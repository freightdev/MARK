// source: src/cli.rs
// src/cli.rs

use clap::{Arg, Command};

pub fn build() -> Command {
    Command::new("mark")
        .version("0.1.0")
        .author("Jesse Conley")
        .about("MARK - Markdown Agent Routing Kernel")
        .arg(
            Arg::new("setup")
                .long("setup")
                .value_name("PROJECT_DIR")
                .help("📦 Interactive setup: builds .mstp, .marks, md.[agent/tool]")
                .num_args(1),
        )
        .arg(
            Arg::new("setup-tests")
                .long("setup-tests")
                .value_name("PROJECT_DIR")
                .help("🧪 Fast dev setup for testing full MARK project generation")
                .num_args(1),
        )
        .arg(
            Arg::new("setup-paths")
                .long("setup-paths")
                .value_name("PROJECT_DIR")
                .help("🔧 Internal path scaffolding (used by setup)")
                .num_args(1),
        )
        .arg(
            Arg::new("setup-marks")
                .long("setup-marks")
                .value_name("PROJECT_DIR")
                .help("🧠 Setup agent task marks")
                .num_args(1),
        )
        .arg(
            Arg::new("setup-markers")
                .long("setup-markers")
                .value_name("PROJECT_DIR")
                .help("📍 Setup agent markers")
                .num_args(1),
        )
        .arg(
            Arg::new("setup-tools")
                .long("setup-tools")
                .value_name("PROJECT_DIR")
                .help("🧰 Setup tool index + md")
                .num_args(1),
        )
        .arg(
            Arg::new("marker")
                .long("marker")
                .value_name("PROJECT_DIR")
                .help("🧠 Reads .mstp and rebuilds .marks, md.*, and routing paths")
                .num_args(1),
        )
        .arg(
            Arg::new("bookmarks")
                .long("bookmarks")
                .value_name("PROJECT_DIR")
                .help("🔖 Visualizes bookmarks.mstp as a story path snapshot")
                .num_args(1),
        )
        .arg(
            Arg::new("marks")
                .long("marks")
                .value_name("PROJECT_DIR")
                .help("📄 Lists all .marks with optional metadata")
                .num_args(1),
        )
        .arg(
            Arg::new("agents")
                .long("agents")
                .value_name("PROJECT_DIR")
                .help("🧬 Lists all agents and their md.[agent] files")
                .num_args(1),
        )
        .arg(
            Arg::new("tools")
                .long("tools")
                .value_name("PROJECT_DIR")
                .help("🧰 Lists all tools from tool.marks with md.[tool] insight")
                .num_args(1),
        )
        .arg(
            Arg::new("scan")
                .long("scan")
                .value_name("PROJECT_DIR")
                .help("🔍 Validates .marks and md.* against .mstp reference paths")
                .num_args(1),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("PROJECT_DIR")
                .help("⚙️ Creates or edits .markrc (tool paths, models, cache)")
                .num_args(1),
        )
        .arg(
            Arg::new("test")
                .long("test")
                .value_name("PROJECT_DIR")
                .help("🧪 Dry-run .mstp execution with simulated logs")
                .num_args(1),
        )
        .arg(
            Arg::new("remark")
                .long("remark")
                .value_name("PROJECT_DIR")
                .help("♻️ Audits .marks → generates .remark patch files")
                .num_args(1),
        )
        .arg(
            Arg::new("clean")
                .long("clean")
                .value_name("PROJECT_DIR")
                .help("🧼 Converts messy folders → MARK-ready layout")
                .num_args(1),
        )
        .arg(
            Arg::new("clean-mark")
                .long("clean-mark")
                .value_name("PROJECT_DIR")
                .help("🧼 Clean up .mark-specific paths")
                .num_args(1),
        )
        .arg(
            Arg::new("fix-src")
                .long("fix-src")
                .value_name("PROJECT_DIR")
                .help("🔧 Auto-patch source headers and mod structure")
                .num_args(1),
        )
        .arg(
            Arg::new("fix-mark")
                .long("fix-mark")
                .value_name("PROJECT_DIR")
                .help("🔧 Fix and restore broken .mark paths, agents, tools")
                .num_args(1),
        )
        .arg(
            Arg::new("chat")
                .long("chat")
                .value_name("PROJECT_DIR")
                .help("💬 Interact with agents in markdown REPL mode")
                .num_args(1),
        )
        .arg(
            Arg::new("learn")
                .long("learn")
                .value_name("PROJECT_DIR")
                .help("🧠 Scan and cache structure of a project in markdown")
                .num_args(1),
        )
        .arg(
            Arg::new("train")
                .long("train")
                .value_name("PROJECT_DIR")
                .help("📚 Train on markdown-defined agents and tasks")
                .num_args(1),
        )
}