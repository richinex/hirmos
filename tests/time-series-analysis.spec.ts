import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { readFileSync } from 'node:fs'
import { choose, prepare } from './examples/support'

function sampleCsv(): string {
  const counts: number[] = JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/fixtures/ingarch_detection.json', import.meta.url), 'utf8')).observations
  let seed = 12345
  const noise = () => { seed = (1664525 * seed + 1013904223) >>> 0; return seed / 4294967296 - 0.5 }
  let x = 10
  let y = 20
  const rows = ['month,x,y,count']
  for (let month = 1; month <= 180; month++) {
    x += noise()
    y = 0.6 * y + 0.8 * x + noise()
    noise() // Preserve the independently verified x/y sequence.
    rows.push([month, x, y, counts[(month - 1) % counts.length]].join(','))
  }
  return rows.join('\n')
}

test('standalone time-series fits, shared results, persistence and deletion', async ({ page }, info) => {
  test.setTimeout(240_000)
  page.setDefaultTimeout(20_000)
  const errors: string[] = []
  page.on('pageerror', (e) => errors.push(e.message))
  const phone = info.project.name === 'mobile-chromium'
  const source = { name: 'series.csv', mimeType: 'text/csv', buffer: Buffer.from(sampleCsv()) }
  const name = `Standalone series ${info.project.name}`
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles(source)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 60_000 })
  await prepare(page, { structure: 'time series', time: 'month', frequency: 'Monthly', columns: ['x', 'y', 'count'] })
  await expect(page.getByRole('radio', { name: /^Count model/ })).toHaveCount(0)
  const chapter = async (name: RegExp) => {
    const toggle = page.getByRole('button', { name: 'Expand chapter list' })
    if (phone && await toggle.isVisible()) await toggle.click()
    await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name }).click()
  }
  await chapter(/Time-series analysis/)
  await expect(page.getByRole('heading', { name: 'Time-series analysis', exact: true })).toBeVisible()
  await page.getByRole('radio', { name: 'ARDL', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Fit ARDL', exact: true })).toBeDisabled()
  await choose(page, 'Outcome series', 'y')
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('button', { name: 'Fit ARDL', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Time-series result' }).filter({ visible: true })).toHaveCount(1, { timeout: 60_000 })
  await expect(page.getByRole('region', { name: 'Time-series result' }).filter({ visible: true })).toContainText('ARDL · y')
  const ardlCharts = page.getByRole('region', { name: 'Time-series result' }).locator('div.space-y-3').filter({ has: page.getByRole('heading', { name: 'Observed outcome and estimated long-run level', exact: true }) })
  await expect(ardlCharts.locator('[_echarts_instance_]')).toHaveCount(2)
  await ardlCharts.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('ardl-long-run.png') })
  await page.evaluate(async () => {
    const charts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    const element = [...document.querySelectorAll('[aria-label="Time-series result"] [_echarts_instance_]')].find(el => el.checkVisibility())
    charts.getInstanceByDom(element).dispatchAction({ type: 'dataZoom', startValue: 30, endValue: 90 })
  })
  const ranges = () => page.evaluate(async () => {
    const charts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    return [...document.querySelectorAll('[aria-label="Time-series result"] [_echarts_instance_]')].filter(el => el.checkVisibility()).slice(0, 2).map(el => {
      const zoom = charts.getInstanceByDom(el).getOption().dataZoom[0]
      return [zoom.startValue, zoom.endValue]
    })
  })
  await expect.poll(ranges).toEqual([[30, 90], [30, 90]])
  await page.getByRole('button', { name: 'Change theme', exact: true }).click()
  await expect.poll(ranges).toEqual([[30, 90], [30, 90]])
  await ardlCharts.getByRole('button', { name: /floating window/ }).first().click()
  await expect(page.getByRole('button', { name: 'Close the floating window', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Close the floating window', exact: true }).click()
  await page.getByTestId('time-series-equation').filter({ visible: true }).getByText('Model equation', { exact: true }).click()
  await page.screenshot({ path: info.outputPath('ardl.png') })
  await page.getByRole('radio', { name: 'VECM', exact: true }).click()
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: 'y', exact: true }).check()
  await page.getByRole('button', { name: 'Fit VECM', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Time-series result' }).filter({ visible: true })).toHaveCount(1, { timeout: 60_000 })
  await expect(page.getByRole('region', { name: 'Time-series result' }).filter({ visible: true })).toContainText('VECM · x, y')
  await expect(page.getByTestId('long-run-charts').filter({ visible: true }).locator('[_echarts_instance_]')).toHaveCount(1)
  await page.getByTestId('long-run-charts').filter({ visible: true }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('vecm.png') })
  await page.getByRole('radio', { name: 'Count models', exact: true }).click()
  await choose(page, 'Count series', 'count')
  await page.getByRole('spinbutton', { name: 'Candidate start row' }).fill('98')
  await page.getByRole('spinbutton', { name: 'Candidate end row' }).fill('102')
  await page.getByRole('button', { name: 'Fit and scan', exact: true }).click()
  await expect(page.getByText('Strongest candidate', { exact: true }).filter({ visible: true })).toBeVisible({ timeout: 60_000 })
  if (!phone) {
    await expect(page.getByRole('complementary')).toContainText('Prepared data')
    const first = page.getByTestId('count-fit-plot').filter({ visible: true })
    const second = page.getByTestId('count-score-plot').filter({ visible: true })
    await first.scrollIntoViewIfNeeded()
    const before = await first.boundingBox()
    const other = await second.boundingBox()
    expect(other!.y).toBeGreaterThanOrEqual(before!.y + before!.height)
    expect(Math.abs(before!.width - other!.width)).toBeLessThan(2)
    await expect(page.getByRole('separator', { name: /Resize Count-model runs/ })).toBeVisible()
  }
  await page.getByTestId('count-fit-plot').filter({ visible: true }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('count-model.png') })
  await expect(page.getByRole('alert')).toHaveCount(0)
  await chapter(/Results/)
  await expect(page.getByRole('region', { name: 'Time-series result' })).toHaveCount(2)
  await expect(page.getByText('Strongest candidate', { exact: true }).filter({ visible: true })).toBeVisible()
  expect(await page.locator('body').evaluate((el) => el.scrollWidth <= window.innerWidth + 1)).toBe(true)
  await page.screenshot({ path: info.outputPath('results.png') })
  if (phone) await page.getByRole('button', { name: 'Expand chapter list' }).click()
  const download = page.waitForEvent('download', { timeout: 30_000 })
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const exported = await download
  const file = await exported.path()
  if (file === null) throw new Error('Export did not produce a file')
  const bundle = JSON.parse(await readFile(file, 'utf8'))
  expect(bundle.project.timeSeriesRuns).toHaveLength(2)
  expect(bundle.project.countSeriesModels).toHaveLength(1)
  expect(bundle.project.estimationRuns).toHaveLength(0)
  expect(bundle.project.dagDocuments).toHaveLength(0)
  // statsmodels 0.14.6 on the same seeded rows, independently generated in Python.
  const ardl = bundle.project.timeSeriesRuns.find((run: { kind: string }) => run.kind === 'ardl-model').evidence
  const vecm = bundle.project.timeSeriesRuns.find((run: { kind: string }) => run.kind === 'vecm').evidence
  // Full multivariate coefficient/covariance parity is covered in ardl-published-examples.
  expect(ardl.predictorLags).toHaveLength(1)
  expect(ardl.longRun.kind).toBe('recorded')
  expect(Number.isFinite(ardl.longRun.boundsStatistic)).toBe(true)
  expect(vecm.beta[1][0]).toBeCloseTo(-0.49377804, 7)
  expect(ardl.longRun.departures).toHaveLength(180)
  expect(vecm.longRun.departures[0]).toHaveLength(178)
  expect(bundle.project.timeSeriesRuns[0].plotTime.values).toEqual(Array.from({ length: 180 }, (_, i) => i + 1))
  const parsed = await page.evaluate(async (value) => {
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    return {
      valid: persistence.parseSnapshotValue(value).ok,
      corrupt: persistence.parseSnapshotValue({ ...value, timeSeriesRuns: value.timeSeriesRuns.map((run: Record<string, unknown>) => ({ ...run, preparedDataset: 'wrong' })) }).ok,
    }
  }, bundle.project)
  expect(parsed).toEqual({ valid: true, corrupt: false })
  await page.waitForTimeout(1000)
  await page.reload()
  await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(source)
  await chapter(/Results/)
  await expect(page.getByRole('region', { name: 'Time-series result' })).toHaveCount(2)
  await chapter(/Time-series analysis/)
  if (phone) await page.getByRole('button', { name: /^Count-model runs/ }).click()
  await page.getByRole('button', { name: 'Delete count-model run' }).click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Cancel' }).click()
  if (!phone) await expect(page.getByText('Strongest candidate', { exact: true }).filter({ visible: true })).toBeVisible()
  await page.getByRole('button', { name: 'Delete count-model run' }).click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Delete run', exact: true }).click()
  if (phone) await page.keyboard.press('Escape')
  for (const model of ['ARDL', 'VECM']) {
    await page.getByRole('radio', { name: model, exact: true }).click()
    if (phone) await page.getByRole('button', { name: /^Time-series runs/ }).click()
    await page.getByRole('button', { name: new RegExp(`^Delete ${model}`) }).click()
    await page.getByRole('alertdialog').getByRole('button', { name: 'Delete run', exact: true }).click()
    await expect(page.getByRole('region', { name: 'Time-series result' })).toHaveCount(0)
    if (phone) await page.keyboard.press('Escape')
  }
  expect(errors).toEqual([])
})

test('time-series boundary rejects incompatible records and preserves chapter separation', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Domain checks run once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const ts = await import(new URL('/src/domain/timeSeries.ts', location.href).href)
    const nav = await import(new URL('/src/domain/navigation.ts', location.href).href)
    const estimation = await import(new URL('/src/domain/estimation.ts', location.href).href)
    const workflow = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const run = {
      kind: 'vecm', id: 'r', preparedDataset: 'p', createdAt: new Date().toISOString(), variables: [{ id: 'x', name: 'x' }, { id: 'y', name: 'y' }],
      specification: { maxLags: 2, deterministic: 'ci', significance: 95 },
      evidence: { kind: 'vecm', observations: 100, deterministic: 'ci', kArDiff: 1, rank: 1, significance: 1, longRunEffect: 2, alpha: [[0.1], [0.2]], beta: [[1], [-2]], gamma: [[0, 0], [0, 0]], pvaluesAlpha: [[0.2], [0.3]], chow: null },
    }
    const parsed = ts.parseTimeSeriesRun(run)
    if (!parsed.ok) throw Error(parsed.error)
    const state = { kind: 'profiled', prepared: { kind: 'prepared-time-series', id: 'p', columns: ['x', 'y'] }, timeSeriesRuns: [], countSeriesModels: [], survivalRuns: [] }
    const created = workflow.stepWorkflow(state, { type: 'time-series-run-created', run: parsed.value })
    return {
      valid: parsed.ok,
      duplicate: ts.parseTimeSeriesRun({ ...run, variables: [run.variables[0], run.variables[0]] }).ok,
      dimensions: ts.parseTimeSeriesRun({ ...run, evidence: { ...run.evidence, beta: [[1]] } }).ok,
      nonfinite: ts.parseTimeSeriesRun({ ...run, evidence: { ...run.evidence, beta: [[Infinity], [-2]] } }).ok,
      wrongKind: ts.parseTimeSeriesRun({ ...run, kind: 'ardl' }).ok,
      created: created.timeSeriesRuns.length,
      wrongPrepared: workflow.stepWorkflow(state, { type: 'time-series-run-created', run: { ...parsed.value, preparedDataset: 'wrong' } }).timeSeriesRuns.length,
      crossSection: workflow.stepWorkflow({ ...state, prepared: { ...state.prepared, kind: 'prepared-cross-section' } }, { type: 'time-series-run-created', run: parsed.value }).timeSeriesRuns.length,
      deleted: workflow.stepWorkflow(created, { type: 'time-series-run-deleted', run: 'r' }).timeSeriesRuns.length,
      cleared: workflow.stepWorkflow(created, { type: 'prepared-dataset-created', artifact: { ...state.prepared, id: 'new' } }).timeSeriesRuns.length,
      nav: nav.CHAPTER_IDS.slice(0, 4),
      causal: estimation.ESTIMATOR_GROUPS.flatMap((group: { estimators: string[] }) => group.estimators),
      encoded: persistence.serialiseSnapshot({ timeSeriesRuns: [parsed.value] }),
    }
  })
  expect(result).toMatchObject({ valid: true, duplicate: false, dimensions: false, nonfinite: false, wrongKind: false, created: 1, wrongPrepared: 0, crossSection: 0, deleted: 0, cleared: 0, nav: ['projects', 'data', 'time-series', 'discovery'] })
  expect(result.causal).not.toContain('ardl-pss')
  expect(result.causal).not.toContain('vecm')
  expect(result.causal).toContain('negative-binomial-ingarch')
  expect(JSON.parse(result.encoded).timeSeriesRuns).toHaveLength(1)
})
