import { chapter, identifyEffect } from './examples/support'
import { expect, test, type Page } from '@playwright/test'
import { readFileSync } from 'node:fs'

/**
 * The cross-fitted boosted T-learner against scikit-learn: each arm's classifier chosen once by grid
 * search on that arm's rows, refitted in each half and predicting the other half.
 */

const ROWS = 400
const COLUMNS = 5
const confounder = (row: number, column: number) => (row * (column + 3) + column * column) % 29
const treatment = Array.from({ length: ROWS }, (_, row) => Number((confounder(row, 0) + confounder(row, 2)) % 7 > 3))
const death = Array.from({ length: ROWS }, (_, row) => Number((confounder(row, 1) * 2 + treatment[row]! * 5) % 11 > 4))
const fixture = (name: string) => JSON.parse(readFileSync(`crates/causal-core/oracle/fixtures/${name}`, 'utf8'))

const expectEffects = (effects: readonly number[], oracle: { order: number[]; effects: number[]; ate: number }, average: number) => {
  oracle.order.forEach((row, at) => expect(Math.abs(effects[row]! - oracle.effects[at]!)).toBeLessThanOrEqual(1e-12))
  expect(Math.abs(average - oracle.ate)).toBeLessThanOrEqual(1e-12)
}

test('the browser build reproduces scikit-learn', async ({ page }) => {
  const oracle = fixture('sklearn_tlearner_global_selection.json')
  const values = [...treatment, ...death, ...Array.from({ length: COLUMNS }, (_, column) => Array.from({ length: ROWS }, (_, row) => confounder(row, column))).flat()]
  await page.goto('/app')
  const result = await page.evaluate(async ({ values, grid, leaf, seed }) => {
    const { runCrossFittedTLearner } = await import(new URL('/src/analysis/client.ts', location.href).href)
    return runCrossFittedTLearner(new Float64Array(values), 400, 7, {
      treatment: 0, outcome: 1, adjustment: [2, 3, 4, 5, 6],
      learningRate: grid.learning_rate, maxDepth: grid.max_depth, nEstimators: grid.n_estimators,
      splits: 5, minSamplesLeaf: leaf, minSamplesSplit: 2, seed,
    })
  }, { values, grid: oracle.grid, leaf: oracle.min_samples_leaf, seed: oracle.seed })
  expect(result.ok, JSON.stringify(result)).toBe(true)
  if (!result.ok) return
  expectEffects(result.value.effects, oracle, result.value.average)
  for (const arm of ['treated', 'control'] as const) {
    const mine = result.value.selected[arm]
    expect({ learning_rate: mine.learningRate, max_depth: mine.maxDepth, n_estimators: mine.nEstimators }).toEqual(oracle.selected[arm])
    expect(Math.abs(mine.validationAuc - oracle.validation_auc[arm])).toBeLessThanOrEqual(1e-12)
  }
})

const choose = async (page: Page, label: string, name: string) => {
  await page.getByRole('combobox', { name: label, exact: true }).click()
  await page.getByRole('option', { name, exact: true }).click()
}

test('the Estimation panel runs it and records both chosen models', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run; the controls are shared.')
  test.setTimeout(180_000)
  const oracle = fixture('sklearn_tlearner_global_selection_leaf1.json')
  const names = Array.from({ length: COLUMNS }, (_, column) => `c${column}`)
  const csv = ['T,death,' + names.join(','), ...treatment.map((t, row) => [t, death[row], ...names.map((_, column) => confounder(row, column))].join(','))].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Boosted T-learner verification')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'boosted.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of ['T', 'death', ...names]) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Boosted T-learner verification')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(`dag { T -> death ${names.map((name) => `${name} -> T ${name} -> death`).join(' ')} }`)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await expect(page.getByText('11 arrows', { exact: true })).toBeVisible()
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', 'Boosted T-learner verification')
  await choose(page, 'Treatment', 'T'); await choose(page, 'Outcome', 'death')
  await page.getByRole('radio', { name: /Each row, given its covariates/ }).check()
  for (const name of names) await page.getByRole('group', { name: 'Effect modifiers' }).getByRole('checkbox', { name: new RegExp('^' + name) }).check()
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('This verification study adjusts for c0 to c4.')
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Boosted, cross-fitted', exact: true }).click()
  await page.getByRole('textbox', { name: 'Learning rates', exact: true }).fill('0.05, 0.15')
  await page.getByRole('textbox', { name: 'Tree depths', exact: true }).fill('1, 2')
  await page.getByRole('textbox', { name: 'Tree counts', exact: true }).fill('5, 10')
  await page.getByRole('spinbutton', { name: 'Tree seed', exact: true }).fill(String(oracle.seed))
  await expect(page.getByText('8 candidates', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: /^Run T-learner/ }).click()
  await expect(page.getByText('Current estimate', { exact: true })).toBeVisible({ timeout: 120_000 })
  const saved = async () => page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await store.listProjects()).find((item: { name: string }) => item.name === 'Boosted T-learner verification')
    const project = header === undefined ? null : await store.loadProject(header.id)
    return project?.ok ? project.value.estimationRuns.at(-1) : null
  })
  await expect.poll(async () => Boolean(await saved())).toBe(true)
  const run = await saved()
  expect(run.configuration.model.kind).toBe('boosted-cross-fitted')
  expect(run.evidence.kind).toBe('crossFittedTLearner')
  expectEffects(run.evidence.effects, oracle, run.evidence.average)
  await expect(page.getByText('Treated model', { exact: true }).first()).toBeVisible()
  await expect(page.getByText('Control model', { exact: true }).first()).toBeVisible()
  await page.screenshot({ path: info.outputPath('boosted-tlearner.png'), fullPage: true })
})
