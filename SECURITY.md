<!-- Caminho relativo: SECURITY.md -->

# Política de segurança

## Como relatar uma vulnerabilidade

**Não abra issue pública.** Use o relato privado do GitHub: aba **Security** →
**Report a vulnerability** neste repositório.

Inclua o que for possível: descrição, passos para reproduzir, impacto e versão ou commit
afetado. A resposta inicial vem em até 7 dias.

## Escopo

O teacher-helper é uma aplicação estática sem servidor: dados e chaves de IA ficam no
navegador de quem usa (ADR-0001). Interessam especialmente:

- execução de conteúdo vindo da IA, de arquivos ou de links (XSS, injeção em SVG, LaTeX ou
  Mermaid);
- vazamento da chave de IA em URL, log, erro ou arquivo exportado;
- requisições para origens fora da lista permitida (spec 00, T-02);
- contorno da Content-Security-Policy.

Fora do escopo: vulnerabilidades dos provedores de IA, do navegador ou do GitHub Pages.

## Versões suportadas

Só a versão publicada no GitHub Pages (branch `main`) recebe correções.
