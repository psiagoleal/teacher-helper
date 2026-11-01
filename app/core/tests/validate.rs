// Caminho relativo: app/core/tests/validate.rs

//! Validação do documento com erro apontado pelo caminho do campo (MT-7, P-01).

use std::path::Path;

use serde_json::{Value, json};
use teacher_helper_core::document::validate::{ValidationCode, ValidationError, validate_document};

fn read(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn valid_exam() -> Value {
    serde_json::from_str(&read("documents/exam-all-question-types.json")).unwrap()
}

fn errors_of(value: &Value) -> Vec<ValidationError> {
    validate_document(&value.to_string()).expect_err("deveria ser inválido")
}

fn single(value: &Value) -> ValidationError {
    let mut errors = errors_of(value);
    assert_eq!(errors.len(), 1, "{errors:?}");
    errors.remove(0)
}

#[test]
fn validate_accepts_valid_documents() {
    for name in ["exam-all-question-types.json", "exercise-list-minimal.json"] {
        let text = read(&format!("documents/{name}"));
        assert!(validate_document(&text).is_ok(), "{name}");
    }
}

#[test]
fn validate_reports_text_that_is_not_json() {
    let errors = validate_document(&read("invalid/truncated.json")).unwrap_err();
    assert_eq!(errors[0].code, ValidationCode::InvalidJson);
}

#[test]
fn validate_points_to_missing_field() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][1]
        .as_object_mut()
        .unwrap()
        .remove("statement");
    let error = single(&doc);
    assert_eq!(error.code, ValidationCode::InvalidShape);
    assert_eq!(error.path, "/sections/0/questions/1");
    assert!(error.detail.unwrap().contains("missing field `statement`"));
}

#[test]
fn validate_points_to_unknown_field() {
    let mut doc = valid_exam();
    doc["sections"][1]["notes"] = json!("x");
    let error = single(&doc);
    assert_eq!(error.code, ValidationCode::InvalidShape);
    assert_eq!(error.path, "/sections/1/notes");
}

#[test]
fn validate_points_to_wrong_type() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][2]["difficulty"] = json!("alta");
    let error = single(&doc);
    assert_eq!(error.code, ValidationCode::InvalidShape);
    assert_eq!(error.path, "/sections/0/questions/2/difficulty");
}

#[test]
fn validate_points_to_unknown_question_type() {
    let text = read("invalid/unknown-question-type.json");
    let errors = validate_document(&text).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].path, "/sections/0/questions/0/body/type");
    assert!(
        errors[0]
            .detail
            .as_ref()
            .unwrap()
            .contains("essay_with_drawing")
    );
}

#[test]
fn validate_points_into_question_body() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][0]["body"]["alternatives"][1]
        .as_object_mut()
        .unwrap()
        .remove("content");
    let error = single(&doc);
    assert_eq!(error.path, "/sections/0/questions/0/body/alternatives/1");
    assert!(error.detail.unwrap().contains("content"));
}

#[test]
fn validate_reports_every_value_out_of_range() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][0]["difficulty"] = json!(5);
    doc["sections"][0]["questions"][3]["points"] = json!(-1);
    doc["sections"][1]["questions"][1]["body"]["lines"] = json!(0);
    doc["target_total_points"] = json!(-10);
    let paths: Vec<String> = errors_of(&doc)
        .into_iter()
        .inspect(|e| assert_eq!(e.code, ValidationCode::OutOfRange))
        .map(|e| e.path)
        .collect();
    assert_eq!(
        paths,
        [
            "/target_total_points",
            "/sections/0/questions/0/difficulty",
            "/sections/0/questions/3/points",
            "/sections/1/questions/1/body/lines",
        ]
    );
}

#[test]
fn validate_range_error_carries_limits() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][0]["difficulty"] = json!(0);
    let error = single(&doc);
    assert_eq!((error.min, error.max), (Some(1.0), Some(3.0)));
}

#[test]
fn validate_error_serializes_as_code_and_params() {
    let mut doc = valid_exam();
    doc["sections"][0]["questions"][0]["difficulty"] = json!(4);
    let value = serde_json::to_value(single(&doc)).unwrap();
    assert_eq!(
        value,
        json!({"path": "/sections/0/questions/0/difficulty", "code": "out_of_range", "min": 1.0, "max": 3.0})
    );
}

#[test]
fn validate_points_into_discursive_body() {
    let mut doc = valid_exam();
    doc["sections"][1]["questions"][0]["body"]["lines"] = json!("três");
    let error = single(&doc);
    assert_eq!(error.code, ValidationCode::InvalidShape);
    assert_eq!(error.path, "/sections/1/questions/0/body/lines");
}
