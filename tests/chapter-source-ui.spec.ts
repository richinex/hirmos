import { expect, test, type Page, type TestInfo } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { addArrow, chapter, choose, createDag, identify, fixture } from './examples/support'
test.beforeEach(async ({ page }) => { page.setDefaultTimeout(15_000) })

// All preparation, fitting and exporting use product controls. Reading an export
// verifies full precision without bypassing the button-driven calculation.
async function upload(page: Page, name: string, file: string) {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(fixture(file))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.getByText('Choose the observation structure', { exact: true })).toBeVisible({ timeout: 60_000 })
}

async function exportProject(page: Page) {
  const open = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await open.isVisible()) await open.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const path = await (await download).path()
  if (path === null) throw new Error('No exported project')
  return JSON.parse(await readFile(path, 'utf8')).project
}

async function capture(page: Page, info: TestInfo, name: string) {
  const close = page.getByRole('button', { name: 'Collapse section list', exact: true })
  if (await close.isVisible()) await close.click()
  await page.getByText('Current estimate', { exact: true }).first().scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath(name) })
  expect(await page.locator('body').evaluate(el => el.scrollWidth <= innerWidth + 1)).toBe(true)
  const plot = page.locator('[data-testid="did-group-means"], [data-testid="sharp-rd-plot"], [data-testid="bayesian-impact-effects"]').first()
  if (await plot.count() > 0) {
    await plot.scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath(name.replace('.png', '-plot.png')) })
  }
}

async function validateSaved(page: Page, snapshot: unknown) {
  const valid = await page.evaluate(async snapshot => {
    const persistence = await import(new URL('/src/domain/persistence.ts',location.href).href)
    const parsed = persistence.parseSnapshotValue(snapshot)
    return parsed.ok ? { ok:true } : parsed
  },snapshot)
  expect(valid.ok,JSON.stringify(valid)).toBe(true)
}

async function reopenResult(page: Page, file: string) {
  await page.reload()
  await page.getByRole('button', { name: /^Open Chapter/ }).click()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(fixture(file))
  await chapter(page, /Estimation/)
  await expect(page.getByText('Current estimate', { exact: true }).first()).toBeVisible()
}

for (const method of ['Regression','Doubly robust'] as const) {
  test(`source chapter ${method} DiD through controls, plot and saved result`, async ({ page },info) => {
    test.setTimeout(240_000)
    await upload(page,`Chapter ${method} DiD`,'chapter-did-covariates.csv')
    await page.getByRole('radio',{ name:/^Panel/ }).click()
    await choose(page,'Unit column','id')
    await choose(page,'Time column','time_points')
    for (const name of ['Y','treatment','A']) await page.getByRole('checkbox',{ name,exact:true }).check()
    await page.getByRole('button',{ name:/Create prepared/ }).click()
    await createDag(page,'Adjusted comparison')
    await addArrow(page,'treatment','Y','The policy changes treatment after baseline.')
    await identify(page,{ target:/Treated rows/, graph:'Adjusted comparison',treatment:'treatment',outcome:'Y',mechanism:'Policy change',sentence:'Compare treated and never-treated units before and after adoption.' })
    await chapter(page,/Estimation/)
    await page.getByRole('radio',{ name:/^Interventions/ }).click()
    await page.getByRole('radio',{ name:/Panel difference-in-differences/ }).click()
    await page.getByRole('radio',{ name:method,exact:true }).click()
    await page.getByRole('group',{ name:'DiD covariates' }).getByRole('checkbox',{ name:'A',exact:true }).check()
    if (method === 'Doubly robust') await page.getByRole('spinbutton',{ name:'DiD folds',exact:true }).fill('5')
    await page.getByRole('button',{ name:/^Run panel difference-in-differences/i }).click()
    await expect(page.getByText('Current estimate',{ exact:true }).first()).toBeVisible({ timeout:60_000 })
    await expect(page.getByTestId('did-group-means').first()).toBeVisible()
    await capture(page,info,`chapter-${method.replaceAll(' ','-')}.png`)
    const saved = await exportProject(page)
    await validateSaved(page,saved)
    const run = saved.estimationRuns[0]
    expect(run.evidence.kind).toBe('panelAdjusted')
    expect(run.estimate.estimand.kind).toBe('average-treatment-effect-on-treated')
    expect(run.evidence.estimate).toBeCloseTo(method === 'Regression' ? 290.53750263095316 : 314.7959088653215,6)
    expect(run.evidence.standardError).toBeCloseTo(method === 'Regression' ? 59.77346090819873 : 44.386926615015476,6)
    await reopenResult(page, 'chapter-did-covariates.csv')
  })
}

test('source sharp RD through cutoff-local study and restored robust evidence',async ({ page },info) => {
  test.setTimeout(240_000)
  await upload(page,'Chapter sharp RD','chapter-rd.csv')
  await page.getByRole('radio',{ name:/Independent observations/ }).click()
  for (const name of ['time','D','Y']) await page.getByRole('checkbox',{ name,exact:true }).check()
  await page.getByRole('button',{ name:/Create prepared/ }).click()
  await createDag(page,'Cutoff assignment')
  await addArrow(page,'time','D','Treatment is assigned at the running-variable cutoff.')
  await addArrow(page,'time','Y','Allow the outcome to vary smoothly with the running variable.')
  await addArrow(page,'D','Y','Treatment may change the outcome at the cutoff.')
  await chapter(page,/Study design/)
  await choose(page,'Causal graph','Cutoff assignment')
  await choose(page,'Treatment','D')
  await choose(page,'Outcome','Y')
  await page.getByRole('radio',{ name:/At an assignment cutoff/ }).click()
  await choose(page,'Running variable','time')
  await page.getByRole('spinbutton',{ name:'Assignment cutoff',exact:true }).fill('0')
  await page.getByRole('radio',{ name:/Policy change/ }).click()
  await page.getByRole('textbox',{ name:'Assignment sentence' }).fill('Treatment is one at and above zero and zero below zero.')
  await page.getByRole('button',{ name:'Identify the effect',exact:true }).click()
  await expect(page.getByText('Sharp RD design recorded',{ exact:true }).first()).toBeVisible({ timeout:60_000 })
  await chapter(page,/Estimation/)
  await page.getByRole('button',{ name:/^Run sharp regression discontinuity/i }).click()
  await expect(page.getByText('Current estimate',{ exact:true }).first()).toBeVisible({ timeout:60_000 })
  await expect(page.getByTestId('sharp-rd-plot').first()).toBeVisible()
  await capture(page,info,'chapter-sharp-rd.png')
  const saved = await exportProject(page)
  await validateSaved(page,saved)
  const run = saved.estimationRuns[0]
  expect(run.estimate.estimand.kind).toBe('local-cutoff-effect')
  expect(run.evidence.robust.value).toBeCloseTo(1040.9315095351085,7)
  await reopenResult(page, 'chapter-rd.csv')
})

test('source chapter conventional DiD runs from upload to exported result', async ({ page }, info) => {
  test.setTimeout(240_000)
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await upload(page, 'Chapter conventional DiD', 'chapter-did.csv')
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await choose(page, 'Unit column', 'id')
  await choose(page, 'Time column', 'time_points')
  for (const column of ['Y', 'treatment']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('heading', { name: 'Build a DAG or run discovery', exact: true })).toBeVisible({ timeout: 60_000 })
  await createDag(page, 'Chapter two-period comparison')
  await addArrow(page, 'treatment', 'Y', 'The chapter assigns treatment to the treated group in the second period.')
  await identify(page, { target:/Treated rows/, graph: 'Chapter two-period comparison', treatment: 'treatment', outcome: 'Y', mechanism: 'Policy change', sentence: 'The treated group receives the intervention after the first period; controls remain untreated.' })
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).click()
  await page.getByRole('radio', { name: /Panel difference-in-differences/ }).click()
  await page.getByRole('radio', { name: 'Conventional', exact: true }).click()
  await page.getByRole('button', { name: /^Run panel difference-in-differences/i }).click()
  await expect(page.getByText('Current estimate', { exact: true }).first()).toBeVisible({ timeout: 60_000 })
  await capture(page, info, 'chapter-did.png')
  const saved = await exportProject(page)
  expect(saved.estimationRuns).toHaveLength(1)
  const run = saved.estimationRuns[0]
  expect(run.configuration.primary).toBe('did')
  expect(run.evidence.kind).toBe('panelDid')
  // Frozen statsmodels oracle, Python chapter data (not the distinct R RNG data).
  expect(run.evidence.did.estimate).toBeCloseTo(290.53750263095304, 8)
  expect(run.estimate.effect.value).toBeCloseTo(290.53750263095304, 8)
  await validateSaved(page, saved)
  await reopenResult(page, 'chapter-did.csv')
  expect(errors).toEqual([])
})

test('source chapter Bayesian impact runs from upload with explicit controls and saved inference', async ({ page }, info) => {
  test.setTimeout(240_000)
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await upload(page, 'Chapter Bayesian impact', 'chapter-impact.csv')
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page, 'Time column', 'time')
  for (const column of ['D', 'Y', 'X_1', 'X_2']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('heading', { name: 'Build a DAG or run discovery', exact: true })).toBeVisible({ timeout: 60_000 })
  await createDag(page, 'Chapter impact model')
  for (const cause of ['D', 'X_1', 'X_2']) await addArrow(page, cause, 'Y', cause === 'D' ? 'The intervention starts at time zero.' : 'The chapter supplies an unaffected predictor of the outcome.')
  await identify(page, { graph: 'Chapter impact model', treatment: 'D', outcome: 'Y', mechanism: 'Policy change', sentence: 'Treatment begins at time zero; the control series are not affected by the intervention.' })
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).click()
  await page.getByRole('radio', { name: /^Causal impact/ }).click()
  await page.getByRole('radio', { name: 'Bayesian', exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Posterior draws', exact: true }).fill('900')
  await page.getByRole('spinbutton', { name: 'Warmup iterations', exact: true }).fill('100')
  for (const column of ['X_1', 'X_2']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /^Run causal impact/i }).click()
  await expect(page.getByText('Current estimate', { exact: true }).first()).toBeVisible({ timeout: 60_000 })
  await capture(page, info, 'chapter-bayesian-impact.png')
  const saved = await exportProject(page)
  expect(saved.estimationRuns).toHaveLength(1)
  const run = saved.estimationRuns[0]
  expect(run.configuration.inference).toEqual({ kind: 'bayesian', draws: 900, warmup: 100, seed: 1234, priorLevelSd: 0.01 })
  expect(run.evidence.kind).toBe('bayesianCausalImpact')
  expect(run.evidence.nPre).toBe(15)
  expect(run.evidence.nPost).toBe(16)
  expect(run.evidence.controls).toHaveLength(2)
  expect(run.evidence.counterfactual).toHaveLength(16)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{ name:'Cumulative',exact:true }).first().check()
  await expect(page.getByRole('img', { name: 'Impact effects', exact: true }).first()).toBeVisible()
  await page.getByRole('spinbutton',{ name:'Posterior draws',exact:true }).fill('100000')
  await page.getByRole('button',{ name:/^Run causal impact/i }).click()
  await expect(page.getByRole('button',{ name:'Cancel run',exact:true })).toBeVisible()
  await chapter(page,/Study design/)
  await chapter(page,/Estimation/)
  await expect(page.getByRole('button',{ name:'Cancel run',exact:true })).toBeVisible()
  await page.getByRole('button',{ name:'Cancel run',exact:true }).click()
  await expect(page.getByRole('button',{ name:'Cancel run',exact:true })).toHaveCount(0)
  const afterCancel = await exportProject(page)
  expect(afterCancel.estimationRuns).toHaveLength(1)
  expect(run.estimate.interval).toEqual({ kind: 'credible', level: 0.95, summary: 'ETI', lower: run.evidence.cumulativeSummary.absolute.lower, upper: run.evidence.cumulativeSummary.absolute.upper })
  await info.attach('exported-evidence', { body: JSON.stringify(run.evidence, null, 2), contentType: 'application/json' })
  await validateSaved(page, afterCancel)
  await reopenResult(page, 'chapter-impact.csv')
  expect(errors).toEqual([])
})
