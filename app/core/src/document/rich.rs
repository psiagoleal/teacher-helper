// Caminho relativo: app/core/src/document/rich.rs

//! Conteúdo rico dos campos de texto (spec 03, "Conteúdo rico"; ADR-0005).
//!
//! Só descreve a forma. A validação de LaTeX, Mermaid e texto alternativo fica no MT-7 e
//! no MT-10; a renderização, na casca e na exportação.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Texto com conteúdo rico: uma sequência de blocos.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct RichText(pub Vec<Block>);

/// Bloco de conteúdo rico.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Block {
    /// Parágrafo de texto, com fórmulas e código em linha.
    Paragraph { inlines: Vec<Inline> },
    /// Fórmula em destaque, em LaTeX (subconjunto do ADR-0005).
    Math { latex: String },
    /// Código com linguagem declarada; linguagens fora da lista de realce aparecem sem realce.
    Code { language: String, source: String },
    /// Imagem já guardada pelo app, com texto alternativo obrigatório.
    Image { asset_id: String, alt: String },
    /// Tabela com uma linha de cabeçalho.
    Table {
        header: Vec<Cell>,
        rows: Vec<Vec<Cell>>,
    },
    /// Diagrama Mermaid em texto (tipos e regras do ADR-0005).
    Diagram { mermaid: String },
}

/// Célula de tabela: texto em linha.
pub type Cell = Vec<Inline>;

/// Trecho em linha dentro de parágrafo ou célula.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Inline {
    /// Texto simples, com negrito e itálico opcionais.
    Text {
        text: String,
        #[serde(default, skip_serializing_if = "is_false")]
        bold: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        italic: bool,
    },
    /// Fórmula em linha, em LaTeX.
    Math { latex: String },
    /// Código em linha, sem realce.
    Code { source: String },
}

fn is_false(value: &bool) -> bool {
    !*value
}

impl RichText {
    /// Texto simples num único parágrafo.
    pub fn plain(text: impl Into<String>) -> Self {
        Self(vec![Block::Paragraph {
            inlines: vec![Inline::Text {
                text: text.into(),
                bold: false,
                italic: false,
            }],
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_text_serializes_without_default_marks() {
        let value = serde_json::to_value(RichText::plain("Olá")).unwrap();
        assert_eq!(
            value,
            json!([{"type": "paragraph", "inlines": [{"type": "text", "text": "Olá"}]}])
        );
    }

    #[test]
    fn unknown_field_is_rejected() {
        let result: Result<Block, _> =
            serde_json::from_value(json!({"type": "math", "latex": "x", "extra": 1}));
        assert!(result.is_err());
    }

    #[test]
    fn unknown_block_type_is_rejected() {
        let result: Result<Block, _> = serde_json::from_value(json!({"type": "video"}));
        assert!(result.is_err());
    }
}
