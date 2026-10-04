import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { choose, prepare } from './examples/support'
import type { ArdlModelEvidence } from '../src/domain/ardlModel'
import { formatStatistic } from '../src/lib/format/number'

const fixtureRoot = new URL('../../octopus/rust-causal-transpile/oracle/fixtures/', import.meta.url)
const regressions = JSON.parse(readFileSync(new URL('ardl_multivariate.json', fixtureRoot), 'utf8'))
const multipliers = JSON.parse(readFileSync(new URL('ardl_multipliers.json', fixtureRoot), 'utf8'))
function close(actual: number, expected: number, label: string) {
  expect(Number.isFinite(actual), label).toBe(true)
  expect(Math.abs(actual - expected), label).toBeLessThanOrEqual(1e-8 * Math.max(1, Math.abs(expected)))
}

test('UK appendix Table 5 matches all nine Hirmos worker fits', async ({ page }) => {
  const reference = JSON.parse(readFileSync(new URL('../docs/2026-09-13-ardl-uk-earnings/data/reference-results.json', import.meta.url), 'utf8'))
  const input = regressions.cases.find((entry: { name: string }) => entry.name === 'uk-earnings')
  await page.goto('/app')
  for (const row of reference.appendix_table_5) {
    const result = await page.evaluate(async ({ input, row }) => {
      const client = await import(new URL('/src/analysis/client.ts', location.href).href)
      const domain = await import(new URL('/src/domain/ardlModel.ts', location.href).href)
      const columns = [input.y, ...input.x, ...input.fixed]
      const model = domain.ardlModelRequestSchema.parse({
        outcome: 0, predictors: [1, 2, 3, 4], fixed: [5, 6],
        terms: row.case === 3 ? 'constant' : row.case === 4 ? 'restricted-trend' : 'trend',
        orders: { kind: 'fixed', outcomeLag: row.lag, predictorLags: [1, row.lag, row.lag, row.lag] },
        holdBack: 8, multiplierHorizon: 12, future: { kind: 'none' },
      })
      return client.runArdlModel(new Float64Array(columns.flat()), input.y.length, columns.length, model)
    }, { input, row })
    expect(result.ok).toBe(true)
    if (!result.ok) throw new Error(JSON.stringify(result.error))
    expect(result.value.fittedRows).toBe(104)
    expect(result.value.longRun.kind).toBe('recorded')
    if (result.value.longRun.kind !== 'recorded') throw new Error('Expected Table 5 bounds test')
    close(result.value.longRun.boundsStatistic, row.statistic, `Table 5 lag ${row.lag}, case ${row.case}`)
  }
})

for (const study of [
  { folder: '2026-09-13-ardl-danish-money-demand', csv: 'danish-money-demand.csv', search: true },
  { folder: '2026-09-13-ardl-uk-earnings', csv: 'uk-earnings.csv', search: false },
]) {
  test(`${study.folder} fits the published specification in the browser`, async ({ page }, info) => {
    test.setTimeout(180_000)
    const root = new URL(`../docs/${study.folder}/data/`, import.meta.url)
    const reference = JSON.parse(readFileSync(new URL('reference-results.json', root), 'utf8'))
    const spec = reference.specification
    const name = study.search ? 'denmark' : 'uk-earnings'
    const oracle = regressions.cases.find((entry: { name: string }) => entry.name === name)
    const multiplierOracle = multipliers.cases.find((entry: { name: string }) => entry.name === name)
    const errors: string[] = []
    page.on('pageerror', error => errors.push(error.message))
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill(study.folder)
    await page.getByRole('button', { name: 'Create project' }).click()
    await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL(study.csv, root)))
    await page.getByRole('button', { name: /Inspect data/ }).click()
    await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
    await prepare(page, { structure: 'time series', time: 'date', frequency: 'Quarterly', columns: [spec.outcome, ...spec.predictors, ...(spec.fixed ?? [])] })
    if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
    await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Time-series analysis/ }).click()
    await page.getByRole('radio', { name: 'ARDL', exact: true }).click()
    await expect(page.getByText('This implementation fits one predictor.', { exact: false })).toHaveCount(0)
    await choose(page, 'Outcome series', spec.outcome)
    await page.getByRole('radio', { name: study.search ? 'Select by AIC' : 'Specify lags', exact: true }).click()
    await page.getByRole('spinbutton', { name: study.search ? 'Maximum outcome lag' : 'Outcome lag', exact: true }).fill(String(spec.orders[0]))
    for (const [i, name] of (spec.predictors as string[]).entries()) {
      await page.getByRole('checkbox', { name, exact: true }).check()
      await page.getByRole('spinbutton', { name: `Lag for ${name}`, exact: true }).fill(String(study.search ? 3 : spec.orders[i + 1]))
    }
    for (const name of spec.fixed ?? []) {
      await page.getByRole('checkbox', { name, exact: true }).check()
      await choose(page, `Role for ${name}`, 'Fixed regressor')
    }
    if (spec.hold_back !== undefined) await page.getByRole('spinbutton', { name: 'Initial observations to exclude' }).fill(String(spec.hold_back))
    await page.getByRole('checkbox', { name: 'Forecast with supplied future values' }).check()
    for (const [i, name] of (spec.predictors as string[]).entries()) {
      await page.getByRole('textbox', { name: `Future ${name}`, exact: true }).fill(oracle.forecast.predictors[i].join(', '))
    }
    for (const [i, name] of ((spec.fixed ?? []) as string[]).entries()) {
      await page.getByRole('textbox', { name: `Future ${name}`, exact: true }).fill(oracle.forecast.fixed[i].join(', '))
    }
    await page.getByRole('button', { name: 'Fit ARDL', exact: true }).click()
    const result = page.getByRole('region', { name: 'Time-series result' })
    await expect(result).toBeVisible({ timeout: 60_000 })
    const evidence = async (): Promise<ArdlModelEvidence | null> => page.evaluate(async name => {
      const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
      const header = (await store.listProjects()).find((entry: { name: string }) => entry.name === name)
      if (!header) return null
      const loaded = await store.loadProject(header.id)
      return loaded.ok ? loaded.value.timeSeriesRuns.at(-1)?.evidence ?? null : null
    }, study.folder)
    await expect.poll(evidence).not.toBeNull()
    const actual = (await evidence())!
    expect(actual.outcomeLag).toBe(spec.orders[0])
    expect(actual.predictorLags).toEqual(spec.orders.slice(1))
    expect(actual.fittedRows).toBe(reference.ardl.nobs)
    const coefficients = Object.values(reference.ardl.coefficients) as number[]
    expect(actual.params).toHaveLength(coefficients.length)
    coefficients.forEach((value, i) => expect(actual.params[i]).toBeCloseTo(value, 7))
    const covariance = reference.ardl.covariance as number[][]
    covariance.forEach((row, i) => row.forEach((value, j) => expect(actual.covariance[i]![j]).toBeCloseTo(value, 7)))
    expect(actual.longRun.kind).toBe('recorded')
    if (actual.longRun.kind !== 'recorded') throw new Error('Expected a long-run result')
    const levels = actual.longRun
    expect(levels.normalized).toHaveLength(oracle.uecm.ci_params.length)
    expect(levels.departures).toHaveLength(oracle.uecm.ci_departures.length)
    oracle.uecm.ci_params.forEach((value: number, i: number) => close(levels.normalized[i]!, value, `normalized ${i}`))
    oracle.uecm.ci_interval.forEach((row: number[], i: number) => row.forEach((value, j) => close(levels.intervals[i]![j]!, value, `long-run interval ${i},${j}`)))
    oracle.uecm.ci_departures.forEach((value: number, i: number) => close(levels.departures[i]!, value, `departure ${i}`))
    close(levels.boundsStatistic, multiplierOracle.bounds, 'R bounds statistic')
    if (reference.bounds_case_3) {
      close(levels.pLower, reference.bounds_case_3.p_values.lower, 'bounds lower p')
      close(levels.pUpper, reference.bounds_case_3.p_values.upper, 'bounds upper p')
      Object.values(reference.bounds_case_3.critical_values.lower).forEach((value, i) => close(levels.boundsCritical[i]![0], Number(value), `critical lower ${i}`))
      Object.values(reference.bounds_case_3.critical_values.upper).forEach((value, i) => close(levels.boundsCritical[i]![1], Number(value), `critical upper ${i}`))
    }
    const longRunTable = result.getByRole('table', { name: 'Long-run relationship', exact: true })
    await expect(longRunTable).toBeVisible()
    for (const [term, value] of Object.entries(reference.long_run.coefficients)) {
      const label = term === 'const' ? 'Constant' : term
      const row = longRunTable.getByRole('row').filter({ has: page.getByRole('cell', { name: label, exact: true }) })
      await expect(row.getByRole('cell').nth(1)).toHaveText(formatStatistic('raw', Number(value)).text)
    }
    expect(actual.multipliers.kind).toBe('recorded')
    if (actual.multipliers.kind !== 'recorded') throw new Error('Expected multipliers')
    for (const curve of actual.multipliers.curves) {
      const term = curve.term.kind === 'constant' ? '(Intercept)' : curve.term.kind === 'trend' ? 'trend(y, scale = FALSE)' : `x${curve.term.column + 1}`
      const lr = multiplierOracle.long_run.find((row: { Term: string }) => row.Term === term)
      const sr = multiplierOracle.short_run.find((row: { Term: string }) => row.Term === term)
      close(curve.longRun, lr.Estimate, `${term} long run`)
      close(curve.longRunSe, lr['Std. Error'], `${term} long-run SE`)
      close(curve.shortRun, sr.Estimate, `${term} short run`)
      multiplierOracle.delay[term].forEach((point: Record<string, number>, i: number) => {
        close(curve.delay[i]!, point.Delay!, `${term} delay ${i}`)
        close(curve.standardError[i]!, point['Std. Error Delay']!, `${term} SE ${i}`)
        close(curve.cumulative[i]!, point.Interim!, `${term} cumulative ${i}`)
        const margin = 1.959963984540054 * point['Std. Error Delay']!
        close(curve.interval[i]![0], point.Delay! - margin, `${term} lower ${i}`)
        close(curve.interval[i]![1], point.Delay! + margin, `${term} upper ${i}`)
      })
    }
    expect(actual.forecast.kind).toBe('recorded')
    if (actual.forecast.kind !== 'recorded') throw new Error('Expected a forecast')
    const forecast = actual.forecast
    expect(forecast.confidence).toBe(0.95)
    oracle.forecast.mean.forEach((value: number, i: number) => {
      close(forecast.mean[i]!, value, `forecast ${i}`)
      close(forecast.variance[i]!, oracle.forecast.variance[i], `forecast variance ${i}`)
      const margin = 1.959963984540054 * Math.sqrt(oracle.forecast.variance[i])
      close(forecast.interval[i]![0], value - margin, `forecast lower ${i}`)
      close(forecast.interval[i]![1], value + margin, `forecast upper ${i}`)
    })
    await expect(result.getByRole('region', { name: 'ARDL forecast', exact: true })).toBeVisible()
    await result.getByText('Model equation', { exact: true }).click()
    await expect(result.locator('.katex-error')).toHaveCount(0)
    expect(await result.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
    await longRunTable.scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath('long-run.png') })
    await result.getByRole('region', { name: 'ARDL forecast', exact: true }).scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath('forecast.png') })
    expect(errors).toEqual([])
  })
}
