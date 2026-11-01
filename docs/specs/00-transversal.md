<!-- Caminho relativo: docs/specs/00-transversal.md -->

# Spec 00 — Requisitos transversais

- **Status:** Aprovada (2026-11-01)
- **Origem:** `docs/insumos/briefing.md`
- **Decisões citadas:** ADR-0001 (sem servidor), ADR-0002 (conexão com IA), ADR-0003 (web
  primeiro, desktop depois), ADR-0004 (stack)

Requisitos que valem para **todas** as features. As demais specs citam este documento em vez
de repeti-lo.

## Problema e resultado esperado

Professores gastam horas montando e formatando material. O teacher-helper permite que
o professor entregue exemplos e instruções, e uma IA **dele** produza o documento no formato
final — sem cadastro, sem servidor nosso, sem custo de hospedagem além do GitHub Pages.

## Proposta de valor

> **A IA gera o conteúdo; o app cuida de estrutura, formatação, memória e controle de
> qualidade.** É o que distingue o teacher-helper de usar um chat de IA diretamente.

| Usando a IA direto | No teacher-helper |
|---|---|
| Texto que precisa ser copiado e formatado no Word; fórmulas quebram | PDF diagramado, DOCX e Markdown prontos (spec 04) |
| "Mude a questão 3" pode reescrever tudo | Documento estruturado; só a parte alvo muda (spec 03) |
| Versões A/B e gabarito pedidos à IA, com custo e risco de erro | Feitos por código, sem IA (specs 03 e 06) |
| Contexto reexplicado a cada conversa | Cabeçalho, estilo e banco de questões guardados (spec 03) |
| Qualidade depende de saber escrever o pedido | Skills pedagógicas com checklist de qualidade (spec 05) |
| Preso a um provedor | Qualquer provedor, inclusive grátis ou local (spec 01) |

Toda feature nova deve reforçar pelo menos uma linha desta tabela; feature que só
reproduz um chat de IA não entra.

## Princípio: perguntar antes de agir

O app **nunca** envia, remove ou altera dados do professor sem confirmação explícita nos
casos abaixo — ele apresenta as opções e espera a escolha:

- primeiro envio de dados a cada provedor de IA (T-14);
- envio de material com possíveis dados pessoais (T-15);
- "Apagar tudo" (T-04);
- alterações propostas pela IA num documento existente (spec 03, P-04);
- nova tentativa de pedido que pode gerar cobrança em dobro (spec 01, C-09).

Gerar um documento **novo** a pedido do professor não exige confirmação adicional.

## Restrições

- Aplicação **estática** (SvelteKit + núcleo Rust compilado para WASM), servida pelo GitHub
  Pages. **Não existe backend do projeto** (ADR-0001).
- A IA é **sempre do usuário**: o projeto não tem chave própria nem paga inferência.
- Interface em **português do Brasil**; textos isolados para tradução futura (i18n).
- **Web primeiro.** Um app desktop (Tauri) poderá existir depois; a lógica fica num núcleo
  compartilhado, sem acesso direto a APIs do navegador (ADR-0003).

## Valores de referência

Valores iniciais, revistos com medição; mudar um deles é mudar esta spec.

| Nome | Valor |
|---|---|
| Computador de referência | 4 núcleos, 8 GB de RAM, Chrome estável, sem GPU dedicada |
| Navegadores suportados | duas últimas versões estáveis de Chrome, Edge, Firefox e Safari |
| Carregamento inicial | ≤ 300 KB (gzip) de JS + CSS, excluindo componentes sob demanda |
| Componentes sob demanda | núcleo WASM pesado (exportação), OCR, Mermaid, modelo local |
| Acessibilidade | zero violações `serious` ou `critical` no axe-core |

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| T-01 | O app DEVE funcionar integralmente servido como arquivos estáticos, sem nenhum endpoint do projeto. | build estático + e2e sobre servidor estático |
| T-02 | O app DEVE fazer requisições de rede somente para: (a) a própria origem; (b) o provedor de IA ou servidor compatível configurado pelo professor; (c) links que o professor pedir para ler, apenas com GET e sem enviar dados além da URL; (d) a origem dos pesos de modelo local (lista fixa), apenas QUANDO o professor iniciar esse download. | e2e com interceptação de rede contra a lista de origens permitidas |
| T-03 | O app DEVE guardar projetos, banco de questões e configurações somente no IndexedDB; o `sessionStorage` só pode guardar dados transitórios de fluxo de login (spec 01, C-01). | teste unitário da camada de armazenamento + e2e de recarga |
| T-04 | QUANDO o professor acionar "Apagar tudo" e confirmar, o app DEVE remover todos os dados do professor (IndexedDB, localStorage, sessionStorage, modelos baixados) e voltar ao estado inicial. | e2e: após apagar, armazenamento vazio e tela inicial |
| T-05 | QUANDO o professor exportar um projeto, o app DEVE gerar um único arquivo que, importado em outro navegador, restaura o projeto idêntico (sem a chave de IA). | teste de ida e volta com comparação estrutural |
| T-06 | SE o armazenamento do navegador estiver indisponível (aba privada, cota cheia), ENTÃO o app DEVE avisar que o trabalho não será salvo e continuar funcionando em memória. | e2e com IndexedDB bloqueado |
| T-07 | O app DEVE ser utilizável em tela a partir de 360 px e por teclado, sem rolagem horizontal, e atender ao valor de referência de acessibilidade. | e2e em 360 px + axe-core |
| T-08 | O carregamento inicial DEVE respeitar o valor de referência; componentes sob demanda DEVEM ser baixados só quando usados. | teste de bundle no CI com limite |
| T-09 | QUANDO o professor salvar pela primeira vez, o app DEVE pedir armazenamento persistente ao navegador e, SE negado, ENTÃO avisar que o navegador pode apagar os dados e sugerir exportar o projeto. | teste unitário com `navigator.storage.persist` simulado |
| T-10 | Conteúdo vindo da IA, de arquivos, de links ou do banco de questões NUNCA DEVE ser interpretado como HTML ou script executável. | testes com fixtures de XSS (HTML, SVG, Markdown, LaTeX e Mermaid maliciosos) |
| T-11 | A página DEVE declarar Content-Security-Policy (via `<meta>`) que proíba scripts de outras origens e scripts inline. | teste no CI sobre o `index.html` gerado + e2e sem violações de CSP |
| T-12 | Projetos e documentos DEVEM carregar o número de versão do esquema; QUANDO um projeto de versão anterior for aberto ou importado, o app DEVE migrá-lo; SE a versão for desconhecida (mais nova), ENTÃO DEVE recusá-lo explicando o motivo, sem alterar o arquivo. | testes de migração com fixtures de cada versão |
| T-13 | ENQUANTO um projeto estiver aberto para edição numa aba, as demais abas DEVEM abri-lo somente para leitura, com aviso. | e2e com duas abas |
| T-14 | ANTES do primeiro envio de dados a cada provedor, o app DEVE mostrar o que será enviado e para onde (incluindo, no OpenRouter, que o pedido pode ser repassado a terceiros) e só prosseguir após confirmação. | e2e |
| T-15 | QUANDO o pedido ou os materiais contiverem possível CPF, e-mail ou telefone, o app DEVE destacar as ocorrências e perguntar se remove, mantém ou cancela, sem agir antes da escolha. | testes unitários do detector + e2e da escolha |
| T-16 | Recursos que dependem de capacidade opcional do navegador (WebGPU) DEVEM ficar ocultos ou explicados quando ela faltar, sem afetar o restante. | teste com capacidades simuladas |

## Fora de escopo (todas as fases)

- Contas de usuário, login no teacher-helper, sincronização em nuvem.
- Backend, banco de dados ou proxy mantido pelo projeto.
- Colaboração em tempo real entre professores.
- Compartilhamento público de material (banco comunitário de questões).
- Cobrança ou revenda de uso de IA.

## Fora do MVP, planejado

- **Anonimização** de dados pessoais: substituir por marcadores antes do envio e restaurar na
  resposta, incluindo detecção de nomes.
- **Correção de provas respondidas**, com o professor no controle:
  - objetivas e lacunas corrigidas por código contra o gabarito, sem IA; o professor revisa e
    decide os casos que não batem exatamente (grafia, sinônimo);
  - discursivas: a IA, se o professor quiser, só **sugere** nota e justificativa a partir do
    critério de correção; cada nota vale apenas depois da confirmação do professor;
  - nenhuma nota é lançada nem exportada sem revisão do professor, e a correção funciona
    inteira sem IA;
  - respostas de aluno são dados pessoais (frequentemente de menores): enviá-las a uma IA
    remota depende da anonimização acima ou do modelo local;
  - primeira entrega sugerida: respostas digitadas e objetivas corrigidas sem IA; leitura de
    folhas manuscritas escaneadas fica para depois.
