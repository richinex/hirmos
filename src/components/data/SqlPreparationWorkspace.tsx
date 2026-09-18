import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useCallback, useLayoutEffect, useRef, useState } from 'react'
import { useStore } from 'zustand'
import { useSqlSession } from './PreparationProvider'
import { createSqlSession, type SqlController } from './sqlSession'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { Dialog } from '@/components/ui/Dialog'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { button, field, fieldHint, literal, num, prose } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { PREPARED_VIEW, type SqlPreparationInput } from '@/domain/sqlPreparation'
import type { SelectedSource, SqlResume } from '@/domain/workflow'
import { describeSqlPreparationProblem } from '@/data/sqlPreparation'

type CopyState = 'idle' | 'copied' | 'refused'

interface SqlShellProps {
  readonly inputs: readonly SqlPreparationInput[]
  /** The recorded statement and output view to start from, when the console reopens on a source it made. */
  readonly resume: SqlResume | null
  readonly onPrepared: (source: SelectedSource) => void
  readonly onCleared: () => void
  readonly clearLabel?: string
  readonly onCancelEditing?: () => void
}

const starterSql = (inputs: readonly SqlPreparationInput[]): string => {
  const [first] = inputs
  if (first === undefined) return `CREATE OR REPLACE VIEW ${PREPARED_VIEW} AS SELECT 1 AS value;`
  if (first.format === 'duckdb-export-file') return 'SHOW TABLES;'
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

function SqlEditor({ inputs, onPrepared, onCleared, clearLabel = 'Choose other files', onCancelEditing, controller }: SqlShellProps & { readonly controller: SqlController }) {
  const container = useRef<HTMLDivElement>(null)
  const scriptPicker = useRef<HTMLInputElement>(null)
  const script = useStore(controller.store, state => state.script)
  const scriptResult = useStore(controller.store, state => state.scriptResult)
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
  const busy = state.kind === 'opening' || state.kind === 'materializing' || state.kind === 'running-script'
  const problem = state.kind === 'failed-to-open' || state.kind === 'materialization-refused'
    ? state.problem
    : null
  return (
    <div className="grid gap-6 lg:grid-cols-[20rem_minmax(0,1fr)] lg:grid-rows-[auto_1fr] lg:gap-x-8 lg:gap-y-4" aria-labelledby="sql-workspace-title">
      <div className="min-w-0">
        <ChapterHeading id="sql-workspace-title" className="mb-3">Prepare with SQL</ChapterHeading>
        <p className={prose('mb-4 mt-0 text-faint')}>Create one or more views, refresh the list, and choose the view that becomes the analysis source. The input files are not changed.</p>
        <input ref={scriptPicker} type="file" accept=".sql,text/plain,application/sql" aria-label="SQL script file" className="sr-only" onChange={event => {
          const file = event.target.files?.[0]
          if (file !== undefined) void controller.loadScript(file)
          event.target.value = ''
        }} />
        <button type="button" className={button('outline', 'mb-4')} disabled={busy || session === null} onClick={() => scriptPicker.current?.click()}>Open SQL file</button>
        {script.kind !== 'closed' && <Dialog open dismissible title="SQL file" onClose={controller.closeScript}>
          <p className={cn(fieldHint, 'break-all')}>{script.name}</p>
          {script.kind === 'reading' && <p role="status">Reading SQL file…</p>}
          {script.kind === 'failed' && <p role="alert" className="text-danger">{script.detail}</p>}
          {script.kind === 'editing' && <>
            {scriptResult.kind === 'none' && state.kind !== 'running-script' && <p className={fieldHint}>Review the SQL before running it. Opening this file has not executed it.</p>}
            <textarea aria-label="SQL file contents" disabled={busy} spellCheck={false} className={field('text', 'min-h-64 w-full font-mono text-label')} value={script.text} onChange={event => controller.editScript(event.target.value)} />
            <button type="button" className={button('signal', 'mt-3 w-full')} disabled={busy || !script.text.trim()} onClick={() => void controller.runScript()}>Run SQL</button>
            {state.kind === 'running-script' && <button type="button" className={button('quiet', 'mt-2 w-full')} onClick={() => void controller.cancel()}>Cancel query</button>}
            {scriptResult.kind === 'failed' && <p role="alert" className="text-danger">{scriptResult.detail}</p>}
            {scriptResult.kind === 'complete' && <EvidenceTable title="Final statement result" rows={scriptResult.rows} columns={scriptResult.columns.map((name, index) => ({ id: String(index), header: name, value: (row: readonly string[]) => row[index] ?? '' }))} rowKey={(_, index) => String(index)} noun="row" empty="The script completed without result rows." total={scriptResult.count} maxHeight="max-h-48" />}
          </>}
        </Dialog>}

        <p className="m-0 text-body font-medium text-ink">{inputs.some(input => input.format === 'duckdb-export-file') ? 'Export files' : 'Input tables'}</p>
        <ul className="mb-4 mt-2 list-none space-y-2 p-0" aria-label="SQL inputs">
          {inputs.map((input) => (
            <li key={input.alias} className="min-w-0">
              {input.format === 'duckdb-export-file'
                ? <code className={literal('block truncate text-label text-muted')}>{input.path}</code>
                : <><code className={literal('block truncate text-body text-ink')}>{input.alias}</code><span className={num('block truncate text-label text-faint')}>{input.fileName}</span></>}
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
          <span className="mac-terminal-title absolute left-1/2 -translate-x-1/2 truncate text-[13px]">duckdb{size !== null ? ` — ${size.cols}×${size.rows}` : ''}</span>
        </div>
        <div ref={container} inert={state.kind === 'running-script'} className="sql-console mac-terminal-body h-[clamp(20rem,60vh,40rem)] min-w-0 px-2 pt-2 pb-4" aria-label="SQL console" />
      </div>

      <div className="min-w-0 self-start lg:col-start-1">
        <div className="grid grid-cols-2 gap-2">
          {onCancelEditing !== undefined && <button type="button" className={button('quiet', 'text-danger hover:border-danger/50 hover:text-danger')} onClick={onCancelEditing}>Cancel editing</button>}
          <button type="button" className={button('signal', onCancelEditing === undefined ? 'col-span-2' : '')} aria-busy={state.kind === 'materializing'} disabled={busy || state.kind === 'failed-to-open' || outputView.kind !== 'available'} onClick={() => void usePreparedView()}>
            {state.kind === 'materializing' ? 'Checking selected view' : 'Use selected view'}
          </button>
          <button type="button" className={button('quiet')} disabled={session === null || cancellation.kind === 'requesting'} onClick={() => void controller.cancel()}>
            Cancel query
          </button>
          <button type="button" className={button('quiet')} disabled={state.kind === 'materializing'} onClick={onCleared}>
            {clearLabel}
          </button>
        </div>

        {busy && <p role="status" className="mb-0 mt-3 flex items-center gap-2 text-body text-muted"><Icon name="progress_activity" size={15} className="animate-spin" />{state.kind === 'opening' ? 'Opening the SQL console' : state.kind === 'running-script' ? 'Running SQL file' : 'Checking and materializing the selected view'}</p>}
        {problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeSqlPreparationProblem(problem)}</p>}
        {cancellation.kind === 'reported' && <p role="status" className="mb-0 mt-3 text-body text-muted">{cancellation.message}</p>}
      </div>
    </div>
  )
}
