import { chapter, identifyEffect } from './examples/support'
import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('T-learner WASM intervals match the Docker oracle for all three methods', async ({ page }, info) => {
  test.setTimeout(120000)
  const fixture = JSON.parse(readFileSync('docs/2026-09-15-gcm-401k/tlearner-reference.json', 'utf8'))
  const values = [...fixture.treatment, ...fixture.y, ...[0, 1].flatMap(c => fixture.x.map((row: number[]) => row[c]))]
  await page.goto('/app')
  for (const method of ['percentile', 'pivot', 'normal']) {
    const result = await page.evaluate(async ({ values, method }) => {
      const { runTLearner } = await import(new URL('/src/analysis/client.ts', location.href).href)
      return runTLearner(new Float64Array(values), 100, 4, { treatment: 0, outcome: 1, adjustment: [2, 3], seed: 7, uncertainty: { kind: 'bootstrap', samples: 20, seed: 47, level: 0.95, method } })
    }, { values, method })
    expect(result.ok, JSON.stringify(result)).toBe(true)
    if (!result.ok) continue
    const actual = result.value, expected = fixture.results[method]
    for (let row = 0; row < 100; row++) {
      expect(Math.abs(actual.effects[row] - fixture.effects[row])).toBeLessThanOrEqual(1e-10)
      expect(Math.abs(actual.uncertainty.standardErrors[row] - expected.standard_errors[row])).toBeLessThanOrEqual(1e-10)
      for (let k = 0; k < 2; k++) expect(Math.abs(actual.uncertainty.intervals[row][k] - expected.intervals[row][k])).toBeLessThanOrEqual(1e-10)
    }
    for (let k = 0; k < 2; k++) expect(Math.abs(actual.uncertainty.average.interval[k] - expected.average.interval[k])).toBeLessThanOrEqual(1e-10)
    expect(Math.abs(actual.uncertainty.average.standardErrorBound - expected.average.standard_error_bound)).toBeLessThanOrEqual(1e-10)
    await info.attach(method + '.json', { body: JSON.stringify(actual), contentType: 'application/json' })
  }
})

test('T-learner bootstrap runs through Estimation and exports row intervals', async ({ page }, info) => {
  test.setTimeout(120000)
  const fixture = JSON.parse(readFileSync('docs/2026-09-15-gcm-401k/tlearner-reference.json', 'utf8'))
  const csv = { name: 'tlearner.csv', mimeType: 'text/csv', buffer: Buffer.from(['T,Y,x0,x1', ...fixture.x.map((row: number[], i: number) => [fixture.treatment[i], fixture.y[i], ...row].join(','))].join('\n')) }
  await page.routeWebSocket(/.*/, socket => socket.close())
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('T-learner interval verification')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles(csv)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of ['T', 'Y', 'x0', 'x1']) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('T-learner verification')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Inspector', exact: true }).click()
  const choose = async (label: string, name: string) => {
    await page.getByRole('combobox', { name: label, exact: true }).click()
    await page.getByRole('option', { name, exact: true }).click()
  }
  for (const [cause, effect] of [['T', 'Y'], ['x0', 'T'], ['x1', 'T'], ['x0', 'Y'], ['x1', 'Y']]) {
    await choose('Proposed cause', cause); await choose('Proposed effect', effect)
    await page.getByRole('textbox', { name: /Rationale/ }).first().fill('Adjustment graph for testing both covariates in the fitted forest models.')
    await page.getByRole('button', { name: 'Add the arrow' }).click()
  }
  if (info.project.name === 'mobile-chromium') {
    await page.keyboard.press('Escape')
    await expect(page.getByRole('dialog', { name: 'Inspector', exact: true })).toBeHidden()
    await expect(page.getByRole('button', { name: 'Expand chapter list', exact: true })).toBeVisible()
  }
  await chapter(page, /Study design/)
  await choose('Causal graph', 'T-learner verification')
  await choose('Treatment', 'T'); await choose('Outcome', 'Y')
  await page.getByRole('radio', { name: /Each row, given its covariates/ }).check()
  for (const name of ['x0', 'x1']) await page.getByRole('group', { name: 'Effect modifiers' }).getByRole('checkbox', { name: new RegExp('^' + name) }).check()
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('This verification study adjusts for x0 and x1.')
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Bootstrap intervals', exact: true }).check()
  await page.getByLabel('Learner seed', { exact: true }).fill('7')
  await page.getByLabel('Bootstrap samples', { exact: true }).fill('20')
  await chapter(page, /DAG workspace/)
  await chapter(page, /Estimation/)
  await expect(page.getByRole('radio', { name: 'Bootstrap intervals', exact: true })).toBeChecked()
  await expect(page.getByLabel('Learner seed', { exact: true })).toHaveValue('7')
  await expect(page.getByLabel('Bootstrap samples', { exact: true })).toHaveValue('20')
  await page.getByRole('button', { name: /^Run T-learner/ }).click()
  await expect(page.getByText('Current estimate', { exact: true })).toBeVisible({ timeout: 90000 })
  const saved = async () => page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const project = await store.loadProject((await store.listProjects())[0].id)
    return project.ok ? project.value.estimationRuns.at(-1) : null
  })
  await expect.poll(async () => Boolean(await saved())).toBe(true)
  const run = await saved()
  expect(run.evidence.uncertainty.kind).toBe('bootstrap')
  for (let i = 0; i < 100; i++) for (let k = 0; k < 2; k++)
    expect(Math.abs(run.evidence.uncertainty.intervals[i][k] - fixture.results.percentile.intervals[i][k])).toBeLessThanOrEqual(1e-10)
  const table = page.getByRole('table').filter({ hasText: 'Standard error' })
  await table.scrollIntoViewIfNeeded()
  const downloading = page.waitForEvent('download')
  await page.getByRole('region', { name: 'Row effects and uncertainty' }).getByRole('button', { name: 'Export CSV' }).click()
  const download = await downloading
  expect(download.suggestedFilename()).toBe('t-learner-row-effects.csv')
  const exportPath = info.outputPath('t-learner-row-effects.csv')
  await download.saveAs(exportPath)
  const exported = readFileSync(exportPath, 'utf8').trim().split(/\r?\n/)
  expect(exported).toHaveLength(101)
  expect(exported[0]).toContain('Standard error')
  await page.screenshot({ path: info.outputPath('tlearner-desktop.png'), fullPage: true })
  await page.setViewportSize({ width: 390, height: 844 })
  await table.scrollIntoViewIfNeeded()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('tlearner-mobile.png'), fullPage: true })
  await info.attach('ui-run.json', { body: JSON.stringify(run), contentType: 'application/json' })
})
