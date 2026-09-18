import { expect, type Page } from '@playwright/test'

export async function showSqlSource(page: Page) {
  const opener = page.getByRole('button', { name: 'SQL source', exact: true })
  if ((page.viewportSize()?.width ?? 1280) < 768 && !await page.getByRole('dialog', { name: 'SQL source' }).isVisible()) await opener.click()
}

export async function hideSqlSource(page: Page) {
  const close = page.getByRole('button', { name: 'Close SQL source', exact: true })
  if (await close.isVisible()) {
    await close.click()
    await expect(page.getByRole('dialog', { name: 'SQL source' })).toHaveCount(0)
  }
}
