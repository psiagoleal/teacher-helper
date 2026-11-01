<!-- Caminho relativo: docs/plano-mvp.md -->

# Plano do MVP — micro-tickets

> Plano **descartável**: fixa a ordem do trabalho, não o resultado (o resultado está nas specs
> 00–06). Cada ticket cabe em um ciclo de contexto do agente (skill `micro-ticket-planner`).
> Painel de acompanhamento: [`TICKETS.md`](TICKETS.md).

## Como ler

- **Ondas** agrupam tickets por dependência. Tickets de uma mesma onda podem andar em
  paralelo quando não compartilham arquivos.
- **Critério de aceite** é sempre um comando, ou um comando mais a aprovação do mantenedor
  (ADRs). Todo ticket de código também precisa passar nos comandos da ilha `comandos-exatos`
  do `AGENTS.md`. Isso não é repetido em cada ticket.
- **Caminhos** seguem o layout do ADR-0004. O MT-1 pode renomear módulos; se isso acontecer,
  atualize os caminhos dos tickets afetados no mesmo trabalho.
- Atalhos: `core` = `app/core`, `wasm` = `app/wasm`, `web` = `app/web`.
- A regra dos testes é `teste-primeiro`: cada critério EARS citado vira teste antes da
  implementação.

## Ordem das ondas

```
Onda 0  decisões (ADRs) ─┬─> Onda 1  núcleo do documento ──> Onda 2  skills
                         ├─> Onda 3  conexão com IA  (também depende do PRE-2)
                         └─> Onda 4  persistência
PRE-9 DESIGN.md ────────────> Onda 6  interface  (depende das ondas 1–4)
Onda 5  guardas de CI (cedo; cresce junto com a interface)
Onda 7  exportação (depende da onda 1; a tela depende da onda 6)
Onda 8  fechamento do MVP
```


## Tickets PRE em aberto

Tickets de preparação, anteriores a este plano.

<a id="pre-2"></a>
### PRE-2 — Verificar premissas de provedores e navegadores
- **Objetivo:** confirmar nas fontes oficiais os termos e o CORS de cada provedor (cabeçalho
  da Anthropic), `http://localhost` a partir de HTTPS no Safari, permissão de rede local no
  Chrome e remoção de dados do IndexedDB no Safari; aceitar ou ajustar ADR-0001 a 0004.
- **Arquivos no escopo:** `docs/adr/0001` a `0004`, `docs/specs/01-conexao-ia.md`
- **Critério de aceite:** cada premissa com link para a fonte oficial e data; ADRs com status
  atualizado pelo mantenedor.
- **Fora de escopo:** código.
- **Depende de:** nenhum

<a id="pre-3"></a>
### PRE-3 — Investigar *Sign in with ChatGPT*
- **Objetivo:** levantar requisitos de cadastro junto à OpenAI para um app gratuito e estático.
- **Arquivos no escopo:** `docs/insumos/sign-in-with-chatgpt.md`
- **Critério de aceite:** nota com fontes; recomendação de entrar ou não como adaptador (C-07).
- **Fora de escopo:** implementação.
- **Depende de:** nenhum

<a id="pre-7"></a>
### PRE-7 — Levantar concorrentes
- **Objetivo:** comparar MagicSchool e equivalentes brasileiros com a proposta de valor da
  spec 00.
- **Arquivos no escopo:** `docs/insumos/concorrentes.md`
- **Critério de aceite:** tabela com fontes; ajustes na spec 00 propostos ao mantenedor.
- **Fora de escopo:** —
- **Depende de:** nenhum

<a id="pre-9"></a>
### PRE-9 — Escrever o contrato visual (`docs/DESIGN.md`)
- **Objetivo:** cores, tipografia, espaçamento, componentes Bits UI e padrões de diálogo de
  confirmação, antes da primeira tela.
- **Arquivos no escopo:** `docs/DESIGN.md`, `app/web/src/routes/layout.css`
- **Critério de aceite:** aprovado pelo mantenedor; tokens no CSS batem com o documento.
- **Fora de escopo:** telas (onda 6).
- **Depende de:** nenhum

<a id="pre-10"></a>
### PRE-10 — Planejar a próxima etapa
- **Objetivo:** anonimização de dados pessoais, BNCC (fonte e licença da base), skills de
  gabarito comentado, importação de prova antiga e correção de provas respondidas (princípios
  na spec 00, "Fora do MVP, planejado").
- **Arquivos no escopo:** `docs/specs/` (rascunhos novos)
- **Critério de aceite:** specs em rascunho criticadas e aprovadas pelo mantenedor.
- **Fora de escopo:** MVP.
- **Depende de:** MT-78

<a id="pre-11"></a>
### PRE-11 — Fazer o primeiro push e ativar o GitHub Pages
- **Objetivo:** o mantenedor cria o repositório, ativa Pages com origem "GitHub Actions" e
  envia o código.
- **Arquivos no escopo:** nenhum (ação do mantenedor)
- **Critério de aceite:** CI verde e página publicada mostrando "Núcleo carregado".
- **Fora de escopo:** —
- **Depende de:** nenhum

---

## Onda 0 — Decisões que destravam o código

<a id="mt-1"></a>
### MT-1 — Registrar a fronteira entre núcleo e interface (ADR-0006)
- **Objetivo:** ADR que fixa: o que roda em Rust e o que roda em TypeScript; a interface de
  plataforma (rede com *streaming* e cancelamento, armazenamento, relógio, aleatoriedade com
  semente); quem guarda o estado do documento aberto; formato de troca pela fronteira WASM
  (JSON via `serde`?); tratamento de erros; um WASM leve ou vários.
- **Arquivos no escopo:** `docs/adr/0006-fronteira-nucleo-interface.md`, `docs/adr/README.md`
- **Critério de aceite:** ADR com alternativas e diretriz de conformidade, aprovado pelo
  mantenedor; `checar-links.py` passa.
- **Fora de escopo:** código.
- **Depende de:** ADR-0003, ADR-0004

<a id="mt-2"></a>
### MT-2 — Escolher extração de PDF, DOCX e OCR (spike + ADR-0007)
- **Objetivo:** comparar candidatos (ex.: `pdf.js` × crates Rust; `tesseract.js` × outros)
  por licença, tamanho gzip, qualidade nas fixtures e funcionamento sem rede externa; registrar
  a escolha.
- **Arquivos no escopo:** `docs/spikes/mt-2/`, `docs/adr/0007-extracao-de-materiais.md`,
  `docs/adr/README.md`
- **Critério de aceite:** tabela de medição no ADR; nenhuma dependência GPL/LGPL; ADR aprovado.
- **Fora de escopo:** integrar ao app.
- **Depende de:** MT-1

<a id="mt-3"></a>
### MT-3 — Escolher a renderização da pré-visualização (ADR-0008)
- **Objetivo:** decidir como fórmulas aparecem na tela (MathML nativo gerado pelo `math-core`
  × KaTeX × outros) e qual biblioteca faz o realce de código das 11 linguagens da spec 03,
  com segurança contra XSS (T-10) e peso no carregamento (T-08).
- **Arquivos no escopo:** `docs/adr/0008-pre-visualizacao.md`, `docs/adr/README.md`
- **Critério de aceite:** ADR aprovado, com medição de tamanho e teste das fórmulas de
  `docs/spikes/pre-5/samples.txt` nos quatro navegadores suportados.
- **Fora de escopo:** componente de pré-visualização (MT-61).
- **Depende de:** ADR-0005

<a id="mt-4"></a>
### MT-4 — Escolher o motor de modelo local (ADR-0009)
- **Objetivo:** decidir a biblioteca de modelo no navegador (ex.: WebLLM), a lista fixa de
  origens dos pesos (T-02 d), os modelos oferecidos, o tamanho de cada um e as capacidades
  declaradas.
- **Arquivos no escopo:** `docs/adr/0009-modelo-local.md`, `docs/adr/README.md`,
  `docs/adr/0001-arquitetura-estatica-sem-servidor.md` (lista de origens)
- **Critério de aceite:** ADR aprovado; licença compatível com MIT.
- **Fora de escopo:** integração (MT-40).
- **Depende de:** MT-1

<a id="mt-5"></a>
### MT-5 — Fechar as questões em aberto das specs 02 e 03
- **Objetivo:** perguntar ao mantenedor e registrar: (a) imagens dentro de PDF/DOCX de entrada
  — reaproveitar ou só referência; (b) cabeçalho — fixo ou template configurável.
- **Arquivos no escopo:** `docs/specs/02-materiais-de-entrada.md`,
  `docs/specs/03-provas-e-listas.md`
- **Critério de aceite:** nenhuma das duas specs tem seção "Questões em aberto"; mudança
  aprovada pelo mantenedor.
- **Fora de escopo:** implementação.
- **Depende de:** nenhum

---

## Onda 1 — Núcleo do documento (`core`, Rust puro)

<a id="mt-6"></a>
### MT-6 — Definir os tipos do documento e da questão
- **Objetivo:** tipos Rust do modelo da spec 03 (documento, seção, questão nos 6 tipos,
  conteúdo rico, `embaralhavel`, estado do gabarito, skills que produziram — S-08, cabeçalho
  com a cópia dos campos do modelo escolhido — P-17) com
  `schema_version`, serialização JSON de ida e volta e JSON Schema gerado a partir dos tipos.
- **Arquivos no escopo:** `core/Cargo.toml`, `core/src/lib.rs`, `core/src/document/mod.rs`,
  `core/src/document/question.rs`, `core/src/document/rich.rs`, `core/tests/fixtures/documents/`
- **Critério de aceite:** `cargo test -p teacher-helper-core document` — fixtures de cada tipo
  de questão fazem ida e volta sem perda.
- **Fora de escopo:** validação semântica (MT-7), migração (MT-8).
- **Depende de:** MT-1

<a id="mt-7"></a>
### MT-7 — Validar documento com erros por caminho (P-01)
- **Objetivo:** validar um JSON contra o esquema e devolver erros com caminho do campo
  (`/sections/0/questions/2/answer`) e mensagem curta, prontos para voltar à IA (P-02).
- **Arquivos no escopo:** `core/src/document/validate.rs`, `core/tests/validate.rs`,
  fixtures inválidas
- **Critério de aceite:** `cargo test -p teacher-helper-core validate` com casos válidos e
  inválidos.
- **Fora de escopo:** verificações pedagógicas CK (MT-9).
- **Depende de:** MT-6

<a id="mt-8"></a>
### MT-8 — Migrar esquema de versões anteriores (T-12)
- **Objetivo:** registro de migrações por versão; projeto antigo é migrado; versão mais nova é
  recusada com motivo, sem alterar a entrada.
- **Arquivos no escopo:** `core/src/document/migrate.rs`, `core/tests/migrate.rs`,
  `core/tests/fixtures/versions/`
- **Critério de aceite:** `cargo test -p teacher-helper-core migrate` com fixture v0 sintética,
  v1 e v99.
- **Fora de escopo:** importação de arquivo de projeto (MT-42).
- **Depende de:** MT-6

<a id="mt-9"></a>
### MT-9 — Implementar as verificações CK-01 a CK-07 e CK-09 (P-07, P-10)
- **Objetivo:** cada verificação é uma função pura que devolve achados por questão; nenhuma
  bloqueia a edição.
- **Arquivos no escopo:** `core/src/checks/mod.rs`, `core/src/checks/*.rs`, `core/tests/checks.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core checks` — uma fixture que viola
  cada verificação, mais uma que passa em todas.
- **Fora de escopo:** CK-08 (MT-10 e MT-62).
- **Depende de:** MT-6

<a id="mt-10"></a>
### MT-10 — Verificar LaTeX no núcleo (CK-08, parte LaTeX)
- **Objetivo:** analisar cada fórmula com `math-core` e acusar a que não converte, com a
  questão e a mensagem.
- **Arquivos no escopo:** `core/Cargo.toml`, `core/src/checks/latex.rs`,
  `core/tests/fixtures/latex/`
- **Critério de aceite:** `cargo test -p teacher-helper-core latex` — as fórmulas de
  `docs/spikes/pre-5/samples.txt` passam; as inválidas da fixture falham.
- **Fora de escopo:** Mermaid (MT-62), conversão para OMML (MT-72).
- **Depende de:** MT-9, ADR-0005

<a id="mt-11"></a>
### MT-11 — Calcular numeração e pontuação (P-11, CK-06)
- **Objetivo:** numerar questões, somar pontos e sinalizar divergência do total definido.
- **Arquivos no escopo:** `core/src/document/scoring.rs`, `core/tests/scoring.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core scoring`.
- **Fora de escopo:** exibição.
- **Depende de:** MT-6

<a id="mt-12"></a>
### MT-12 — Gerar o gabarito separado e controlar a conferência (P-06, P-08)
- **Objetivo:** extrair o gabarito (resposta das objetivas, critério das discursivas); toda
  resposta criada ou alterada pela IA nasce `nao_conferido`; função que lista as não
  conferidas.
- **Arquivos no escopo:** `core/src/document/answer_key.rs`, `core/tests/answer_key.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core answer_key`.
- **Fora de escopo:** aviso na exportação (MT-76).
- **Depende de:** MT-6

<a id="mt-13"></a>
### MT-13 — Embaralhar versões A, B, … com semente (P-09)
- **Objetivo:** gerar N versões embaralhando questões e alternativas `embaralhavel` e a
  coluna B das associações, com gabarito por versão, de forma determinística pela semente.
- **Arquivos no escopo:** `core/Cargo.toml` (dev: `proptest`), `core/src/document/shuffle.rs`,
  `core/tests/shuffle.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core shuffle` com teste de
  propriedade (mesmo conjunto de questões, gabarito coerente).
- **Fora de escopo:** arquivo por versão (MT-69).
- **Depende de:** MT-12

<a id="mt-14"></a>
### MT-14 — Implementar desfazer e refazer (P-14)
- **Objetivo:** histórico de operações sobre o documento, com origem (professor ou IA).
- **Arquivos no escopo:** `core/src/document/history.rs`, `core/tests/history.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core history`.
- **Fora de escopo:** atalhos de teclado (onda 6).
- **Depende de:** MT-6

<a id="mt-15"></a>
### MT-15 — Calcular o diff estrutural por questão (P-04, núcleo)
- **Objetivo:** comparar dois documentos e produzir mudanças por questão; numeração e total
  entram no diff; aplicar ou recusar a proposta.
- **Arquivos no escopo:** `core/src/document/diff.rs`, `core/tests/diff.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core diff` — mudar a questão 3 de 20
  só afeta a questão 3 e, se for o caso, numeração e total.
- **Fora de escopo:** tela de proposta (MT-64).
- **Depende de:** MT-14

<a id="mt-16"></a>
### MT-16 — Buscar questões compatíveis no banco (Q-05)
- **Objetivo:** busca sem IA: mesmo tema (comparação normalizada), mesmo nível, dificuldade
  a no máximo 1 de distância.
- **Arquivos no escopo:** `core/src/bank.rs`, `core/tests/bank.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core bank`.
- **Fora de escopo:** armazenamento do banco (MT-41), tela (MT-66).
- **Depende de:** MT-6

<a id="mt-17"></a>
### MT-17 — Detectar dados pessoais (T-15, M-13)
- **Objetivo:** achar CPF (com dígito verificador), e-mail e telefone brasileiro em texto,
  com posição de cada ocorrência, e aplicar a escolha "remover".
- **Arquivos no escopo:** `core/src/privacy.rs`, `core/tests/privacy.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core privacy` com verdadeiros e falsos
  positivos (ex.: número de 11 dígitos que não é CPF).
- **Fora de escopo:** diálogo (MT-52); anonimização (fora do MVP).
- **Depende de:** nenhum

<a id="mt-18"></a>
### MT-18 — Expor as operações do documento no WASM
- **Objetivo:** bindings das operações das MT-6 a MT-17 conforme a fronteira do ADR-0006, com
  tipos TypeScript gerados.
- **Arquivos no escopo:** `wasm/src/lib.rs`, `wasm/src/document.rs`, `wasm/tests/`,
  `web/src/lib/core.ts`
- **Critério de aceite:** `wasm-pack test --node app/wasm` e `pnpm -C app/web check`.
- **Fora de escopo:** interface.
- **Depende de:** MT-7, MT-8, MT-9, MT-11, MT-12, MT-13, MT-14, MT-15, MT-16, MT-17

---

## Onda 2 — Skills pedagógicas

<a id="mt-19"></a>
### MT-19 — Definir o esquema de skill e validar no CI (S-01, S-02)
- **Objetivo:** formato Markdown + YAML da spec 05, analisador no núcleo e validador que
  falha o build apontando arquivo e campo.
- **Arquivos no escopo:** `core/src/skill/mod.rs`, `core/src/skill/parse.rs`,
  `core/src/bin/validate_skills.rs`, `core/tests/fixtures/skills/`, `content/skills/.gitkeep`,
  `.github/workflows/ci.yml`, `AGENTS.md` (ilha `comandos-exatos`)
- **Critério de aceite:** `cargo test -p teacher-helper-core skill` e
  `cargo run -p teacher-helper-core --bin validate_skills -- content/skills` (comando registrado
  na ilha depois de executado).
- **Fora de escopo:** conteúdo das skills (MT-24 a MT-28).
- **Depende de:** MT-6

<a id="mt-20"></a>
### MT-20 — Montar o pedido com prefixo estável (S-04, Q-07)
- **Objetivo:** pedido = instruções e esquema da skill (prefixo idêntico byte a byte) +
  conteúdo variável; só a skill executada entra.
- **Arquivos no escopo:** `core/src/skill/prompt.rs`, `core/tests/prompt.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core prompt`.
- **Fora de escopo:** contexto de edição (MT-21), resumo de material (MT-57).
- **Depende de:** MT-19

<a id="mt-21"></a>
### MT-21 — Montar o contexto mínimo de edição (Q-04)
- **Objetivo:** pedido de alteração de uma questão com só o contexto mínimo da spec 06.
- **Arquivos no escopo:** `core/src/skill/edit_context.rs`, `core/src/tokens.rs`,
  `core/tests/edit_context.rs`, fixture de 20 questões
- **Critério de aceite:** `cargo test -p teacher-helper-core edit_context` — ≤ 10 % da
  estimativa da geração completa.
- **Fora de escopo:** tokenizador do provedor (estimativa por caracteres ÷ 4 basta aqui).
- **Depende de:** MT-20

<a id="mt-22"></a>
### MT-22 — Implementar a matriz de capacidades (C-18, S-07, M-04, M-09)
- **Objetivo:** dado pedido + modelo, dizer o que falta e qual alternativa oferecer (OCR,
  transcrição, outro modelo).
- **Arquivos no escopo:** `core/src/capabilities.rs`, `core/tests/capabilities.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core capabilities`.
- **Fora de escopo:** avisos na tela.
- **Depende de:** MT-19

<a id="mt-23"></a>
### MT-23 — Fechar o ciclo da resposta (P-02, S-05, S-06, S-08)
- **Objetivo:** validar a resposta contra o esquema da skill, devolver erros à IA (máx. 2
  correções), rodar as verificações declaradas, registrar skill e versão no documento.
- **Arquivos no escopo:** `core/src/skill/run.rs`, `core/tests/run.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core run` com provedor simulado que
  devolve resposta inválida 1, 2 e 3 vezes, e documento que cita skill removida.
- **Fora de escopo:** adaptadores reais (onda 3).
- **Depende de:** MT-7, MT-9, MT-10, MT-20, MT-30

<a id="mt-24"></a>
### MT-24 — Escrever a skill "Gerar prova"
- **Objetivo:** skill completa (entradas, instruções, esquema, capacidades, verificações).
- **Arquivos no escopo:** `content/skills/gerar-prova.md`, fixture de resposta em
  `core/tests/fixtures/skills/`
- **Critério de aceite:** validador da MT-19 passa; a resposta de fixture passa no ciclo da
  MT-23 sem achados.
- **Fora de escopo:** avaliação de qualidade com IA real (roteiro da MT-79).
- **Depende de:** MT-23

<a id="mt-25"></a>
### MT-25 — Escrever a skill "Gerar lista de exercícios"
- **Objetivo, arquivos e aceite:** como a MT-24, para `content/skills/gerar-lista.md`.
- **Fora de escopo:** idem MT-24.
- **Depende de:** MT-24

<a id="mt-26"></a>
### MT-26 — Escrever a skill "Revisar prova"
- **Objetivo:** skill que aponta ambiguidade, mais de uma correta, enunciado que entrega a
  resposta e nível inadequado, como achados por questão.
- **Arquivos no escopo:** `content/skills/revisar-prova.md`, fixture de resposta
- **Critério de aceite:** validador passa; fixture passa no ciclo da MT-23.
- **Fora de escopo:** aplicar as correções sugeridas (vai por proposta, MT-64).
- **Depende de:** MT-23

<a id="mt-27"></a>
### MT-27 — Escrever a skill "Criar distratores"
- **Objetivo, arquivos e aceite:** como a MT-26, para `content/skills/criar-distratores.md`;
  usa o contexto mínimo de edição (MT-21).
- **Fora de escopo:** idem MT-26.
- **Depende de:** MT-21, MT-23

<a id="mt-28"></a>
### MT-28 — Escrever a skill "Ajustar dificuldade"
- **Objetivo, arquivos e aceite:** como a MT-27, para `content/skills/ajustar-dificuldade.md`.
- **Fora de escopo:** idem MT-26.
- **Depende de:** MT-21, MT-23

<a id="mt-29"></a>
### MT-29 — Escrever o guia de contribuição de skills (S-09)
- **Objetivo:** guia com uma skill de exemplo completa, extraída para arquivo que o CI valida.
- **Arquivos no escopo:** `CONTRIBUTING.md`, `content/skills/exemplo/skill-exemplo.md`,
  `.github/workflows/ci.yml`
- **Critério de aceite:** o validador da MT-19 roda sobre o exemplo no CI.
- **Fora de escopo:** criação de skill dentro do app.
- **Depende de:** MT-19

---

## Onda 3 — Conexão com a IA

Depende também do **PRE-2** (verificar CORS e termos de cada provedor).

<a id="mt-30"></a>
### MT-30 — Definir a interface de provedor e a suíte de contrato (C-07 a C-10)
- **Objetivo:** interface única (gerar com *streaming*, cancelar, informar tokens), erros
  classificados (autenticação, 429 com espera, cota esgotada, tempo limite, rede) e adaptador
  simulado; suíte de contrato reutilizável por todo adaptador.
- **Arquivos no escopo:** `core/src/provider/mod.rs`, `core/src/provider/error.rs`,
  `core/src/provider/mock.rs`, `core/tests/provider_contract.rs`
- **Critério de aceite:** `cargo test -p teacher-helper-core provider` — adaptador simulado
  passa na suíte.
- **Fora de escopo:** adaptadores reais.
- **Depende de:** MT-1

<a id="mt-31"></a>
### MT-31 — Implementar o adaptador Chat Completions (OpenRouter e compatíveis) (C-12, C-17)
- **Objetivo:** adaptador para a API Chat Completions com endereço base, chave opcional,
  modelo, *streaming*, cancelamento e `response_format` com JSON Schema quando suportado.
- **Arquivos no escopo:** `core/src/provider/openai_compat.rs`, fixtures de resposta
- **Critério de aceite:** suíte de contrato da MT-30 passa contra servidor simulado.
- **Fora de escopo:** OAuth (MT-35), diagnóstico (MT-37).
- **Depende de:** MT-30, PRE-2

<a id="mt-32"></a>
### MT-32 — Implementar o adaptador Anthropic com cache de prompt (Q-08)
- **Objetivo:** adaptador da API Messages com o cabeçalho de acesso direto do navegador e
  `cache_control` no prefixo estável.
- **Arquivos no escopo:** `core/src/provider/anthropic.rs`, fixtures
- **Critério de aceite:** suíte de contrato passa; teste confere a marca de cache no prefixo.
- **Fora de escopo:** validação de chave (MT-34).
- **Depende de:** MT-30, MT-20, PRE-2

<a id="mt-33"></a>
### MT-33 — Implementar o adaptador Gemini
- **Objetivo:** adaptador da API do Gemini, com saída estruturada quando suportada.
- **Arquivos no escopo:** `core/src/provider/gemini.rs`, fixtures
- **Critério de aceite:** suíte de contrato passa.
- **Fora de escopo:** tutorial de chave (MT-50).
- **Depende de:** MT-30, PRE-2

<a id="mt-34"></a>
### MT-34 — Validar a chave colada (C-03)
- **Objetivo:** chamada mínima por provedor antes de salvar; mensagens simples para 200, 401,
  403 e 429.
- **Arquivos no escopo:** `core/src/provider/validate_key.rs`, testes
- **Critério de aceite:** `cargo test -p teacher-helper-core validate_key` com respostas
  simuladas por provedor.
- **Fora de escopo:** tela.
- **Depende de:** MT-31, MT-32, MT-33

<a id="mt-35"></a>
### MT-35 — Conectar o OpenRouter por OAuth PKCE (C-01, C-02)
- **Objetivo:** fluxo PKCE S256 com `state`, `code_verifier` só no `sessionStorage` durante o
  fluxo, rejeição de `state` divergente e mensagens de erro simples.
- **Arquivos no escopo:** `web/src/lib/auth/openrouter.ts`, `web/src/routes/conectar/retorno/`,
  `web/e2e/openrouter-oauth.e2e.ts`
- **Critério de aceite:** `pnpm -C app/web test:e2e -- openrouter-oauth` com OpenRouter
  simulado, incluindo `state` adulterado e autorização negada.
- **Fora de escopo:** tela de conexão completa (MT-50).
- **Depende de:** MT-31

<a id="mt-36"></a>
### MT-36 — Guardar a chave com segurança (C-05, C-11)
- **Objetivo:** chave persistida ou só em memória ("não lembrar"); nunca em URL, log, erro ou
  exportação.
- **Arquivos no escopo:** `web/src/lib/storage/keys.ts`, `web/src/lib/storage/keys.spec.ts`,
  `web/e2e/chave-canario.e2e.ts`
- **Critério de aceite:** teste com chave canário varre logs, erros, exportações e URLs; e2e
  de recarga pede a chave de novo quando "não lembrar".
- **Fora de escopo:** cofre do sistema (desktop).
- **Depende de:** MT-41

<a id="mt-37"></a>
### MT-37 — Diagnosticar servidor compatível (C-13, C-14)
- **Objetivo:** recusar HTTP fora de localhost com explicação; "Testar conexão" distingue
  servidor fora do ar, CORS, rede local negada, chave inválida e modelo inexistente.
- **Arquivos no escopo:** `core/src/provider/diagnose.rs`, testes
- **Critério de aceite:** `cargo test -p teacher-helper-core diagnose` com cada falha simulada.
- **Fora de escopo:** guia de CORS (MT-50).
- **Depende de:** MT-31

<a id="mt-38"></a>
### MT-38 — Criar as tabelas de custo e de modelos conhecidos (C-04, C-16)
- **Objetivo:** tabela de provedor → grátis/pago + link do painel de uso; tabela de modelos
  conhecidos → capacidades, usada para pré-preencher servidores compatíveis.
- **Arquivos no escopo:** `content/providers.yaml`, `content/models.yaml`,
  `core/src/catalog.rs`, testes
- **Critério de aceite:** `cargo test -p teacher-helper-core catalog`.
- **Fora de escopo:** atualização automática das tabelas.
- **Depende de:** MT-22

<a id="mt-39"></a>
### MT-39 — Rotear para o "modelo rápido" (Q-09)
- **Objetivo:** quando configurado, extração, classificação e resumo vão ao modelo rápido;
  geração e revisão, ao principal.
- **Arquivos no escopo:** `core/src/provider/routing.rs`, testes
- **Critério de aceite:** `cargo test -p teacher-helper-core routing`.
- **Fora de escopo:** tela de configuração (MT-50).
- **Depende de:** MT-30

<a id="mt-40"></a>
### MT-40 — Integrar o modelo local (C-06, T-16)
- **Objetivo:** oferecer só com WebGPU; mostrar o tamanho e pedir confirmação antes do
  download; adaptador passa na suíte de contrato.
- **Arquivos no escopo:** `web/src/lib/local-model/`, testes de componente
- **Critério de aceite:** teste com WebGPU simulado presente e ausente; suíte de contrato.
- **Fora de escopo:** outros motores.
- **Depende de:** MT-4, MT-30

---

## Onda 4 — Persistência (`web`)

<a id="mt-41"></a>
### MT-41 — Guardar no IndexedDB com reserva em memória (T-03, T-06, T-09)
- **Objetivo:** camada de armazenamento de projetos, banco e configurações; aviso e operação
  em memória sem IndexedDB; pedido de armazenamento persistente no primeiro salvamento.
- **Arquivos no escopo:** `web/src/lib/storage/db.ts`, `web/src/lib/storage/db.spec.ts`,
  `web/e2e/armazenamento.e2e.ts`
- **Critério de aceite:** testes unitários (com `navigator.storage.persist` simulado) e e2e
  de recarga e de IndexedDB bloqueado.
- **Fora de escopo:** exportar projeto (MT-42).
- **Depende de:** MT-1, MT-8

<a id="mt-42"></a>
### MT-42 — Exportar e importar projeto (T-05)
- **Objetivo:** um arquivo por projeto, sem a chave, que restaura o projeto idêntico em outro
  navegador; importação passa pela migração.
- **Arquivos no escopo:** `web/src/lib/storage/project-file.ts`, testes
- **Critério de aceite:** teste de ida e volta com comparação estrutural.
- **Fora de escopo:** sincronização.
- **Depende de:** MT-41

<a id="mt-43"></a>
### MT-43 — Apagar tudo (T-04)
- **Objetivo:** com confirmação, limpar IndexedDB, localStorage, sessionStorage e modelos
  baixados e voltar à tela inicial.
- **Arquivos no escopo:** `web/src/lib/storage/wipe.ts`, `web/e2e/apagar-tudo.e2e.ts`
- **Critério de aceite:** e2e — armazenamento vazio e tela inicial após confirmar; nada muda
  ao cancelar.
- **Fora de escopo:** —
- **Depende de:** MT-41

<a id="mt-44"></a>
### MT-44 — Travar edição entre abas (T-13)
- **Objetivo:** Web Locks; segunda aba abre somente leitura, com aviso.
- **Arquivos no escopo:** `web/src/lib/storage/tab-lock.ts`, `web/e2e/duas-abas.e2e.ts`
- **Critério de aceite:** e2e com duas abas.
- **Fora de escopo:** —
- **Depende de:** MT-41

---

## Onda 5 — Guardas de CI

<a id="mt-45"></a>
### MT-45 — Vigiar as origens de rede no e2e (T-02)
- **Objetivo:** ajudante Playwright que falha o teste em requisição fora da lista permitida;
  ligado a todos os e2e.
- **Arquivos no escopo:** `web/e2e/support/network-guard.ts`, `web/e2e/*.e2e.ts`
- **Critério de aceite:** `pnpm -C app/web test:e2e`; um teste de controle com origem proibida
  falha como esperado.
- **Fora de escopo:** CSP (já coberta no scaffold).
- **Depende de:** nenhum

<a id="mt-46"></a>
### MT-46 — Limitar o tamanho do carregamento inicial (T-08)
- **Objetivo:** script que mede JS + CSS gzip do carregamento inicial no build e falha acima
  de 300 KB.
- **Arquivos no escopo:** `web/scripts/check-bundle.mjs`, `web/package.json`,
  `.github/workflows/ci.yml`, `AGENTS.md` (ilha `comandos-exatos`)
- **Critério de aceite:** comando novo executado e registrado; CI roda.
- **Fora de escopo:** otimização.
- **Depende de:** nenhum

<a id="mt-47"></a>
### MT-47 — Verificar acessibilidade e 360 px em toda tela (T-07)
- **Objetivo:** ajudante e2e com axe-core (zero `serious`/`critical`) e checagem de rolagem
  horizontal em 360 px.
- **Arquivos no escopo:** `web/package.json`, `web/e2e/support/a11y.ts`, `web/e2e/home.e2e.ts`
- **Critério de aceite:** `pnpm -C app/web test:e2e` com a home verificada.
- **Fora de escopo:** telas futuras (cada uma chama o ajudante).
- **Depende de:** nenhum

---

## Onda 6 — Interface (`web`)

Toda tela segue o `docs/DESIGN.md` (**PRE-9**) e chama os ajudantes das MT-45 e MT-47.

<a id="mt-48"></a>
### MT-48 — Montar a casca do app e os textos isolados
- **Objetivo:** navegação, lista de projetos (criar, abrir, renomear) e arquivo único de
  textos em pt-BR para i18n futura.
- **Arquivos no escopo:** `web/src/routes/+layout.svelte`, `web/src/routes/+page.svelte`,
  `web/src/lib/i18n/pt-BR.ts`, `web/src/lib/components/`, `web/e2e/projetos.e2e.ts`
- **Critério de aceite:** e2e de criar e reabrir projeto.
- **Fora de escopo:** editor.
- **Depende de:** PRE-9, MT-41

<a id="mt-49"></a>
### MT-49 — Pedir confirmação no primeiro envio a cada provedor (T-14)
- **Objetivo:** diálogo que mostra o que vai e para onde (no OpenRouter, o repasse a
  terceiros) e só segue após confirmar.
- **Arquivos no escopo:** `web/src/lib/components/SendConsent.svelte`, `web/e2e/consentimento.e2e.ts`
- **Critério de aceite:** e2e — sem confirmação, nenhuma requisição ao provedor.
- **Fora de escopo:** dados pessoais (MT-52).
- **Depende de:** MT-48

<a id="mt-50"></a>
### MT-50 — Criar a tela "Conectar IA" (C-04, C-15, Q-09)
- **Objetivo:** escolher provedor, colar chave, botão OpenRouter, tutorial Gemini, servidor
  compatível com guia de CORS já com a origem do app, capacidades pré-preenchidas, custo e
  modelo rápido opcional.
- **Arquivos no escopo:** `web/src/routes/conectar/`, `web/src/lib/components/connect/`,
  `web/e2e/conectar.e2e.ts`
- **Critério de aceite:** e2e de cada caminho com provedores simulados; guia mostra a origem
  atual.
- **Fora de escopo:** modelo local (MT-40).
- **Depende de:** MT-48, MT-34, MT-35, MT-36, MT-37, MT-38

<a id="mt-51"></a>
### MT-51 — Anexar materiais (M-06, M-11, M-12)
- **Objetivo:** anexar arquivos, colar texto ou lista de tópicos, escolher o papel (conteúdo,
  estilo, ambos), recusar com motivo arquivos inválidos ou acima do limite.
- **Arquivos no escopo:** `web/src/lib/components/materials/`, `web/src/lib/materials/`,
  `web/e2e/materiais.e2e.ts`
- **Critério de aceite:** e2e com fixtures válidas, inválidas e grandes.
- **Fora de escopo:** extração (MT-53 a MT-56).
- **Depende de:** MT-48

<a id="mt-52"></a>
### MT-52 — Perguntar sobre dados pessoais antes do envio (T-15)
- **Objetivo:** destacar ocorrências e oferecer remover, manter ou cancelar; nada sai antes da
  escolha.
- **Arquivos no escopo:** `web/src/lib/components/PiiReview.svelte`, `web/e2e/dados-pessoais.e2e.ts`
- **Critério de aceite:** e2e das três escolhas.
- **Fora de escopo:** anonimização.
- **Depende de:** MT-17, MT-18, MT-49

<a id="mt-53"></a>
### MT-53 — Extrair texto de PDF (M-01, M-02)
- **Objetivo:** extrair no navegador, mostrar para conferência e tratar como imagem a página
  abaixo do limite de página escaneada.
- **Arquivos no escopo:** conforme ADR-0007; `web/src/lib/materials/pdf.ts`, fixtures
- **Critério de aceite:** testes com PDF de texto e PDF misto.
- **Fora de escopo:** OCR (MT-55), imagens embutidas (MT-81).
- **Depende de:** MT-2, MT-51

<a id="mt-54"></a>
### MT-54 — Extrair texto de DOCX (M-03)
- **Objetivo:** texto, listas, tabelas e fórmulas (OMML → LaTeX) no navegador.
- **Arquivos no escopo:** conforme ADR-0007; fixture DOCX com equação
- **Critério de aceite:** teste com a fixture.
- **Fora de escopo:** imagens embutidas (MT-81).
- **Depende de:** MT-2, MT-51

<a id="mt-55"></a>
### MT-55 — Enviar imagem por visão ou reconhecer por OCR local (M-04, M-05)
- **Objetivo:** com `visao`, a imagem vai ao modelo; sem, OCR local com texto editável antes
  do uso.
- **Arquivos no escopo:** conforme ADR-0007; `web/src/lib/materials/image.ts`, e2e
- **Critério de aceite:** teste da montagem do pedido + e2e de OCR com fixture.
- **Fora de escopo:** —
- **Depende de:** MT-2, MT-22, MT-51

<a id="mt-56"></a>
### MT-56 — Ler links e tratar vídeos (M-07, M-08, M-09)
- **Objetivo:** GET direto sem outros dados; explicação simples quando falhar; vídeo por URL
  só com `video_por_url`, senão pedir transcrição ou resumo.
- **Arquivos no escopo:** `web/src/lib/materials/link.ts`, testes
- **Critério de aceite:** testes com `fetch` simulado (sucesso e falhas).
- **Fora de escopo:** seguir links.
- **Depende de:** MT-22, MT-51

<a id="mt-57"></a>
### MT-57 — Resumir o material uma vez e reaproveitar (Q-02, Q-03)
- **Objetivo:** no primeiro uso, gerar e guardar resumo e perfil de estilo; depois enviar o
  resumo, salvo com "usar material completo"; nunca enviar o arquivo original.
- **Arquivos no escopo:** `core/src/materials/summary.rs`, `web/src/lib/materials/cache.ts`,
  testes
- **Critério de aceite:** teste — a segunda geração não contém o texto integral.
- **Fora de escopo:** —
- **Depende de:** MT-20, MT-39, MT-51

<a id="mt-58"></a>
### MT-58 — Mostrar a estimativa antes do envio e registrar os tokens (M-10, Q-06, Q-10)
- **Objetivo:** estimativa com "≈", aviso quando exceder o contexto, registro por documento
  (real ou estimado).
- **Arquivos no escopo:** `web/src/lib/components/TokenEstimate.svelte`, `core/src/tokens.rs`,
  testes
- **Critério de aceite:** teste de componente + teste unitário do registro.
- **Fora de escopo:** tokenizadores de provedor.
- **Depende de:** MT-21, MT-51

<a id="mt-59"></a>
### MT-59 — Gerar o formulário a partir da skill (S-03)
- **Objetivo:** formulário montado das entradas declaradas, sem código por skill; aviso de
  capacidade faltante (S-07).
- **Arquivos no escopo:** `web/src/lib/components/SkillForm.svelte`, testes
- **Critério de aceite:** teste de componente com duas skills de fixture.
- **Fora de escopo:** —
- **Depende de:** MT-18, MT-19, MT-22, MT-48

<a id="mt-60"></a>
### MT-60 — Gerar com progresso e cancelamento (P-15)
- **Objetivo:** progresso visível, cancelar descarta a parcial, aba fechada mantém o documento
  anterior.
- **Arquivos no escopo:** `web/src/lib/generation/`, `web/e2e/geracao.e2e.ts`
- **Critério de aceite:** e2e com provedor simulado lento e recarga no meio.
- **Fora de escopo:** —
- **Depende de:** MT-23, MT-49, MT-59

<a id="mt-61"></a>
### MT-61 — Pré-visualizar conteúdo rico com segurança (P-12)
- **Objetivo:** fórmulas, código com realce, imagens e tabelas sem executar conteúdo.
- **Arquivos no escopo:** conforme ADR-0008; `web/src/lib/components/preview/`, testes
- **Critério de aceite:** teste de componente com documento de fixture.
- **Fora de escopo:** Mermaid (MT-62).
- **Depende de:** MT-3, MT-18, MT-48

<a id="mt-62"></a>
### MT-62 — Normalizar, validar e desenhar Mermaid (CK-08 Mermaid, ADR-0005)
- **Objetivo:** pôr rótulos entre aspas, validar com `mermaid.parse`, renderizar com
  `htmlLabels: false` e `securityLevel: 'strict'`, carregado sob demanda; sem `foreignObject`.
- **Arquivos no escopo:** `web/src/lib/mermaid/`, testes
- **Critério de aceite:** testes com os 6 tipos, rótulo com `<`/`>` e diagrama inválido.
- **Fora de escopo:** PNG para DOCX (MT-73).
- **Depende de:** MT-61

<a id="mt-63"></a>
### MT-63 — Editar direto e mostrar o aviso de IA (P-05, P-16)
- **Objetivo:** editar qualquer campo sem IA; desfazer e refazer; aviso permanente na tela.
- **Arquivos no escopo:** `web/src/lib/components/editor/`, `web/e2e/edicao.e2e.ts`
- **Critério de aceite:** e2e de edição, desfazer e aviso visível.
- **Fora de escopo:** proposta da IA (MT-64).
- **Depende de:** MT-61

<a id="mt-64"></a>
### MT-64 — Propor alterações da IA com diff (P-04)
- **Objetivo:** pedido em linguagem natural vira proposta com diff por questão; aplicar só
  após aceite.
- **Arquivos no escopo:** `web/src/lib/components/ProposalDiff.svelte`, `web/e2e/proposta.e2e.ts`
- **Critério de aceite:** e2e de aceite e recusa.
- **Fora de escopo:** —
- **Depende de:** MT-15, MT-60, MT-63

<a id="mt-65"></a>
### MT-65 — Mostrar verificações, pontuação e conferência (P-07, P-08, P-11, S-06)
- **Objetivo:** achados por questão sem bloquear a edição; soma e divergência; marcar
  resposta como conferida.
- **Arquivos no escopo:** `web/src/lib/components/checks/`, testes
- **Critério de aceite:** teste de componente com fixtures de cada achado.
- **Fora de escopo:** —
- **Depende de:** MT-63

<a id="mt-66"></a>
### MT-66 — Usar o banco de questões (P-13, Q-05)
- **Objetivo:** salvar questão no banco, sugerir compatíveis antes de gerar, reutilizar em
  outro documento.
- **Arquivos no escopo:** `web/src/lib/components/bank/`, `web/e2e/banco.e2e.ts`
- **Critério de aceite:** e2e de salvar, sugerir e reutilizar.
- **Fora de escopo:** busca semântica.
- **Depende de:** MT-16, MT-41, MT-63

---

## Onda 7 — Exportação

<a id="mt-67"></a>
### MT-67 — Criar o módulo de exportação com Typst (ADR-0005)
- **Objetivo:** crate `app/export` e WASM próprio, com `typst`, `typst-pdf`, `mitex` e specs
  vendorizadas (licença e aviso de modificação), substituto do `xarrow` e fontes mínimas
  servidas como arquivos; gera um PDF de fixture.
- **Arquivos no escopo:** `Cargo.toml`, `app/export/`, `app/export-wasm/` (ou conforme
  ADR-0006), `docs/adr/0004-stack-e-layout.md` (layout), `AGENTS.md` (estrutura e comandos)
- **Critério de aceite:** `cargo test -p teacher-helper-export`; build WASM do módulo.
- **Fora de escopo:** layout final da prova.
- **Depende de:** MT-1, MT-6

<a id="mt-68"></a>
### MT-68 — Diagramar a prova no PDF (E-02, E-07, E-08)
- **Objetivo:** template Typst com o cabeçalho do documento (P-17), numeração, páginas, papel,
  margens, fonte, linhas de resposta, sem quebrar múltipla escolha.
- **Arquivos no escopo:** `app/export/templates/`, `app/export/src/pdf.rs`, fixtures
- **Critério de aceite:** testes de layout com fixtures.
- **Fora de escopo:** diagramas e imagens (MT-71).
- **Depende de:** MT-67, MT-80

<a id="mt-69"></a>
### MT-69 — Separar arquivos e apontar a questão com erro (E-06, E-10)
- **Objetivo:** prova e gabarito em arquivos separados, um por versão; falha aponta a
  questão e não gera arquivo parcial.
- **Arquivos no escopo:** `app/export/src/bundle.rs`, testes
- **Critério de aceite:** testes de nomes, conteúdos e fixture inválida.
- **Fora de escopo:** —
- **Depende de:** MT-13, MT-68

<a id="mt-70"></a>
### MT-70 — Validar PDF/UA no CI (E-09)
- **Objetivo:** PDF/UA-1 com idioma pt-BR e texto alternativo; veraPDF `ua1` no CI sobre as
  fixtures.
- **Arquivos no escopo:** `.github/workflows/ci.yml`, `app/export/tests/pdfua.rs`, `AGENTS.md`
- **Critério de aceite:** passo do CI aprova as fixtures; uma fixture sem texto alternativo
  falha.
- **Fora de escopo:** —
- **Depende de:** MT-68

<a id="mt-71"></a>
### MT-71 — Pôr diagramas e imagens no PDF (E-02, E-05)
- **Objetivo:** SVG vetorial do Mermaid e imagens com texto alternativo; regressão visual.
- **Arquivos no escopo:** `app/export/src/pdf.rs`, `web/src/lib/export/`, testes de
  regressão visual
- **Critério de aceite:** regressão visual com fixtures de diagrama e imagem.
- **Fora de escopo:** DOCX.
- **Depende de:** MT-62, MT-68

<a id="mt-72"></a>
### MT-72 — Converter fórmulas para equação do Word (E-12)
- **Objetivo:** MathML → OMML em Rust, sem operadores invisíveis; fórmula fora do subconjunto
  vira imagem com LaTeX como texto alternativo e gera aviso com a questão.
- **Arquivos no escopo:** `app/export/src/omml.rs`, testes
- **Critério de aceite:** `cargo test -p teacher-helper-export omml` com as 12 fórmulas do
  spike e uma fora do subconjunto.
- **Fora de escopo:** montar o DOCX (MT-73).
- **Depende de:** MT-67

<a id="mt-73"></a>
### MT-73 — Gerar o DOCX e validá-lo no LibreOffice (E-03, E-05)
- **Objetivo:** DOCX válido com equações editáveis e diagramas como PNG com texto
  alternativo; LibreOffice *headless* no CI.
- **Arquivos no escopo:** `app/export/src/docx.rs`, `.github/workflows/ci.yml`, testes
- **Critério de aceite:** validação OOXML + abertura no LibreOffice no CI.
- **Fora de escopo:** Word e Google Docs (roteiro manual, MT-79).
- **Depende de:** MT-71, MT-72

<a id="mt-74"></a>
### MT-74 — Exportar Markdown (E-04)
- **Objetivo:** GFM com `$…$`, blocos `mermaid`, gabarito separado e `.zip` quando houver
  imagens.
- **Arquivos no escopo:** `core/src/export/markdown.rs`, testes
- **Critério de aceite:** teste de ida e volta do `.md`.
- **Fora de escopo:** —
- **Depende de:** MT-12

<a id="mt-75"></a>
### MT-75 — Carregar a exportação sob demanda e medir o tempo (E-01, E-11)
- **Objetivo:** baixar o módulo e as fontes só da própria origem na primeira exportação;
  20 questões com fórmulas em até 10 s.
- **Arquivos no escopo:** `web/src/lib/export/loader.ts`, `web/e2e/exportacao-rede.e2e.ts`,
  benchmark no CI
- **Critério de aceite:** e2e com a guarda de rede da MT-45; benchmark calibrado ao runner.
- **Fora de escopo:** reduzir o WASM (seguimento do ADR-0005).
- **Depende de:** MT-45, MT-67

<a id="mt-76"></a>
### MT-76 — Criar a tela de exportação (P-08, P-09, P-16)
- **Objetivo:** escolher formatos e número de versões, opções de layout, aviso de gabarito não
  conferido; sem o aviso de IA nos arquivos.
- **Arquivos no escopo:** `web/src/lib/components/export/`, `web/e2e/exportar.e2e.ts`
- **Critério de aceite:** e2e com os três formatos.
- **Fora de escopo:** —
- **Depende de:** MT-65, MT-69, MT-73, MT-74, MT-75

---

## Onda 8 — Fechamento do MVP

<a id="mt-77"></a>
### MT-77 — Provar a resistência a XSS (T-10)
- **Objetivo:** fixtures maliciosas (HTML, SVG, Markdown, LaTeX, Mermaid) passam por
  pré-visualização, banco e exportação sem executar nada.
- **Arquivos no escopo:** `web/src/lib/security/xss.spec.ts`, `web/e2e/xss.e2e.ts`, fixtures
- **Critério de aceite:** os testes passam e nenhum alerta ou violação de CSP aparece.
- **Fora de escopo:** —
- **Depende de:** MT-62, MT-66, MT-76

<a id="mt-78"></a>
### MT-78 — Testar o fluxo completo sem IA onde ela não é necessária (Q-01)
- **Objetivo:** e2e de ponta a ponta (conectar, anexar, gerar, editar, versões, exportar) com
  provedor simulado que falha se chamado em etapa que não deve usar IA.
- **Arquivos no escopo:** `web/e2e/fluxo-completo.e2e.ts`
- **Critério de aceite:** o e2e passa no CI.
- **Fora de escopo:** —
- **Depende de:** MT-76

<a id="mt-79"></a>
### MT-79 — Escrever os roteiros manuais de release
- **Objetivo:** roteiros para Word, Google Docs, Ollama, llama.cpp e qualidade das skills com
  IA real, com tabela de registro por release.
- **Arquivos no escopo:** `docs/roteiros/`
- **Critério de aceite:** roteiros executados uma vez pelo mantenedor, com o resultado
  registrado.
- **Fora de escopo:** automação.
- **Depende de:** MT-76

---

## Acréscimos do MT-5

<a id="mt-80"></a>
### MT-80 — Gerenciar modelos de cabeçalho (P-17)
- **Objetivo:** criar, editar, duplicar e excluir modelos de cabeçalho (campos, ordem, valores
  fixos, logotipo); escolher um por documento e um padrão; alterar o modelo não muda
  documentos existentes sem pedido do professor.
- **Arquivos no escopo:** `core/src/document/header.rs`, `core/tests/header.rs`,
  `web/src/lib/components/header-templates/`, `web/e2e/cabecalho.e2e.ts`
- **Critério de aceite:** `cargo test -p teacher-helper-core header` + e2e de criar, aplicar e
  alterar.
- **Fora de escopo:** diagramação no PDF (MT-68).
- **Depende de:** MT-6, MT-41, MT-48

<a id="mt-81"></a>
### MT-81 — Escolher o uso das imagens dos materiais (M-14)
- **Objetivo:** listar as imagens de PDF e DOCX de entrada e deixar o professor marcar cada
  uma como "usar no documento" (com texto alternativo obrigatório) ou "só referência" (padrão).
- **Arquivos no escopo:** conforme ADR-0007; `web/src/lib/materials/images.ts`,
  `web/src/lib/components/materials/ImageChoice.svelte`, `web/e2e/imagens-material.e2e.ts`
- **Critério de aceite:** teste com PDF e DOCX de fixture contendo imagens + e2e da escolha.
- **Fora de escopo:** edição de imagem.
- **Depende de:** MT-53, MT-54
