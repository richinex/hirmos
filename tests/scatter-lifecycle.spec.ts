import { expect, test } from '@playwright/test'

test('expanding a dense scatter matrix does not redraw unchanged charts', async ({ page }, info) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ React, createRoot }, { ExpandableChart }, { scatterMatrixOption }, { readChartTheme }] = await Promise.all([
      load('/tests/support/reactRuntime.ts'), load('/src/charts/ExpandableChart.tsx'), load('/src/charts/data/scatterMatrix.ts'), load('/src/charts/theme.ts'),
    ])
    const host = document.createElement('div')
    document.getElementById('root')!.style.display = 'none'
    host.style.cssText = 'position:fixed;inset:100px 0 0;z-index:1;background:white;overflow:auto;padding:20px'
    document.body.append(host)
    const columns = Array.from({ length: 6 }, (_, c) => ({ name: `Variable ${c+1}`, values: Array.from({ length: 10000 }, (_, r) => 0.1+c+Math.sin(r*0.02+c)+r/10000) }))
    createRoot(host).render(React.createElement(ExpandableChart, { option: scatterMatrixOption(columns, readChartTheme()), label: 'Dense scatter lifecycle', testId: 'dense-scatter', defaultWidth: 1100, defaultHeight: 800, style: { height: 700 } }))
  })
  await expect(page.getByTestId('dense-scatter')).not.toHaveAttribute('aria-busy', 'true')
  await page.evaluate(async () => {
    await document.fonts.ready
    const charts = await import(new URL('/tests/support/echartsRuntime.ts', location.href).href)
    const chart = charts.getInstanceByDom(document.querySelector('[data-testid=dense-scatter]'))
    const prototype = Object.getPrototypeOf(chart)
    const calls: {method:string; expanded:boolean}[] = []
    Reflect.set(window, 'chartCalls', calls)
    for (const method of ['setOption', 'resize', 'dispose']) {
      const original = prototype[method]
      prototype[method] = function (...args: unknown[]) {
        calls.push({ method, expanded: this.getDom().closest('[role=dialog]') !== null })
        return original.apply(this, args)
      }
    }
  })
  await page.getByRole('button', { name: 'Open Dense scatter lifecycle in a floating window' }).click()
  const dialog = page.getByRole('dialog', { name: 'Dense scatter lifecycle' })
  await expect(dialog.locator('[role=img]')).not.toHaveAttribute('aria-busy', 'true')
  await page.waitForTimeout(1000)
  const calls = await page.evaluate(() => Reflect.get(window, 'chartCalls'))
  console.log(JSON.stringify(calls))
  await info.attach('chart-calls', { body: JSON.stringify(calls), contentType: 'application/json' })
  expect(calls.filter((call: {method:string;expanded:boolean}) => !call.expanded)).toEqual([])
  expect(calls.filter((call: {method:string}) => call.method === 'setOption')).toHaveLength(1)
  expect(calls.filter((call: {method:string}) => call.method === 'resize')).toHaveLength(0)
  await page.screenshot({ path: info.outputPath('expanded-scatter.png') })
  await dialog.locator('[role=img]').evaluate((element) => { element.style.width = '85%' })
  await expect.poll(() => page.evaluate(() => Reflect.get(window, 'chartCalls').filter((call: {method:string;expanded:boolean}) => call.expanded && call.method === 'resize').length)).toBeGreaterThan(0)
  expect(await page.evaluate(() => Reflect.get(window, 'chartCalls').filter((call: {expanded:boolean}) => !call.expanded))).toEqual([])
  await dialog.getByRole('button', { name: 'Close the floating window' }).click()
})
