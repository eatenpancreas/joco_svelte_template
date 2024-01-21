import { expect, test } from '@playwright/test';

test('index page has expected h1', async ({ page }) => {
	await page.goto('/');
	await expect(page.getByRole('heading', { name: 'Welcome, This is the Joco Svelte Template!' })).toBeVisible();
});
test('index page has no expected murrrr', async ({ page }) => {
	await page.goto('/');
	await expect(page.getByRole('heading', { name: 'murrrr' })).not.toBeVisible();
});

