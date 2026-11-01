// Caminho relativo: app/core/tests/document.rs

//! Ida e volta em JSON das fixtures do modelo de documento (MT-6), sem perda.

use std::path::Path;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use teacher_helper_core::document::{Header, rich::RichText};

fn fixture(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/documents")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap()
}

/// Lê o JSON no tipo e grava de volta: o resultado tem de ser igual ao original.
fn assert_round_trip<T: Serialize + DeserializeOwned>(name: &str) {
    let original = fixture(name);
    let parsed: T = serde_json::from_value(original.clone())
        .unwrap_or_else(|e| panic!("{name} não segue o modelo: {e}"));
    assert_eq!(
        serde_json::to_value(&parsed).unwrap(),
        original,
        "{name} perdeu dados"
    );
}

#[test]
fn document_rich_content_round_trips() {
    assert_round_trip::<RichText>("rich-content.json");
}

#[test]
fn document_header_round_trips() {
    assert_round_trip::<Header>("header.json");
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
