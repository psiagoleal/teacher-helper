// Caminho relativo: app/wasm/src/lib.rs

//! Bindings WebAssembly do núcleo. Só adapta tipos; nenhuma regra de negócio mora aqui.

use wasm_bindgen::prelude::*;

/// Versão do esquema de documento suportada por este build.
#[wasm_bindgen(js_name = schemaVersion)]
pub fn schema_version() -> u32 {
    teacher_helper_core::SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn exposes_core_schema_version() {
        assert_eq!(schema_version(), teacher_helper_core::SCHEMA_VERSION);
    }
}
