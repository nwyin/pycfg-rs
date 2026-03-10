mod common;

use common::run_pycfg;

#[test]
fn test_cli_text_output() {
    let output = run_pycfg(&["tests/test_code/basic_if.py"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("def check_sign:"));
    assert!(stdout.contains("Block 0 (entry):"));
}

#[test]
fn test_cli_json_output() {
    let output = run_pycfg(&["--format", "json", "tests/test_code/basic_if.py"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(parsed["files"].is_array());
}

#[test]
fn test_cli_dot_output() {
    let output = run_pycfg(&["--format", "dot", "tests/test_code/basic_if.py"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("digraph CFG {"));
}

#[test]
fn test_cli_function_targeting() {
    let output = run_pycfg(&["--format", "json", "tests/test_code/loops.py::my_func"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let funcs = parsed["files"][0]["functions"].as_array().unwrap();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0]["name"], "my_func");
}

#[test]
fn test_cli_directory_input() {
    let output = run_pycfg(&["--format", "json", "tests/test_code"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let arr = parsed["files"].as_array().unwrap();
    assert!(arr.len() >= 4, "should have >= 4 files, got {}", arr.len());
}

#[test]
fn test_cli_multiple_files_text() {
    let output = run_pycfg(&["tests/test_code/basic_if.py", "tests/test_code/loops.py"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# file: tests/test_code/basic_if.py"));
    assert!(stdout.contains("# file: tests/test_code/loops.py"));
    assert!(stdout.contains("def check_sign:"));
    assert!(stdout.contains("def my_func:"));
}

#[test]
fn test_cli_explicit_exceptions() {
    let output = run_pycfg(&[
        "--format",
        "json",
        "--explicit-exceptions",
        "tests/test_code/try_except.py",
    ]);
    assert!(output.status.success());
}

#[test]
fn test_cli_nonexistent_file() {
    let output = run_pycfg(&["nonexistent_file_xyz.py"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success() || stdout.is_empty() || stderr.contains("Failed"),
        "nonexistent file should produce error or warning"
    );
}

#[test]
fn test_cli_multi_file_text_separator() {
    let output = run_pycfg(&["tests/test_code/basic_if.py", "tests/test_code/loops.py"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.starts_with('\n'),
        "output should not start with blank line"
    );
    assert!(stdout.contains("\n\n# file: tests/test_code/loops.py\n\n"));
}

#[test]
fn test_cli_json_single_vs_multi() {
    let output1 = run_pycfg(&["--format", "json", "tests/test_code/basic_if.py"]);
    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let parsed1: serde_json::Value = serde_json::from_str(&stdout1).unwrap();
    assert!(
        parsed1.is_object(),
        "single file JSON should be an object envelope"
    );
    assert_eq!(parsed1["files"].as_array().unwrap().len(), 1);

    let output2 = run_pycfg(&[
        "--format",
        "json",
        "tests/test_code/basic_if.py",
        "tests/test_code/loops.py",
    ]);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    let parsed2: serde_json::Value = serde_json::from_str(&stdout2).unwrap();
    assert!(
        parsed2.is_object(),
        "multi-file JSON should use the same envelope"
    );
    assert_eq!(parsed2["files"].as_array().unwrap().len(), 2);
}

#[test]
fn test_cli_dot_multiple_files_single_graph() {
    let output = run_pycfg(&[
        "--format",
        "dot",
        "tests/test_code/basic_if.py",
        "tests/test_code/loops.py",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.matches("digraph CFG {").count(), 1);
    assert!(stdout.contains("subgraph cluster_file_0"));
    assert!(stdout.contains("subgraph cluster_file_1"));
}
