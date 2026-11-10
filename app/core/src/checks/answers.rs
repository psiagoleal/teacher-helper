// Caminho relativo: app/core/src/checks/answers.rs

//! CK-01 (gabarito existe e aponta para algo que existe) e CK-09 (associação completa e sem
//! repetição).

use std::collections::{BTreeMap, HashSet};

use serde_json::json;

use super::{CheckId, Finding, Severity, params, questions, text};
use crate::document::Document;
use crate::document::question::{Choice, QuestionBody};

fn ids(choices: &[Choice]) -> HashSet<&str> {
    choices.iter().map(|c| c.id.as_str()).collect()
}

pub fn ck01(document: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for at in questions(document) {
        let unknown = |suffix: String, id: &str| {
            at.finding(
                CheckId::Ck01,
                Severity::Error,
                &suffix,
                "answer_unknown_id",
                params([("id", json!(id))]),
            )
        };
        match &at.question.body {
            QuestionBody::MultipleChoice(mc) => match &mc.answer {
                None => out.push(at.error(CheckId::Ck01, "/body/answer", "answer_missing")),
                Some(id) if !ids(&mc.alternatives).contains(id.as_str()) => {
                    out.push(unknown("/body/answer".into(), id));
                }
                Some(_) => {}
            },
            QuestionBody::TrueFalse(tf) => {
                for (i, item) in tf.items.iter().enumerate() {
                    if item.answer.is_none() {
                        let suffix = format!("/body/items/{i}/answer");
                        out.push(at.error(CheckId::Ck01, &suffix, "answer_missing"));
                    }
                }
            }
            QuestionBody::Matching(m) => {
                if m.answer.is_empty() {
                    out.push(at.error(CheckId::Ck01, "/body/answer", "answer_missing"));
                }
                let (left, right) = (ids(&m.left), ids(&m.right));
                for (k, pair) in m.answer.iter().enumerate() {
                    if !left.contains(pair.left.as_str()) {
                        out.push(unknown(format!("/body/answer/{k}/left"), &pair.left));
                    }
                    if !right.contains(pair.right.as_str()) {
                        out.push(unknown(format!("/body/answer/{k}/right"), &pair.right));
                    }
                }
            }
            QuestionBody::FillInBlanks(fb) => {
                let blanks = text::blank_ids(&at.question.statement);
                for blank in &blanks {
                    let answered = fb.answer.iter().any(|a| {
                        a.blank_id == *blank && a.accepted.iter().any(|s| !s.trim().is_empty())
                    });
                    if !answered {
                        out.push(at.finding(
                            CheckId::Ck01,
                            Severity::Error,
                            "/body/answer",
                            "answer_missing",
                            params([("blank_id", json!(blank))]),
                        ));
                    }
                }
                for (k, answer) in fb.answer.iter().enumerate() {
                    if !blanks.contains(&answer.blank_id.as_str()) {
                        out.push(unknown(
                            format!("/body/answer/{k}/blank_id"),
                            &answer.blank_id,
                        ));
                    }
                }
            }
            QuestionBody::ShortAnswer(_) | QuestionBody::LongAnswer(_) => {}
        }
    }
    out
}

pub fn ck09(document: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for at in questions(document) {
        let QuestionBody::Matching(m) = &at.question.body else {
            continue;
        };
        // Sem nenhum par, o CK-01 já acusa; não repetir um achado por item.
        if m.answer.is_empty() {
            continue;
        }
        let mut left_uses: BTreeMap<&str, usize> = BTreeMap::new();
        let mut right_uses: BTreeMap<&str, usize> = BTreeMap::new();
        for pair in &m.answer {
            *left_uses.entry(pair.left.as_str()).or_default() += 1;
            *right_uses.entry(pair.right.as_str()).or_default() += 1;
        }
        for (i, item) in m.left.iter().enumerate() {
            let uses = left_uses.get(item.id.as_str()).copied().unwrap_or(0);
            let code = match uses {
                0 => "match_incomplete",
                1 => continue,
                _ => "match_repeated",
            };
            out.push(at.finding(
                CheckId::Ck09,
                Severity::Error,
                &format!("/body/left/{i}"),
                code,
                params([("id", json!(item.id)), ("uses", json!(uses))]),
            ));
        }
        for (i, item) in m.right.iter().enumerate() {
            let uses = right_uses.get(item.id.as_str()).copied().unwrap_or(0);
            if uses > 1 {
                out.push(at.finding(
                    CheckId::Ck09,
                    Severity::Error,
                    &format!("/body/right/{i}"),
                    "match_repeated",
                    params([("id", json!(item.id)), ("uses", json!(uses))]),
                ));
            }
        }
    }
    out
}
