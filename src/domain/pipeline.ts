import { z } from 'zod'
import { brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { inputDescriptorSchema, parseInputDescriptors, type SqlInputAlias, type SqlInputDescriptor } from './sourceInputs'

/**
 * A preparation pipeline: blocks on a canvas, each one operation on a table, wired into a directed
 * acyclic graph that ends at one output block. The pipeline is the recipe: it is stored with the
 * project and replayed on reopen. Every block but `script` compiles to one DuckDB statement over the
 * views of its inputs; a `script` block runs a pandas script in the Python runtime and its result is
 * registered as a table under the same name a compiled view would have had.
 */

export type PipelineBlockId = Brand<string, 'PipelineBlockId'>

export const pipelineBlockId = (value: string): PipelineBlockId => brand<string, 'PipelineBlockId'>(value)

export type RowTest = 'eq' | 'ne' | 'lt' | 'le' | 'gt' | 'ge' | 'contains' | 'starts-with' | 'is-null' | 'not-null'
export type JoinKind = 'inner' | 'left' | 'right' | 'full' | 'cross'
export type AggregateFunction = 'count' | 'sum' | 'mean' | 'min' | 'max' | 'first' | 'count-distinct'
export type SortDirection = 'ascending' | 'descending'

export type NullTest = 'is-null' | 'not-null'
export type ValueTest = Exclude<RowTest, NullTest>

/** A test against a value, or a test that needs none: the shape says which. */
export type RowCondition =
  | { readonly column: string; readonly test: NullTest }
  | { readonly column: string; readonly test: ValueTest; readonly value: string | number }

export const isNullTest = (test: RowTest): test is NullTest => test === 'is-null' || test === 'not-null'

/** The same condition under another test, keeping the value where the new test has one. */
export const conditionWithTest = (condition: RowCondition, test: RowTest): RowCondition =>
  isNullTest(test) ? { column: condition.column, test } : { column: condition.column, test, value: 'value' in condition ? condition.value : '' }

export interface DerivedColumn {
  readonly name: string
  readonly expression: string
}

export type ColumnAggregate = Exclude<AggregateFunction, 'count'>

/** A count of rows needs no column; every other function aggregates one. */
export type AggregateMeasure =
  | { readonly function: 'count'; readonly as: string }
  | { readonly function: ColumnAggregate; readonly column: string; readonly as: string }

/** The same measure under another function, keeping the column where the new function takes one. */
export const measureWithFunction = (measure: AggregateMeasure, fn: AggregateFunction, fallbackColumn: string): AggregateMeasure =>
  fn === 'count' ? { function: fn, as: measure.as } : { function: fn, column: 'column' in measure ? measure.column : fallbackColumn, as: measure.as }

/** An input block either holds a chosen file, known by its alias, or is still waiting for one. */
export type InputFile =
  | { readonly kind: 'chosen'; readonly alias: SqlInputAlias }
  | { readonly kind: 'empty' }

export type PipelineBlock =
  | { readonly kind: 'input'; readonly file: InputFile }
  | { readonly kind: 'filter-rows'; readonly match: 'all' | 'any'; readonly conditions: readonly RowCondition[] }
  | { readonly kind: 'select-columns'; readonly mode: 'keep' | 'drop'; readonly columns: readonly string[]; readonly renames: readonly { readonly from: string; readonly to: string }[] }
  | { readonly kind: 'derive-columns'; readonly columns: readonly DerivedColumn[] }
  | { readonly kind: 'join'; readonly how: JoinKind; readonly keys: readonly { readonly left: string; readonly right: string }[] }
  | { readonly kind: 'union'; readonly by: 'name' | 'position'; readonly distinct: boolean }
  | { readonly kind: 'aggregate'; readonly groupBy: readonly string[]; readonly measures: readonly AggregateMeasure[] }
  | { readonly kind: 'sort-limit'; readonly sort: readonly { readonly column: string; readonly direction: SortDirection }[]; readonly limit: number | null }
  | { readonly kind: 'script'; readonly code: string }
  | { readonly kind: 'output' }

export type PipelineBlockKind = PipelineBlock['kind']

export interface PipelineNode {
  readonly id: PipelineBlockId
  readonly block: PipelineBlock
  readonly position: { readonly x: number; readonly y: number }
}

export interface PipelineEdge {
  readonly from: PipelineBlockId
  readonly to: PipelineBlockId
  /** Which input of the target this feeds: 0 for the left side of a join, 1 for the right; the order of a union's inputs. */
  readonly port: number
}

export interface PipelineGraph {
  readonly nodes: readonly PipelineNode[]
  readonly edges: readonly PipelineEdge[]
}

export type BlockArity = { readonly kind: 'exactly'; readonly count: number } | { readonly kind: 'at-least'; readonly count: number }

export function blockArity(kind: PipelineBlockKind): BlockArity {
  switch (kind) {
    case 'input': return { kind: 'exactly', count: 0 }
    case 'join': return { kind: 'exactly', count: 2 }
    case 'union': return { kind: 'at-least', count: 2 }
    case 'script': return { kind: 'at-least', count: 0 }
    case 'filter-rows':
    case 'select-columns':
    case 'derive-columns':
    case 'aggregate':
    case 'sort-limit':
    case 'output': return { kind: 'exactly', count: 1 }
    default: { const exhaustive: never = kind; return exhaustive }
  }
}

export function blockLabel(kind: PipelineBlockKind): string {
  switch (kind) {
    case 'input': return 'Input file'
    case 'filter-rows': return 'Filter rows'
    case 'select-columns': return 'Select columns'
    case 'derive-columns': return 'Derive columns'
    case 'join': return 'Join'
    case 'union': return 'Union'
    case 'aggregate': return 'Group and aggregate'
    case 'sort-limit': return 'Sort and limit'
    case 'script': return 'Script'
    case 'output': return 'Use as source'
    default: { const exhaustive: never = kind; return exhaustive }
  }
}

/** The kinds a person adds from the palette; input and output blocks come with the canvas. */
export type AddableBlockKind = Exclude<PipelineBlockKind, 'input' | 'output'>

export function emptyBlock(kind: AddableBlockKind): PipelineBlock {
  switch (kind) {
    case 'filter-rows': return { kind, match: 'all', conditions: [] }
    case 'select-columns': return { kind, mode: 'keep', columns: [], renames: [] }
    case 'derive-columns': return { kind, columns: [] }
    case 'join': return { kind, how: 'inner', keys: [] }
    case 'union': return { kind, by: 'name', distinct: false }
    case 'aggregate': return { kind, groupBy: [], measures: [] }
    case 'sort-limit': return { kind, sort: [], limit: null }
    case 'script': return { kind, code: SCRIPT_TEMPLATE }
    default: { const exhaustive: never = kind; return exhaustive }
  }
}

export const SCRIPT_TEMPLATE = `import pandas as pd
import numpy as np

df = inputs[0] if inputs else pd.DataFrame()

prepared = df
`

export type PipelineProblem =
  | { readonly kind: 'no-output-block' }
  | { readonly kind: 'several-output-blocks' }
  | { readonly kind: 'unknown-node'; readonly id: PipelineBlockId }
  | { readonly kind: 'cycle' }
  | { readonly kind: 'wrong-input-count'; readonly id: PipelineBlockId; readonly expected: BlockArity; readonly actual: number }
  | { readonly kind: 'port-taken'; readonly id: PipelineBlockId; readonly port: number }
  | { readonly kind: 'unknown-input-alias'; readonly id: PipelineBlockId; readonly alias: string }
  | { readonly kind: 'incomplete-block'; readonly id: PipelineBlockId; readonly detail: string }
  | { readonly kind: 'unreachable-block'; readonly id: PipelineBlockId }

export type PipelineStep =
  | { readonly kind: 'sql'; readonly id: PipelineBlockId; readonly view: string; readonly statement: string }
  | { readonly kind: 'script'; readonly id: PipelineBlockId; readonly view: string; readonly code: string; readonly inputs: readonly string[] }

export interface CompiledPipeline {
  readonly steps: NonEmptyArray<PipelineStep>
  readonly outputView: string
  readonly views: ReadonlyMap<PipelineBlockId, string>
}

const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const identifier = (value: string): string => `"${value.replaceAll('"', '""')}"`
const literal = (value: string | number): string => typeof value === 'number' ? String(value) : sqlString(value)

const viewName = (index: number): string => `block_${index + 1}`

function conditionSql(condition: RowCondition): string {
  const column = identifier(condition.column)
  if (!('value' in condition)) return condition.test === 'is-null' ? `${column} IS NULL` : `${column} IS NOT NULL`
  switch (condition.test) {
    case 'eq': return `${column} = ${literal(condition.value)}`
    case 'ne': return `${column} <> ${literal(condition.value)}`
    case 'lt': return `${column} < ${literal(condition.value)}`
    case 'le': return `${column} <= ${literal(condition.value)}`
    case 'gt': return `${column} > ${literal(condition.value)}`
    case 'ge': return `${column} >= ${literal(condition.value)}`
    case 'contains': return `contains(${column}::VARCHAR, ${sqlString(String(condition.value))})`
    case 'starts-with': return `starts_with(${column}::VARCHAR, ${sqlString(String(condition.value))})`
    default: { const exhaustive: never = condition; return exhaustive }
  }
}

function measureSql(measure: AggregateMeasure): string {
  const call = (() => {
    if (measure.function === 'count') return 'count(*)'
    const column = identifier(measure.column)
    switch (measure.function) {
      case 'count-distinct': return `count(DISTINCT ${column})`
      case 'sum': return `sum(${column})`
      case 'mean': return `avg(${column})`
      case 'min': return `min(${column})`
      case 'max': return `max(${column})`
      case 'first': return `first(${column})`
      default: { const exhaustive: never = measure; return exhaustive }
    }
  })()
  return `${call} AS ${identifier(measure.as)}`
}

/** Input views are in port order. */
export function blockSql(node: PipelineNode, inputs: readonly string[]): Result<string, PipelineProblem> {
  const block = node.block
  const only = inputs[0] === undefined ? null : identifier(inputs[0])
  const incomplete = (detail: string): Result<never, PipelineProblem> => err({ kind: 'incomplete-block', id: node.id, detail })
  switch (block.kind) {
    case 'input': return incomplete('an input block compiles at registration, not here')
    case 'output': return only === null ? incomplete('the output block has nothing wired into it') : ok(`SELECT * FROM ${only}`)
    case 'filter-rows': {
      if (only === null) return incomplete('nothing is wired in')
      if (block.conditions.length === 0) return ok(`SELECT * FROM ${only}`)
      if (block.conditions.some((condition) => condition.column === '')) return incomplete('every condition needs a column')
      const joiner = block.match === 'all' ? ' AND ' : ' OR '
      return ok(`SELECT * FROM ${only} WHERE ${block.conditions.map(conditionSql).join(joiner)}`)
    }
    case 'select-columns': {
      if (only === null) return incomplete('nothing is wired in')
      if (block.renames.some((rename) => rename.from === '' || rename.to.trim() === '')) return incomplete('every rename needs a column and a new name')
      const renamed = new Map(block.renames.map((rename) => [rename.from, rename.to]))
      const columns = block.columns.map((column) => {
        const to = renamed.get(column)
        return to === undefined || to === column ? identifier(column) : `${identifier(column)} AS ${identifier(to)}`
      })
      if (block.mode === 'keep') {
        return columns.length === 0 ? incomplete('no column is kept') : ok(`SELECT ${columns.join(', ')} FROM ${only}`)
      }
      const dropped = block.columns.length === 0 ? '' : ` EXCLUDE (${block.columns.map(identifier).join(', ')})`
      const renames = block.renames.filter((rename) => !block.columns.includes(rename.from) && rename.to !== rename.from)
      const replace = renames.length === 0 ? '' : ` RENAME (${renames.map((rename) => `${identifier(rename.from)} AS ${identifier(rename.to)}`).join(', ')})`
      return ok(`SELECT *${dropped}${replace} FROM ${only}`)
    }
    case 'derive-columns': {
      if (only === null) return incomplete('nothing is wired in')
      if (block.columns.length === 0) return ok(`SELECT * FROM ${only}`)
      const empty = block.columns.find((column) => column.name.trim().length === 0 || column.expression.trim().length === 0)
      if (empty !== undefined) return incomplete('a derived column needs a name and an expression')
      return ok(`SELECT *, ${block.columns.map((column) => `${column.expression} AS ${identifier(column.name)}`).join(', ')} FROM ${only}`)
    }
    case 'join': {
      const [left, right] = inputs
      if (left === undefined || right === undefined) return incomplete('a join needs both sides wired in')
      if (block.how === 'cross') return ok(`SELECT * FROM ${identifier(left)} CROSS JOIN ${identifier(right)}`)
      if (block.keys.length === 0) return incomplete('a join needs at least one key column on each side')
      if (block.keys.some((key) => key.left === '' || key.right === '')) return incomplete('every key needs a column on each side')
      const same = block.keys.every((key) => key.left === key.right)
      if (same) {
        return ok(`SELECT * FROM ${identifier(left)} ${block.how.toUpperCase()} JOIN ${identifier(right)} USING (${block.keys.map((key) => identifier(key.left)).join(', ')})`)
      }
      const on = block.keys.map((key) => `l.${identifier(key.left)} = r.${identifier(key.right)}`).join(' AND ')
      return ok(`SELECT * FROM ${identifier(left)} AS l ${block.how.toUpperCase()} JOIN ${identifier(right)} AS r ON ${on}`)
    }
    case 'union': {
      if (inputs.length < 2) return incomplete('a union needs at least two inputs wired in')
      const operator = `UNION${block.distinct ? '' : ' ALL'}${block.by === 'name' ? ' BY NAME' : ''}`
      return ok(inputs.map((input) => `SELECT * FROM ${identifier(input)}`).join(` ${operator} `))
    }
    case 'aggregate': {
      if (only === null) return incomplete('nothing is wired in')
      if (block.measures.length === 0) return incomplete('an aggregate needs at least one measure')
      const missingName = block.measures.find((measure) => measure.as.trim().length === 0)
      if (missingName !== undefined) return incomplete('every measure needs a name for its result column')
      if (block.measures.some((measure) => measure.function !== 'count' && measure.column === '')) return incomplete('every measure needs a column')
      const groups = block.groupBy.map(identifier)
      const selected = [...groups, ...block.measures.map(measureSql)].join(', ')
      const grouping = groups.length === 0 ? '' : ` GROUP BY ${groups.join(', ')}`
      return ok(`SELECT ${selected} FROM ${only}${grouping}`)
    }
    case 'sort-limit': {
      if (only === null) return incomplete('nothing is wired in')
      if (block.sort.some((entry) => entry.column === '')) return incomplete('every sort needs a column')
      const order = block.sort.length === 0 ? '' : ` ORDER BY ${block.sort.map((entry) => `${identifier(entry.column)} ${entry.direction === 'ascending' ? 'ASC' : 'DESC'}`).join(', ')}`
      const limit = block.limit === null ? '' : ` LIMIT ${Math.max(0, Math.floor(block.limit))}`
      return ok(`SELECT * FROM ${only}${order}${limit}`)
    }
    case 'script': return incomplete('a script block runs in Python, not in SQL')
    default: { const exhaustive: never = block; return exhaustive }
  }
}

/** The chosen port map of a graph: which block feeds each port of each block, and which blocks have a port fed twice. */
interface Wiring {
  readonly byId: ReadonlyMap<PipelineBlockId, PipelineNode>
  readonly inputIdsOf: (id: PipelineBlockId) => readonly PipelineBlockId[]
  readonly doubled: ReadonlySet<PipelineBlockId>
}

const wiringOf = (graph: PipelineGraph): Wiring => {
  const byId = new Map(graph.nodes.map((node) => [node.id, node]))
  const ports = new Map<PipelineBlockId, Map<number, PipelineBlockId>>()
  const doubled = new Set<PipelineBlockId>()
  for (const edge of graph.edges) {
    if (!byId.has(edge.from) || !byId.has(edge.to)) continue
    const own = ports.get(edge.to) ?? new Map<number, PipelineBlockId>()
    if (own.has(edge.port)) doubled.add(edge.to)
    own.set(edge.port, edge.from)
    ports.set(edge.to, own)
  }
  const inputIdsOf = (id: PipelineBlockId): readonly PipelineBlockId[] => {
    const own = ports.get(id)
    return own === undefined ? [] : [...own.entries()].sort((a, b) => a[0] - b[0]).map(([, from]) => from)
  }
  return { byId, inputIdsOf, doubled }
}

export type DraftStep = PipelineStep & { readonly inputIds: readonly PipelineBlockId[] }

/**
 * What the canvas runs while the pipeline is being built: every block whose inputs are all wired and
 * themselves ready, in dependency order. The rest wait, each with the reason, so a half-built graph
 * still shows rows on the blocks that can produce them. Source readiness is a separate check of
 * the output branch in `compilePipeline`.
 */
export interface DraftPipeline {
  readonly steps: readonly DraftStep[]
  readonly views: ReadonlyMap<PipelineBlockId, string>
  readonly waiting: ReadonlyMap<PipelineBlockId, string>
}

export function compileDraft(graph: PipelineGraph, aliases: ReadonlySet<string>): DraftPipeline {
  const { inputIdsOf, doubled } = wiringOf(graph)
  const views = new Map<PipelineBlockId, string>()
  const waiting = new Map<PipelineBlockId, string>()
  const steps: DraftStep[] = []
  for (const node of graph.nodes) {
    if (node.block.kind !== 'input') continue
    if (node.block.file.kind === 'empty') waiting.set(node.id, 'no file chosen yet')
    else if (aliases.has(node.block.file.alias)) views.set(node.id, node.block.file.alias)
    else waiting.set(node.id, `${node.block.file.alias} is not one of the input files`)
  }
  // Blocks become ready as their inputs do; a pass that settles nothing means the rest are in a loop.
  let pending = graph.nodes.filter((node) => node.block.kind !== 'input')
  let settled = true
  while (pending.length > 0 && settled) {
    settled = false
    const later: PipelineNode[] = []
    for (const node of pending) {
      const inputIds = inputIdsOf(node.id)
      const arity = blockArity(node.block.kind)
      const fits = arity.kind === 'exactly' ? inputIds.length === arity.count : inputIds.length >= arity.count
      if (doubled.has(node.id)) { waiting.set(node.id, 'two arrows point at the same input'); settled = true; continue }
      if (!fits) {
        const expected = arity.kind === 'exactly' ? `${arity.count}` : `at least ${arity.count}`
        waiting.set(node.id, `needs ${expected} ${arity.count === 1 ? 'input' : 'inputs'} wired in; it has ${inputIds.length}`)
        settled = true
        continue
      }
      if (inputIds.some((from) => waiting.has(from))) { waiting.set(node.id, 'waits on a block that is not ready'); settled = true; continue }
      if (!inputIds.every((from) => views.has(from))) { later.push(node); continue }
      const view = viewName(steps.length)
      const inputViews = inputIds.map((from) => views.get(from)!)
      if (node.block.kind === 'script') {
        steps.push({ kind: 'script', id: node.id, view, code: node.block.code, inputs: inputViews, inputIds })
      } else {
        const sql = blockSql(node, inputViews)
        if (!sql.ok) { waiting.set(node.id, sql.error.kind === 'incomplete-block' ? sql.error.detail : sql.error.kind); settled = true; continue }
        steps.push({ kind: 'sql', id: node.id, view, statement: `CREATE OR REPLACE VIEW ${identifier(view)} AS ${sql.value}`, inputIds })
      }
      views.set(node.id, view)
      settled = true
    }
    pending = later
  }
  for (const node of pending) waiting.set(node.id, 'the arrows into it form a loop')
  return { steps, views, waiting }
}

/**
 * Only the output and its ancestors determine source readiness. Unconnected draft work remains
 * editable without becoming a dependency of the saved source.
 */
export function compilePipeline(graph: PipelineGraph, aliases: ReadonlySet<string>): Result<CompiledPipeline, PipelineProblem> {
  const outputs = graph.nodes.filter((node) => node.block.kind === 'output')
  if (outputs.length === 0) return err({ kind: 'no-output-block' })
  if (outputs.length > 1) return err({ kind: 'several-output-blocks' })
  const output = outputs[0]!
  const { byId, inputIdsOf } = wiringOf(graph)
  for (const edge of graph.edges) {
    if (!byId.has(edge.from)) return err({ kind: 'unknown-node', id: edge.from })
    if (!byId.has(edge.to)) return err({ kind: 'unknown-node', id: edge.to })
  }
  const draft = compileDraft(graph, aliases)

  const reached = new Set<PipelineBlockId>()
  const pending = [output.id]
  while (pending.length > 0) {
    const id = pending.pop()!
    if (reached.has(id)) continue
    reached.add(id)
    pending.push(...inputIdsOf(id))
  }
  const sourceGraph = { nodes: graph.nodes.filter((node) => reached.has(node.id)), edges: graph.edges.filter((edge) => reached.has(edge.to)) }
  const problem = draftProblem(sourceGraph, draft)
  if (problem !== null) return err(problem)
  const steps = draft.steps.filter((step) => reached.has(step.id))
  const fed = inputIdsOf(output.id)[0]
  const outputView = fed === undefined ? undefined : draft.views.get(fed)
  if (outputView === undefined || !isNonEmpty(steps)) return err({ kind: 'incomplete-block', id: output.id, detail: 'the output block has nothing wired into it' })
  return ok({ steps, outputView, views: new Map([...draft.views].filter(([id]) => reached.has(id))) })
}

/** The first waiting block's reason as a problem, in the shape the strict compile reports. */
const draftProblem = (graph: PipelineGraph, draft: DraftPipeline): PipelineProblem | null => {
  const { inputIdsOf, doubled } = wiringOf(graph)
  for (const node of graph.nodes) {
    const reason = draft.waiting.get(node.id)
    if (reason === undefined) continue
    if (doubled.has(node.id)) return { kind: 'port-taken', id: node.id, port: 0 }
    if (node.block.kind === 'input') return node.block.file.kind === 'empty' ? { kind: 'incomplete-block', id: node.id, detail: 'no file chosen yet' } : { kind: 'unknown-input-alias', id: node.id, alias: node.block.file.alias }
    if (reason === 'the arrows into it form a loop') return { kind: 'cycle' }
    const arity = blockArity(node.block.kind)
    const actual = inputIdsOf(node.id).length
    const fits = arity.kind === 'exactly' ? actual === arity.count : actual >= arity.count
    if (!fits) return { kind: 'wrong-input-count', id: node.id, expected: arity, actual }
    if (reason === 'waits on a block that is not ready') continue
    return { kind: 'incomplete-block', id: node.id, detail: reason }
  }
  return null
}

export function describePipelineProblem(problem: PipelineProblem, name: (id: PipelineBlockId) => string): string {
  switch (problem.kind) {
    case 'no-output-block': return 'Add a "Use as source" block and wire the last step into it.'
    case 'several-output-blocks': return 'Only one "Use as source" block is allowed.'
    case 'unknown-node': return 'An arrow points at a block that no longer exists.'
    case 'cycle': return 'The arrows form a loop; a pipeline must flow one way.'
    case 'wrong-input-count': {
      const expected = problem.expected.kind === 'exactly' ? `${problem.expected.count}` : `at least ${problem.expected.count}`
      return `${name(problem.id)} needs ${expected} ${problem.expected.count === 1 ? 'input' : 'inputs'} wired in; it has ${problem.actual}.`
    }
    case 'port-taken': return `${name(problem.id)} has two arrows into the same input.`
    case 'unknown-input-alias': return `${name(problem.id)} refers to ${problem.alias}, which is not one of the input files.`
    case 'incomplete-block': return `${name(problem.id)}: ${problem.detail}.`
    case 'unreachable-block': return `${name(problem.id)} is not wired into the output. Remove it or connect it.`
    default: { const exhaustive: never = problem; return exhaustive }
  }
}

const identifierSchema = z.string().min(1)

const blockSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('input'), file: z.discriminatedUnion('kind', [z.object({ kind: z.literal('chosen'), alias: z.string().min(1) }).strict(), z.object({ kind: z.literal('empty') }).strict()]) }).strict(),
  z.object({
    kind: z.literal('filter-rows'),
    match: z.enum(['all', 'any']),
    conditions: z.array(z.discriminatedUnion('test', [
      z.object({ column: identifierSchema, test: z.enum(['is-null', 'not-null']) }).strict(),
      z.object({ column: identifierSchema, test: z.enum(['eq', 'ne', 'lt', 'le', 'gt', 'ge', 'contains', 'starts-with']), value: z.union([z.string(), z.number()]) }).strict(),
    ])),
  }).strict(),
  z.object({ kind: z.literal('select-columns'), mode: z.enum(['keep', 'drop']), columns: z.array(identifierSchema), renames: z.array(z.object({ from: identifierSchema, to: identifierSchema }).strict()) }).strict(),
  z.object({ kind: z.literal('derive-columns'), columns: z.array(z.object({ name: z.string(), expression: z.string() }).strict()) }).strict(),
  z.object({ kind: z.literal('join'), how: z.enum(['inner', 'left', 'right', 'full', 'cross']), keys: z.array(z.object({ left: identifierSchema, right: identifierSchema }).strict()) }).strict(),
  z.object({ kind: z.literal('union'), by: z.enum(['name', 'position']), distinct: z.boolean() }).strict(),
  z.object({ kind: z.literal('aggregate'), groupBy: z.array(identifierSchema), measures: z.array(z.discriminatedUnion('function', [
    z.object({ function: z.literal('count'), as: z.string() }).strict(),
    z.object({ function: z.enum(['sum', 'mean', 'min', 'max', 'first', 'count-distinct']), column: identifierSchema, as: z.string() }).strict(),
  ])) }).strict(),
  z.object({ kind: z.literal('sort-limit'), sort: z.array(z.object({ column: identifierSchema, direction: z.enum(['ascending', 'descending']) }).strict()), limit: z.number().int().nonnegative().nullable() }).strict(),
  z.object({ kind: z.literal('script'), code: z.string() }).strict(),
  z.object({ kind: z.literal('output') }).strict(),
])

export const pipelineGraphSchema = z.object({
  nodes: z.array(z.object({ id: z.string().min(1), block: blockSchema, position: z.object({ x: z.number().finite(), y: z.number().finite() }).strict() }).strict()),
  edges: z.array(z.object({ from: z.string().min(1), to: z.string().min(1), port: z.number().int().nonnegative() }).strict()),
}).strict()

export const pipelineRecipeSchema = z.object({
  kind: z.literal('pipeline-derived'),
  graph: pipelineGraphSchema,
  inputs: z.array(inputDescriptorSchema),
}).strict()

export interface PipelineRecipe {
  readonly kind: 'pipeline-derived'
  readonly graph: PipelineGraph
  readonly inputs: readonly SqlInputDescriptor[]
}

export function parsePipelineRecipe(value: unknown): Result<PipelineRecipe, { readonly kind: 'invalid-pipeline-recipe'; readonly detail: string }> {
  const parsed = pipelineRecipeSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-pipeline-recipe', detail: z.prettifyError(parsed.error) })
  const inputs = parseInputDescriptors(parsed.data.inputs)
  if (!inputs.ok) return err({ kind: 'invalid-pipeline-recipe', detail: inputs.error.detail })
  const graph: PipelineGraph = {
    nodes: parsed.data.graph.nodes.map((node) => ({
      id: pipelineBlockId(node.id),
      block: node.block.kind === 'input'
        ? { kind: 'input', file: node.block.file.kind === 'chosen' ? { kind: 'chosen', alias: brand<string, 'SqlInputAlias'>(node.block.file.alias) } : { kind: 'empty' } }
        : node.block,
      position: node.position,
    })),
    edges: parsed.data.graph.edges.map((edge) => ({ from: pipelineBlockId(edge.from), to: pipelineBlockId(edge.to), port: edge.port })),
  }
  const compiled = compilePipeline(graph, new Set(inputs.value.map((input) => input.alias as string)))
  if (!compiled.ok) return err({ kind: 'invalid-pipeline-recipe', detail: describePipelineProblem(compiled.error, (id) => id) })
  return ok({ kind: 'pipeline-derived', graph, inputs: inputs.value })
}
