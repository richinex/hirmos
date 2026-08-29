import { expect, test } from '@playwright/test'

test('landing page opens the workbench entry', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('link', { name: 'Start an analysis' }).click()
  await expect(page).toHaveURL('/app')
  await expect(page.getByRole('textbox', { name: 'Project name' })).toBeVisible()
})
