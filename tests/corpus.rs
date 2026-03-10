mod common;

use common::{analyze_corpus, corpus_dir};

#[test]
fn test_corpus_requests() {
    if let Some(dir) = corpus_dir("requests") {
        let (files, functions) = analyze_corpus(&dir);
        eprintln!("requests: {} files, {} functions", files, functions);
        assert!(files > 10, "expected >10 files, got {}", files);
        assert!(functions > 50, "expected >50 functions, got {}", functions);
    } else {
        eprintln!("Skipping requests corpus (not found). Run ./scripts/bootstrap-corpora.sh");
    }
}

#[test]
fn test_corpus_flask() {
    if let Some(dir) = corpus_dir("flask") {
        let (files, functions) = analyze_corpus(&dir);
        eprintln!("flask: {} files, {} functions", files, functions);
        assert!(files > 10, "expected >10 files, got {}", files);
        assert!(functions > 50, "expected >50 functions, got {}", functions);
    } else {
        eprintln!("Skipping flask corpus (not found). Run ./scripts/bootstrap-corpora.sh");
    }
}

#[test]
fn test_corpus_rich() {
    if let Some(dir) = corpus_dir("rich") {
        let (files, functions) = analyze_corpus(&dir);
        eprintln!("rich: {} files, {} functions", files, functions);
        assert!(files > 20, "expected >20 files, got {}", files);
        assert!(
            functions > 200,
            "expected >200 functions, got {}",
            functions
        );
    } else {
        eprintln!("Skipping rich corpus (not found). Run ./scripts/bootstrap-corpora.sh");
    }
}
