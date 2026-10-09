import { expect, test, type Locator } from '@playwright/test'

test('WASM calendar boundaries, duplicate periods and bounded previews', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Numerical bridge contract runs once')
  await page.goto('/app')
  const reports = await page.evaluate(async () => {
    const { inspectCalendar } = await import(new URL('/src/data/calendar.ts', location.href).href)
    const inspect = (schedule: string, dates: string[]) => inspectCalendar(schedule, [{ name: 'Series', times: dates.map((date) => Date.parse(`${date}T00:00:00Z`)) }])
    return {
      month: await inspect('month-end', ['2024-01-31', '2024-03-31']),
      duplicate: await inspect('daily', ['2024-01-01', '2024-01-01']),
      offGrid: await inspect('month-start', ['2024-01-02', '2024-03-01']),
      empty: await inspect('daily', []),
      bounded: await inspectCalendar('daily', [{ name: 'Series', times: Array.from({ length: 202 }, (_, index) => Date.UTC(2024, 0, 1 + 2 * index)) }]),
    }
  })
  expect(reports.month.gaps).toEqual([{ unit: 'Series', first: '2024-02-29', last: '2024-02-29', count: 1 }])
  expect(reports.duplicate.issues[0].detail).toContain('More than one observation')
  expect(reports.offGrid.issues[0].detail).toContain('does not match')
  expect(reports.empty.issues[0].detail).toBe('No observations.')
  expect(reports.bounded.missing).toBe(201)
  expect(reports.bounded.ranges).toBe(201)
  expect(reports.bounded.gaps).toHaveLength(200)
})

async function choose(trigger: Locator, option: string) {
  await trigger.click()
  await trigger.page().getByRole('option', { name: option, exact: true }).click()
}

for (const scenario of [
  { frequency: 'Quarterly', alignment: 'Quarter start (Jan, Apr, Jul, Oct)', dates: ['2023-10-01', '2024-07-01'], missing: '2024-01-01 to 2024-04-01' },
  { frequency: 'Quarterly', alignment: 'Quarter end (Mar, Jun, Sep, Dec)', dates: ['2023-12-31', '2024-09-30'], missing: '2024-03-31 to 2024-06-30' },
  { frequency: 'Yearly', alignment: 'Year start (1 January)', dates: ['2023-01-01', '2026-01-01'], missing: '2024-01-01 to 2025-01-01' },
  { frequency: 'Yearly', alignment: 'Year end (31 December)', dates: ['2023-12-31', '2026-12-31'], missing: '2024-12-31 to 2025-12-31' },
]) {
  test(`reports ${scenario.alignment} gaps through the UI`, async ({ page }) => {
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill(scenario.alignment)
    await page.getByRole('button', { name: 'Create project' }).click()
    await page.locator('input[type="file"]').setInputFiles({ name: 'calendar.csv', mimeType: 'text/csv', buffer: Buffer.from(`date,x\n${scenario.dates.map((date, index) => `${date},${index}`).join('\n')}\n`) })
    await page.getByRole('button', { name: /Inspect data/ }).click()
    await page.getByRole('radio', { name: /Regular time series/ }).click()
    await choose(page.getByLabel('Time column'), 'date')
    await choose(page.getByLabel('Source frequency'), scenario.frequency)
    const report = page.getByTestId('calendar-report')
    await report.locator('summary').click()
    await choose(report.getByLabel('Calendar alignment'), scenario.alignment)
    await expect(report).toContainText('2 missing calendar periods across 1 gap.', { timeout: 60_000 })
    await expect(report.getByRole('cell', { name: scenario.missing, exact: true })).toBeVisible()
    await choose(page.getByLabel('Source frequency'), 'Daily')
    await expect(report.getByLabel('Calendar alignment')).toHaveCount(0)
    await expect(report).not.toContainText('2 missing calendar periods across 1 gap.')
  })
}

test('reports weekly panel gaps independently and retains the balance gate', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Calendar panel coverage')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles({ name: 'panel.csv', mimeType: 'text/csv', buffer: Buffer.from('unit,date,x\nA,2026-01-05,1\nA,2026-01-12,2\nA,2026-01-26,3\nB,2026-01-12,4\nB,2026-01-19,5\nB,2026-01-26,6\n') })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: 'Panel', exact: true }).click()
  await choose(page.getByLabel('Unit column'), 'unit')
  await choose(page.getByLabel('Time column'), 'date')
  await choose(page.getByLabel('Source frequency'), 'Weekly')
  const report = page.getByTestId('calendar-report')
  await report.locator('summary').click()
  await expect(report).toContainText('1 missing calendar period across 1 gap.', { timeout: 60_000 })
  await expect(report.getByRole('cell', { name: '2026-01-19', exact: true })).toHaveCount(1)
  await report.evaluate((element) => element.scrollIntoView({ block: 'center' }))
  await page.screenshot({ path: info.outputPath('calendar-panel.png'), fullPage: true })
  await choose(report.getByLabel('Calendar alignment'), 'Tuesday')
  await expect(report).toContainText('0 of 2 series checked.')
  await expect(report).toContainText('does not match the selected calendar alignment')
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('alert').filter({ hasText: /balanc/i })).toBeVisible({ timeout: 30_000 })
})

test('reports daily gaps without adding rows or modifying preparation', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Daily calendar coverage')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles({ name: 'daily.csv', mimeType: 'text/csv', buffer: Buffer.from('date,x\n2026-01-01,1\n2026-01-03,3\n') })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'date')
  await choose(page.getByLabel('Source frequency'), 'Daily')
  const report = page.getByTestId('calendar-report')
  await report.locator('summary').click()
  await expect(report).toContainText('1 missing calendar period across 1 gap.', { timeout: 60_000 })
  await expect(report.getByRole('cell', { name: '2026-01-02', exact: true })).toHaveCount(1)
  await report.evaluate((element) => element.scrollIntoView({ block: 'center' }))
  await page.screenshot({ path: info.outputPath('calendar-daily.png'), fullPage: true })
  const preview = page.locator('details').filter({ has: page.locator('summary').filter({ hasText: 'Time preview' }) })
  await preview.locator('summary').click()
  await expect(preview.getByRole('cell', { name: '2026-01-02', exact: true })).toHaveCount(0)
  await expect(preview.getByRole('cell', { name: '2026-01-01', exact: true })).toHaveCount(1)
})

test('reports the real Parquet ISO-week gap through upload controls', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Richdata calendar coverage')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles('docs/2026-09-17-richdata01/data/105w_tcsp_gapped.parquet')
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'year_week')
  await choose(page.getByLabel('Time interpretation'), 'ISO week (2024-W02)')
  await choose(page.getByLabel('Source frequency'), 'Weekly')
  const report = page.getByTestId('calendar-report')
  await report.locator('summary').click()
  await expect(report).toContainText('2 missing calendar periods across 1 gap.', { timeout: 60_000 })
  await expect(report).toContainText('2024-12-23 to 2024-12-30')
  await report.evaluate((element) => element.scrollIntoView({ block: 'center' }))
  await page.screenshot({ path: info.outputPath('calendar-richdata.png'), fullPage: true })
})
