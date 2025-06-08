use std::fs;
use std::process::Command;

#[tokio::test]
async fn test_mark_setup_creates_expected_files() {
    let sandbox = "./sandbox";
    let base = format!("{}/.mark", sandbox);

    let result = Command::new("./target/debug/mark")
        .arg("--setup-tests")
        .arg(sandbox)
        .output()
        .expect("failed to execute MARK");

    assert!(result.status.success(), "MARK setup-tests failed");

    let paths = [
        "agents/agent.marks",
        "agents/test_agent/marks/marks.test_agent",
        "agents/test_agent/marks/markers.test_agent",
        "agents/test_agent/marks/md.test_agent",
        "tools/tool.marks",
        "tools/test_tool/marks/marks.test_tool",
        "tools/test_tool/marks/markers.test_tool",
        "tools/test_tool/marks/md.test_tool",
        "mark.mstp",
        "bookmarks.mstp",
    ];

    for rel_path in paths.iter() {
        let full_path = format!("{}/{}", base, rel_path);
        assert!(
            fs::metadata(&full_path).is_ok(),
            "❌ Missing file: {}",
            full_path
        );
    }

    println!("✅ All expected MARK files exist in sandbox.");
}
