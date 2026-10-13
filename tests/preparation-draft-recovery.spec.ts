import { test, expect } from '@playwright/test'
import { showSqlSource, hideSqlSource } from './sql-pane'
import { ask, connectAssistant } from './assistantTestFlow'

test('an unapplied assistant proposal survives reload without recovering approval', async ({
  page,
}) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Recover proposal')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
  await page.getByRole('button', { name: 'Open empty SQL editor' }).click()
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Open SQL file' })).toBeEnabled({ timeout: 30000 })
  await hideSqlSource(page)
  await connectAssistant(page, [
    'p = await tool("project")\nr = await tool("propose_sql", {"statement":"CREATE VIEW hirmos_prepared AS SELECT 77 AS value", "summary":"Prepare one value.", "checks":["Check the value is 77."]})\nassert r["kind"] == "awaiting-user-review", r\nawait FINAL_VAR("SQL proposed, not run.", ["project-summary"])',
  ])
  await ask(page, 'Propose SQL without running it.')
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'SQL proposed',
    { timeout: 30000 },
  )
  await expect(page.getByRole('button', { name: 'Load SQL for review', exact: true })).toBeVisible()
  await page.goto('/app/projects')
  await page.reload()
  await page.getByRole('button', { name: 'Open Recover proposal', exact: true }).click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  const saved = page.getByRole('region', { name: 'Saved proposal', exact: true })
  await expect(saved).toBeVisible()
  await saved.getByText('Saved draft contents', { exact: true }).click()
  await expect(saved).toContainText('SELECT 77 AS value')
  await expect(page.getByRole('button', { name: 'Load SQL for review', exact: true })).toHaveCount(
    0,
  )
})

test('unfinished SQL survives reload without executing or adopting it', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Recover unfinished SQL')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  const open = async () => {
    await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
    await page.getByRole('button', { name: 'Open empty SQL editor' }).click()
    await showSqlSource(page)
    await expect(page.getByRole('button', { name: 'Open SQL file' })).toBeEnabled({
      timeout: 30000,
    })
    await hideSqlSource(page)
  }
  await open()
  const text = 'CREATE VIEW must_not_run AS SELECT 42 AS value;'
  await page.getByLabel('SQL script file').setInputFiles({
    name: 'unfinished.sql',
    mimeType: 'application/sql',
    buffer: Buffer.from(text),
  })
  await expect
    .poll(async () =>
      page.evaluate(async () => {
        const { listProjects } = await import(
          new URL('/src/data/projectStore.ts', location.href).href
        )
        const { loadPreparationDraft } = await import(
          new URL('/src/data/preparationDraftStore.ts', location.href).href
        )
        const project = (await listProjects()).find(
          (p: { name: string }) => p.name === 'Recover unfinished SQL',
        )
        return project ? (await loadPreparationDraft(project.id, 'sql', '[]'))?.text : null
      }),
    )
    .toBe(text)
  await page.goto('/app/projects')
  await page.reload()
  await page.getByRole('button', { name: 'Open Recover unfinished SQL', exact: true }).click()
  await open()
  await page.getByRole('button', { name: 'Restore draft', exact: true }).click()
  await expect(page.getByLabel('SQL file contents')).toHaveValue(text)
  await page.getByRole('button', { name: 'Close sql file' }).click()
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled({ timeout: 30000 })
  await page.getByRole('button', { name: 'Refresh views' }).click()
  await expect(page.getByRole('combobox', { name: 'Output view' })).toHaveCount(0)
})

test('draft storage isolates projects and restores pipelines without running blocks', async ({
  page,
}) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const { savePreparationDraft } = await import(
      new URL('/src/data/preparationDraftStore.ts', location.href).href
    )
    const { initialGraph } = await import(
      new URL('/src/components/data/pipeline/pipelineWorkspaceModel.ts', location.href).href
    )
    await savePreparationDraft('draft-a', 'pipeline', 'null', {
      kind: 'pipeline',
      graph: initialGraph([]),
      inputs: [],
    })
    await savePreparationDraft('draft-b', 'sql', '[]', {
      kind: 'sql',
      name: 'other.sql',
      text: 'SELECT 2',
      inputs: [
        {
          alias: 'sample',
          fileName: 'sample.csv',
          bytes: 4,
          fingerprint: 'a'.repeat(64),
          format: 'csv',
          file: new File(['x\n1\n'], 'sample.csv'),
        },
      ],
    })
  })
  await page.reload()
  const result = await page.evaluate(async () => {
    const { loadPreparationDraft, deletePreparationDrafts } = await import(
      new URL('/src/data/preparationDraftStore.ts', location.href).href
    )
    const { createPipelineSession } = await import(
      new URL('/src/components/data/pipeline/pipelineSession.ts', location.href).href
    )
    const draft = await loadPreparationDraft('draft-a', 'pipeline', 'null')
    const controller = createPipelineSession(null)
    controller.restoreDraft(draft)
    const state = controller.store.getState()
    controller.dispose()
    await deletePreparationDrafts('draft-a')
    return {
      run: state.run.kind,
      graph: state.graph,
      expected: draft.graph,
      removed: await loadPreparationDraft('draft-a', 'pipeline', 'null'),
      other: (await loadPreparationDraft('draft-b', 'sql', '[]'))?.text,
      file: await (await loadPreparationDraft('draft-b', 'sql', '[]'))?.inputs[0].file.text(),
      wrong: await loadPreparationDraft('draft-b', 'pipeline', 'null'),
    }
  })
  expect(result.run).toBe('restored')
  expect(result.graph).toEqual(result.expected)
  expect(result.removed).toBeNull()
  expect(result.wrong).toBeNull()
  expect(result.other).toBe('SELECT 2')
  expect(result.file).toBe('x\n1\n')
})
