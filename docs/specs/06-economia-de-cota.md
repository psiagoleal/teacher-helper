<!-- Caminho relativo: docs/specs/06-economia-de-cota.md -->

# Spec 06 — Economia de cota da IA

- **Status:** Aprovada (2026-11-01)
- **Fase:** MVP
- **Cita:** `00-transversal.md`, `01-conexao-ia.md`, `03-provas-e-listas.md`

## Problema e resultado esperado

A cota é do professor — gratuita e limitada, ou paga por uso. O app gasta IA **só onde ela é
insubstituível**: gerar e revisar conteúdo. Todo o resto é código.

## Definições

- **Estimativa de tokens:** contagem pelo tokenizador do provedor QUANDO houver um disponível
  no navegador; caso contrário, número de caracteres ÷ 4, sempre exibido com "≈".
- **Contexto mínimo de edição:** instruções da skill + a questão alvo + tema e nível da seção
  + resumo do perfil de estilo. **Não** inclui outras questões nem o texto integral dos
  materiais.
- **Questão compatível (banco):** mesmo tema (comparação normalizada), mesmo nível e
  dificuldade a no máximo 1 de distância.

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| Q-01 | Embaralhar versões, numerar, somar pontuação, montar gabarito de objetivas, executar verificações do catálogo e exportar DEVEM ocorrer sem chamada à IA. | teste com provedor simulado que falha se chamado |
| Q-02 | O app DEVE enviar à IA o texto extraído localmente dos materiais (e imagens só quando necessárias, spec 02), nunca o arquivo PDF/DOCX original. | teste unitário da montagem do pedido |
| Q-03 | QUANDO um material for usado pela primeira vez, o app DEVE produzir e guardar resumo e perfil de estilo; nas gerações seguintes, DEVE enviar o resumo, exceto ONDE o professor marcar "usar material completo". | teste unitário: segunda geração não contém o texto integral |
| Q-04 | QUANDO o professor alterar uma única questão, o pedido DEVE conter apenas o contexto mínimo de edição e somar no máximo 10 % da estimativa de tokens de entrada da geração completa do mesmo documento. | teste com documento de fixture de 20 questões |
| Q-05 | ANTES de gerar novas questões, o app DEVE sugerir questões compatíveis do banco local, sem usar IA na busca. | teste unitário da busca |
| Q-06 | O app DEVE exibir a estimativa de tokens de entrada antes de enviar cada pedido. | teste de componente |
| Q-07 | Pedidos da mesma skill DEVEM começar por prefixo idêntico (instruções e esquema), seguido do conteúdo variável. | teste unitário: prefixo byte a byte igual entre duas execuções |
| Q-08 | ONDE o provedor suportar cache de prompt explícito, o conector DEVE marcá-lo no prefixo estável. | teste de contrato do conector |
| Q-09 | ONDE o professor configurar um "modelo rápido" além do principal, o app DEVE usá-lo para extração, classificação e resumo, e o principal para geração e revisão. | teste unitário de roteamento |
| Q-10 | O app DEVE registrar localmente os tokens por documento — reais quando o provedor os informar, estimados caso contrário — e exibi-los ao professor. | teste unitário |

## Fora de escopo do MVP

- Busca semântica no banco de questões com embeddings locais.
- Escolha automática de modelo por custo.
