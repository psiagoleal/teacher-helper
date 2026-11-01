<!-- Caminho relativo: docs/specs/03-provas-e-listas.md -->

# Spec 03 — Geração de provas e listas de exercícios

- **Status:** Aprovada (2026-11-01)
- **Fase:** MVP
- **Cita:** `00-transversal.md`, `01-conexao-ia.md`, `02-materiais-de-entrada.md`,
  `05-skills-pedagogicas.md`

## Problema e resultado esperado

O professor descreve o que quer (série/curso, conteúdo, número e tipo de questões,
dificuldade, valor), anexa materiais e recebe uma prova ou lista **estruturada**, que ele
revisa, ajusta com a ajuda da IA e exporta (spec 04).

A IA trabalha sobre um **documento estruturado**, usando **ferramentas** definidas pelo app.
Isso permite validar o resultado, editar uma questão sem regerar tudo e produzir variações
por código.

## Modelo do documento

- **Documento:** versão do esquema, tipo (prova | lista), cabeçalho, seções, pontuação total
  desejada, skills e versões que o produziram (spec 05, S-08).
- **Questão:** tipo, enunciado, alternativas ou itens, resposta, critério de correção
  (discursivas), pontos, **tema**, **nível** (etapa e ano/período), **dificuldade** (1 a 3),
  `embaralhavel` (sim/não), estado do gabarito (`nao_conferido` | `conferido`).
- **Conteúdo rico** em qualquer campo de texto: fórmula em LaTeX (subconjunto definido pelas fixtures de teste — ADR-0005),
  código com linguagem declarada, imagem com texto alternativo, tabela e **diagrama Mermaid**
  (tipos suportados no MVP: fluxograma, sequência, mapa mental, pizza, classes e estados; rótulos sempre entre aspas — ADR-0005).
- **Tipos de questão:** múltipla escolha (uma correta), verdadeiro/falso, associação de
  colunas, lacunas, discursiva curta e longa (espaço de resposta em linhas).
- **Linguagens de código com realce:** Python, C, C++, Java, JavaScript, TypeScript, Rust, SQL,
  HTML, CSS e pseudocódigo; outras aparecem sem realce.

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| P-01 | O documento DEVE obedecer ao esquema versionado, e o app DEVE rejeitar resposta da IA que não o obedeça. | teste unitário de validação com casos válidos e inválidos |
| P-02 | SE a resposta da IA for inválida (esquema, LaTeX ou Mermaid que não analisam), ENTÃO o app DEVE devolver os erros à IA e tentar corrigir no máximo 2 vezes antes de mostrar a falha ao professor. | teste unitário com provedor simulado retornando conteúdo inválido |
| P-03 | A IA DEVE dispor de ferramentas para: criar, editar, remover e reordenar questões; gerar e editar gabarito; ajustar pontuação; consultar os materiais anexados. | testes de contrato de cada ferramenta |
| P-04 | QUANDO o professor pedir alteração em linguagem natural num documento existente, o app DEVE apresentar as mudanças da IA como proposta (diff por questão) e aplicá-las só após aceite; elementos não afetados NÃO DEVEM mudar, exceto numeração e total de pontos, que são recalculados e aparecem no diff. | teste unitário: diff estrutural afeta só a questão alvo + numeração/total; e2e de aceite e recusa |
| P-05 | O professor DEVE poder editar diretamente qualquer campo de questão, alternativa e gabarito, sem usar a IA. | e2e |
| P-06 | O app DEVE gerar gabarito separado da prova, com a resposta de cada questão objetiva e o critério de correção de cada discursiva. | teste unitário |
| P-07 | O app DEVE executar por código, a cada mudança, as verificações de gabarito do catálogo (spec 05) e marcar a questão com erro sem bloquear a edição. | teste unitário com fixtures de gabarito errado (alternativa inexistente, duas corretas, V/F sem resposta, associação incompleta) |
| P-08 | Toda resposta produzida ou alterada pela IA DEVE nascer `nao_conferido`; QUANDO o professor exportar gabarito com respostas não conferidas, o app DEVE avisar e pedir confirmação. | teste unitário + e2e |
| P-09 | QUANDO o professor pedir N versões (A, B, …), o app DEVE, sem usar a IA, embaralhar a ordem das questões e das alternativas das questões `embaralhavel`, embaralhar a coluna B das associações mantendo o mapeamento, e gerar o gabarito de cada versão. | teste de propriedade: mesmo conjunto de questões e gabarito coerente em cada versão |
| P-10 | O esquema NÃO DEVE permitir referência posicional entre alternativas ("todas as anteriores", "alternativa B") em questão `embaralhavel`; a verificação do catálogo DEVE detectá-la. | teste unitário |
| P-11 | A soma da pontuação DEVE ser exibida e, SE divergir do total definido, ENTÃO o app DEVE sinalizar a divergência. | teste unitário |
| P-12 | O app DEVE renderizar fórmulas, código com realce, imagens, tabelas e diagramas Mermaid na pré-visualização, sem executar conteúdo (spec 00, T-10). | teste de componente com documento de fixture |
| P-13 | O professor DEVE poder salvar questões num banco local e reutilizá-las em outros documentos. | e2e |
| P-14 | O app DEVE permitir desfazer e refazer qualquer alteração, feita pelo professor ou pela IA. | teste unitário do histórico |
| P-15 | ENQUANTO a IA estiver gerando, o app DEVE mostrar o progresso e permitir cancelar; QUANDO a geração for cancelada ou a aba fechada, a resposta parcial DEVE ser descartada e o documento anterior mantido. | e2e com provedor simulado lento + recarga no meio |
| P-16 | O app DEVE exibir, na tela de edição, aviso permanente de que o conteúdo foi gerado por IA e precisa de revisão; o aviso NÃO DEVE aparecer nos arquivos exportados. | e2e + teste da exportação |

## Fora de escopo do MVP

- Alinhamento à BNCC (exige base de dados com fonte e licença definidas — fase seguinte).
- Correção automática de provas respondidas pelos alunos.
- Aplicação online da prova (formulário para aluno).
- Geração de imagens pela IA (o professor anexa imagens; diagramas via Mermaid).
- Questões adaptadas (acessibilidade) — candidata a fase seguinte.
- Planos de aula e slides/apostilas — fases seguintes, com spec própria.

## Questões em aberto

- Cabeçalho (escola, professor, aluno, data, nota): fixo no app ou template configurável?
