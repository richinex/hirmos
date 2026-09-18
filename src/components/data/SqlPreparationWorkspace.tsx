import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useCallback, useRef, useState } from 'react'
import { WorkbenchLayout, useClosePane } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { Tooltip } from '@/components/ui/Tooltip'
import { useStore } from 'zustand'
import { useSqlSession } from './PreparationProvider'
import { createSqlSession, type SqlController } from './sqlSession'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { Dialog } from '@/components/ui/Dialog'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { button, field, fieldHint, iconControl, literal, num } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { PREPARED_VIEW, type SqlPreparationInput } from '@/domain/sqlPreparation'
import type { SelectedSource, SqlResume } from '@/domain/workflow'
import { describeSqlPreparationProblem } from '@/data/sqlPreparation'

type CopyState = 'idle' | 'copied' | 'refused'

function SqlFileButton({ disabled, onOpen }: { readonly disabled: boolean; readonly onOpen: () => void }) {
  const closePane = useClosePane()
  return <button type="button" className={button('outline', 'w-full')} disabled={disabled} onClick={() => { closePane(); onOpen() }}><Icon name="file_open" />Open SQL file</button>
}

function CloseSqlSource() {
  const closePane = useClosePane()
  return <button type="button" aria-label="Close SQL source" className={iconControl('quiet', 'md:hidden')} onClick={closePane}><Icon name="close" /></button>
}

function SqlOutputButton({ controller, onPrepared, disabled, busy }: { readonly controller: SqlController; readonly onPrepared: (source: SelectedSource) => void; readonly disabled: boolean; readonly busy: boolean }) {
  const closePane = useClosePane()
  const adopt = async () => {
    const source = await controller.adopt()
    if (source === null || !controller.active()) return
    closePane()
    onPrepared(source)
  }
  return <button type="button" className={button('signal', 'mt-3 w-full')} aria-busy={busy} disabled={disabled} onClick={() => void adopt()}>Use selected view</button>
}

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
  const attachConsole = useCallback((element: HTMLDivElement | null) => element === null ? undefined : controller.attach(element), [controller])
  const scriptPicker = useRef<HTMLInputElement>(null)
  const script = useStore(controller.store, state => state.script)
  const scriptResult = useStore(controller.store, state => state.scriptResult)
  const state = useStore(controller.store, state => state.shell)
  const cancellation = useStore(controller.store, state => state.cancellation)
  const outputView = useStore(controller.store, state => state.output)
  const size = useStore(controller.store, state => state.size)
  const query = useStore(controller.store, state => state.query)
  const [copy, setCopy] = useState<CopyState>('idle')
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
  const busy = state.kind === 'opening' || state.kind === 'materializing' || state.kind === 'running-script' || query === 'running'
  const problem = state.kind === 'failed-to-open' || state.kind === 'materialization-refused'
    ? state.problem
    : null
  const stage = <div className="flex h-[calc(100dvh-9rem)] min-h-80 min-w-0 flex-col md:h-full md:min-h-0">
    <ChapterHeading id="sql-workspace-title" className="sr-only">Prepare with SQL</ChapterHeading>
    <div className="mac-terminal m-3 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden rounded-[10px]">
      <div className="mac-terminal-bar relative flex h-7 shrink-0 select-none items-center px-2">
        <span className="flex gap-2" aria-hidden><i className="size-3 rounded-full bg-[#FF5F57]" /><i className="size-3 rounded-full bg-[#FEBC2E]" /><i className="size-3 rounded-full bg-[#28C840]" /></span>
        <span className="mac-terminal-title absolute left-1/2 -translate-x-1/2 truncate text-[13px]">duckdb{size !== null ? ` — ${size.cols}×${size.rows}` : ''}</span>
        {query === 'running' && state.kind !== 'running-script' && <button type="button" aria-label="Cancel query" title="Cancel query" className={iconControl('quiet', 'absolute right-1 h-6 w-6')} disabled={cancellation.kind === 'requesting'} onClick={() => void controller.cancel()}><Icon name="stop" size={16} /></button>}
      </div>
      <div ref={attachConsole} inert={state.kind === 'running-script'} className="sql-console mac-terminal-body min-h-0 flex-1 min-w-0 overflow-hidden px-2 pt-2 pb-4" aria-label="SQL console" />
    </div>
    {state.kind === 'opening' && <p role="status" className={fieldHint}>Opening the SQL console…</p>}
    {cancellation.kind === 'reported' && <Alert tone="info">{cancellation.message}</Alert>}
  </div>
  return <>
        <input ref={scriptPicker} type="file" accept=".sql,text/plain,application/sql" aria-label="SQL script file" className="sr-only" onChange={event => {
          const file = event.target.files?.[0]
          if (file !== undefined) void controller.loadScript(file)
          event.target.value = ''
        }} />
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

      <WorkbenchLayout id="sql" stage={stage} stagePadding={false} stageScroll={false} inspector={{ title: 'SQL source', trigger: { label: 'Source', icon: 'database' }, controls: <CloseSqlSource />, body: <div className="space-y-5">
        <SqlFileButton disabled={busy || session === null} onOpen={() => scriptPicker.current?.click()} />
        {inputs.length > 0 && <div><p className="m-0 text-body font-medium text-ink">{inputs.some(input => input.format === 'duckdb-export-file') ? 'Export files' : 'Input tables'}</p>
        <ul className="mb-4 mt-2 list-none space-y-2 p-0" aria-label="SQL inputs">
          {inputs.map((input) => (
            <li key={input.alias} className="min-w-0">
              {input.format === 'duckdb-export-file'
                ? <code className={literal('block truncate text-label text-muted')}>{input.path}</code>
                : <><code className={literal('block truncate text-body text-ink')}>{input.alias}</code><span className={num('block truncate text-label text-faint')}>{input.fileName}</span></>}
            </li>
          ))}
        </ul></div>}

        <div className="mb-4">
          <div className="flex items-center justify-between gap-2">
            <span className="text-body font-medium text-ink">Output view</span>
            <Tooltip text="Refresh views"><button type="button" aria-label="Refresh views" className={iconControl('quiet')} disabled={busy || session === null || outputView.kind === 'checking'} onClick={() => void controller.refresh()}><Icon name="refresh" /></button></Tooltip>
          </div>
          {outputView.kind === 'available' && (
            <Select aria-label="Output view" className={field('text', 'mt-2')} value={outputView.selected} onChange={(event) => {
              controller.select(event.target.value)
            }}>
              {outputView.views.map((view) => <option key={view} value={view}>{view}</option>)}
            </Select>
          )}
          {outputView.kind === 'checking' && <p className={cn(fieldHint, 'mt-2')}>Listing the views.</p>}
          {outputView.kind === 'none' && <p className={cn(fieldHint, 'mt-2')}>Create a view to use as your source.</p>}
          {outputView.kind === 'failed' && <p role="alert" className="mt-2 text-body text-danger">{describeSqlPreparationProblem(outputView.problem)}</p>}
          <SqlOutputButton controller={controller} onPrepared={onPrepared} busy={state.kind === 'materializing'} disabled={busy || state.kind === 'failed-to-open' || outputView.kind !== 'available'} />
          {state.kind === 'materializing' && <p role="status" className={fieldHint}>Checking selected view…</p>}
          {problem !== null && <Alert tone="danger">{describeSqlPreparationProblem(problem)}</Alert>}
        </div>

        <details className="mb-4">
          <DisclosureSummary className="cursor-pointer text-body font-medium text-ink">Starting query</DisclosureSummary>
          <div className="relative mt-2 rounded-md border border-line bg-panel">
            <Tooltip text={copy === 'copied' ? 'Copied' : copy === 'refused' ? 'Copy refused' : 'Copy query'}>
              <button type="button" aria-label={copy === 'copied' ? 'Copied' : copy === 'refused' ? 'Copy refused' : 'Copy query'} className={iconControl('quiet', 'absolute right-1 top-1')} onClick={() => void copyStarter()}><Icon name={copy === 'copied' ? 'check' : 'content_copy'} size={14} /></button>
            </Tooltip>
            <pre className={literal('m-0 overflow-x-auto whitespace-pre-wrap p-2 pr-12 text-label text-muted')}>{starterSql(inputs)}</pre>
            <span role="status" className="sr-only">{copy === 'copied' ? 'Query copied' : copy === 'refused' ? 'Copy refused' : ''}</span>
          </div>
        </details>
        <div className="flex flex-col gap-2">
          {onCancelEditing !== undefined && <button type="button" className={button('quiet', 'text-danger hover:border-danger/50 hover:text-danger')} onClick={onCancelEditing}>Cancel editing</button>}
          <button type="button" className={button('quiet')} disabled={state.kind === 'materializing'} onClick={onCleared}>
            {clearLabel}
          </button>
        </div>

      </div> }} />
    </>
}
