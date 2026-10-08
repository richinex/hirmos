import { expect, test } from '@playwright/test'
import { showSqlSource, hideSqlSource } from './sql-pane'
import { compilePipeline, parsePipelineRecipe, pipelineBlockId, type PipelineGraph } from '../src/domain/pipeline'
import { parseSourceRecipe } from '../src/domain/sqlPreparation'

const id = pipelineBlockId
const graph: PipelineGraph = {
  nodes: [
    { id: id('script'), block: { kind: 'script', code: 'prepared = pd.DataFrame({"value": [1, 2, 3]})' }, position: { x: 0, y: 0 } },
    { id: id('output'), block: { kind: 'output' }, position: { x: 0, y: 200 } },
  ],
  edges: [{ from: id('script'), to: id('output'), port: 0 }],
}
const draftGraph: PipelineGraph = {
  ...graph,
  nodes: [...graph.nodes,
    { id: id('unused-input'), block: { kind: 'input', file: { kind: 'empty' } }, position: { x: 200, y: 0 } },
    { id: id('unused-script'), block: { kind: 'script', code: 'raise RuntimeError("Unconnected draft must not run during adoption")' }, position: { x: 400, y: 0 } },
  ],
}

test('empty dependencies are valid; a missing connected file is not', () => {
  expect(compilePipeline(graph, new Set()).ok).toBe(true)
  expect(parsePipelineRecipe({ kind: 'pipeline-derived', graph, inputs: [] }).ok).toBe(true)
  expect(parseSourceRecipe({ kind: 'sql-derived', outputView: 'generated', statement: 'CREATE VIEW generated AS SELECT 1 AS x;', inputs: [] }).ok).toBe(true)
  const missing: PipelineGraph = { nodes: [...graph.nodes, { id: id('input'), block: { kind: 'input', file: { kind: 'empty' } }, position: { x: 0, y: -100 } }], edges: [...graph.edges, { from: id('input'), to: id('script'), port: 0 }] }
  expect(compilePipeline(missing, new Set()).ok).toBe(false)
  const compiled = compilePipeline(draftGraph, new Set())
  expect(compiled.ok).toBe(true)
  if (compiled.ok) expect([...compiled.value.views.keys()]).toEqual(['script', 'output'])
  expect(parsePipelineRecipe({ kind: 'pipeline-derived', graph: draftGraph, inputs: [] }).ok).toBe(true)
})

test('pandas-created data materializes and replays with no files', async ({ page }) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  const result = await page.evaluate(async graph => {
    const pipeline = await import(new URL('/src/data/pipeline.ts', window.location.href).href)
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', window.location.href).href)
    const python = createPythonRuntime()
    const opened = await pipeline.openPipeline([], python.scripts)
    if (!opened.ok) throw new Error(JSON.stringify(opened.error))
    try {
      const output = await pipeline.materializePipeline(opened.value, graph)
      if (!output.ok) throw new Error(JSON.stringify(output.error))
      const replay = await pipeline.replayPipelineRecipe(output.value.recipe, [], python.scripts)
      if (!replay.ok) throw new Error(JSON.stringify(replay.error))
      return { rows: output.value.rowCount, inputs: output.value.recipe.inputs, same: String(new Uint8Array(await output.value.file.arrayBuffer())) === String(new Uint8Array(await replay.value.arrayBuffer())) }
    } finally { await pipeline.closePipeline(opened.value); python.dispose() }
  }, draftGraph)
  expect(result).toEqual({ rows: 3, inputs: [], same: true })
})

test('empty SQL editor creates a source without uploaded files', async ({ page }) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Generated SQL')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
  await page.getByRole('button', { name: 'Open empty SQL editor' }).click()
  const terminal = page.getByLabel('SQL console').locator('.xterm-helper-textarea')
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled({ timeout: 30_000 })
  await hideSqlSource(page)
  await terminal.focus()
  await terminal.pressSequentially('CREATE VIEW generated AS SELECT * FROM (VALUES (1), (2), (3)) t(value);')
  await terminal.press('Enter')
  await showSqlSource(page)
  await expect(async () => {
    await page.getByRole('button', { name: 'Refresh views' }).click()
    await expect(page.getByRole('combobox', { name: 'Output view' })).toBeVisible()
  }).toPass({ timeout: 20_000 })
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.screenshot({ path: 'test-results/file-free-sql.png' })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.waitForTimeout(1000)
  await page.goto('/app/projects')
  await page.getByRole('list', { name: 'Projects' }).getByRole('button', { name: 'Open Generated SQL', exact: true }).click()
  await page.getByRole('button', { name: 'Rebuild from saved recipe' }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
})

test('creates the Facure DataFrame despite an unused empty input card', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Canvas pointer verification runs on desktop')
  test.setTimeout(90_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Facure generated data')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.getByRole('button', { name: 'Open the editor' }).click()
  await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name: 'Script', exact: true }).click()
  const editor = page.getByRole('textbox', { name: 'Python script', exact: true })
  await editor.fill('import pandas as pd\nprepared = pd.DataFrame({"profits_prev_6m": [1.,1.,1.,5.,5.,5.], "consultancy": [0,0,1,0,1,1], "profits_next_6m": [1.,1.1,1.2,5.5,5.7,5.7]})')
  const block = page.locator('[data-testid^="block-script-"]')
  await expect(block).toContainText('6 rows, 3 columns', { timeout: 60_000 })
  const a = await block.locator('[data-handleid="out"]').boundingBox()
  const b = await page.getByTestId('block-output').locator('[data-handleid="in-0"]').boundingBox()
  if (!a || !b) throw new Error('Missing canvas handles')
  await page.mouse.move(a.x + a.width / 2, a.y + a.height / 2)
  await page.mouse.down()
  await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2, { steps: 20 })
  await page.mouse.up()
  await expect(page.getByTestId('block-output')).toContainText('6 rows, 3 columns')
  await page.screenshot({ path: 'test-results/file-free-pipeline.png' })
  await page.getByRole('button', { name: 'Use as source', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.waitForTimeout(1000)
  await page.goto('/app/projects')
  await page.getByRole('list', { name: 'Projects' }).getByRole('button', { name: 'Open Facure generated data', exact: true }).click()
  await page.getByRole('button', { name: 'Rebuild from saved recipe' }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
})
