import { expect, test, type Locator, type Page } from '@playwright/test'

async function scale(viewport: Locator) {
  return viewport.locator('.react-flow__viewport').evaluate((element) => new DOMMatrix(getComputedStyle(element).transform).a)
}

async function verify(page: Page, viewport: Locator, mobile: boolean) {
  const toolbar = viewport.getByRole('toolbar', { name: 'Canvas', exact: true })
  await expect(toolbar).toBeVisible()
  const buttons = await toolbar.getByRole('button').all()
  const a = (await buttons[0].boundingBox())!
  const b = (await buttons[1].boundingBox())!
  const box = (await viewport.boundingBox())!
  const controls = (await toolbar.boundingBox())!
  if (mobile) {
    expect(Math.abs(a.y - b.y)).toBeLessThan(1)
    expect(Math.abs(controls.x + controls.width / 2 - box.x - box.width / 2)).toBeLessThan(2)
  } else {
    expect(Math.abs(a.x - b.x)).toBeLessThan(1)
    expect(Math.abs(controls.x + controls.width - box.x - box.width + 8)).toBeLessThan(2)
  }
  expect(Math.abs(controls.y + controls.height - box.y - box.height + 8)).toBeLessThan(2)
  const chart = viewport.locator('[_echarts_instance_]')
  await expect(chart).toHaveCount(1)
  const instance = await chart.getAttribute('_echarts_instance_')
  const initial = await scale(viewport)
  await toolbar.getByRole('button', { name: 'Zoom in', exact: true }).click()
  await expect.poll(() => scale(viewport)).toBeGreaterThan(initial * 1.1)
  await toolbar.getByRole('button', { name: 'Zoom out', exact: true }).click()
  await expect.poll(async () => Math.abs(await scale(viewport) - initial)).toBeLessThan(0.005)

  // Start inside the drawing, not only on an empty part of the viewport.
  const drawing = (await chart.boundingBox())!
  const x = Math.max(box.x + 20, drawing.x + drawing.width / 2)
  const y = Math.max(box.y + 20, drawing.y + drawing.height / 2)
  const before = await viewport.locator('.react-flow__viewport').getAttribute('style')
  if (mobile) {
    const cdp = await page.context().newCDPSession(page)
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y }] })
    for (let step = 1; step <= 6; step++) await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: x + step * 8, y: y + step * 4 }] })
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] })
    await cdp.detach()
  } else {
    await page.mouse.move(x, y)
    await page.mouse.down()
    await page.mouse.move(x + 48, y + 24, { steps: 6 })
    await page.mouse.up()
  }
  await expect(viewport.locator('.react-flow__viewport')).not.toHaveAttribute('style', before!)
  await toolbar.getByRole('button', { name: 'Fit lag grid' }).click()
  await expect.poll(async () => {
    const fit = (await chart.boundingBox())!
    return fit.x >= box.x && fit.y >= box.y && fit.x + fit.width <= box.x + box.width && fit.y + fit.height <= box.y + box.height
  }).toBe(true)
  await expect(chart).toHaveAttribute('_echarts_instance_', instance!)
  expect(await viewport.evaluate((element) => element.scrollWidth <= element.clientWidth && element.scrollHeight <= element.clientHeight)).toBe(true)
  if (mobile) {
    const beforePinch = await scale(viewport)
    const cx = box.x + box.width / 2
    const cy = box.y + box.height / 2
    const cdp = await page.context().newCDPSession(page)
    const points = (distance: number) => [{ id: 0, x: cx - distance, y: cy }, { id: 1, x: cx + distance, y: cy }]
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: points(20) })
    for (let distance = 25; distance <= 50; distance += 5) await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: points(distance) })
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] })
    await cdp.detach()
    await expect.poll(() => scale(viewport)).toBeGreaterThan(beforePinch * 1.2)
    await toolbar.getByRole('button', { name: 'Fit lag grid' }).click()
    await expect.poll(async () => Math.abs(await scale(viewport) - beforePinch)).toBeLessThan(0.005)
  }
}

test('lag-grid uses DAG controls and pans, zooms and fits without remounting the chart', async ({ page }, info) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ default: React }, { default: ReactDOM }, { LagGraphViews }] = await Promise.all([
      load('/node_modules/.vite/deps/react.js'), load('/node_modules/.vite/deps/react-dom_client.js'), load('/src/components/discovery/LagGraphViews.tsx'),
    ])
    document.getElementById('root')!.style.display = 'none'
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;overflow:auto;padding:12px;background:var(--color-panel)'
    document.body.append(host)
    const variables = Array.from({ length: 24 }, (_, i) => ({ id: `v${i}`, name: `Service ${i} with a long variable name`, latent: false }))
    const links = variables.slice(1).map((_, i) => ({ from: i, to: i + 1, lag: 1 + i % 8, fromEndpoint: 'tail', toEndpoint: 'arrow', mark: '-->', strength: { kind: 'signed-unit', value: 0.5 } }))
    ReactDOM.createRoot(host).render(React.createElement(LagGraphViews, { graph: { variables, links, tauMax: 8, semantics: 'stationary-lag-graph' }, label: 'Lag viewport test', initial: 'lag-grid' }))
  })
  const viewport = page.getByTestId('lag-graph-viewport')
  await expect(viewport.locator('[_echarts_instance_]')).toHaveCount(1)
  await verify(page, viewport, info.project.name === 'mobile-chromium')
  await page.screenshot({ path: info.outputPath('lag-grid-inline.png') })
  await page.getByRole('button', { name: 'Open Lag viewport test in a floating window' }).click()
  const dialog = page.getByRole('dialog', { name: 'Lag viewport test' })
  await verify(page, dialog.getByTestId('lag-graph-viewport'), info.project.name === 'mobile-chromium')
  await page.screenshot({ path: info.outputPath('lag-grid-expanded.png') })
  await dialog.getByRole('button', { name: 'Close the floating window' }).click()
  await expect(viewport.getByRole('toolbar', { name: 'Canvas' })).toBeVisible()
})
