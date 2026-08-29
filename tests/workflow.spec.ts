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
  const profileFacts = page.locator('dl[aria-label="Dataset size"]')
  await expect(profileFacts.locator('dd').nth(0)).toHaveText('192')
  await expect(profileFacts.locator('dd').nth(1)).toHaveText('9')
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

  const mobileNav = page.getByRole('navigation', { name: 'Workspace chapters' })
  await expect(mobileNav).toHaveCSS('width', '48px')
  await expect(mobileNav.getByRole('button', { name: 'Data Studio', exact: true })).toHaveAttribute('aria-current', 'page')
  await expect(mobileNav.getByRole('button', { name: 'Discovery Lab', exact: true })).toHaveAttribute('aria-disabled', 'true')
  await expect(mobileNav.getByRole('button', { name: 'Study Design', exact: true })).toHaveAttribute('aria-disabled', 'true')
  await expect(mobileNav.getByRole('button')).toHaveCount(9)
  await page.getByRole('button', { name: 'Expand chapter list' }).click()
  const chapterSheet = page.getByRole('dialog', { name: 'Chapters' })
  const chapterList = chapterSheet.getByRole('list', { name: 'Chapter list' })
  await expect(chapterList.getByRole('button', { name: /Data Studio/ })).toHaveAttribute('aria-current', 'page')
  await expect(chapterList.getByRole('button', { name: /Counterfactuals/ })).toHaveAttribute('aria-disabled', 'true')
  await page.keyboard.press('Escape')
  await expect(chapterSheet).toHaveCount(0)
  await expect(mobileNav).toHaveCSS('width', '48px')
  await expect(page.getByRole('region', { name: 'Physical schema' }).getByRole('row').nth(1).getByRole('cell').nth(4)).not.toHaveText('…', { timeout: 30_000 })
  await expect(page).toHaveScreenshot('seatbelts-source-selected-mobile.png', { fullPage: true, maxDiffPixels: 500 })
})

test('keeps temporal discovery usable on a phone without widening the page', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'Mobile discovery workflow')
  await createProject(page)
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Prepare analysis data' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('radio', { name: 'Regular time series' }).click()
  await choose(page.getByLabel('Time column'), { label: 'rownames' })
  await page.getByRole('checkbox', { name: 'DriversKilled', exact: true }).check()
  await page.getByRole('checkbox', { name: 'drivers', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared version' }).click()
  await expect(page.getByText('Prepared time series · 192 observations')).toBeVisible({ timeout: 30_000 })

  const mobileGranger = page.getByRole('region', { name: 'Granger predictive test' })
  await choose(mobileGranger.getByLabel('Candidate cause'), { label: 'drivers' })
  await choose(mobileGranger.getByLabel('Target'), { label: 'DriversKilled' })
  await choose(mobileGranger.getByLabel('Maximum lag'), { label: '2' })
  await mobileGranger.getByRole('button', { name: 'Run Granger test' }).click()
  await expect(mobileGranger.getByText('drivers → DriversKilled')).toBeVisible({ timeout: 30_000 })
  const mobileGrangerTable = mobileGranger.getByRole('region', { name: 'Granger raw evidence' })
  await expect(mobileGrangerTable.getByRole('row')).toHaveCount(3)
  await expect(mobileGrangerTable.getByRole('columnheader', { name: 'SSR F statistic' })).toBeVisible()
  await expect(mobileGranger.getByText('not a causal estimate', { exact: false }).first()).toBeVisible()

  const mobileNav = page.getByRole('navigation', { name: 'Workspace chapters' })
  await mobileNav.getByRole('button', { name: 'Discovery Lab', exact: true }).click()

  const pageWidth = await page.evaluate(() => ({ viewport: window.innerWidth, document: document.documentElement.scrollWidth }))
  expect(pageWidth.document).toBeLessThanOrEqual(pageWidth.viewport)

  await page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: 'Prepared input and method requirements' }).click()
  const sheet = page.getByRole('dialog', { name: 'Prepared input and method requirements' })
  await expect(sheet).toBeVisible()
  await expect(sheet.getByRole('region', { name: 'Method requirements' })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(sheet).toBeHidden()
  await expect(page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: 'Prepared input and method requirements' })).toBeFocused()
})

