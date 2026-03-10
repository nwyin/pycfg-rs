#![allow(dead_code)]

use pycfg_rs::cfg::{self, CfgOptions};
use std::path::Path;

pub(crate) fn analyze_file(path: &str) -> cfg::FileCfg {
    let source = std::fs::read_to_string(path).unwrap();
    cfg::build_cfgs(&source, path, &CfgOptions::default())
}

pub(crate) fn analyze_file_source(source: &str) -> cfg::FileCfg {
    cfg::build_cfgs(source, "test.py", &CfgOptions::default())
}

pub(crate) fn run_pycfg(args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_pycfg"))
        .args(args)
        .output()
        .expect("failed to execute pycfg")
}

pub(crate) fn corpus_dir(name: &str) -> Option<String> {
    let path = format!("benchmark/corpora/{}/", name);
    if Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

pub(crate) fn analyze_corpus(dir: &str) -> (usize, usize) {
    let mut total_functions = 0;
    let mut total_files = 0;
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "py")
                && !e.path().to_string_lossy().contains("__pycache__")
        })
    {
        let path = entry.path().to_string_lossy().to_string();
        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let result = cfg::build_cfgs(&source, &path, &CfgOptions::default());
        total_functions += result.functions.len();
        total_files += 1;

        for func in &result.functions {
            assert!(
                func.metrics.cyclomatic_complexity >= 1,
                "cyclomatic complexity < 1 for {} in {}",
                func.name,
                path
            );
            assert!(
                func.blocks.len() >= 2,
                "fewer than 2 blocks for {} in {}",
                func.name,
                path
            );

            let json = serde_json::to_string(&func).unwrap();
            let _: serde_json::Value = serde_json::from_str(&json).unwrap();
        }
    }
    (total_files, total_functions)
}
