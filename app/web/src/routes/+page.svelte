<!-- Caminho relativo: app/web/src/routes/+page.svelte -->

<script lang="ts">
	import { onMount } from 'svelte';
	import { loadCore } from '#lib/core.js';

	let coreStatus = $state('Carregando o núcleo…');

	onMount(async () => {
		try {
			const core = await loadCore();
			coreStatus = `Núcleo carregado — esquema v${core.schemaVersion}`;
		} catch {
			coreStatus = 'Não foi possível carregar o núcleo.';
		}
	});
</script>

<svelte:head>
	<title>teacher-helper</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-12">
	<h1 class="text-3xl font-bold">teacher-helper</h1>
	<p class="mt-4">
		Provas, listas e materiais de aula com a sua IA — direto do navegador, sem cadastro.
	</p>
	<p class="mt-8 text-sm text-gray-600" data-testid="core-status">{coreStatus}</p>
</main>
