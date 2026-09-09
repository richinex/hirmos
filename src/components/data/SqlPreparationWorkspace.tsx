import { useCallback, useEffect, useRef, useState } from 'react'
import { embed } from '@duckdb/duckdb-wasm-shell'
import { Terminal, type ITheme } from 'xterm'
import shellModule from '@duckdb/duckdb-wasm-shell/dist/shell_bg.wasm?url'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { button, field, fieldHint, label, literal, num, prose } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { assertNever, isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { chapterLabel } from '@/domain/navigation'
import { PREPARED_VIEW, type SqlPreparationInput, type SqlViewName } from '@/domain/sqlPreparation'
import type { SelectedSource } from '@/domain/workflow'
import {
  cancelSqlPreparationQuery,
  closeSqlPreparation,
  describeSqlPreparationProblem,
  listSqlPreparationViews,
  materializePreparedView,
  openSqlPreparation,
  type SqlPreparationProblem,
  type SqlPreparationSession,
} from '@/data/sqlPreparation'

type ShellState =
  | { readonly kind: 'opening' }
  | { readonly kind: 'ready'; readonly session: SqlPreparationSession }
  | { readonly kind: 'materializing'; readonly session: SqlPreparationSession }
  | { readonly kind: 'failed-to-open'; readonly problem: SqlPreparationProblem }
  | { readonly kind: 'materialization-refused'; readonly session: SqlPreparationSession; readonly problem: SqlPreparationProblem }

type CancellationState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'requesting' }
  | { readonly kind: 'reported'; readonly message: string }

type OutputViewState =
  | { readonly kind: 'checking' }
  | { readonly kind: 'none' }
  | { readonly kind: 'available'; readonly views: NonEmptyArray<SqlViewName>; readonly selected: SqlViewName }
  | { readonly kind: 'failed'; readonly problem: SqlPreparationProblem }

type CopyState = 'idle' | 'copied' | 'refused'

interface SqlShellProps {
  readonly inputs: NonEmptyArray<SqlPreparationInput>
  readonly onPrepared: (source: SelectedSource) => void
  readonly onCleared: () => void
}

const starterSql = (inputs: NonEmptyArray<SqlPreparationInput>): string => {
  const [first] = inputs
  return [
    `CREATE OR REPLACE VIEW ${PREPARED_VIEW} AS`,
    'SELECT *',
    `FROM "${first.alias}";`,
  ].join('\n')
}

const token = (name: string, fallback: string): string => getComputedStyle(document.documentElement).getPropertyValue(name).trim() || fallback

/** True when the page is on its dark theme: the explicit choice on the root element, or the system setting when there is none. */
const pageIsDark = (): boolean => {
  const chosen = document.documentElement.getAttribute('data-theme')
  if (chosen === 'dark' || chosen === 'light') return chosen === 'dark'
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

/** Apple's ANSI palette and the light and dark macOS Terminal profile colours used by this console. */
const APPLE_ANSI = {
  black: '#000000', red: '#990000', green: '#00A600', yellow: '#999900', blue: '#0000B2', magenta: '#B200B2', cyan: '#00A6B2', white: '#BFBFBF',
  brightBlack: '#666666', brightRed: '#E50000', brightGreen: '#00D900', brightYellow: '#E5E500', brightBlue: '#0000FF', brightMagenta: '#E500E5', brightCyan: '#00E5E5', brightWhite: '#E5E5E5',
} as const

const TERMINAL_PROFILES = {
  basic: { background: '#FFFFFF', foreground: '#000000', cursor: '#7F7F7F', selectionBackground: '#B4D5FF' },
  pro: { background: '#000000', foreground: '#F2F2F2', cursor: '#4D4D4D', selectionBackground: '#414141' },
} as const

const terminalProfile = () => (pageIsDark() ? TERMINAL_PROFILES.pro : TERMINAL_PROFILES.basic)

/** Match the console background to the current page background. */
const consoleGround = (): string => token('--color-stage', terminalProfile().background)

const consoleTheme = (): ITheme => {
  const profile = terminalProfile()
  return {
    background: consoleGround(),
    foreground: profile.foreground,
    cursor: profile.cursor,
    cursorAccent: consoleGround(),
    selectionBackground: profile.selectionBackground,
    selectionForeground: profile.foreground,
    ...APPLE_ANSI,
  }
}

/** Capture the terminal created by the shell so its text and selection colours can follow the page theme. */
const embedThemed = async (host: HTMLDivElement, resolveDatabase: () => Promise<Awaited<ReturnType<Parameters<typeof embed>[0]['resolveDatabase']>>>): Promise<Terminal | null> => {
  let caught: Terminal | null = null
  const open = Terminal.prototype.open
  Terminal.prototype.open = function (this: Terminal, parent: HTMLElement) {
    if (parent === host) caught = this
    return open.call(this, parent)
  }
  try {
    await embed({
      shellModule,
      container: host,
      resolveDatabase,
      backgroundColor: consoleGround(),
      fontFamily: token('--font-mono', 'ui-monospace, monospace'),
    })
  } finally {
    Terminal.prototype.open = open
  }
  if (caught !== null) (caught as Terminal).options.theme = consoleTheme()
  return caught
}

/** Subscribe to the page theme and the system theme used when the page has no explicit selection. */
const onThemeChange = (listener: () => void): (() => void) => {
  const observer = new MutationObserver(listener)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme', 'class'] })
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  media.addEventListener('change', listener)
  return () => { observer.disconnect(); media.removeEventListener('change', listener) }
}

/** Display input and output controls beside the SQL console. */
export function SqlShell({ inputs, onPrepared, onCleared }: SqlShellProps) {
  const container = useRef<HTMLDivElement>(null)
  const [state, setState] = useState<ShellState>({ kind: 'opening' })
  const [cancellation, setCancellation] = useState<CancellationState>({ kind: 'idle' })
  const [outputView, setOutputView] = useState<OutputViewState>({ kind: 'checking' })
  const [copy, setCopy] = useState<CopyState>('idle')
  const [size, setSize] = useState<{ readonly cols: number; readonly rows: number } | null>(null)

  const refreshViews = useCallback(async (session: SqlPreparationSession) => {
    const listed = await listSqlPreparationViews(session)
    if (!listed.ok) { setOutputView({ kind: 'failed', problem: listed.error }); return }
    if (!isNonEmpty(listed.value)) { setOutputView({ kind: 'none' }); return }
    const views = listed.value
    setOutputView((current) => {
      const retained = current.kind === 'available' && views.includes(current.selected)
        ? current.selected
        : views.find((view) => view === PREPARED_VIEW) ?? views[0]
      return { kind: 'available', views, selected: retained }
    })
  }, [])

  useEffect(() => {
    let cancelled = false
    let observer: ResizeObserver | null = null
    let unwatchTheme: (() => void) | null = null
    let session: SqlPreparationSession | null = null
    const start = async () => {
      await Promise.resolve()
      if (cancelled || container.current === null) return
      const opened = await openSqlPreparation(inputs)
      if (!opened.ok) { setState({ kind: 'failed-to-open', problem: opened.error }); return }
      session = opened.value
      if (cancelled) { await closeSqlPreparation(opened.value); return }
      const host = container.current
      if (host === null) return
      try {
        const terminal = await embedThemed(host, async () => opened.value.shellDatabase)
        if (cancelled) return
        const measure = () => { if (terminal !== null) setSize({ cols: terminal.cols, rows: terminal.rows }) }
        observer = new ResizeObserver(() => { host.dispatchEvent(new UIEvent('resize')); measure() })
        observer.observe(host)
        measure()
        if (terminal !== null) unwatchTheme = onThemeChange(() => { terminal.options.theme = consoleTheme() })
        setState({ kind: 'ready', session: opened.value })
        await refreshViews(opened.value)
      } catch (cause) {
        setState({
          kind: 'failed-to-open',
          problem: { kind: 'engine-unavailable', detail: cause instanceof Error ? cause.message : String(cause) },
        })
      }
    }
    void start()
    return () => {
      cancelled = true
      observer?.disconnect()
      unwatchTheme?.()
      // The shell appends its terminal to the host; a later embed into the same host must start empty.
      container.current?.replaceChildren()
      if (session !== null) {
        void closeSqlPreparation(session).then((closed) => {
          if (!closed.ok) console.error(describeSqlPreparationProblem(closed.error))
        })
      }
    }
  }, [inputs, refreshViews])

  const sessionOf = (value: ShellState): SqlPreparationSession | null => {
    switch (value.kind) {
      case 'ready': return value.session
      case 'materializing': return value.session
      case 'materialization-refused': return value.session
      case 'opening': return null
      case 'failed-to-open': return null
      default: return assertNever(value)
    }
  }

  const usePreparedView = async () => {
    const session = sessionOf(state)
    if (session === null || state.kind === 'materializing' || outputView.kind !== 'available') return
    setState({ kind: 'materializing', session })
    const materialized = await materializePreparedView(session, outputView.selected)
    if (!materialized.ok) {
      setState({ kind: 'materialization-refused', session, problem: materialized.error })
      return
    }
    const { selectSqlDerivedSource } = await import('@/domain/workflow')
    const selected = selectSqlDerivedSource(materialized.value.file, materialized.value.recipe)
    if (!selected.ok) {
      setState({
        kind: 'materialization-refused',
        session,
        problem: { kind: 'materialization-failed', detail: selected.error.kind },
      })
      return
    }
    onPrepared(selected.value)
  }

  const cancelQuery = async () => {
    const session = sessionOf(state)
    if (session === null) return
    setCancellation({ kind: 'requesting' })
    const result = await cancelSqlPreparationQuery(session)
    if (!result.ok) {
      setCancellation({ kind: 'reported', message: describeSqlPreparationProblem(result.error) })
      return
    }
    setCancellation({
      kind: 'reported',
      message: result.value ? 'DuckDB accepted the cancellation request.' : 'No query was running.',
    })
  }

  const copyStarter = async () => {
    try {
      await navigator.clipboard.writeText(starterSql(inputs))
      setCopy('copied')
    } catch {
      setCopy('refused')
    }
    window.setTimeout(() => setCopy('idle'), 1500)
  }

  const session = sessionOf(state)
  const busy = state.kind === 'opening' || state.kind === 'materializing'
  const problem = state.kind === 'failed-to-open' || state.kind === 'materialization-refused'
    ? state.problem
    : null
  return (
    <div className="grid gap-6 lg:grid-cols-[20rem_minmax(0,1fr)] lg:grid-rows-[auto_1fr] lg:gap-x-8 lg:gap-y-4" aria-labelledby="sql-workspace-title">
      <div className="min-w-0">
        <span className={label('text-signal')}>{chapterLabel('data')}</span>
        <h2 id="sql-workspace-title" className="mb-3 mt-3 text-heading text-ink">Prepare with SQL</h2>
        <p className={prose('mb-4 mt-0 text-faint')}>Create one or more views, refresh the list, and choose the view that becomes the analysis source. The input files are not changed.</p>

        <p className="m-0 text-body font-medium text-ink">Input tables</p>
        <ul className="mb-4 mt-2 list-none space-y-2 p-0" aria-label="SQL inputs">
          {inputs.map((input) => (
            <li key={input.alias} className="min-w-0">
              <code className={literal('block truncate text-body text-ink')}>{input.alias}</code>
              <span className={num('block truncate text-label text-faint')}>{input.fileName}</span>
            </li>
          ))}
        </ul>

        <div className="mb-4">
          <span className="text-body font-medium text-ink">Output view</span>
          {outputView.kind === 'available' && (
            <Select aria-label="Output view" className={field('text', 'mt-2')} value={outputView.selected} onChange={(event) => {
              const selected = outputView.views.find((view) => view === event.target.value)
              if (selected !== undefined) setOutputView({ ...outputView, selected })
            }}>
              {outputView.views.map((view) => <option key={view} value={view}>{view}</option>)}
            </Select>
          )}
          {outputView.kind === 'checking' && <p className={cn(fieldHint, 'mt-2')}>Listing the views.</p>}
          {outputView.kind === 'none' && <p className={cn(fieldHint, 'mt-2')}>No view yet. Run a <code className={literal()}>CREATE VIEW</code> statement in the console, then refresh the list. Input tables are not offered.</p>}
          {outputView.kind === 'failed' && <p role="alert" className="mt-2 text-body text-danger">{describeSqlPreparationProblem(outputView.problem)}</p>}
          <button type="button" className={button('quiet', 'mt-2', 'sm')} disabled={session === null || outputView.kind === 'checking'} onClick={() => { if (session !== null) void refreshViews(session) }}>
            Refresh views
          </button>
        </div>

        <details className="mb-4">
          <summary className="cursor-pointer text-body font-medium text-ink">Starting query</summary>
          <pre className={literal('mb-2 mt-2 overflow-x-auto whitespace-pre-wrap rounded-md border border-line bg-panel p-2 text-label text-muted')}>{starterSql(inputs)}</pre>
          <button type="button" className={button('quiet', 'gap-1', 'sm')} onClick={() => void copyStarter()}>
            <Icon name="content_copy" size={13} />
            {copy === 'copied' ? 'Copied' : copy === 'refused' ? 'Copy refused' : 'Copy'}
          </button>
        </details>
      </div>

      <div className="mac-terminal flex min-w-0 flex-col overflow-hidden rounded-[10px] lg:col-start-2 lg:row-span-2">
        <div className="mac-terminal-bar relative flex h-7 shrink-0 select-none items-center px-2">
          <span className="flex gap-2" aria-hidden>
            <i className="size-3 rounded-full bg-[#FF5F57]" />
            <i className="size-3 rounded-full bg-[#FEBC2E]" />
            <i className="size-3 rounded-full bg-[#28C840]" />
          </span>
          <span className="mac-terminal-title absolute left-1/2 -translate-x-1/2 truncate text-[13px]">duckdb — {inputs.length === 1 ? '1 table' : `${inputs.length} tables`}{size !== null ? ` — ${size.cols}×${size.rows}` : ''}</span>
        </div>
        <div ref={container} className="sql-console mac-terminal-body h-[clamp(20rem,60vh,40rem)] min-w-0 px-2 pt-2 pb-4" aria-label="SQL console" />
      </div>

      <div className="min-w-0 self-start lg:col-start-1">
        <div className="grid grid-cols-2 gap-2">
          <button type="button" className={button('signal', 'col-span-2')} aria-busy={state.kind === 'materializing'} disabled={busy || state.kind === 'failed-to-open' || outputView.kind !== 'available'} onClick={() => void usePreparedView()}>
            {state.kind === 'materializing' ? 'Checking selected view' : 'Use selected view'}
          </button>
          <button type="button" className={button('quiet')} disabled={session === null || cancellation.kind === 'requesting'} onClick={() => void cancelQuery()}>
            Cancel query
          </button>
          <button type="button" className={button('quiet')} disabled={state.kind === 'materializing'} onClick={onCleared}>
            Choose other files
          </button>
        </div>

        {busy && <p role="status" className="mb-0 mt-3 flex items-center gap-2 text-body text-muted"><Icon name="progress_activity" size={15} className="animate-spin" />{state.kind === 'opening' ? 'Opening the SQL console' : 'Checking and materializing the selected view'}</p>}
        {problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeSqlPreparationProblem(problem)}</p>}
        {cancellation.kind === 'reported' && <p role="status" className="mb-0 mt-3 text-body text-muted">{cancellation.message}</p>}
      </div>
    </div>
  )
}
