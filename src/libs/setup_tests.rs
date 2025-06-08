// source: src/libs/setup_tests.rs
// src/libs/setup_tests.rs

use std::fs::{self, create_dir_all};
use std::path::Path;
use std::io::Write;

pub async fn run(dir: &str) {
    println!("\n🧪 --setup-test: Initializing quick test MARK project in `{dir}`\n");

    let mark_root = Path::new(dir).join(".mark");
    let agent_dir = mark_root.join("agents/test_agent/marks");
    let tool_dir = mark_root.join("tools/test_tool/marks");

    create_dir_all(&agent_dir).unwrap();
    create_dir_all(&tool_dir).unwrap();
    create_dir_all(mark_root.join("agents/workspace")).unwrap();
    create_dir_all(mark_root.join("agents/notebook")).unwrap();
    create_dir_all(mark_root.join("tools/workspace")).unwrap();
    create_dir_all(mark_root.join("tools/notebook")).unwrap();

    // Agent markdowns
    fs::write(
        agent_dir.join("marks.test_agent"),
        "# Task: Say Hello\n\n### Description\nResponds with a friendly hello.\n",
    )
    .unwrap();

    fs::write(
        agent_dir.join("markers.test_agent"),
        "# Marker: StartTest\n\n### Trigger\nStartup\n\n### Effect\nActivate test agent task\n",
    )
    .unwrap();

    fs::write(
        agent_dir.join("md.test_agent"),
        "# Agent: test_agent\n\n## Capabilities\n- Say Hello\n\n## Ethics\n- Always greet kindly\n",
    )
    .unwrap();

    // Tool markdowns
    fs::write(
        tool_dir.join("marks.test_tool"),
        "# Tool: echo\n\n## Task\nEcho input back to stdout\n",
    )
    .unwrap();

    fs::write(
        tool_dir.join("markers.test_tool"),
        "# Marker: EchoReady\n\n## Trigger\nWhen tool is idle\n\n## Effect\nAllow echo command\n",
    )
    .unwrap();

    fs::write(
        tool_dir.join("md.test_tool"),
        "# Tool: test_tool\n\n## Intent\nEcho user input\n\n## Example Output\nHello, world!\n",
    )
    .unwrap();

    fs::write(
        mark_root.join("tools/tool.marks"),
        "# Tool Index\n- tools/test_tool/marks/marks.test_tool\n",
    )
    .unwrap();

    fs::write(
        mark_root.join("agents/agent.marks"),
        "# Agent Index\n- agents/test_agent/marks/marks.test_agent\n",
    )
    .unwrap();

    fs::write(
        mark_root.join("mark.mstp"),
        "# MARK Setup Path\n- .mark/agents/agent.marks\n- .mark/tools/tool.marks\n",
    )
    .unwrap();

    fs::write(
        mark_root.join("bookmarks.mstp"),
        "# Bookmarks Cache\n- Last Active Agent: test_agent\n- Last Completed Task: test_agent/say_hello\n- Current Story Path: .mark/mark.mstp\n",
    )
    .unwrap();

    println!("✅ Test MARK project created at `{dir}`\n");
}