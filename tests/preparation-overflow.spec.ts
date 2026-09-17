import { expect, test, type Locator } from '@playwright/test'

async function choose(trigger: Locator, option: string) {
  await trigger.click()
  await trigger.page().getByRole('option', { name: option, exact: true }).click()
}

test('preparation remains inside the stage with all real-data columns selected', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Preparation width')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles('docs/2026-09-17-richdata01/data/105w_tcsp_weekly.parquet')
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'year_week')
  await choose(page.getByLabel('Time interpretation'), 'ISO week (2024-W02)')
  await choose(page.getByLabel('Source frequency'), 'Weekly')
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  const transforms = page.getByRole('group', { name: 'Transformations by column', exact: true })
  await transforms.evaluate((element) => element.scrollIntoView({ block: 'start' }))
  const overflow = await transforms.evaluate((element) => {
    const ancestors = []
    for (let node: HTMLElement | null = element as HTMLElement; node; node = node.parentElement) {
      ancestors.push({ tag: node.tagName, classes: node.className, width: node.clientWidth, scroll: node.scrollWidth, left: node.scrollLeft })
    }
    return ancestors
  })
  await info.attach('ancestor-widths', { body: JSON.stringify(overflow, null, 2), contentType: 'application/json' })
  expect(overflow.filter((node) => node.scroll > node.width + 2)).toEqual([])
  const preview = page.locator('table.table-fixed').locator('..')
  const before = await transforms.boundingBox()
  expect(before).not.toBeNull()
  const moved = await preview.evaluate((element) => {
    element.scrollLeft = 100_000
    return { left: element.scrollLeft, width: element.clientWidth, scroll: element.scrollWidth, mode: getComputedStyle(element).overflowX }
  })
  expect(moved.left).toBeGreaterThan(0)
  expect(moved.scroll).toBeGreaterThan(moved.width)
  expect(moved.mode).toBe('auto')
  expect((await transforms.boundingBox())?.x).toBe(before?.x)
  for (const width of info.project.name === 'chromium' ? [768, 1280, 1920] : [390, 412]) {
    await page.setViewportSize({ width, height: 900 })
    await transforms.evaluate((element) => element.scrollIntoView({ block: 'start' }))
    await expect.poll(() => transforms.evaluate((element) => {
      for (let node: HTMLElement | null = element as HTMLElement; node; node = node.parentElement) {
        node.scrollLeft = 100_000
        if (node.scrollLeft > 0 || node.scrollWidth > node.clientWidth + 2) return false
      }
      return true
    })).toBe(true)
    const bounds = await transforms.boundingBox()
    if (bounds === null) throw new Error('Missing transformation controls')
    await page.mouse.move(bounds.x + Math.min(bounds.width / 2, 150), Math.max(120, bounds.y + 50))
    await page.mouse.down()
    await page.mouse.move(10, 160, { steps: 8 })
    await page.mouse.up()
    await page.evaluate(() => window.getSelection()?.removeAllRanges())
    expect((await transforms.boundingBox())?.x).toBe(bounds.x)
    await page.screenshot({ path: info.outputPath(`preparation-${width}.png`), fullPage: true })
  }
  await page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  const plots = page.getByRole('list', { name: 'Prepared series plots', exact: true })
  await expect(plots.getByTestId('prepared-series').first()).toBeVisible({ timeout: 30_000 })
  for (const width of info.project.name === 'chromium' ? [1280, 1920, 1024] : [390, 412]) {
    await page.setViewportSize({ width, height: 900 })
    await expect.poll(() => plots.evaluate((element) => Array.from(element.querySelectorAll('[data-testid="prepared-series"]')).every((chart) => {
      const card = chart.closest('ul[aria-label$="preparation stages"]')?.parentElement
      const drawing = chart.querySelector('svg, canvas')
      if (card === null || card === undefined || drawing === null) return false
      const a = card.getBoundingClientRect(), b = drawing.getBoundingClientRect()
      return b.left >= a.left && b.right <= a.right && chart.clientWidth <= card.clientWidth
    })), { timeout: 10_000 }).toBe(true)
    await plots.getByTestId('prepared-series').first().evaluate((element) => element.scrollIntoView({ block: 'center' }))
    await page.screenshot({ path: info.outputPath(`prepared-plots-${width}.png`), fullPage: true })
  }
  await page.getByRole('button', { name: 'Open active_components, Saved values in a floating window' }).click()
  await expect(page.getByRole('dialog')).toBeVisible()
  await page.getByRole('button', { name: 'Close the floating window' }).click()
  await expect(page.getByRole('dialog')).toHaveCount(0)
  if (info.project.name === 'chromium') {
    const handle = page.getByRole('separator', { name: 'Resize Column profile' })
    const bounds = await handle.boundingBox()
    if (bounds === null) throw new Error('Missing pane resize handle')
    await page.mouse.move(bounds.x + bounds.width / 2, bounds.y + 50)
    await page.mouse.down()
    await page.mouse.move(bounds.x - 100, bounds.y + 50, { steps: 10 })
    await page.mouse.up()
  }
  await expect.poll(() => plots.evaluate((element) => Array.from(element.querySelectorAll('[data-testid="prepared-series"]')).every((chart) => {
    const card = chart.closest('ul[aria-label$="preparation stages"]')?.parentElement
    const drawing = chart.querySelector('svg, canvas')
    return card !== null && card !== undefined && drawing !== null && drawing.getBoundingClientRect().right <= card.getBoundingClientRect().right
  }))).toBe(true)
})
