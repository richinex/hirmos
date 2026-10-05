import { createStore } from 'zustand/vanilla'
import type * as duckdb from '@duckdb/duckdb-wasm'
import { err, ok, type Result } from '@/domain/dop'
import type { PipelineBlockId } from '@/domain/pipeline'
import {
  parsePythonEvent,
  type PythonCommand,
  type PythonEvent,
  type PythonInput,
} from '@/workers/pythonProtocol'
import type { ScriptFailure, ScriptRun, ScriptRuntime } from './pipeline'

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
/** A blocked or stalled download never rejects, so the wait for the runtime needs its own deadline. */
export const PYTHON_LOAD_LIMIT_MS = 2 * 60_000
/** Completes the sentence the panel opens with. The cause is the network, not the script. */
export const PYTHON_UNREACHABLE =
  'the download did not finish. A VPN or a firewall can block it. Check the connection, then reload the page.'

type RunEvent = Extract<PythonEvent, { readonly kind: 'ran' | 'run-failed' }>

interface RuntimeSnapshot {
  readonly runtime: PythonRuntimeState
  readonly active: (ActivePythonRun & { readonly request: string }) | null
}

const identifier = (value: string): string => '"' + value.replaceAll('"', '""') + '"'
const arrowOf = async (
  connection: duckdb.AsyncDuckDBConnection,
  view: string,
): Promise<PythonInput> => ({
  format: 'arrow-file',
  bytes: await connection.useUnsafe((bindings, id) =>
    bindings.runQuery(id, 'SELECT * FROM ' + identifier(view)),
  ),
})

/** Owns one worker and its pending requests; snapshots contain status, never Arrow buffers. */
export function createPythonRuntime(
  makeWorker: () => Worker = () =>
    new Worker(new URL('../workers/python.worker.ts', import.meta.url), {
      type: 'module',
      name: 'hirmos-python',
    }),
  loadLimitMs: number = PYTHON_LOAD_LIMIT_MS,
) {
  const store = createStore<RuntimeSnapshot>(() => ({ runtime: { kind: 'idle' }, active: null }))
  let worker: Worker | null = null
  let closed = false
  const pending = new Map<string, (event: RunEvent) => void>()
  const current = (request: string) => !closed && store.getState().active?.request === request
  const failAll = (detail: string) => {
    const waiting = [...pending]
    pending.clear()
    for (const [request, resolve] of waiting)
      resolve({ kind: 'run-failed', request, detail, stdout: '' })
  }
  const recycle = (detail: string, runtime: PythonRuntimeState = { kind: 'idle' }) => {
    const previous = worker
    worker = null
    previous?.terminate()
    failAll(detail)
    store.setState({ active: null, runtime })
  }
  const pythonWorker = (): Worker => {
    if (closed) throw new Error('The script session was closed.')
    if (worker !== null) return worker
    const created = makeWorker()
    worker = created
    const stopped = (detail: string) => {
      if (worker !== created) return
      recycle(detail, { kind: 'failed', detail })
    }
    created.onmessage = (message: MessageEvent<unknown>) => {
      if (worker !== created || closed) return
      const parsed = parsePythonEvent(message.data)
      if (!parsed.ok) {
        stopped(parsed.error.detail)
        return
      }
      const event = parsed.value
      switch (event.kind) {
        case 'loading':
          store.setState({ runtime: { kind: 'loading', detail: event.detail } })
          return
        case 'ready':
          settleLoading()
          store.setState({ runtime: { kind: 'ready', python: event.python } })
          return
        case 'start-failed':
          settleLoading()
          stopped(event.detail)
          return
        case 'ran':
        case 'run-failed': {
          const resolve = pending.get(event.request)
          pending.delete(event.request)
          resolve?.(event)
          return
        }
        default: {
          const exhaustive: never = event
          return exhaustive
        }
      }
    }
    created.onerror = (event) => stopped(event.message || 'The Python worker stopped.')
    created.onmessageerror = () => stopped('The Python worker sent an unreadable message.')
    return created
  }
  const send = (command: PythonCommand, transfer: readonly ArrayBuffer[] = []) =>
    pythonWorker().postMessage(command, [...transfer])
  let loading: ReturnType<typeof setTimeout> | null = null
  const settleLoading = () => {
    if (loading !== null) {
      clearTimeout(loading)
      loading = null
    }
  }
  const warm = () => {
    if (closed) return
    const state = store.getState().runtime
    if (state.kind !== 'idle' && state.kind !== 'failed') return
    store.setState({ runtime: { kind: 'loading', detail: 'Loading Python' } })
    settleLoading()
    loading = setTimeout(() => {
      loading = null
      if (closed || store.getState().runtime.kind !== 'loading') return
      recycle(PYTHON_UNREACHABLE, { kind: 'failed', detail: PYTHON_UNREACHABLE })
    }, loadLimitMs)
    try {
      send({ kind: 'start' })
    } catch (cause) {
      recycle(String(cause), { kind: 'failed', detail: String(cause) })
    }
  }
  const cancel = () => {
    if (store.getState().active === null) return
    recycle('The script was cancelled.')
    warm()
  }
  const scripts: ScriptRuntime = {
    async run(step, connection): Promise<Result<ScriptRun, ScriptFailure>> {
      if (closed) return err({ detail: 'The script session was closed.', stdout: '' })
      if (store.getState().active !== null)
        return err({ detail: 'Another script is running in this session.', stdout: '' })
      const request = crypto.randomUUID()
      store.setState({ active: { request, step: step.id, startedAt: Date.now() } })
      try {
        let inputs: PythonInput[]
        try {
          inputs = await Promise.all(step.inputs.map((view) => arrowOf(connection, view)))
        } catch (cause) {
          return err({
            detail: 'The inputs could not be handed to Python: ' + String(cause),
            stdout: '',
          })
        }
        if (!current(request))
          return err({ detail: 'The script was cancelled or its session closed.', stdout: '' })
        const limit = window.setTimeout(() => {
          if (!pending.has(request)) return
          recycle('The script ran for more than five minutes and was stopped.')
          warm()
        }, PYTHON_RUN_LIMIT_MS)
        const event = await new Promise<RunEvent>((resolve) => {
          pending.set(request, resolve)
          try {
            send(
              { kind: 'run', request, code: step.code, inputs },
              inputs.map((input) => input.bytes.buffer as ArrayBuffer),
            )
          } catch (cause) {
            recycle(String(cause), { kind: 'failed', detail: String(cause) })
          }
        })
        window.clearTimeout(limit)
        if (event.kind === 'run-failed') return err({ detail: event.detail, stdout: event.stdout })
        if (!current(request))
          return err({
            detail: 'The script was cancelled or its session closed.',
            stdout: event.stdout,
          })
        if (event.columns.length === 0)
          return err({ detail: 'prepared has no columns', stdout: event.stdout })
        const staging = `script_result_${request.replaceAll('-', '')}`
        try {
          // Validate insertion before replacing the last successful result. A failed
          // conversion must not leave a partially populated result behind.
          await connection.insertArrowFromIPCStream(event.prepared.bytes, { name: staging })
          if (!current(request)) throw new Error('The script was cancelled or its session closed.')
          const table = `script_${step.id.replaceAll('"', '')}`
          await connection.query('BEGIN TRANSACTION')
          try {
            await connection.query(
              `CREATE OR REPLACE TABLE ${identifier(table)} AS SELECT * FROM ${identifier(staging)}`,
            )
            await connection.query(
              `CREATE OR REPLACE VIEW ${identifier(step.view)} AS SELECT * FROM ${identifier(table)}`,
            )
            if (!current(request))
              throw new Error('The script was cancelled or its session closed.')
            await connection.query('COMMIT')
          } catch (cause) {
            await connection.query('ROLLBACK')
            throw cause
          }
          return current(request)
            ? ok({ stdout: event.stdout, shape: event.shape })
            : err({ detail: 'The script session was closed.', stdout: event.stdout })
        } catch (cause) {
          return err({
            detail: `prepared could not be read back: ${cause instanceof Error ? cause.message : String(cause)}`,
            stdout: event.stdout,
          })
        } finally {
          await connection
            .query(`DROP TABLE IF EXISTS ${identifier(staging)}`)
            .catch(() => undefined)
        }
      } finally {
        if (current(request)) store.setState({ active: null })
      }
    },
  }
  return {
    store,
    scripts,
    warm,
    cancel,
    activate: () => {
      closed = false
    },
    dispose: () => {
      closed = true
      recycle('The script session was closed.')
    },
  }
}
