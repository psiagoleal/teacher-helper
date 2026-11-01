<!-- Caminho relativo: docs/adr/0001-arquitetura-estatica-sem-servidor.md -->

# ADR 0001: Aplicação estática no navegador, sem backend do projeto

- **Status:** Proposed
- **Data:** 2026-10-31
- **Decisores:** Iago Leal
- **Tags:** arquitetura, hospedagem, privacidade

## Contexto

O pedido é uma aplicação sem conta, com dados temporários, hospedada no GitHub Pages. O
GitHub Pages só serve arquivos estáticos. Um backend teria custo, exigiria operação e
receberia material de professores (potencialmente com dados de alunos). A IA é do próprio
usuário, chamada a partir do navegador (ADR-0002).

Alternativas consideradas:

1. **Estático + núcleo Rust/WASM (escolhida).** Zero custo de hospedagem, privacidade por
   construção, Rust reaproveitável depois na ponte para CLI (fase 2).
2. **SvelteKit com backend Rust (Axum) em container.** Resolve CORS de links e permitiria
   proxy, mas exige hospedagem paga e passa a custodiar dados do professor.
3. **Estático só com TypeScript.** Mais simples, mas perde o desempenho do Rust no
   processamento pesado (geração de PDF, parsing) e o ecossistema Typst.

## Decisão

O teacher-helper é uma aplicação **SvelteKit (Svelte 5) com `adapter-static`**, servida pelo
GitHub Pages, com o processamento pesado (modelo de documento, validação, geração de
PDF/DOCX, parsing de entradas) em **Rust compilado para WebAssembly**. **Não há backend do
projeto.** Persistência apenas no IndexedDB do navegador.

## Consequências

- **Impacto positivo:** custo zero de hospedagem; nenhum dado do professor passa por servidor
  do projeto; funciona offline após o carregamento (exceto a chamada à IA na nuvem).
- **Impacto negativo:** leitura de links limitada por CORS; tamanho do WASM e de componentes
  opcionais exige carregamento sob demanda; sem sincronização entre dispositivos.
- **Trade-offs aceitos:** links bloqueados por CORS caem no fallback "cole o texto"
  (spec 02, M-08).
- **Limite do GitHub Pages:** não permite definir cabeçalhos HTTP. Sem COOP/COEP não há
  `SharedArrayBuffer`, então o WASM deve rodar sem threads (ou com contorno via service
  worker, decidido em ADR próprio se necessário). A CSP é declarada por `<meta>`.

## Diretriz de Conformidade de Código

- Proibido: introduzir endpoint, função serverless ou proxy mantido pelo projeto sem ADR que
  substitua este.
- Proibido: requisição de rede fora da lista de origens da spec 00, T-02 — própria origem;
  provedor ou servidor de IA configurado pelo professor; GET de link informado pelo
  professor, sem enviar outros dados; origem fixa de pesos do modelo local, só a pedido.
- Obrigatório: o build de produção é um diretório de arquivos estáticos publicável no GitHub Pages.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
