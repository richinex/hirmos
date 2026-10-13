import { z } from 'zod'
import type { AsyncDuckDB } from '@duckdb/duckdb-wasm'
import { isNumericDuckDbType } from '@/domain/dataset'
import { err, ok, type Result } from '@/domain/dop'
import { parsePipelineRecipe, pipelineGraphSchema, type PipelineGraph } from '@/domain/pipeline'
import type { SqlInputDescriptor } from '@/domain/sourceInputs'
import { proposalReviewSchema, type ProposalReview } from './proposalReview'
import { preparationCheckSchema, checkPreparation } from './preparationChecks'

export interface TableProfile {
  readonly checks?: Awaited<ReturnType<typeof checkPreparation>>
  readonly table: string
  readonly rows: number
  readonly columns: readonly { readonly name: string; readonly type: string }[]
  readonly summaries: readonly {
    readonly column: string
    readonly nulls: number
    readonly distinct: number
    readonly minimum: string | null
    readonly maximum: string | null
    readonly categories: readonly { readonly value: string | null; readonly count: number }[]
    readonly formats: readonly { readonly pattern: string; readonly count: number }[]
  }[]
}
export type PreparationInspection = {
  readonly tables: readonly string[]
} & (
  | { readonly kind: 'source'; readonly name: string; readonly meaning: string }
  | { readonly kind: 'sql' }
  | {
      readonly kind: 'pipeline'
      readonly graph: PipelineGraph
      readonly outputs: readonly { readonly block: string; readonly table: string }[]
    }
)
export type PreparationCapability = {
  readonly revision: string
  readonly profile: (request: ProfileRequest, signal: AbortSignal) => Promise<TableProfile>
} & (
  | { readonly kind: 'source'; readonly inspection: Extract<PreparationInspection, { kind: 'source' }> }
  | { readonly kind: 'sql'; readonly inspection: Extract<PreparationInspection, { kind: 'sql' }> }
  | {
      readonly kind: 'pipeline'
      readonly inspection: Extract<PreparationInspection, { kind: 'pipeline' }>
      readonly inputs: readonly SqlInputDescriptor[]
      readonly apply: (graph: PipelineGraph) => void
    }
)
export function pipelineSchema(raw: unknown) {
  const parsed = z
    .object({ blocks: z.array(z.string()).max(3).optional() })
    .strict()
    .safeParse(raw)
  if (!parsed.success) return { kind: 'refused', reason: 'Request up to three block kinds.' }
  const options = pipelineGraphSchema.shape.nodes.element.shape.block.options
  const kinds = options.map((option) => option.shape.kind.value)
  const requested = parsed.data.blocks ?? []
  if (requested.some((kind) => !kinds.some((known) => known === kind)))
    return { kind: 'refused', reason: 'Unknown block kind.', available: kinds }
  return {
    kind: 'schema',
    expressions: {
      dialect: 'DuckDB SQL',
      deriveColumns:
        'Expressions are SELECT-list expressions producing a value for each input row. Window functions such as LAG(value, 12) OVER (ORDER BY period) are allowed; add PARTITION BY for separate units. A lag counts rows, not calendar periods, so check spacing, ordering and ties.',
      filterRows:
        'Filter expressions run in WHERE, where window functions are not allowed. Derive a window column in an upstream block before filtering it.',
    },
    available: kinds,
    graph: {
      nodes: '[{id: string, block: one of the block schemas, position: {x: number, y: number}}]',
      edges: '[{from: node id, to: node id, port: nonnegative integer}]',
    },
    blocks: options
      .filter((option) => requested.includes(option.shape.kind.value))
      .map((option) => z.toJSONSchema(option)),
  }
}
export const profileRequestSchema = z
  .object({
    table: z.string().min(1).max(256),
    columns: z.array(z.string().min(1)).max(12),
    categoryColumn: z.string().min(1).optional(),
    checks: z.array(preparationCheckSchema).max(8).optional(),
  })
  .strict()
export type ProfileRequest = z.infer<typeof profileRequestSchema>
export const profileToolSchema = profileRequestSchema.extend({
  compareTo: z.string().uuid().optional(),
})
export interface ProfileReceipt {
  readonly id: string
  readonly revision: string
  readonly capturedAt: string
  readonly profile: TableProfile
}
export function compareProfiles(before: TableProfile, after: TableProfile) {
  const names = new Set(before.columns.map((column) => column.name))
  return {
    rowsBefore: before.rows,
    rowsAfter: after.rows,
    rowDifference: after.rows - before.rows,
    addedColumns: after.columns.filter((column) => !names.has(column.name)),
    removedColumns: before.columns.filter(
      (column) => !after.columns.some((item) => item.name === column.name),
    ),
    typeChanges: after.columns.flatMap((column) => {
      const previous = before.columns.find((item) => item.name === column.name)
      return previous && previous.type !== column.type
        ? [{ column: column.name, before: previous.type, after: column.type }]
        : []
    }),
    nullCounts: after.summaries.map((column) => ({
      column: column.column,
      before: before.summaries.find((item) => item.column === column.column)?.nulls ?? null,
      after: column.nulls,
    })),
    limitation:
      'These are marginal summaries. Equal counts do not establish that rows, keys, order or values were preserved. Unprofiled columns have no missing-value comparison.',
  }
}
export interface PipelineProposal extends ProposalReview {
  readonly kind: 'pipeline-draft'
  readonly revision: string
  readonly graph: PipelineGraph
}
export function proposePipeline(
  capability: PreparationCapability | null,
  raw: unknown,
): Result<PipelineProposal, string> {
  if (!capability || capability.kind !== 'pipeline')
    return err('Open the pipeline workspace first.')
  const parsed = proposalReviewSchema.extend({ graph: pipelineGraphSchema }).strict().safeParse(raw)
  if (!parsed.success)
    return err(
      'Supply a pipeline graph, a one-line summary and 1–12 review checks, each on one line and at most 400 characters.',
    )
  if (parsed.data.graph.nodes.length > 80) return err('Propose at most 80 pipeline blocks.')
  const recipe = parsePipelineRecipe({
    kind: 'pipeline-derived',
    graph: parsed.data.graph,
    inputs: capability.inputs,
  })
  if (!recipe.ok) return err(recipe.error.detail)
  return ok({
    kind: 'pipeline-draft',
    revision: capability.revision,
    graph: recipe.value.graph,
    summary: parsed.data.summary,
    checks: parsed.data.checks,
  })
}

const identifier = (value: string) => `"${value.replaceAll('"', '""')}"`
const count = z
  .union([z.number(), z.bigint().transform(Number)])
  .pipe(z.number().int().nonnegative().safe())
const columnSchema = z.object({ column_name: z.string(), column_type: z.string() })
const summarySchema = z.object({
  nulls: count,
  distinct: count,
  minimum: z.string().nullable(),
  maximum: z.string().nullable(),
})
const categorySchema = z.object({ value: z.string().nullable(), count })
const formatSchema = z.object({ pattern: z.string(), count })

export async function profileTable(
  database: AsyncDuckDB,
  tables: readonly string[],
  request: ProfileRequest,
  signal: AbortSignal,
): Promise<TableProfile> {
  if (!tables.includes(request.table))
    throw new Error('This table is not available in the current workspace.')
  signal.throwIfAborted()
  const connection = await database.connect()
  try {
    await connection.query('BEGIN TRANSACTION')
    const table = identifier(request.table)
    const description = await connection.query(`DESCRIBE SELECT * FROM ${table}`)
    const columns = description
      .toArray()
      .map((row) => columnSchema.parse(row))
      .map((row) => ({ name: row.column_name, type: row.column_type }))
    if (columns.length > 200)
      throw new Error('This table has more than 200 columns. Select a narrower view to profile.')
    if (request.columns.some((name) => !columns.some((column) => column.name === name)))
      throw new Error('A requested column is not in this table.')
    if (request.categoryColumn && !request.columns.includes(request.categoryColumn))
      throw new Error('Include the category column in columns.')
    const total = await connection.query(`SELECT count(*) AS count FROM ${table}`)
    const rows = z.object({ count }).parse(total.toArray()[0]).count
    const summaries: TableProfile['summaries'][number][] = []
    for (const name of [...new Set(request.columns)]) {
      signal.throwIfAborted()
      const column = columns.find((item) => item.name === name)
      if (!column) throw new Error('Column disappeared during profiling.')
      const quoted = identifier(name)
      const ordered = isNumericDuckDbType(column.type) || /^(DATE|TIMESTAMP|TIME)/.test(column.type)
      const range = ordered
        ? `CAST(min(${quoted}) AS VARCHAR) AS minimum, CAST(max(${quoted}) AS VARCHAR) AS maximum`
        : 'NULL::VARCHAR AS minimum, NULL::VARCHAR AS maximum'
      const stats = await connection.query(
        `SELECT count(*) - count(${quoted}) AS nulls, count(DISTINCT ${quoted}) AS distinct, ${range} FROM ${table}`,
      )
      const summary = summarySchema.parse(stats.toArray()[0])
      const categories =
        request.categoryColumn === name
          ? (
              await connection.query(
                `SELECT left(CAST(${quoted} AS VARCHAR), 120) AS value, count(*) AS count FROM ${table} GROUP BY ${quoted} ORDER BY count DESC, value NULLS LAST LIMIT 20`,
              )
            )
              .toArray()
              .map((row) => categorySchema.parse(row))
          : []
      const formats =
        column.type === 'VARCHAR'
          ? (
              await connection.query(
                `SELECT regexp_replace(regexp_replace(left(${quoted}, 80), '[0-9]', '9', 'g'), '[A-Za-z]', 'A', 'g') AS pattern, count(*) AS count FROM ${table} WHERE ${quoted} IS NOT NULL GROUP BY pattern ORDER BY count DESC, pattern LIMIT 8`,
              )
            )
              .toArray()
              .map((row) => formatSchema.parse(row))
          : []
      summaries.push({ column: name, ...summary, categories, formats })
    }
    const checks = request.checks?.length
      ? await checkPreparation(connection, tables, request.table, request.checks, signal)
      : undefined
    signal.throwIfAborted()
    await connection.query('COMMIT')
    return { table: request.table, rows, columns, summaries, ...(checks ? { checks } : {}) }
  } finally {
    await connection.close()
  }
}
