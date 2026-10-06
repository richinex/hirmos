import { expect, test, type Page } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { parseSnapshotValue } from '../src/domain/persistence'
import { chapter, choose, identify } from './examples/support'

// The shipped cohort now asks for ATT. Regression requires its own population-effect study;
// changing the estimator must not silently change that target.
const cohortRegressionStudy = async (page: Page) => {
  await identify(page, {
    graph: 'March cohort adoption', treatment: 'adopted', outcome: 'bugs_per_kloc',
    target: /All prepared rows/, mechanism: 'Policy change',
    sentence: 'Teams A, B and C switched the AI assistant on in month 13; teams F, G and H never did.',
  })
  await chapter(page, /Estimation/)
  await choose(page, 'Identified study', 'Average effect of adopted on bugs_per_kloc, March cohort adoption')
}

/**
 * The adjusted regression with one fixed effect per unit: the control offers it, the run absorbs the
 * adjustment variables that do not vary within a unit and says which, and the figures match
 * fixest 0.14.2 with its default nonnested correction on the same rows.
 * oracle/mixtape-did/fixest-ui.R in the transpile project generates the references.
 */

const REGRESSORS = 'age asq bmi hispanic black other asian schooling cohab married divorced separated age_cl unsafe llength reg asq_cl appearance_cl provider_second asian_cl black_cl hispanic_cl othrace_cl hot massage_cl'.split(' ')
const CONSTANT_WITHIN_PROVIDER = 'age asq bmi hispanic black other asian schooling cohab married divorced separated'.split(' ')
const GRAPH = `dag {\n  unsafe [exposure]\n  lnw [outcome]\n${REGRESSORS.filter((name) => name !== 'unsafe').flatMap((name) => [`  ${name} -> unsafe`, `  ${name} -> lnw`]).join('\n')}\n  unsafe -> lnw\n}`
const REFERENCE = { estimate: 0.05103386003796254, clusteredSe: 0.028283096650613505, units: 257, degreesOfFreedom: 758 }

test('fixed effects by unit absorb the provider-level regressors and cluster the interval by the unit', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run path.')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Fixed effects by provider')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL('./sasp_panel.csv', import.meta.url)))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByText('Choose the observation structure').waitFor({ timeout: 90_000 })
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await page.getByRole('heading', { name: 'Build a DAG or run discovery', exact: true }).waitFor({ timeout: 120_000 })

  const sections = page.getByRole('navigation', { name: 'Workspace sections' })
  await sections.getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Unprotected sex and the wage')
  await page.getByRole('button', { name: 'Create DAG draft', exact: true }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(GRAPH)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await page.getByText('49 arrows', { exact: true }).waitFor()

  await sections.getByRole('button', { name: /Study design/ }).click()
  const choose = async (label: string, option: string) => {
    await page.getByRole('combobox', { name: label }).click()
    await page.getByRole('option', { name: option, exact: true }).click()
  }
  await choose('Causal graph', 'Unprotected sex and the wage')
  await choose('Treatment', 'unsafe')
  await choose('Outcome', 'lnw')
  await page.getByRole('radio', { name: /Observed choice/ }).click()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('Whether a session involved unprotected sex was decided by the provider and the client.')
  await page.getByRole('button', { name: 'Identify the effect', exact: true }).click()
  await page.getByText(/Identified by/).first().waitFor({ timeout: 60_000 })

  await sections.getByRole('button', { name: /Estimation/ }).click()
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()
  await page.getByRole('radiogroup', { name: 'Fixed effects' }).getByRole('radio', { name: 'By unit', exact: true }).click()
  await choose('Unit column', 'id')
  await expect(page.getByText(/Each variable is replaced by its deviation from the mean of its id\./)).toBeVisible()
  const errors = page.getByRole('radiogroup', { name: 'Error treatment' })
  await expect(errors.getByRole('radio', { name: 'Newey–West HAC', exact: true })).toHaveCount(0)
  await errors.getByRole('radio', { name: 'Clustered', exact: true }).click()
  await choose('Cluster column', 'id')
  await expect(page.getByText(/The clustered interval allows errors to correlate within each value of id/)).toBeVisible()
  await expect(page.getByRole('region', { name: 'Method requirements' })).toContainText('additive differences that stay constant within each id')

  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByRole('heading', { name: 'Runs (1)', exact: true })).toBeVisible({ timeout: 120_000 })
  const run = page.getByRole('article').first()
  await expect(run.getByText('Fixed effects by id', { exact: true })).toBeVisible()
  await expect(run.getByText('Clustered by id', { exact: true })).toBeVisible()
  await expect(run.getByText('R squared, within', { exact: true })).toBeVisible()
  await expect(run.getByText('13 regressors', { exact: true })).toBeVisible()
  await expect(run.getByText('257 units', { exact: true })).toBeVisible()
  await expect(run.getByText('Absorbed by the fixed effects', { exact: true })).toBeVisible()
  // The absorbed names follow the adjustment set's own order, so each is checked on its own.
  const absorbedTile = run.getByText(/^(?:[a-z_]+, )+[a-z_]+$/).first()
  for (const name of CONSTANT_WITHIN_PROVIDER) await expect(absorbedTile).toContainText(name)
  await expect(run.getByText(/The fixed effects absorb 12 of the adjustment variables/)).toBeVisible()
  await expect(run.getByText(/After accounting for the 24 variables of the identified adjustment set and a fixed effect for each id/)).toBeVisible()

  const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const saved = JSON.parse(await readFile(await (await download).path(), 'utf8')).project
  const evidence = saved.estimationRuns.at(-1).evidence
  expect(Math.abs(evidence.estimate - REFERENCE.estimate)).toBeLessThan(1e-9)
  expect(evidence.errorModel.kind).toBe('cluster')
  expect(Math.abs(evidence.errorModel.standardError - REFERENCE.clusteredSe)).toBeLessThan(1e-9)
  expect(evidence.errorModel.clusters).toBe(REFERENCE.units)
  expect(evidence.degreesOfFreedom).toBe(REFERENCE.degreesOfFreedom)
  expect(evidence.fixedEffects.kind).toBe('unit')
  const last = saved.estimationRuns.at(-1)
  expect(evidence.fixedEffects.absorbed.map((column: number) => last.columns[column].name).sort()).toEqual([...CONSTANT_WITHIN_PROVIDER].sort())
})

// fixest default clustered inference on tests/fixtures/cohort-march.csv.
const COHORT = { estimate: -0.5549111111111109, clusteredSe: 0.0062739563167684758, units: 6, degreesOfFreedom: 137 }

test('a panel-structured dataset offers its own unit key for fixed effects without a prepared column', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run path.')
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await cohortRegressionStudy(page)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()
  await page.getByRole('radiogroup', { name: 'Fixed effects' }).getByRole('radio', { name: 'By unit', exact: true }).click()
  await expect(page.getByRole('combobox', { name: 'Unit column' })).toContainText('team')
  await page.getByRole('radiogroup', { name: 'Error treatment' }).getByRole('radio', { name: 'Clustered', exact: true }).click()
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByRole('heading', { name: 'Runs (3)', exact: true })).toBeVisible({ timeout: 120_000 })
  const run = page.getByRole('article').first()
  await expect(run.getByText('Fixed effects by team', { exact: true })).toBeVisible()
  await expect(run.getByText('Clustered by team', { exact: true })).toBeVisible()
  await expect(run.getByText('6 units', { exact: true })).toBeVisible()

  const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const saved = JSON.parse(await readFile(await (await download).path(), 'utf8')).project
  const evidence = saved.estimationRuns.at(-1).evidence
  expect(Math.abs(evidence.estimate - COHORT.estimate)).toBeLessThan(1e-9)
  expect(Math.abs(evidence.errorModel.standardError - COHORT.clusteredSe)).toBeLessThan(1e-9)
  expect(evidence.errorModel.clusters).toBe(COHORT.units)
  expect(evidence.fixedEffects.units).toBe(COHORT.units)
  expect(evidence.degreesOfFreedom).toBe(COHORT.degreesOfFreedom)
})

// fixest on the same file with unit and time effects and unit clustering.
const TWO_WAY = { estimate: -0.45448333333333263, clusteredSe: 0.021684734732218484, degreesOfFreedom: 114, periods: 24 }

test('time-only effects keep unit clustering independent and survive saved-run decoding', async ({ page }, info) => {
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  const estimation = async () => {
    const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
    if (info.project.name === 'mobile-chromium' && await expand.isVisible()) await expand.click()
    await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Estimation/ }).click()
  }
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await cohortRegressionStudy(page)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()
  const effects = page.getByRole('radiogroup', { name: 'Fixed effects' })
  await effects.getByRole('radio', { name: 'By time', exact: true }).click()
  await expect(page.getByRole('combobox', { name: 'Unit column', exact: true })).toHaveCount(0)
  await expect(page.getByRole('combobox', { name: 'Time column', exact: true })).toContainText('month')
  await page.getByRole('radiogroup', { name: 'Error treatment' }).getByRole('radio', { name: 'Clustered', exact: true }).click()
  await expect(page.getByRole('combobox', { name: 'Cluster column' })).toContainText('team')
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  const result = page.getByRole('article').first()
  await expect(result.getByText('Time fixed effects by month', { exact: true })).toBeVisible({ timeout: 30_000 })
  await expect(result.getByText('After accounting for a fixed effect for each month', { exact: false })).toBeVisible()
  await effects.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('time-only-controls.png') })
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('time-only-effects.png') })
  const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const exported = JSON.parse(await readFile(await (await download).path(), 'utf8'))
  const saved = exported.project
  const evidence = saved.estimationRuns.at(-1).evidence
  expect(evidence.fixedEffects.kind).toBe('time')
  expect(evidence.fixedEffects.periods).toBe(24)
  expect(Math.abs(evidence.estimate - (-1.5278222222222226))).toBeLessThan(1e-9)
  expect(Math.abs(evidence.errorModel.standardError - 0.17244713009492182)).toBeLessThan(1e-9)
  expect(evidence.degreesOfFreedom).toBe(119)
  expect(parseSnapshotValue(saved).ok).toBe(true)
  await page.reload()
  await page.getByLabel('Exported project file', { exact: true }).setInputFiles({ name: 'time-only.hirmos.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(exported)) })
  await expect(page.getByRole('heading', { name: 'Choose the data file again', exact: true })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL('./fixtures/cohort-march.csv', import.meta.url)))
  await estimation()
  await expect(page.getByRole('article').first().getByText('Time fixed effects by month', { exact: true })).toBeVisible()
  await expect(page.getByRole('article').first().getByText('Clustered by team', { exact: true })).toBeVisible()
  expect(errors).toEqual([])
  saved.estimationRuns.at(-1).configuration.fixedEffects.kind = 'unit'
  expect(parseSnapshotValue(saved).ok).toBe(false)
})

test('unit and time fixed effects on a panel reproduce the conventional difference in differences', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run path.')
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await cohortRegressionStudy(page)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()
  await page.getByRole('radiogroup', { name: 'Fixed effects' }).getByRole('radio', { name: 'By unit and time', exact: true }).click()
  await expect(page.getByRole('combobox', { name: 'Unit column' })).toContainText('team')
  await expect(page.getByRole('combobox', { name: 'Time column' })).toContainText('month')
  await page.getByRole('radiogroup', { name: 'Error treatment' }).getByRole('radio', { name: 'Clustered', exact: true }).click()
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByRole('heading', { name: 'Runs (3)', exact: true })).toBeVisible({ timeout: 120_000 })
  const run = page.getByRole('article').first()
  await expect(run.getByText('Fixed effects by team and month', { exact: true })).toBeVisible()
  await expect(run.getByText('24 periods', { exact: true })).toBeVisible()

  const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const saved = JSON.parse(await readFile(await (await download).path(), 'utf8')).project
  const runs = saved.estimationRuns
  const evidence = runs.at(-1).evidence
  expect(Math.abs(evidence.estimate - TWO_WAY.estimate)).toBeLessThan(1e-9)
  expect(Math.abs(evidence.errorModel.standardError - TWO_WAY.clusteredSe)).toBeLessThan(1e-9)
  expect(evidence.degreesOfFreedom).toBe(TWO_WAY.degreesOfFreedom)
  expect(evidence.fixedEffects.kind).toBe('unitAndTime')
  expect(evidence.fixedEffects.periods).toBe(TWO_WAY.periods)
  // The example's own panel run carries the conventional DiD; the two-way regression must equal it.
  const panelRun = runs.find((candidate: { kind: string }) => candidate.kind === 'panel-intervention-run')
  expect(Math.abs(evidence.estimate - panelRun.evidence.did.estimate)).toBeLessThan(1e-9)
})
