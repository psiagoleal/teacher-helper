<!-- Caminho relativo: docs/TICKETS.md -->

# Tickets

> Visão geral rápida do que foi feito e do que falta. **Uma linha por ticket**: identificador
> com link para o detalhe, e uma frase curta. O detalhe mora no destino do link, não aqui.
> Atualizado **sempre** que um ticket é criado, concluído ou abandonado — skill
> `micro-ticket-planner`.

## Em aberto

- [ ] [PRE-2](plano-mvp.md#pre-2) — Verificar nas fontes oficiais: termos e CORS de cada provedor (header da Anthropic), `localhost` via HTTPS no Safari, permissão de rede local no Chrome, eviction do IndexedDB no Safari; aceitar ou ajustar ADR-0001 a 0004.
- [ ] [PRE-3](plano-mvp.md#pre-3) — Investigar *Sign in with ChatGPT*: requisitos de cadastro para app gratuito estático.
- [ ] [PRE-7](plano-mvp.md#pre-7) — Levantar concorrentes (ex.: MagicSchool e equivalentes brasileiros) e validar a proposta de valor da spec 00.
- [ ] [PRE-9](plano-mvp.md#pre-9) — `docs/DESIGN.md` (contrato visual) antes do primeiro componente de interface.
- [ ] [PRE-10](plano-mvp.md#pre-10) — Planejar a próxima etapa: anonimização de dados pessoais, BNCC (fonte e licença da base), skills de gabarito comentado, importação de prova antiga e correção de provas respondidas.

### MVP — micro-tickets ([plano](plano-mvp.md))

- [ ] [MT-2](plano-mvp.md#mt-2) — Escolher extração de PDF, DOCX e OCR.
- [ ] [MT-3](plano-mvp.md#mt-3) — Escolher a renderização da pré-visualização.
- [ ] [MT-4](plano-mvp.md#mt-4) — Escolher o motor de modelo local.
- [ ] [MT-7](plano-mvp.md#mt-7) — Validar documento com erros por caminho.
- [ ] [MT-8](plano-mvp.md#mt-8) — Migrar esquema de versões anteriores.
- [ ] [MT-9](plano-mvp.md#mt-9) — Implementar as verificações CK-01 a CK-07 e CK-09.
- [ ] [MT-10](plano-mvp.md#mt-10) — Verificar LaTeX no núcleo.
- [ ] [MT-11](plano-mvp.md#mt-11) — Calcular numeração e pontuação.
- [ ] [MT-12](plano-mvp.md#mt-12) — Gerar o gabarito separado e controlar a conferência.
- [ ] [MT-13](plano-mvp.md#mt-13) — Embaralhar versões A, B, … com semente.
- [ ] [MT-14](plano-mvp.md#mt-14) — Implementar desfazer e refazer.
- [ ] [MT-15](plano-mvp.md#mt-15) — Calcular o diff estrutural por questão.
- [ ] [MT-16](plano-mvp.md#mt-16) — Buscar questões compatíveis no banco.
- [ ] [MT-17](plano-mvp.md#mt-17) — Detectar dados pessoais.
- [ ] [MT-18](plano-mvp.md#mt-18) — Expor as operações do documento no WASM.
- [ ] [MT-19](plano-mvp.md#mt-19) — Definir o esquema de skill e validar no CI.
- [ ] [MT-20](plano-mvp.md#mt-20) — Montar o pedido com prefixo estável.
- [ ] [MT-21](plano-mvp.md#mt-21) — Montar o contexto mínimo de edição.
- [ ] [MT-22](plano-mvp.md#mt-22) — Implementar a matriz de capacidades.
- [ ] [MT-23](plano-mvp.md#mt-23) — Fechar o ciclo da resposta.
- [ ] [MT-24](plano-mvp.md#mt-24) — Escrever a skill "Gerar prova".
- [ ] [MT-25](plano-mvp.md#mt-25) — Escrever a skill "Gerar lista de exercícios".
- [ ] [MT-26](plano-mvp.md#mt-26) — Escrever a skill "Revisar prova".
- [ ] [MT-27](plano-mvp.md#mt-27) — Escrever a skill "Criar distratores".
- [ ] [MT-28](plano-mvp.md#mt-28) — Escrever a skill "Ajustar dificuldade".
- [ ] [MT-29](plano-mvp.md#mt-29) — Escrever o guia de contribuição de skills.
- [ ] [MT-30](plano-mvp.md#mt-30) — Definir a interface de provedor e a suíte de contrato.
- [ ] [MT-31](plano-mvp.md#mt-31) — Implementar o adaptador Chat Completions (OpenRouter e compatíveis).
- [ ] [MT-32](plano-mvp.md#mt-32) — Implementar o adaptador Anthropic com cache de prompt.
- [ ] [MT-33](plano-mvp.md#mt-33) — Implementar o adaptador Gemini.
- [ ] [MT-34](plano-mvp.md#mt-34) — Validar a chave colada.
- [ ] [MT-35](plano-mvp.md#mt-35) — Conectar o OpenRouter por OAuth PKCE.
- [ ] [MT-36](plano-mvp.md#mt-36) — Guardar a chave com segurança.
- [ ] [MT-37](plano-mvp.md#mt-37) — Diagnosticar servidor compatível.
- [ ] [MT-38](plano-mvp.md#mt-38) — Criar as tabelas de custo e de modelos conhecidos.
- [ ] [MT-39](plano-mvp.md#mt-39) — Rotear para o "modelo rápido".
- [ ] [MT-40](plano-mvp.md#mt-40) — Integrar o modelo local.
- [ ] [MT-41](plano-mvp.md#mt-41) — Guardar no IndexedDB com reserva em memória.
- [ ] [MT-42](plano-mvp.md#mt-42) — Exportar e importar projeto.
- [ ] [MT-43](plano-mvp.md#mt-43) — Apagar tudo.
- [ ] [MT-44](plano-mvp.md#mt-44) — Travar edição entre abas.
- [ ] [MT-45](plano-mvp.md#mt-45) — Vigiar as origens de rede no e2e.
- [ ] [MT-46](plano-mvp.md#mt-46) — Limitar o tamanho do carregamento inicial.
- [ ] [MT-47](plano-mvp.md#mt-47) — Verificar acessibilidade e 360 px em toda tela.
- [ ] [MT-48](plano-mvp.md#mt-48) — Montar a casca do app e os textos isolados.
- [ ] [MT-49](plano-mvp.md#mt-49) — Pedir confirmação no primeiro envio a cada provedor.
- [ ] [MT-50](plano-mvp.md#mt-50) — Criar a tela "Conectar IA".
- [ ] [MT-51](plano-mvp.md#mt-51) — Anexar materiais.
- [ ] [MT-52](plano-mvp.md#mt-52) — Perguntar sobre dados pessoais antes do envio.
- [ ] [MT-53](plano-mvp.md#mt-53) — Extrair texto de PDF.
- [ ] [MT-54](plano-mvp.md#mt-54) — Extrair texto de DOCX.
- [ ] [MT-55](plano-mvp.md#mt-55) — Enviar imagem por visão ou reconhecer por OCR local.
- [ ] [MT-56](plano-mvp.md#mt-56) — Ler links e tratar vídeos.
- [ ] [MT-57](plano-mvp.md#mt-57) — Resumir o material uma vez e reaproveitar.
- [ ] [MT-58](plano-mvp.md#mt-58) — Mostrar a estimativa antes do envio e registrar os tokens.
- [ ] [MT-59](plano-mvp.md#mt-59) — Gerar o formulário a partir da skill.
- [ ] [MT-60](plano-mvp.md#mt-60) — Gerar com progresso e cancelamento.
- [ ] [MT-61](plano-mvp.md#mt-61) — Pré-visualizar conteúdo rico com segurança.
- [ ] [MT-62](plano-mvp.md#mt-62) — Normalizar, validar e desenhar Mermaid.
- [ ] [MT-63](plano-mvp.md#mt-63) — Editar direto e mostrar o aviso de IA.
- [ ] [MT-64](plano-mvp.md#mt-64) — Propor alterações da IA com diff.
- [ ] [MT-65](plano-mvp.md#mt-65) — Mostrar verificações, pontuação e conferência.
- [ ] [MT-66](plano-mvp.md#mt-66) — Usar o banco de questões.
- [ ] [MT-67](plano-mvp.md#mt-67) — Criar o módulo de exportação com Typst.
- [ ] [MT-68](plano-mvp.md#mt-68) — Diagramar a prova no PDF.
- [ ] [MT-69](plano-mvp.md#mt-69) — Separar arquivos e apontar a questão com erro.
- [ ] [MT-70](plano-mvp.md#mt-70) — Validar PDF/UA no CI.
- [ ] [MT-71](plano-mvp.md#mt-71) — Pôr diagramas e imagens no PDF.
- [ ] [MT-72](plano-mvp.md#mt-72) — Converter fórmulas para equação do Word.
- [ ] [MT-73](plano-mvp.md#mt-73) — Gerar o DOCX e validá-lo no LibreOffice.
- [ ] [MT-74](plano-mvp.md#mt-74) — Exportar Markdown.
- [ ] [MT-75](plano-mvp.md#mt-75) — Carregar a exportação sob demanda e medir o tempo.
- [ ] [MT-76](plano-mvp.md#mt-76) — Criar a tela de exportação.
- [ ] [MT-77](plano-mvp.md#mt-77) — Provar a resistência a XSS.
- [ ] [MT-78](plano-mvp.md#mt-78) — Testar o fluxo completo sem IA onde ela não é necessária.
- [ ] [MT-79](plano-mvp.md#mt-79) — Escrever os roteiros manuais de release.
- [ ] [MT-80](plano-mvp.md#mt-80) — Gerenciar modelos de cabeçalho.
- [ ] [MT-81](plano-mvp.md#mt-81) — Escolher o uso das imagens dos materiais.

## Concluídos

- [x] [PRE-0](insumos/briefing.md) — Entrevista inicial, briefing (`docs/insumos/briefing.md`) e rascunho das specs 00–04.
- [x] [PRE-0b](specs/05-skills-pedagogicas.md) — Proposta de valor, specs 05 (skills) e 06 (economia de cota), conector compatível com OpenAI, ADR-0003 (desktop).
- [x] [PRE-0c](specs/README.md) — Crítica das specs em contexto limpo e rodada 2 das specs 00–06.
- [x] [PRE-4](adr/0004-stack-e-layout.md) — Questionário de stack: ADR-0004 e ilhas do `AGENTS.md`.
- [x] [PRE-1](specs/README.md) — Specs 00–06 aprovadas pelo mantenedor (2026-11-01).
- [x] [PRE-5](spikes/pre-5/README.md) — Spike de fórmulas, PDF/UA, DOCX e Mermaid: viável; ADR-0005 e código em `docs/spikes/pre-5/`.
- [x] [PRE-6](plano-mvp.md) — MVP quebrado em 79 micro-tickets em 9 ondas (`docs/plano-mvp.md`).
- [x] [MT-5](plano-mvp.md#mt-5) — Specs 02 e 03 sem questões em aberto: imagens de entrada à escolha do professor (M-14), cabeçalho por modelo configurável (P-17).
- [x] [PRE-8](adr/0004-stack-e-layout.md) — Scaffold (workspace Cargo, SvelteKit, WASM, CSP, CI/deploy); todos os comandos da ilha `comandos-exatos` executados.
- [x] [MT-1](adr/0006-fronteira-nucleo-interface.md) — ADR-0006 aceito: sessão de documento por comandos no núcleo, interface de plataforma `Http`/`ExternalModel`, JSON e esquema gerado na fronteira.
- [x] [PRE-11](plano-mvp.md#pre-11) — Repositório público no GitHub com `main` protegida, CI verde e Pages publicado mostrando "Núcleo carregado".
- [x] [MT-6](plano-mvp.md#mt-6) — Tipos Rust do documento, das seções, dos 6 tipos de questão, do cabeçalho e do conteúdo rico, com ida e volta em JSON e esquema gerado.

## Abandonados

<!-- - [~] MT-0 — <frase curta> · motivo: <uma frase> -->
