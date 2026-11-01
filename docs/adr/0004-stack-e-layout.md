<!-- Caminho relativo: docs/adr/0004-stack-e-layout.md -->

# ADR 0004: Stack, ferramentas e layout do repositório

- **Status:** Proposed
- **Data:** 2026-11-01
- **Decisores:** Iago Leal
- **Tags:** stack, ferramentas, layout

## Contexto

ADR-0001 fixou app estático com núcleo Rust/WASM; ADR-0003 exige núcleo compartilhado que
sirva também a um futuro app Tauri. Falta fixar ferramentas, convenções e layout antes do
primeiro código. O perfil do repositório (`AGENTS.md`, seção 2) já indica Rust/Python e
SvelteKit; aqui se usa Rust e SvelteKit, sem Python no produto.

Alternativas consideradas e descartadas:

- **npm** em vez de pnpm: zero instalação extra, mas mais lento e lockfile menos estrito.
- **Workspace Cargo e pnpm dentro de `app/`**: raiz mais limpa, mas ferramentas de Rust e CI
  esperam o workspace na raiz.
- **CSS próprio** ou **shadcn-svelte** em vez de Tailwind + Bits UI: o primeiro exige
  construir componentes acessíveis do zero; o segundo copia muito código para manter.
- **Python no núcleo** (Pyodide): runtime pesado no navegador e sem reaproveitamento no Tauri.

## Decisão

| Área | Escolha |
|---|---|
| Núcleo | Rust estável (≥ 1.97), crates `app/core` (Rust puro, sem API de navegador) e `app/wasm` (bindings `wasm-bindgen`), compilado com `wasm-pack` para `wasm32-unknown-unknown` |
| Interface | SvelteKit + Svelte 5 com `adapter-static`, TypeScript estrito, em `app/web` |
| Pacotes JS | pnpm (≥ 11), Node ≥ 24 |
| Estilo | Tailwind CSS 4 + Bits UI (componentes acessíveis sem estilo); contrato visual em `docs/DESIGN.md` |
| Qualidade Rust | `rustfmt`, `clippy -D warnings`, `cargo test`, `wasm-bindgen-test` |
| Qualidade web | Prettier, ESLint, `svelte-check`, Vitest, Playwright (e2e), axe-core |
| Armazenamento | IndexedDB (sem banco de dados) |
| CI/CD | GitHub Actions; deploy no GitHub Pages |
| Desktop (futuro) | Tauri 2 em `app/desktop` (ADR-0003) |
| Idioma | identificadores em inglês; comentários, docs, interface e commits em português (pt-BR) |
| Commits | Conventional Commits (`feat:`, `fix:`, `docs:` …) com descrição em português |
| Licença | MIT |

Layout:

```
Cargo.toml            # [workspace] members = ["app/core", "app/wasm"]
pnpm-workspace.yaml   # packages: ["app/web"]
app/
  core/               # Rust puro
  wasm/               # bindings WASM do core
  web/                # SvelteKit
  desktop/            # Tauri 2 — futuro
content/skills/       # skills pedagógicas do produto (spec 05)
                      # testes: unitários ao lado do código; e2e Playwright em app/web/e2e/
docs/                 # specs, ADRs, insumos, handoff
skills/  agents/      # regramento do agente (não é produto)
```

## Consequências

- **Impacto positivo:** mesma linguagem no núcleo da web e do desktop; ferramentas já
  instaladas na máquina do mantenedor.
- **Impacto negativo:** contribuidor precisa de Rust e Node; build do WASM aumenta o tempo de CI.
- **Trade-offs aceitos:** dois ecossistemas de teste (cargo e Vitest/Playwright).

## Diretriz de Conformidade de Código

- Obrigatório: código do sistema só sob `app/`; skills pedagógicas só em `content/skills/`.
- Obrigatório: `app/core` não depende de `web-sys`, `js-sys` nem `wasm-bindgen`.
- Proibido: npm ou yarn no lugar do pnpm; nova linguagem no produto sem ADR.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
