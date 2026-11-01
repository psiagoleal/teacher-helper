<!-- Caminho relativo: docs/spikes/pre-5/README.md -->

# Spike PRE-5 — fórmulas, PDF/UA, DOCX e Mermaid

Código **descartável** que sustentou o ADR-0005. Não é código do produto e não é compilado
pelo CI; fica aqui só para reproduzir as medições. Resultados e decisão: ADR-0005.

| Arquivo | O que prova |
|---|---|
| `typst_pdf_lib.rs`, `typst_pdf_main.rs`, `typst_pdf_Cargo.toml` | LaTeX → `mitex` → Typst → PDF/UA-1, em memória; mesmo código compila para WASM |
| `xarrow.typ` | substituto próprio (MIT) do pacote GPL `xarrow` |
| `bench-wasm.mjs` | tempo de inicialização e de compilação do WASM no Node |
| `latex_to_mathml.rs`, `samples.txt` | LaTeX → MathML com `math-core` |
| `mml2omml.py` | protótipo MathML → OMML e DOCX mínimo (o real será em Rust) |
| `mermaid_render.mjs` | Mermaid → SVG sem `foreignObject` no Chromium do Playwright |
| `amostra-*.png` | saídas renderizadas (PDF do Typst; DOCX aberto no LibreOffice) |

## Reproduzir (resumo)

1. Crate com `typst_pdf_Cargo.toml` e os `.rs`; em `assets/` colocar `specs/` do pacote
   `@preview/mitex:0.2.4` (Typst Universe), trocando o import do `xarrow` por `/xarrow.typ` e o
   símbolo `kai` (removido no Typst 0.15) por `K`.
2. `cargo run --release -- prova.pdf [diagrama.svg]`; WASM: `wasm-pack build --target web --release`.
3. `cargo run --release < samples.txt > mathml.tsv` (crate com `math-core`), depois
   `python3 -I mml2omml.py mathml.tsv formulas.docx` e
   `soffice --headless --convert-to pdf formulas.docx`.
4. `node mermaid_render.mjs <app/web> <mermaid.min.js> <saida>`.
5. veraPDF: `docker create verapdf/cli:latest --flavour ua1 --format text /tmp/x.pdf`, `docker cp`
   do PDF e `docker start -a`.
