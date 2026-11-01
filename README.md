<!-- Caminho relativo: README.md -->

# teacher-helper

> Ferramenta web para professores criarem provas, listas de exercícios e materiais de aula
> com a ajuda de uma IA — a **sua** IA, rodando direto do navegador, sem cadastro e sem
> servidor.

![status](https://img.shields.io/badge/status-em%20desenvolvimento-orange)
![version](https://img.shields.io/badge/version-0.0.0-blue)
![license](https://img.shields.io/badge/license-MIT-green)

> **Estado:** specs do MVP aprovadas; esqueleto do projeto pronto, funcionalidades em construção. Veja `docs/specs/` e `docs/TICKETS.md`.

## Como vai funcionar

1. O professor conecta uma IA: **OpenRouter** (um clique, com modelos gratuitos), **Gemini**
   (chave gratuita), chave própria **OpenAI/Anthropic**, um servidor próprio (**Ollama,
   llama.cpp, vLLM, LiteLLM**…) ou um **modelo local** no navegador.
2. Escolhe uma **skill pedagógica** (gerar prova, criar distratores, revisar prova…), anexa
   materiais de exemplo — PDF, DOCX, fotos, texto, links — e preenche um formulário curto.
3. A IA monta uma prova ou lista **estruturada** (questões, gabarito, pontuação), que o
   professor revisa e ajusta, com ou sem a IA.
4. Exporta em **PDF** pronto para imprimir, **DOCX** editável e **Markdown**, com versões
   A/B, gabarito, fórmulas e diagramas.

Tudo fica no navegador do professor. Nada passa por servidor do projeto. Tudo que não
precisa de IA (diagramação, versões, gabarito, pontuação) é feito por código, para poupar a
cota do professor.

**A IA gera o conteúdo; o teacher-helper cuida de estrutura, formatação, memória e controle
de qualidade.**

## Roadmap

- **MVP (web):** provas e listas de exercícios; skills pedagógicas; entrada
  PDF/DOCX/fotos/texto/links; saída PDF/DOCX.
- **Fase 2:** planos de aula; app desktop (Tauri) para recursos avançados, incluindo ponte
  opcional para Codex CLI / Claude Code locais.
- **Fase 3:** slides e apostilas.

## Stack (proposta)

SvelteKit / Svelte 5 estático (GitHub Pages) + núcleo em Rust compilado para WebAssembly.
Futuro app desktop em Tauri 2, reaproveitando a mesma interface e o mesmo núcleo.
Ver [`docs/adr/`](./docs/adr/).

## Pré-requisitos (desenvolvimento)

- Rust estável ≥ 1.97 com o alvo `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`)
- [`wasm-pack`](https://github.com/wasm-bindgen/wasm-pack) ≥ 0.15
- Node ≥ 24 e pnpm ≥ 11

## Instalação e uso

```bash
pnpm wasm                      # compila o núcleo Rust para WebAssembly
pnpm install
pnpm -C app/web dev            # servidor de desenvolvimento
pnpm -C app/web build          # build estático em app/web/build
```

Testes e verificações: `cargo test --workspace`, `pnpm -C app/web check`,
`pnpm -C app/web lint`, `pnpm -C app/web test` e `pnpm -C app/web test:e2e`
(lista completa no `AGENTS.md`, seção 3).

## Estrutura de diretórios

```
app/              # código do sistema (core e wasm em Rust, web em SvelteKit, desktop futuro)
content/skills/   # skills pedagógicas
app/web/e2e/      # testes ponta a ponta
docs/insumos/     # briefing e material de referência
docs/specs/       # contratos de cada feature (o que conta como pronto)
docs/adr/         # decisões de arquitetura
docs/TICKETS.md   # painel de tarefas
```

> `content/skills/` será criado com a primeira skill pedagógica.

## Como contribuir

1. Leia `AGENTS.md`, as specs em `docs/specs/` e os ADRs em `docs/adr/`.
2. Faça um *fork* e crie uma branch (`feature/minha-feature`).
3. Garanta que linter e testes passam.
4. Abra um PR descrevendo a mudança (ver checklist da skill `pr-review-guard`).

## Licença

Distribuído sob a licença MIT. Veja [`LICENSE`](./LICENSE).

---

## Apoie

**Feito com ❤️ por Iago Leal** | [☕ Apoie o criador]

Se este projeto ajudou você, considere apoiar:

- Buy Me a Coffee: https://buymeacoffee.com/psiagoleal

<a href="https://buymeacoffee.com/psiagoleal" target="_blank" rel="noopener">
  <img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me a Coffee" height="41" width="174" />
</a>
