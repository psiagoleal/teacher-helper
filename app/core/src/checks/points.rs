// Caminho relativo: app/core/src/checks/points.rs

//! CK-06: soma da pontuação igual ao total definido (P-11).

use serde_json::json;

use super::{CheckId, Finding, Severity, params, questions};
use crate::document::Document;

/// Pontos são comparados em centésimos, para 0,1 + 0,2 somar 0,3.
fn cents(points: f64) -> i64 {
    (points * 100.0).round() as i64
}

pub fn ck06(document: &Document) -> Vec<Finding> {
    let Some(target) = document.target_total_points else {
        return Vec::new();
    };
    let sum: i64 = questions(document)
        .map(|at| cents(at.question.points))
        .sum();
    if sum == cents(target) {
        return Vec::new();
    }
    vec![Finding {
        check: CheckId::Ck06,
        severity: Severity::Error,
        path: "/target_total_points".into(),
        question_id: None,
        code: "points_sum_mismatch".into(),
        params: params([
            ("sum", json!(sum as f64 / 100.0)),
            ("target", json!(cents(target) as f64 / 100.0)),
        ]),
    }]
}
