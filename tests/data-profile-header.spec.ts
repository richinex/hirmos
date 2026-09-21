import { expect, test } from '@playwright/test'

for (const filename of ['Seatbelts.csv', 'team-adoption-and-code-quality-observations-with-a-long-source-filename.csv']) {
  test(`Data Studio uses the source filename as its heading: ${filename}`, async ({ page }, info) => {
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill('Data header review')
    await page.getByRole('button', { name: 'Create project' }).click()
    await page.locator('input[type=file]').setInputFiles({ name: filename, mimeType: 'text/csv', buffer: Buffer.from('time,value\n1,10\n2,20\n') })
    await page.getByRole('button', { name: 'Inspect data' }).click()
    const heading = page.getByRole('heading', { name: filename, exact: true })
    await expect(heading).toBeVisible({ timeout: 30_000 })
    const header = heading.locator('..').locator('..')
    await expect(header).not.toContainText('02 · Data studio')
    await expect(header).not.toContainText('Data profile')
    await expect(header.getByRole('heading')).toHaveCount(1)
    await expect(header.getByLabel('Dataset size').locator('dt')).toHaveText(['rows', 'columns', 'numeric'])
    await expect(header.getByLabel('Dataset size').locator('dd > [aria-hidden]')).toHaveText(['2', '2', '2'])
    // The header carries the one action on the source, editing it; nothing else competes with the name.
    await expect(header.getByRole('button')).toHaveCount(1)
    await expect(header.getByRole('button', { name: 'Edit data', exact: true })).toBeVisible()
    for (const theme of ['light', 'dark']) {
      await page.evaluate(theme => { document.documentElement.dataset.theme = theme }, theme)
      await heading.scrollIntoViewIfNeeded()
      expect(await header.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
      await page.screenshot({ path: info.outputPath(`header-${theme}.png`) })
    }
  })
}
