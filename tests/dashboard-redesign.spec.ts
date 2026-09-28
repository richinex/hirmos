import { expect, test } from '@playwright/test'

// The type ramp (index.css): a 16px root, a 1.333 scale from 720px and a 1.2 scale on a phone; the card
// title is the title tier, its copy the body tier, its metadata the label tier; padding is 1.5rem.
for (const screen of [
  { width: 390, columns: 1, heading: 23.04 },
  { width: 900, columns: 2, heading: 28.4448 },
  { width: 1440, columns: 3, heading: 28.4448 },
]) {
  test(`reference typography and cards at ${screen.width}px`, async ({ page }, info) => {
    await page.setViewportSize({ width: screen.width, height: 1000 })
    await page.goto('/app/projects')
    await page.evaluate(() => document.fonts.ready)
    await expect(page.locator('.example-card').first()).toBeVisible()
    const metrics = await page.evaluate(() => {
      const style = (selector: string) => getComputedStyle(document.querySelector(selector)!)
      return {
        heading: parseFloat(style('.chapter-heading').fontSize),
        cardTitle: parseFloat(style('.example-card h3').fontSize),
        body: parseFloat(style('.example-card p').fontSize),
        metadata: parseFloat(style('.example-card-source').fontSize),
        padding: parseFloat(style('.example-card').paddingLeft),
        columns: style('.example-cards').gridTemplateColumns.split(' ').length,
        overflow: document.documentElement.scrollWidth - window.innerWidth,
      }
    })
    expect(metrics.heading).toBeCloseTo(screen.heading, 1)
    expect(metrics.cardTitle).toBeCloseTo(screen.width < 720 ? 19.2 : 21.3328, 1)
    expect(metrics.body).toBeCloseTo(13.3328, 1)
    expect(metrics.metadata).toBeCloseTo(11.1104, 1)
    expect({ padding: metrics.padding, columns: metrics.columns, overflow: metrics.overflow }).toEqual({ padding: 24, columns: screen.columns, overflow: 0 })
    for (const theme of ['light', 'dark']) {
      await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
      await page.waitForTimeout(250)
      await page.screenshot({ path: info.outputPath(`projects-${screen.width}-${theme}.png`) })
    }
  })
}

test('dashboard cards, responsive navigation and prepared data remain usable', async ({ page }, info) => {
  await page.emulateMedia({ colorScheme: 'dark' })
  await page.goto('/app/projects')
  await expect(page.getByRole('heading', { name: 'Projects', exact: true })).toBeVisible()
  await page.evaluate(() => document.fonts.ready)
  await expect(page.locator('.example-card')).toHaveCount(16)
  expect(await page.locator('.chapter-heading').evaluate(el => getComputedStyle(el).fontFamily)).toContain('Roboto Slab')
  await page.getByRole('searchbox', { name: 'Search examples' }).fill('company-wide')
  await expect(page.locator('.example-card')).toHaveCount(1)
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand section list' }).click()
  await expect(page.getByRole('navigation', { name: 'Workspace sections' })).toBeVisible()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /^Survival analysis/ }).click()
  await expect(page.getByRole('heading', { name: 'Survival analysis', exact: true })).toBeVisible()
  if (info.project.name === 'mobile-chromium') {
    await expect(page.getByRole('button', { name: 'Expand section list' })).toBeVisible()
  }
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)
  expect(overflow).toBeLessThanOrEqual(1)
  await page.screenshot({ path: info.outputPath('survival-dashboard.png') })
})
