import { expect, test } from '@playwright/test'

// The pipeline data layer in the browser: inputs registered on DuckDB, the compiled steps run, every
// block previewed, the output written to Parquet with its recipe, and the recipe replayed from the
// same files. A join, an aggregate, a filter and a derived column on two small tables.
const runInBrowser = async (page: import('@playwright/test').Page) => page.evaluate(async () => {
  const sql = await import(new URL('/src/data/sqlPreparation.ts', window.location.href).href)
  const pipeline = await import(new URL('/src/data/pipeline.ts', window.location.href).href)
  const domain = await import(new URL('/src/domain/pipeline.ts', window.location.href).href)
  const files = [
    new File(['id,city,population\n1,Lyon,513\n2,Nantes,318\n3,Lille,232\n4,Nice,342\n'], 'cities.csv', { type: 'text/csv' }),
    new File(['id,region\n1,Auvergne-Rhône-Alpes\n2,Pays de la Loire\n3,Hauts-de-France\n4,Provence-Alpes-Côte d\'Azur\n'], 'regions.csv', { type: 'text/csv' }),
  ]
  const inputs = await sql.prepareSqlInputs(files)
  if (!inputs.ok) throw new Error(`inputs: ${inputs.error.kind}`)
  const [cities, regions] = inputs.value
  const id = domain.pipelineBlockId
  const graph = {
    nodes: [
      { id: id('cities'), block: { kind: 'input', file: { kind: 'chosen', alias: cities.alias } }, position: { x: 0, y: 0 } },
      { id: id('regions'), block: { kind: 'input', file: { kind: 'chosen', alias: regions.alias } }, position: { x: 1, y: 0 } },
      { id: id('join'), block: { kind: 'join', how: 'inner', keys: [{ left: 'id', right: 'id' }] }, position: { x: 0, y: 1 } },
      { id: id('big'), block: { kind: 'filter-rows', match: 'all', conditions: [{ column: 'population', test: 'gt', value: 300 }] }, position: { x: 0, y: 2 } },
      { id: id('thousands'), block: { kind: 'derive-columns', columns: [{ name: 'population_m', expression: 'population / 1000.0' }] }, position: { x: 0, y: 3 } },
      { id: id('out'), block: { kind: 'output' }, position: { x: 0, y: 4 } },
    ],
    edges: [
      { from: id('cities'), to: id('join'), port: 0 },
      { from: id('regions'), to: id('join'), port: 1 },
      { from: id('join'), to: id('big'), port: 0 },
      { from: id('big'), to: id('thousands'), port: 0 },
      { from: id('thousands'), to: id('out'), port: 0 },
    ],
  }
  const session = await pipeline.openPipeline(inputs.value)
  if (!session.ok) throw new Error(`open: ${session.error.kind}`)
  try {
    const ran = await pipeline.runPipeline(session.value, graph)
    if (!ran.ok) throw new Error(`run: ${pipeline.describePipelineRunProblem(ran.error, (blockId: string) => blockId)}`)
    const outcomes = Object.fromEntries([...ran.value.outcomes.entries()].map(([blockId, outcome]) => [blockId, outcome.kind === 'ran' ? { rows: outcome.rowCount, columns: outcome.columns.map((column: { name: string }) => column.name) } : outcome]))
    const bigOutcome = ran.value.outcomes.get(id('big'))
    const preview = await pipeline.previewBlock(session.value, ran.value.views.get(id('big')), id('big'), bigOutcome)
    if (!preview.ok) throw new Error(`preview: ${preview.error.kind}`)
    const materialized = await pipeline.materializePipeline(session.value, graph)
    if (!materialized.ok) throw new Error(`materialize: ${pipeline.describePipelineRunProblem(materialized.error, (blockId: string) => blockId)}`)
    // A broken block: the aggregate names a column that does not exist; the blocks after it are skipped.
    const brokenGraph = { ...graph, nodes: graph.nodes.map((node) => node.id === 'big' ? { ...node, block: { kind: 'filter-rows', match: 'all', conditions: [{ column: 'populaton', test: 'gt', value: 300 }] } } : node) }
    const broken = await pipeline.runPipeline(session.value, brokenGraph)
    if (!broken.ok) throw new Error('broken run should still return outcomes')
    const brokenOutcomes = Object.fromEntries([...broken.value.outcomes.entries()].map(([blockId, outcome]) => [blockId, outcome.kind]))
    const brokenDetail = broken.value.outcomes.get(id('big'))
    // Replay from the recipe and the same files gives the same bytes.
    const replayed = await pipeline.replayPipelineRecipe(materialized.value.recipe, files, null)
    if (!replayed.ok) throw new Error(`replay: ${replayed.error.kind}`)
    const bytes = async (file: File) => Array.from(new Uint8Array(await file.arrayBuffer()))
    return {
      outcomes,
      preview: { rows: preview.value.rows.map((row: readonly { kind: string; value?: unknown }[]) => row.map((cell) => cell.kind === 'integer' ? Number(cell.value) : cell.value)), columns: preview.value.columns.map((column: { name: string; type: string }) => `${column.name}:${column.type}`) },
      output: { rows: materialized.value.rowCount, columns: materialized.value.columns.map((column: { name: string }) => column.name), fileName: materialized.value.file.name, recipeKind: materialized.value.recipe.kind, recipeInputs: materialized.value.recipe.inputs.map((input: { alias: string }) => input.alias) },
      brokenOutcomes,
      brokenDetail: brokenDetail?.kind === 'failed' ? brokenDetail.detail : null,
      sameBytes: JSON.stringify(await bytes(replayed.value)) === JSON.stringify(await bytes(materialized.value.file)),
    }
  } finally {
    await pipeline.closePipeline(session.value)
  }
})

test('runs a joined, filtered and derived pipeline on DuckDB and replays it from the recipe', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The engine test runs once')
  await page.goto('/app')
  const result = await runInBrowser(page)
  expect(result.outcomes).toEqual({
    cities: { rows: 4, columns: ['id', 'city', 'population'] },
    regions: { rows: 4, columns: ['id', 'region'] },
    join: { rows: 4, columns: ['id', 'city', 'population', 'region'] },
    big: { rows: 3, columns: ['id', 'city', 'population', 'region'] },
    thousands: { rows: 3, columns: ['id', 'city', 'population', 'region', 'population_m'] },
    out: { rows: 3, columns: ['id', 'city', 'population', 'region', 'population_m'] },
  })
  expect(result.preview.columns).toEqual(['id:BIGINT', 'city:VARCHAR', 'population:BIGINT', 'region:VARCHAR'])
  expect(result.preview.rows).toEqual([[1, 'Lyon', 513, 'Auvergne-Rhône-Alpes'], [2, 'Nantes', 318, 'Pays de la Loire'], [4, 'Nice', 342, "Provence-Alpes-Côte d'Azur"]])
  expect(result.output).toEqual({ rows: 3, columns: ['id', 'city', 'population', 'region', 'population_m'], fileName: 'pipeline_prepared.parquet', recipeKind: 'pipeline-derived', recipeInputs: ['cities', 'regions'] })
  expect(result.brokenOutcomes).toEqual({ cities: 'ran', regions: 'ran', join: 'ran', big: 'failed', thousands: 'skipped', out: 'skipped' })
  expect(result.brokenDetail).toContain('populaton')
  expect(result.sameBytes).toBe(true)
})

test('previews 128-bit sums and decimals as numbers, not as Arrow words', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The DuckDB preview runs once')
  await page.goto('/app')
  const preview = await page.evaluate(async () => {
    const sql = await import(new URL('/src/data/sqlPreparation.ts', window.location.href).href)
    const pipeline = await import(new URL('/src/data/pipeline.ts', window.location.href).href)
    const files = [new File(['year,cases,price\n1997,20,1.25\n1997,2,2.50\n1998,234,9.99\n'], 'counts.csv', { type: 'text/csv' })]
    const inputs = await sql.prepareSqlInputs(files)
    if (!inputs.ok) throw new Error(`inputs: ${inputs.error.kind}`)
    const alias = inputs.value[0].alias
    const graph = {
      nodes: [
        { id: 'in', block: { kind: 'input', file: { kind: 'chosen', alias } }, position: { x: 0, y: 0 } },
        { id: 'exact', block: { kind: 'derive-columns', columns: [{ name: 'price_exact', expression: 'CAST(price AS DECIMAL(10,2))' }] }, position: { x: 0, y: 1 } },
        { id: 'sum', block: { kind: 'aggregate', groupBy: ['year'], measures: [{ function: 'sum', column: 'cases', as: 'total' }, { function: 'sum', column: 'price', as: 'spent' }, { function: 'sum', column: 'price_exact', as: 'spent_exact' }] }, position: { x: 0, y: 2 } },
        { id: 'out', block: { kind: 'output' }, position: { x: 0, y: 3 } },
      ],
      edges: [{ from: 'in', to: 'exact', port: 0 }, { from: 'exact', to: 'sum', port: 0 }, { from: 'sum', to: 'out', port: 0 }],
    }
    const session = await pipeline.openPipeline(inputs.value)
    if (!session.ok) throw new Error(`open: ${session.error.kind}`)
    try {
      const ran = await pipeline.runPipeline(session.value, graph)
      if (!ran.ok) throw new Error('run')
      const shown = await pipeline.previewBlock(session.value, ran.value.views.get('sum'), 'sum', ran.value.outcomes.get('sum'))
      if (!shown.ok) throw new Error(`preview: ${shown.error.kind}`)
      return {
        columns: shown.value.columns.map((column: { name: string; type: string }) => `${column.name}:${column.type}`),
        rows: shown.value.rows.map((row: readonly { kind: string; value?: unknown }[]) => row.map((cell) => `${cell.kind}:${cell.value}`)).sort(),
      }
    } finally {
      await pipeline.closePipeline(session.value)
    }
  })
  expect(preview.columns).toEqual(['year:BIGINT', 'total:HUGEINT', 'spent:DOUBLE', 'spent_exact:DECIMAL(38,2)'])
  expect(preview.rows).toEqual([['integer:1997', 'integer:22', 'number:3.75', 'number:3.75'], ['integer:1998', 'integer:234', 'number:9.99', 'number:9.99']])
})
