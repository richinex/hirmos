import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { ardlLongRunSchema, plotTimeSchema } from '../src/domain/longRun'
import { parseTimeSeriesRun } from '../src/domain/timeSeries'

const fixture = JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/fixtures/long_run_charts.json', import.meta.url), 'utf8'))

test('long-run chart quantities match statsmodels in runtime WASM', async ({ page }) => {
  await page.goto('/app')
  const results = await page.evaluate(async ({ y, x, bridge }) => {
    const wasm = await import(new URL('/src/generated/analysis-wasm/hirmos_analysis.js', location.href).href)
    await wasm.default({})
    return bridge.map((entry: { command: unknown; values?: number[] }) => JSON.parse(wasm.runAnalysis(JSON.stringify(entry.command), new Float64Array(entry.values ?? [...y, ...x]), new Uint8Array(), new Uint8Array(), () => {})))
  }, fixture)
  for (let i = 0; i < results.length; i++) {
    const result = results[i]
    const oracle = fixture.bridge[i]
    expect(result.longRun.kind).toBe('recorded')
    const actual: number[] = result.longRun.departures.flat()
    const expected: number[] = oracle.departures.flat()
    expect(actual).toHaveLength(expected.length)
    actual.forEach((value, j) => expect(Math.abs(value - expected[j]!)).toBeLessThanOrEqual(1e-8 * Math.max(1, Math.abs(expected[j]!))))
    if (result.kind === 'ardlPss') expect(result.longRun.observed).toEqual(fixture.y)
    else {
      expect(result.rank).toBe(oracle.rank)
      expect(result.longRun.startRow).toBe(oracle.lag)
    }
  }
})

test('multiple VECM relationships retain their values and calendar dates when selected', async ({ page }, info) => {
  await page.goto('/app')
  const example = fixture.bridge.find((entry: { command: { columns: number } }) => entry.command.columns === 3)
  const expected = await page.evaluate(async (entry) => {
    const wasm = await import(new URL('/src/generated/analysis-wasm/hirmos_analysis.js', location.href).href)
    await wasm.default({})
    const evidence = JSON.parse(wasm.runAnalysis(JSON.stringify(entry.command), new Float64Array(entry.values), new Uint8Array(), new Uint8Array(), () => {}))
    const domain = await import(new URL('/src/domain/timeSeries.ts', location.href).href)
    const { LongRunCharts } = await import(new URL('/src/components/time-series/LongRunCharts.tsx', location.href).href)
    const { default: React } = await import(new URL('/node_modules/.vite/deps/react.js', location.href).href)
    const { default: ReactDOM } = await import(new URL('/node_modules/.vite/deps/react-dom_client.js', location.href).href)
    const values = Array.from({ length: evidence.observations }, (_, i) => Date.UTC(2000, i, 1))
    const parsed = domain.parseTimeSeriesRun({ kind: 'vecm', id: 'chart-review', preparedDataset: 'p', createdAt: '2026-09-13T10:00:00.000Z', variables: ['Output', 'Income', 'Spending'].map(id => ({ id, name: id })), specification: { maxLags: 3, deterministic: 'ci', significance: 95 }, evidence, plotTime: { kind: 'calendar', values } })
    if (!parsed.ok) throw Error(parsed.error)
    const host = document.createElement('div')
    document.getElementById('root')!.style.display = 'none'
    host.className = 'flex flex-col gap-4 text-muted'
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;overflow:auto;padding:24px;background:var(--color-panel)'
    document.body.append(host)
    ReactDOM.createRoot(host).render(React.createElement(LongRunCharts, { run: parsed.value }))
    // Display-state fixtures only: no numerical claims about these synthetic matrices.
    const hidden = [0, 3].map(rank => {
      const matrix = rank === 0 ? [] : Array.from({ length: 3 }, () => [0, 0, 0])
      const longRun = rank === 0 ? { kind: 'notFitted' } : { ...evidence.longRun, departures: Array.from({ length: 3 }, () => evidence.longRun.departures[0]) }
      const state = domain.parseTimeSeriesRun({ ...parsed.value, evidence: { ...evidence, rank, longRun, alpha: matrix, beta: matrix, pvaluesAlpha: matrix } })
      if (!state.ok) throw Error(state.error)
      return LongRunCharts({ run: state.value }) === null
    })
    return { rank: evidence.rank, firstDate: values[evidence.longRun.startRow], firstValue: evidence.longRun.departures[1][0], hidden }
  }, example)
  expect(expected.rank).toBe(2)
  expect(expected.hidden).toEqual([true, true])
  await page.getByRole('combobox', { name: 'Long-run relationship' }).click()
  await page.getByRole('option', { name: 'Relationship 2', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Departure from long-run relationship 2' })).toBeVisible()
  await expect.poll(() => page.evaluate(async () => {
    const charts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    const element = document.querySelector('[data-testid=long-run-charts] [_echarts_instance_]')
    return element ? charts.getInstanceByDom(element).getOption().series[0].data[0] : null
  })).toEqual([expected.firstDate, expected.firstValue])
  expect(await page.locator('body').evaluate(el => el.scrollWidth <= innerWidth + 1)).toBe(true)
  await page.screenshot({ path: info.outputPath('vecm-relationships.png') })
})

test('chart boundaries reject mismatched rows, dates and relationship dimensions', () => {
  expect(ardlLongRunSchema.safeParse({ kind: 'recorded', observed: [1, 2], departures: [1] }).success).toBe(false)
  expect(plotTimeSchema.safeParse({ kind: 'ordinal', values: [1, 1, 2] }).success).toBe(false)
  expect(plotTimeSchema.safeParse({ kind: 'calendar', values: [3, 2, 1] }).success).toBe(false)
  expect(ardlLongRunSchema.safeParse({ kind: 'recorded', observed: [Infinity], departures: [0] }).success).toBe(false)
  const matrix = [[1], [-1]]
  const base = { kind: 'vecm', id: 'v', preparedDataset: 'p', createdAt: '2026-09-13T10:00:00.000Z', variables: [{ id: 'a', name: 'a' }, { id: 'b', name: 'b' }], specification: { maxLags: 2, deterministic: 'ci', significance: 95 }, plotTime: { kind: 'ordinal', values: [10, 20, 30, 40, 50] }, evidence: { kind: 'vecm', observations: 5, deterministic: 'ci', kArDiff: 1, rank: 1, significance: 1, longRunEffect: null, alpha: matrix, beta: matrix, gamma: [], pvaluesAlpha: matrix, chow: null, longRun: { kind: 'recorded', startRow: 1, departures: [[1, 2, 3]] } } }
  expect(parseTimeSeriesRun(base).ok).toBe(true)
  for (const invalid of [{ kind: 'notFitted' }, { kind: 'recorded', startRow: 2, departures: [[1, 2, 3]] }, { kind: 'recorded', startRow: 1, departures: [[1, 2]] }, { kind: 'recorded', startRow: 1, departures: [[1, 2, 3], [1, 2, 3]] }]) expect(parseTimeSeriesRun({ ...base, evidence: { ...base.evidence, longRun: invalid } }).ok).toBe(false)
})
