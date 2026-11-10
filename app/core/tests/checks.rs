// Caminho relativo: app/core/tests/checks.rs

//! Verificações de gabarito (MT-9): uma fixture que viola cada verificação, mais uma que passa
//! em todas.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::json;
use teacher_helper_core::checks::{CheckId, Finding, Severity, run_checks, run_selected};
use teacher_helper_core::document::Document;
use teacher_helper_core::document::validate::validate_document;

fn read(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn load(name: &str) -> Document {
    validate_document(&read(&format!("checks/{name}")))
        .unwrap_or_else(|e| panic!("{name} deveria ser válido: {e:?}"))
}

/// Achados da fixture, conferindo que só a verificação esperada disparou.
fn only(name: &str, check: CheckId) -> Vec<Finding> {
    let findings = run_checks(&load(name));
    let fired: BTreeSet<CheckId> = findings.iter().map(|f| f.check).collect();
    assert_eq!(fired, BTreeSet::from([check]), "{name}: {findings:#?}");
    findings
}

/// (caminho, código) de cada achado, na ordem.
fn summary(findings: &[Finding]) -> Vec<(String, String)> {
    findings
        .iter()
        .map(|f| (f.path.clone(), f.code.clone()))
        .collect()
}

fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(p, c)| ((*p).to_owned(), (*c).to_owned()))
        .collect()
}

#[test]
fn checks_pass_on_complete_exam() {
    let text = read("documents/exam-all-question-types.json");
    let findings = run_checks(&validate_document(&text).unwrap());
    assert!(findings.is_empty(), "{findings:#?}");
}

#[test]
fn checks_ck01_answer_missing_or_unknown() {
    let findings = only("ck01-answer-missing-or-unknown.json", CheckId::Ck01);
    assert_eq!(
        summary(&findings),
        pairs(&[
            ("/sections/0/questions/0/body/answer", "answer_unknown_id"),
            (
                "/sections/0/questions/1/body/items/1/answer",
                "answer_missing"
            ),
            ("/sections/0/questions/2/body/answer", "answer_missing"),
            (
                "/sections/0/questions/2/body/answer/0/blank_id",
                "answer_unknown_id"
            ),
            ("/sections/0/questions/3/body/answer", "answer_missing"),
        ])
    );
    assert_eq!(findings[0].question_id.as_deref(), Some("q1"));
    assert_eq!(findings[2].params["blank_id"], json!("b1"));
}

/// CK-02: duas corretas não cabem no esquema; a validação recusa.
#[test]
fn checks_ck02_two_correct_is_rejected_by_schema() {
    let errors = validate_document(&read("checks/ck02-two-correct.json")).unwrap_err();
    assert_eq!(errors[0].path, "/sections/0/questions/0/body/answer");
}

#[test]
fn checks_ck03_duplicate_alternatives() {
    let findings = only("ck03-duplicate-alternatives.json", CheckId::Ck03);
    assert_eq!(
        summary(&findings),
        pairs(&[(
            "/sections/0/questions/0/body/alternatives/1",
            "duplicate_choice"
        )])
    );
    assert_eq!(findings[0].params["duplicate_of"], json!("a"));
}

#[test]
fn checks_ck04_positional_reference() {
    let findings = only("ck04-positional-reference.json", CheckId::Ck04);
    assert_eq!(
        summary(&findings),
        pairs(&[(
            "/sections/0/questions/0/body/alternatives/2",
            "positional_reference"
        )])
    );
}

#[test]
fn checks_ck04_ignores_question_that_does_not_shuffle() {
    let mut document = load("ck04-positional-reference.json");
    document.sections[0].questions[0].shuffleable = false;
    assert!(run_checks(&document).is_empty());
}

#[test]
fn checks_ck05_answer_length_clue_is_warning() {
    let findings = only("ck05-answer-length-clue.json", CheckId::Ck05);
    assert_eq!(
        summary(&findings),
        pairs(&[(
            "/sections/0/questions/0/body/alternatives/2",
            "answer_length_clue"
        )])
    );
    assert_eq!(findings[0].severity, Severity::Warning);
}

#[test]
fn checks_ck06_points_sum_mismatch() {
    let findings = only("ck06-points-sum.json", CheckId::Ck06);
    assert_eq!(findings[0].question_id, None);
    assert_eq!(findings[0].params["sum"], json!(9.5));
    assert_eq!(findings[0].params["target"], json!(10.0));
}

#[test]
fn checks_ck06_tolerates_decimal_rounding() {
    let mut document = load("ck06-points-sum.json");
    document.sections[0].questions[0].points = 0.1;
    document.sections[0].questions[1].points = 0.2;
    document.target_total_points = Some(0.3);
    assert!(run_checks(&document).is_empty());
}

#[test]
fn checks_ck07_required_empty() {
    let findings = only("ck07-required-empty.json", CheckId::Ck07);
    assert_eq!(
        summary(&findings),
        pairs(&[
            ("/sections/0/questions/0/statement", "required_empty"),
            (
                "/sections/0/questions/0/body/alternatives",
                "too_few_choices"
            ),
            ("/sections/0/questions/1/body/criteria", "required_empty"),
            ("/sections/0/questions/2/statement", "no_blanks"),
        ])
    );
}

#[test]
fn checks_ck09_matching_incomplete_and_repeated() {
    let findings = only("ck09-matching-incomplete.json", CheckId::Ck09);
    assert_eq!(
        summary(&findings),
        pairs(&[
            ("/sections/0/questions/0/body/left/1", "match_repeated"),
            ("/sections/0/questions/0/body/left/2", "match_incomplete"),
            ("/sections/0/questions/0/body/right/0", "match_repeated"),
        ])
    );
}

#[test]
fn checks_run_selected_limits_to_requested() {
    let document = load("ck01-answer-missing-or-unknown.json");
    assert!(run_selected(&document, &[CheckId::Ck06, CheckId::Ck09]).is_empty());
    assert!(!run_selected(&document, &[CheckId::Ck01]).is_empty());
}

#[test]
fn checks_finding_serializes_as_code_and_params() {
    let findings = only("ck04-positional-reference.json", CheckId::Ck04);
    assert_eq!(
        serde_json::to_value(&findings[0]).unwrap(),
        json!({
            "check": "CK-04",
            "severity": "error",
            "path": "/sections/0/questions/0/body/alternatives/2",
            "question_id": "q1",
            "code": "positional_reference",
            "params": {"found": "anteriores"}
        })
    );
}
