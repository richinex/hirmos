import { expect, test, type Page } from '@playwright/test'
import { fileURLToPath } from 'node:url'
import { bodyText, prepare } from './examples/support'

test.describe.configure({ timeout: 180_000 })

const upstreamData = (name: string): string =>
  fileURLToPath(new URL(`../public/examples/data/survival/${name}`, import.meta.url))

const openExample = async (page: Page, name: string): Promise<void> => {
  const title = page.getByText(name, { exact: true })
  const entry = title.locator('xpath=ancestor::*[.//button[normalize-space()="Open"]][1]')
  await entry.getByRole('button', { name: 'Open', exact: true }).click()
}

const openSurvivalChapter = async (page: Page, phone: boolean): Promise<void> => {
  if (phone) await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await page
    .getByRole('navigation', { name: 'Workspace chapters' })
    .getByRole('button', { name: /Survival analysis/ })
    .click()
}

const chooseColumn = async (page: Page, label: string, option: string): Promise<void> => {
  await page.getByRole('combobox', { name: label }).click()
  await page.getByRole('option', { name: option, exact: true }).click()
}

const createPreparedProject = async (
  page: Page,
  name: string,
  source: string,
  columns: readonly string[],
): Promise<void> => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(upstreamData(source))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
  await prepare(page, { structure: 'cross-section', columns })
  await openSurvivalChapter(page, false)
}

test('the standalone survival chapter renders in desktop and phone workbenches', async ({ page }, testInfo) => {
  const phone = testInfo.project.name === 'mobile-chromium'
  await page.goto('/app/projects')
  await openExample(page, phone ? 'AI adoption, company-wide' : 'Seat-belt law and road deaths')
  await openSurvivalChapter(page, phone)

  await expect(page.getByRole('heading', { name: 'Time until an event' })).toBeVisible()
  await expect(page.getByText('09 · Survival analysis')).toBeVisible()
  await expect(page.getByRole('button', { name: 'Run survival analysis' })).toBeVisible()
  await expect(page.locator('main#stage')).toHaveCSS('display', 'flex')
  await expect(page.getByRole('navigation', { name: 'Workspace chapters' })).toHaveCSS('scrollbar-width', 'none')
})

test('does not expose an implementation error when a no-covariate example run is refused', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The same runtime path is used at both widths.')
  await page.goto('/app/projects')
  await openExample(page, 'AI adoption, company-wide')
  await openSurvivalChapter(page, false)

  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('alert').or(page.getByText('Survival runs · 1'))).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText(/isNonEmpty|ReferenceError/)).toHaveCount(0)
})

test('runs ComparisonSurv on the exact crossing-curves package data and records the result', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'ComparisonSurv crossing curves', 'comparison_surv_crossdata.csv', ['time', 'status', 'group'])

  await page.getByRole('radio', { name: 'Compare groups' }).click()
  await chooseColumn(page, 'Duration', 'time')
  await chooseColumn(page, 'Event · 1 observed, 0 censored', 'status')
  await chooseColumn(page, 'Group · 0 or 1', 'group')
  await page.getByRole('spinbutton', { name: 'Compare through time' }).fill('2')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Group 1 compared with group 0' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('Two-group survival comparison').first()).toBeVisible()
  await expect(page.getByText(/when they cross/i).first()).toBeVisible()
  await expect(page.getByText('Survival runs · 1')).toBeVisible()
  await expect(page.getByRole('table', { name: 'Number at risk' })).toBeVisible()

  await page.getByRole('radio', { name: 'Event-free time' }).click()
  await expect(page.getByTestId('restricted-mean-groups')).toBeVisible()
  await expect(page.getByText(/group 0 accumulated/i)).toBeVisible()

  await page.getByRole('radio', { name: 'Cumulative hazard' }).click()
  await expect(page.getByTestId('cumulative-hazard-groups')).toBeVisible()

  await page.getByRole('radio', { name: 'Smoothed hazard' }).click()
  await expect(page.getByTestId('smoothed-hazard-groups')).toBeVisible()
})

test('runs right-censored flexsurv on the exact breast-cancer package data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'flexsurv breast cancer', 'flexsurv_bc.csv', ['censrec', 'recyrs'])

  await chooseColumn(page, 'Duration', 'recyrs')
  await chooseColumn(page, 'Event · 1 observed, 0 censored', 'censrec')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('5')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Weibull AFT' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('686 rows · 299 events').first()).toBeVisible()
  await expect(page.getByText('Survival runs · 1')).toBeVisible()

  await page
    .getByRole('navigation', { name: 'Workspace chapters' })
    .getByRole('button', { name: /Results/ })
    .click()
  await expect(page.getByRole('heading', { name: 'Review the complete analysis' })).toBeVisible()
  await expect(page.getByRole('combobox', { name: 'Survival run' })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Weibull AFT' }).first()).toBeVisible()
})

test('runs start-stop and multi-state flexsurv on the exact bosms3 package data', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The numerical browser flow runs once; phone layout is covered separately.')
  await createPreparedProject(page, 'flexsurv multi-state', 'flexsurv_bosms3.csv', ['from', 'to', 'Tstart', 'Tstop', 'status'])

  await page.getByRole('radio', { name: 'Start–stop' }).click()
  await chooseColumn(page, 'Start time', 'Tstart')
  await chooseColumn(page, 'Stop time', 'Tstop')
  await chooseColumn(page, 'Event · 1 observed, 0 censored', 'status')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('12')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Weibull PH' }).first()).toBeVisible({ timeout: 120_000 })

  await page.getByRole('radio', { name: 'Multi-state' }).click()
  await chooseColumn(page, 'Start time', 'Tstart')
  await chooseColumn(page, 'Stop time', 'Tstop')
  await chooseColumn(page, 'Event · 1 transition, 0 censored', 'status')
  await chooseColumn(page, 'Origin state', 'from')
  await chooseColumn(page, 'Destination state', 'to')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('12')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByText('Multi-state survival').first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('Survival runs · 2')).toBeVisible()
  await bodyText(page, 'For observations starting in state 1')

  await page.getByRole('radio', { name: 'Transitions' }).click()
  await expect(page.getByTestId('state-transitions')).toBeVisible()
  await page.getByRole('radio', { name: 'Probability matrix' }).click()
  await expect(page.getByTestId('transition-probability-matrix')).toBeVisible()
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
  await expect(page.getByText('455 rows · 132 events').first()).toBeVisible()

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
})
