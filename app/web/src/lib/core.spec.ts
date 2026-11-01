// Caminho relativo: app/web/src/lib/core.spec.ts

import { readFile } from 'node:fs/promises';
import { describe, expect, it } from 'vitest';
import { initSync, schemaVersion } from 'teacher-helper-wasm';

describe('núcleo WASM', () => {
	it('expõe a versão do esquema definida no núcleo Rust', async () => {
		const wasmUrl = import.meta.resolve('teacher-helper-wasm/teacher_helper_wasm_bg.wasm');
		initSync({ module: await readFile(new URL(wasmUrl)) });
		expect(schemaVersion()).toBe(1);
	});
});
