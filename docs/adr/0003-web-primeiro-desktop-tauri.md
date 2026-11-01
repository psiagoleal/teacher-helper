<!-- Caminho relativo: docs/adr/0003-web-primeiro-desktop-tauri.md -->

# ADR 0003: Web primeiro; desktop (Tauri) para recursos avançados, sobre núcleo compartilhado

- **Status:** Proposed
- **Data:** 2026-10-31
- **Decisores:** Iago Leal
- **Tags:** arquitetura, plataforma, distribuição

## Contexto

A aplicação web no GitHub Pages (ADR-0001) cobre o essencial, mas o navegador impõe limites
que alguns recursos desejados esbarram:

- leitura de links e vídeos bloqueada por CORS;
- chamadas a servidores de IA na rede local exigem HTTPS e permissão de rede local;
- não pode iniciar processos (llama.cpp, Codex CLI) — a "ponte para CLI" exigiria um
  programa separado;
- acesso a arquivos restrito ao que o usuário escolhe a cada vez; armazenamento sujeito a cota;
- chave de IA guardada no navegador, sem cofre do sistema.

Alternativas consideradas:

1. **Tauri 2 (escolhida para o desktop).** Reaproveita a interface Svelte e o núcleo Rust;
   binário pequeno; acesso nativo a processos, rede e arquivos.
2. **Electron.** Maduro, mas binário grande e núcleo em Node — perde o reaproveitamento Rust.
3. **Só web, com programa auxiliar local para o que o navegador não faz.** Dois artefatos para
   instalar e manter, com pior experiência que um app único.

## Decisão

- O desenvolvimento **começa pela aplicação web** (ADR-0001). O desktop **não** faz parte do MVP.
- Quando um recurso justificar, o desktop será um app **Tauri 2** que reutiliza a mesma
  interface SvelteKit e o mesmo **núcleo Rust**.
- **Desktop contém a web:** todo recurso da web funciona no desktop.
- Um recurso só é **exclusivo do desktop** se o navegador não puder implementá-lo de forma
  aceitável (processo local, rede sem CORS, arquivos sem restrição, dependência nativa
  pesada). Conveniência não é motivo.
- Candidatos iniciais ao desktop: ponte para Codex CLI / Claude Code (absorve a "ponte para
  CLI" da fase 2), iniciar e gerenciar llama.cpp local, leitura irrestrita de links e
  vídeos, pastas de materiais monitoradas, chave no cofre do sistema.

## Consequências

- **Impacto positivo:** um só código de interface e de lógica; a ponte para CLI deixa de ser
  um programa separado; recursos avançados sem degradar a web.
- **Impacto negativo:** quando o desktop entrar, haverá custo de assinatura de código
  (Apple Developer, certificado Windows), atualização automática e builds para três SOs.
- **Trade-offs aceitos:** o núcleo precisa ser escrito desde já sem depender de APIs do
  navegador, mesmo antes de existir o desktop.

## Diretriz de Conformidade de Código

- Obrigatório: regras de negócio (modelo de documento, skills, validação, exportação,
  conectores de IA) ficam no núcleo compartilhado, sem acesso direto a APIs do navegador.
- Obrigatório: rede, armazenamento, arquivos e processos são acessados por uma **interface de
  plataforma**, com implementação web e (futuramente) desktop.
- Proibido: recurso exclusivo do desktop sem registrar na spec o motivo pelo qual o navegador
  não o comporta.
- Proibido: recurso da web ausente no desktop.

> Qualquer desvio desta regra viola as diretrizes de conformidade arquitetural do projeto
> e deve ser reportado para revisão antes de prosseguir.
