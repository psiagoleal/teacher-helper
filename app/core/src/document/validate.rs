// Caminho relativo: app/core/src/document/validate.rs

//! Validação do documento contra o esquema (P-01), com erro apontado pelo caminho do campo.
//!
//! Os erros saem como código + parâmetros (ADR-0006, item 5): a casca escreve a frase para o
//! professor, e o ciclo de resposta (MT-23) monta o retorno para a IA (P-02).
//! Fora daqui: verificações pedagógicas CK (MT-9) e migração de versão (MT-8).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use serde_path_to_error::Segment;

use super::Document;
use super::question::{
    Discursive, FillInBlanks, Matching, MultipleChoice, Question, QuestionBody, TrueFalse,
};

/// Erro de validação de um campo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationError {
    /// Caminho do campo em JSON Pointer (ex.: `/sections/0/questions/2/difficulty`).
    pub path: String,
    pub code: ValidationCode,
    /// Detalhe técnico do leitor JSON (em inglês), útil para devolver à IA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCode {
    /// O texto não é JSON.
    InvalidJson,
    /// JSON válido fora do esquema: campo ausente, desconhecido ou de tipo errado.
    InvalidShape,
    /// Número fora da faixa do esquema.
    OutOfRange,
}

/// Lê e valida um documento em JSON. Devolve o documento ou todos os erros encontrados.
///
/// Erros de forma param a leitura, então vem só o primeiro; erros de faixa vêm todos.
pub fn validate_document(json: &str) -> Result<Document, Vec<ValidationError>> {
    let deserializer = &mut serde_json::Deserializer::from_str(json);
    let document: Document = match serde_path_to_error::deserialize(deserializer) {
        Ok(document) => document,
        Err(error) => return Err(vec![refine_body_path(json, shape_error(error))]),
    };
    let errors = range_errors(&document);
    if errors.is_empty() {
        Ok(document)
    } else {
        Err(errors)
    }
}

fn shape_error(error: serde_path_to_error::Error<serde_json::Error>) -> ValidationError {
    let path = pointer(error.path());
    let inner = error.into_inner();
    let code = match inner.classify() {
        serde_json::error::Category::Data => ValidationCode::InvalidShape,
        _ => ValidationCode::InvalidJson,
    };
    ValidationError {
        path,
        code,
        detail: Some(inner.to_string()),
        min: None,
        max: None,
    }
}

/// O corpo da questão é marcado por `type`, e o serde lê esse tipo de enum de uma vez: o
/// caminho do erro para em `/body`. Aqui o corpo é lido de novo pelo tipo declarado, para
/// apontar o campo exato (ex.: `/body/alternatives/1`).
fn refine_body_path(json: &str, error: ValidationError) -> ValidationError {
    if error.code != ValidationCode::InvalidShape || !error.path.ends_with("/body") {
        return error;
    }
    let Ok(root) = serde_json::from_str::<Value>(json) else {
        return error;
    };
    let Some(mut body) = root.pointer(&error.path).cloned() else {
        return error;
    };
    let Some(Value::String(kind)) = body.as_object_mut().and_then(|o| o.remove("type")) else {
        return error;
    };
    let deeper = match kind.as_str() {
        "multiple_choice" => first_error::<MultipleChoice>(body),
        "true_false" => first_error::<TrueFalse>(body),
        "matching" => first_error::<Matching>(body),
        "fill_in_blanks" => first_error::<FillInBlanks>(body),
        "short_answer" | "long_answer" => first_error::<Discursive>(body),
        _ => None,
    };
    match deeper {
        Some((path, detail)) => ValidationError {
            path: format!("{}{path}", error.path),
            detail: Some(detail),
            ..error
        },
        None => error,
    }
}

fn first_error<T: DeserializeOwned>(value: Value) -> Option<(String, String)> {
    serde_path_to_error::deserialize::<_, T>(value)
        .err()
        .map(|e| (pointer(e.path()), e.into_inner().to_string()))
}

/// Converte o caminho do serde em JSON Pointer (RFC 6901).
fn pointer(path: &serde_path_to_error::Path) -> String {
    let mut out = String::new();
    for segment in path.iter() {
        match segment {
            Segment::Seq { index } => out.push_str(&format!("/{index}")),
            Segment::Map { key } => {
                out.push('/');
                out.push_str(&key.replace('~', "~0").replace('/', "~1"));
            }
            Segment::Enum { .. } | Segment::Unknown => {}
        }
    }
    out
}

fn range_errors(document: &Document) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    if let Some(total) = document.target_total_points {
        check_min(&mut errors, "/target_total_points".into(), total, 0.0);
    }
    for (s, section) in document.sections.iter().enumerate() {
        for (q, question) in section.questions.iter().enumerate() {
            question_errors(
                &mut errors,
                &format!("/sections/{s}/questions/{q}"),
                question,
            );
        }
    }
    errors
}

fn question_errors(errors: &mut Vec<ValidationError>, base: &str, question: &Question) {
    check_min(errors, format!("{base}/points"), question.points, 0.0);
    if !(1..=3).contains(&question.difficulty) {
        errors.push(out_of_range(
            format!("{base}/difficulty"),
            Some(1.0),
            Some(3.0),
        ));
    }
    if let QuestionBody::ShortAnswer(d) | QuestionBody::LongAnswer(d) = &question.body
        && d.lines < 1
    {
        errors.push(out_of_range(format!("{base}/body/lines"), Some(1.0), None));
    }
}

fn check_min(errors: &mut Vec<ValidationError>, path: String, value: f64, min: f64) {
    if value < min {
        errors.push(out_of_range(path, Some(min), None));
    }
}

fn out_of_range(path: String, min: Option<f64>, max: Option<f64>) -> ValidationError {
    ValidationError {
        path,
        code: ValidationCode::OutOfRange,
        detail: None,
        min,
        max,
    }
}
