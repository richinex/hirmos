import type * as duckdb from '@duckdb/duckdb-wasm'
import { isolatedDuckDbEngine, previewCell, type DuckDbEngine } from './duckdb'
import { describeSqlPreparationProblem, materializeView, prepareSqlInputs, registerInputs, type SqlPreparationProblem } from './sqlPreparation'
import type { PreviewCell } from '@/domain/dataset'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { compileDraft, compilePipeline, describePipelineProblem, type CompiledPipeline, type PipelineBlockId, type PipelineGraph, type PipelineProblem, type PipelineRecipe, type PipelineStep } from '@/domain/pipeline'
import { inputDescriptor, type SqlPreparationInput } from '@/domain/sourceInputs'

export type PipelineRunProblem =
  | { readonly kind: 'engine-unavailable'; readonly detail: string }
  | { readonly kind: 'input'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'wiring'; readonly problem: PipelineProblem }
  | { readonly kind: 'block-failed'; readonly id: PipelineBlockId; readonly detail: string }
  | { readonly kind: 'preview-failed'; readonly id: PipelineBlockId; readonly detail: string }
  | { readonly kind: 'materialization'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'session-closed' }

export interface PreviewColumn {
  readonly name: string
  readonly type: string
}

export interface BlockPreview {
  readonly id: PipelineBlockId
  readonly rowCount: number
  readonly columns: NonEmptyArray<PreviewColumn>
  /** One array per row, in column order. */
  readonly rows: readonly (readonly PreviewCell[])[]
}

export type BlockOutcome =
  | { readonly kind: 'ran'; readonly rowCount: number; readonly columns: NonEmptyArray<PreviewColumn>; readonly stdout?: string }
  | { readonly kind: 'failed'; readonly detail: string; readonly stdout?: string }
  | { readonly kind: 'skipped' }
  | { readonly kind: 'waiting'; readonly detail: string }

export interface PipelineRun {
  /** Views by block for every block that ran, so a preview can read them. */
  readonly views: ReadonlyMap<PipelineBlockId, string>
  readonly outcomes: ReadonlyMap<PipelineBlockId, BlockOutcome>
  /** The whole pipeline, when every block is wired through to the output; otherwise why not. */
  readonly complete: Result<CompiledPipeline, PipelineProblem>
}

/**
 * Runs a script step: given the tables named by `inputs`, read from the connection, it registers the
 * rows the script assigned to `prepared` under `view`. The Python worker implements this.
 */
export interface ScriptRun { readonly stdout: string }
export interface ScriptFailure { readonly detail: string; readonly stdout: string }

export interface ScriptRuntime {
  run(step: Extract<PipelineStep, { readonly kind: 'script' }>, connection: duckdb.AsyncDuckDBConnection, database: duckdb.AsyncDuckDB): Promise<Result<ScriptRun, ScriptFailure>>
}

export interface PipelineSession {
  readonly engine: DuckDbEngine
  readonly inputs: NonEmptyArray<SqlPreparationInput>
  readonly lifecycle: { current: 'open' | 'closed' }
  readonly scripts: ScriptRuntime
}

const PREVIEW_ROWS = 8
/** DuckDB's message without its category prefix and the statement echo after it; the block's SQL is in the inspector already. */
const detailOf = (cause: unknown): string => {
  const raw = cause instanceof Error ? cause.message : String(cause)
  const line = raw.indexOf('\nLINE ')
  return (line === -1 ? raw : raw.slice(0, line)).replace(/^[A-Za-z ]+ Error: /, '').trim()
}
const identifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

export async function openPipeline(inputs: NonEmptyArray<SqlPreparationInput>, scripts: ScriptRuntime): Promise<Result<PipelineSession, PipelineRunProblem>> {
  let engine: DuckDbEngine
  try { engine = await isolatedDuckDbEngine() } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
  try {
    const registered = await registerInputs(engine, inputs, `pipeline-${crypto.randomUUID()}`)
    if (registered.ok) return ok({ engine, inputs, lifecycle: { current: 'open' }, scripts })
    await engine.db.terminate()
    return err({ kind: 'input', problem: registered.error })
  } catch (cause) {
    await engine.db.terminate().catch(() => undefined)
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
}

/** One connection for the work, closed afterwards; a session closed under it reports that instead of throwing. */
async function withConnection<Value>(session: PipelineSession, work: (connection: duckdb.AsyncDuckDBConnection) => Promise<Result<Value, PipelineRunProblem>>): Promise<Result<Value, PipelineRunProblem>> {
  const lifecycle: { readonly current: 'open' | 'closed' } = session.lifecycle
  if (lifecycle.current === 'closed') return err({ kind: 'session-closed' })
  const failure = (cause: unknown): Result<never, PipelineRunProblem> =>
    lifecycle.current === 'closed' ? err({ kind: 'session-closed' }) : err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  let connection: duckdb.AsyncDuckDBConnection
  try { connection = await session.engine.db.connect() } catch (cause) { return failure(cause) }
  try {
    return await work(connection)
  } catch (cause) {
    return failure(cause)
  } finally {
    await connection.close().catch(() => undefined)
  }
}

export async function closePipeline(session: PipelineSession): Promise<void> {
  if (session.lifecycle.current === 'closed') return
  session.lifecycle.current = 'closed'
  await session.engine.db.terminate()
}

type Described = { readonly rowCount: number; readonly columns: NonEmptyArray<PreviewColumn> }

/** A view's columns and row count. A view can be created and still fail to run, since DuckDB binds columns when the view is read; that failure is the detail. */
async function describeView(connection: duckdb.AsyncDuckDBConnection, view: string): Promise<Result<Described, { readonly detail: string }>> {
  try {
    const [description, countTable] = await Promise.all([
      connection.query(`DESCRIBE SELECT * FROM ${identifier(view)}`),
      connection.query(`SELECT count(*)::UBIGINT AS rows FROM ${identifier(view)}`),
    ])
    const columns = Array.from({ length: description.numRows }, (_, index) => ({
      name: String(description.getChild('column_name')?.get(index) ?? ''),
      type: String(description.getChild('column_type')?.get(index) ?? ''),
    }))
    if (!isNonEmpty(columns)) return err({ detail: 'the block produced no columns' })
    const raw = countTable.getChild('rows')?.get(0)
    return ok({ rowCount: typeof raw === 'bigint' ? Number(raw) : Number(raw ?? 0), columns })
  } catch (cause) {
    return err({ detail: detailOf(cause) })
  }
}

/**
 * Runs every step in order on one connection. A step that fails stops the run there: its block
 * carries the message, the blocks after it are marked skipped, and the blocks before it keep their
 * outcomes, so the canvas can show where the pipeline broke and the last good preview stays readable.
 */
export async function runPipeline(session: PipelineSession, graph: PipelineGraph): Promise<Result<PipelineRun, PipelineRunProblem>> {
  const aliases = new Set(session.inputs.map((input) => input.alias as string))
  const draft = compileDraft(graph, aliases)
  const complete = compilePipeline(graph, aliases)
  const outcomes = new Map<PipelineBlockId, BlockOutcome>()
  for (const [id, detail] of draft.waiting) outcomes.set(id, { kind: 'waiting', detail })
  return withConnection(session, async (connection) => {
    const inputs = graph.nodes.filter((node) => node.block.kind === 'input' && draft.views.has(node.id))
    const describedInputs = await Promise.all(inputs.map((node) => describeView(connection, node.block.kind === 'input' ? node.block.alias : '')))
    inputs.forEach((node, index) => {
      const described = describedInputs[index]!
      outcomes.set(node.id, described.ok ? { kind: 'ran', ...described.value } : { kind: 'failed', detail: `the input file could not be read: ${described.error.detail}` })
    })
    const stopped = new Set<PipelineBlockId>()
    for (const step of draft.steps) {
      if (step.inputIds.some((from) => stopped.has(from))) { outcomes.set(step.id, { kind: 'skipped' }); stopped.add(step.id); continue }
      const fail = (detail: string, stdout?: string) => { outcomes.set(step.id, { kind: 'failed', detail, stdout }); stopped.add(step.id) }
      let stdout: string | undefined
      if (step.kind === 'script') {
        const ran = await session.scripts.run(step, connection, session.engine.db)
        if (!ran.ok) { fail(ran.error.detail, ran.error.stdout); continue }
        stdout = ran.value.stdout
      } else {
        try {
          await connection.query(step.statement)
        } catch (cause) {
          fail(detailOf(cause))
          continue
        }
      }
      const described = await describeView(connection, step.view)
      if (described.ok) outcomes.set(step.id, { kind: 'ran', ...described.value, stdout })
      else fail(described.error.detail, stdout)
    }
    return ok({ views: draft.views, outcomes, complete })
  })
}

/** The first rows of a block that ran; its columns and count are already known from the run. */
export async function previewBlock(session: PipelineSession, view: string, id: PipelineBlockId, known: Described): Promise<Result<BlockPreview, PipelineRunProblem>> {
  return withConnection(session, async (connection) => {
    try {
      const table = await connection.query(`SELECT * FROM ${identifier(view)} LIMIT ${PREVIEW_ROWS}`)
      const rows: PreviewCell[][] = []
      for (let row = 0; row < table.numRows; row += 1) {
        rows.push(known.columns.map((column) => previewCell(table.getChild(column.name)?.get(row), column.type)))
      }
      return ok({ id, rowCount: known.rowCount, columns: known.columns, rows })
    } catch (cause) {
      return err({ kind: 'preview-failed', id, detail: detailOf(cause) })
    }
  })
}

export interface PipelineOutput {
  readonly file: File
  readonly recipe: PipelineRecipe
  readonly rowCount: number
  readonly columns: NonEmptyArray<PreviewColumn>
}

export async function materializePipeline(session: PipelineSession, graph: PipelineGraph): Promise<Result<PipelineOutput, PipelineRunProblem>> {
  const ran = await runPipeline(session, graph)
  if (!ran.ok) return ran
  if (!ran.value.complete.ok) return err({ kind: 'wiring', problem: ran.value.complete.error })
  for (const [id, outcome] of ran.value.outcomes) {
    if (outcome.kind === 'failed') return err({ kind: 'block-failed', id, detail: outcome.detail })
  }
  const outputView = ran.value.complete.value.outputView
  return withConnection(session, async (connection) => {
    const materialized = await materializeView(session.engine, connection, outputView, 'pipeline_prepared.parquet')
    if (!materialized.ok) return err({ kind: 'materialization', problem: materialized.error })
    const [first, ...rest] = session.inputs
    return ok({ ...materialized.value, recipe: { kind: 'pipeline-derived', graph, inputs: [inputDescriptor(first), ...rest.map(inputDescriptor)] } })
  })
}

/**
 * Builds the recorded source again from the recipe: the same input files, matched by fingerprint and
 * given the aliases the blocks refer to, the blocks run in order, and the output written. The caller
 * checks the file against the recorded source fingerprint, as it does for a file chosen by hand.
 */
export async function replayPipelineRecipe(
  recipe: PipelineRecipe,
  files: readonly File[],
  scripts: ScriptRuntime,
): Promise<Result<File, PipelineRunProblem>> {
  const offered = await prepareSqlInputs(files)
  if (!offered.ok) return err({ kind: 'input', problem: offered.error })
  const matched: SqlPreparationInput[] = []
  const missing: string[] = []
  for (const descriptor of recipe.inputs) {
    const input = offered.value.find((candidate) => candidate.fingerprint === descriptor.fingerprint)
    if (input === undefined) missing.push(descriptor.fileName)
    else matched.push({ ...input, alias: descriptor.alias })
  }
  if (missing.length > 0 || !isNonEmpty(matched)) return err({ kind: 'input', problem: { kind: 'replay-inputs-missing', fileNames: missing } })
  const session = await openPipeline(matched, scripts)
  if (!session.ok) return session
  try {
    const output = await materializePipeline(session.value, recipe.graph)
    return output.ok ? ok(output.value.file) : output
  } finally {
    await closePipeline(session.value)
  }
}

export function describePipelineRunProblem(problem: PipelineRunProblem, name: (id: PipelineBlockId) => string): string {
  switch (problem.kind) {
    case 'engine-unavailable': return `DuckDB could not start: ${problem.detail}`
    case 'input': return describeSqlPreparationProblem(problem.problem)
    case 'wiring': return describePipelineProblem(problem.problem, name)
    case 'block-failed': return `${name(problem.id)} failed: ${problem.detail}`
    case 'preview-failed': return `${name(problem.id)} could not be previewed: ${problem.detail}`
    case 'materialization': return `The output could not be written: ${describeSqlPreparationProblem(problem.problem)}`
    case 'session-closed': return 'The pipeline session has been closed. Choose the input files again.'
    default: { const exhaustive: never = problem; return exhaustive }
  }
}
