import { expect, test, type Page } from '@playwright/test'
import { fileURLToPath } from 'node:url'

const seatbelts = fileURLToPath(new URL('./fixtures/Seatbelts.csv', import.meta.url))
const seatbeltsParquet = fileURLToPath(new URL('./fixtures/Seatbelts.parquet', import.meta.url))

const createProject = async (page: Page) => {
  await page.goto('/')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Seat-belt law')
  await page.getByRole('button', { name: 'Create project' }).click()
  await expect(page.getByRole('heading', { name: 'Choose a data file' })).toBeVisible()
}

test('validates project names at the form boundary', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: 'Create project' }).click()
  await expect(page.getByRole('alert')).toHaveText('Give the analysis a name.')
  await expect(page.getByText('Recommendations support judgment')).toHaveCount(0)
})

test('rejects unsupported and empty files', async ({ page }) => {
  await createProject(page)
  const input = page.locator('input[type="file"]')

  await input.setInputFiles({ name: 'notes.txt', mimeType: 'text/plain', buffer: Buffer.from('not data') })
  await expect(page.getByRole('alert')).toContainText('.txt is not supported yet')
  await expect(page.getByRole('alert')).toContainText('CSV, TSV, or Parquet')

  await input.setInputFiles({ name: 'empty.csv', mimeType: 'text/csv', buffer: Buffer.from('') })
  await expect(page.getByRole('alert')).toHaveText('The selected file is empty.')
})

test('loads the pinned R Seatbelts data through the current workflow', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Desktop workflow snapshot')
  const externalRequests = new Set<string>()
  const localRequests: string[] = []
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') {
      externalRequests.add(url.href)
    } else {
      localRequests.push(url.pathname)
    }
  })
  await createProject(page)

  await expect(page.getByRole('button', { name: 'Data Studio' })).toHaveAttribute('aria-current', 'page')
  for (const chapter of ['Discovery Lab', 'DAG Workspace', 'Study Design', 'Estimation', 'Robustness', 'Results']) {
    await expect(page.getByRole('button', { name: chapter })).toBeDisabled()
  }

  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible()
  await expect(page.getByText('Seatbelts.csv')).toBeVisible()
  await expect(page.getByText(/^csv · /)).toBeVisible()
  expect(localRequests.some((path) => path.includes('data.worker'))).toBe(false)
  expect(localRequests.some((path) => path.includes('duckdb') && path.endsWith('.wasm'))).toBe(false)

  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
  const profileFacts = page.locator('dl')
  await expect(profileFacts.getByText('192', { exact: true })).toBeVisible()
  await expect(profileFacts.getByText('9', { exact: true })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Physical schema' })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Preview' })).toBeVisible()
  await expect(page.getByRole('columnheader', { name: 'DriversKilled', exact: true })).toBeVisible()
  await expect(page.getByText(/SHA-256 61f8faf2aba2/)).toBeVisible()
  expect(localRequests.some((path) => path.includes('data.worker'))).toBe(true)
  expect(localRequests.some((path) => path.includes('duckdb') && path.endsWith('.wasm'))).toBe(true)
  expect([...externalRequests]).toEqual([])
  await expect(page).toHaveScreenshot('seatbelts-source-selected.png', { fullPage: true })

  await expect(page.getByRole('button', { name: 'Discovery Lab' })).toBeDisabled()
  await expect(page.getByRole('button', { name: 'Study Design' })).toBeDisabled()
  await page.getByRole('button', { name: 'Choose another file' }).click()
  await expect(page.getByRole('heading', { name: 'Choose a data file' })).toBeVisible()
})

test('keeps null distinct from a real zero at the Arrow boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Canonical boundary test runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'null-and-zero.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('x,y\n1,\n0,2\n'),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()

  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
  const schemaRows = page.getByRole('heading', { name: 'Physical schema' }).locator('..').getByRole('row')
  await expect(schemaRows.filter({ has: page.getByRole('cell', { name: 'y', exact: true }) })).toContainText('1')
  const preview = page.getByLabel('Preview')
  await expect(preview.getByRole('cell', { name: 'null', exact: true })).toBeVisible()
  await expect(preview.getByRole('cell', { name: '0', exact: true })).toBeVisible()
})

test('profiles the pinned Seatbelts Parquet through the same canonical worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Canonical Parquet boundary runs once')
  const externalRequests = new Set<string>()
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') externalRequests.add(url.href)
  })
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbeltsParquet)
  await expect(page.getByText(/^parquet · /)).toBeVisible()
  await page.getByRole('button', { name: 'Inspect data' }).click()

  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
  const profileFacts = page.locator('dl')
  await expect(profileFacts.getByText('192', { exact: true })).toBeVisible()
  await expect(profileFacts.getByText('9', { exact: true })).toBeVisible()
  await expect(page.getByRole('columnheader', { name: 'DriversKilled', exact: true })).toBeVisible()
  await expect(page.getByText(/SHA-256 8e1ce6d1b4e6/)).toBeVisible()
  const firstPreviewRow = page.getByLabel('Preview').getByRole('row').nth(1)
  await expect(firstPreviewRow.getByRole('cell').nth(0)).toHaveText('1')
  await expect(firstPreviewRow.getByRole('cell').nth(1)).toHaveText('107')
  expect([...externalRequests]).toEqual([])
})

test('refuses a header-only CSV after canonical parsing', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Canonical boundary test runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'headers-only.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('time,outcome\n'),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()

  await expect(page.getByRole('heading', { name: 'The file has headers but no data rows.' })).toBeVisible({ timeout: 30_000 })
  await expect(page.getByRole('button', { name: 'Try again' })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Discovery Lab' })).toBeDisabled()
})

test('builds an explicit preprocessing recipe and reruns diagnostics after differencing', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Preprocessing workflow runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })

  const prepare = page.getByRole('button', { name: 'Create prepared version' })
  await expect(prepare).toBeDisabled()
  await expect(page.getByText('Choose whether rows form a time series or independent observations.')).toBeVisible()
  await page.getByRole('button', { name: 'Regular time series' }).click()
  await expect(page.getByText('Choose the time column for this regular series.')).toBeVisible()
  await page.getByLabel('Time column').selectOption({ label: 'rownames' })
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await expect(prepare).toBeEnabled()
  await prepare.click()

  await expect(page.getByText('Prepared time series · 192 observations')).toBeVisible({ timeout: 30_000 })
  const run = page.getByRole('button', { name: 'Run stationarity tests' })
  await expect(run).toBeEnabled()
  const assumptions = page.getByRole('region', { name: 'Review before running' })
  await expect(assumptions.getByText('Augmented Dickey–Fuller · 3 caveats')).toBeVisible()
  await assumptions.getByText('Augmented Dickey–Fuller · 3 caveats').click()
  await expect(assumptions.getByText('Interpret the null as a unit root; failure to reject is not proof that a unit root exists.')).toBeVisible()
  await expect(assumptions.getByText(/Source: statsmodels/).first()).toBeVisible()
  await run.click()

  await expect(page.getByText('Evidence run · 192 observations · levels')).toBeVisible({ timeout: 30_000 })
  const stationarity = page.getByRole('region', { name: 'Stationarity evidence' })
  await expect(stationarity.getByRole('cell', { name: 'DriversKilled', exact: true })).toBeVisible()
  await expect(stationarity.getByRole('cell', { name: 'drivers', exact: true })).toBeVisible()
  await expect(page.getByText('ADF null: unit root. KPSS null: stationarity.')).toBeVisible()
  await page.getByText('DriversKilled · complete numerical evidence').click()
  const rawStationarity = page.getByLabel('Stationarity raw evidence')
  await expect(rawStationarity.getByRole('row')).toHaveCount(8)
  await expect(rawStationarity.getByRole('cell', { name: 'ADF · constant + trend' })).toBeVisible()
  await expect(rawStationarity.getByRole('cell', { name: 'Zivot–Andrews · level + trend' })).toBeVisible()
  await expect(rawStationarity.getByRole('columnheader', { name: 'Critical values · reference order' })).toBeVisible()

  await page.getByRole('button', { name: 'First difference' }).click()
  await expect(page.getByText('Evidence run · 192 observations · levels')).toHaveCount(0)
  await expect(page.getByText('Prepared time series · 192 observations')).toBeVisible()
  await page.getByRole('button', { name: 'Run stationarity tests' }).click()
  await expect(page.getByText('Evidence run · 191 observations · first difference')).toBeVisible({ timeout: 30_000 })
})

test('prepares independent observations without inventing time or requiring stationarity', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Cross-sectional preparation runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })

  await page.getByRole('button', { name: 'Independent observations' }).click()
  await expect(page.getByText('Rows are independent units. Their order will not be interpreted as time.')).toBeVisible()
  await expect(page.getByLabel('Time column')).toHaveCount(0)
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()

  await page.getByRole('button', { name: 'Create prepared version' }).click()
  await expect(page.getByText('Prepared cross-section · 192 observations')).toBeVisible({ timeout: 30_000 })
  await expect(page.getByRole('button', { name: 'Run stationarity tests' })).toHaveCount(0)
  await expect(page.getByText('Stationarity and temporal transforms do not apply to independent observations.')).toBeVisible()
})

test('runs Granger and PCMCI+ from the prepared series and surfaces complete raw evidence', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Discovery workflow runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Regular time series' }).click()
  await page.getByLabel('Time column').selectOption({ label: 'rownames' })
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared version' }).click()
  await expect(page.getByText('Prepared time series · 192 observations')).toBeVisible({ timeout: 30_000 })

  await page.getByRole('button', { name: 'Discovery Lab' }).click()
  await expect(page).toHaveURL(/\?chapter=discovery$/)
  await expect(page.getByRole('heading', { name: 'Explore temporal structure' })).toBeVisible()
  await expect(page.getByText('Available with unresolved assumptions')).toBeVisible()
  await page.getByText(/Review \d+ unresolved requirements/).click()
  await expect(page.getByText('No stationarity evidence has been run for this prepared version.', { exact: false }).first()).toBeVisible()

  await page.getByRole('button', { name: 'Granger SSR F' }).click()
  await page.getByLabel('Candidate cause').selectOption({ label: 'drivers' })
  await page.getByLabel('Target').selectOption({ label: 'DriversKilled' })
  await page.getByLabel('Maximum lag').selectOption('2')
  const grangerAssumptions = page.getByRole('region', { name: 'Review before running' })
  await expect(grangerAssumptions.getByText('Granger SSR F test · 5 caveats')).toBeVisible()
  await page.getByRole('button', { name: 'Run Granger SSR F test' }).click()

  await expect(page.getByRole('heading', { name: 'drivers → DriversKilled predictive evidence' })).toBeVisible({ timeout: 30_000 })
  const grangerTable = page.getByLabel('Granger raw evidence')
  await expect(grangerTable.getByRole('row')).toHaveCount(3)
  await expect(grangerTable.getByRole('columnheader', { name: 'SSR F statistic' })).toBeVisible()
  await expect(page.getByText('This is not intervention causality.', { exact: false })).toBeVisible()
  const grangerRun = page.getByRole('article', { name: 'drivers → DriversKilled predictive evidence' })
  await grangerRun.getByText('Assumption snapshot retained with this run').click()
  await expect(grangerRun.getByText('Interpret rejection as lagged predictive precedence, not intervention causality.')).toBeVisible()

  await page.getByRole('button', { name: 'PCMCI+' }).click()
  await page.getByLabel('Maximum lag').selectOption('2')
  await page.getByRole('button', { name: 'Run PCMCI+ with ParCorr' }).click()

  await expect(page.getByRole('heading', { name: 'Stationary lag-graph evidence' })).toBeVisible({ timeout: 30_000 })
  const pcmciTable = page.getByLabel('PCMCI+ raw evidence')
  await expect(pcmciTable.getByRole('row')).toHaveCount(13)
  await expect(pcmciTable.getByRole('columnheader', { name: 'Mark' })).toBeVisible()
  await expect(pcmciTable.getByRole('columnheader', { name: 'ParCorr' })).toBeVisible()
  await expect(page.getByText('2 runs', { exact: true })).toBeVisible()
  const pcmciRun = page.getByRole('article', { name: 'Stationary lag-graph evidence' })
  await expect(pcmciRun).toHaveScreenshot('seatbelts-pcmci-result.png')

  await page.goBack()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible()
  await page.goForward()
  await expect(page.getByRole('heading', { name: 'Explore temporal structure' })).toBeVisible()
  await expect(page.getByText('2 runs', { exact: true })).toBeVisible()
})

test('refuses temporal discovery for an explicit cross-section', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Cross-sectional discovery eligibility runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Independent observations' }).click()
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared version' }).click()
  await expect(page.getByText('Prepared cross-section · 192 observations')).toBeVisible({ timeout: 30_000 })

  await page.getByRole('button', { name: 'Discovery Lab' }).click()
  await expect(page.getByText('Method refused for this prepared dataset')).toBeVisible()
  await expect(page.getByText('Temporal discovery is refused because this dataset declares independent observations.')).toBeVisible()
  await expect(page.getByRole('button', { name: 'Run PCMCI+ with ParCorr' })).toBeDisabled()
  await page.getByText('PCMCI+ with ParCorr · 6 caveats').click()
  await expect(page.getByText('Rows must be an ordered time series on the declared sampling grid.')).toBeVisible()
})

test('rejects an unknown chapter query without losing the workflow entry point', async ({ page }) => {
  await page.goto('/?chapter=unknown')
  await expect(page.getByRole('alert')).toContainText('Unknown chapter “unknown”')
  await expect(page.getByRole('heading', { name: 'Create an analysis' })).toBeVisible()
})

test('keeps the current stage usable on a phone', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'Mobile workflow snapshot')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })

  const mobileNav = page.getByRole('navigation', { name: 'Workspace chapters' })
  await expect(mobileNav.getByRole('button', { name: 'Data' })).toHaveAttribute('aria-current', 'page')
  await expect(mobileNav.getByRole('button', { name: 'Discovery' })).toBeDisabled()
  await expect(mobileNav.getByRole('button', { name: 'Study' })).toBeDisabled()
  await expect(page).toHaveScreenshot('seatbelts-source-selected-mobile.png', { fullPage: true })
})

test('keeps temporal discovery usable on a phone without widening the page', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'Mobile discovery workflow')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Regular time series' }).click()
  await page.getByLabel('Time column').selectOption({ label: 'rownames' })
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared version' }).click()
  await expect(page.getByText('Prepared time series · 192 observations')).toBeVisible({ timeout: 30_000 })

  const mobileNav = page.getByRole('navigation', { name: 'Workspace chapters' })
  await mobileNav.getByRole('button', { name: 'Discovery' }).click()
  await page.getByRole('button', { name: 'Granger SSR F' }).click()
  await page.getByLabel('Candidate cause').selectOption({ label: 'drivers' })
  await page.getByLabel('Target').selectOption({ label: 'DriversKilled' })
  await page.getByLabel('Maximum lag').selectOption('2')
  await page.getByRole('button', { name: 'Run Granger SSR F test' }).click()
  await expect(page.getByRole('heading', { name: 'drivers → DriversKilled predictive evidence' })).toBeVisible({ timeout: 30_000 })
  await expect(page.getByLabel('Granger raw evidence').getByRole('row')).toHaveCount(3)

  const pageWidth = await page.evaluate(() => ({ viewport: window.innerWidth, document: document.documentElement.scrollWidth }))
  expect(pageWidth.document).toBeLessThanOrEqual(pageWidth.viewport)
})
