import { expect, test } from '@playwright/test'

test('desktop intake methods share a row without crowding the file chooser', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Intake layout')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  const methods = page.getByRole('radiogroup', { name: 'Data input method' })
  await expect(methods).toBeVisible()
  const widths = info.project.name === 'chromium' ? [1024, 1280, 1440] : [393]
  for (const width of widths) {
    await page.setViewportSize({ width, height: 900 })
    const tops = await methods.locator('[data-segment-option]').evaluateAll((labels) => labels.map((label) => label.getBoundingClientRect().top))
    if (width >= 1024) expect(Math.max(...tops) - Math.min(...tops)).toBeLessThan(2)
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
    await expect(page.getByText('Choose data file', { exact: true })).toBeVisible()
    await page.screenshot({ path: info.outputPath(`intake-${width}.png`) })
  }
})
