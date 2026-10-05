import {
  PYODIDE_INDEX_URL,
  PYTHON_PACKAGES,
  type PythonCommand,
  type PythonEvent,
  type ScriptShape,
} from './pythonProtocol'
import harnessCode from './pythonScript.py?raw'

interface PyodideRuntime {
  readonly FS: {
    writeFile(path: string, data: Uint8Array | string): void
    readFile(path: string): Uint8Array
    unlink(path: string): void
  }
  loadPackage(names: readonly string[]): Promise<unknown>
  runPython(code: string): unknown
  readonly globals: { get(name: string): unknown }
}
interface PyodideModule {
  loadPyodide(options: { indexURL: string }): Promise<PyodideRuntime>
}
type HarnessShape = 'table' | 'value'
type HarnessAnswer =
  | readonly ['ok', number, readonly string[], string, HarnessShape]
  | readonly ['error', string, string]
interface Harness {
  (
    code: string,
    paths: readonly string[],
    output: string,
  ): { toJs(): HarnessAnswer; destroy(): void }
}
interface Loaded {
  readonly pyodide: PyodideRuntime
  readonly harness: Harness
  readonly python: string
}

const post = (event: PythonEvent, transfer: readonly ArrayBuffer[] = []): void => {
  self.postMessage(event, [...transfer])
}
let runtime: Promise<Loaded> | null = null

const start = (): Promise<Loaded> => {
  if (runtime !== null) {
    void runtime
      .then((loaded) => post({ kind: 'ready', python: loaded.python }))
      .catch(() => undefined)
    return runtime
  }
  runtime = (async () => {
    post({ kind: 'loading', detail: 'Loading Python' })
    const module = (await import(
      /* @vite-ignore */ `${PYODIDE_INDEX_URL}pyodide.mjs`
    )) as PyodideModule
    const pyodide = await module.loadPyodide({ indexURL: PYODIDE_INDEX_URL })
    post({ kind: 'loading', detail: 'Loading pandas, NumPy and PyArrow' })
    await pyodide.loadPackage(PYTHON_PACKAGES)
    pyodide.runPython(harnessCode)
    const python = String(pyodide.runPython('import sys; sys.version.split()[0]'))
    post({ kind: 'ready', python })
    return { pyodide, harness: pyodide.globals.get('_hirmos_run') as Harness, python }
  })()
  runtime.catch(() => {
    runtime = null
  })
  return runtime
}

const describe = (cause: unknown): string => {
  const raw = cause instanceof Error ? cause.message : String(cause)
  return (
    raw
      .trim()
      .split('\n')
      .filter((line) => line.length > 0)
      .slice(-1)[0] ?? raw
  )
}

const run = async (command: Extract<PythonCommand, { readonly kind: 'run' }>): Promise<void> => {
  let loaded: Loaded
  try {
    loaded = await start()
  } catch (cause) {
    post({
      kind: 'run-failed',
      request: command.request,
      detail: `the Python runtime could not load: ${describe(cause)}`,
      stdout: '',
    })
    return
  }
  const { pyodide, harness } = loaded
  const paths = command.inputs.map((_, index) => `/${command.request}-in-${index}.arrow`)
  const output = `/${command.request}-out.arrow`
  try {
    command.inputs.forEach((input, index) => pyodide.FS.writeFile(paths[index]!, input.bytes))
    const proxy = harness(command.code, paths, output)
    let answer: HarnessAnswer
    try {
      answer = proxy.toJs()
    } finally {
      proxy.destroy()
    }
    if (answer[0] === 'error') {
      post({ kind: 'run-failed', request: command.request, detail: answer[1], stdout: answer[2] })
      return
    }
    const [, rows, columns, stdout, shape] = answer
    const bytes = pyodide.FS.readFile(output)
    // A value is the single cell of its own one-column table, so the column carries over unchanged.
    const reported: ScriptShape =
      shape === 'value' && columns[0] !== undefined
        ? { kind: 'value', column: columns[0] }
        : { kind: 'table' }
    post(
      {
        kind: 'ran',
        request: command.request,
        prepared: { format: 'arrow-stream', bytes },
        rows,
        columns: [...columns],
        stdout,
        shape: reported,
      },
      [bytes.buffer as ArrayBuffer],
    )
  } catch (cause) {
    post({ kind: 'run-failed', request: command.request, detail: describe(cause), stdout: '' })
  } finally {
    for (const path of [...paths, output]) {
      try {
        pyodide.FS.unlink(path)
      } catch {
        /* The run may have failed before creating the file. */
      }
    }
  }
}

self.onmessage = (message: MessageEvent<PythonCommand>) => {
  switch (message.data.kind) {
    case 'start':
      start().catch((cause: unknown) => post({ kind: 'start-failed', detail: describe(cause) }))
      return
    case 'run':
      void run(message.data)
      return
    default: {
      const exhaustive: never = message.data
      return exhaustive
    }
  }
}
