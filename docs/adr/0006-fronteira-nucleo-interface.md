<!-- Caminho relativo: docs/adr/0006-fronteira-nucleo-interface.md -->

# ADR 0006: Fronteira entre núcleo Rust e interface, com sessão de documento por comandos

- **Status:** Accepted (2026-11-01)
- **Data:** 2026-11-01
- **Decisores:** Iago Leal
- **Tags:** arquitetura, wasm, fronteira, rede

## Contexto

O ADR-0003 manda pôr as regras de negócio num núcleo compartilhado (modelo de documento,
skills, validação, exportação, conectores de IA), sem APIs do navegador, e acessar rede,
armazenamento, arquivos e processos por uma **interface de plataforma**. Ele não diz onde
passa a linha entre Rust e TypeScript, como o núcleo usa a rede, nem quem guarda o documento
aberto. Sem isso, nenhum ticket de código do MVP tem escopo fechado (plano, MT-1).

Forças em jogo:

- **Sem threads no WASM.** O GitHub Pages não envia COOP/COEP, então não há
  `SharedArrayBuffer` (ADR-0001). Trabalho pesado na thread principal trava a tela.
- **Desktop futuro (Tauri).** Lá o núcleo roda nativo; tudo que ele precisar do ambiente tem
  de ter uma versão nativa.
- **Mesmas operações para professor e IA.** Desfazer (P-14), diff de proposta (P-04) e
  ferramentas da IA (P-03) mexem no mesmo documento e precisam do mesmo registro de mudanças.
- **Respostas em *streaming* e cancelamento** (P-15), com classificação de erros no núcleo
  (C-08 a C-10).
- **O JSON Schema do documento já é obrigatório** (saída estruturada da IA, C-17).
- **Algumas peças só existem em JS:** Mermaid (ADR-0005), possivelmente extração de PDF e OCR
  (MT-2) e o modelo local (MT-4).

Alternativas consideradas:

1. **Núcleo sem estado; o documento vive no TypeScript.** Cada operação recebe e devolve o
   documento inteiro. Simples de testar, mas o histórico de desfazer e o registro de quem
   mudou o quê ficam espalhados entre as duas linguagens, e o desktop teria de reimplementar
   isso.
2. **Sessão de documento no núcleo, alterada por comandos (escolhida).** O núcleo guarda o
   documento aberto e o histórico; a interface envia comandos e recebe instantâneos. As
   ferramentas da IA são os mesmos comandos.
3. **Tudo em Rust, inclusive a interface** (Leptos, Dioxus). Contradiz o ADR-0004
   (SvelteKit) e afasta contribuidores de front-end.
4. **Troca de dados por `serde-wasm-bindgen` (objetos JS)** em vez de texto JSON. Um pouco
   mais rápido, mas mais difícil de inspecionar e de reproduzir em teste; o documento é
   pequeno (20 questões ≈ dezenas de KB), então o ganho não compensa.
5. **Tipos TypeScript escritos à mão.** Divergem do Rust sem aviso.

## Decisão

Fica acordado o seguinte:

1. **Divisão de responsabilidades.**

   | Núcleo Rust (`app/core`, sem APIs do navegador) | Casca TypeScript (`app/web`) |
   |---|---|
   | modelo do documento, validação, migração | telas, estado reativo de exibição, textos pt-BR |
   | sessão de documento, comandos, histórico, diff | armazenamento (IndexedDB), trava entre abas (Web Locks) |
   | verificações CK, pontuação, gabarito, versões | fluxo OAuth com redirecionamento (C-01) |
   | skills: análise, montagem do pedido, ciclo de resposta | leitura de arquivos escolhidos (entrega bytes ao núcleo) |
   | conectores de IA, *streaming*, classificação de erros | Mermaid e demais bibliotecas que só existem em JS |
   | detector de dados pessoais, estimativa de tokens | implementação web da interface de plataforma |
   | exportação (crate separado, item 6) | |

   Armazenamento fica **fora** do núcleo: ele recebe e devolve dados, nunca lê nem grava. A
   casca guarda por uma interface TypeScript própria (`Storage`), que no desktop pode trocar
   de implementação sem mudar o núcleo.

2. **Interface de plataforma.** O núcleo declara traits; `app/wasm` as implementa com
   `web-sys`, e o desktop as implementará nativamente:
   - `Http`: envia requisição e devolve o corpo como sequência assíncrona de pedaços de
     bytes, com cancelamento (na web, `fetch` + `AbortController`).
   - `ExternalModel`: gera texto por um modelo implementado fora do Rust (modelo local em JS,
     MT-4), visto pelo núcleo como mais um provedor.

   **Aleatoriedade e relógio não são traits:** toda função que precisa deles recebe a semente
   ou o instante como parâmetro, e a casca os obtém (`crypto.getRandomValues`, `Date.now`).
   Assim o núcleo é determinístico e testável.

3. **Sessão de documento por comandos.** O documento aberto vive numa `DocumentSession` do
   núcleo. Toda mudança — do professor, da IA ou de uma skill — é um **comando** (enum
   serializável com origem). A sessão aplica o comando, guarda o histórico (P-14) e devolve
   um **instantâneo** do documento com os achados das verificações. Propostas da IA são lotes
   de comandos ainda não aplicados, mostrados como diff (P-04). As ferramentas da IA (P-03)
   são esses mesmos comandos, descritos no JSON Schema.

4. **Formato na fronteira.** Texto JSON (`serde_json`) em ambas as direções; bytes de arquivo
   como `Uint8Array`. O **JSON Schema** é gerado dos tipos Rust (`schemars`) e serve a três
   usos: validar respostas da IA, saída estruturada dos provedores e gerar os tipos
   TypeScript no build (`json-schema-to-typescript`). Os tipos TypeScript gerados não são
   editados à mão.

5. **Erros.** O núcleo devolve erros como **código + parâmetros** (ex.:
   `{"code": "provider.rate_limited", "retry_after_s": 30}`), nunca frases. As mensagens em
   pt-BR ficam na casca, junto dos demais textos. Nenhum erro carrega a chave de IA (C-11).

6. **Módulos WASM e threads.**
   - **Núcleo leve** (`app/core` + `app/wasm`): carregado com a página, roda na thread
     principal. Teto inicial: **600 KB gzip**, medido no CI junto com o T-08 (MT-46).
   - **Exportação** (`app/export` + `app/export-wasm`, ADR-0005): carregada sob demanda e
     executada num **Web Worker**, com as fontes como arquivos próprios.
   - Tarefas pesadas de JS (OCR, modelo local) também rodam em Web Worker.
   - Nenhum dos dois depende de threads WASM.

## Consequências

- **Impacto positivo:** desfazer, diff de proposta e ferramentas da IA usam um único
  mecanismo; o núcleo é testável em `cargo test` sem navegador; o desktop reaproveita sessão,
  conectores e exportação e só reescreve a casca de plataforma; os tipos TypeScript não
  divergem do Rust.
- **Impacto negativo:** cada operação serializa o documento em JSON pela fronteira; a
  interface precisa tratar o instantâneo como somente leitura; uma etapa a mais no build
  (geração de tipos).
- **Trade-offs aceitos:** o estado do documento aberto existe duas vezes (sessão no núcleo e
  cópia de exibição na casca), com a sessão como fonte da verdade; o teto de 600 KB do núcleo
  leve é estimativa inicial, revista com medição.

## Diretriz de Conformidade de Código

- Obrigatório: toda mudança de documento passa por um comando da `DocumentSession`; a casca
  não altera o documento diretamente.
- Obrigatório: o núcleo recebe semente e instante como parâmetro; não lê relógio nem gera
  entropia sozinho.
- Obrigatório: rede do núcleo só pela trait `Http`; os conectores de IA não chamam `fetch`
  nem `web-sys`.
- Obrigatório: tipos TypeScript da fronteira gerados do JSON Schema no build.
- Obrigatório: erros do núcleo como código + parâmetros; frases só na casca.
- Proibido: o núcleo acessar armazenamento, arquivos ou APIs do navegador.
- Proibido: executar a exportação ou o OCR na thread principal.
- Proibido: depender de threads WASM ou `SharedArrayBuffer`.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
