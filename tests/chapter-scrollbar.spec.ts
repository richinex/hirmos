import { expect, test } from '@playwright/test'

test('panel, figure, editor and toolbar scrollbars are hidden at rest', async ({ page }) => {
  await page.goto('/app')
  await page.evaluate(() => {
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;background:var(--color-panel);display:grid;align-content:start;gap:12px;padding:12px'
    for (const name of ['panel-scroll', 'figure-strip', 'cm-scroller', 'unclassified-scroll']) {
      const element = document.createElement('div')
      element.className = name
      element.tabIndex = 0
      element.dataset.testid = name
      element.style.cssText = 'width:250px;height:80px;overflow:auto'
      const content = document.createElement('div')
      content.style.cssText = 'width:500px;height:200px'
      content.textContent = name
      element.append(content)
      host.append(element)
    }
    document.body.append(host)
  })
  const hidden = 'rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)'
  for (const name of ['panel-scroll', 'figure-strip', 'cm-scroller', 'unclassified-scroll']) {
    const surface = page.getByTestId(name)
    await expect(surface).toHaveCSS('scrollbar-color', hidden)
    const width = await surface.evaluate(element => element.clientWidth)
    await surface.focus()
    await page.keyboard.press('ArrowDown')
    await expect.poll(() => surface.evaluate(element => element.scrollTop)).toBeGreaterThan(0)
    await expect(surface).not.toHaveCSS('scrollbar-color', hidden)
    await surface.evaluate(element => { element.scrollLeft = 80 })
    await expect.poll(() => surface.evaluate(element => element.scrollLeft)).toBeGreaterThan(0)
    expect(await surface.evaluate(element => element.clientWidth)).toBe(width)
    await expect(surface).toHaveCSS('scrollbar-color', hidden)
  }
})

test('chapter scrollbar follows scrolling without shifting navigation', async ({ page }, info) => {
  await page.setViewportSize({ width: info.project.name === 'chromium' ? 1280 : 390, height: 480 })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Expand chapter list', exact: true }).click()
  const nav = page.getByRole('navigation', { name: 'Workspace chapters' })
  await expect(nav).toBeVisible()
  await expect.poll(() => nav.evaluate(element => element.getBoundingClientRect().left)).toBe(0)
  const hidden = 'rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)'
  await expect(nav).toHaveCSS('scrollbar-color', hidden)
  expect(await nav.evaluate(element => element.scrollHeight > element.clientHeight)).toBe(true)
  // The rail slides open; measure once its width has settled.
  await expect.poll(async () => { const first = await nav.evaluate(element => element.clientWidth); await page.waitForTimeout(200); return first === await nav.evaluate(element => element.clientWidth) }).toBe(true)
  const before = await nav.evaluate(element => ({ width: element.clientWidth, left: element.getBoundingClientRect().left }))
  await nav.evaluate(element => { element.scrollTop = 100 })
  await expect(nav).toHaveAttribute('data-scrolling', 'true')
  await expect(nav).not.toHaveCSS('scrollbar-color', hidden)
  expect(await nav.evaluate(element => ({ width: element.clientWidth, left: element.getBoundingClientRect().left }))).toEqual(before)
  await page.screenshot({ path: info.outputPath('chapters-scrolling.png') })
  await expect(nav).not.toHaveAttribute('data-scrolling', 'true')
  await expect(nav).toHaveCSS('scrollbar-color', hidden)
  await page.screenshot({ path: info.outputPath('chapters-idle.png') })
  await page.setViewportSize({ width: info.project.name === 'chromium' ? 1280 : 390, height: 1600 })
  await expect.poll(() => nav.evaluate(element => element.scrollHeight <= element.clientHeight)).toBe(true)
  await expect(nav).not.toHaveAttribute('data-scrolling', 'true')
  await expect(nav).toHaveCSS('scrollbar-color', hidden)
})
