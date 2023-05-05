import { expect, test } from '@playwright/test';

test('index page has title', async ({ page }) => {
	await page.goto('/');
	await expect(page).toHaveTitle('Written articles | Actix Web & SvelteKit');
});

test('index page has h1 content', async ({ page }) => {
	await page.goto('/');
	expect(await page.textContent('h1')).toContain(
		'Authentication system using Actix Web and Sveltekit'
	);
});

test('index page has img alt content', async ({ page }) => {
	await page.goto('/');
	const isVisible = await page.getByAltText('Rust (actix-web) and Sveltekit').isVisible();
	expect(isVisible).toBe(true);
});

test('test some elements', async ({ page }) => {
	await page.goto('/');
	await page
		.getByRole('heading', { name: 'Authentication system using Actix Web and Sveltekit' })
		.click();
	await page.getByRole('link', { name: 'Login' }).click();

	await page.getByRole('button', { name: '+' }).click();
});
