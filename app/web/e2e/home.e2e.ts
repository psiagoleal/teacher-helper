// Caminho relativo: app/web/e2e/home.e2e.ts

import { expect, test } from '@playwright/test';

test('a página inicial carrega o núcleo WASM sem violar a CSP', async ({ page }) => {
	const cspViolations: string[] = [];
	page.on('console', (msg) => {
		if (msg.type() === 'error' && /Content Security Policy/i.test(msg.text())) {
			cspViolations.push(msg.text());
		}
	});

	await page.goto('/');
	await expect(page.getByRole('heading', { level: 1 })).toHaveText('teacher-helper');
	await expect(page.getByTestId('core-status')).toHaveText('Núcleo carregado — esquema v1');
	expect(cspViolations).toEqual([]);
});
