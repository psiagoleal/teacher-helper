// Caminho relativo: app/core/tests/latex.rs

//! CK-08, parte LaTeX (MT-10): as fórmulas do spike PRE-5 convertem, as inválidas da fixture
//! não, e o achado aponta o campo `latex` de cada fórmula no documento.

use std::path::Path;

use serde_json::json;
use teacher_helper_core::checks::{CheckId, LatexChecker, Severity, run_checks, run_selected};
use teacher_helper_core::document::validate::validate_document;

fn read(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.trim().is_empty()).collect()
}

#[test]
fn latex_spike_samples_convert() {
    let text = read("../../docs/spikes/pre-5/samples.txt");
    let checker = LatexChecker::default();
    let samples = lines(&text);
    assert_eq!(samples.len(), 12);
    for latex in samples {
        for block in [false, true] {
            assert_eq!(checker.check(latex, block), Ok(()), "{latex}");
        }
    }
}

#[test]
fn latex_invalid_fixtures_fail() {
    let text = read("tests/fixtures/latex/invalid.txt");
    let checker = LatexChecker::default();
    for latex in lines(&text) {
        let issue = checker
            .check(latex, false)
            .expect_err(&format!("deveria falhar: {latex}"));
        assert!(!issue.detail.is_empty(), "{latex}");
        assert!(
            issue.position <= latex.chars().count(),
            "{latex}: {issue:?}"
        );
    }
}

#[test]
fn latex_position_counts_characters() {
    // O erro está no fim da entrada: 22 caracteres, mas 24 bytes ("ç" e "ã" ocupam 2).
    let latex = r"\text{ação} + \frac{1}";
    let issue = LatexChecker::default().check(latex, false).unwrap_err();
    assert_eq!((latex.chars().count(), latex.len()), (22, 24));
    assert_eq!(issue.position, 22, "{issue:?}");
}

#[test]
fn latex_findings_point_to_each_formula() {
    let text = read("tests/fixtures/latex/document-invalid.json");
    let document = validate_document(&text).expect("fixture válida no esquema");
    let findings = run_selected(&document, &[CheckId::Ck08]);
    let got: Vec<(&str, Option<&str>, &serde_json::Value)> = findings
        .iter()
        .map(|f| {
            (
                f.path.as_str(),
                f.question_id.as_deref(),
                &f.params["latex"],
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            (
                "/sections/0/instructions/0/latex",
                None,
                &json!(r"\frac{1}")
            ),
            (
                "/sections/0/questions/0/statement/0/inlines/1/latex",
                Some("q1"),
                &json!("x^{2"),
            ),
            (
                "/sections/0/questions/0/body/alternatives/1/content/0/inlines/0/latex",
                Some("q1"),
                &json!(r"\left( 3"),
            ),
            (
                "/sections/0/questions/1/statement/0/rows/0/1/0/latex",
                Some("q2"),
                &json!("x^2^3"),
            ),
            (
                "/sections/0/questions/1/body/expected_answer/0/latex",
                Some("q2"),
                &json!(r"\comandoinexistente{x}"),
            ),
        ]
    );
    for f in &findings {
        assert_eq!(f.check, CheckId::Ck08);
        assert_eq!(f.severity, Severity::Error);
        assert_eq!(f.code, "latex_invalid");
        assert!(f.params["position"].is_u64());
        assert!(f.params["detail"].as_str().is_some_and(|d| !d.is_empty()));
    }
}

#[test]
fn latex_check_passes_on_complete_exam() {
    let text = read("tests/fixtures/documents/exam-all-question-types.json");
    let document = validate_document(&text).unwrap();
    assert!(run_selected(&document, &[CheckId::Ck08]).is_empty());
    assert!(run_checks(&document).is_empty());
}
