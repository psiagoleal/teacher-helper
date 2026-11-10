// Caminho relativo: app/core/src/lib.rs

//! Núcleo do teacher-helper.
//!
//! Rust puro, sem acesso a APIs de navegador (ADR-0003): o mesmo código serve à web
//! (via `app/wasm`) e ao futuro app desktop.

pub mod checks;
pub mod document;

/// Versão atual do esquema de documento (spec 00, T-12).
pub const SCHEMA_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_starts_at_one() {
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
