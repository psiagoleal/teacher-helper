<!-- Caminho relativo: CHANGELOG.md -->

# Changelog

Todos os registros notáveis deste projeto são documentados aqui.
O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/)
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Não lançado]

### Adicionado
- Estrutura inicial do projeto (perfil pessoal de agentes).
- Briefing do produto em `docs/insumos/briefing.md`.
- Specs 00–04 do MVP (rascunho) em `docs/specs/`.
- ADR-0001 (aplicação estática sem backend) e ADR-0002 (conexão com a IA do usuário), propostos.
- Proposta de valor na spec 00; specs 05 (skills pedagógicas) e 06 (economia de cota).
- Conector para servidores compatíveis com a API da OpenAI na spec 01.
- ADR-0003 (web primeiro; desktop Tauri sobre núcleo compartilhado), proposto.
- Rodada 2 das specs após crítica: segurança (XSS, CSP), privacidade (aviso e detector de
  dados pessoais), princípio "perguntar antes de agir", verificação de gabarito, migração de
  esquema, múltiplas abas, exportação Markdown e diagramas Mermaid.
- ADR-0004 (stack, ferramentas e layout), proposto; ilhas de stack do `AGENTS.md` preenchidas.
- Specs 00–06 aprovadas.
- Esqueleto do projeto: workspace Cargo (`app/core`, `app/wasm`), SvelteKit estático
  (`app/web`) carregando o núcleo WASM, CSP via `<meta>`, testes unitários e e2e, e
  workflow de CI com deploy no GitHub Pages.
- Spike PRE-5 e ADR-0005: Typst em WASM gera PDF/UA-1 (validado com veraPDF) a partir de
  LaTeX; LaTeX → equação editável do Word viável; Mermaid em SVG puro.
- Plano do MVP em `docs/plano-mvp.md`: 79 micro-tickets em 9 ondas, refletidos no painel
  `docs/TICKETS.md`.
- Specs 02 e 03 sem questões em aberto: o professor escolhe o uso das imagens dos materiais
  (M-14) e o cabeçalho vem de modelo configurável (P-17).
- ADR-0006 (fronteira entre núcleo Rust e interface; sessão de documento por comandos),
  aceito.
- GitHub: repositório público, `main` protegida por ruleset versionado, PR com *squash*,
  varredura de segredos (gitleaks no CI e proteção de push), CodeQL, Dependabot, relato
  privado de vulnerabilidade (`SECURITY.md`) e actions do CI fixadas por SHA.
- Site publicado em <https://psiagoleal.github.io/teacher-helper/>.
- Núcleo: tipos de conteúdo rico (parágrafo, fórmula, código, imagem, tabela, Mermaid) e
  do cabeçalho do documento, com JSON de ida e volta e JSON Schema gerado (parte do MT-6).
- Núcleo: tipos do documento, das seções e dos 6 tipos de questão (múltipla escolha,
  verdadeiro/falso, associação, lacunas, discursiva curta e longa), com gabarito por `id`
  estável e registro das skills que produziram o documento (MT-6).

### Corrigido
- CI: pnpm fixado em 11.28.10 (a 11.12.0 foi retirada por defeito e quebrava a instalação).
- Dependabot agrupa só atualizações *minor* e *patch*; *major* chega em PR própria, e
  TypeScript e `@types/node` não sobem de *major* sozinhos.
