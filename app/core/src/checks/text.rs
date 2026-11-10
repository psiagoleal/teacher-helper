// Caminho relativo: app/core/src/checks/text.rs

//! Texto comparável a partir do conteúdo rico.

use crate::document::rich::{Block, Inline, RichText};

/// Texto do conteúdo, com fórmulas e código em LaTeX ou fonte, separado por espaço.
/// Imagens e diagramas entram pelo texto alternativo e pela fonte Mermaid.
pub fn plain(content: &RichText) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for block in &content.0 {
        match block {
            Block::Paragraph { inlines } => inline_parts(inlines, &mut parts),
            Block::Math { latex } => parts.push(latex),
            Block::Code { source, .. } => parts.push(source),
            Block::Image { alt, .. } => parts.push(alt),
            Block::Table { header, rows } => {
                for cell in header.iter().chain(rows.iter().flatten()) {
                    inline_parts(cell, &mut parts);
                }
            }
            Block::Diagram { mermaid } => parts.push(mermaid),
        }
    }
    parts.join(" ")
}

fn inline_parts<'a>(inlines: &'a [Inline], parts: &mut Vec<&'a str>) {
    for inline in inlines {
        match inline {
            Inline::Text { text, .. } => parts.push(text),
            Inline::Math { latex } => parts.push(latex),
            Inline::Code { source } => parts.push(source),
            Inline::Blank { .. } => parts.push("___"),
        }
    }
}

/// Forma normalizada para comparar: minúsculas, sem acento, sem pontuação nas pontas e com
/// espaços colapsados.
pub fn normalized(content: &RichText) -> String {
    let lowered: String = plain(content)
        .to_lowercase()
        .chars()
        .map(fold_accent)
        .collect();
    let collapsed = lowered.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed
        .trim_matches(|c: char| c.is_ascii_punctuation())
        .trim()
        .to_owned()
}

/// Conteúdo vazio: nenhum bloco, ou só texto em branco.
pub fn is_empty(content: &RichText) -> bool {
    let has_blank = content.0.iter().any(|block| {
        matches!(block, Block::Paragraph { inlines }
            if inlines.iter().any(|i| matches!(i, Inline::Blank { .. })))
    });
    !has_blank && plain(content).trim().is_empty()
}

/// Identificadores das lacunas, na ordem em que aparecem.
pub fn blank_ids(content: &RichText) -> Vec<&str> {
    let mut ids = Vec::new();
    for block in &content.0 {
        let cells: Vec<&[Inline]> = match block {
            Block::Paragraph { inlines } => vec![inlines],
            Block::Table { header, rows } => header
                .iter()
                .chain(rows.iter().flatten())
                .map(Vec::as_slice)
                .collect(),
            _ => Vec::new(),
        };
        for inline in cells.into_iter().flatten() {
            if let Inline::Blank { id } = inline {
                ids.push(id.as_str());
            }
        }
    }
    ids
}

fn fold_accent(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_ignores_case_accents_spaces_and_edge_punctuation() {
        assert_eq!(
            normalized(&RichText::plain("  Ação   Rápida. ")),
            normalized(&RichText::plain("acao rapida"))
        );
    }

    #[test]
    fn empty_detects_whitespace_only_text() {
        assert!(is_empty(&RichText::plain("   ")));
        assert!(is_empty(&RichText::default()));
        assert!(!is_empty(&RichText::plain("x")));
    }
}
