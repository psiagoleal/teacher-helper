<!-- Caminho relativo: docs/adr/0005-formulas-diagramas-e-motor-de-pdf.md -->

# ADR 0005: Fórmulas em LaTeX, motor de PDF Typst e diagramas Mermaid em SVG puro

- **Status:** Proposed
- **Data:** 2026-11-01
- **Decisores:** Iago Leal
- **Tags:** exportação, fórmulas, diagramas, licenças
- **Origem:** spike PRE-5 (código de reprodução em `docs/spikes/pre-5/`)

## Contexto

A spec 04 exige PDF fiel e acessível, DOCX com fórmulas editáveis e diagramas Mermaid nos
dois formatos, tudo gerado no navegador. O briefing fixou LaTeX como linguagem de fórmulas
da IA. Faltava provar que isso é viável sem servidor, e a que custo.

## O que o spike mediu (2026-11-01)

| Pergunta | Resultado |
|---|---|
| LaTeX → Typst | `mitex` 0.2.4 converteu 11/11 fórmulas de teste (fração, raiz, integral, somatório, matriz, limite, vetor, química, casos, `\text` com acento, `\xrightarrow`). |
| Typst compila em WASM? | Sim, `typst` 0.15.1 + `typst-pdf` para `wasm32-unknown-unknown` sem adaptação. |
| Tamanho do módulo | 23,6 MB otimizado / **8,9 MB gzip** sem fontes; fontes mínimas (NewCM 10 regular/itálico/negrito, NewCM Math, DejaVu Sans Mono) **2,3 MB gzip**. Com todas as fontes embutidas: 15,3 MB gzip. |
| Desempenho (WASM em Node) | inicialização 52 ms; prova de 20 questões: **268 ms** na 1ª compilação, ~15 ms nas seguintes. Nativo: 125 ms. |
| PDF/UA | Validador do Typst exige texto alternativo em cada fórmula; com ele, **veraPDF aprovou PDF/UA-1** (prova com fórmulas e prova com fluxograma). |
| LaTeX → equação do Word | `math-core` 0.9 (MIT) gerou MathML para 12/12 fórmulas; protótipo próprio MathML → OMML (~150 linhas) gerou DOCX que o **LibreOffice abriu com 12/12 equações editáveis**. Pendências de acabamento: remover operadores invisíveis (U+2061–2064) e estilo upright de `\mathrm`. Word não testado (sem Word no ambiente). |
| Mermaid sem `foreignObject` | `mermaid` 12.2 com `htmlLabels: false`: fluxograma, sequência, mapa mental, pizza, classes e estados com **0** `foreignObject`; todos entram no PDF via Typst. `mermaid.parse` rejeita diagrama inválido. Biblioteca: 1,6 MB gzip. |
| Defeito encontrado | Com rótulos em SVG puro, o Mermaid **descarta `<` e `>` de rótulo sem aspas**; com rótulo entre aspas (`B{"x > 0?"}`) os sinais se preservam. |
| Licenças | `mitex` (crate e specs Typst): Apache-2.0. O pacote Typst `xarrow`, importado pelas specs do mitex, é **GPL-3.0** — substituído por implementação própria de 2 linhas com `math.stretch`. |

## Decisão

1. **Fórmulas:** a IA escreve LaTeX (subconjunto suportado pelo `mitex` e pelo `math-core`;
   lista oficial derivada dos testes de fixture). Cada fórmula leva o LaTeX original como
   texto alternativo.
2. **PDF:** Typst (`typst`, `typst-pdf`) no núcleo Rust, LaTeX convertido por `mitex`,
   exportação **PDF/UA-1**. Specs Typst do mitex vendorizadas no repositório com a licença
   Apache-2.0 e aviso de modificação; `xarrow` substituído por código próprio.
3. **DOCX:** LaTeX → MathML (`math-core`) → **OMML por conversor próprio em Rust** no núcleo,
   para o subconjunto suportado. **SE** uma fórmula não converter, **ENTÃO** ela vai como imagem
   com o LaTeX no texto alternativo, e o professor é avisado.
4. **Diagramas:** Mermaid renderizado no navegador com `htmlLabels: false` e `securityLevel:
   'strict'`, carregado sob demanda; o app **normaliza todo rótulo para a forma entre aspas**
   antes de renderizar, e a skill instrui a IA a usá-la. No PDF, SVG vetorial; no DOCX, PNG
   gerado do SVG no navegador.
5. **Módulo de exportação** é um WASM separado do núcleo leve, baixado na primeira exportação
   (ou em segundo plano após a carga da página), com fontes servidas como arquivos próprios.

## Consequências

- **Impacto positivo:** PDF acessível e fiel gerado no navegador em menos de 0,3 s; DOCX com
  equações editáveis sem dependência LGPL/GPL; um só caminho de conversão para todos os
  formatos.
- **Impacto negativo:** ~11 MB gzip baixados na primeira exportação, mais 1,6 MB do Mermaid
  quando houver diagrama. Conversor MathML → OMML é código nosso a manter.
- **Trade-offs aceitos:** formatos fora do subconjunto viram imagem no DOCX; Word e Google Docs
  validados por roteiro manual a cada release (spec 04, E-03).
- **A investigar depois:** reduzir o WASM do Typst (análise com `twiggy`, remover recursos não
  usados) e compatibilidade dos specs do mitex com versões futuras do Typst (já houve uma
  quebra: o símbolo `kai` saiu no Typst 0.15).

## Diretriz de Conformidade de Código

- Obrigatório: toda fórmula no PDF tem texto alternativo; o build de CI valida as fixtures com
  veraPDF (perfil `ua1`).
- Obrigatório: rótulos Mermaid entre aspas antes da renderização; nenhum `foreignObject` no
  SVG que vai para o Typst.
- Proibido: dependência GPL ou LGPL embutida no app (verificar licenças de pacotes Typst e npm
  antes de vendorizar).
- Proibido: buscar pacotes Typst na internet em tempo de execução (`@preview/...`); tudo é
  vendorizado e servido da própria origem.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
