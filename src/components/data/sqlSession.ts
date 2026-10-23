import { createStore } from 'zustand/vanilla'
import type { PreparationDraft } from '@/data/preparationDraftStore'
import type { Terminal } from 'xterm'
import { tableFromIPC } from 'apache-arrow'
import { assertNever, isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { PREPARED_VIEW, type SqlPreparationInput, type SqlViewName } from '@/domain/sqlPreparation'
import { changesCatalog } from '@/domain/sqlStatement'
import { selectSqlDerivedSource, type SelectedSource, type SqlResume } from '@/domain/workflow'
import {
  cancelSqlPreparationQuery,
  closeSqlPreparation,
  describeSqlPreparationProblem,
  listSqlPreparationViews,
  materializePreparedView,
  openSqlPreparation,
  readSqlCatalog,
  readSqlRelation,
  replayDefinitionsInShell,
  summarizeSqlRelation,
  type SqlPreparationProblem,
  type SqlPreparationSession,
} from '@/data/sqlPreparation'
import {
  catalogEffect,
  inspected,
  inspecting as inspectingState,
  type SqlCatalogState,
  type SqlInspection,
  type SqlRelation,
} from '@/domain/sqlCatalog'
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
  readonly draft: Extract<PreparationDraft, { kind: 'sql' }> | null
  readonly query: 'idle' | 'running'
  readonly script: Script
  readonly scriptResult: ScriptResult
  readonly shell: Shell
  readonly cancellation: Cancellation
  readonly output: Output
  readonly catalog: SqlCatalogState
  readonly inspection: SqlInspection
  readonly size: { readonly cols: number; readonly rows: number } | null
}

/** Owns the database and terminal together; editor removal only detaches the host. */
export function createSqlSession(inputs: readonly SqlPreparationInput[], resume: SqlResume | null) {
  const store = createStore<State>(() => ({
    draft: null,
    query: 'idle',
    scriptResult: { kind: 'none' },
    script: { kind: 'closed' },
    shell: { kind: 'opening' },
    cancellation: { kind: 'idle' },
    output: { kind: 'checking' },
    catalog: { kind: 'checking' },
    inspection: { kind: 'none' },
    size: null,
  }))
  let reading = 0
  let closed = true
  let epoch = 0
  let listing = 0
  let inspecting = 0
  let opened = false
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
  const inspect = async (relation: SqlRelation, inPlace = false) => {
    if (closed || session === null) return
    const generation = epoch
    const request = ++inspecting
    store.setState({ inspection: inspectingState(store.getState().inspection, relation, inPlace) })
    const read = await readSqlRelation(session, relation)
    if (!valid(generation) || inspecting !== request) return
    store.setState({ inspection: inspected(store.getState().inspection, relation, inPlace, read) })
  }
  const summarize = async () => {
    const inspection = store.getState().inspection
    if (closed || session === null || inspection.kind !== 'read') return
    if (inspection.summary.kind !== 'idle') return
    const generation = epoch
    const request = inspecting
    store.setState({ inspection: { ...inspection, summary: { kind: 'summarizing' } } })
    const summarized = await summarizeSqlRelation(
      session,
      inspection.relation,
      inspection.data.rowCount,
    )
    const current = store.getState().inspection
    if (!valid(generation) || inspecting !== request || current.kind !== 'read') return
    store.setState({
      inspection: {
        ...current,
        summary: summarized.ok
          ? { kind: 'summarized', byColumn: summarized.value }
          : { kind: 'failed', problem: summarized.error },
      },
    })
  }
  const catalogue = async (generation: number) => {
    if (session === null) return
    const read = await readSqlCatalog(session)
    if (!valid(generation)) return
    if (!read.ok) {
      store.setState({ catalog: { kind: 'failed', problem: read.error } })
      return
    }
    store.setState({ catalog: { kind: 'listed', relations: read.value } })
    const effect = catalogEffect(store.getState().inspection, read.value, !opened)
    opened = true
    switch (effect.kind) {
      case 'keep':
        return
      case 'clear':
        inspecting++
        store.setState({ inspection: { kind: 'none' } })
        return
      case 'read':
        await inspect(effect.relation, effect.inPlace)
        return
      default:
        return assertNever(effect)
    }
  }
  const refresh = async (prefer: SqlViewName | null = null) => {
    if (closed || session === null) return
    const generation = epoch
    const request = ++listing
    await catalogue(generation)
    if (!valid(generation) || listing !== request) return
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
    opened = false
    const generation = ++epoch
    store.setState({
      shell: { kind: 'opening' },
      output: { kind: 'checking' },
      catalog: { kind: 'checking' },
      inspection: { kind: 'none' },
      cancellation: { kind: 'idle' },
      size: null,
    })
    const element = document.createElement('div')
    element.style.cssText = 'width:100%;height:100%;min-width:0'
    host = element
    mount?.append(element)
    void (async () => {
      const opened = await openSqlPreparation(inputs, (running, sql) => {
        if (!valid(generation)) return
        store.setState({
          query: running ? 'running' : 'idle',
          ...(running ? { cancellation: { kind: 'idle' as const } } : {}),
        })
        if (!running && changesCatalog(sql)) void refresh()
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
    inspecting++
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
      catalog: { kind: 'checking' },
      inspection: { kind: 'none' },
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
      store.setState({
        script: { kind: 'editing', name: file.name, text },
        draft: { kind: 'sql', name: file.name, text, inputs },
      })
    } catch (cause) {
      if (valid(generation) && reading === request)
        store.setState({ script: { kind: 'failed', name: file.name, detail: String(cause) } })
    }
  }
  const editScript = (text: string) =>
    store.setState((state) =>
      state.script.kind === 'editing'
        ? {
            script: { ...state.script, text },
            draft: { kind: 'sql', name: state.script.name, text, inputs },
          }
        : state,
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
      store.setState({
        script: { kind: 'closed' },
        scriptResult: { kind: 'complete', count: result.numRows, columns, rows },
      })
    } catch (cause) {
      if (valid(generation))
        store.setState({ scriptResult: { kind: 'failed', detail: String(cause) } })
    } finally {
      if (valid(generation)) store.setState({ shell: { kind: 'ready', session: live } })
    }
  }
  return {
    store,
    open,
    dispose,
    attach,
    refresh,
    inspect,
    summarize,
    select,
    cancel,
    adopt,
    loadScript,
    editScript,
    closeScript,
    runScript,
    restoreDraft: (draft: Extract<PreparationDraft, { kind: 'sql' }>) => {
      dispose()
      inputs = draft.inputs
      resume = null
      store.setState({
        draft,
        script: { kind: 'editing', name: draft.name, text: draft.text },
        scriptResult: { kind: 'none' },
      })
      open()
    },
    active: () => !closed,
  }
}
export type SqlController = ReturnType<typeof createSqlSession>
