import { expect, test } from '@playwright/test'

test('prepared summary belongs to the inspector on desktop and mobile', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Prepared summary layout')
  await page.getByRole('button', { name: 'Create project' }).click()
  const csv = ['value', ...Array.from({ length: 24 }, (_, i) => String(i + 1))].join('\n')
  await page.locator('input[type=file]').setInputFiles({ name: 'summary.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  await page.getByRole('checkbox', { name: 'value', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Column profile', exact: true }).click()
  const inspector = info.project.name === 'mobile-chromium'
    ? page.getByRole('dialog', { name: 'Column profile', exact: true })
    : page.getByRole('complementary', { name: 'Column profile', exact: true })
  await expect(inspector.getByRole('region', { name: 'Prepared data', exact: true })).toContainText('Cross-section, 24 rows')
  await expect(page.getByRole('region', { name: 'Prepared data', exact: true })).toHaveCount(1)
  await expect(page.getByText('Prepared cross-section', { exact: true })).toHaveCount(0)
  await expect(inspector.getByRole('region', { name: 'Prepared data', exact: true }).locator('.msym')).toHaveText('table_view')
  await page.screenshot({ path: info.outputPath('prepared-summary.png'), fullPage: true })
})
