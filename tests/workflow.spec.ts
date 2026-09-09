import { expect, test, type Locator, type Page } from '@playwright/test'
import { fileURLToPath } from 'node:url'

const seatbelts = fileURLToPath(new URL('./fixtures/Seatbelts.csv', import.meta.url))
const seatbeltsParquet = fileURLToPath(new URL('./fixtures/Seatbelts.parquet', import.meta.url))
const gaps = fileURLToPath(new URL('./fixtures/gaps.csv', import.meta.url))

/** Pick an option in a Radix select: open the trigger, then click the option in the listbox. */
const choose = async (trigger: Locator, option: { readonly label?: string; readonly index?: number }) => {
  await trigger.click()
  const list = trigger.page().getByRole('listbox')
  await (option.label !== undefined ? list.getByRole('option', { name: option.label, exact: true }) : list.getByRole('option').nth(option.index ?? 0)).click()
}

const createProject = async (page: Page) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Seat-belt law')
  await page.getByRole('button', { name: 'Create project' }).click()
  await expect(page.getByRole('heading', { name: 'Choose a data file' })).toBeVisible()
}

test('validates project names at the form boundary', async ({ page }) => {
  await page.goto('/app')
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
  const schemaRows = page.getByRole('region', { name: 'Physical schema' }).getByRole('row')
  await expect(schemaRows.filter({ has: page.getByRole('cell', { name: 'y', exact: true }) })).toContainText('1')
  const preview = page.getByLabel('Preview')
  await expect(preview.getByRole('cell', { name: 'unavailable: missing value', exact: true })).toBeVisible()
  await expect(preview.getByRole('cell', { name: '0', exact: true })).toBeVisible()
})

test('renders calendar dates as dates in the source preview', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Canonical preview runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'dated.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('date,x\n1958-03-29,1\n1958-04-05,2\n'),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()

  const first = page.getByLabel('Preview').getByRole('row').nth(1)
  await expect(first.getByRole('cell').nth(0)).toHaveText('1958-03-29 00:00:00', { timeout: 30_000 })
  await expect(first).not.toContainText('-371174400000')
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
  // Each figure renders a screen-reader form beside the visible one, so assert the visible span.
  const profileFacts = page.locator('dl[aria-label="Dataset size"]')
  await expect(profileFacts.locator('dd').nth(0).locator('[aria-hidden]')).toHaveText('192')
  await expect(profileFacts.locator('dd').nth(1).locator('[aria-hidden]')).toHaveText('9')
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
  await expect(page.getByRole('button', { name: 'Discovery Lab' })).toHaveAttribute('aria-disabled', 'true')
})

test('rejects an unknown chapter query without losing the workflow entry point', async ({ page }) => {
  await page.goto('/app?chapter=unknown')
  await expect(page.getByRole('alert')).toContainText('Unknown chapter “unknown”')
  await expect(page.getByRole('heading', { name: 'Create an analysis' })).toBeVisible()
})

test('keeps the current stage usable on a phone', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'Mobile workflow snapshot')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })

  const mobileNav = page.locator('nav[aria-label="Workspace chapters"]')
  await expect(mobileNav).toHaveAttribute('inert', '')
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await expect(mobileNav).not.toHaveAttribute('inert', '')
  await expect(mobileNav.getByRole('button', { name: /Data studio/ })).toHaveAttribute('aria-current', 'page')
  await expect(mobileNav.getByRole('button', { name: /Discovery lab/ })).toHaveAttribute('aria-disabled', 'true')
  await expect(mobileNav.getByRole('button', { name: /Study design/ })).toHaveAttribute('aria-disabled', 'true')
  await expect(mobileNav.getByRole('button', { name: /Counterfactuals/ })).toHaveAttribute('aria-disabled', 'true')
  await expect(mobileNav.getByRole('button')).toHaveCount(10)
  await page.keyboard.press('Escape')
  await expect(mobileNav).toHaveAttribute('inert', '')
  await expect(page.getByRole('region', { name: 'Physical schema' }).getByRole('row').nth(1).getByRole('cell').nth(4)).not.toHaveText('…', { timeout: 30_000 })
  await expect(page).toHaveScreenshot('seatbelts-source-selected-mobile.png', { fullPage: true, maxDiffPixels: 500 })
})

test('keeps temporal discovery usable on a phone without widening the page', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'Mobile discovery workflow')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Set the analysis dataset' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('radio', { name: 'Regular time series' }).click()
  await choose(page.getByLabel('Time column'), { label: 'rownames' })
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByText('Prepared time series · 192 rows')).toBeVisible({ timeout: 30_000 })

  await page.getByRole('radio', { name: /Granger/ }).click()
  const mobileGranger = page.getByRole('region', { name: 'Granger predictive test' })
  await choose(mobileGranger.getByLabel('Candidate cause'), { label: 'drivers' })
  await choose(mobileGranger.getByLabel('Target'), { label: 'DriversKilled' })
  await choose(mobileGranger.getByLabel('Maximum lag'), { label: '2' })
  await mobileGranger.getByRole('button', { name: 'Run Granger test' }).click()
  await expect(mobileGranger.getByText('drivers → DriversKilled')).toBeVisible({ timeout: 30_000 })
  const mobileGrangerTable = mobileGranger.getByRole('region', { name: 'Granger raw evidence' })
  await expect(mobileGrangerTable.getByRole('row')).toHaveCount(3)
  await expect(mobileGrangerTable.getByRole('columnheader', { name: 'Sum-of-squared-residuals F statistic' })).toBeVisible()
  await expect(mobileGranger.getByText('not a causal estimate', { exact: false }).first()).toBeVisible()

  const mobileNav = page.locator('nav[aria-label="Workspace chapters"]')
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await mobileNav.getByRole('button', { name: /Discovery lab/ }).click()

  // Exact, so the family segmented control above it is not matched as well. The keys move within one
  // family, whose last method is RPCMCI.
  const discoveryFamilies = page.getByRole('radiogroup', { name: 'Discovery method family' })
  const discoveryMethods = page.getByRole('radiogroup', { name: 'Discovery method', exact: true })
  const pcmci = discoveryMethods.getByRole('radio', { name: /^PCMCI\+/ })
  const lpcmci = discoveryMethods.getByRole('radio', { name: /LPCMCI/ })
  const rpcmci = discoveryMethods.getByRole('radio', { name: /RPCMCI/ })
  expect(await pcmci.evaluate((element) => element instanceof HTMLInputElement && element.type === 'radio')).toBe(true)
  await pcmci.focus()
  await pcmci.press('ArrowRight')
  await expect(lpcmci).toBeChecked()
  await lpcmci.press('End')
  await expect(rpcmci).toBeChecked()
  await rpcmci.press('Home')
  await expect(pcmci).toBeChecked()
  await discoveryFamilies.getByRole('radio', { name: 'Neural' }).click()
  await expect(discoveryMethods.getByRole('radio', { name: 'cMLP' })).toBeEnabled()
  await expect(discoveryMethods.getByRole('radio', { name: 'cLSTM' })).toBeEnabled()
  await discoveryMethods.getByRole('radio', { name: 'cMLP' }).click()
  await page.getByRole('button', { name: 'Run cMLP' }).click()
  await expect(page.getByRole('button', { name: 'Cancel run' })).toBeVisible()

  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await mobileNav.getByRole('button', { name: /Data studio/ }).click()
  const discoveryActivity = page.getByRole('button', { name: 'Discovery running in Discovery lab; open it' })
  await expect(discoveryActivity).toBeVisible()
  await discoveryActivity.click()
  await expect(discoveryFamilies.getByRole('radio', { name: 'Neural' })).toBeChecked()
  await expect(discoveryMethods.getByRole('radio', { name: 'cMLP' })).toBeChecked()
  await expect(page.getByRole('button', { name: 'Cancel run' })).toBeVisible()

  await page.getByRole('button', { name: 'Cancel run' }).click()
  await expect(page.getByRole('button', { name: 'Cancel run' })).toBeHidden()
  await expect(page.getByRole('button', { name: 'Run cMLP' })).toBeEnabled()
  await page.getByRole('spinbutton', { name: 'Hidden width' }).fill('30')
  await page.getByText('Training settings', { exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Maximum iterations' }).fill('1000')
  await page.getByRole('spinbutton', { name: 'Check every' }).fill('100')
  await page.getByRole('button', { name: 'Run cMLP' }).click()

  await expect(page.getByRole('button', { name: 'Cancel run' })).toBeVisible()
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await mobileNav.getByRole('button', { name: /Data studio/ }).click()
  await expect(discoveryActivity).toBeVisible()
  await expect(discoveryActivity).toBeHidden({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await mobileNav.getByRole('button', { name: /Discovery lab/ }).click()
  await expect(page.getByText('Lag-resolved neural Granger evidence')).toBeVisible({ timeout: 30_000 })
  await expect(page.getByText('1 run')).toBeVisible()
  await expect(page.getByText('divided by its recorded population standard deviation')).toBeVisible()
  await discoveryFamilies.getByRole('radio', { name: 'PCMCI' }).click()
  await discoveryMethods.getByRole('radio', { name: /^PCMCI\+/ }).click()

  const pageWidth = await page.evaluate(() => ({ viewport: window.innerWidth, document: document.documentElement.scrollWidth }))
  expect(pageWidth.document).toBeLessThanOrEqual(pageWidth.viewport)

  await page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: 'Prepared dataset and method requirements' }).click()
  const sheet = page.getByRole('dialog', { name: 'Prepared dataset and method requirements' })
  await expect(sheet).toBeVisible()
  await expect(sheet.getByRole('region', { name: 'Method requirements' })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(sheet).toBeHidden()
  await expect(page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: 'Prepared dataset and method requirements' })).toBeFocused()
})

test('runs DirectLiNGAM for independent observations and carries its relations into the DAG workspace', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Cross-sectional discovery workflow runs once')
  await createProject(page)
  const records = ['x,y,z']
  for (let row = 0; row < 96; row += 1) {
    const x = ((row * 37 % 101) - 50) / 25
    const y = 0.8 * x + ((row * 61 % 103) - 51) / 30
    const z = -0.5 * y + ((row * 73 % 107) - 53) / 35
    records.push(`${x},${y},${z}`)
  }
  await page.locator('input[type="file"]').setInputFiles({
    name: 'direct-lingam-cross-section.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(`${records.join('\n')}\n`),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Set the analysis dataset' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('radio', { name: 'Independent observations' }).click()
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: 'y', exact: true }).check()
  await page.getByRole('checkbox', { name: 'z', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByText('Prepared cross-section · 96 rows')).toBeVisible({ timeout: 30_000 })

  const navigation = page.getByRole('navigation', { name: 'Workspace chapters' })
  await navigation.getByRole('button', { name: /Discovery lab/ }).click()
  await expect(page.getByRole('heading', { name: 'Examine candidate relationships' })).toBeVisible()
  // Methods are grouped into families, so only the selected family's methods are on the page. The
  // eligible method decides the family, and the time-series methods are checked in their own.
  const discoveryFamilies = page.getByRole('radiogroup', { name: 'Discovery method family' })
  const discoveryMethods = page.getByRole('radiogroup', { name: 'Discovery method', exact: true })
  await expect(discoveryFamilies.getByRole('radio')).toHaveCount(7)
  await expect(page.getByRole('radio', { name: 'DirectLiNGAM' })).toBeChecked()
  await expect(discoveryMethods.getByRole('radio', { name: /VAR-LiNGAM/ })).toBeDisabled()
  await discoveryFamilies.getByRole('radio', { name: /^Constraint Cross-sectional/ }).click()
  await expect(discoveryMethods.getByRole('radio', { name: 'PC-stable' })).toBeEnabled()
  await expect(discoveryMethods.getByRole('radio', { name: 'FCI' })).toBeEnabled()
  await discoveryMethods.getByRole('radio', { name: 'FCI' }).click()
  await expect(page.getByRole('combobox', { name: 'CI test' })).toBeVisible()
  await expect(page.getByText('Background knowledge', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Run FCI' }).click()
  await expect(page.getByRole('heading', { name: 'Partial ancestral graph' })).toBeVisible({ timeout: 30_000 })
  await expect(page.getByRole('img', { name: 'FCI partial ancestral graph' })).toBeVisible()
  await discoveryFamilies.getByRole('radio', { name: 'PCMCI' }).click()
  await expect(discoveryMethods.getByRole('radio', { name: /^PCMCI\+/ })).toBeDisabled()
  await discoveryFamilies.getByRole('radio', { name: 'LiNGAM' }).click()
  await page.getByRole('radio', { name: 'DirectLiNGAM' }).click()
  await discoveryFamilies.getByRole('radio', { name: 'Neural' }).click()
  await expect(discoveryMethods.getByRole('radio', { name: 'cMLP' })).toBeDisabled()
  await expect(discoveryMethods.getByRole('radio', { name: 'cLSTM' })).toBeDisabled()
  await discoveryFamilies.getByRole('radio', { name: 'LiNGAM' }).click()
  await page.getByRole('button', { name: 'Run DirectLiNGAM' }).click()

  await expect(page.getByLabel('DirectLiNGAM causal order')).toContainText('→', { timeout: 30_000 })
  await expect(page.getByRole('img', { name: 'DirectLiNGAM structure' })).toBeVisible()
  await expect(page.getByRole('img', { name: 'DirectLiNGAM weight heatmap' })).toBeVisible()
  const weights = page.getByRole('region', { name: 'DirectLiNGAM raw weights' })
  await expect(weights.getByRole('row')).toHaveCount(10)

  await navigation.getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: 'Discovery-informed' }).click()
  // The origin records only the runs that are ticked, so the contributing run is chosen explicitly.
  await page.getByRole('group', { name: 'Discovery runs reviewed for this DAG' })
    .getByRole('checkbox', { name: /DirectLiNGAM/ }).check()
  await page.getByLabel('DAG name').fill('DirectLiNGAM review')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  await expect(page.getByRole('heading', { name: 'DirectLiNGAM', exact: true })).toBeVisible()
  await expect(page.getByLabel('DirectLiNGAM evidence graph', { exact: true })).toBeVisible()
  await expect(page.getByLabel('Discovered relations').getByRole('button')).not.toHaveCount(0)
})

test('saves per-column time-series transformations and previews the materialized values', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Prepared transformation workflow runs once')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'prepared-transformations.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('time,x,y\n1,10,1\n2,12,4\n3,15,9\n4,19,16\n5,24,25\n6,30,36\n'),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Set the analysis dataset' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('radio', { name: 'Regular time series' }).click()
  await choose(page.getByLabel('Time column'), { label: 'time' })
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: 'y', exact: true }).check()
  await page.getByRole('radiogroup', { name: 'Transformation for x' }).getByRole('radio', { name: 'First difference' }).click()
  await page.getByRole('radiogroup', { name: 'Transformation for y' }).getByRole('radio', { name: 'Linear detrend' }).click()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByText('Prepared time series · 5 rows')).toBeVisible({ timeout: 30_000 })
  await expect(page.getByText(/x: first difference, y: linear detrend/)).toBeVisible()

  const plots = page.getByRole('list', { name: 'Prepared series plots' })
  await expect(plots.getByTestId('prepared-series')).toHaveCount(2, { timeout: 30_000 })
  await expect(plots).toContainText('x')
  await expect(plots).toContainText('first difference')
  await expect(plots).toContainText('linear detrend')
  await expect(page.getByText('5 aligned rows')).toBeVisible()
  await expect(page.getByText('first source row removed for alignment')).toBeVisible()
})

test('shows the saved STL decomposition and ACF/PACF diagnostics', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Time-series visualization workflow runs once')
  const errors: string[] = []
  page.on('pageerror', (error) => errors.push(error.message))
  await createProject(page)
  const rows = Array.from({ length: 72 }, (_, index) => `${index + 1},${0.03 * index + 2 * Math.sin(index * Math.PI / 6) + 0.1 * Math.cos(index * 0.7)}`)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'seasonal-series.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(`time,x\n${rows.join('\n')}\n`),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Regular time series' }).click()
  await choose(page.getByLabel('Time column'), { label: 'time' })
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: /Remove the seasonal component/ }).check()
  await page.getByRole('group', { name: 'Columns to adjust seasonally' }).getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()

  await expect(page.getByText('Prepared time series · 72 rows')).toBeVisible({ timeout: 30_000 })
  const decomposition = page.getByTestId('stl-decomposition')
  await expect(decomposition).toBeVisible({ timeout: 30_000 })
  await expect(decomposition).toHaveAttribute('aria-description', /observed equals trend plus seasonal plus remainder/i)

  await page.getByRole('radio', { name: /Breaks/ }).click()
  await page.getByRole('button', { name: 'Analyse temporal structure' }).click()
  const correlation = page.getByTestId('acf-pacf')
  await expect(correlation).toBeVisible({ timeout: 30_000 })
  await expect(correlation).toHaveAttribute('aria-description', /autocorrelation and partial autocorrelation from lag 0 through lag 35/i)
  expect(errors).toEqual([])
})

test('explains the opposing stationarity null hypotheses and labels every critical value', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Stationarity presentation runs once')
  await createProject(page)
  const rows = Array.from({ length: 96 }, (_, index) => `${index + 1},${index + Math.sin(index / 4)}`)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'stationarity-copy.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(`time,x\n${rows.join('\n')}\n`),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Regular time series' }).click()
  await choose(page.getByLabel('Time column'), { label: 'time' })
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByText('Prepared time series · 96 rows')).toBeVisible({ timeout: 30_000 })

  // Diagnostics open on redundancy, so the stationarity pane has to be selected before it is read.
  await page.getByRole('radio', { name: /Stationarity/ }).click()
  await expect(page.getByText(/ADF tests a unit root as its null; KPSS tests stationarity as its null/)).toBeVisible()
  await expect(page.getByText(/prepared data are not changed unless first differencing is saved as a transformation/)).toBeVisible()
  await page.getByRole('button', { name: 'Run stationarity tests' }).click()
  await expect(page.getByText(/Stationarity tests · 1 variables · 96 rows/)).toBeVisible({ timeout: 120_000 })
  await expect(page.getByRole('columnheader', { name: 'ADF p · constant' })).toBeVisible()
  await expect(page.getByRole('columnheader', { name: 'KPSS p · constant' })).toBeVisible()
  await expect(page.getByText(/ADF · constant · prepared values: p/)).toBeVisible()
  await page.getByText('x · test statistics', { exact: true }).click()

  const rawEvidence = page.getByRole('region', { name: 'Stationarity raw evidence' })
  const adf = rawEvidence.getByRole('row').filter({ hasText: 'ADF · constant' }).first()
  const kpss = rawEvidence.getByRole('row').filter({ hasText: 'KPSS · constant' }).first()
  await expect(adf).toContainText('1%:')
  await expect(adf).toContainText('5%:')
  await expect(adf).toContainText('10%:')
  await expect(kpss).toContainText('10%:')
  await expect(kpss).toContainText('5%:')
  await expect(kpss).toContainText('2.5%:')
  await expect(kpss).toContainText('1%:')
})

test('keeps the current chapter visible until a cold lazy chapter is ready', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Cold chunk navigation runs once')
  let releaseChunk: () => void = () => undefined
  const chunkGate = new Promise<void>((resolve) => { releaseChunk = resolve })
  let reportChunkRequest: () => void = () => undefined
  const chunkRequested = new Promise<void>((resolve) => { reportChunkRequest = resolve })
  await page.route('**/src/components/dag/DagWorkspace.tsx*', async (route) => {
    reportChunkRequest()
    await chunkGate
    await route.continue()
  })

  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles({
    name: 'cold-chapter.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('x,y\n1,2\n2,4\n3,6\n4,8\n'),
  })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Set the analysis dataset' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('radio', { name: 'Independent observations' }).click()
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: 'y', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByText('Prepared cross-section · 4 rows')).toBeVisible({ timeout: 30_000 })

  const navigation = page.getByRole('navigation', { name: 'Workspace chapters' })
  await navigation.getByRole('button', { name: /DAG workspace/ }).click()
  await chunkRequested
  await expect(page).toHaveURL(/\/app$/)
  await expect(page.getByRole('heading', { name: 'Set the analysis dataset' })).toBeVisible()
  await expect(page.getByRole('status', { name: 'Loading DAG editor…' })).toHaveCount(0)

  releaseChunk()
  await expect(page).toHaveURL(/\/app\/dag$/)
  await expect(page.getByRole('button', { name: 'Substantive knowledge' })).toBeVisible()
})
