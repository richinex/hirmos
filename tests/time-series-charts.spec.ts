import { expect, test } from '@playwright/test'

test('count plots share one time window through theme changes and expansion', async ({ page }, info) => {
  test.setTimeout(90000)
  // Delay delivery, not the numerical result, to inspect the real in-progress UI deterministically.
  await page.addInitScript(() => {
    const post = Worker.prototype.postMessage
    Worker.prototype.postMessage = function (message: unknown, options?: Transferable[] | StructuredSerializeOptions) {
      const args = [message, options]
      if (message !== null && typeof message === 'object' && Reflect.get(message, 'kind') === 'count-series-intervention-scan') {
        setTimeout(() => Reflect.apply(post, this, args), 700)
        return
      }
      Reflect.apply(post, this, args)
    }
  })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open Campylobacter cases and an outbreak step', exact: true }).click()
  const expand = page.getByRole('button', { name: 'Expand chapter list' })
  if (info.project.name === 'mobile-chromium' && await expand.isVisible()) await expand.click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Time-series analysis/ }).click()
  await page.getByRole('heading', { name: 'Time-series analysis', exact: true }).waitFor()
  await page.getByRole('combobox', { name: 'Count series' }).click()
  await page.getByRole('option', { name: 'cases', exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Candidate start row' }).fill('98')
  await page.getByRole('spinbutton', { name: 'Candidate end row' }).fill('102')
  const setup = page.getByRole('region', { name: 'Time-series setup' })
  const before = await setup.boundingBox()
  const original = await setup.elementHandle()
  await page.getByRole('button', { name: 'Fit and scan', exact: true }).click()
  await expect(page.getByLabel('Count model running', { exact: true })).toBeVisible()
  expect(await original!.evaluate(el => el.isConnected)).toBe(true)
  const during = await setup.boundingBox()
  expect(Math.abs(during!.height - before!.height)).toBeLessThan(1)
  await expect(page.getByRole('button', { name: 'Fit and scan', exact: true })).toHaveAttribute('aria-busy', 'true')
  await page.locator('[data-testid=count-fit-plot] [_echarts_instance_]').first().waitFor()
  await page.evaluate(async () => {
    const echarts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    echarts.getInstanceByDom(document.querySelector('[data-testid=count-fit-plot] [_echarts_instance_]')).dispatchAction({ type: 'dataZoom', startValue: 90, endValue: 110 })
  })
  const ranges = () => page.evaluate(async () => {
    const echarts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    return [...document.querySelectorAll('[_echarts_instance_]')].filter(el => el.checkVisibility()).map((el) => {
      const zoom = echarts.getInstanceByDom(el).getOption().dataZoom?.[0]
      return zoom ? [zoom.startValue, zoom.endValue] : null
    }).filter(Boolean)
  })
  await expect.poll(ranges).toEqual([[90, 110], [90, 110]])
  for (let index = 0; index < 4; index++) {
    await page.getByRole('button', { name: 'Change theme', exact: true }).click()
    await expect.poll(ranges).toEqual([[90, 110], [90, 110]])
    await page.screenshot({ path: info.outputPath(`theme-${index}-plots.png`) })
  }
  await page.getByTestId('count-fit-plot').filter({ visible: true }).getByRole('button', { name: /floating window/ }).click()
  await expect.poll(ranges).toEqual([[90, 110], [90, 110], [90, 110]])
  await page.getByRole('button', { name: 'Close the floating window', exact: true }).click()
  const equation = page.getByTestId('time-series-equation').filter({ visible: true })
  await equation.getByText('Model equation', { exact: true }).click()
  await expect(equation.locator('.katex')).toHaveCount(4)
  await expect(equation.locator('.katex-error')).toHaveCount(0)
  await equation.scrollIntoViewIfNeeded()
  expect(await page.locator('body').evaluate(e => e.scrollWidth <= innerWidth + 1)).toBe(true)
  await page.screenshot({ path: info.outputPath('count-equation.png') })
})
