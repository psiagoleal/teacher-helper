<!-- Caminho relativo: docs/insumos/briefing.md -->

# Briefing — teacher-helper

> Registro do pedido original e das decisões tomadas na entrevista inicial (2026-10-31).
> Fonte para as specs em `docs/specs/`. Não editar retroativamente: decisão nova vai para
> spec ou ADR.

## Pedido original (resumo fiel)

- Ferramenta web que ajuda professores a criar **provas, testes e materiais de aula**.
- O professor fornece **material de exemplo** (conteúdo, estilo) e uma **IA monta o documento**.
- O professor conecta **a própria conta de IA** (paga ou gratuita), de preferência sem precisar
  entender de API — idealmente contas como Claude Pro, Codex, Gemini.
- Aplicação **local no navegador (WASM)**, **sem conta**, dados **temporários**, hospedável no
  **GitHub Pages**.
- Dar **ferramental completo** tanto ao professor quanto à IA para trabalhar no material.
- Stack preferida: **Svelte/SvelteKit** no front; **Rust** (preferência) ou Python no back;
  Docker se necessário.

## Fato que mudou o pedido

Assinaturas de consumidor **não podem** ser usadas por apps de terceiros em geral:

- **Anthropic** (Claude Free/Pro/Max): uso de tokens OAuth da assinatura fora do Claude Code e
  do claude.ai é proibido pelos Termos de Consumidor (esclarecido em fev/2026, com bloqueio
  no servidor).
- **OpenAI** (ChatGPT Plus/Pro): lançou em 2026 o *Sign in with ChatGPT*, que permite a apps
  **parceiros** consumir a cota do plano do usuário. Exige cadastro do app junto à OpenAI —
  **a investigar** (ver `docs/TICKETS.md`).
- **Google** (Gemini): a chave gratuita do Google AI Studio é o caminho viável.

## Decisões da entrevista

| Tema | Decisão |
|---|---|
| Conexão com IA | OpenRouter (login OAuth, sem colar chave) + chave gratuita Gemini com tutorial + chave própria OpenAI/Anthropic + modelo local no navegador (WebLLM). **Ponte para CLI** (Codex CLI / Claude Code local) na **fase 2, opcional**, com aviso sobre termos de cada provedor. |
| Tipos de material | Todos os quatro, em fases: **provas + listas (MVP)** → planos de aula → slides/apostilas. |
| Formatos de saída | **PDF** pronto e **DOCX** editável. |
| Público | Educação **básica (BNCC) + superior/técnico** — fórmulas pesadas, código, figuras. |
| Persistência | Tudo no **navegador** (IndexedDB), nada em servidor; botão "apagar tudo"; exportar/importar arquivo de projeto. |
| Entradas aceitas | PDF e DOCX; fotos/escaneados; texto colado/temas; links e vídeos. |

## Decisões da segunda rodada (2026-10-31)

| Tema | Decisão |
|---|---|
| Proposta de valor | "A IA gera o conteúdo; o app cuida de estrutura, formatação, memória e controle de qualidade." Tabela em `docs/specs/00-transversal.md`. |
| Skills pedagógicas | Centro do produto: pacotes declarativos versionados no repositório. **MVP só com skills do projeto**; criação pelo professor depois, formato já aberto. |
| Economia de cota | Requisito medido, não intenção (spec 06). **Um modelo por padrão**; "modelo rápido" opcional em configuração avançada. |
| Servidores próprios | Conector único compatível com a API da OpenAI (Ollama, llama.cpp, vLLM, LiteLLM, LM Studio…). |
| Plataformas | **Web primeiro** (GitHub Pages). App **desktop** (Tauri) depois, para recursos que o navegador não comporta; desktop contém a web; núcleo compartilhado (ADR-0003). |

## Decisões da terceira rodada (2026-11-01, após crítica das specs)

| Tema | Decisão |
|---|---|
| Skills do MVP | 5: gerar prova, gerar lista, revisar prova, criar distratores, ajustar dificuldade. BNCC, gabarito comentado e importar prova antiga ficam para a etapa seguinte. |
| Fórmulas | A IA escreve **LaTeX**; spike valida conversão para PDF e DOCX antes de aprovar a spec 04. |
| Novos formatos | Exportação em **Markdown** e diagramas **Mermaid** no MVP. |
| Privacidade | Aviso antes do primeiro envio a cada provedor + detector local de CPF/e-mail/telefone. **Anonimização** planejada para a etapa seguinte. |
| Princípio | **O app sempre pergunta ao professor como proceder antes de agir** sobre os dados dele (spec 00). |
| Stack | pnpm; `app/` + workspace Cargo na raiz; Tailwind 4 + Bits UI; código em inglês, docs e commits em português, Conventional Commits (ADR-0004). |
