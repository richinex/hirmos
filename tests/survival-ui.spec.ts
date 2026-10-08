import { expect, test, type Page } from '@playwright/test'
import { fileURLToPath } from 'node:url'
import { readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { bodyText, prepare } from './examples/support'

test.describe.configure({ timeout: 180_000 })

const upstreamData = (name: string): string =>
  fileURLToPath(new URL(`../public/examples/data/survival/${name}`, import.meta.url))

const testData = (name: string): string =>
  fileURLToPath(new URL(`fixtures/${name}`, import.meta.url))

const openExample = async (page: Page, name: string): Promise<void> => {
  await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
}

const openSurvivalChapter = async (page: Page, phone: boolean): Promise<void> => {
  if (phone) await page.getByRole('button', { name: 'Expand section list' }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Survival analysis/ })
    .click()
}

const chooseColumn = async (page: Page, label: string, option: string): Promise<void> => {
  await page.getByRole('combobox', { name: label }).click()
  await page.getByRole('option', { name: option, exact: true }).click()
}

const chooseColumnWithin = async (
  page: Page,
  scope: ReturnType<Page['getByRole']>,
  label: string,
  option: string,
): Promise<void> => {
  await scope.getByRole('combobox', { name: label }).click()
  await page.getByRole('option', { name: option, exact: true }).click()
}

const createPreparedProjectFrom = async (
  page: Page,
  name: string,
  source: string | { name: string; mimeType: string; buffer: Buffer },
  columns: readonly string[],
): Promise<void> => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(source)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
  await prepare(page, { structure: 'cross-section', columns })
  await openSurvivalChapter(page, (page.viewportSize()?.width ?? 1280) < 640)
}

const createPreparedProject = async (
  page: Page,
  name: string,
  source: string,
  columns: readonly string[],
): Promise<void> => createPreparedProjectFrom(page, name, upstreamData(source), columns)

test('the standalone survival chapter renders in desktop and phone workbenches', async ({ page }, testInfo) => {
  const phone = testInfo.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await openExample(page, phone ? 'AI adoption, company-wide' : 'Seat-belt law and road deaths')
  await openSurvivalChapter(page, phone)

  await expect(page.getByRole('heading', { name: 'Survival analysis', exact: true })).toBeVisible()
  await expect(page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Survival analysis/ })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Run survival analysis' })).toBeVisible()
  await expect(page.locator('main#stage')).toHaveCSS('display', 'flex')
  // Idle scrollbars are thin and transparent everywhere; they show only while scrolling.
  await expect(page.getByRole('navigation', { name: 'Workspace sections' })).toHaveCSS('scrollbar-width', 'thin')
  await expect(page.getByRole('navigation', { name: 'Workspace sections' })).toHaveCSS('scrollbar-color', 'rgba(0, 0, 0, 0) rgba(0, 0, 0, 0)')
})

test('does not expose an implementation error when a no-covariate example run is refused', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The same runtime path is used at both widths.')
  await page.goto('/app/projects')
  await openExample(page, 'AI adoption, company-wide')
  await openSurvivalChapter(page, false)

  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('alert').or(page.getByText('Survival runs (1)'))).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/isNonEmpty|ReferenceError/)).toHaveCount(0)
})

test('runs ComparisonSurv on the exact crossing-curves package data and records the result', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'ComparisonSurv crossing curves', 'comparison_surv_crossdata.csv', ['time', 'status', 'group'])

  await page.getByRole('radio', { name: 'Compare groups' }).click()
  await chooseColumn(page, 'Duration', 'time')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  await chooseColumn(page, 'Group 0 or 1', 'group')
  await page.getByRole('spinbutton', { name: 'Compare through time' }).fill('2')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Group 1 compared with group 0' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('Two-group survival comparison').first()).toBeVisible()
  await expect(page.getByText(/designed for crossing survival curves/i).first()).toBeVisible()
  await expect(page.getByText('Conversion comparisons').first()).toBeVisible()
  await expect(page.getByText('Fixed-time interval calculations').first()).toBeVisible()
  await expect(page.getByText('Peto–Peto modified Gehan–Wilcoxon').first()).toBeVisible()
  await expect(page.getByRole('region', { name: 'Interpretation' }).first()).toContainText('a difference of −8.64 pp')
  await expect(page.getByText('Survival runs (1)')).toBeVisible()
  await expect(page.getByRole('table', { name: 'Number at risk' })).toBeVisible()

  await page.getByRole('radio', { name: 'Event-free time' }).click()
  await expect(page.getByTestId('restricted-mean-groups')).toBeVisible()
  await expect(page.getByText(/group 0 accumulated/i)).toBeVisible()

  await page.getByRole('radio', { name: 'Cumulative hazard' }).click()
  await expect(page.getByTestId('cumulative-hazard-groups')).toBeVisible()

  await page.getByRole('radio', { name: 'Smoothed hazard' }).click()
  await expect(page.getByTestId('smoothed-hazard-groups')).toBeVisible()
})

test('reproduces the three survival comparisons from the A/B article', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProjectFrom(page, 'A/B survival comparison', testData('ab-survival.csv'), ['tstatus', 'status', 'group'])

  await page.getByRole('radio', { name: 'Compare groups' }).click()
  await chooseColumn(page, 'Duration', 'tstatus')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  await chooseColumn(page, 'Group 0 or 1', 'group')
  await page.getByRole('spinbutton', { name: 'Compare through time' }).fill('7')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Group 1 compared with group 0' }).first()).toBeVisible({ timeout: 120_000 })
  const conversions = page.getByRole('region', { name: /Conversion comparisons/ }).first()
  await expect(conversions).toContainText('Observed conversion; follow-up time ignored')
  await expect(conversions).toContainText('23.0%')
  await expect(conversions).toContainText('25.3%')
  await expect(conversions).toContainText('2.30 pp')
  await expect(conversions).toContainText('0.089')
  await expect(conversions).toContainText('Conversion by time 7.00; Kaplan–Meier')
  await expect(conversions).toContainText('21.1%')
  await expect(conversions).toContainText('22.7%')
  await expect(conversions).toContainText('1.62 pp')
  await expect(conversions).toContainText('0.224')
  const tests = page.getByRole('region', { name: /Comparison tests/ }).first()
  await expect(tests).toContainText('Peto–Peto modified Gehan–Wilcoxon')
  await expect(tests).toContainText('0.040')
})

test('runs right-censored flexsurv on the exact breast-cancer package data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'flexsurv breast cancer', 'flexsurv_bc.csv', ['censrec', 'recyrs'])

  await chooseColumn(page, 'Duration', 'recyrs')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'censrec')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('5')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Weibull AFT' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/686 observations\s+299 events/).first()).toBeVisible()
  await expect(page.getByText('Survival runs (1)')).toBeVisible()

  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Results/ })
    .click()
  await expect(page.getByRole('heading', { name: 'Results', exact: true })).toBeVisible()
  await expect(page.getByRole('combobox', { name: 'Survival run' })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Weibull AFT' }).first()).toBeVisible()
})

test('runs start-stop and multi-state flexsurv on the exact bosms3 package data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'flexsurv multi-state', 'flexsurv_bosms3.csv', ['from', 'to', 'Tstart', 'Tstop', 'status'])

  await page.getByRole('radio', { name: 'Start–stop' }).click()
  await chooseColumn(page, 'Start time', 'Tstart')
  await chooseColumn(page, 'Stop time', 'Tstop')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('12')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Weibull PH' }).first()).toBeVisible({ timeout: 120_000 })

  await page.getByRole('radio', { name: 'Multi-state' }).click()
  await chooseColumn(page, 'Start time', 'Tstart')
  await chooseColumn(page, 'Stop time', 'Tstop')
  await chooseColumn(page, 'Event 1 transition, 0 censored', 'status')
  await chooseColumn(page, 'Origin state', 'from')
  await chooseColumn(page, 'Destination state', 'to')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('12')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByText('Multi-state survival').first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('Survival runs (2)')).toBeVisible()
  await bodyText(page, 'For observations starting in state 1')

  await page.getByRole('radio', { name: 'Transitions' }).click()
  await expect(page.getByTestId('state-transitions')).toBeVisible()
  await page.getByRole('radio', { name: 'Probability matrix' }).click()
  await expect(page.getByTestId('transition-probability-matrix')).toBeVisible()
})

test('runs start-stop Cox regression through the worker and records its diagnostics', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProjectFrom(page, 'Cox start-stop regression', testData('survival/panel.csv'), ['id', 'start', 'stop', 'event', 'feature_active', 'experience', 'complexity'])

  await page.getByRole('radio', { name: 'Cox regression' }).click()
  await page.getByRole('radiogroup', { name: 'Cox observation structure' }).getByRole('radio', { name: 'Start–stop' }).click()
  await chooseColumn(page, 'Subject', 'id')
  await chooseColumn(page, 'Start time', 'start')
  await chooseColumn(page, 'Stop time', 'stop')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'event')
  for (const covariate of ['feature_active', 'experience', 'complexity']) {
    await page.getByRole('checkbox', { name: covariate, exact: true }).check()
  }
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Cox proportional-hazards model' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/455 intervals\s+132 events/).first()).toBeVisible()
  await expect(page.getByRole('region', { name: 'Covariate estimates' }).first()).toContainText('feature_active')
  await expect(page.getByTestId('cox-forest').first()).toBeVisible()
  await expect(page.getByTestId('cox-baseline-survival').first()).toBeVisible()
  await expect(page.getByText('Survival runs (1)')).toBeVisible()
})

test('fits clustered Breslow through the UI and preserves the Efron default', async ({ page }, testInfo) => {
  await createPreparedProjectFrom(page, 'Clustered Breslow', testData('kidney.csv'), ['time', 'status', 'age', 'sex', 'id'])
  await page.getByRole('radio', { name: 'Cox regression', exact: true }).click()
  await chooseColumn(page, 'Duration', 'time')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  await page.getByRole('radiogroup', { name: 'Cox standard errors' }).getByRole('radio', { name: 'Clustered', exact: true }).click()
  const ties = page.getByRole('radiogroup', { name: 'Cox clustered tied event times' })
  await expect(ties.getByRole('radio', { name: 'Efron' })).toBeChecked()
  await ties.getByRole('radio', { name: 'Breslow' }).click()
  for (const [group, option] of [['Cox delayed entry', 'Entry column'], ['Cox observation weights', 'Weight column'], ['Cox strata', 'Stratum column'], ['Cox penalty', 'Elastic net']]) {
    await expect(page.getByRole('radiogroup', { name: group, exact: true }).getByRole('radio', { name: option })).toBeDisabled()
  }
  await chooseColumn(page, 'Cluster', 'id')
  for (const name of ['age', 'sex']) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByText('Breslow ties', { exact: true }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('38 clusters', { exact: true }).first()).toBeVisible()
  const estimates = page.getByRole('region', { name: 'Covariate estimates' }).first()
  await expect(estimates).toContainText('0.440')
  await expect(estimates).toContainText('0.482')
  await expect(page.getByTestId('cox-baseline-survival').first()).toBeVisible()
  await estimates.scrollIntoViewIfNeeded()
  await page.screenshot({ path: testInfo.outputPath('clustered-breslow.png') })
  const readRun = () => page.evaluate(async () => {
    const db = await new Promise<IDBDatabase>((resolve, reject) => {
      const request = indexedDB.open('hirmos', 1)
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
    const records = await new Promise<{ body: string }[]>((resolve, reject) => {
      const request = db.transaction('projects').objectStore('projects').getAll()
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
    db.close()
    return records.map(({ body }) => JSON.parse(body)).find((record) => record.project.name === 'Clustered Breslow')?.survivalRuns[0]
  })
  await expect.poll(async () => (await readRun())?.evidence.fitting.kind).toBe('clusteredBreslow')
  const saved = await readRun()
  expect(saved.evidence.coefficients[0].coefficient).toBeCloseTo(0.0021815164528771131, 9)
  expect(saved.evidence.coefficients[1].coefficient).toBeCloseTo(-0.82099531459508091, 9)
  expect(saved.evidence.fitting.robustCovariance[3]).toBeCloseTo(0.23260797648908862, 9)
  expect(saved.configuration.observation.standardErrors.kind).toBe('clustered-breslow')
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Clustered Breslow', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(testData('kidney.csv'))
  await openSurvivalChapter(page, testInfo.project.name === 'mobile-chromium')
  await expect(page.getByText('Breslow ties', { exact: true }).first()).toBeVisible({ timeout: 30_000 })
})

test('reproduces the full CLSA commit-clustered fit through the UI', async ({ page }, testInfo) => {
  const directory = process.env.CLSA_COX_ORACLE
  test.skip(directory === undefined || testInfo.project.name !== 'chromium', 'Set CLSA_COX_ORACLE to the retained R clsa fixture directory for the full-data check.')
  test.setTimeout(300_000)
  const names = readFileSync(join(directory!, 'names.txt'), 'utf8').trim().split('\n')
  const source = { name: 'clsa-commit-clustered.csv', mimeType: 'text/csv', buffer: Buffer.from(['duration,event,commit,' + names.join(','), readFileSync(join(directory!, 'input.csv'), 'utf8')].join('\n')) }
  await createPreparedProjectFrom(page, 'CLSA commit-clustered', source, ['duration', 'event', 'commit', ...names])
  await page.getByRole('radio', { name: 'Cox regression', exact: true }).click()
  await chooseColumn(page, 'Duration', 'duration')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'event')
  await page.getByRole('radiogroup', { name: 'Cox standard errors' }).getByRole('radio', { name: 'Clustered', exact: true }).click()
  await chooseColumn(page, 'Cluster', 'commit')
  await page.getByRole('radiogroup', { name: 'Cox clustered tied event times' }).getByRole('radio', { name: 'Breslow' }).click()
  for (const name of names) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByText('107,802 clusters', { exact: true }).first()).toBeVisible({ timeout: 180_000 })
  const readEvidence = () => page.evaluate(async () => {
    const db = await new Promise<IDBDatabase>((resolve, reject) => {
      const request = indexedDB.open('hirmos', 1)
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
    const records = await new Promise<{ body: string }[]>((resolve, reject) => {
      const request = db.transaction('projects').objectStore('projects').getAll()
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
    db.close()
    return records.map(({ body }) => JSON.parse(body)).find((record) => record.project.name === 'CLSA commit-clustered')?.survivalRuns[0]?.evidence
  })
  await expect.poll(async () => (await readEvidence())?.observations, { timeout: 30_000 }).toBe(346808)
  const evidence = await readEvidence()
  const numbers = (name: string) => readFileSync(join(directory!, name + '.csv'), 'utf8').trim().split(/[,\s]+/).map(Number)
  numbers('coefficients').forEach((value, index) => expect(evidence.coefficients[index].coefficient).toBeCloseTo(value, 7))
  numbers('covariance').forEach((value, index) => expect(evidence.fitting.robustCovariance[index]).toBeCloseTo(value, 7))
  numbers('naive').forEach((value, index) => expect(evidence.covariance[index]).toBeCloseTo(value, 7))
  expect(evidence.fitting.convergence).toBe('converged')
  writeFileSync(testInfo.outputPath('clsa-browser-evidence.json'), JSON.stringify(evidence, null, 2))
  await testInfo.attach('clsa-browser-evidence', { body: JSON.stringify(evidence, null, 2), contentType: 'application/json' })
})

test('fits a shared gamma frailty as survival does on the kidney data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProjectFrom(page, 'Kidney gamma frailty', testData('kidney.csv'), ['time', 'status', 'age', 'sex', 'id'])

  await page.getByRole('radio', { name: 'Cox regression' }).click()
  await chooseColumn(page, 'Duration', 'time')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  await page.getByRole('radiogroup', { name: 'Cox shared frailty' }).getByRole('radio', { name: 'Gamma by group' }).click()
  await chooseColumn(page, 'Frailty group', 'id')
  await page.getByRole('radiogroup', { name: 'Cox tied event times' }).getByRole('radio', { name: 'Breslow' }).click()
  for (const covariate of ['age', 'sex']) {
    await page.getByRole('checkbox', { name: covariate, exact: true }).check()
  }
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Cox proportional-hazards model' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/76 observations\s+58 events/).first()).toBeVisible()
  // survival 3.6-4 prints the frailty variance of the final fit as 0.398; sex has coefficient −1.5568
  // (hazard ratio 0.211) and the concordance is 0.813.
  await expect(page.getByText('Shared gamma frailty by id').first()).toBeVisible()
  const estimates = page.getByRole('region', { name: 'Covariate estimates' }).first()
  await expect(estimates).toContainText('0.211')
  await expect(page.getByTestId('cox-forest').first()).toBeVisible()
  await bodyText(page, '0.398')
  await bodyText(page, '0.813')
})

test('fits a penalised Weibull AFT as lifelines does on the kidney data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProjectFrom(page, 'Kidney penalised AFT', testData('kidney.csv'), ['time', 'status', 'age', 'sex', 'id'])

  await page.getByRole('radio', { name: 'Penalised AFT' }).click()
  await chooseColumn(page, 'Duration', 'time')
  await chooseColumn(page, 'Event 1 observed, 0 censored', 'status')
  for (const covariate of ['age', 'sex']) {
    await page.getByRole('checkbox', { name: covariate, exact: true }).check()
  }
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  // lifelines 0.30.3, WeibullAFTFitter(penalizer=0.1): AIC 682.40, sex time ratio 2.30 [1.28, 4.15], concordance 0.662.
  await expect(page.getByRole('heading', { name: 'Weibull AFT with an L2 penalty' }).first()).toBeVisible({ timeout: 120_000 })
  const estimates = page.getByRole('region', { name: 'Parameter estimates' }).first()
  await expect(estimates).toContainText('2.30')
  await expect(estimates).toContainText('1.28 to 4.15')
  await expect(page.getByTestId('aft-forest').first()).toBeVisible()
  await bodyText(page, '682')
  await bodyText(page, '0.662')
  await expect(page.getByText('Survival runs (1)')).toBeVisible()
})

test('converts longitudinal state observations before fitting the multi-state model', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once.')
  await createPreparedProject(page, 'Longitudinal state observations', 'longitudinal-states.csv', ['time', 'state'])

  await page.getByRole('radio', { name: 'Multi-state' }).click()
  await page.getByRole('radio', { name: 'State observations' }).click()
  await chooseColumn(page, 'Subject', 'subject')
  await chooseColumn(page, 'Observation time', 'time')
  await chooseColumn(page, 'Observed state', 'state')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('10')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByText('Multi-state survival').first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/converted 28 exact state observations into 28 transition-risk rows/i).first()).toBeVisible()
})

test('converts the exact mstate wide illness-death data before fitting', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once.')
  await createPreparedProject(page, 'mstate wide illness-death', 'mstate_illness_death.csv', ['time2', 'status2', 'time3', 'status3'])

  await page.getByRole('radio', { name: 'Multi-state' }).click()
  await page.getByRole('radio', { name: 'Wide event history' }).click()
  const stateTwo = page.getByRole('group', { name: 'State 2' })
  const stateThree = page.getByRole('group', { name: 'State 3' })
  await chooseColumnWithin(page, stateTwo, 'Time reached or last followed', 'time2')
  await chooseColumnWithin(page, stateTwo, 'Reached 1 yes, 0 censored', 'status2')
  await chooseColumnWithin(page, stateThree, 'Time reached or last followed', 'time3')
  await chooseColumnWithin(page, stateThree, 'Reached 1 yes, 0 censored', 'status3')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('12')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByText('Multi-state survival').first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/converted 6 subject records into 16 transition-risk rows/i).first()).toBeVisible()
})

// The generated panel in tests/fixtures/survival: Weibull proportional hazards with shape 1.4 and
// coefficients −0.6, −0.3 and 0.4, drawn exactly from the piecewise cumulative hazard (truth.json).
// The fit through the chapter must land on those, with each 95% interval covering the truth.
test('recovers the planted start-stop Weibull PH truth through the chapter', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once.')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Survival panel truth')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(fileURLToPath(new URL('fixtures/survival/panel.csv', import.meta.url)))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await openSurvivalChapter(page, false)

  await page.getByRole('radio', { name: 'Start–stop' }).click()
  for (const covariate of ['feature_active', 'experience', 'complexity']) await page.getByRole('checkbox', { name: covariate, exact: true }).check()
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('36')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Weibull PH' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/455 intervals\s+132 events/).first()).toBeVisible()

  const table = page.getByRole('region', { name: 'Fitted parameters' }).first()
  const cells = async (parameter: string) => (await table.getByRole('row', { name: new RegExp(`^${parameter}\\b`) }).first().innerText()).split('\t')
  const expectRow = async (parameter: string, truth: number, tolerance: number) => {
    const row = await cells(parameter)
    const estimate = Number(row[2]?.replace('−', '-'))
    const [lower, upper] = (row[3] ?? '').replaceAll('−', '-').split(' to ').map(Number)
    expect(Math.abs(estimate - truth)).toBeLessThan(tolerance)
    expect(lower).toBeLessThanOrEqual(truth)
    expect(upper).toBeGreaterThanOrEqual(truth)
  }
  await expectRow('shape', 1.4, 0.15)
  await expectRow('feature_active', -0.6, 0.15)
  await expectRow('experience', -0.3, 0.1)
  await expectRow('complexity', 0.4, 0.1)

  // Leaving and returning shows the controls as the recorded run set them, not the defaults.
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Data studio/ }).click()
  await openSurvivalChapter(page, false)
  await expect(page.getByRole('radio', { name: 'Start–stop' })).toBeChecked()
  for (const covariate of ['feature_active', 'experience', 'complexity']) await expect(page.getByRole('checkbox', { name: covariate, exact: true })).toBeChecked()
  await expect(page.getByRole('spinbutton', { name: 'Prediction horizon' })).toHaveValue('36')
})
