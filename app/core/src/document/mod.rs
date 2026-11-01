// Caminho relativo: app/core/src/document/mod.rs

//! Modelo do documento (spec 03): cabeçalho e conteúdo rico.
//!
//! Documento, seção e tipos de questão entram no restante do MT-6.

pub mod rich;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Cabeçalho do documento: cópia dos campos do modelo escolhido (P-17).
///
/// É uma cópia, não uma referência: alterar o modelo não muda documentos já existentes.
/// `template_id` só lembra a origem, para o professor reaplicar o modelo se pedir.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Header {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Campos na ordem em que aparecem.
    pub fields: Vec<HeaderField>,
}

/// Campo do cabeçalho.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HeaderField {
    pub kind: HeaderFieldKind,
    /// Rótulo exibido (ex.: "Turma:").
    pub label: String,
    /// Valor fixo; ausente quando o campo é preenchido à mão no papel (aluno, nota).
    /// No logotipo, é o identificador da imagem guardada.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Tipos de campo de cabeçalho (spec 03, "Cabeçalho").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HeaderFieldKind {
    School,
    Logo,
    Subject,
    Teacher,
    Class,
    Student,
    Date,
    Points,
    Grade,
    Instructions,
    FreeText,
}
