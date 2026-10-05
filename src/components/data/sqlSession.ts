import { createStore } from 'zustand/vanilla'
import type { Terminal } from 'xterm'
import { tableFromIPC } from 'apache-arrow'
import { isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { PREPARED_VIEW, type SqlPreparationInput, type SqlViewName } from '@/domain/sqlPreparation'
import { selectSqlDerivedSource, type SelectedSource, type SqlResume } from '@/domain/workflow'
import {
  cancelSqlPreparationQuery,
  closeSqlPreparation,
  describeSqlPreparationProblem,
  listSqlPreparationViews,
  materializePreparedView,
  openSqlPreparation,
  replayDefinitionsInShell,
  type SqlPreparationProblem,
  type SqlPreparationSession,
} from '@/data/sqlPreparation'
import {
  bridgeSoftKeyboard,
  consoleTheme,
  disposeTerminal,
  embedThemed,
  onThemeChange,
} from './sqlTerminal'

type Shell =
  | { readonly kind: 'opening' }
  | { readonly kind: 'ready'; readonly session: SqlPreparationSession }
  | { readonly kind: 'materializing'; readonly session: SqlPreparationSession }
  | { readonly kind: 'running-script'; readonly session: SqlPreparationSession }
  | { readonly kind: 'failed-to-open'; readonly problem: SqlPreparationProblem }
  | {
      readonly kind: 'materialization-refused'
      readonly session: SqlPreparationSession
      readonly problem: SqlPreparationProblem
    }
type Cancellation =
  | { readonly kind: 'idle' }
  | { readonly kind: 'requesting' }
  | { readonly kind: 'reported'; readonly message: string }
type Output =
  | { readonly kind: 'checking' }
  | { readonly kind: 'none' }
  | {
      readonly kind: 'available'
      readonly views: NonEmptyArray<SqlViewName>
      readonly selected: SqlViewName
    }
  | { readonly kind: 'failed'; readonly problem: SqlPreparationProblem }
type Script =
  | { readonly kind: 'closed' }
  | { readonly kind: 'reading'; readonly name: string }
  | { readonly kind: 'editing'; readonly name: string; readonly text: string }
  | { readonly kind: 'failed'; readonly name: string; readonly detail: string }
type ScriptResult =
  | { readonly kind: 'none' }
  | { readonly kind: 'failed'; readonly detail: string }
  | {
      readonly kind: 'complete'
      readonly count: number
      readonly columns: readonly string[]
      readonly rows: readonly (readonly string[])[]
    }
interface State {
  readonly query: 'idle' | 'running'
  readonly script: Script
  readonly scriptResult: ScriptResult
  readonly shell: Shell
  readonly cancellation: Cancellation
  readonly output: Output
  readonly size: { readonly cols: number; readonly rows: number } | null
}

/** Owns the database and terminal together; editor removal only detaches the host. */
export function createSqlSession(inputs: readonly SqlPreparationInput[], resume: SqlResume | null) {
  const store = createStore<State>(() => ({
    query: 'idle',
    scriptResult: { kind: 'none' },
    script: { kind: 'closed' },
    shell: { kind: 'opening' },
    cancellation: { kind: 'idle' },
    output: { kind: 'checking' },
    size: null,
  }))
  let reading = 0
  let closed = true
  let epoch = 0
  let listing = 0
  let session: SqlPreparationSession | null = null
  let host: HTMLDivElement | null = null
  let mount: HTMLDivElement | null = null
  let terminal: Terminal | null = null
  let observer: ResizeObserver | null = null
  let unwatch: (() => void) | null = null
  let unbridge: (() => void) | null = null
  const valid = (generation: number) => !closed && epoch === generation
  const measure = () => {
    if (host === null || !host.isConnected || terminal === null) return
    host.dispatchEvent(new UIEvent('resize'))
    const size = { cols: terminal.cols, rows: terminal.rows }
    const previous = store.getState().size
    if (previous?.cols !== size.cols || previous.rows !== size.rows) store.setState({ size })
  }
  const refresh = async (prefer: SqlViewName | null = null) => {
    if (closed || session === null) return
    const generation = epoch
    const request = ++listing
    const listed = await listSqlPreparationViews(session)
    if (!valid(generation) || listing !== request) return
    if (!listed.ok) {
      store.setState({ output: { kind: 'failed', problem: listed.error } })
      return
    }
    if (!isNonEmpty(listed.value)) {
      store.setState({ output: { kind: 'none' } })
      return
    }
    const views = listed.value
    const current = store.getState().output
    const candidates = [
      prefer,
      current.kind === 'available' ? current.selected : null,
      PREPARED_VIEW,
    ]
    const selected =
      candidates
        .map((name) => views.find((view) => view === name))
        .find((name) => name !== undefined) ?? views[0]
    store.setState({ output: { kind: 'available', views, selected } })
  }
  const open = () => {
    if (!closed) return
    closed = false
    const generation = ++epoch
    store.setState({
      shell: { kind: 'opening' },
      output: { kind: 'checking' },
      cancellation: { kind: 'idle' },
      size: null,
    })
    const element = document.createElement('div')
    element.style.cssText = 'width:100%;height:100%;min-width:0'
    host = element
    mount?.append(element)
    void (async () => {
      const opened = await openSqlPreparation(inputs, (running) => {
        if (valid(generation))
          store.setState({
            query: running ? 'running' : 'idle',
            ...(running ? { cancellation: { kind: 'idle' as const } } : {}),
          })
      })
      if (!valid(generation)) {
        if (opened.ok) await closeSqlPreparation(opened.value)
        return
      }
      if (!opened.ok) {
        store.setState({ shell: { kind: 'failed-to-open', problem: opened.error } })
        return
      }
      session = opened.value
      const embedded = await embedThemed(element, async () => {
        if (!valid(generation)) throw new Error('SQL session closed during initialization')
        return opened.value.shellDatabase
      })
      if (!valid(generation)) {
        if (embedded !== null) disposeTerminal(embedded)
        element.onresize = null
        element.replaceChildren()
        return
      }
      terminal = embedded
      observer = new ResizeObserver(measure)
      observer.observe(element)
      if (embedded !== null) {
        unwatch = onThemeChange(() => {
          embedded.options.theme = consoleTheme()
        })
        unbridge = bridgeSoftKeyboard(embedded)
      }
      measure()
      if (resume !== null) {
        const replayed = await replayDefinitionsInShell(opened.value, resume.statement)
        if (!valid(generation)) return
        if (!replayed.ok) {
          store.setState({
            shell: { kind: 'ready', session: opened.value },
            output: { kind: 'failed', problem: replayed.error },
          })
          return
        }
      }
      if (!valid(generation)) return
      store.setState({ shell: { kind: 'ready', session: opened.value } })
      await refresh(resume?.outputView ?? null)
    })().catch((cause) => {
      if (!valid(generation)) return
      store.setState({
        shell: {
          kind: 'failed-to-open',
          problem: { kind: 'engine-unavailable', detail: String(cause) },
        },
      })
    })
  }
  const dispose = () => {
    if (closed) return
    closed = true
    epoch++
    reading++
    listing++
    observer?.disconnect()
    unwatch?.()
    unbridge?.()
    if (terminal !== null) disposeTerminal(terminal)
    if (host !== null) {
      host.onresize = null
      host.remove()
      host.replaceChildren()
    }
    if (session !== null) void closeSqlPreparation(session)
    session = null
    terminal = null
    host = null
    observer = null
    unwatch = null
    unbridge = null
    store.setState({
      query: 'idle',
      shell: { kind: 'opening' },
      output: { kind: 'checking' },
      cancellation: { kind: 'idle' },
      size: null,
    })
  }
  const attach = (element: HTMLDivElement) => {
    mount = element
    if (host !== null) element.append(host)
    measure()
    return () => {
      if (mount !== element) return
      host?.remove()
      mount = null
    }
  }
  const select = (name: string) => {
    const output = store.getState().output
    if (closed || output.kind !== 'available' || store.getState().shell.kind === 'materializing')
      return
    const selected = output.views.find((view) => view === name)
    if (selected !== undefined) store.setState({ output: { ...output, selected } })
  }
  const cancel = async () => {
    if (closed || session === null || store.getState().cancellation.kind === 'requesting') return
    const generation = epoch
    store.setState({ cancellation: { kind: 'requesting' } })
    const result = await cancelSqlPreparationQuery(session)
    if (!valid(generation)) return
    store.setState({
      cancellation: {
        kind: 'reported',
        message: result.ok
          ? result.value
            ? 'DuckDB accepted the cancellation request.'
            : 'No query was running.'
          : describeSqlPreparationProblem(result.error),
      },
    })
  }
  const adopt = async (): Promise<SelectedSource | null> => {
    const { shell, output } = store.getState()
    if (
      closed ||
      session === null ||
      (shell.kind !== 'ready' && shell.kind !== 'materialization-refused') ||
      output.kind !== 'available'
    )
      return null
    const generation = epoch
    const live = session
    store.setState({ shell: { kind: 'materializing', session: live } })
    const materialized = await materializePreparedView(live, output.selected)
    if (!valid(generation)) return null
    if (!materialized.ok) {
      store.setState({
        shell: { kind: 'materialization-refused', session: live, problem: materialized.error },
      })
      return null
    }
    const selected = selectSqlDerivedSource(materialized.value.file, materialized.value.recipe)
    if (!selected.ok) {
      store.setState({
        shell: {
          kind: 'materialization-refused',
          session: live,
          problem: { kind: 'materialization-failed', detail: selected.error.kind },
        },
      })
      return null
    }
    store.setState({ shell: { kind: 'ready', session: live } })
    return selected.value
  }
  const closeScript = () => {
    reading++
    store.setState({ script: { kind: 'closed' } })
  }
  const loadScript = async (file: File) => {
    if (closed) return
    const request = ++reading
    const generation = epoch
    store.setState({ script: { kind: 'reading', name: file.name }, scriptResult: { kind: 'none' } })
    try {
      const text = await file.text()
      if (!valid(generation) || reading !== request) return
      store.setState({ script: { kind: 'editing', name: file.name, text } })
    } catch (cause) {
      if (valid(generation) && reading === request)
        store.setState({ script: { kind: 'failed', name: file.name, detail: String(cause) } })
    }
  }
  const editScript = (text: string) =>
    store.setState((state) =>
      state.script.kind === 'editing' ? { script: { ...state.script, text } } : state,
    )
  const runScript = async () => {
    const { script, shell } = store.getState()
    if (
      closed ||
      session === null ||
      session.shellConnection.current === null ||
      script.kind !== 'editing' ||
      !script.text.trim() ||
      (shell.kind !== 'ready' && shell.kind !== 'materialization-refused')
    )
      return
    const generation = epoch
    const live = session
    const connection = session.shellConnection.current
    store.setState({
      shell: { kind: 'running-script', session: live },
      scriptResult: { kind: 'none' },
    })
    try {
      // The shell's existing cancellable batch path lets DuckDB parse the complete file.
      const result = tableFromIPC(await live.shellDatabase.runQuery(connection, script.text))
      if (!valid(generation)) return
      const columns = result.schema.fields.map((field) => field.name)
      const rows = Array.from({ length: Math.min(200, result.numRows) }, (_, row) =>
        columns.map((_, column) => {
          const value = result.getChildAt(column)?.get(row)
          return value === null || value === undefined ? 'NULL' : String(value)
        }),
      )
      store.setState({ scriptResult: { kind: 'complete', count: result.numRows, columns, rows } })
    } catch (cause) {
      if (valid(generation))
        store.setState({ scriptResult: { kind: 'failed', detail: String(cause) } })
    } finally {
      if (valid(generation)) {
        store.setState({ shell: { kind: 'ready', session: live } })
        await refresh()
      }
    }
  }
  return {
    store,
    open,
    dispose,
    attach,
    refresh,
    select,
    cancel,
    adopt,
    loadScript,
    editScript,
    closeScript,
    runScript,
    active: () => !closed,
  }
}
export type SqlController = ReturnType<typeof createSqlSession>
