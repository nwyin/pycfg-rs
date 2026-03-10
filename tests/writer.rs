mod common;

use common::{analyze_file, analyze_file_source};
use pycfg_rs::cfg::{self, CfgOptions};

#[test]
fn test_json_roundtrip() {
    let result = analyze_file("tests/test_code/basic_if.py");
    let json = serde_json::to_string(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed["functions"].is_array());
    assert!(parsed["functions"][0]["metrics"]["cyclomatic_complexity"].is_number());
}

#[test]
fn test_dot_wellformed() {
    let result = analyze_file("tests/test_code/basic_if.py");
    let dot = pycfg_rs::writer::write_dot(&result);
    assert!(dot.starts_with("digraph CFG {"));
    assert!(dot.contains("subgraph cluster_"));
    assert!(dot.ends_with("}\n"));
    assert!(dot.contains("->"));
}

#[test]
fn test_text_output_format() {
    let result = analyze_file("tests/test_code/basic_if.py");
    let text = pycfg_rs::writer::write_text(&result);
    assert!(text.contains("def check_sign:"));
    assert!(text.contains("Block 0 (entry):"));
    assert!(text.contains("[L"));
    assert!(text.contains("-> Block"));
    assert!(text.contains("# blocks="));
}

#[test]
fn test_text_multi_function_separator() {
    let source = "def foo():\n    return 1\n\ndef bar():\n    return 2\n";
    let result = analyze_file_source(source);
    let text = pycfg_rs::writer::write_text(&result);
    assert!(text.contains("def foo:"));
    assert!(text.contains("def bar:"));
    let foo_end = text.find("def bar:").unwrap();
    let before_bar = &text[..foo_end];
    assert!(
        before_bar.ends_with("\n\n"),
        "functions should be separated by blank line"
    );
}

#[test]
fn test_text_single_function_no_leading_blank() {
    let source = "def foo():\n    return 1\n";
    let result = analyze_file_source(source);
    let text = pycfg_rs::writer::write_text(&result);
    assert!(
        !text.starts_with('\n'),
        "single function should not start with blank line"
    );
}

#[test]
fn test_json_output_valid() {
    let source = "def foo(x):\n    if x > 0:\n        return 1\n    return 0\n";
    let result = analyze_file_source(source);
    let json = pycfg_rs::writer::write_json(&result);
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON should be valid");
    assert!(parsed["functions"].is_array());
    let funcs = parsed["functions"].as_array().unwrap();
    assert!(!funcs.is_empty());
    assert_eq!(funcs[0]["name"], "foo");
    assert!(
        funcs[0]["metrics"]["cyclomatic_complexity"]
            .as_u64()
            .unwrap()
            >= 2
    );
    assert!(funcs[0]["blocks"].is_array());
    let blocks = funcs[0]["blocks"].as_array().unwrap();
    assert!(blocks.len() >= 2);
}

#[test]
fn test_json_report_output_stable_envelope() {
    let source = "def foo():\n    return 1\n";
    let result = analyze_file_source(source);
    let json = pycfg_rs::writer::write_json_report(&[result]);
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON should be valid");
    assert!(parsed["files"].is_array());
    let files = parsed["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["functions"][0]["name"], "foo");
}

#[test]
fn test_text_report_single_file_omits_header() {
    let source = "def foo():\n    return 1\n";
    let result = analyze_file_source(source);
    let text = pycfg_rs::writer::write_text_report(&[result]);
    assert!(!text.starts_with("# file:"));
    assert!(!text.contains("\n# file:"));
}

#[test]
fn test_dot_entry_exit_mrecord() {
    let source = "def foo():\n    return 1\n";
    let result = analyze_file_source(source);
    let dot = pycfg_rs::writer::write_dot(&result);
    assert!(
        dot.contains("shape=Mrecord"),
        "entry/exit blocks should use Mrecord shape"
    );
}

#[test]
fn test_dot_body_block_record() {
    let source = "def foo(x):\n    if x:\n        return 1\n    return 0\n";
    let result = analyze_file_source(source);
    let dot = pycfg_rs::writer::write_dot(&result);

    assert!(
        dot.contains("foo_0 [shape=Mrecord"),
        "entry block should use Mrecord"
    );
    assert!(
        dot.contains("foo_1 [shape=Mrecord"),
        "exit block should use Mrecord"
    );
    assert!(
        dot.contains("foo_2 [shape=record,"),
        "body blocks should use record (not Mrecord)"
    );
    let mrecord_count = dot.matches("shape=Mrecord").count();
    let record_count = dot.matches("shape=record,").count();
    assert!(mrecord_count >= 2, "need at least 2 Mrecord (entry+exit)");
    assert!(record_count >= 1, "need at least 1 plain record (body)");
}

#[test]
fn test_dot_edge_colors() {
    let source = "def foo(x):\n    while x > 0:\n        if x == 5:\n            break\n        if x == 3:\n            continue\n        x -= 1\n    return x\n";
    let result = analyze_file_source(source);
    let dot = pycfg_rs::writer::write_dot(&result);
    assert!(dot.contains("color=green"), "True edges should be green");
    assert!(dot.contains("color=red"), "False edges should be red");
    assert!(dot.contains("color=purple"), "break edges should be purple");
    assert!(dot.contains("color=cyan"), "continue edges should be cyan");
}

#[test]
fn test_dot_return_edge_color() {
    let source = "def foo():\n    return 1\n";
    let result = analyze_file_source(source);
    let dot = pycfg_rs::writer::write_dot(&result);
    assert!(dot.contains("color=blue"), "return edges should be blue");
}

#[test]
fn test_dot_exception_edge_color() {
    let source = "def foo():\n    raise ValueError()\n";
    let result = analyze_file_source(source);
    let dot = pycfg_rs::writer::write_dot(&result);
    assert!(dot.contains("color=orange"), "raise edges should be orange");
}

#[test]
fn test_dot_report_multi_file_single_graph() {
    let foo = analyze_file_source("def foo():\n    return 1\n");
    let bar = cfg::build_cfgs(
        "def bar():\n    return 2\n",
        "other.py",
        &CfgOptions::default(),
    );
    let dot = pycfg_rs::writer::write_dot_report(&[foo, bar]);
    assert_eq!(dot.matches("digraph CFG {").count(), 1);
    assert!(dot.contains("subgraph cluster_file_0"));
    assert!(dot.contains("subgraph cluster_file_1"));
}

#[test]
fn test_dot_report_single_file_omits_file_cluster() {
    let foo = analyze_file_source("def foo():\n    return 1\n");
    let dot = pycfg_rs::writer::write_dot_report(&[foo]);
    assert!(dot.starts_with("digraph CFG {"));
    assert!(!dot.contains("subgraph cluster_file_0"));
}

#[test]
fn test_dot_report_escapes_special_labels() {
    let source = "def foo():\n    value = \"a{b}|c\"\n    return value\n";
    let weird_file = "odd\"name{file}|test.py";
    let result = cfg::build_cfgs(source, weird_file, &CfgOptions::default());
    let other = cfg::build_cfgs(
        "def bar():\n    return 2\n",
        "plain.py",
        &CfgOptions::default(),
    );
    let dot = pycfg_rs::writer::write_dot_report(&[result, other]);
    assert!(dot.contains("label=\"odd\\\"name\\{file\\}\\|test.py\""));
    assert!(dot.contains("[L2] value = \\\"a\\{b\\}\\|c\\\""));
}

#[test]
fn test_write_dot_function_writes_output() {
    let result = analyze_file_source("def foo():\n    return 1\n");
    let mut out = String::new();
    pycfg_rs::writer::write_dot_function(&mut out, &result.functions[0]);
    assert!(out.contains("subgraph cluster_foo"));
    assert!(out.contains("foo_0"));
    assert!(out.contains("foo_1"));
}
