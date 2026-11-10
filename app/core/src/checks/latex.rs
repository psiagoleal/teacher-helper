// Caminho relativo: app/core/src/checks/latex.rs

//! CK-08, parte LaTeX: toda fórmula precisa converter com o `math-core` (ADR-0005), o mesmo
//! conversor da exportação. Mermaid fica no MT-62.

use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};
use serde_json::json;

use super::{CheckId, Finding, Severity, params, questions};
use crate::document::Document;
use crate::document::question::{Choice, Question, QuestionBody};
use crate::document::rich::{Block, Inline, RichText};

/// Fórmula que o `math-core` recusou.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatexIssue {
    /// Posição do erro na fórmula, em caracteres a partir de 0.
    pub position: usize,
    /// Mensagem técnica do `math-core`, em inglês. Serve para a IA corrigir a fórmula (P-02);
    /// a casca não a mostra ao professor.
    pub detail: String,
}

/// Conversor sem macros próprias, igual para a verificação e para a exportação.
pub struct LatexChecker(LatexToMathML);

impl Default for LatexChecker {
    fn default() -> Self {
        match LatexToMathML::new(MathCoreConfig::default(), Vec::new()) {
            Ok(converter) => Self(converter),
            // Sem macros, a construção não tem como falhar.
            Err(_) => unreachable!("math-core recusou a configuração padrão"),
        }
    }
}

impl LatexChecker {
    /// Confere uma fórmula; `block` é a fórmula em destaque, fora do parágrafo.
    pub fn check(&self, latex: &str, block: bool) -> Result<(), LatexIssue> {
        let display = if block {
            MathDisplay::Block
        } else {
            MathDisplay::Inline
        };
        self.0
            .convert_with_local_state(latex, display)
            .map(|_| ())
            .map_err(|error| {
                let start = latex.floor_char_boundary(error.0.start);
                LatexIssue {
                    position: latex[..start].chars().count(),
                    detail: error.error_message(),
                }
            })
    }
}

/// Fórmula encontrada no conteúdo, com o caminho relativo até o campo `latex`.
struct Formula<'a> {
    suffix: String,
    latex: &'a str,
    block: bool,
}

fn formulas<'a>(base: &str, content: &'a RichText) -> Vec<Formula<'a>> {
    let mut out = Vec::new();
    let inline = |prefix: String, inlines: &'a [Inline], out: &mut Vec<Formula<'a>>| {
        for (j, item) in inlines.iter().enumerate() {
            if let Inline::Math { latex } = item {
                out.push(Formula {
                    suffix: format!("{prefix}/{j}/latex"),
                    latex,
                    block: false,
                });
            }
        }
    };
    for (i, block) in content.0.iter().enumerate() {
        let at = format!("{base}/{i}");
        match block {
            Block::Math { latex } => out.push(Formula {
                suffix: format!("{at}/latex"),
                latex,
                block: true,
            }),
            Block::Paragraph { inlines } => inline(format!("{at}/inlines"), inlines, &mut out),
            Block::Table { header, rows } => {
                for (c, cell) in header.iter().enumerate() {
                    inline(format!("{at}/header/{c}"), cell, &mut out);
                }
                for (r, row) in rows.iter().enumerate() {
                    for (c, cell) in row.iter().enumerate() {
                        inline(format!("{at}/rows/{r}/{c}"), cell, &mut out);
                    }
                }
            }
            Block::Code { .. } | Block::Image { .. } | Block::Diagram { .. } => {}
        }
    }
    out
}

/// Campos de conteúdo rico da questão, com o caminho de cada um.
fn rich_fields(question: &Question) -> Vec<(String, &RichText)> {
    let mut out = vec![("/statement".to_owned(), &question.statement)];
    let groups: Vec<(&str, &[Choice])> = match &question.body {
        QuestionBody::MultipleChoice(mc) => vec![("/body/alternatives", &mc.alternatives)],
        QuestionBody::Matching(m) => vec![("/body/left", &m.left), ("/body/right", &m.right)],
        _ => Vec::new(),
    };
    for (base, list) in groups {
        for (i, choice) in list.iter().enumerate() {
            out.push((format!("{base}/{i}/content"), &choice.content));
        }
    }
    match &question.body {
        QuestionBody::MultipleChoice(_) | QuestionBody::Matching(_) => {}
        QuestionBody::TrueFalse(tf) => {
            for (i, item) in tf.items.iter().enumerate() {
                out.push((format!("/body/items/{i}/content"), &item.content));
            }
        }
        QuestionBody::FillInBlanks(_) => {}
        QuestionBody::ShortAnswer(d) | QuestionBody::LongAnswer(d) => {
            out.push(("/body/criteria".to_owned(), &d.criteria));
            if let Some(expected) = &d.expected_answer {
                out.push(("/body/expected_answer".to_owned(), expected));
            }
        }
    }
    out
}

fn finding(
    path: String,
    question_id: Option<String>,
    formula: &Formula,
    issue: LatexIssue,
) -> Finding {
    Finding {
        check: CheckId::Ck08,
        severity: Severity::Error,
        path,
        question_id,
        code: "latex_invalid".into(),
        params: params([
            ("latex", json!(formula.latex)),
            ("position", json!(issue.position)),
            ("detail", json!(issue.detail)),
        ]),
    }
}

pub fn ck08(document: &Document) -> Vec<Finding> {
    let checker = LatexChecker::default();
    let mut out = Vec::new();
    for (s, section) in document.sections.iter().enumerate() {
        if let Some(instructions) = &section.instructions {
            for formula in formulas("", instructions) {
                if let Err(issue) = checker.check(formula.latex, formula.block) {
                    let path = format!("/sections/{s}/instructions{}", formula.suffix);
                    out.push(finding(path, None, &formula, issue));
                }
            }
        }
    }
    for at in questions(document) {
        let q = at.question;
        for (field, content) in rich_fields(q) {
            for formula in formulas(&field, content) {
                if let Err(issue) = checker.check(formula.latex, formula.block) {
                    let path = format!("{}{}", at.path, formula.suffix);
                    out.push(finding(path, Some(q.id.clone()), &formula, issue));
                }
            }
        }
    }
    out
}
