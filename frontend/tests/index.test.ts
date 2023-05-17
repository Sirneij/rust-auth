import { expect, test } from '@playwright/test';

test('index page has title', async ({ page }) => {
	await page.goto('/');
	await expect(page).toHaveTitle('Written articles | Auth Systems with SvelteKit');
});

test('test some elements', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('link', { name: 'Login' }).click();
});
