mod common;

use common::analyze_file;
use pycfg_rs::cfg::{self, CfgOptions};

#[test]
fn test_basic_if_fixture() {
    let result = analyze_file("tests/test_code/basic_if.py");
    assert!(!result.functions.is_empty());
    let func = result
        .functions
        .iter()
        .find(|f| f.name == "check_sign")
        .expect("should find check_sign");
    assert_eq!(func.metrics.cyclomatic_complexity, 3);
}

#[test]
fn test_loops_fixture() {
    let result = analyze_file("tests/test_code/loops.py");
    assert!(result.functions.iter().any(|f| f.name == "my_func"));
    assert!(result.functions.iter().any(|f| f.name == "while_loop"));
    assert!(result.functions.iter().any(|f| f.name == "break_loop"));
}

#[test]
fn test_loops_targeting() {
    let source = std::fs::read_to_string("tests/test_code/loops.py").unwrap();
    let result =
        cfg::build_cfg_for_function(&source, "loops.py", "my_func", &CfgOptions::default());
    assert!(result.is_some());
    let file_cfg = result.unwrap();
    assert_eq!(file_cfg.functions.len(), 1);
    assert_eq!(file_cfg.functions[0].name, "my_func");
}

#[test]
fn test_try_except_fixture() {
    let result = analyze_file("tests/test_code/try_except.py");
    let func = result.functions.iter().find(|f| f.name == "func").unwrap();
    let has_exception = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "exception");
    assert!(has_exception);
    let has_finally = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "finally");
    assert!(has_finally);
}

#[test]
fn test_match_case_fixture() {
    let result = analyze_file("tests/test_code/match_case.py");
    let func = result.functions.iter().find(|f| f.name == "func").unwrap();
    let case_edges: Vec<_> = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.label.starts_with("case "))
        .collect();
    assert_eq!(case_edges.len(), 4);
}

#[test]
fn test_nested_loops_fixture() {
    let result = analyze_file("tests/test_code/nested_loops.py");
    let funcs: Vec<&str> = result.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(
        funcs.contains(&"nested_for_while"),
        "missing nested_for_while"
    );
    assert!(
        funcs.contains(&"nested_while_while"),
        "missing nested_while_while"
    );
    assert!(funcs.contains(&"triple_nested"), "missing triple_nested");
    assert!(
        funcs.contains(&"break_outer_via_flag"),
        "missing break_outer_via_flag"
    );

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "nested_for_while")
        .unwrap();
    let edge_labels: Vec<&str> = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .map(|e| e.label.as_str())
        .collect();
    assert!(edge_labels.contains(&"break"));
    assert!(edge_labels.contains(&"continue"));
}

#[test]
fn test_loop_else_fixture() {
    let result = analyze_file("tests/test_code/loop_else.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "for_else_no_break")
        .unwrap();
    assert!(
        func.blocks.len() >= 4,
        "for-else needs extra block for else body"
    );

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "while_else")
        .unwrap();
    let edge_labels: Vec<&str> = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .map(|e| e.label.as_str())
        .collect();
    assert!(
        edge_labels.contains(&"loop-back"),
        "while-else should have loop-back"
    );

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "for_else_with_break")
        .unwrap();
    let has_break = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "break");
    assert!(has_break);
}

#[test]
fn test_try_complex_fixture() {
    let result = analyze_file("tests/test_code/try_complex.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "multiple_excepts")
        .unwrap();
    let exc_edges = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.label == "exception")
        .count();
    assert_eq!(
        exc_edges, 3,
        "multiple_excepts should have 3 exception edges"
    );

    let has_bare = func
        .blocks
        .iter()
        .any(|b| b.statements.iter().any(|s| s.text == "except:"));
    assert!(has_bare);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "nested_try")
        .unwrap();
    let exc_edges = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.label == "exception")
        .count();
    assert!(exc_edges >= 2);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "try_except_else")
        .unwrap();
    let has_try_else = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "try-else");
    assert!(has_try_else);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "try_except_else_finally")
        .unwrap();
    let all_labels: Vec<&str> = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .map(|e| e.label.as_str())
        .collect();
    assert!(all_labels.contains(&"finally"));
    assert!(all_labels.contains(&"try-else"));
    assert!(all_labels.contains(&"exception"));

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "bare_raise")
        .unwrap();
    let has_raise = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "raise");
    assert!(has_raise);
}

#[test]
fn test_async_constructs_fixture() {
    let result = analyze_file("tests/test_code/async_constructs.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "async_for_loop")
        .unwrap();
    let has_async_for = func
        .blocks
        .iter()
        .any(|b| b.statements.iter().any(|s| s.text.starts_with("async for")));
    assert!(has_async_for);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "async_with_statement")
        .unwrap();
    let has_async_with = func.blocks.iter().any(|b| {
        b.statements
            .iter()
            .any(|s| s.text.starts_with("async with"))
    });
    assert!(has_async_with);
}

#[test]
fn test_generators_fixture() {
    let result = analyze_file("tests/test_code/generators.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "simple_generator")
        .unwrap();
    let yields = func
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .filter(|s| s.text.starts_with("yield"))
        .count();
    assert_eq!(yields, 3);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "yield_from_example")
        .unwrap();
    let yield_froms = func
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .filter(|s| s.text.starts_with("yield from"))
        .count();
    assert_eq!(yield_froms, 2);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "generator_with_return")
        .unwrap();
    let has_return = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .any(|e| e.label == "return");
    assert!(has_return);
}

#[test]
fn test_straight_line_fixture() {
    let result = analyze_file("tests/test_code/straight_line.py");

    for func_name in &[
        "straight_line",
        "single_statement",
        "pass_only",
        "assignments_only",
    ] {
        let func = result
            .functions
            .iter()
            .find(|f| f.name == *func_name)
            .unwrap_or_else(|| panic!("missing {}", func_name));
        assert_eq!(
            func.metrics.cyclomatic_complexity, 1,
            "{} should have complexity 1, got {}",
            func_name, func.metrics.cyclomatic_complexity
        );
        assert_eq!(
            func.metrics.branches, 0,
            "{} should have 0 branches",
            func_name
        );
    }
}

#[test]
fn test_complex_nesting_fixture() {
    let result = analyze_file("tests/test_code/complex_nesting.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "if_in_loop_in_try")
        .unwrap();
    assert!(func.metrics.cyclomatic_complexity >= 3);

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "deeply_nested_returns")
        .unwrap();
    let exit_id = func.blocks.iter().find(|b| b.label == "exit").unwrap().id;
    let return_edges = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.target == exit_id && e.label == "return")
        .count();
    assert!(
        return_edges >= 3,
        "deeply_nested_returns should have >= 3 return edges"
    );
}

#[test]
fn test_multiple_returns_fixture() {
    let result = analyze_file("tests/test_code/multiple_returns.py");

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "guard_clauses")
        .unwrap();
    let exit_id = func.blocks.iter().find(|b| b.label == "exit").unwrap().id;
    let return_edges = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.target == exit_id && e.label == "return")
        .count();
    assert_eq!(
        return_edges, 4,
        "guard_clauses: 3 guard returns + 1 final return"
    );

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "return_in_branches")
        .unwrap();
    let exit_id = func.blocks.iter().find(|b| b.label == "exit").unwrap().id;
    let return_edges = func
        .blocks
        .iter()
        .flat_map(|b| &b.successors)
        .filter(|e| e.target == exit_id && e.label == "return")
        .count();
    assert_eq!(return_edges, 3, "return_in_branches has 3 returns");
}

#[test]
fn test_classes_fixture() {
    let result = analyze_file("tests/test_code/classes.py");

    let names: Vec<&str> = result.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"Simple.__init__"));
    assert!(names.contains(&"Simple.get_x"));
    assert!(names.contains(&"WithClassMethod.create"));
    assert!(names.contains(&"WithClassMethod.validate"));
    assert!(names.contains(&"Nested.Inner.inner_method"));
    assert!(names.contains(&"Nested.outer_method"));
    assert!(names.contains(&"WithProperties.__init__"));
    assert!(names.contains(&"WithProperties.value"));

    let func = result
        .functions
        .iter()
        .find(|f| f.name == "WithClassMethod.validate")
        .unwrap();
    assert!(func.metrics.cyclomatic_complexity >= 2);
}

#[test]
fn test_all_fixtures_json_roundtrip() {
    let fixtures = [
        "tests/test_code/basic_if.py",
        "tests/test_code/loops.py",
        "tests/test_code/try_except.py",
        "tests/test_code/match_case.py",
        "tests/test_code/nested_loops.py",
        "tests/test_code/loop_else.py",
        "tests/test_code/try_complex.py",
        "tests/test_code/async_constructs.py",
        "tests/test_code/generators.py",
        "tests/test_code/straight_line.py",
        "tests/test_code/complex_nesting.py",
        "tests/test_code/multiple_returns.py",
        "tests/test_code/classes.py",
    ];
    for fixture in &fixtures {
        let result = analyze_file(fixture);
        for func in &result.functions {
            let json = serde_json::to_string(func).unwrap();
            let _: serde_json::Value = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("invalid JSON for {} in {}: {}", func.name, fixture, e));
            assert!(
                func.metrics.cyclomatic_complexity >= 1,
                "cc < 1 for {} in {}",
                func.name,
                fixture
            );
            assert!(
                func.blocks.len() >= 2,
                "< 2 blocks for {} in {}",
                func.name,
                fixture
            );
        }
    }
}

#[test]
fn test_all_fixtures_dot_output() {
    let fixtures = [
        "tests/test_code/basic_if.py",
        "tests/test_code/nested_loops.py",
        "tests/test_code/try_complex.py",
        "tests/test_code/complex_nesting.py",
    ];
    for fixture in &fixtures {
        let result = analyze_file(fixture);
        let dot = pycfg_rs::writer::write_dot(&result);
        assert!(
            dot.starts_with("digraph CFG {"),
            "bad DOT start for {}",
            fixture
        );
        assert!(dot.ends_with("}\n"), "bad DOT end for {}", fixture);
        assert!(dot.contains("->"), "no edges in DOT for {}", fixture);
    }
}
