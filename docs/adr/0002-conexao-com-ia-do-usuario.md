<!-- Caminho relativo: docs/adr/0002-conexao-com-ia-do-usuario.md -->

# ADR 0002: Conexão com a IA do usuário — sem login de assinatura de consumidor

- **Status:** Proposed
- **Data:** 2026-10-31
- **Decisores:** Iago Leal
- **Tags:** ia, termos-de-uso, segurança

## Contexto

O pedido original previa conectar assinaturas de consumidor (Claude Pro, ChatGPT/Codex,
Gemini), porque o público não domina APIs. Situação em out/2026, segundo cobertura pública
(confirmar nas fontes oficiais antes de aceitar este ADR):

- **Anthropic:** usar tokens OAuth de Claude Free/Pro/Max fora do Claude Code e do claude.ai
  viola os Termos de Consumidor; terceiros não podem oferecer login claude.ai nem repassar
  pedidos com essas credenciais. Há bloqueio no servidor.
- **OpenAI:** *Sign in with ChatGPT* permite a apps parceiros consumir a cota do plano
  Plus/Pro do usuário; exige cadastro/aprovação do app.
- **OpenRouter:** oferece OAuth PKCE para apps no navegador; o usuário faz login e o app
  recebe uma chave controlada pelo usuário, com limites configuráveis.
- **Google AI Studio:** chave gratuita com cota, obtida em minutos com conta Google.

## Decisão

O app conecta IA **somente por mecanismos que o provedor autoriza para terceiros**:

1. **OpenRouter via OAuth PKCE** — caminho principal para o professor leigo.
2. **Chave gratuita do Gemini** com tutorial guiado.
3. **Chave de API própria** (OpenAI, Anthropic).
4. **Servidor compatível com a API da OpenAI** (Ollama, llama.cpp, vLLM, LiteLLM, LM Studio
   e provedores compatíveis), local ou remoto, configurado por endereço e chave opcional.
5. **Modelo local no navegador** (WebGPU), opcional.

Todos atrás de **uma interface única de provedor** (spec 01, C-06). O app **não** implementa
login com assinatura Claude. *Sign in with ChatGPT* entra como adaptador **se** o cadastro do
app for aprovado. A **ponte para CLI** (fase 2) é opcional, com aviso explícito sobre os
termos de cada provedor; preferencialmente vive no app desktop (ADR-0003) ou como servidor
local compatível com OpenAI (item 4). Para Claude Code, só pode ser oferecida se os termos
vigentes permitirem.

## Consequências

- **Impacto positivo:** sem risco de banimento da conta do professor; sem custo para o
  projeto; troca de provedor sem retrabalho.
- **Impacto negativo:** o professor com assinatura Claude Pro não reaproveita a assinatura;
  precisa de OpenRouter, Gemini grátis ou chave paga.
- **Trade-offs aceitos:** a chave fica no navegador do usuário (IndexedDB ou só memória),
  exposta a quem tiver acesso àquele navegador — mitigado por "não lembrar a chave" e
  "apagar tudo".

## Diretriz de Conformidade de Código

- Proibido: usar ou solicitar credenciais de assinatura de consumidor (cookies de sessão,
  tokens OAuth de claude.ai/chatgpt.com) fora de programa oficial do provedor.
- Proibido: chamar provedor sem passar pela interface única de provedor.
- Obrigatório: a chave nunca aparece em URL, log, mensagem de erro ou arquivo exportado.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
