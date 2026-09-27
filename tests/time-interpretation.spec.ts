import { expect, test } from '@playwright/test'
import { choose } from './examples/support'
import { parseDataWorkerEvent } from '../src/workers/dataProtocol'

for (const timezoneId of ['UTC', 'Europe/Amsterdam', 'America/New_York']) {
  test(`calendar parsing is independent of browser timezone: ${timezoneId}`, async ({ browser }) => {
    const context = await browser.newContext({ timezoneId })
    try {
      const page = await context.newPage()
      await page.goto('/app')
      const dates = await page.evaluate(async () => {
        const data = await import(new URL('/src/data/client.ts', location.href).href)
        const workflow = await import(new URL('/src/domain/workflow.ts', location.href).href)
        async function read(value: string) {
          const file = new File([`date,x\n${value},1\n`], 'date.csv', { type: 'text/csv' })
          const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
          if (!profile.ok) throw new Error(JSON.stringify(profile.error))
          const date = profile.value.columns.find((c: { name: string }) => c.name === 'date')
          const x = profile.value.columns.find((c: { name: string }) => c.name === 'x')
          const matrix = await data.materializeTimeSeriesColumnsInWorker(file, profile.value, date.id, [x.id])
          if (!matrix.ok) throw new Error(JSON.stringify(matrix.error))
          return new Date(matrix.value.timeAxis.timestamps[0]).toISOString()
        }
        return Promise.all([
          read('2026-01-05'),
          read('2026-01-05T00:00:00'),
          read('2026-01-05T00:00:00+02:00'),
          read('2024-03-31 00:30:00 Europe/Amsterdam'),
          read('2024-03-31 03:30:00 Europe/Amsterdam'),
        ])
      })
      expect(dates).toEqual([
        '2026-01-05T00:00:00.000Z',
        '2026-01-05T00:00:00.000Z',
        '2026-01-04T22:00:00.000Z',
        '2024-03-30T23:30:00.000Z',
        '2024-03-31T01:30:00.000Z',
      ])
    } finally {
      await context.close()
    }
  })
}

test('time preview protocol rejects unknown variants and malformed results', () => {
  const request = '8c2b1a19-9b68-45b3-8f47-e31450850436'
  expect(parseDataWorkerEvent({ kind: 'unknown', request }).ok).toBe(false)
  expect(parseDataWorkerEvent({ kind: 'time-preview-succeeded', request, preview: { kind: 'calendar', rows: [{ original: 'bad', parsed: NaN }], spacing: { kind: 'single-period' } } }).ok).toBe(false)
  expect(parseDataWorkerEvent({ kind: 'time-preview-succeeded', request, preview: { kind: 'calendar', rows: [{ original: 'bad', parsed: null }], spacing: { kind: 'single-period' } } }).ok).toBe(true)
})

test('direct upload previews and prepares ISO weeks through existing controls', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Weekly interpretation')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  const csv = ['week,value', ...Array.from({ length: 16 }, (_, i) => `2024-W${String(i + 1).padStart(2, '0')},${i + 1}`)].join('\n')
  await page.locator('input[type=file]').setInputFiles({ name: 'weeks.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  await choose(page, 'Time column', 'week')
  await choose(page, 'Time interpretation', 'ISO week (2024-W02)')
  const help = page.getByRole('button', { name: 'About Time interpretation', exact: true })
  if (info.project.name === 'mobile-chromium') await help.click()
  else await help.focus()
  await expect(page.getByText('Weeks start on Monday. The ISO week-year can differ from the calendar year.', { exact: true }).first()).toBeVisible()
  await page.keyboard.press('Escape')
  await choose(page, 'Source frequency', 'Weekly')
  await page.locator('summary').filter({ hasText: 'Time preview' }).click()
  const preview = page.getByRole('table', { name: 'Time preview', exact: true })
  await expect(preview).toContainText('2024-01-08 00:00:00')
  await preview.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('time-preview.png') })
  await page.getByRole('checkbox', { name: 'value', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Column profile', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Prepared data', exact: true })).toContainText('16 rows')
})

test('DuckDB parses explicit time formats without guessing or rolling invalid weeks forward', async ({ page }) => {
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const data = await import(new URL('/src/data/client.ts', location.href).href)
    const workflow = await import(new URL('/src/domain/workflow.ts', location.href).href)
    async function read(values: string[], interpretation: object) {
      const file = new File(['time,x\n' + values.map((value, i) => `${value},${i + 1}`).join('\n')], 'times.csv', { type: 'text/csv' })
      const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
      if (!profile.ok) throw new Error(JSON.stringify(profile.error))
      const time = profile.value.columns.find((column: { name: string }) => column.name === 'time')
      const x = profile.value.columns.find((column: { name: string }) => column.name === 'x')
      const result = await data.materializeTimeSeriesColumnsInWorker(file, profile.value, time.id, [x.id], interpretation)
      if (!result.ok) return result.error.kind
      return result.value.timeAxis.kind === 'calendar'
        ? Array.from(result.value.timeAxis.timestamps, (value) => new Date(Number(value)).toISOString())
        : Array.from(result.value.timeAxis.values)
    }
    return {
      weeks: await read(['2020-W53', '2021-W01', '2020-W01'], { kind: 'iso-week' }),
      invalidWeek: await read(['2021-W53'], { kind: 'iso-week' }),
      zeroWeek: await read(['2024-W00'], { kind: 'iso-week' }),
      repeated: await read(['2024-W02', '2024-W02'], { kind: 'iso-week' }),
      ordinal: await read(['2', '1'], { kind: 'ordinal' }),
      dayFirst: await read(['03/04/2024'], { kind: 'date-format', format: '%d/%m/%Y' }),
      monthFirst: await read(['03/04/2024'], { kind: 'date-format', format: '%m/%d/%Y' }),
      invalidDate: await read(['31/02/2024', '03/04/2024'], { kind: 'date-format', format: '%d/%m/%Y' }),
    }
  })
  expect(results).toEqual({
    weeks: ['2019-12-30T00:00:00.000Z', '2020-12-28T00:00:00.000Z', '2021-01-04T00:00:00.000Z'],
    invalidWeek: 'time-value-unparseable', zeroWeek: 'time-value-unparseable',
    repeated: 'duplicate-time-value', ordinal: [1, 2], invalidDate: 'time-value-unparseable',
    dayFirst: ['2024-04-03T00:00:00.000Z'], monthFirst: ['2024-03-04T00:00:00.000Z'],
  })
})
