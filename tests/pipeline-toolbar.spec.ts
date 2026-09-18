import { expect, test } from '@playwright/test'

test('block toolbar reveals overflow without moving the canvas', async ({ page }, info) => {
  if (info.project.name === 'chromium') await page.setViewportSize({ width: 860, height: 800 })
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Toolbar check')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.getByRole('button', { name: 'Open the editor' }).click()
  await expect(page.getByText('Opening DuckDB', { exact: true })).toHaveCount(0, { timeout: 30_000 })
  if (info.project.name === 'mobile-chromium') await page.setViewportSize({ width: 320, height: 850 })
  else {
    const seam = (await page.getByRole('separator', { name: 'Resize Pipeline', exact: true }).boundingBox())!
    await page.mouse.move(seam.x + seam.width / 2, seam.y + 40)
    await page.mouse.down()
    await page.mouse.move(seam.x - 240, seam.y + 40, { steps: 12 })
    await page.mouse.up()
  }
  const toolbar = page.getByRole('toolbar', { name: 'Add a block' })
  const scroll = toolbar.locator('[data-block-scroll]')
  await expect(toolbar).toBeVisible()
  const top = (await page.getByTestId('block-output').boundingBox())!.y
  await expect(toolbar.getByRole('button', { name: /Scroll blocks/ })).toHaveCount(0)
  const right = toolbar.locator('[data-overflow-edge="right"]')
  const left = toolbar.locator('[data-overflow-edge="left"]')
  await expect(right).toBeVisible()
  await scroll.evaluate(e => e.scrollTo({ left: e.scrollWidth, behavior: 'instant' }))
  await expect.poll(() => scroll.evaluate(e => e.scrollLeft)).toBeGreaterThan(0)
  await expect(left).toBeVisible()
  await expect(right).toBeHidden()
  expect((await page.getByTestId('block-output').boundingBox())!.y).toBe(top)
  await page.screenshot({ path: info.outputPath('toolbar-end.png') })
  await scroll.evaluate(e => e.scrollTo({ left: 0, behavior: 'instant' }))
  await expect(left).toBeHidden()
  await expect(right).toBeVisible()
  if (info.project.name === 'chromium') {
    await page.setViewportSize({ width: 2100, height: 1000 })
    await expect(right).toBeHidden()
    await expect(toolbar.locator('.pipeline-block-label').first()).toBeVisible()
  }
  await page.screenshot({ path: info.outputPath('toolbar-start.png') })
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
})

test('pane resizing preserves labels without shifting the canvas or nodes', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Desktop resizable panes')
  await page.setViewportSize({ width: 1500, height: 900 })
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Pane resize')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.getByRole('button', { name: 'Open the editor' }).click()
  await expect(page.getByText('Opening DuckDB', { exact: true })).toHaveCount(0, { timeout: 30_000 })
  const toolbar = page.getByRole('toolbar', { name: 'Add a block' })
  await expect(toolbar.locator('.pipeline-block-label').first()).toBeVisible()
  const geometry = () => page.evaluate(() => ({
    height: document.querySelector('[aria-label="Add a block"]')!.getBoundingClientRect().height,
    canvas: document.querySelector('[data-testid="pipeline-canvas"]')!.getBoundingClientRect().top,
    node: document.querySelector('[data-testid="block-output"]')!.getBoundingClientRect().top,
    transform: (document.querySelector('.react-flow__viewport') as HTMLElement).style.transform,
  }))
  const before = await geometry()
  const header = page.locator('.workbench-pane-header').filter({ has: page.locator('#pipeline-inspector-title') })
  const toolbarBox = (await toolbar.boundingBox())!
  const headerBox = (await header.boundingBox())!
  expect(headerBox.y + headerBox.height).toBeCloseTo(toolbarBox.y + toolbarBox.height, 1)
  const seam = (await page.getByRole('separator', { name: 'Resize Pipeline', exact: true }).boundingBox())!
  await page.mouse.move(seam.x + seam.width / 2, seam.y + 50)
  await page.mouse.down()
  await page.mouse.move(seam.x - 300, seam.y + 50, { steps: 20 })
  await page.mouse.up()
  for (const label of await toolbar.locator('.pipeline-block-label').all()) {
    await expect(label).toBeVisible()
  }
  expect(await geometry()).toEqual(before)
  await page.screenshot({ path: info.outputPath('narrow-pane.png') })
})
