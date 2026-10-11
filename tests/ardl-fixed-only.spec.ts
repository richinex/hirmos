import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { ardlModelRequestSchema } from '../src/domain/ardlModel'
import { chapter, choose, prepare } from './examples/support'

test('the controls fit fixed-only and pure autoregressive models without placeholder columns', async ({
  page,
}, info) => {
  const item = reference.cases[0]
  const csv = [
    'date,y,z0,z1',
    ...item.y.map((y: number, i: number) =>
      [
        new Date(Date.UTC(2020, 0, i + 1)).toISOString().slice(0, 10),
        y,
        item.fixed[0][i],
        item.fixed[1][i],
      ].join(','),
    ),
  ].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('ARDL fixed-only controls')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page
    .locator('input[type=file]')
    .first()
    .setInputFiles({ name: 'ardl.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await prepare(page, {
    structure: 'time series',
    time: 'date',
    frequency: 'Daily',
    columns: ['y', 'z0', 'z1'],
  })
  await chapter(page, /Time-series analysis/)
  await page.getByRole('radio', { name: 'ARDL', exact: true }).click()
  await choose(page, 'Outcome series', 'y')
  await page.getByRole('radio', { name: 'Specify lags', exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Outcome lag', exact: true }).fill('2')
  for (const name of ['z0', 'z1']) {
    await page.getByRole('checkbox', { name, exact: true }).check()
    await choose(page, `Role for ${name}`, 'Fixed regressor')
  }
  await page
    .getByRole('checkbox', { name: 'Forecast with supplied future values', exact: true })
    .check()
  for (const [i, name] of ['z0', 'z1'].entries())
    await page
      .getByRole('textbox', { name: `Future ${name}`, exact: true })
      .fill(item.futureFixed[i].join(', '))
  await page.getByRole('button', { name: 'Fit ARDL', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Time-series result', exact: true })).toBeVisible()
  await expect(
    page.getByTestId('canvas').getByText(/The fitted model contains no distributed-lag predictors/),
  ).toBeVisible()
  await page.screenshot({ path: info.outputPath('fixed-only.png'), fullPage: true })
  for (const name of ['z0', 'z1']) await page.getByRole('checkbox', { name, exact: true }).uncheck()
  await page.getByRole('spinbutton', { name: 'Forecast periods', exact: true }).fill('5')
  await page.getByRole('radio', { name: 'Select by AIC', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Fit ARDL', exact: true })).toBeDisabled()
  await expect(
    page.getByText(
      'Automatic lag selection is unavailable without lagged predictors. Choose “Specify lags” to fit this model.',
      { exact: true },
    ),
  ).toBeVisible()
  await page.getByRole('radio', { name: 'Specify lags', exact: true }).click()
  await page.getByRole('button', { name: 'Fit ARDL', exact: true }).click()
  await expect
    .poll(async () =>
      page.evaluate(async () => {
        const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
        const header = (await store.listProjects()).find(
          (p: { name: string }) => p.name === 'ARDL fixed-only controls',
        )
        if (!header) return 0
        const loaded = await store.loadProject(header.id)
        return loaded.ok ? loaded.value.timeSeriesRuns.length : 0
      }),
    )
    .toBe(2)
  await page
    .getByRole('region', { name: 'Time-series result', exact: true })
    .scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('autoregressive.png') })
})

const reference = JSON.parse(
  readFileSync(
    new URL(
      '../../octopus/rust-causal-transpile/oracle/fixtures/ardl_fixed_only.json',
      import.meta.url,
    ),
    'utf8',
  ),
)
const base = {
  outcome: 0,
  predictors: [],
  fixed: [1],
  terms: 'constant',
  orders: { kind: 'fixed', outcomeLag: 2, predictorLags: [] },
  holdBack: null,
  multiplierHorizon: 5,
  future: { kind: 'none' },
}

test('fixed-only requests distinguish supported fits, forecasts and unavailable selection', () => {
  expect(ardlModelRequestSchema.safeParse(base).success).toBe(true)
  expect(
    ardlModelRequestSchema.safeParse({
      ...base,
      future: { kind: 'scenario', predictors: [], fixed: [[1, 1]], confidence: 0.95 },
    }).success,
  ).toBe(true)
  expect(
    ardlModelRequestSchema.safeParse({
      ...base,
      fixed: [],
      future: { kind: 'autoregressive', horizon: 5, confidence: 0.95 },
    }).success,
  ).toBe(true)
  for (const change of [
    { orders: { kind: 'search', maximumLag: 4, maximumOrders: [] } },
    { orders: { kind: 'rFixed', outcomeLag: 2, predictorLags: [] } },
    { orders: { kind: 'fixed', outcomeLag: 2, predictorLags: [1] } },
    { future: { kind: 'autoregressive', horizon: 5, confidence: 0.95 } },
    { future: { kind: 'scenario', predictors: [], fixed: [], confidence: 0.95 } },
    { fixed: [], future: { kind: 'autoregressive', horizon: 0, confidence: 0.95 } },
  ])
    expect(ardlModelRequestSchema.safeParse({ ...base, ...change }).success).toBe(false)
})

for (const item of reference.cases) {
  test(`${item.name}: worker fit, forecast and saved run match statsmodels ${reference.version}`, async ({
    page,
  }) => {
    await page.goto('/app')
    const result = await page.evaluate(async (item) => {
      const domain = await import(new URL('/src/domain/ardlModel.ts', location.href).href)
      const client = await import(new URL('/src/analysis/client.ts', location.href).href)
      const runs = await import(new URL('/src/domain/timeSeries.ts', location.href).href)
      const request = domain.ardlModelRequestSchema.parse({
        outcome: 0,
        predictors: [],
        fixed: item.fixed.map((_: unknown, i: number) => i + 1),
        terms: item.trend === 'ct' ? 'trend' : 'constant',
        orders: { kind: 'fixed', outcomeLag: item.p, predictorLags: [] },
        holdBack: item.holdBack,
        multiplierHorizon: 5,
        future: item.fixed.length
          ? { kind: 'scenario', predictors: [], fixed: item.futureFixed, confidence: 0.95 }
          : { kind: 'autoregressive', horizon: 5, confidence: 0.95 },
      })
      const result = await client.runArdlModel(
        new Float64Array([item.y, ...item.fixed].flat()),
        item.y.length,
        1 + item.fixed.length,
        request,
      )
      if (!result.ok) return { result, saved: null }
      const saved = runs.parseTimeSeriesRun({
        kind: 'ardl-model',
        id: crypto.randomUUID(),
        preparedDataset: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        outcome: { id: 'y', name: 'y' },
        predictors: [],
        fixed: item.fixed.map((_: unknown, i: number) => ({ id: `z${i}`, name: `z${i}` })),
        specification: request,
        evidence: result.value,
      })
      return { result, saved }
    }, item)
    expect(result.result.ok, JSON.stringify(result)).toBe(true)
    expect(result.saved?.ok, JSON.stringify(result.saved)).toBe(true)
    if (!result.result.ok) return
    const actual = result.result.value
    const close = (a: number[], b: number[]) => {
      expect(a.length).toBe(b.length)
      a.forEach((v, i) =>
        expect(Math.abs(v - b[i])).toBeLessThanOrEqual(1e-8 * Math.max(1, Math.abs(b[i]))),
      )
    }
    close(actual.params, item.params)
    close(actual.covariance.flat(), item.covariance.flat())
    close(actual.fitted, item.fitted)
    expect(actual.longRun).toEqual({ kind: 'unavailable', reason: 'noPredictors' })
    expect(actual.forecast.kind).toBe('recorded')
    if (actual.forecast.kind === 'recorded') {
      close(actual.forecast.mean, item.forecast)
      close(actual.forecast.variance, item.variance)
      close(actual.forecast.interval.flat(), item.interval.flat())
    }
  })
}
