import { expect, test } from '@playwright/test';

test('login page has title, h1 and url', async ({ page }) => {
	await page.goto('/auth/login');
	await expect(page).toHaveTitle('Auth - Login | Auth Systems with SvelteKit');
	await expect(page).toHaveURL('/auth/login');
	expect(await page.textContent('h1')).toBe('Login');
});

test('login page form element', async ({ page }) => {
	await page.goto('/auth/login');

	// Form element
	const formElement = page.locator('form');
	const formAction = await formElement.getAttribute('action');
	expect(formAction).toBe('?/login');

	// Email input
	const inputField = page.getByRole('textbox', { name: 'Email address' });
	await inputField.click();
	await inputField.type('jane.doe@example.com');
	await expect(inputField).toHaveValue('jane.doe@example.com');

	// Password input
	const passwordInput = page.getByRole('textbox', { name: 'Password' });
	await passwordInput.click();
	await passwordInput.type('mypassword');
	expect(await page.inputValue("input[type='password']")).toBe('mypassword');
});

test('login page has some links', async ({ page }) => {
	await page.goto('/auth/login');
	await page.getByRole('link', { name: 'Create an account.' }).click();
	await page.getByRole('link', { name: 'Forgot password?' }).click();
});
