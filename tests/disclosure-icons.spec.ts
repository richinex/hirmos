import { expect, test } from '@playwright/test'

test('study details show and hide with eye icons and native keyboard controls', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Study design/ }).click()
  const summary = page.locator('summary.disclosure-summary').filter({ hasText: 'Study details' }).first()
  await expect(summary).toBeVisible()
  const details = summary.locator('..')
  await expect(summary.locator('.disclosure-show')).toBeVisible()
  await expect(summary.locator('.disclosure-hide')).toBeHidden()
  expect(await summary.evaluate(el => getComputedStyle(el).listStyleType)).toBe('none')
  await summary.click()
  await expect(details).toHaveAttribute('open', '')
  await expect(summary.locator('.disclosure-hide')).toBeVisible()
  await expect(summary.locator('.disclosure-show')).toBeHidden()
  await summary.focus()
  await page.keyboard.press('Enter')
  await expect(details).not.toHaveAttribute('open')
  await page.keyboard.press('Space')
  await expect(details).toHaveAttribute('open', '')
  for (const theme of ['light', 'dark']) {
    await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
    await summary.screenshot({ path: info.outputPath(`disclosure-${theme}.png`) })
  }
})
