import { createStore } from 'zustand/vanilla'
import type { Terminal } from 'xterm'
import { isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { PREPARED_VIEW, type SqlPreparationInput, type SqlViewName } from '@/domain/sqlPreparation'
import { selectSqlDerivedSource, type SelectedSource, type SqlResume } from '@/domain/workflow'
import { cancelSqlPreparationQuery, closeSqlPreparation, describeSqlPreparationProblem, listSqlPreparationViews, materializePreparedView, openSqlPreparation, replayDefinitionsInShell, type SqlPreparationProblem, type SqlPreparationSession } from '@/data/sqlPreparation'
import { bridgeSoftKeyboard, consoleTheme, disposeTerminal, embedThemed, onThemeChange } from './sqlTerminal'

type Shell =
  | { readonly kind: 'opening' }
  | { readonly kind: 'ready'; readonly session: SqlPreparationSession }
  | { readonly kind: 'materializing'; readonly session: SqlPreparationSession }
  | { readonly kind: 'failed-to-open'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'materialization-refused'; readonly session: SqlPreparationSession; readonly problem: SqlPreparationProblem }
type Cancellation = { readonly kind: 'idle' } | { readonly kind: 'requesting' } | { readonly kind: 'reported'; readonly message: string }
type Output =
  | { readonly kind: 'checking' }
  | { readonly kind: 'none' }
  | { readonly kind: 'available'; readonly views: NonEmptyArray<SqlViewName>; readonly selected: SqlViewName }
  | { readonly kind: 'failed'; readonly problem: SqlPreparationProblem }
interface State {
  readonly shell: Shell
  readonly cancellation: Cancellation
  readonly output: Output
  readonly size: { readonly cols: number; readonly rows: number } | null
}

/** Owns the database and terminal together; editor removal only detaches the host. */
export function createSqlSession(inputs: NonEmptyArray<SqlPreparationInput>, resume: SqlResume | null) {
  const store = createStore<State>(() => ({ shell: { kind: 'opening' }, cancellation: { kind: 'idle' }, output: { kind: 'checking' }, size: null }))
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
    if (!listed.ok) { store.setState({ output: { kind: 'failed', problem: listed.error } }); return }
    if (!isNonEmpty(listed.value)) { store.setState({ output: { kind: 'none' } }); return }
    const views = listed.value
    const current = store.getState().output
    const candidates = [prefer, current.kind === 'available' ? current.selected : null, PREPARED_VIEW]
    const selected = candidates.map(name => views.find(view => view === name)).find(name => name !== undefined) ?? views[0]
    store.setState({ output: { kind: 'available', views, selected } })
  }
  const open = () => {
    if (!closed) return
    closed = false
    const generation = ++epoch
    store.setState({ shell: { kind: 'opening' }, output: { kind: 'checking' }, cancellation: { kind: 'idle' }, size: null })
    const element = document.createElement('div')
    element.style.cssText = 'width:100%;height:100%;min-width:0'
    host = element
    mount?.append(element)
    void (async () => {
      const opened = await openSqlPreparation(inputs)
      if (!valid(generation)) { if (opened.ok) await closeSqlPreparation(opened.value); return }
      if (!opened.ok) { store.setState({ shell: { kind: 'failed-to-open', problem: opened.error } }); return }
      session = opened.value
      const embedded = await embedThemed(element, async () => {
        if (!valid(generation)) throw new Error('SQL session closed during initialization')
        return opened.value.shellDatabase
      })
      if (!valid(generation)) { if (embedded !== null) disposeTerminal(embedded); element.onresize = null; element.replaceChildren(); return }
      terminal = embedded
      observer = new ResizeObserver(measure)
      observer.observe(element)
      if (embedded !== null) {
        unwatch = onThemeChange(() => { embedded.options.theme = consoleTheme() })
        unbridge = bridgeSoftKeyboard(embedded)
      }
      measure()
      if (resume !== null) {
        const replayed = await replayDefinitionsInShell(opened.value, resume.statement)
        if (!valid(generation)) return
        if (!replayed.ok) {
          store.setState({ shell: { kind: 'ready', session: opened.value }, output: { kind: 'failed', problem: replayed.error } })
          return
        }
      }
      if (!valid(generation)) return
      store.setState({ shell: { kind: 'ready', session: opened.value } })
      await refresh(resume?.outputView ?? null)
    })().catch(cause => {
      if (!valid(generation)) return
      store.setState({ shell: { kind: 'failed-to-open', problem: { kind: 'engine-unavailable', detail: String(cause) } } })
    })
  }
  const dispose = () => {
    if (closed) return
    closed = true
    epoch++
    listing++
    observer?.disconnect()
    unwatch?.()
    unbridge?.()
    if (terminal !== null) disposeTerminal(terminal)
    if (host !== null) { host.onresize = null; host.remove(); host.replaceChildren() }
    if (session !== null) void closeSqlPreparation(session)
    session = null
    terminal = null
    host = null
    observer = null
    unwatch = null
    unbridge = null
    store.setState({ shell: { kind: 'opening' }, output: { kind: 'checking' }, cancellation: { kind: 'idle' }, size: null })
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
    if (closed || output.kind !== 'available' || store.getState().shell.kind === 'materializing') return
    const selected = output.views.find(view => view === name)
    if (selected !== undefined) store.setState({ output: { ...output, selected } })
  }
  const cancel = async () => {
    if (closed || session === null || store.getState().cancellation.kind === 'requesting') return
    const generation = epoch
    store.setState({ cancellation: { kind: 'requesting' } })
    const result = await cancelSqlPreparationQuery(session)
    if (!valid(generation)) return
    store.setState({ cancellation: { kind: 'reported', message: result.ok ? (result.value ? 'DuckDB accepted the cancellation request.' : 'No query was running.') : describeSqlPreparationProblem(result.error) } })
  }
  const adopt = async (): Promise<SelectedSource | null> => {
    const { shell, output } = store.getState()
    if (closed || session === null || (shell.kind !== 'ready' && shell.kind !== 'materialization-refused') || output.kind !== 'available') return null
    const generation = epoch
    const live = session
    store.setState({ shell: { kind: 'materializing', session: live } })
    const materialized = await materializePreparedView(live, output.selected)
    if (!valid(generation)) return null
    if (!materialized.ok) { store.setState({ shell: { kind: 'materialization-refused', session: live, problem: materialized.error } }); return null }
    const selected = selectSqlDerivedSource(materialized.value.file, materialized.value.recipe)
    if (!selected.ok) { store.setState({ shell: { kind: 'materialization-refused', session: live, problem: { kind: 'materialization-failed', detail: selected.error.kind } } }); return null }
    store.setState({ shell: { kind: 'ready', session: live } })
    return selected.value
  }
  return { store, open, dispose, attach, refresh, select, cancel, adopt, active: () => !closed }
}
export type SqlController = ReturnType<typeof createSqlSession>
