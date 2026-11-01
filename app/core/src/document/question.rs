// Caminho relativo: app/core/src/document/question.rs

//! Questão e seus seis tipos (spec 03).
//!
//! Alternativas, itens e lacunas têm `id` estável: o gabarito aponta para o `id`, não para a
//! posição, e por isso continua certo quando as versões embaralham a ordem (P-09).
//! As regras de coerência (CK-01 a CK-09) ficam no MT-9; aqui só a forma.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::rich::RichText;

/// Questão de prova ou lista.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Question {
    pub id: String,
    pub statement: RichText,
    /// Tipo da questão, com alternativas, itens e gabarito.
    pub body: QuestionBody,
    /// Pontos; aceita frações (ex.: 0.5). A soma é conferida com arredondamento (MT-11).
    #[schemars(range(min = 0))]
    pub points: f64,
    pub topic: String,
    pub level: Level,
    /// 1 = fácil, 2 = média, 3 = difícil.
    #[schemars(range(min = 1, max = 3))]
    pub difficulty: u8,
    /// Se as versões A, B, … podem embaralhar suas alternativas ou colunas (P-09, P-10).
    pub shuffleable: bool,
    /// Gabarito conferido pelo professor; o que a IA produz nasce `unchecked` (P-08).
    #[serde(default)]
    pub answer_status: AnswerStatus,
}

/// Os seis tipos de questão do MVP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuestionBody {
    /// Múltipla escolha com uma correta.
    MultipleChoice(MultipleChoice),
    /// Verdadeiro ou falso, item a item.
    TrueFalse(TrueFalse),
    /// Associação de colunas.
    Matching(Matching),
    /// Lacunas marcadas no enunciado por `Inline::Blank`.
    FillInBlanks(FillInBlanks),
    /// Discursiva curta.
    ShortAnswer(Discursive),
    /// Discursiva longa.
    LongAnswer(Discursive),
}

/// Alternativa, item ou elemento de coluna, com `id` estável.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub id: String,
    pub content: RichText,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MultipleChoice {
    pub alternatives: Vec<Choice>,
    /// `id` da alternativa correta; ausente enquanto não há gabarito.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrueFalse {
    pub items: Vec<TrueFalseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrueFalseItem {
    pub id: String,
    pub content: RichText,
    /// Verdadeiro ou falso; ausente enquanto não há gabarito.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Matching {
    /// Coluna A, na ordem fixa.
    pub left: Vec<Choice>,
    /// Coluna B, embaralhada nas versões.
    pub right: Vec<Choice>,
    /// Pares corretos.
    #[serde(default)]
    pub answer: Vec<MatchPair>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MatchPair {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FillInBlanks {
    /// Resposta de cada lacuna do enunciado.
    #[serde(default)]
    pub answer: Vec<BlankAnswer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlankAnswer {
    pub blank_id: String,
    /// Respostas aceitas; a primeira vai para o gabarito.
    pub accepted: Vec<String>,
}

/// Discursiva, curta ou longa.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Discursive {
    /// Linhas de espaço para a resposta.
    #[schemars(range(min = 1))]
    pub lines: u32,
    /// Critério de correção, que vai para o gabarito.
    pub criteria: RichText,
    /// Resposta esperada, opcional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_answer: Option<RichText>,
}

/// Etapa e ano ou período da questão.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub stage: Stage,
    /// Texto livre: "9º ano", "2ª série", "3º semestre".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Etapas atendidas: educação básica (BNCC), técnico e superior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    /// Ensino fundamental, anos iniciais.
    ElementaryEarly,
    /// Ensino fundamental, anos finais.
    ElementaryLate,
    HighSchool,
    Technical,
    HigherEducation,
    Other,
}

/// Estado do gabarito da questão.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnswerStatus {
    #[default]
    Unchecked,
    Checked,
}
