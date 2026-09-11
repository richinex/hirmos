import { PYODIDE_INDEX_URL, PYTHON_PACKAGES, type PythonCommand, type PythonEvent } from './pythonProtocol'

/**
 * Pyodide in a worker, so a long pandas script never blocks the page. The runtime and its wheels
 * are fetched on the first `start`; every `run` writes the inputs to Pyodide's file system, runs the
 * script with `inputs`, `pd` and `np` in scope, and sends back whatever the script assigned to
 * `prepared`.
 */

interface PyodideRuntime {
  readonly FS: { writeFile(path: string, data: Uint8Array | string): void; mkdirTree(path: string): void; unlink(path: string): void }
  loadPackage(names: readonly string[]): Promise<unknown>
  runPython(code: string): unknown
  globals: { get(name: string): unknown }
  readonly version: string
}

interface PyodideModule { loadPyodide(options: { indexURL: string }): Promise<PyodideRuntime> }

interface Harness { (code: string, paths: readonly string[]): unknown }

/**
 * The Python side. It answers with data, never an exception: ['ok', csv, rows, columns, stdout] or
 * ['error', message, stdout]. Whole-number columns with gaps stay whole numbers (numpy_nullable), a
 * named index such as groupby keys comes back as columns, an unnamed one is dropped, and a
 * two-level header is flattened, since the CSV that goes back to DuckDB has one header row.
 */
const HARNESS = `
import contextlib
import io
import traceback
import pandas as pd
import numpy as np

def _hirmos_run(code, input_paths):
    printed = io.StringIO()
    inputs = [pd.read_csv(path, dtype_backend='numpy_nullable') for path in input_paths]
    namespace = {'inputs': inputs, 'pd': pd, 'np': np, '__name__': '__hirmos_script__'}
    try:
        with contextlib.redirect_stdout(printed):
            exec(compile(code, '<script>', 'exec'), namespace)
    except SyntaxError as error:
        return ['error', f'line {error.lineno}: SyntaxError: {error.msg}', printed.getvalue()]
    except Exception as error:
        frames = [frame for frame in traceback.extract_tb(error.__traceback__) if frame.filename == '<script>']
        where = f'line {frames[-1].lineno}: ' if frames else ''
        message = ' '.join(str(error).split())
        return ['error', f'{where}{type(error).__name__}: {message}', printed.getvalue()]
    if 'prepared' not in namespace:
        return ['error', 'the script did not assign prepared', printed.getvalue()]
    prepared = namespace['prepared']
    if isinstance(prepared, pd.Series):
        prepared = prepared.to_frame()
    if not isinstance(prepared, pd.DataFrame):
        return ['error', f'prepared must be a DataFrame, not {type(prepared).__name__}', printed.getvalue()]
    if any(name is not None for name in prepared.index.names):
        prepared = prepared.reset_index()
    if isinstance(prepared.columns, pd.MultiIndex):
        prepared = prepared.set_axis(['_'.join(str(part) for part in column if str(part)) for column in prepared.columns], axis=1)
    return ['ok', prepared.to_csv(index=False), int(len(prepared)), [str(column) for column in prepared.columns], printed.getvalue()]
`

const post = (event: PythonEvent, transfer: readonly ArrayBuffer[] = []): void => { self.postMessage(event, [...transfer]) }

interface Loaded { readonly pyodide: PyodideRuntime; readonly harness: Harness; readonly python: string }

let runtime: Promise<Loaded> | null = null

const start = (): Promise<Loaded> => {
  if (runtime !== null) {
    void runtime.then((loaded) => post({ kind: 'ready', python: loaded.python }))
    return runtime
  }
  runtime = (async () => {
    post({ kind: 'loading', detail: 'Loading Python' })
    const module = (await import(/* @vite-ignore */ `${PYODIDE_INDEX_URL}pyodide.mjs`)) as PyodideModule
    const pyodide = await module.loadPyodide({ indexURL: PYODIDE_INDEX_URL })
    post({ kind: 'loading', detail: 'Loading pandas and numpy' })
    await pyodide.loadPackage(PYTHON_PACKAGES)
    pyodide.runPython(HARNESS)
    pyodide.FS.mkdirTree('/inputs')
    const python = String(pyodide.runPython('import sys; sys.version.split()[0]'))
    post({ kind: 'ready', python })
    return { pyodide, harness: pyodide.globals.get('_hirmos_run') as Harness, python }
  })()
  runtime.catch(() => { runtime = null })
  return runtime
}

const describe = (cause: unknown): string => {
  const raw = cause instanceof Error ? cause.message : String(cause)
  return raw.trim().split('\n').filter((line) => line.length > 0).slice(-1)[0] ?? raw
}

type HarnessAnswer = readonly ['ok', string, number, readonly string[], string] | readonly ['error', string, string]

const run = async (command: Extract<PythonCommand, { readonly kind: 'run' }>): Promise<void> => {
  let loaded: Loaded
  try {
    loaded = await start()
  } catch (cause) {
    post({ kind: 'run-failed', request: command.request, detail: `the Python runtime could not load: ${describe(cause)}`, stdout: '' })
    return
  }
  const { pyodide, harness } = loaded
  const paths = command.inputs.map((_, index) => `/inputs/${command.request}-${index}.csv`)
  try {
    command.inputs.forEach((input, index) => pyodide.FS.writeFile(paths[index]!, input))
    const proxy = harness(command.code, paths) as { toJs(): HarnessAnswer; destroy(): void }
    const answer = proxy.toJs()
    proxy.destroy()
    if (answer[0] === 'error') { post({ kind: 'run-failed', request: command.request, detail: answer[1], stdout: answer[2] }); return }
    const [, csv, rows, columns, stdout] = answer
    const prepared = new TextEncoder().encode(csv)
    post({ kind: 'ran', request: command.request, prepared, rows, columns: [...columns], stdout }, [prepared.buffer])
  } catch (cause) {
    post({ kind: 'run-failed', request: command.request, detail: describe(cause), stdout: '' })
  } finally {
    for (const path of paths) { try { pyodide.FS.unlink(path) } catch { /* the input was never written */ } }
  }
}

self.onmessage = (message: MessageEvent<PythonCommand>) => {
  const command = message.data
  if (command.kind === 'start') { start().catch((cause: unknown) => post({ kind: 'start-failed', detail: describe(cause) })); return }
  void run(command)
}
