import { expect, test } from '@playwright/test'

test('technical metadata belongs only to its history entry', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Sensitivity/ }).click()
  await expect(page.getByTestId('sidebar-run-details')).toHaveCount(0)
  await expect(page.getByRole('combobox', { name: 'Run record', exact: true })).toHaveCount(0)
  if (info.project.name === 'mobile-chromium') {
    await page.getByRole('group', { name: 'Panes' }).getByRole('button').filter({ hasText: 'History' }).click()
  }
  const history = page.locator('li > details').filter({ has: page.getByTestId('history-run-details') }).first()
  await history.locator(':scope > summary').click()
  const metadata = history.getByTestId('history-run-details')
  await expect(metadata.locator('summary')).toContainText('Probe run details')
  expect(await metadata.evaluate(el => el === el.parentElement?.firstElementChild)).toBe(true)
  await expect(metadata).not.toHaveAttribute('open')
  await metadata.locator('summary').click()
  await expect(metadata).toHaveAttribute('open', '')
  await expect(metadata.locator('summary')).toContainText('receipt_long')
  await expect(metadata).toContainText('Estimation run')
  await expect(metadata).toContainText('Probe')
  await metadata.screenshot({ path: info.outputPath('history-metadata.png') })
})
