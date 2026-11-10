// Caminho relativo: app/core/src/checks/alternatives.rs

//! CK-03 (alternativas distintas), CK-04 (sem referência à posição em questão embaralhável,
//! P-10) e CK-05 (alternativa correta longa demais: pista).

use std::collections::HashMap;

use serde_json::json;

use super::{CheckId, Finding, Located, Severity, params, questions, text};
use crate::document::Document;
use crate::document::question::{Choice, QuestionBody};
use crate::document::rich::RichText;

/// Grupos de opções comparáveis de uma questão, com o caminho de cada grupo.
fn choice_groups(body: &QuestionBody) -> Vec<(&'static str, &[Choice])> {
    match body {
        QuestionBody::MultipleChoice(mc) => vec![("/body/alternatives", &mc.alternatives)],
        QuestionBody::Matching(m) => vec![("/body/left", &m.left), ("/body/right", &m.right)],
        _ => Vec::new(),
    }
}

pub fn ck03(document: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for at in questions(document) {
        for (base, choices) in choice_groups(&at.question.body) {
            let mut seen: HashMap<String, &str> = HashMap::new();
            for (i, choice) in choices.iter().enumerate() {
                let key = text::normalized(&choice.content);
                if key.is_empty() {
                    continue; // vazio é CK-07
                }
                if let Some(first) = seen.get(&key) {
                    out.push(at.finding(
                        CheckId::Ck03,
                        Severity::Error,
                        &format!("{base}/{i}"),
                        "duplicate_choice",
                        params([("id", json!(choice.id)), ("duplicate_of", json!(first))]),
                    ));
                } else {
                    seen.insert(key, &choice.id);
                }
            }
        }
    }
    out
}

/// Expressões que só fazem sentido na ordem original das opções.
const POSITIONAL_PHRASES: [&str; 10] = [
    "anteriores",
    "acima",
    "abaixo",
    "a seguir",
    "alternativa anterior",
    "item anterior",
    "opcao anterior",
    "ultima alternativa",
    "primeira alternativa",
    "todas as alternativas",
];

/// Palavras que, seguidas de uma letra de A a E, apontam para uma opção pela posição.
const LETTER_POINTERS: [&str; 8] = [
    "alternativa",
    "alternativas",
    "letra",
    "letras",
    "item",
    "itens",
    "opcao",
    "opcoes",
];

/// Referência posicional encontrada no texto, se houver.
fn positional_reference(content: &RichText) -> Option<String> {
    let normalized = text::normalized(content);
    let words: Vec<&str> = normalized.split_whitespace().collect();
    // Palavras sem a pontuação colada ("anteriores;" vira "anteriores").
    let bare: Vec<&str> = words
        .iter()
        .map(|w| w.trim_matches(|c: char| ".,;:!?".contains(c)))
        .collect();
    let padded = format!(" {} ", bare.join(" "));
    for phrase in POSITIONAL_PHRASES {
        if padded.contains(&format!(" {phrase} ")) {
            return Some(phrase.to_owned());
        }
    }
    let is_letter = |w: &str| {
        let core = w.trim_matches(|c: char| "().,;:".contains(c));
        core.len() == 1 && ('a'..='e').contains(&core.chars().next().unwrap_or(' '))
    };
    for pair in bare.windows(2) {
        if LETTER_POINTERS.contains(&pair[0]) && is_letter(pair[1]) {
            return Some(format!("{} {}", pair[0], pair[1]));
        }
    }
    // "(a)" ou "b)" soltos citam opção pela letra.
    words
        .iter()
        .find(|w| {
            let w = w.trim_end_matches([',', ';', '.']);
            (w.len() == 3 && w.starts_with('(') && w.ends_with(')') && is_letter(w))
                || (w.len() == 2 && w.ends_with(')') && is_letter(w))
        })
        .map(|w| (*w).to_owned())
}

pub fn ck04(document: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for at in questions(document) {
        if !at.question.shuffleable {
            continue;
        }
        let mut contents: Vec<(String, &RichText)> = Vec::new();
        match &at.question.body {
            QuestionBody::TrueFalse(tf) => {
                for (i, item) in tf.items.iter().enumerate() {
                    contents.push((format!("/body/items/{i}"), &item.content));
                }
            }
            body => {
                for (base, choices) in choice_groups(body) {
                    for (i, choice) in choices.iter().enumerate() {
                        contents.push((format!("{base}/{i}"), &choice.content));
                    }
                }
            }
        }
        for (suffix, content) in contents {
            if let Some(found) = positional_reference(content) {
                out.push(at.finding(
                    CheckId::Ck04,
                    Severity::Error,
                    &suffix,
                    "positional_reference",
                    params([("found", json!(found))]),
                ));
            }
        }
    }
    out
}

/// Razão a partir da qual a correta vira pista (catálogo CK-05).
const CLUE_RATIO: f64 = 1.5;

pub fn ck05(document: &Document) -> Vec<Finding> {
    questions(document).filter_map(|at| clue(&at)).collect()
}

fn clue(at: &Located<'_>) -> Option<Finding> {
    let QuestionBody::MultipleChoice(mc) = &at.question.body else {
        return None;
    };
    let answer = mc.answer.as_deref()?;
    let index = mc.alternatives.iter().position(|c| c.id == answer)?;
    let length = |c: &Choice| text::normalized(&c.content).chars().count() as f64;
    let others: Vec<f64> = mc
        .alternatives
        .iter()
        .filter(|c| c.id != answer)
        .map(length)
        .collect();
    if others.is_empty() {
        return None;
    }
    let mean = others.iter().sum::<f64>() / others.len() as f64;
    let correct = length(&mc.alternatives[index]);
    (mean > 0.0 && correct > CLUE_RATIO * mean).then(|| {
        at.finding(
            CheckId::Ck05,
            Severity::Warning,
            &format!("/body/alternatives/{index}"),
            "answer_length_clue",
            params([
                ("length", json!(correct)),
                ("others_mean", json!((mean * 10.0).round() / 10.0)),
            ]),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(text: &str) -> Option<String> {
        positional_reference(&RichText::plain(text))
    }

    #[test]
    fn detects_positional_phrases() {
        assert_eq!(found("Todas as anteriores."), Some("anteriores".into()));
        assert_eq!(found("Nenhuma das anteriores"), Some("anteriores".into()));
        assert_eq!(
            found("Apenas a alternativa B"),
            Some("alternativa b".into())
        );
        assert_eq!(found("As letras a e c"), Some("letras a".into()));
        assert_eq!(found("Só (a) e (b)"), Some("(a)".into()));
        assert_eq!(found("Ambas as opções acima"), Some("acima".into()));
        assert_eq!(
            found("As anteriores; exceto uma"),
            Some("anteriores".into())
        );
        assert_eq!(
            found("Vale a alternativa, b."),
            Some("alternativa b".into())
        );
        assert_eq!(
            found("Só a alternativa c, nada mais"),
            Some("alternativa c".into())
        );
    }

    #[test]
    fn ignores_ordinary_text() {
        assert_eq!(found("A fase anterior à mitose"), None);
        assert_eq!(found("Se a e b são inteiros, a + b é inteiro"), None);
        assert_eq!(found("Item de higiene pessoal"), None);
        assert_eq!(found("4"), None);
    }
}
