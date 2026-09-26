import { expect, test, type Locator } from '@playwright/test'
import { stepPreprocessing, suggestFrequency, type PreprocessingDraft } from '../src/domain/preprocessing'
import type { ColumnId } from '../src/domain/dataset'

/**
 * The source frequency is read from the data: the most common gap between distinct times. A reading
 * fills the placeholder and replaces an earlier reading; it never replaces a frequency the reader chose.
 */

test('the modal gap between times names the frequency, allowing for month and year lengths', () => {
  expect(suggestFrequency({ kind: 'days', modal: 1 })).toEqual({ kind: 'suggested', frequency: 'daily' })
  expect(suggestFrequency({ kind: 'days', modal: 23 / 24 })).toEqual({ kind: 'suggested', frequency: 'daily' })
  expect(suggestFrequency({ kind: 'days', modal: 7 })).toEqual({ kind: 'suggested', frequency: 'weekly' })
  expect(suggestFrequency({ kind: 'days', modal: 28 })).toEqual({ kind: 'suggested', frequency: 'monthly' })
  expect(suggestFrequency({ kind: 'days', modal: 31 })).toEqual({ kind: 'suggested', frequency: 'monthly' })
  expect(suggestFrequency({ kind: 'days', modal: 91 })).toEqual({ kind: 'suggested', frequency: 'quarterly' })
  expect(suggestFrequency({ kind: 'days', modal: 366 })).toEqual({ kind: 'suggested', frequency: 'yearly' })
  expect(suggestFrequency({ kind: 'days', modal: 14 })).toEqual({ kind: 'unrecognised', days: 14 })
  expect(suggestFrequency({ kind: 'ordinal' })).toEqual({ kind: 'ordinal' })
  expect(suggestFrequency({ kind: 'single-period' })).toEqual({ kind: 'single-period' })
})

const draft: PreprocessingDraft = {
  sampling: { kind: 'unconfigured' }, frequencyOrigin: 'assumed', variables: { kind: 'empty' }, missingness: { kind: 'not-present' },
  resampling: { kind: 'none' }, seasonal: { kind: 'none' }, seriesTransforms: [], diagnosticTransform: { kind: 'levels' },
}
const date = 'date' as ColumnId
const city = 'city' as ColumnId
const frequencyOf = (state: PreprocessingDraft) => state.sampling.kind === 'regular-panel' ? state.sampling.frequency : null

test('a reading fills the placeholder but never replaces a chosen frequency', () => {
  let state = stepPreprocessing(draft, { type: 'regular-panel-selected' })
  state = stepPreprocessing(state, { type: 'unit-column-selected', unitColumn: city })
  state = stepPreprocessing(state, { type: 'time-column-selected', timeColumn: date })
  expect(state.frequencyOrigin).toBe('assumed')

  const read = stepPreprocessing(state, { type: 'time-spacing-observed', timeColumn: date, suggestion: { kind: 'suggested', frequency: 'daily' } })
  expect(frequencyOf(read)).toBe('daily')
  expect(read.frequencyOrigin).toBe('observed')

  const reread = stepPreprocessing(read, { type: 'time-spacing-observed', timeColumn: date, suggestion: { kind: 'suggested', frequency: 'weekly' } })
  expect(frequencyOf(reread)).toBe('weekly')

  const chosen = stepPreprocessing(read, { type: 'frequency-selected', frequency: 'monthly' })
  expect(chosen.frequencyOrigin).toBe('chosen')
  const late = stepPreprocessing(chosen, { type: 'time-spacing-observed', timeColumn: date, suggestion: { kind: 'suggested', frequency: 'daily' } })
  expect(frequencyOf(late)).toBe('monthly')

  // A reading for a column no longer selected, or one that names no frequency, changes nothing.
  const other = stepPreprocessing(read, { type: 'time-spacing-observed', timeColumn: city, suggestion: { kind: 'suggested', frequency: 'yearly' } })
  expect(frequencyOf(other)).toBe('daily')
  const ordinal = stepPreprocessing(read, { type: 'time-spacing-observed', timeColumn: date, suggestion: { kind: 'ordinal' } })
  expect(frequencyOf(ordinal)).toBe('daily')

  // Choosing another time column asks for a new reading.
  const switched = stepPreprocessing(chosen, { type: 'time-column-selected', timeColumn: city })
  expect(switched.frequencyOrigin).toBe('assumed')
})

const choose = async (trigger: Locator, option: string) => {
  await trigger.click()
  await trigger.page().getByRole('option', { name: option, exact: true }).click()
}

const csv = (header: string, rows: readonly string[]) => ({ name: 'spacing.csv', mimeType: 'text/csv', buffer: Buffer.from(`${header}\n${rows.join('\n')}\n`) })

test('a daily panel reads as daily and an integer clock has no frequency to set', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough for a prepare-step reading.')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Frequency read from dates')
  await page.getByRole('button', { name: 'Create project' }).click()
  const days = Array.from({ length: 10 }, (_, index) => `2021-05-${String(index + 1).padStart(2, '0')}`)
  await page.locator('input[type="file"]').setInputFiles(csv('date,city,y', days.flatMap((day) => ['1', '2'].map((unit) => `${day},${unit},${unit}`))))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await choose(page.getByLabel('Unit column'), 'city')
  await choose(page.getByLabel('Time column'), 'date')
  await expect(page.getByLabel('Source frequency')).toContainText('Daily', { timeout: 60_000 })

  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Frequency with an integer clock')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(csv('period,city,y', ['0', '1'].flatMap((period) => ['1', '2'].map((unit) => `${period},${unit},${unit}`))))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await choose(page.getByLabel('Unit column'), 'city')
  await choose(page.getByLabel('Time column'), 'period')
  await expect(page.getByLabel('Time column')).toContainText('period')
  await expect(page.getByLabel('Source frequency')).toHaveCount(0)
})

test('a weekly series reads as weekly', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough for a prepare-step reading.')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Frequency read from weeks')
  await page.getByRole('button', { name: 'Create project' }).click()
  const mondays = Array.from({ length: 8 }, (_, index) => new Date(Date.UTC(2024, 0, 1 + 7 * index)).toISOString().slice(0, 10))
  await page.locator('input[type="file"]').setInputFiles(csv('week,y', mondays.map((week, index) => `${week},${index}`)))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'week')
  await expect(page.getByLabel('Source frequency')).toContainText('Weekly', { timeout: 60_000 })
})
