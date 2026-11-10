// Caminho relativo: app/core/tests/migrate.rs

//! Migração entre versões do esquema (MT-8, T-12), com fixtures v0 sintética, v1 e v99.

use std::path::Path;

use serde_json::{Value, json};
use teacher_helper_core::SCHEMA_VERSION;
use teacher_helper_core::document::migrate::{Migration, MigrationError, migrate, migrate_with};
use teacher_helper_core::document::validate::validate_document;

fn version_fixture(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/versions")
        .join(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// Passo sintético v0 → v1: a v0 não tinha seções, só uma lista de questões.
fn v0_to_v1(mut document: Value) -> Result<Value, MigrationError> {
    let object = document.as_object_mut().unwrap();
    let questions = object
        .remove("questions")
        .ok_or(MigrationError::StepFailed {
            from: 0,
            reason: "missing questions".into(),
        })?;
    object.insert(
        "sections".into(),
        json!([{ "id": "s1", "questions": questions }]),
    );
    Ok(document)
}

const SYNTHETIC: &[Migration] = &[Migration {
    from: 0,
    apply: v0_to_v1,
}];

#[test]
fn migrate_current_version_is_unchanged() {
    let v1 = version_fixture("v1.json");
    assert_eq!(migrate(&v1).unwrap(), v1);
}

#[test]
fn migrate_old_version_reaches_current_and_validates() {
    let v0 = version_fixture("v0-synthetic.json");
    let migrated = migrate_with(&v0, SYNTHETIC, SCHEMA_VERSION).unwrap();
    assert_eq!(migrated["schema_version"], json!(SCHEMA_VERSION));
    assert_eq!(migrated["sections"][0]["questions"][0]["id"], json!("q1"));
    let document = validate_document(&migrated.to_string()).expect("migrado deve validar");
    assert_eq!(document.sections[0].questions.len(), 1);
}

#[test]
fn migrate_does_not_change_input() {
    let v0 = version_fixture("v0-synthetic.json");
    let before = v0.clone();
    migrate_with(&v0, SYNTHETIC, SCHEMA_VERSION).unwrap();
    assert_eq!(v0, before);
}

#[test]
fn migrate_rejects_newer_version_with_reason() {
    let v99 = version_fixture("v99.json");
    let before = v99.clone();
    let error = migrate(&v99).unwrap_err();
    assert_eq!(
        error,
        MigrationError::NewerVersion {
            found: 99,
            supported: SCHEMA_VERSION
        }
    );
    assert_eq!(v99, before);
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({"code": "newer_version", "found": 99, "supported": SCHEMA_VERSION})
    );
}

#[test]
fn migrate_rejects_missing_or_invalid_version() {
    for bad in [
        json!({}),
        json!({"schema_version": "1"}),
        json!({"schema_version": -1}),
    ] {
        assert_eq!(
            migrate(&bad).unwrap_err(),
            MigrationError::MissingVersion,
            "{bad}"
        );
    }
}

#[test]
fn migrate_reports_gap_in_steps() {
    let v0 = version_fixture("v0-synthetic.json");
    assert_eq!(
        migrate(&v0).unwrap_err(),
        MigrationError::NoMigration { from: 0 }
    );
}

#[test]
fn migrate_reports_failed_step() {
    let mut v0 = version_fixture("v0-synthetic.json");
    v0.as_object_mut().unwrap().remove("questions");
    assert!(matches!(
        migrate_with(&v0, SYNTHETIC, SCHEMA_VERSION),
        Err(MigrationError::StepFailed { from: 0, .. })
    ));
}
