<!-- Caminho relativo: docs/specs/01-conexao-ia.md -->

# Spec 01 — Conexão com a IA do professor

- **Status:** Aprovada (2026-11-01)
- **Fase:** MVP (ponte para CLI: fase 2)
- **Cita:** `00-transversal.md`, ADR-0002, ADR-0003

## Problema e resultado esperado

O professor típico não sabe o que é chave de API. Ele precisa conectar uma IA em poucos
cliques, entender quanto custa (ou que é grátis) e poder trocar de provedor sem perder o
trabalho.

## Provedores do MVP

| Provedor | Como conecta | Custo para o professor |
|---|---|---|
| OpenRouter | Botão "Conectar" → login no OpenRouter → volta com a chave (OAuth PKCE) | Modelos gratuitos ou créditos pré-pagos |
| Google Gemini | Tutorial guiado para gerar chave gratuita no Google AI Studio e colar | Cota gratuita |
| OpenAI / Anthropic | Colar chave própria de API | Pago por uso |
| Servidor compatível com OpenAI | Endereço + chave opcional — Ollama, llama.cpp, vLLM, LiteLLM, LM Studio e provedores compatíveis | Grátis (local) ou conforme o servidor |
| Modelo local | Download de modelo para rodar no navegador (WebGPU) | Grátis, offline, qualidade menor |

## Capacidades de modelo

Catálogo fechado, usado por skills (spec 05) e materiais (spec 02): `visao`, `ferramentas`,
`saida_estruturada`, `video_por_url`, `contexto_max_tokens`.

## Critérios de aceite (EARS)

| ID | Critério | Prova |
|---|---|---|
| C-01 | QUANDO o professor clicar em "Conectar OpenRouter", o app DEVE executar o fluxo OAuth PKCE (S256) com parâmetro `state` aleatório, guardar `code_verifier` e `state` apenas no `sessionStorage` durante o fluxo, rejeitar retorno com `state` divergente, apagar ambos ao final e guardar a chave recebida sem exibi-la. | e2e com OpenRouter simulado, incluindo `state` adulterado |
| C-02 | SE o professor negar a autorização ou o retorno trouxer erro ou código inválido, ENTÃO o app DEVE explicar em linguagem simples e oferecer tentar de novo. | e2e com retornos de erro simulados |
| C-03 | QUANDO o professor colar uma chave, o app DEVE validá-la com uma chamada mínima ao provedor antes de salvá-la e informar sucesso ou o motivo da falha em linguagem simples. | teste unitário por provedor com respostas simuladas (200, 401, 403, 429) |
| C-04 | O app DEVE exibir, para o provedor conectado, se o uso é gratuito ou pago conforme tabela mantida no projeto, e um link para o painel de uso do provedor. | teste de componente com tabela de fixture |
| C-05 | ONDE o professor marcar "não lembrar a chave neste computador", o app DEVE manter a chave só em memória e descartá-la ao fechar a aba. | e2e: recarregar pede a chave de novo |
| C-06 | O app DEVE oferecer o modelo local somente QUANDO o navegador suportar WebGPU, e DEVE informar o tamanho do download e pedir confirmação antes de iniciá-lo. | teste com WebGPU simulado (presente/ausente) |
| C-07 | Toda geração DEVE passar por uma interface única de provedor, de modo que trocar de provedor não altere projetos, skills nem documentos já gerados. | testes de contrato com a mesma suíte contra cada adaptador simulado |
| C-08 | SE o provedor responder erro de autenticação, ENTÃO o app DEVE marcar a conexão como inválida e pedir nova conexão, sem perder o pedido em andamento. | teste unitário + e2e |
| C-09 | SE o provedor responder limite de taxa (429 temporário), ENTÃO o app DEVE informar o tempo de espera quando o provedor o indicar; SE responder cota ou crédito esgotado, ENTÃO DEVE explicar e sugerir trocar de provedor ou modelo. | teste unitário com as duas respostas por provedor |
| C-10 | SE a requisição exceder o tempo limite ou cair a rede, ENTÃO o app DEVE oferecer tentar de novo — avisando que o provedor pode cobrar as duas tentativas — sem exigir reenvio manual dos materiais. | teste unitário com rede simulada |
| C-11 | O app NUNCA DEVE incluir a chave em URL, log, arquivo exportado ou mensagem de erro exibida. | teste com chave canário: varredura de logs, erros, exportações e URLs não a encontra |
| C-12 | O app DEVE conectar a qualquer servidor que implemente a API de Chat Completions da OpenAI acessível por HTTPS ou por `http://localhost` / `http://127.0.0.1`, a partir de endereço base, chave opcional e nome do modelo. | testes de contrato contra servidor simulado + roteiro manual registrado com Ollama e llama.cpp |
| C-13 | SE o endereço do servidor for HTTP fora de localhost, ENTÃO o app DEVE explicar que o navegador bloqueia esse acesso e indicar HTTPS ou o app desktop futuro. | teste unitário |
| C-14 | QUANDO o professor acionar "Testar conexão" num servidor compatível, o app DEVE distinguir e explicar: servidor fora do ar, bloqueio de CORS, permissão de rede local negada, chave inválida, modelo inexistente. | teste unitário com cada falha simulada |
| C-15 | O app DEVE oferecer guia de configuração de CORS para Ollama, llama.cpp, vLLM, LiteLLM e LM Studio, com a origem do app já preenchida. | e2e: guia exibe a origem atual |
| C-16 | Para servidores compatíveis, o professor DEVE declarar as capacidades do modelo; o app DEVE pré-preenchê-las a partir de uma tabela de modelos conhecidos mantida no projeto. | teste unitário |
| C-17 | ONDE o servidor ou provedor suportar saída restrita a esquema (JSON schema), o conector DEVE usá-la nas gerações estruturadas. | teste de contrato |
| C-18 | QUANDO o modelo escolhido não tiver uma capacidade exigida pelo pedido, o app DEVE avisar antes de enviar e oferecer alternativa (ver specs 02 e 05). | teste unitário da matriz de capacidades |

## Fora de escopo do MVP

- **Ponte para CLI** (Codex CLI / Claude Code do usuário) — fase 2, opcional, com aviso sobre
  os termos de cada provedor; candidata ao app desktop (ADR-0003) ou a servidor local
  compatível (C-12). Ganha spec própria.
- **Login com assinatura Claude Pro/Max** — proibido pelos termos da Anthropic.
- **Sign in with ChatGPT** — depende de cadastro do app junto à OpenAI; investigação em ticket.
  Se viável, entra como adaptador sob C-07.
- Escolha automática do "melhor modelo" ou comparação de custo entre provedores.
- Servidores HTTP na rede local (exigem o app desktop ou HTTPS).

## Premissas a verificar (ticket PRE-2)

- CORS direto do navegador: OpenAI, Gemini e OpenRouter; Anthropic exige o cabeçalho
  `anthropic-dangerous-direct-browser-access`.
- Comportamento de `http://localhost` a partir de HTTPS no Safari e permissão de rede local
  no Chrome.
- Termos do plano gratuito do OpenRouter e do Sign in with ChatGPT.
