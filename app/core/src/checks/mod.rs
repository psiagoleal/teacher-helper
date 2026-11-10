// Caminho relativo: app/core/src/checks/mod.rs

//! Verificações de gabarito por código (spec 05, catálogo CK; spec 03, P-07 e P-10).
//!
//! Cada verificação é uma função pura que devolve achados. Nenhuma bloqueia a edição: o
//! achado só marca a questão. Os achados saem como código + parâmetros (ADR-0006, item 5).
//!
//! CK-02 ("exatamente uma correta") é garantida pelo esquema: o gabarito da múltipla escolha
//! é um único `id`, e duas corretas são recusadas na validação (MT-7). CK-08 (LaTeX e
//! Mermaid) fica no MT-10 e no MT-62.

mod alternatives;
mod answers;
mod points;
mod required;
mod text;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::document::Document;
use crate::document::question::Question;

/// Verificações do catálogo implementadas no núcleo.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum CheckId {
    #[serde(rename = "CK-01")]
    Ck01,
    #[serde(rename = "CK-03")]
    Ck03,
    #[serde(rename = "CK-04")]
    Ck04,
    #[serde(rename = "CK-05")]
    Ck05,
    #[serde(rename = "CK-06")]
    Ck06,
    #[serde(rename = "CK-07")]
    Ck07,
    #[serde(rename = "CK-09")]
    Ck09,
}

impl CheckId {
    pub const ALL: [CheckId; 7] = [
        CheckId::Ck01,
        CheckId::Ck03,
        CheckId::Ck04,
        CheckId::Ck05,
        CheckId::Ck06,
        CheckId::Ck07,
        CheckId::Ck09,
    ];
}

/// Gravidade do achado: erro de gabarito ou só alerta (CK-05, pista no tamanho).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
}

/// Achado de uma verificação.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    pub check: CheckId,
    pub severity: Severity,
    /// Campo apontado, em JSON Pointer (ex.: `/sections/0/questions/2/body/answer`).
    pub path: String,
    /// Questão do achado; ausente nos achados do documento inteiro (CK-06).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    /// Código do problema dentro da verificação (ex.: `answer_missing`).
    pub code: String,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub params: Map<String, Value>,
}

/// Executa todas as verificações.
pub fn run_checks(document: &Document) -> Vec<Finding> {
    run_selected(document, &CheckId::ALL)
}

/// Executa só as verificações pedidas (a skill declara o seu subconjunto, S-06).
pub fn run_selected(document: &Document, checks: &[CheckId]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for check in checks {
        let mut found = match check {
            CheckId::Ck01 => answers::ck01(document),
            CheckId::Ck03 => alternatives::ck03(document),
            CheckId::Ck04 => alternatives::ck04(document),
            CheckId::Ck05 => alternatives::ck05(document),
            CheckId::Ck06 => points::ck06(document),
            CheckId::Ck07 => required::ck07(document),
            CheckId::Ck09 => answers::ck09(document),
        };
        findings.append(&mut found);
    }
    findings
}

/// Questão com o caminho dela no documento.
struct Located<'a> {
    path: String,
    question: &'a Question,
}

fn questions(document: &Document) -> impl Iterator<Item = Located<'_>> {
    document
        .sections
        .iter()
        .enumerate()
        .flat_map(|(s, section)| {
            section
                .questions
                .iter()
                .enumerate()
                .map(move |(q, question)| Located {
                    path: format!("/sections/{s}/questions/{q}"),
                    question,
                })
        })
}

impl Located<'_> {
    fn finding(
        &self,
        check: CheckId,
        severity: Severity,
        suffix: &str,
        code: &str,
        params: Map<String, Value>,
    ) -> Finding {
        Finding {
            check,
            severity,
            path: format!("{}{suffix}", self.path),
            question_id: Some(self.question.id.clone()),
            code: code.to_owned(),
            params,
        }
    }

    fn error(&self, check: CheckId, suffix: &str, code: &str) -> Finding {
        self.finding(check, Severity::Error, suffix, code, Map::new())
    }
}

/// Monta parâmetros a partir de pares chave e valor.
fn params<const N: usize>(pairs: [(&str, Value); N]) -> Map<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
}
