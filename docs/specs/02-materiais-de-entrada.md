<!-- Caminho relativo: docs/specs/02-materiais-de-entrada.md -->

# Spec 02 — Materiais de entrada

- **Status:** Aprovada (2026-11-01)
- **Fase:** MVP
- **Cita:** `00-transversal.md`, `01-conexao-ia.md`

## Problema e resultado esperado

O professor já tem material: provas antigas, apostilas, fotos de livro, listas de tópicos.
Ele quer dizer "faça uma prova parecida com esta, sobre este conteúdo" e anexar o que tem.
O app transforma cada material em conteúdo que a IA consegue usar, **no navegador**, e mostra
ao professor o que foi extraído.

## Papéis de um material

Cada material anexado recebe um papel, escolhido pelo professor (com sugestão do app):
**Conteúdo** (o assunto a cobrar), **Modelo de estilo** (formato, tom, tipo de questão a
imitar) ou **Ambos**.

## Limites de referência

| Nome | Valor inicial |
|---|---|
| Tamanho máximo por arquivo | 25 MB |
| Tamanho máximo de materiais por projeto | 100 MB |
| Página considerada escaneada | menos de 50 caracteres de texto extraído |

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| M-01 | QUANDO o professor anexar PDF, o app DEVE extrair o texto no navegador e exibi-lo para conferência. | teste unitário com PDFs de fixture |
| M-02 | QUANDO uma página de PDF ficar abaixo do limite de página escaneada, o app DEVE tratá-la como imagem (M-04/M-05). | teste unitário com PDF misto (texto + página escaneada) |
| M-03 | QUANDO o professor anexar DOCX, o app DEVE extrair texto, listas, tabelas e fórmulas no navegador. | teste unitário com DOCX de fixture contendo equação |
| M-04 | QUANDO houver imagem ou página escaneada, ENQUANTO o modelo conectado tiver a capacidade `visao`, o app DEVE enviar a imagem ao modelo. | teste unitário da montagem do pedido |
| M-05 | QUANDO houver imagem ou página escaneada, ENQUANTO o modelo NÃO tiver `visao`, o app DEVE oferecer OCR local e exibir o texto reconhecido para correção antes do uso. | e2e com fixture de imagem |
| M-06 | O app DEVE aceitar texto colado e lista de tópicos como material. | e2e |
| M-07 | QUANDO o professor informar um link de página, o app DEVE tentar obtê-lo com um GET direto do navegador, sem enviar outros dados. | teste unitário com fetch simulado |
| M-08 | SE o link não puder ser lido (CORS, login, paywall), ENTÃO o app DEVE explicar em linguagem simples e pedir que o professor cole o texto ou envie o arquivo. | teste unitário com fetch rejeitado |
| M-09 | ONDE o modelo tiver a capacidade `video_por_url`, o app DEVE enviar o link de vídeo ao modelo; nos demais, DEVE pedir ao professor a transcrição ou um resumo. | teste unitário da matriz de capacidades |
| M-10 | O app DEVE mostrar a estimativa de tamanho do pedido (método da spec 06) e, SE ela exceder o contexto do modelo, ENTÃO DEVE avisar antes de enviar e sugerir remover ou resumir materiais. | teste unitário com materiais grandes |
| M-11 | SE um arquivo estiver corrompido, protegido por senha ou em formato não suportado, ENTÃO o app DEVE recusá-lo com o motivo e manter os demais materiais. | testes unitários com fixtures inválidas |
| M-12 | O app DEVE recusar arquivo ou conjunto acima dos limites de referência, informando o limite. | teste unitário |
| M-13 | Todo texto extraído DEVE passar pelo detector de dados pessoais (spec 00, T-15) antes de qualquer envio. | teste unitário |

## Fora de escopo do MVP

- Transcrição de áudio/vídeo no navegador.
- Rastreamento de páginas (seguir links a partir de um link).
- Leitura de Google Docs/Drive por integração (o professor exporta e anexa).
- Formatos além de PDF, DOCX, imagem (PNG/JPG/WebP) e texto — ex.: PPTX, ODT.

## Questões em aberto

- Imagens e figuras dentro do PDF/DOCX de entrada: extrair para reaproveitar no documento
  gerado, ou só como referência para a IA?
