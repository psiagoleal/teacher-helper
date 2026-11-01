// Spike PRE-5: Mermaid -> SVG sem foreignObject, renderizado no Chromium.
import { createRequire } from 'node:module';
import { writeFile } from 'node:fs/promises';
const [, , webDir, mermaidJs, outDir] = process.argv;
const { chromium } = createRequire(`${webDir}/package.json`)('@playwright/test');

const diagrams = {
  flowchart: `flowchart TD\n  A[Início] --> B{x > 0?}\n  B -- sim --> C[Positivo]\n  B -- não --> D[Não positivo]\n  C --> E((Fim))\n  D --> E`,
  sequence: `sequenceDiagram\n  Aluno->>Professor: Entrega a prova\n  Professor-->>Aluno: Devolve com nota`,
  mindmap: `mindmap\n  root((Fotossíntese))\n    Luz\n    Água\n    CO2\n    Glicose`,
  pie: `pie title Notas da turma\n  "A" : 12\n  "B" : 9\n  "C" : 4`,
  class: `classDiagram\n  class Animal {\n    +String nome\n    +comer()\n  }\n  Animal <|-- Cachorro`,
  state: `stateDiagram-v2\n  [*] --> Sólido\n  Sólido --> Líquido: fusão\n  Líquido --> Gasoso: vaporização`,
};

const browser = await chromium.launch();
const page = await browser.newPage();
await page.setContent('<!doctype html><html><body></body></html>');
await page.addScriptTag({ path: mermaidJs });
for (const [name, src] of Object.entries(diagrams)) {
  const res = await page.evaluate(async ([name, src]) => {
    const m = window.mermaid;
    m.initialize({ startOnLoad: false, securityLevel: 'strict', htmlLabels: false,
      flowchart: { htmlLabels: false }, theme: 'neutral' });
    try {
      await m.parse(src);
      const { svg } = await m.render(`d-${name}`, src);
      return { svg };
    } catch (e) { return { error: String(e) }; }
  }, [name, src]);
  if (res.error) { console.log(`${name}: ERRO ${res.error}`); continue; }
  const fo = (res.svg.match(/<foreignObject/g) || []).length;
  await writeFile(`${outDir}/${name}.svg`, res.svg);
  console.log(`${name}: ${res.svg.length} bytes, foreignObject=${fo}`);
}
// Mermaid inválido deve ser detectado pelo parse (critério P-02).
const bad = await page.evaluate(async () => { try { await window.mermaid.parse('flowchart TD\n A -->'); return 'aceito'; } catch { return 'rejeitado'; } });
console.log(`mermaid inválido: ${bad}`);
await browser.close();
