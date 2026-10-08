import { expect, test } from '@playwright/test'
import { readFileSync, readdirSync } from 'node:fs'
import { parseSnapshot } from '../src/domain/persistence'
import { createWorkflowStore } from '../src/domain/workflowStore'
import { readyPreprocessingRecipe } from '../src/domain/preprocessing'
import { preparedDraft, retainPreprocessing } from '../src/domain/preprocessingSession'
import { brand } from '../src/domain/dop'

test('all shipped preparations restore through the store without overwriting later edits', () => {
  for (const name of readdirSync('public/examples').filter(name => name.endsWith('.hirmos.json'))) {
    const parsed = parseSnapshot(JSON.stringify(JSON.parse(readFileSync(`public/examples/${name}`, 'utf8')).project))
    if (!parsed.ok) throw new Error(`${name}: ${JSON.stringify(parsed.error)}`)
    const snapshot = parsed.value
    if (snapshot.prepared === null || snapshot.profile === null) continue
    const store = createWorkflowStore()
    store.getState().dispatch({ type: 'project-reopened', snapshot })
    expect(store.getState().preprocessing).toBeNull()
    store.getState().dispatch({ type: 'project-restored', file: new File(['fixture'], 'fixture.csv', { type: 'text/csv' }) })
    const restored = store.getState()
    const session = restored.preprocessing!
    expect(session.draft.sampling, name).toEqual(snapshot.prepared.sampling)
    expect(session.draft.variables).toEqual({ kind: 'selected', columns: snapshot.prepared.columns })
    expect(session.draft.missingness).toEqual(snapshot.prepared.missingness)
    const recipe = readyPreprocessingRecipe(session.draft)
    expect(recipe.ok, name).toBe(true)
    if (!recipe.ok) throw new Error(name)
    expect(session.savedRecipe).toBe(JSON.stringify(recipe.value))
    expect(createWorkflowStore(restored.workflow).getState().preprocessing).toEqual(session)
    store.getState().changePreprocessing(snapshot.profile.id, { type: 'variable-toggled', column: snapshot.prepared.columns[0] })
    const edited = store.getState().preprocessing
    store.getState().dispatch({ type: 'study-draft-changed', draft: snapshot.studyDraft })
    expect(store.getState().preprocessing).toBe(edited)
    expect(edited).not.toBe(session)
    if (restored.workflow.kind !== 'profiled') throw new Error(name)
    const fresh = retainPreprocessing(null, { ...restored.workflow, prepared: null })!
    expect(fresh.draft.sampling.kind).toBe('unconfigured')
    expect(fresh.savedRecipe).toBeNull()
    const foreign = retainPreprocessing(null, { ...restored.workflow, prepared: { ...snapshot.prepared, sourceProfile: brand<string, 'DatasetProfileId'>('other') } })!
    expect(foreign.draft.sampling.kind).toBe('unconfigured')
    store.getState().dispatch({ type: 'project-closed' })
    expect(store.getState().preprocessing).toBeNull()
  }
})

test('restoration retains source frequency, aggregation, STL and transformations', () => {
  const parsed = parseSnapshot(JSON.stringify(JSON.parse(readFileSync('public/examples/seatbelts.hirmos.json', 'utf8')).project))
  if (!parsed.ok || parsed.value.prepared?.kind !== 'prepared-time-series') throw new Error('Missing time-series fixture')
  const prepared = parsed.value.prepared
  const columns = prepared.columns
  const draft = preparedDraft({
    ...prepared,
    sampling: { ...prepared.sampling, frequency: 'daily', interpretation: { kind: 'iso-week' } },
    resampling: { kind: 'daily-downsample', sourceFrequency: 'daily', targetFrequency: 'monthly', weekStartsOn: 'monday', calendar: 'utc', incompleteBins: 'drop', aggregations: [ { column: columns[0], aggregation: 'sum' }, ...columns.slice(1).map(column => ({ column, aggregation: 'mean' as const })) ], sourceRows: 1000, outputRows: 30, incompleteBinsFound: 2, binsDropped: 2, sourceRowsDropped: 10 },
    seasonalAdjustment: { kind: 'stl', columns, robust: true, period: 12 },
    seriesTransforms: [{ column: columns[0], transform: { kind: 'difference', order: 1 } }, ...columns.slice(1).map(column => ({ column, transform: { kind: 'linear-detrend' as const } }))],
  })
  const recipe = readyPreprocessingRecipe(draft)
  expect(recipe.ok).toBe(true)
  expect(draft.sampling).toMatchObject({ frequency: 'daily', interpretation: { kind: 'iso-week' } })
  expect(draft.resampling).not.toHaveProperty('outputRows')
  expect(draft.seasonal).toEqual({ kind: 'stl', columns, robust: true })
  expect(draft.seriesTransforms[0].transform.kind).toBe('difference')
})

test('real weekly upload restores controls after reload and rejects different contents', async ({ page }, info) => {
  test.setTimeout(120_000)
  const file = 'docs/2026-09-17-richdata01/data/105w_tcsp_weekly.parquet'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Restore weekly preparation')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  for (const [label, option] of [['Time column', 'year_week'], ['Time interpretation', 'ISO week (2024-W02)'], ['Source frequency', 'Weekly']]) {
    await page.getByRole('combobox', { name: label }).click()
    await page.getByRole('option', { name: option, exact: true }).click()
  }
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  await expect(page.getByTestId('prepared-series').first()).toBeVisible({ timeout: 30_000 })
  await expect.poll(() => page.evaluate(async () => {
    const { listProjects, loadProject } = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await listProjects()).find((item: { name: string }) => item.name === 'Restore weekly preparation')
    if (header === undefined) return false
    const saved = await loadProject(header.id)
    return saved.ok && saved.value.prepared !== null
  })).toBe(true)
  await page.goto('/app/projects')
  await page.getByRole('list', { name: 'Projects' }).getByRole('button', { name: 'Open Restore weekly preparation', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles({ name: '105w_tcsp_weekly.parquet', mimeType: 'application/octet-stream', buffer: Buffer.from('different contents') })
  await expect(page.getByText('This is not the file the project was built from.', { exact: false })).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(file)
  await expect(page.getByRole('radio', { name: /^Regular time series/ })).toBeChecked()
  await expect(page.getByRole('combobox', { name: 'Time column' })).toContainText('year_week')
  await expect(page.getByRole('combobox', { name: 'Time interpretation' })).toContainText('ISO week')
  await expect(page.getByRole('combobox', { name: 'Source frequency' })).toContainText('Weekly')
  await expect(page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true })).toBeChecked()
  await expect(page.getByText('Choose a time series, a panel, or independent observations.', { exact: true })).toHaveCount(0)
  await expect(page.getByText('These preparation settings are already saved.', { exact: true })).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Create prepared dataset version', exact: true })).toBeDisabled()
  await expect(page.getByTestId('prepared-series').first()).toBeVisible({ timeout: 30_000 })
  await page.getByRole('combobox', { name: 'Time column' }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('restored-preparation.png'), fullPage: true })
})
