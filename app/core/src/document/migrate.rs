// Caminho relativo: app/core/src/document/migrate.rs

//! Migração do documento entre versões do esquema (spec 00, T-12).
//!
//! Cada passo leva da versão `from` para `from + 1`. Documento antigo passa por todos os
//! passos até a versão atual; documento de versão mais nova é recusado com motivo. A entrada
//! nunca é alterada: a migração trabalha numa cópia.
//!
//! A validação contra o esquema (MT-7) vem depois da migração.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::SCHEMA_VERSION;

/// Um passo de migração, da versão `from` para `from + 1`.
pub struct Migration {
    pub from: u32,
    pub apply: fn(Value) -> Result<Value, MigrationError>,
}

/// Passos do app. Vazio enquanto só existe a versão 1; o primeiro passo entra junto com a
/// versão 2 do esquema.
pub const MIGRATIONS: &[Migration] = &[];

/// Motivo de recusa, como código + parâmetros (ADR-0006, item 5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum MigrationError {
    /// Falta `schema_version`, ou ela não é um inteiro não negativo.
    MissingVersion,
    /// Documento gravado por versão mais nova do app.
    NewerVersion { found: u32, supported: u32 },
    /// Não há passo que parta desta versão.
    NoMigration { from: u32 },
    /// Um passo não conseguiu converter o documento.
    StepFailed { from: u32, reason: String },
}

/// Migra com os passos do app até a versão atual.
pub fn migrate(document: &Value) -> Result<Value, MigrationError> {
    migrate_with(document, MIGRATIONS, SCHEMA_VERSION)
}

/// Migra com os passos dados até `target`. Separada de [`migrate`] para os testes usarem
/// passos sintéticos.
pub fn migrate_with(
    document: &Value,
    migrations: &[Migration],
    target: u32,
) -> Result<Value, MigrationError> {
    let mut version = schema_version(document)?;
    if version > target {
        return Err(MigrationError::NewerVersion {
            found: version,
            supported: target,
        });
    }
    let mut current = document.clone();
    while version < target {
        let step = migrations
            .iter()
            .find(|m| m.from == version)
            .ok_or(MigrationError::NoMigration { from: version })?;
        current = (step.apply)(current)?;
        version += 1;
        current["schema_version"] = Value::from(version);
    }
    Ok(current)
}

fn schema_version(document: &Value) -> Result<u32, MigrationError> {
    document
        .get("schema_version")
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or(MigrationError::MissingVersion)
}
