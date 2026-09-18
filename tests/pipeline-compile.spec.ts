import { expect, test } from '@playwright/test'
import { brand } from '../src/domain/dop'
import { blockSql, compileDraft, compilePipeline, describePipelineProblem, parsePipelineRecipe, pipelineBlockId, type PipelineGraph, type PipelineNode } from '../src/domain/pipeline'
import { parseSourceRecipe } from '../src/domain/sqlPreparation'

const alias = brand<string, 'SqlInputAlias'>('frailty_data_tok')
const id = pipelineBlockId
const at = (x: number, y: number) => ({ x, y })

// The frailty preparation from the CLSA walkthrough, as blocks: repositories with at least 30
// events, joined back to the lines, comment-like lines dropped, ast_group coded as five indicators.
const frailty: PipelineGraph = {
  nodes: [
    { id: id('in'), block: { kind: 'input', file: { kind: 'chosen', alias } }, position: at(0, 0) },
    { id: id('events'), block: { kind: 'aggregate', groupBy: ['repo_id'], measures: [{ function: 'sum', column: 'event', as: 'events' }] }, position: at(0, 1) },
    { id: id('kept'), block: { kind: 'filter-rows', match: 'all', conditions: [{ column: 'events', test: 'ge', value: 30 }] }, position: at(0, 2) },
    { id: id('join'), block: { kind: 'join', how: 'inner', keys: [{ left: 'repo_id', right: 'repo_id' }] }, position: at(1, 2) },
    { id: id('lines'), block: { kind: 'filter-rows', match: 'all', conditions: [{ column: 'ast_group', test: 'ne', value: 'comment_like' }] }, position: at(1, 3) },
    { id: id('code'), block: { kind: 'derive-columns', columns: ['declaration', 'expression', 'import_export', 'other', 'type_system'].map((group) => ({ name: `ast_group_${group}`, expression: `CAST(ast_group = '${group}' AS INTEGER)` })) }, position: at(2, 3) },
    { id: id('out'), block: { kind: 'output' }, position: at(2, 4) },
  ],
  edges: [
    { from: id('in'), to: id('events'), port: 0 },
    { from: id('events'), to: id('kept'), port: 0 },
    { from: id('in'), to: id('join'), port: 0 },
    { from: id('kept'), to: id('join'), port: 1 },
    { from: id('join'), to: id('lines'), port: 0 },
    { from: id('lines'), to: id('code'), port: 0 },
    { from: id('code'), to: id('out'), port: 0 },
  ],
}
const aliases = new Set([alias as string])

test('compiles the frailty pipeline to one view per block in dependency order', () => {
  const compiled = compilePipeline(frailty, aliases)
  expect(compiled.ok).toBe(true)
  if (!compiled.ok) return
  const statements = compiled.value.steps.map((step) => step.kind === 'sql' ? step.statement : step.code)
  expect(statements).toEqual([
    'CREATE OR REPLACE VIEW "block_1" AS SELECT "repo_id", sum("event") AS "events" FROM "frailty_data_tok" GROUP BY "repo_id"',
    'CREATE OR REPLACE VIEW "block_2" AS SELECT * FROM "block_1" WHERE "events" >= 30',
    'CREATE OR REPLACE VIEW "block_3" AS SELECT * FROM "frailty_data_tok" INNER JOIN "block_2" USING ("repo_id")',
    `CREATE OR REPLACE VIEW "block_4" AS SELECT * FROM "block_3" WHERE "ast_group" <> 'comment_like'`,
    `CREATE OR REPLACE VIEW "block_5" AS SELECT *, CAST(ast_group = 'declaration' AS INTEGER) AS "ast_group_declaration", CAST(ast_group = 'expression' AS INTEGER) AS "ast_group_expression", CAST(ast_group = 'import_export' AS INTEGER) AS "ast_group_import_export", CAST(ast_group = 'other' AS INTEGER) AS "ast_group_other", CAST(ast_group = 'type_system' AS INTEGER) AS "ast_group_type_system" FROM "block_4"`,
    'CREATE OR REPLACE VIEW "block_6" AS SELECT * FROM "block_5"',
  ])
  expect(compiled.value.outputView).toBe('block_5')
  expect(compiled.value.views.get(id('in'))).toBe('frailty_data_tok')
})

test('a script block becomes a step for the Python runtime with its inputs named', () => {
  const graph: PipelineGraph = {
    nodes: [
      { id: id('in'), block: { kind: 'input', file: { kind: 'chosen', alias } }, position: at(0, 0) },
      { id: id('py'), block: { kind: 'script', code: 'prepared = inputs[0]' }, position: at(0, 1) },
      { id: id('out'), block: { kind: 'output' }, position: at(0, 2) },
    ],
    edges: [{ from: id('in'), to: id('py'), port: 0 }, { from: id('py'), to: id('out'), port: 0 }],
  }
  const compiled = compilePipeline(graph, aliases)
  expect(compiled.ok).toBe(true)
  if (!compiled.ok) return
  expect(compiled.value.steps[0]).toEqual({ kind: 'script', id: id('py'), view: 'block_1', code: 'prepared = inputs[0]', inputs: ['frailty_data_tok'], inputIds: [id('in')] })
})

test('the other blocks compile to the statements the documentation lists', () => {
  const node = (block: PipelineNode['block']): PipelineNode => ({ id: id('x'), block, position: at(0, 0) })
  const sql = (block: PipelineNode['block'], inputs: readonly string[]) => { const result = blockSql(node(block), inputs); return result.ok ? result.value : result.error }
  expect(sql({ kind: 'select-columns', mode: 'keep', columns: ['a', 'b'], renames: [{ from: 'b', to: 'c' }] }, ['t'])).toBe('SELECT "a", "b" AS "c" FROM "t"')
  expect(sql({ kind: 'select-columns', mode: 'drop', columns: ['a'], renames: [{ from: 'b', to: 'c' }] }, ['t'])).toBe('SELECT * EXCLUDE ("a") RENAME ("b" AS "c") FROM "t"')
  expect(sql({ kind: 'filter-rows', match: 'any', conditions: [{ column: 'a', test: 'is-null' }, { column: 'n', test: 'contains', value: 'x' }] }, ['t'])).toBe(`SELECT * FROM "t" WHERE "a" IS NULL OR contains("n"::VARCHAR, 'x')`)
  expect(sql({ kind: 'join', how: 'left', keys: [{ left: 'id', right: 'key' }] }, ['a', 'b'])).toBe('SELECT * FROM "a" AS l LEFT JOIN "b" AS r ON l."id" = r."key"')
  expect(sql({ kind: 'join', how: 'cross', keys: [] }, ['a', 'b'])).toBe('SELECT * FROM "a" CROSS JOIN "b"')
  expect(sql({ kind: 'union', by: 'name', distinct: false }, ['a', 'b', 'c'])).toBe('SELECT * FROM "a" UNION ALL BY NAME SELECT * FROM "b" UNION ALL BY NAME SELECT * FROM "c"')
  expect(sql({ kind: 'union', by: 'position', distinct: true }, ['a', 'b'])).toBe('SELECT * FROM "a" UNION SELECT * FROM "b"')
  expect(sql({ kind: 'aggregate', groupBy: [], measures: [{ function: 'count', as: 'n' }, { function: 'mean', column: 'v', as: 'mean_v' }] }, ['t'])).toBe('SELECT count(*) AS "n", avg("v") AS "mean_v" FROM "t"')
  expect(sql({ kind: 'sort-limit', sort: [{ column: 'v', direction: 'descending' }], limit: 10 }, ['t'])).toBe('SELECT * FROM "t" ORDER BY "v" DESC LIMIT 10')
  expect(sql({ kind: 'aggregate', groupBy: ['g'], measures: [] }, ['t'])).toEqual({ kind: 'incomplete-block', id: id('x'), detail: 'an aggregate needs at least one measure' })
})

test('refuses wiring the runtime could not follow, and says which block', () => {
  const name = (blockId: string) => blockId
  const withoutOutput = compilePipeline({ nodes: [frailty.nodes[0]!], edges: [] }, aliases)
  expect(withoutOutput.ok).toBe(false)
  if (!withoutOutput.ok) expect(describePipelineProblem(withoutOutput.error, name)).toBe('Add a "Use as source" block and wire the last step into it.')

  const halfJoin = compilePipeline({ nodes: frailty.nodes, edges: frailty.edges.filter((edge) => !(edge.to === 'join' && edge.port === 1)) }, aliases)
  expect(halfJoin.ok).toBe(false)
  if (!halfJoin.ok) expect(describePipelineProblem(halfJoin.error, name)).toBe('join needs 2 inputs wired in; it has 1.')

  const loop = compilePipeline({ nodes: frailty.nodes, edges: [...frailty.edges, { from: id('code'), to: id('events'), port: 1 }] }, aliases)
  expect(loop.ok).toBe(false)
  if (!loop.ok) expect(loop.error.kind === 'cycle' || loop.error.kind === 'wrong-input-count').toBe(true)

  const stray = compilePipeline({ nodes: [...frailty.nodes, { id: id('stray'), block: { kind: 'sort-limit', sort: [], limit: null }, position: at(9, 9) }], edges: [...frailty.edges, { from: id('in'), to: id('stray'), port: 0 }] }, aliases)
  expect(stray.ok).toBe(true)
  if (stray.ok) expect(stray.value.views.has(id('stray'))).toBe(false)
})

test('an input file nothing reads is allowed; a draft runs what is wired and says what the rest waits on', () => {
  const spare = { id: id('spare'), block: { kind: 'input' as const, file: { kind: 'chosen' as const, alias: brand<string, 'SqlInputAlias'>('spare') } }, position: at(9, 0) }
  const withSpare = compilePipeline({ ...frailty, nodes: [...frailty.nodes, spare] }, new Set([alias as string, 'spare']))
  expect(withSpare.ok).toBe(true)

  const half = { nodes: frailty.nodes, edges: frailty.edges.filter((edge) => !(edge.to === 'join' && edge.port === 1)) }
  const draft = compileDraft(half, aliases)
  expect(draft.steps.map((step) => step.id)).toEqual(['events', 'kept'])
  expect(draft.views.get(id('kept'))).toBe('block_2')
  expect(Object.fromEntries(draft.waiting)).toEqual({
    join: 'needs 2 inputs wired in; it has 1',
    lines: 'waits on a block that is not ready',
    code: 'waits on a block that is not ready',
    out: 'waits on a block that is not ready',
  })

  const looped = compileDraft({ nodes: frailty.nodes, edges: [...frailty.edges.filter((edge) => edge.to !== 'events'), { from: id('code'), to: id('events'), port: 0 }] }, aliases)
  expect(looped.steps).toEqual([])
  expect(looped.waiting.get(id('events'))).toBe('the arrows into it form a loop')
})

test('a pipeline recipe round-trips through the source recipe parser', () => {
  const recipe = {
    kind: 'pipeline-derived',
    graph: { nodes: frailty.nodes, edges: frailty.edges },
    inputs: [{ alias: 'frailty_data_tok', fileName: 'frailty_data_tok.csv', bytes: 41051892, format: 'csv', fingerprint: 'de1e77b21ac00bc71d3a54927b830c4950e59a103b5e99e716c36a101442e7d2' }],
  }
  const parsed = parseSourceRecipe(JSON.parse(JSON.stringify(recipe)))
  expect(parsed.ok).toBe(true)
  if (parsed.ok && parsed.value.kind === 'pipeline-derived') expect(parsed.value.graph.nodes).toHaveLength(7)
  const broken = parsePipelineRecipe({ ...recipe, graph: { nodes: recipe.graph.nodes, edges: [] } })
  expect(broken.ok).toBe(false)
  if (!broken.ok) expect(broken.error.detail).toContain('needs')
})
