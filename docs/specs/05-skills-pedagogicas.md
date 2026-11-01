<!-- Caminho relativo: docs/specs/05-skills-pedagogicas.md -->

# Spec 05 — Skills pedagógicas

- **Status:** Aprovada (2026-11-01)
- **Fase:** MVP (skills criadas pelo professor: fase seguinte)
- **Cita:** `00-transversal.md`, `01-conexao-ia.md`, `03-provas-e-listas.md`,
  `06-economia-de-cota.md`

## Problema e resultado esperado

A qualidade do que a IA genérica produz depende de o professor saber escrever um bom
pedido. As skills embutem esse conhecimento pedagógico: o professor preenche um formulário
curto e a skill monta o pedido, define o formato da resposta e confere a qualidade do
resultado.

## O que é uma skill

Um arquivo declarativo em `content/skills/` (Markdown com cabeçalho YAML) contendo:
metadados (nome, descrição, versão, tipos de documento), entradas do formulário, instruções
para a IA, esquema da resposta, capacidades exigidas (catálogo da spec 01) e verificações
de qualidade (catálogo abaixo).

## Skills do MVP

1. Gerar prova
2. Gerar lista de exercícios
3. Revisar prova (ambiguidade, mais de uma resposta correta, enunciado que entrega a resposta,
   nível inadequado)
4. Criar distratores plausíveis
5. Ajustar dificuldade de uma questão

## Catálogo de verificações por código

| ID | Verificação |
|---|---|
| CK-01 | Gabarito de objetiva existe e aponta alternativa/item existente |
| CK-02 | Múltipla escolha tem exatamente uma alternativa correta |
| CK-03 | Alternativas distintas entre si (comparação normalizada) |
| CK-04 | Sem referência posicional em questão `embaralhavel` |
| CK-05 | Alternativa correta com comprimento acima de 1,5× a média das demais (alerta de pista) |
| CK-06 | Soma da pontuação igual ao total definido |
| CK-07 | Campos obrigatórios não vazios (enunciado, alternativas, critério de discursiva) |
| CK-08 | LaTeX e Mermaid analisam sem erro |
| CK-09 | Associação de colunas com correspondência completa e sem repetição |

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| S-01 | Toda skill DEVE validar contra o esquema de skill versionado. | validador de skills no CI sobre `content/skills/` |
| S-02 | SE um arquivo de skill for inválido, ENTÃO o build DEVE falhar apontando arquivo e campo. | teste do validador com fixtures inválidas |
| S-03 | O app DEVE gerar o formulário da skill a partir das entradas declaradas, sem código específico por skill. | teste de componente com duas skills de fixture |
| S-04 | QUANDO uma skill for executada, o pedido enviado à IA DEVE conter apenas as instruções daquela skill. | teste unitário da montagem do pedido |
| S-05 | O app DEVE validar a resposta contra o esquema declarado pela skill (aplica P-01/P-02). | teste unitário |
| S-06 | QUANDO a resposta for válida, o app DEVE executar as verificações declaradas pela skill (subconjunto de CK-01 a CK-09) e exibir as que falharem, por questão. | teste unitário com resposta de fixture que viola cada verificação |
| S-07 | SE o modelo conectado não tiver uma capacidade exigida pela skill, ENTÃO o app DEVE avisar antes de executar e sugerir alternativa. | teste unitário da matriz de capacidades |
| S-08 | Todo documento DEVE registrar quais skills e versões o produziram ou alteraram; QUANDO uma skill registrada não existir mais, o documento DEVE abrir normalmente. | teste unitário com documento de fixture referindo skill removida |
| S-09 | O guia de contribuição DEVE conter uma skill de exemplo completa que o validador do CI aceita. | o exemplo do guia é validado no CI |

## Fora de escopo do MVP

- Skills: gabarito comentado, alinhamento à BNCC, importar prova antiga para o banco — fase
  seguinte.
- Criação ou edição de skills pelo professor dentro do app (formato já aberto para isso).
- Catálogo de skills de terceiros carregadas da internet.
- Skills de planos de aula e slides — entram com as specs dessas fases.
