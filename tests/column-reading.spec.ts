import { expect, test } from '@playwright/test'
import { fileRelation } from '../src/data/fileRelation'
import { columnReading, declareColumn, DECLARED_TYPE_NAMES, fileReadingOf, NO_DECLARATIONS, type ColumnDeclarations, type DeclaredType, type FileReading } from '../src/domain/fileReading'

/**
 * A delimited file's columns read as DuckDB detects them unless a type is declared. The laws are the
 * ones LAWS.bend states over bend/column_reading.bend; here they are checked on the TypeScript.
 */

const YES_NO = 'id,death,dnr1,age\n1,Yes,No,70.25\n2,No,No,78.18\n3,Yes,Yes,46.1\n4,No,Yes,68.4\n'

// A seeded generator, so a failing draw can be replayed.
const draws = (seed: number) => {
  let state = seed >>> 0
  return (below: number) => { state = (Math.imul(state, 1664525) + 1013904223) >>> 0; return state % below }
}
const COLUMNS = ['a', 'b', 'c', 'death', 'dnr1']
const randomDeclarations = (next: (below: number) => number): ColumnDeclarations => {
  let declared: ColumnDeclarations = NO_DECLARATIONS
  for (let step = next(6); step > 0; step -= 1) declared = declareColumn(declared, COLUMNS[next(COLUMNS.length)]!, DECLARED_TYPE_NAMES[next(DECLARED_TYPE_NAMES.length)]!)
  return declared
}
const delimited = (declared: ColumnDeclarations): FileReading => ({ format: 'csv', declared })

test('with nothing declared the reader writes the SQL it always wrote', () => {
  expect(fileRelation(delimited(NO_DECLARATIONS), 'p.csv')).toBe(`read_csv_auto('p.csv', header = true, sample_size = 20480)`)
  expect(fileRelation(delimited(NO_DECLARATIONS), 'p.csv', 'when')).toBe(`read_csv_auto('p.csv', header = true, sample_size = 20480, types = {'when': 'VARCHAR'})`)
  expect(fileRelation({ format: 'parquet' }, 'p.parquet')).toBe(`read_parquet('p.parquet')`)
})

test('declared columns become DuckDB types in name order, and a parsed time column stays text', () => {
  const declared = declareColumn(declareColumn(NO_DECLARATIONS, 'death', 'text'), "o'clock", 'time')
  expect(fileRelation(delimited(declared), 'p.csv')).toBe(`read_csv_auto('p.csv', header = true, sample_size = 20480, types = {'death': 'VARCHAR', 'o''clock': 'TIME'})`)
  expect(fileRelation(delimited(declared), 'p.csv', "o'clock")).toBe(`read_csv_auto('p.csv', header = true, sample_size = 20480, types = {'death': 'VARCHAR', 'o''clock': 'VARCHAR'})`)
})

test('the column-reading laws hold on 500 random declarations', () => {
  const next = draws(20260927)
  for (let draw = 0; draw < 500; draw += 1) {
    const declared = randomDeclarations(next)
    const column = COLUMNS[next(COLUMNS.length)]!
    const other = COLUMNS.find((name) => name !== column)!
    const type: DeclaredType = DECLARED_TYPE_NAMES[next(DECLARED_TYPE_NAMES.length)]!
    // Nothing declared: every column is detected.
    expect(columnReading(delimited(NO_DECLARATIONS), column)).toEqual({ kind: 'detected' })
    // A declared column reads as declared.
    expect(columnReading(delimited(declareColumn(declared, column, type)), column)).toEqual({ kind: 'declared', type })
    // A declaration leaves every other column as it was.
    expect(columnReading(delimited(declareColumn(declared, column, type)), other)).toEqual(columnReading(delimited(declared), other))
    expect(columnReading(delimited(declareColumn(declared, column, null)), other)).toEqual(columnReading(delimited(declared), other))
    // Returning a column to detection undoes its declaration.
    expect(columnReading(delimited(declareColumn(declareColumn(declared, column, type), column, null)), column)).toEqual({ kind: 'detected' })
    // Declaring the same type twice is declaring it once.
    expect(declareColumn(declareColumn(declared, column, type), column, type)).toEqual(declareColumn(declared, column, type))
  }
  // A Parquet file stores its types: nothing is declared on it, and a record that says otherwise is refused.
  expect(columnReading({ format: 'parquet' }, 'death')).toEqual({ kind: 'detected' })
  expect(fileReadingOf('parquet', { death: 'text' })).toEqual({ ok: false, error: { kind: 'declarations-on-parquet' } })
  expect(fileReadingOf('csv', undefined)).toEqual({ ok: true, value: { format: 'csv', declared: NO_DECLARATIONS } })
})

test('the worker profiles the file with its declarations, and later reads follow the profile', async ({ page }) => {
  await page.goto('/app')
  const outcome = await page.evaluate(async (text) => {
    const { newImportRequestId } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const { profileSourceInWorker, previewWindowInWorker } = await import(new URL('/src/data/client.ts', location.href).href)
    const { datasetProfileId } = await import(new URL('/src/domain/dataset.ts', location.href).href)
    const file = new File([text], 'patients.csv', { type: 'text/csv' })
    const detected = await profileSourceInWorker(newImportRequestId(), file)
    const declared = await profileSourceInWorker(newImportRequestId(), file, { death: 'text', dnr1: 'text' })
    if (!detected.ok || !declared.ok) throw new Error(JSON.stringify([detected, declared]))
    const types = (profile: typeof detected.value) => Object.fromEntries(profile.columns.map((column: { name: string; duckdbType: string }) => [column.name, column.duckdbType]))
    const window = await previewWindowInWorker(file, declared.value, { offset: 0, limit: 2, sort: null, filters: [], search: '' })
    return {
      detected: types(detected.value),
      declared: types(declared.value),
      source: declared.value.source.declared,
      sameIdWhenNoneDeclared: detected.value.id === datasetProfileId(detected.value.source.fingerprint, { format: 'csv', declared: {} }),
      distinctIds: detected.value.id !== declared.value.id,
      firstDeath: window.ok ? window.value.rows[0]?.cells[1] : JSON.stringify(window),
    }
  }, YES_NO)
  expect(outcome.detected).toEqual({ id: 'BIGINT', death: 'BOOLEAN', dnr1: 'BOOLEAN', age: 'DOUBLE' })
  expect(outcome.declared).toEqual({ id: 'BIGINT', death: 'VARCHAR', dnr1: 'VARCHAR', age: 'DOUBLE' })
  expect(outcome.source).toEqual({ death: 'text', dnr1: 'text' })
  expect(outcome.sameIdWhenNoneDeclared).toBe(true)
  expect(outcome.distinctIds).toBe(true)
  expect(outcome.firstDeath).toEqual({ kind: 'text', value: 'Yes' })
})

test('a saved project keeps its declarations, and one saved before them reads as declaring none', async ({ page }) => {
  await page.goto('/app')
  const outcome = await page.evaluate(async (text) => {
    const { createWorkflowStore } = await import(new URL('/src/domain/workflowStore.ts', location.href).href)
    const { newImportRequestId } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const { profileSourceInWorker } = await import(new URL('/src/data/client.ts', location.href).href)
    const { snapshotWorkflow, serialiseSnapshot, parseSnapshot } = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const store = createWorkflowStore()
    const dispatch = store.getState().dispatch
    dispatch({ type: 'project-name-changed', value: 'Declared reading' })
    dispatch({ type: 'project-submitted' })
    const file = new File([text], 'patients.csv', { type: 'text/csv' })
    dispatch({ type: 'file-selected', file })
    const request = newImportRequestId()
    dispatch({ type: 'profile-requested', request })
    const profile = await profileSourceInWorker(request, file, { death: 'text' })
    if (!profile.ok) throw new Error(JSON.stringify(profile.error))
    dispatch({ type: 'profile-succeeded', request, profile: profile.value })
    const saved = serialiseSnapshot(snapshotWorkflow(store.getState().workflow, '2026-09-27T00:00:00Z'))
    const parsed = parseSnapshot(saved)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const restored = createWorkflowStore()
    restored.getState().dispatch({ type: 'project-reopened', snapshot: parsed.value })
    restored.getState().dispatch({ type: 'project-restored', file })
    const workflow = restored.getState().workflow
    // A record written before declarations existed: no `declared` field, and the id without a suffix.
    const legacy = JSON.parse(serialiseSnapshot(snapshotWorkflow(createWorkflowStore().getState().workflow, '2026-09-27T00:00:00Z')) ?? 'null')
    const older = JSON.parse(saved)
    delete older.profile.source.declared
    older.profile.id = older.profile.id.split(':').slice(0, 3).join(':')
    const reread = parseSnapshot(JSON.stringify(older))
    return {
      restored: workflow.kind === 'profiled' ? { declared: workflow.source.declared, profile: workflow.profile.source.declared } : workflow.kind,
      older: reread.ok ? reread.value.profile?.source.declared : JSON.stringify(reread.error),
      legacy: legacy === null,
    }
  }, YES_NO)
  expect(outcome.restored).toEqual({ declared: { death: 'text' }, profile: { death: 'text' } })
  expect(outcome.older).toEqual({})
})

test('Read as in the Data studio profiles the file again, and asks first when prepared data would be cleared', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the control; the phone layout shares it.')
  test.setTimeout(120_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Read as')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'patients.csv', mimeType: 'text/csv', buffer: Buffer.from(YES_NO) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  const row = (name: string) => page.getByRole('row').filter({ has: page.getByRole('combobox', { name: `Read ${name} as`, exact: true }) })
  const death = page.getByRole('combobox', { name: 'Read death as', exact: true })
  await expect(row('death')).toContainText('BOOLEAN')
  await expect(row('death')).not.toContainText('declared')
  await death.click()
  await expect(page.getByRole('option', { name: 'Detected, BOOLEAN', exact: true })).toHaveAttribute('aria-selected', 'true')
  await page.keyboard.press('Escape')
  await death.click()
  await page.getByRole('option', { name: 'Text', exact: true }).click()
  await expect(row('death')).toContainText('VARCHARdeclared')
  await expect(row('dnr1')).toContainText('BOOLEAN')
  await expect(row('dnr1')).not.toContainText('declared')
  await page.getByRole('radio', { name: /^Independent observations/ }).check()
  await page.getByRole('checkbox', { name: 'age', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  const prepared = page.getByRole('status').filter({ hasText: /^Cross-section, / })
  await expect(prepared).toBeVisible()
  await page.getByRole('combobox', { name: 'Read death as', exact: true }).click()
  await page.getByRole('option', { name: /^Detected/ }).click()
  const confirm = page.getByRole('alertdialog', { name: 'Read the file again?' })
  await expect(confirm).toContainText('Reading death as detected profiles the file again and clears the prepared dataset')
  await confirm.getByRole('button', { name: 'Cancel' }).click()
  await expect(row('death')).toContainText('VARCHARdeclared')
  await expect(prepared).toBeVisible()
  await page.getByRole('combobox', { name: 'Read death as', exact: true }).click()
  await page.getByRole('option', { name: /^Detected/ }).click()
  await page.getByRole('alertdialog', { name: 'Read the file again?' }).getByRole('button', { name: 'Read again' }).click()
  await expect(row('death')).toContainText('BOOLEAN')
  await expect(row('death')).not.toContainText('declared')
  await expect(prepared).toHaveCount(0)
  await page.screenshot({ path: info.outputPath('read-as.png'), fullPage: true })
})

test('a pipeline input reads a declared column as its type, and the recipe replays it', async ({ page }) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  const outcome = await page.evaluate(async (text) => {
    const { openPipeline, redeclarePipelineInput, runPipeline, materializePipeline, replayPipelineRecipe, closePipeline } = await import(new URL('/src/data/pipeline.ts', location.href).href)
    const { prepareSqlInputs } = await import(new URL('/src/data/sqlPreparation.ts', location.href).href)
    const { fingerprintFile } = await import(new URL('/src/data/fingerprint.ts', location.href).href)
    const scripts = { run: async () => { throw new Error('No script block here.') } }
    const file = new File([text], 'patients.csv', { type: 'text/csv' })
    const inputs = await prepareSqlInputs([file])
    if (!inputs.ok) throw new Error(JSON.stringify(inputs.error))
    const session = await openPipeline(inputs.value, scripts)
    if (!session.ok) throw new Error(JSON.stringify(session.error))
    const alias = inputs.value[0].alias
    const graph = {
      nodes: [
        { id: 'in', position: { x: 0, y: 0 }, block: { kind: 'input', file: { kind: 'chosen', alias } } },
        { id: 'out', position: { x: 0, y: 1 }, block: { kind: 'output' } },
      ],
      edges: [{ from: 'in', to: 'out', port: 0 }],
    }
    const typeOf = async () => {
      const run = await runPipeline(session.value, graph)
      if (!run.ok) throw new Error(JSON.stringify(run.error))
      const outcome = run.value.outcomes.get('in')
      return outcome?.kind === 'ran' ? outcome.columns.find((column: { name: string }) => column.name === 'death')?.type : JSON.stringify(outcome)
    }
    const before = await typeOf()
    const declared = await redeclarePipelineInput(session.value, alias, 'death', 'text')
    const after = await typeOf()
    const unknown = await redeclarePipelineInput(session.value, 'nothing', 'death', 'text')
    const output = await materializePipeline(session.value, graph)
    if (!output.ok) throw new Error(JSON.stringify(output.error))
    await closePipeline(session.value)
    const closed = await redeclarePipelineInput(session.value, alias, 'death', 'text')
    const replayed = await replayPipelineRecipe(output.value.recipe, [file], scripts)
    const [original, again] = await Promise.all([fingerprintFile(output.value.file), replayed.ok ? fingerprintFile(replayed.value) : null])
    return {
      before, after,
      recorded: output.value.recipe.inputs[0].declared,
      declaredOk: declared.ok,
      replaysIdentically: original?.ok === true && again?.ok === true && original.value === again.value,
      unknown: unknown.ok ? 'ok' : unknown.error.kind,
      closed: closed.ok ? 'ok' : closed.error.kind,
    }
  }, YES_NO)
  expect(outcome.before).toBe('BOOLEAN')
  expect(outcome.after).toBe('VARCHAR')
  expect(outcome.declaredOk).toBe(true)
  expect(outcome.recorded).toEqual({ death: 'text' })
  expect(outcome.replaysIdentically).toBe(true)
  expect(outcome.unknown).toBe('unknown-input')
  expect(outcome.closed).toBe('session-closed')
})

test('declaring on the pipeline input card rereads the file: the card and its preview show the new type', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the card; the phone layout shares it.')
  test.setTimeout(120_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Read as on a pipeline input')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click()
  await page.getByRole('button', { name: 'Open the editor' }).click()
  await page.getByTestId('pipeline-canvas').waitFor({ timeout: 90_000 })
  const card = page.locator('[data-testid^="block-input-"]').last()
  if (await card.count() === 0) await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name: /^Input/ }).click()
  await page.locator('[data-testid^="block-input-"]').last().locator('p').first().click()
  await page.getByLabel('File for this card').setInputFiles({ name: 'patients.csv', mimeType: 'text/csv', buffer: Buffer.from(YES_NO) })
  await page.locator('[data-testid^="block-input-"]').last().getByText(/4 rows/).waitFor({ timeout: 60_000 })
  await page.locator('summary').filter({ hasText: 'Columns' }).first().click()
  const row = (name: string) => page.getByTestId('block-schema').locator('li').filter({ has: page.getByRole('combobox', { name: `Read ${name} as`, exact: true }) })
  // Two declarations back to back, as a reader setting several columns makes them.
  for (const name of ['death', 'dnr1']) {
    await expect(row(name)).toContainText('BOOLEAN')
    await page.getByRole('combobox', { name: `Read ${name} as`, exact: true }).click()
    await page.getByRole('option', { name: 'Text', exact: true }).click()
    await row(name).getByText('declared').waitFor()
  }
  for (const name of ['death', 'dnr1']) await expect(row(name)).toContainText('VARCHARdeclared', { timeout: 30_000 })
  await expect(page.getByRole('columnheader', { name: /death/ }).first()).toContainText('VARCHAR')
})
