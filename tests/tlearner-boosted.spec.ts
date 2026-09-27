import { chapter, identifyEffect } from './examples/support'
import { expect, test, type Page } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { tLearnerEffect, tLearnerRowEffects } from '../src/domain/estimation'

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
      splits: 5, minSamplesLeaf: leaf, minSamplesSplit: 2, seed, selection: { kind: 'search' },
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

/** How the study states its target; the T-learner reports every row's effect or their mean. */
type Target = { readonly kind: 'per-row' } | { readonly kind: 'average' }

/** Builds the verification project for the target, runs the boosted T-learner, and returns the saved run. */
const runBoostedTLearner = async (page: Page, target: Target) => {
  const oracle = fixture('sklearn_tlearner_global_selection_leaf1.json')
  const project = `Boosted T-learner verification, ${target.kind}`
  const names = Array.from({ length: COLUMNS }, (_, column) => `c${column}`)
  const csv = ['T,death,' + names.join(','), ...treatment.map((t, row) => [t, death[row], ...names.map((_, column) => confounder(row, column))].join(','))].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(project)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'boosted.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of ['T', 'death', ...names]) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(project)
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(`dag { T -> death ${names.map((name) => `${name} -> T ${name} -> death`).join(' ')} }`)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await expect(page.getByText('11 arrows', { exact: true })).toBeVisible()
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', project)
  await choose(page, 'Treatment', 'T'); await choose(page, 'Outcome', 'death')
  if (target.kind === 'per-row') {
    await page.getByRole('radio', { name: /Each row, given its covariates/ }).check()
    for (const name of names) await page.getByRole('group', { name: 'Effect modifiers' }).getByRole('checkbox', { name: new RegExp('^' + name) }).check()
  }
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('This verification study adjusts for c0 to c4.')
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  if (target.kind === 'average') {
    await page.getByRole('radio', { name: /^Adjustment/ }).click()
    await page.getByRole('radio', { name: /^T-learner/ }).click()
  }
  await page.getByRole('radio', { name: 'Boosted, cross-fitted', exact: true }).click()
  await page.getByText('Search grid', { exact: true }).click()
  await page.getByRole('textbox', { name: 'Learning rates', exact: true }).fill('0.05, 0.15')
  await page.getByRole('textbox', { name: 'Tree depths', exact: true }).fill('1, 2')
  await page.getByRole('textbox', { name: 'Tree counts', exact: true }).fill('5, 10')
  await page.getByRole('spinbutton', { name: 'Tree seed', exact: true }).fill(String(oracle.seed))
  await expect(page.getByText('8 candidates', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: /^Run T-learner/ }).click()
  await expect(page.getByText('Current estimate', { exact: true })).toBeVisible({ timeout: 120_000 })
  const saved = async () => page.evaluate(async (name) => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await store.listProjects()).find((item: { name: string }) => item.name === name)
    const loaded = header === undefined ? null : await store.loadProject(header.id)
    return loaded?.ok ? loaded.value.estimationRuns.at(-1) : null
  }, project)
  await expect.poll(async () => Boolean(await saved())).toBe(true)
  const run = await saved()
  expect(run.configuration.model.kind).toBe('boosted-cross-fitted')
  expect(run.evidence.kind).toBe('crossFittedTLearner')
  expectEffects(run.evidence.effects, oracle, run.evidence.average)
  await expect(page.getByText('Treated model', { exact: true }).first()).toBeVisible()
  await expect(page.getByText('Control model', { exact: true }).first()).toBeVisible()
  await expect(page.getByText('Row effects', { exact: true }).first()).toBeVisible()
  return run
}

test('a per-row study records every row\'s effect with their mean', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run; the controls are shared.')
  test.setTimeout(180_000)
  const run = await runBoostedTLearner(page, { kind: 'per-row' })
  expect(run.estimate.effect.kind).toBe('perRow')
  expect(run.estimate.effect.overall).toBe(run.evidence.average)
  expect(run.estimate.effect.effects).toEqual(run.evidence.effects)
  await page.screenshot({ path: info.outputPath('boosted-tlearner-per-row.png'), fullPage: true })
})

test('an average-effect study reports the mean of the row effects as its ATE', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run; the controls are shared.')
  test.setTimeout(180_000)
  const run = await runBoostedTLearner(page, { kind: 'average' })
  expect(run.estimate.estimand.kind).toBe('average-treatment-effect')
  expect(run.estimate.effect).toEqual({ kind: 'additive', value: run.evidence.average, unit: '' })
  await page.screenshot({ path: info.outputPath('boosted-tlearner-average.png'), fullPage: true })
})

test('the T-learner reports every row for a per-row target, their mean for ATE, and nothing for other targets', () => {
  const effects: [number, ...number[]] = [0.1, -0.2, 0.4]
  const variable = { column: 'c', name: 'c' } as never
  expect(tLearnerEffect({ kind: 'conditional-average-treatment-effect-per-row', modifiers: [] } as never, 0.1, effects)).toEqual({ kind: 'perRow', overall: 0.1, effects })
  expect(tLearnerEffect({ kind: 'average-treatment-effect' } as never, 0.1, effects)).toEqual({ kind: 'additive', value: 0.1, unit: '' })
  expect(tLearnerEffect({ kind: 'average-treatment-effect-on-treated' } as never, 0.1, effects)).toBeNull()
  expect(tLearnerEffect({ kind: 'conditional-average-treatment-effect', modifier: variable } as never, 0.1, effects)).toBeNull()
  expect(tLearnerEffect({ kind: 'local-cutoff-effect', cutoff: 0, running: variable } as never, 0.1, effects)).toBeNull()
  // Effects that miss a row are no reading of every row.
  expect(tLearnerRowEffects({ observations: 4, effects } as never)).toBeNull()
  expect(tLearnerRowEffects({ observations: 3, effects } as never)).toEqual(effects)
})
