<!-- Caminho relativo: docs/specs/04-exportacao.md -->

# Spec 04 — Exportação para PDF, DOCX e Markdown

- **Status:** Aprovada (2026-11-01) — critérios de fórmula no DOCX dependem do spike PRE-5
- **Fase:** MVP
- **Cita:** `00-transversal.md`, `03-provas-e-listas.md`

## Problema e resultado esperado

O professor precisa de um **PDF pronto para imprimir**, de um **DOCX** para ajustar no Word ou
Google Docs e de um **Markdown** para levar o conteúdo a outras ferramentas (Obsidian, Notion,
GitHub). Os três são gerados **no navegador**, a partir do mesmo documento estruturado, sem
chamada à IA.

## Decisão pendente

A IA escreve fórmulas em **LaTeX** (decisão do briefing). A conversão para Typst (PDF) e para
equação editável do Word (DOCX), e a renderização de Mermaid como SVG sem `foreignObject`
(exigência do Typst), serão validadas num spike. **SE o DOCX não conseguir equação editável,
ENTÃO a fórmula vai como imagem com o LaTeX no texto alternativo** — decisão registrada em ADR.

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| E-01 | QUANDO o professor exportar, o app DEVE baixar os componentes de exportação da própria origem (se ainda não baixados) e gerar os arquivos sem nenhuma outra requisição de rede. | e2e com interceptação: só a própria origem |
| E-02 | O PDF DEVE reproduzir fórmulas, código, imagens, tabelas, diagramas, cabeçalho, numeração de questões e de páginas da pré-visualização. | regressão visual com documentos de fixture |
| E-03 | O DOCX DEVE ser OOXML válido e abrir sem erro no LibreOffice; fórmulas DEVEM ser equações editáveis ou imagem conforme o ADR do spike. | validação contra o esquema OOXML + LibreOffice headless no CI; Word e Google Docs em roteiro manual registrado a cada release |
| E-04 | O app DEVE exportar Markdown (GFM) com fórmulas em `$…$`/`$$…$$`, diagramas em blocos `mermaid` e gabarito em arquivo separado; ONDE houver imagens, DEVE empacotar `.md` e imagens num `.zip`. | teste unitário: o `.md` gerado é analisado e reconstrói o mesmo conteúdo |
| E-05 | Diagramas Mermaid DEVEM sair como vetor no PDF e como imagem no DOCX, com texto alternativo. | teste unitário + regressão visual |
| E-06 | O app DEVE exportar prova e gabarito como arquivos separados, e cada versão (A, B, …) como arquivo próprio. | teste unitário dos nomes e conteúdos |
| E-07 | O app DEVE permitir escolher tamanho de papel (A4 padrão, Carta), margens, fonte base e linhas de resposta das discursivas. | teste unitário de layout |
| E-08 | O app DEVE evitar quebrar questão de múltipla escolha entre páginas, exceto quando ela sozinha ocupar mais de uma página. | teste de layout com fixture |
| E-09 | O PDF DEVE ser marcado (tagged), com idioma pt-BR e texto alternativo em imagens e diagramas. | verificação com veraPDF (perfil PDF/UA-1) no CI |
| E-10 | SE a geração falhar (fórmula inválida, imagem corrompida), ENTÃO o app DEVE apontar a questão problemática e não gerar arquivo parcial. | teste unitário com fixture inválida |
| E-11 | A exportação de 20 questões com fórmulas DEVE concluir em até 10 s no computador de referência (spec 00). | benchmark no CI com limite calibrado ao runner |

## Fora de escopo do MVP

- Exportar para Google Docs diretamente (o professor importa o DOCX).
- PPTX, ODT, HTML, Moodle XML, Google Forms.
- Editor de layout livre (arrastar elementos).
