import type * as duckdb from '@duckdb/duckdb-wasm'
import { isolatedDuckDbEngine, previewCell, type DuckDbEngine } from './duckdb'
import { describeSqlPreparationProblem, materializeView, prepareSqlInputs, recoverSqlInputs, redeclareInput, registerInputs, type SqlPreparationProblem } from './sqlPreparation'
import type { PreviewCell } from '@/domain/dataset'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { compileDraft, compilePipeline, describePipelineProblem, type CompiledPipeline, type PipelineBlockId, type PipelineGraph, type PipelineProblem, type PipelineRecipe, type PipelineStep } from '@/domain/pipeline'
import type { ScriptShape } from '@/workers/pythonProtocol'
import { inputDescriptor, type SqlInputAlias, type SqlPreparationInput } from '@/domain/sourceInputs'
import type { DeclaredType } from '@/domain/fileReading'

export type PipelineRunProblem =
  | { readonly kind: 'engine-unavailable'; readonly detail: string }
  | { readonly kind: 'input'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'wiring'; readonly problem: PipelineProblem }
  | { readonly kind: 'block-failed'; readonly id: PipelineBlockId; readonly detail: string }
  | { readonly kind: 'preview-failed'; readonly id: PipelineBlockId; readonly detail: string }
  | { readonly kind: 'materialization'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'session-closed' }
  | { readonly kind: 'unknown-input'; readonly alias: SqlInputAlias }

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
  | { readonly kind: 'ran'; readonly rowCount: number; readonly columns: NonEmptyArray<PreviewColumn>; readonly stdout?: string; readonly shape?: ScriptShape }
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
export interface ScriptRun { readonly stdout: string; readonly shape: ScriptShape }
export interface ScriptFailure { readonly detail: string; readonly stdout: string }

export interface ScriptRuntime {
  run(step: Extract<PipelineStep, { readonly kind: 'script' }>, connection: duckdb.AsyncDuckDBConnection, database: duckdb.AsyncDuckDB): Promise<Result<ScriptRun, ScriptFailure>>
}

export interface PipelineSession {
  readonly engine: DuckDbEngine
  /** The files registered so far, each a DuckDB view under its alias; input blocks add and replace them from the canvas. */
  readonly inputs: SqlPreparationInput[]
  readonly lifecycle: { current: 'open' | 'closed' }
  readonly scripts: ScriptRuntime
  readonly namespace: string
}

const PREVIEW_ROWS = 8
/** DuckDB's message without its category prefix and the statement echo after it; the block's SQL is in the inspector already. */
const detailOf = (cause: unknown): string => {
  const raw = cause instanceof Error ? cause.message : String(cause)
  const line = raw.indexOf('\nLINE ')
  return (line === -1 ? raw : raw.slice(0, line)).replace(/^[A-Za-z ]+ Error: /, '').trim()
}
const identifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

export async function openPipeline(inputs: readonly SqlPreparationInput[], scripts: ScriptRuntime): Promise<Result<PipelineSession, PipelineRunProblem>> {
  let engine: DuckDbEngine
  try { engine = await isolatedDuckDbEngine() } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
  const namespace = `pipeline-${crypto.randomUUID()}`
  try {
    const registered = isNonEmpty(inputs) ? await registerInputs(engine, inputs, namespace) : ok(null)
    if (registered.ok) return ok({ engine, inputs: [...inputs], lifecycle: { current: 'open' }, scripts, namespace })
    await engine.db.terminate()
    return err({ kind: 'input', problem: registered.error })
  } catch (cause) {
    await engine.db.terminate().catch(() => undefined)
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
}

/** Registers one more file under an alias no other input uses, and returns the input the block now refers to. */
export async function addPipelineInput(session: PipelineSession, file: File): Promise<Result<SqlPreparationInput, PipelineRunProblem>> {
  if (session.lifecycle.current === 'closed') return err({ kind: 'session-closed' })
  const prepared = await prepareSqlInputs([file], new Set(session.inputs.map((input) => input.alias as string)))
  if (!prepared.ok) return err({ kind: 'input', problem: prepared.error })
  const input = prepared.value[0]
  const registered = await registerInputs(session.engine, [input], session.namespace)
  if (!registered.ok) return err({ kind: 'input', problem: registered.error })
  session.inputs.push(input)
  return ok(input)
}

/** Reads a registered file again with one column's declaration changed, under the same alias. */
export async function redeclarePipelineInput(session: PipelineSession, alias: SqlInputAlias, column: string, type: DeclaredType | null): Promise<Result<SqlPreparationInput, PipelineRunProblem>> {
  if (session.lifecycle.current === 'closed') return err({ kind: 'session-closed' })
  const index = session.inputs.findIndex((input) => input.alias === alias)
  if (index === -1) return err({ kind: 'unknown-input', alias })
  const updated = await redeclareInput(session.engine, session.namespace, session.inputs[index]!, column, type)
  if (!updated.ok) return err({ kind: 'input', problem: updated.error })
  session.inputs[index] = updated.value
  return ok(updated.value)
}

/** Drops a registered file's view and forgets it; the block that held it is the caller's to update. */
export async function removePipelineInput(session: PipelineSession, alias: SqlInputAlias): Promise<void> {
  const index = session.inputs.findIndex((input) => input.alias === alias)
  if (index === -1 || session.lifecycle.current === 'closed') return
  const [removed] = session.inputs.splice(index, 1)
  await withConnection(session, async (connection) => {
    await connection.query(`DROP VIEW IF EXISTS ${identifier(alias)}`)
    return ok(null)
  })
  await session.engine.db.dropFile(`${session.namespace}-${removed!.fingerprint}.${removed!.format}`).catch(() => undefined)
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
    const describedInputs = await Promise.all(inputs.map((node) => describeView(connection, draft.views.get(node.id) ?? '')))
    inputs.forEach((node, index) => {
      const described = describedInputs[index]!
      outcomes.set(node.id, described.ok ? { kind: 'ran', ...described.value } : { kind: 'failed', detail: `the input file could not be read: ${described.error.detail}` })
    })
    const stopped = new Set<PipelineBlockId>()
    for (const step of draft.steps) {
      if (step.inputIds.some((from) => stopped.has(from))) { outcomes.set(step.id, { kind: 'skipped' }); stopped.add(step.id); continue }
      const fail = (detail: string, stdout?: string) => { outcomes.set(step.id, { kind: 'failed', detail, stdout }); stopped.add(step.id) }
      let stdout: string | undefined
      // Only a script can report a single value; every other block produces a table.
      let shape: ScriptShape | undefined
      if (step.kind === 'script') {
        const ran = await session.scripts.run(step, connection, session.engine.db)
        if (!ran.ok) { fail(ran.error.detail, ran.error.stdout); continue }
        stdout = ran.value.stdout
        shape = ran.value.shape
      } else {
        try {
          await connection.query(step.statement)
        } catch (cause) {
          fail(detailOf(cause))
          continue
        }
      }
      const described = await describeView(connection, step.view)
      if (described.ok) outcomes.set(step.id, { kind: 'ran', ...described.value, stdout, shape })
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
  const compiled = compilePipeline(graph, new Set(session.inputs.map((input) => input.alias as string)))
  if (!compiled.ok) return err({ kind: 'wiring', problem: compiled.error })
  const required = compiled.value.views
  const sourceGraph = { nodes: graph.nodes.filter((node) => required.has(node.id)), edges: graph.edges.filter((edge) => required.has(edge.to)) }
  const ran = await runPipeline(session, sourceGraph)
  if (!ran.ok) return ran
  if (!ran.value.complete.ok) return err({ kind: 'wiring', problem: ran.value.complete.error })
  for (const [id, outcome] of ran.value.outcomes) {
    if (outcome.kind === 'failed') return err({ kind: 'block-failed', id, detail: outcome.detail })
  }
  const outputView = ran.value.complete.value.outputView
  return withConnection(session, async (connection) => {
    const materialized = await materializeView(session.engine, connection, outputView, 'pipeline_prepared.parquet')
    if (!materialized.ok) return err({ kind: 'materialization', problem: materialized.error })
    // The recipe records the files the graph reads, not every file that was ever chosen.
    const used = new Set(sourceGraph.nodes.flatMap((node) => node.block.kind === 'input' && node.block.file.kind === 'chosen' ? [node.block.file.alias as string] : []))
    const descriptors = session.inputs.filter((input) => used.has(input.alias)).map(inputDescriptor)
    return ok({ ...materialized.value, recipe: { kind: 'pipeline-derived', graph, inputs: descriptors } })
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
  const matched = await recoverSqlInputs(recipe.inputs, files)
  if (!matched.ok) return err({ kind: 'input', problem: matched.error })
  const session = await openPipeline(matched.value, scripts)
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
    case 'unknown-input': return `No input file is named ${problem.alias}.`
    default: { const exhaustive: never = problem; return exhaustive }
  }
}
