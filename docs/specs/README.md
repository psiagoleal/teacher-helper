<!-- Caminho relativo: docs/specs/README.md -->

# Specs

Contratos de resultado, um por feature (skill `spec-como-contrato`). A spec fixa **o que**
conta como pronto; a sequência de trabalho fica no plano/tickets, e a escolha com
alternativas fica em ADR.

| Spec | Feature | Fase | Status |
|------|---------|------|--------|
| [00](00-transversal.md) | Requisitos transversais (sem servidor, segurança, privacidade, persistência) | todas | Aprovada |
| [01](01-conexao-ia.md) | Conexão com a IA do professor | MVP | Aprovada |
| [02](02-materiais-de-entrada.md) | Materiais de entrada (PDF, DOCX, fotos, texto, links) | MVP | Aprovada |
| [03](03-provas-e-listas.md) | Geração de provas e listas de exercícios | MVP | Aprovada |
| [04](04-exportacao.md) | Exportação para PDF, DOCX e Markdown | MVP | Aprovada — DOCX depende do spike PRE-5 |
| [05](05-skills-pedagogicas.md) | Skills pedagógicas | MVP | Aprovada |
| [06](06-economia-de-cota.md) | Economia de cota da IA | MVP | Aprovada |

Specs 00–06 aprovadas em 2026-11-01, após crítica em contexto limpo.

## Fases seguintes (spec ainda não escrita)

- **App desktop (Tauri)** — recursos que o navegador não comporta (ADR-0003).
- **Ponte para CLI** — repassa pedidos ao Codex CLI / Claude Code; candidata ao desktop.
- **Planos de aula** — objetivos, habilidades BNCC, sequência didática.
- **Slides e apostilas** — material expositivo.

## Regras

- Critério de aceite em **EARS** com **prova** associada; critério sem prova não vira ticket.
- Mudou o comportamento → atualize a spec no mesmo trabalho.
- Spec só sai de "Rascunho" após crítica em contexto limpo **e** aprovação humana.
