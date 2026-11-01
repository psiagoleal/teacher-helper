// Caminho relativo: app/core/src/document/mod.rs

//! Modelo do documento (spec 03): documento, seções, questões, cabeçalho e conteúdo rico.
//!
//! Só a forma e a serialização. Validação (MT-7) e migração (MT-8) vêm depois.

pub mod question;
pub mod rich;
pub mod validate;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use question::Question;
use rich::RichText;

/// Prova ou lista de exercícios.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Document {
    /// Versão do esquema em que o documento foi gravado (`crate::SCHEMA_VERSION`).
    pub schema_version: u32,
    pub kind: DocumentKind,
    pub title: String,
    pub header: Header,
    pub sections: Vec<Section>,
    /// Total de pontos desejado; a soma das questões é conferida contra ele (P-11).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 0))]
    pub target_total_points: Option<f64>,
    /// Skills e versões que produziram ou alteraram o documento (S-08). Só registro: uma
    /// skill que deixou de existir não impede abrir o documento.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produced_by: Vec<SkillRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Exam,
    ExerciseList,
}

/// Seção do documento, com título e instruções opcionais.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<RichText>,
    pub questions: Vec<Question>,
}

/// Skill e versão registradas no documento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SkillRef {
    pub id: String,
    pub version: String,
}

/// JSON Schema do documento, gerado dos tipos (ADR-0006, item 4).
pub fn document_schema() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(Document)).expect("esquema serializa")
}

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
