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
