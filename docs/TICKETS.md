<!-- Caminho relativo: docs/TICKETS.md -->

# Tickets

> Visão geral rápida do que foi feito e do que falta. **Uma linha por ticket**: identificador
> com link para o detalhe, e uma frase curta. O detalhe mora no destino do link, não aqui.
> Atualizado **sempre** que um ticket é criado, concluído ou abandonado — skill
> `micro-ticket-planner`.

## Em aberto

- [ ] PRE-2 — Verificar nas fontes oficiais: termos e CORS de cada provedor (header da Anthropic), `localhost` via HTTPS no Safari, permissão de rede local no Chrome, eviction do IndexedDB no Safari; aceitar ou ajustar ADR-0001 a 0004.
- [ ] PRE-3 — Investigar *Sign in with ChatGPT*: requisitos de cadastro para app gratuito estático.
- [ ] PRE-5 — Spike: LaTeX→Typst (PDF, incl. PDF/UA), LaTeX→equação do Word (DOCX) no navegador, Mermaid→SVG sem `foreignObject`; resultado vira ADR e destrava a spec 04.
- [ ] PRE-6 — Quebrar o MVP em micro-tickets (`micro-ticket-planner`).
- [ ] PRE-7 — Levantar concorrentes (ex.: MagicSchool e equivalentes brasileiros) e validar a proposta de valor da spec 00.
- [ ] PRE-11 — Primeiro push: criar o repositório no GitHub, ativar Pages com origem "GitHub Actions" e confirmar o CI verde.
- [ ] PRE-9 — `docs/DESIGN.md` (contrato visual) antes do primeiro componente de interface.
- [ ] PRE-10 — Planejar a próxima etapa: anonimização de dados pessoais, BNCC (fonte e licença da base), skills de gabarito comentado e importação de prova antiga.

## Concluídos

- [x] PRE-0 — Entrevista inicial, briefing (`docs/insumos/briefing.md`) e rascunho das specs 00–04.
- [x] PRE-0b — Proposta de valor, specs 05 (skills) e 06 (economia de cota), conector compatível com OpenAI, ADR-0003 (desktop).
- [x] PRE-0c — Crítica das specs em contexto limpo e rodada 2 das specs 00–06.
- [x] PRE-4 — Questionário de stack: ADR-0004 e ilhas do `AGENTS.md`.
- [x] PRE-1 — Specs 00–06 aprovadas pelo mantenedor (2026-11-01).
- [x] PRE-8 — Scaffold (workspace Cargo, SvelteKit, WASM, CSP, CI/deploy); todos os comandos da ilha `comandos-exatos` executados.

## Abandonados

<!-- - [~] MT-0 — <frase curta> · motivo: <uma frase> -->
