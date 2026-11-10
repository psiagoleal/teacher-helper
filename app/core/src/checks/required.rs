// Caminho relativo: app/core/src/checks/required.rs

//! CK-07: campos obrigatórios não vazios (enunciado, alternativas, itens, colunas, lacunas e
//! critério de discursiva).

use serde_json::json;

use super::{CheckId, Finding, Located, Severity, params, questions, text};
use crate::document::Document;
use crate::document::question::{Choice, QuestionBody};

pub fn ck07(document: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for at in questions(document) {
        let q = at.question;
        if text::is_empty(&q.statement) {
            out.push(at.error(CheckId::Ck07, "/statement", "required_empty"));
        }
        match &q.body {
            QuestionBody::MultipleChoice(mc) => {
                choices(&at, &mut out, "/body/alternatives", &mc.alternatives, 2);
            }
            QuestionBody::TrueFalse(tf) => {
                too_few(&at, &mut out, "/body/items", tf.items.len(), 1);
                for (i, item) in tf.items.iter().enumerate() {
                    if text::is_empty(&item.content) {
                        let suffix = format!("/body/items/{i}");
                        out.push(at.error(CheckId::Ck07, &suffix, "required_empty"));
                    }
                }
            }
            QuestionBody::Matching(m) => {
                choices(&at, &mut out, "/body/left", &m.left, 1);
                choices(&at, &mut out, "/body/right", &m.right, 1);
            }
            QuestionBody::FillInBlanks(_) => {
                if text::blank_ids(&q.statement).is_empty() {
                    out.push(at.error(CheckId::Ck07, "/statement", "no_blanks"));
                }
            }
            QuestionBody::ShortAnswer(d) | QuestionBody::LongAnswer(d) => {
                if text::is_empty(&d.criteria) {
                    out.push(at.error(CheckId::Ck07, "/body/criteria", "required_empty"));
                }
            }
        }
    }
    out
}

fn choices(at: &Located<'_>, out: &mut Vec<Finding>, base: &str, list: &[Choice], min: usize) {
    too_few(at, out, base, list.len(), min);
    for (i, choice) in list.iter().enumerate() {
        if text::is_empty(&choice.content) {
            out.push(at.error(CheckId::Ck07, &format!("{base}/{i}"), "required_empty"));
        }
    }
}

fn too_few(at: &Located<'_>, out: &mut Vec<Finding>, path: &str, count: usize, min: usize) {
    if count < min {
        out.push(at.finding(
            CheckId::Ck07,
            Severity::Error,
            path,
            "too_few_choices",
            params([("count", json!(count)), ("min", json!(min))]),
        ));
    }
}
