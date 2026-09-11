import type * as duckdb from '@duckdb/duckdb-wasm'
import { err, ok, type Result } from '@/domain/dop'
import type { PipelineBlockId, PipelineStep } from '@/domain/pipeline'
import { parsePythonEvent, type PythonCommand, type PythonEvent } from '@/workers/pythonProtocol'
import type { ScriptFailure, ScriptRun, ScriptRuntime } from './pipeline'

/**
 * The page side of the Python worker: one worker for the page, started on the first script run or
 * on request, its state observable so the inspector can say "Loading pandas and numpy". A script
 * step's inputs are read out of DuckDB as CSV, run in the worker, and the result registered back
 * into DuckDB as the step's view.
 */

export type PythonRuntimeState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'loading'; readonly detail: string }
  | { readonly kind: 'ready'; readonly python: string }
  | { readonly kind: 'failed'; readonly detail: string }

/** The script step running right now, so a card and the inspector can show it and offer to stop it. */
export interface ActivePythonRun {
  readonly step: PipelineBlockId
  readonly startedAt: number
}

/** A script that runs this long is taken for a runaway and stopped. */
export const PYTHON_RUN_LIMIT_MS = 5 * 60_000

type ScriptStep = Extract<PipelineStep, { readonly kind: 'script' }>
type RunEvent = Extract<PythonEvent, { readonly kind: 'ran' | 'run-failed' }>

let worker: Worker | null = null
let state: PythonRuntimeState = { kind: 'idle' }
let active: ActivePythonRun | null = null
const listeners = new Set<() => void>()
const pending = new Map<string, (event: RunEvent) => void>()

const notify = (): void => { for (const listener of listeners) listener() }
const setState = (next: PythonRuntimeState): void => { state = next; notify() }
const setActive = (next: ActivePythonRun | null): void => { active = next; notify() }

export const pythonRuntimeState = (): PythonRuntimeState => state
export const activePythonRun = (): ActivePythonRun | null => active

export const subscribePythonRuntime = (listener: () => void): (() => void) => {
  listeners.add(listener)
  return () => { listeners.delete(listener) }
}

const failAll = (detail: string): void => {
  for (const [request, resolve] of pending) resolve({ kind: 'run-failed', request, detail, stdout: '' })
  pending.clear()
}

/** Stops whatever is running by discarding the worker; the next script starts a fresh runtime. */
const recycle = (detail: string): void => {
  failAll(detail)
  worker?.terminate()
  worker = null
  setActive(null)
  setState({ kind: 'idle' })
}

/** Cancelling discards the worker and starts a fresh one at once, so the next run does not pay the load. */
export const cancelPythonRun = (): void => {
  if (active === null) return
  recycle('The script was cancelled.')
  warmPythonRuntime()
}

const pythonWorker = (): Worker => {
  if (worker !== null) return worker
  const created = new Worker(new URL('../workers/python.worker.ts', import.meta.url), { type: 'module', name: 'hirmos-python' })
  created.onmessage = (message: MessageEvent<unknown>) => {
    const parsed = parsePythonEvent(message.data)
    if (!parsed.ok) { failAll(parsed.error.detail); setState({ kind: 'failed', detail: parsed.error.detail }); return }
    const event = parsed.value
    switch (event.kind) {
      case 'loading': setState({ kind: 'loading', detail: event.detail }); return
      case 'ready': setState({ kind: 'ready', python: event.python }); return
      case 'start-failed': setState({ kind: 'failed', detail: event.detail }); failAll(event.detail); return
      case 'ran':
      case 'run-failed': {
        const resolve = pending.get(event.request)
        pending.delete(event.request)
        resolve?.(event)
        return
      }
      default: { const exhaustive: never = event; return exhaustive }
    }
  }
  const stopped = (detail: string) => {
    failAll(detail)
    created.terminate()
    if (worker === created) worker = null
    setActive(null)
    setState({ kind: 'failed', detail })
  }
  created.onerror = (event) => stopped(event.message || 'the Python worker stopped')
  created.onmessageerror = () => stopped('the Python worker sent a message the page could not read')
  worker = created
  return created
}

const send = (command: PythonCommand, transfer: readonly ArrayBuffer[] = []): void => { pythonWorker().postMessage(command, [...transfer]) }

/** Starts loading the runtime now, so the first script does not wait for twenty megabytes. */
export const warmPythonRuntime = (): void => {
  if (state.kind === 'idle' || state.kind === 'failed') { setState({ kind: 'loading', detail: 'Loading Python' }); send({ kind: 'start' }) }
}

const identifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const csvOf = async (connection: duckdb.AsyncDuckDBConnection, database: duckdb.AsyncDuckDB, view: string, file: string): Promise<Uint8Array> => {
  try {
    await connection.query(`COPY (SELECT * FROM ${identifier(view)}) TO '${file}' (FORMAT CSV, HEADER)`)
    return await database.copyFileToBuffer(file)
  } finally {
    await database.dropFile(file).catch(() => undefined)
  }
}

export const pythonScriptRuntime = (): ScriptRuntime => ({
  async run(step: ScriptStep, connection, database): Promise<Result<ScriptRun, ScriptFailure>> {
    const request = crypto.randomUUID()
    let inputs: Uint8Array[]
    try {
      inputs = await Promise.all(step.inputs.map((view, index) => csvOf(connection, database, view, `${request}-in-${index}.csv`)))
    } catch (cause) {
      return err({ detail: `the inputs could not be handed to Python: ${cause instanceof Error ? cause.message : String(cause)}`, stdout: '' })
    }
    setActive({ step: step.id, startedAt: Date.now() })
    const limit = window.setTimeout(() => {
      if (!pending.has(request)) return
      recycle('The script ran for more than five minutes and was stopped.')
      warmPythonRuntime()
    }, PYTHON_RUN_LIMIT_MS)
    const event = await new Promise<RunEvent>((resolve) => {
      pending.set(request, resolve)
      send({ kind: 'run', request, code: step.code, inputs }, inputs.map((input) => input.buffer as ArrayBuffer))
    })
    window.clearTimeout(limit)
    if (active?.step === step.id) setActive(null)
    if (event.kind === 'run-failed') return err({ detail: event.detail, stdout: event.stdout })
    if (event.columns.length === 0) return err({ detail: 'prepared has no columns', stdout: event.stdout })
    const file = `${request}-out.csv`
    try {
      await database.registerFileBuffer(file, event.prepared)
      // The rows land in a table of their own and the step's view reads it, so every step stays a view
      // whatever kind it is, and a rerun that renumbers the steps never finds a table where a view should be.
      const table = `script_${step.id.replaceAll('"', '')}`
      await connection.query(`CREATE OR REPLACE TABLE ${identifier(table)} AS SELECT * FROM read_csv('${file}', header = true)`)
      await connection.query(`CREATE OR REPLACE VIEW ${identifier(step.view)} AS SELECT * FROM ${identifier(table)}`)
      return ok({ stdout: event.stdout })
    } catch (cause) {
      return err({ detail: `prepared could not be read back: ${cause instanceof Error ? cause.message : String(cause)}`, stdout: event.stdout })
    } finally {
      await database.dropFile(file).catch(() => undefined)
    }
  },
})
