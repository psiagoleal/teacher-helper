// Caminho relativo: app/core/tests/document.rs

//! Ida e volta em JSON das fixtures do modelo de documento (MT-6), sem perda.

use std::path::Path;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use teacher_helper_core::SCHEMA_VERSION;
use teacher_helper_core::document::question::{AnswerStatus, Question, QuestionBody};
use teacher_helper_core::document::{Document, Header, document_schema, rich::RichText};

fn fixture(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/documents")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap()
}

/// Números viram `f64` antes de comparar: `1` e `1.0` são o mesmo valor.
fn normalize(value: Value) -> Value {
    match value {
        Value::Number(n) => serde_json::json!(n.as_f64().unwrap()),
        Value::Array(items) => Value::Array(items.into_iter().map(normalize).collect()),
        Value::Object(map) => {
            Value::Object(map.into_iter().map(|(k, v)| (k, normalize(v))).collect())
        }
        other => other,
    }
}

/// Lê o JSON no tipo e grava de volta: o resultado tem de ser igual ao original.
fn assert_round_trip<T: Serialize + DeserializeOwned>(name: &str) -> T {
    let original = fixture(name);
    let parsed: T = serde_json::from_value(original.clone())
        .unwrap_or_else(|e| panic!("{name} não segue o modelo: {e}"));
    assert_eq!(
        normalize(serde_json::to_value(&parsed).unwrap()),
        normalize(original),
        "{name} perdeu dados"
    );
    parsed
}

#[test]
fn document_rich_content_round_trips() {
    let _: RichText = assert_round_trip("rich-content.json");
}

#[test]
fn document_header_round_trips() {
    let _: Header = assert_round_trip("header.json");
}

#[test]
fn document_rich_content_fixture_covers_every_block_type() {
    let types: Vec<String> = fixture("rich-content.json")
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["type"].as_str().unwrap().to_owned())
        .collect();
    for expected in ["paragraph", "math", "code", "image", "table", "diagram"] {
        assert!(
            types.iter().any(|t| t == expected),
            "falta bloco {expected}"
        );
    }
}

#[test]
fn document_schema_is_generated_from_types() {
    let schema = serde_json::to_value(schemars::schema_for!(Header)).unwrap();
    let text = schema.to_string();
    for name in ["HeaderField", "HeaderFieldKind", "free_text"] {
        assert!(text.contains(name), "esquema sem {name}");
    }
    let rich = serde_json::to_value(schemars::schema_for!(RichText))
        .unwrap()
        .to_string();
    for name in ["paragraph", "diagram", "asset_id", "latex"] {
        assert!(rich.contains(name), "esquema sem {name}");
    }
}

fn questions(document: &Document) -> Vec<&Question> {
    document
        .sections
        .iter()
        .flat_map(|s| &s.questions)
        .collect()
}

#[test]
fn document_exam_with_every_question_type_round_trips() {
    let document: Document = assert_round_trip("exam-all-question-types.json");
    assert_eq!(document.schema_version, SCHEMA_VERSION);
    let mut kinds: Vec<&str> = questions(&document)
        .iter()
        .map(|q| match q.body {
            QuestionBody::MultipleChoice(_) => "multiple_choice",
            QuestionBody::TrueFalse(_) => "true_false",
            QuestionBody::Matching(_) => "matching",
            QuestionBody::FillInBlanks(_) => "fill_in_blanks",
            QuestionBody::ShortAnswer(_) => "short_answer",
            QuestionBody::LongAnswer(_) => "long_answer",
        })
        .collect();
    kinds.sort_unstable();
    kinds.dedup();
    assert_eq!(
        kinds.len(),
        6,
        "a fixture precisa ter os 6 tipos: {kinds:?}"
    );
}

#[test]
fn document_minimal_exercise_list_round_trips() {
    let _: Document = assert_round_trip("exercise-list-minimal.json");
}

/// S-08: skill registrada que não existe mais não impede abrir o documento.
#[test]
fn document_with_removed_skill_opens() {
    let document: Document = assert_round_trip("exam-all-question-types.json");
    assert!(
        document
            .produced_by
            .iter()
            .any(|s| s.id == "skill-que-nao-existe-mais")
    );
}

/// P-08: sem estado explícito, o gabarito nasce não conferido.
#[test]
fn document_question_answer_status_defaults_to_unchecked() {
    let mut value = fixture("exam-all-question-types.json");
    let question = &mut value["sections"][0]["questions"][0];
    question.as_object_mut().unwrap().remove("answer_status");
    let question: Question = serde_json::from_value(question.clone()).unwrap();
    assert_eq!(question.answer_status, AnswerStatus::Unchecked);
}

#[test]
fn document_rejects_unknown_fields_in_question_body() {
    let mut value = fixture("exam-all-question-types.json");
    value["sections"][0]["questions"][0]["body"]["extra"] = Value::Bool(true);
    let error = serde_json::from_value::<Document>(value)
        .unwrap_err()
        .to_string();
    assert!(error.contains("unknown field `extra`"), "{error}");
}

#[test]
fn document_schema_lists_every_question_type() {
    let text = document_schema().to_string();
    for name in [
        "multiple_choice",
        "true_false",
        "matching",
        "fill_in_blanks",
        "short_answer",
        "long_answer",
        "produced_by",
        "answer_status",
        "blank",
    ] {
        assert!(text.contains(name), "esquema sem {name}");
    }
}
