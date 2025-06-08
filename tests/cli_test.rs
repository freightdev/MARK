// tests/cli_test.rs

use std::fs;

#[tokio::test]
async fn test_setup_tests_creates_files() {
    let path = "./sandbox";
    let mark_dir = format!("{}/.mark", path);

    let output = std::process::Command::new("./target/debug/mark")
        .arg("--setup-tests")
        .arg(path)
        .output()
        .expect("Failed to run --setup-tests");

    assert!(output.status.success(), "Setup did not succeed.");

    let expected_files = [
        "agents/test_agent/marks/marks.test_agent",
        "agents/test_agent/marks/markers.test_agent",
        "agents/test_agent/marks/md.test_agent",
        "tools/test_tool/marks/marks.test_tool",
        "tools/test_tool/marks/markers.test_tool",
        "tools/test_tool/marks/md.test_tool",
        "tools/tool.marks",
        "agents/agent.marks",
        "mark.mstp",
        "bookmarks.mstp"
    ];

    for rel_path in expected_files {
        let full_path = format!("{}/{}", mark_dir, rel_path);
        assert!(
            fs::metadata(&full_path).is_ok(),
            "Missing file: {}",
            full_path
        );
    }
}
