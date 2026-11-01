import { readFile, writeFile } from 'node:fs/promises';
const dir = process.argv[2];
const mod = await import(`${dir}/spike_typst.js`);
let t = performance.now();
mod.initSync({ module: await readFile(`${dir}/spike_typst_bg.wasm`) });
console.log(`init: ${(performance.now() - t).toFixed(0)} ms`);
const latex = [
  String.raw`\frac{a+b}{2}`, String.raw`x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}`,
  String.raw`\int_0^1 x^2\,dx`, String.raw`\sum_{i=1}^{n} i`, String.raw`\lim_{x \to 0} \frac{\sin x}{x}`,
];
const twenty = Array.from({ length: 20 }, (_, i) => latex[i % latex.length]);
for (const run of [1, 2, 3]) {
  t = performance.now();
  const pdf = mod.render(twenty);
  console.log(`render 20 questões #${run}: ${(performance.now() - t).toFixed(0)} ms, ${pdf.length} bytes`);
  if (run === 1) await writeFile(`${dir}/../prova-wasm.pdf`, pdf);
}
