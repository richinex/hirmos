import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useCallback, useLayoutEffect, useRef, useState } from 'react'
import { useStore } from 'zustand'
import { useSqlSession } from './PreparationProvider'
import { createSqlSession, type SqlController } from './sqlSession'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { button, field, fieldHint, literal, num, prose } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { type NonEmptyArray } from '@/domain/dop'
import { PREPARED_VIEW, type SqlPreparationInput } from '@/domain/sqlPreparation'
import type { SelectedSource, SqlResume } from '@/domain/workflow'
import { describeSqlPreparationProblem } from '@/data/sqlPreparation'

type CopyState = 'idle' | 'copied' | 'refused'

interface SqlShellProps {
  readonly inputs: NonEmptyArray<SqlPreparationInput>
  /** The recorded statement and output view to start from, when the console reopens on a source it made. */
  readonly resume: SqlResume | null
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

export function SqlShell(props: SqlShellProps) {
  const create = useCallback(() => createSqlSession(props.inputs, props.resume), [props.inputs, props.resume])
  const controller = useSqlSession(props.inputs, props.resume, create)
  if (controller === null) return null
  return <SqlEditor {...props} controller={controller} />
}

function SqlEditor({ inputs, onPrepared, onCleared, controller }: SqlShellProps & { readonly controller: SqlController }) {
  const container = useRef<HTMLDivElement>(null)
  const state = useStore(controller.store, state => state.shell)
  const cancellation = useStore(controller.store, state => state.cancellation)
  const outputView = useStore(controller.store, state => state.output)
  const size = useStore(controller.store, state => state.size)
  const [copy, setCopy] = useState<CopyState>('idle')
  useLayoutEffect(() => {
    if (container.current !== null) return controller.attach(container.current)
  }, [controller])
  const usePreparedView = async () => {
    const source = await controller.adopt()
    if (source !== null && controller.active()) onPrepared(source)
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

  const session = 'session' in state ? state.session : null
  const busy = state.kind === 'opening' || state.kind === 'materializing'
  const problem = state.kind === 'failed-to-open' || state.kind === 'materialization-refused'
    ? state.problem
    : null
  return (
    <div className="grid gap-6 lg:grid-cols-[20rem_minmax(0,1fr)] lg:grid-rows-[auto_1fr] lg:gap-x-8 lg:gap-y-4" aria-labelledby="sql-workspace-title">
      <div className="min-w-0">
        <ChapterHeading id="sql-workspace-title" className="mb-3">Prepare with SQL</ChapterHeading>
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
              controller.select(event.target.value)
            }}>
              {outputView.views.map((view) => <option key={view} value={view}>{view}</option>)}
            </Select>
          )}
          {outputView.kind === 'checking' && <p className={cn(fieldHint, 'mt-2')}>Listing the views.</p>}
          {outputView.kind === 'none' && <p className={cn(fieldHint, 'mt-2')}>No view yet. Run a <code className={literal()}>CREATE VIEW</code> statement in the console, then refresh the list. Input tables are not offered.</p>}
          {outputView.kind === 'failed' && <p role="alert" className="mt-2 text-body text-danger">{describeSqlPreparationProblem(outputView.problem)}</p>}
          <button type="button" className={button('quiet', 'mt-2', 'sm')} disabled={session === null || outputView.kind === 'checking'} onClick={() => { if (session !== null) void controller.refresh() }}>
            Refresh views
          </button>
        </div>

        <details className="mb-4">
          <DisclosureSummary className="cursor-pointer text-body font-medium text-ink">Starting query</DisclosureSummary>
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
          <button type="button" className={button('quiet')} disabled={session === null || cancellation.kind === 'requesting'} onClick={() => void controller.cancel()}>
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
