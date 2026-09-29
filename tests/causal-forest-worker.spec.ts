import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

const fixture = JSON.parse(readFileSync('crates/causal-core/oracle/grf/fixtures/nuisance.json', 'utf8'))

test('binary and continuous forest workers reproduce pinned R results', async ({ page }) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  for (const source of fixture.cases.slice(0, 2)) {
    const result = await page.evaluate(async (source) => {
      const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
      const domain = await import(new URL('/src/domain/causalForest.ts', location.href).href)
      const values = new Float64Array([source.W, source.Y, ...source.X].flat().map(Number))
      const binary = source.W.every((v: string | number) => Number(v) === 0 || Number(v) === 1)
      const configuration = { ...domain.DEFAULT_CAUSAL_FOREST, trees: 100, seed: source.seed, variablesPerSplit: { kind: 'specified', count: 4 } }
      const design = { treatment: 0, outcome: 1, adjustment: [2, 3, 4, 5], configuration,
        target: { kind: binary ? 'binary-average' : 'continuous-average', population: 'all' } }
      const result = await analysis.runCausalForest(values.slice(), source.Y.length, 6, design)
      if (!result.ok) throw new Error(JSON.stringify(result.error))
      const again = await analysis.runCausalForest(values.slice(), source.Y.length, 6, design)
      if (!again.ok) throw new Error(JSON.stringify(again.error))
      return { result: result.value, repeated: JSON.stringify(result.value) === JSON.stringify(again.value) }
    }, source)
    expect(result.repeated).toBe(true)
    expect(result.result.summary.kind).toBe('estimated')
    expect(result.result.summary.estimate).toBeCloseTo(Number(source.ate[0]), 7)
    expect(result.result.summary.standardError).toBeCloseTo(Number(source.ate[1]), 7)
    expect(result.result.predictions).toHaveLength(source.Y.length)
    for (const [index, row] of result.result.predictions.entries()) {
      expect(row.kind).toBe('estimated')
      expect(row.estimate).toBeCloseTo(Number(source.prediction[index].predictions), 7)
      expect(row.uncertainty.standardError ** 2).toBeCloseTo(Number(source.prediction[index]['variance.estimates']), 7)
    }
  }
})

test('forest controls and results follow existing desktop and mobile chart conventions', async ({ page }, testInfo) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.evaluate(async (source) => {
    const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
    const domain = await import(new URL('/src/domain/causalForest.ts', location.href).href)
    const { CausalForestResults } = await import(new URL('/src/components/estimation/CausalForestResults.tsx', location.href).href)
    const { CausalForestControls } = await import(new URL('/src/components/estimation/CausalForestControls.tsx', location.href).href)
    const { default: React } = await import(new URL('/node_modules/.vite/deps/react.js', location.href).href)
    const { default: ReactDOM } = await import(new URL('/node_modules/.vite/deps/react-dom_client.js', location.href).href)
    const configuration = { ...domain.DEFAULT_CAUSAL_FOREST, trees: 100, seed: source.seed, variablesPerSplit: { kind: 'specified', count: 4 } }
    const fitted = await analysis.runCausalForest(new Float64Array([source.W, source.Y, ...source.X].flat().map(Number)), source.Y.length, 6,
      { treatment: 0, outcome: 1, adjustment: [2,3,4,5], configuration, target: { kind: 'binary-average', population: 'all' } })
    if (!fitted.ok) throw new Error(JSON.stringify(fitted.error))
    // Isolated component verification, not an assertion that the Study route is integrated.
    const host = document.createElement('main')
    host.id = 'forest-component-check'
    host.className = '@container/panel bg-panel text-ink p-4'
    host.style.cssText = 'position:fixed;inset:0;overflow:auto;z-index:9999'
    document.body.append(host)
    function Example() {
      const [settings, update] = React.useState(configuration)
      return React.createElement('div', { className: 'mx-auto max-w-5xl space-y-8' },
        React.createElement(CausalForestControls, { configuration: settings, onChange: update }),
        React.createElement(CausalForestResults, { evidence: fitted.value, outcome: 'Spending' }))
    }
    ReactDOM.createRoot(host).render(React.createElement(Example))
  }, fixture.cases[0])
  const host = page.locator('#forest-component-check')
  await expect(host.getByLabel('Trees', { exact: true })).toHaveValue('100')
  await host.getByRole('radio', { name: 'Automatic tuning', exact: true }).click()
  await expect(host.getByText('Tuning budget', { exact: true })).toBeVisible()
  await host.getByRole('radio', { name: 'Specified settings', exact: true }).click()
  for (const theme of ['light', 'dark']) {
    await page.evaluate(theme => { document.documentElement.dataset.theme = theme }, theme)
    await host.getByTestId('causal-forest-controls').scrollIntoViewIfNeeded()
    await page.screenshot({ path: testInfo.outputPath(`forest-controls-${theme}.png`) })
    const plot = host.getByTestId('causal-forest-predictions')
    await plot.scrollIntoViewIfNeeded()
    await expect(plot.locator('svg')).toBeVisible()
    await page.screenshot({ path: testInfo.outputPath(`forest-result-${theme}.png`) })
    const width = await host.evaluate(element => ({ scroll: element.scrollWidth, client: element.clientWidth }))
    expect(width.scroll).toBeLessThanOrEqual(width.client + 1)
    await expect(host.getByText('Each prediction estimates an average treatment effect conditional on the recorded characteristics. It is not an observed individual treatment effect.', { exact: true })).toBeVisible()
  }
  expect(errors).toEqual([])
})
