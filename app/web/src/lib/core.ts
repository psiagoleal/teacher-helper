// Caminho relativo: app/web/src/lib/core.ts

// Ponto único de acesso ao núcleo Rust/WASM pela interface.
import init, { schemaVersion } from 'teacher-helper-wasm';

let ready: Promise<unknown> | undefined;

/** Carrega o núcleo WASM uma única vez e devolve a versão do esquema de documento. */
export async function loadCore(): Promise<{ schemaVersion: number }> {
	ready ??= init();
	await ready;
	return { schemaVersion: schemaVersion() };
}
