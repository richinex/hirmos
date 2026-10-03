import { BlockToolbar } from './BlockToolbar'
import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { Icon } from '@/components/Icon'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { TapNote, Tooltip } from '@/components/ui/Tooltip'
import { button, caption, chromeAction, label, literal, num } from '@/components/ui/recipes'
import { isNumericDuckDbType, type PreviewCell } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { blockLabel, describeCalendarWindow, describePipelineProblem, type PipelineBlock, type PipelineBlockId, type PipelineEdge, type PipelineNode, type RowTest } from '@/domain/pipeline'
import type { SqlInputAlias, SqlPreparationInput } from '@/domain/sourceInputs'
import { columnReading, type DeclaredType } from '@/domain/fileReading'
import { ColumnTypeText, ReadAsButton, TypeGlyph } from '../ReadAs'
import { type PipelineResume, type SelectedSource } from '@/domain/workflow'
import {
  previewBlock,
  type BlockOutcome,
  type BlockPreview,
} from '@/data/pipeline'
import { PythonProvider } from './PythonProvider'
import { useStore } from 'zustand'
import { usePipelineSession } from '../PreparationProvider'
import { createPipelineSession, type PipelineController } from './pipelineSession'
import { formatBytes, formatCount } from '@/lib/format/number'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'
import { EvidenceTable, type EvidenceColumn, type EvidenceValue } from '@/components/table/EvidenceTable'
import { columnDecimals, PreviewCellText } from '../previewCell'
import { BlockSettings, blockSqlText, preloadPythonEditor } from './BlockSettings'
import { usePythonRun } from './usePythonRun'
import { BLOCK_DRAG_TYPE, blockIcon, PALETTE_GROUPS, type PaletteKind } from './blockIcons'
import { PipelineCanvas } from './PipelineCanvas'
import { addBlock, addInputBlock, configureBlock, connect, describeConnectionRefusal, indexGraph, inputsOf, moveBlock, placeFor, removeEdge, tidyGraph, type ConnectionRefusal, type GraphIndex } from './pipelineWorkspaceModel'

const testWord = (test: RowTest): string => {
  switch (test) {
    case 'eq': return '='
    case 'ne': return '≠'
    case 'lt': return '<'
    case 'le': return '≤'
    case 'gt': return '>'
    case 'ge': return '≥'
    case 'contains': return 'contains'
    case 'starts-with': return 'starts with'
    case 'is-null': return 'is missing'
    case 'not-null': return 'is present'
    default: return assertNever(test)
  }
}

interface PreviewRow { readonly index: number; readonly cells: readonly PreviewCell[] }

const previewNotice = (outcome: BlockOutcome | undefined, nothingSelected: boolean): string => {
  if (nothingSelected) return 'Select a block to see its rows.'
  if (outcome === undefined) return 'Not run yet.'
  switch (outcome.kind) {
    case 'failed': return 'No preview available.'
    case 'waiting': return 'No preview available.'
    case 'skipped': return 'Not run: a block before it failed.'
    case 'ran': return 'No rows to show yet.'
    default: return assertNever(outcome)
  }
}

/** What a cell sorts and searches by: the number for figures, the text otherwise. */
const sortableValue = (cell: PreviewCell | undefined): EvidenceValue => {
  if (cell === undefined) return ''
  switch (cell.kind) {
    case 'null': return ''
    case 'number': return cell.value
    case 'integer': { const value = Number(cell.value); return Number.isSafeInteger(value) ? value : cell.value }
    case 'boolean': return cell.value ? 'true' : 'false'
    case 'temporal': return cell.value
    case 'text': return cell.value
    default: return assertNever(cell)
  }
}

const summarise = (block: PipelineBlock): string => {
  switch (block.kind) {
    case 'input': return block.file.kind === 'chosen' ? block.file.alias : 'choose a file'
    case 'filter-rows': return block.conditions.length === 0 ? 'every row' : block.conditions.map((c) => `${c.column} ${testWord(c.test)}${'value' in c ? ` ${c.value}` : ''}`).join(block.match === 'all' ? ' and ' : ' or ')
    case 'select-columns': return block.columns.length === 0 ? 'every column' : `${block.mode} ${block.columns.join(', ')}`
    case 'derive-columns': return block.columns.length === 0 ? 'no new columns yet' : block.columns.map((c) => c.name || '?').join(', ')
    case 'calendar-events': return block.column === '' ? 'choose the date column' : `${block.name || '?'}: ${describeCalendarWindow(block.window)} over ${block.column}`
    case 'join': return `${block.how}${block.keys.length === 0 ? '' : ` on ${block.keys.map((k) => k.left === k.right ? k.left : `${k.left} = ${k.right}`).join(', ')}`}`
    case 'union': return `by ${block.by}${block.distinct ? ', distinct' : ''}`
    case 'aggregate': return `${block.groupBy.length === 0 ? 'all rows' : `by ${block.groupBy.join(', ')}`}, ${block.measures.map((m) => m.as || m.function).join(', ') || 'no measures yet'}`
    case 'sort-limit': return `${block.sort.map((s) => `${s.column} ${s.direction === 'ascending' ? '↑' : '↓'}`).join(', ') || 'unsorted'}${block.limit === null ? '' : `, first ${block.limit}`}`
    case 'script': return `${block.code.split('\n').filter((line) => line.trim().length > 0).length} lines`
    case 'output': return 'the source'
    default: return assertNever(block)
  }
}

type Props = {
  /** A graph and its files to start from, when the canvas reopens on a source it made. */
  readonly resume: PipelineResume | null
  readonly onPrepared: (source: SelectedSource) => void
  readonly onCancelEditing?: () => void
}

export function PipelineWorkspace(props: Props) {
  const create = useCallback(() => createPipelineSession(props.resume), [props.resume])
  const controller = usePipelineSession(props.resume, create)
  if (controller === null) return null
  return <PythonProvider runtime={controller.python}><PipelineEditor {...props} controller={controller} /></PythonProvider>
}

function PipelineEditor({ onPrepared, onCancelEditing, controller }: Props & { readonly controller: PipelineController }) {
  const graph = useStore(controller.store, state => state.graph)
  const files = useStore(controller.store, state => state.files)
  const session = useStore(controller.store, state => state.session)
  const run = useStore(controller.store, state => state.run)
  const choosing = useStore(controller.store, state => state.choosing)
  const materializing = useStore(controller.store, state => state.materializing)
  const setGraph = controller.setGraph
  const [selected, setSelected] = useState<PipelineBlockId | null>(null)
  const [shown, setShown] = useState<{ readonly data: BlockPreview; readonly title: string; readonly outcome: BlockOutcome } | null>(null)
  const preview = shown?.data ?? null
  const [refusal, setRefusal] = useState<string | null>(null)
  const index = useMemo(() => indexGraph(graph), [graph])
  const isMobile = useIsMobile()

  const nameOf = useCallback((id: PipelineBlockId): string => {
    const node = index.nodes.get(id)
    if (node === undefined) return id
    return node.block.kind === 'input' && node.block.file.kind === 'chosen' ? `${blockLabel('input')} ${node.block.file.alias}` : blockLabel(node.block.kind)
  }, [index])

  const outcomes = useMemo(() => run.kind === 'ran' ? run.result.outcomes : new Map<PipelineBlockId, BlockOutcome>(), [run])
  const viewOf = useCallback((id: PipelineBlockId): string | null => run.kind === 'ran' ? run.result.views.get(id) ?? null : null, [run])

  // The rows of the selected block, read once per run of it; the run already knows its columns and count.
  const selectedOutcome = selected === null ? undefined : outcomes.get(selected)
  const selectedView = selected === null ? null : viewOf(selected)
  const selectedTitle = selected === null ? 'Block' : nameOf(selected)
  useEffect(() => {
    if (session.kind !== 'ready' || selected === null || selectedView === null || selectedOutcome?.kind !== 'ran') return
    let cancelled = false
    void previewBlock(session.live, selectedView, selected, selectedOutcome).then((result) => {
      if (!cancelled) setShown(result.ok ? { data: result.value, title: selectedTitle, outcome: selectedOutcome } : null)
    })
    return () => { cancelled = true }
  }, [selected, selectedOutcome, selectedView, session, selectedTitle])

  const selectedNode = selected === null ? null : index.nodes.get(selected) ?? null
  const summaries = useMemo(() => new Map(graph.nodes.map((node) => [node.id, summarise(node.block)])), [graph.nodes])

  const add = useCallback((kind: PaletteKind, position?: { readonly x: number; readonly y: number }) => {
    const current = controller.store.getState().graph
    const added = kind === 'input'
      ? addInputBlock(position === undefined ? current : { ...current })
      : addBlock(current, kind, position ?? placeFor(current, selected))
    setGraph(position === undefined || kind !== 'input' ? added.graph : moveBlock(added.graph, added.id, position))
    setSelected(added.id)
  }, [selected, controller, setGraph])

  const chooseFile = controller.chooseFile
  const dropBlock = (id: PipelineBlockId) => {
    controller.dropBlock(id)
    setSelected(current => current === id ? null : current)
  }
  const tidy = useCallback(() => {
    const graph = controller.store.getState().graph
    void tidyGraph(graph).then(tidied => { if (controller.store.getState().graph === graph) setGraph(tidied) })
  }, [controller, setGraph])
  const refuse = (refusal: ConnectionRefusal | null) => setRefusal(refusal === null ? null : describeConnectionRefusal(refusal))
  useEffect(() => {
    if (refusal === null) return
    const handle = window.setTimeout(() => setRefusal(null), 8000)
    return () => window.clearTimeout(handle)
  }, [refusal])

  const adoptAsSource = async () => {
    const source = await controller.adopt()
    if (source !== null && controller.active()) onPrepared(source)
  }

  const completeProblem = run.kind === 'ran' && !run.result.complete.ok ? describePipelineProblem(run.result.complete.error, nameOf) : null
  const outputReady = run.kind === 'ran' && run.result.complete.ok && [...run.result.complete.value.views.keys()].every((id) => run.result.outcomes.get(id)?.kind === 'ran')

  const stage = (
    <div className="flex min-h-0 min-w-0 flex-1 flex-col">
      <BlockToolbar>
        {PALETTE_GROUPS.map((group) => (
          <div key={group.label} className="flex shrink-0 items-center gap-0.5" role="group" aria-label={group.label}>
            {group.kinds.map((kind) => (
              <button
                key={kind}
                type="button"
                className={chromeAction('quiet', 'min-w-10 shrink-0 cursor-grab justify-center whitespace-nowrap text-muted hover:bg-panel active:cursor-grabbing active:bg-well')}
                aria-label={blockLabel(kind)}
                title={`Click to add ${blockLabel(kind).toLowerCase()} after the selected block, or drag it onto the canvas`}
                draggable
                onDragStart={(event) => { event.dataTransfer.setData(BLOCK_DRAG_TYPE, kind); event.dataTransfer.effectAllowed = 'copy' }}
                onPointerEnter={kind === 'script' ? preloadPythonEditor : undefined}
                onFocus={kind === 'script' ? preloadPythonEditor : undefined}
                onClick={() => add(kind)}
              >
                <Icon name={blockIcon(kind)} size={16} /><span className="pipeline-block-label">{blockLabel(kind)}</span>
              </button>
            ))}
          </div>
        ))}
        <span className="ml-auto flex items-center gap-2" role="status">
          {session.kind === 'opening' && <span className={caption('m-0')}>Opening DuckDB</span>}
          {session.kind === 'failed' && <span className="text-label text-danger">{session.detail}</span>}
        </span>
      </BlockToolbar>
      {refusal !== null && <Alert tone="warn" className="mx-3 mt-2"><p className="m-0">{refusal}</p></Alert>}
      {run.kind === 'refused' && <Alert tone="danger" className="mx-3 mt-2"><p className="m-0">{run.detail}</p></Alert>}
      <div className="relative min-h-[16rem] flex-1">
        <PipelineCanvas
          graph={graph}
          outcomes={outcomes}
          selected={selected}
          summaries={summaries}
          onSelect={setSelected}
          onMove={(id, position) => setGraph((current) => moveBlock(current, id, position))}
          onConnect={(edge: PipelineEdge) => setGraph((current) => connect(current, edge))}
          onRemoveEdge={(edge) => setGraph((current) => removeEdge(current, edge))}
          onRemoveBlock={dropBlock}
          onRefused={refuse}
          onDropBlock={(kind, position) => add(kind, position)}
          onTidy={tidy}
        />
      </div>
    </div>
  )

  const outputNotice = completeProblem === null ? null : <p id="pipeline-output-problem" data-testid="pipeline-incomplete" className="m-0 flex items-start gap-1.5 text-body text-warn"><Icon name="warning" size={16} className="mt-0.5 shrink-0" />{completeProblem}</p>
  const inspector = selectedNode === null
    ? (
      <div className="space-y-3">
        <Alert tone="info" live={false} testId="pipeline-instructions">Connect input files to “Use as source” through transformation blocks. Select a block to edit its settings and preview its rows.</Alert>
        {outputNotice}
        <div className={caption('m-0 flex items-center gap-3')} aria-label="Pipeline size">
          {[
            { icon: 'deployed_code', count: graph.nodes.length, singular: 'block', plural: 'blocks' },
            { icon: 'arrow_right_alt', count: graph.edges.length, singular: 'connection', plural: 'connections' },
          ].map(({ icon, count, singular, plural }) => {
            const description = `${formatCount(count).text} ${count === 1 ? singular : plural}`
            const Note = isMobile ? TapNote : Tooltip
            return <Note key={icon} text={description}>
              <button type="button" aria-label={description} className="inline-flex items-center gap-1.5 rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-current">
                <Icon name={icon} size={16} /><span aria-hidden className="tabular-nums">{formatCount(count).text}</span>
              </button>
            </Note>
          })}
        </div>
        {files.length > 0 && (
          <div>
            <span className={label('block text-muted')}>Files</span>
            <ul className="m-0 mt-1 list-none space-y-0.5 p-0">
              {files.map((input) => <li key={input.alias} className="grid min-w-0 grid-cols-[minmax(0,1fr)_auto] items-start gap-x-3 gap-y-1 py-1 text-body"><span className={literal('min-w-0 text-ink [overflow-wrap:anywhere]')}>{input.alias}</span><span className="text-micro text-faint">{formatBytes(input.bytes)}</span>{input.fileName.replace(/\.[^.]+$/, '') !== input.alias && <span className="col-span-2 min-w-0 text-micro text-muted [overflow-wrap:anywhere]">{input.fileName}</span>}</li>)}
            </ul>
          </div>
        )}
      </div>
    )
    : <Inspector node={selectedNode} index={index} outcomes={outcomes} viewOf={viewOf} files={files} choosing={choosing} onChooseFile={(file) => void chooseFile(selectedNode.id, file)} onDeclare={(alias, column, type) => void controller.declare(alias, column, type)} onChange={(block) => setGraph((current) => configureBlock(current, selectedNode.id, block))} onRemove={() => dropBlock(selectedNode.id)} />

  const decimals = useMemo(() => preview === null ? [] : preview.columns.map((_, index) => columnDecimals(preview.rows, index)), [preview])
  const previewColumns = useMemo<EvidenceColumn<PreviewRow>[]>(() => preview === null ? [] : [
    { id: '#', header: '#', align: 'right', value: (row) => row.index + 1 },
    ...preview.columns.map((column, columnIndex): EvidenceColumn<PreviewRow> => ({
      id: column.name,
      header: column.name,
      detail: column.type,
      align: isNumericDuckDbType(column.type) ? 'right' : 'left',
      mono: false,
      value: (row) => sortableValue(row.cells[columnIndex]),
      format: (_value, row) => { const cell = row.cells[columnIndex]; return cell === undefined ? null : <PreviewCellText cell={cell} decimals={decimals[columnIndex] ?? 0} /> },
    })),
  ], [decimals, preview])
  const previewRows = useMemo<PreviewRow[]>(() => preview === null ? [] : preview.rows.map((cells, index) => ({ index, cells })), [preview])
  const previewVisible = shown !== null && selectedNode !== null && selectedOutcome?.kind === 'ran'
  const previewPending = previewVisible && (shown.data.id !== selected || shown.outcome !== selectedOutcome)
  // A script that evaluated to one value reads as a number, not as a table of one cell.
  const readout = selectedOutcome?.kind === 'ran' && selectedOutcome.shape?.kind === 'value' && preview !== null
    ? { column: selectedOutcome.shape.column, cell: preview.rows[0]?.[0] ?? null }
    : null

  const bottom = (
    <div className="flex h-full min-h-0 flex-col" data-testid="pipeline-preview" aria-busy={previewPending}>
      {!previewVisible && <p className={caption('m-3')}>{previewNotice(selectedNode === null ? undefined : outcomes.get(selectedNode.id), selectedNode === null)}</p>}
      {readout !== null && readout.cell !== null && (
        <div hidden={!previewVisible} className={cn('min-h-0 flex-1 overflow-auto p-3', !previewVisible && 'hidden')} data-testid="pipeline-value">
          <p className={num('m-0 text-title text-ink')}><PreviewCellText cell={readout.cell} decimals={columnDecimals(preview?.rows ?? [], 0)} /></p>
        </div>
      )}
      {preview !== null && shown !== null && readout === null && (
          <div hidden={!previewVisible} className={cn('min-h-0 flex-1 overflow-auto p-3', !previewVisible && 'hidden')}>
            <EvidenceTable
              appearance="data"
              title={shown.title}
              rows={previewRows}
              columns={previewColumns}
              rowKey={(row) => String(row.index)}
              noun="row"
              empty="The block produced no rows."
              total={preview.rowCount}
              exportName={`${preview.id}-preview`}
              maxHeight="max-h-none"
              frame="none"
            />
          </div>
        )}
    </div>
  )

  const previewTitle = 'Preview'

  return (
    <section className="flex min-h-0 min-w-0 w-full flex-1 flex-col" aria-label="Build a pipeline">
      <WorkbenchLayout
        id="pipeline"
        stage={stage}
        stagePadding={false}
        stageScroll={false}
        inspector={{
          title: selectedNode === null ? 'Pipeline' : 'Block settings',
          body: (
            <div className="flex min-w-0 flex-col gap-4">
              <div role="group" aria-label="Pipeline actions" className={cn('grid gap-2', onCancelEditing === undefined ? 'grid-cols-1' : 'grid-cols-2')}>
                {onCancelEditing !== undefined && <button type="button" className={button('quiet', 'text-danger hover:border-danger/50 hover:text-danger')} onClick={onCancelEditing}>Cancel editing</button>}
                <button type="button" className={button('signal')} disabled={!outputReady || materializing.kind === 'busy'} aria-busy={materializing.kind === 'busy'} aria-describedby={completeProblem === null ? undefined : 'pipeline-output-problem'} onClick={() => void adoptAsSource()}>Use as source</button>
              </div>
              {materializing.kind === 'refused' && <Alert tone="danger">{materializing.detail}</Alert>}
              {selectedNode !== null && outputNotice}
              {inspector}
            </div>
          ),
        }}
        bottom={{ title: previewTitle, body: bottom, defaultSize: 220 }}
      />
    </section>
  )
}

function Inspector({ node, index, outcomes, viewOf, files, choosing, onChooseFile, onDeclare, onChange, onRemove }: {
  readonly node: PipelineNode
  readonly index: GraphIndex
  readonly outcomes: ReadonlyMap<PipelineBlockId, BlockOutcome>
  readonly viewOf: (id: PipelineBlockId) => string | null
  readonly files: readonly SqlPreparationInput[]
  readonly choosing: { readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }
  readonly onChooseFile: (file: File) => void
  readonly onDeclare: (alias: SqlInputAlias, column: string, type: DeclaredType | null) => void
  readonly onChange: (block: PipelineBlock) => void
  readonly onRemove: () => void
}) {
  const inputs = inputsOf(index, node.id)
  const running = usePythonRun(node.id)
  const inputColumns = inputs.map((input) => { const outcome = outcomes.get(input.id); return outcome?.kind === 'ran' ? outcome.columns : [] })
  const inputNames = inputs.map((input) => blockLabel(input.block.kind) + (input.block.kind === 'input' && input.block.file.kind === 'chosen' ? ` ${input.block.file.alias}` : ''))
  const outcome = outcomes.get(node.id)
  const sql = blockSqlText(node, inputs.map((input) => viewOf(input.id) ?? '?'))
  const removable = node.block.kind !== 'output'
  const heldAlias = node.block.kind === 'input' && node.block.file.kind === 'chosen' ? node.block.file.alias : null
  const held = heldAlias === null ? null : files.find((input) => input.alias === heldAlias) ?? null
  const picker = useRef<HTMLInputElement>(null)
  const [tab, setTab] = useState<'settings' | 'columns' | 'sql'>('settings')
  const ran = outcome?.kind === 'ran' ? outcome : null
  const hasSettings = node.block.kind !== 'input' && node.block.kind !== 'output'
  const tabs = [...(hasSettings ? ['settings' as const] : []), ...(ran !== null ? ['columns' as const] : []), ...(sql !== null ? ['sql' as const] : [])]
  const shownTab = tabs.includes(tab) ? tab : tabs[0] ?? 'settings'
  const subtitle = node.block.kind === 'input' ? '' : summarise(node.block)
  return (
    <div className="flex flex-col gap-4">
      <section aria-labelledby="block-settings-title">
        <h3 id="block-settings-title" className="mb-1 mt-0 text-body font-medium text-ink">{blockLabel(node.block.kind)}</h3>
        {subtitle !== '' && <p className={literal('m-0 text-label text-muted [overflow-wrap:anywhere]')}>{subtitle}</p>}
      </section>
      {node.block.kind === 'input' && <InputFileField held={held} choosing={choosing} />}
      {!(node.block.kind === 'input' && node.block.file.kind === 'empty') && !(node.block.kind === 'output' && outcome?.kind === 'waiting') && <BlockStatus outcome={outcome} running={running !== null} />}
      {tabs.length > 1 && <SegmentedControl size="sm" fill ariaLabel="Block inspector" value={shownTab} onChange={setTab} options={[
        ...(hasSettings ? [{ value: 'settings' as const, label: <span className="flex items-center gap-1.5"><Icon name="tune" size={14} />Settings</span> }] : []),
        ...(ran !== null ? [{ value: 'columns' as const, label: <span className="flex items-center gap-1.5"><Icon name="view_column" size={14} />Columns</span> }] : []),
        ...(sql !== null ? [{ value: 'sql' as const, label: <span className="flex items-center gap-1.5"><Icon name="code" size={14} />SQL</span> }] : []),
      ]} />}
      {shownTab === 'columns' && ran !== null && (
          <ul className="m-0 list-none space-y-0.5 p-0" data-testid="block-schema">
            {ran.columns.map((column) => (
              <li key={column.name} className="flex items-center gap-1.5 text-body">
                {held === null || held.format === 'parquet' || held.format === 'duckdb-export-file'
                  ? <TypeGlyph type={column.type} />
                  : <ReadAsButton column={column.name} reading={columnReading(held, column.name)} type={column.type} disabled={choosing.kind === 'busy'} onDeclare={(type) => onDeclare(held.alias, column.name, type)} />}
                <span className={literal('min-w-0 truncate text-ink')}>{column.name}</span>
                <span className="ml-auto shrink-0 text-micro"><ColumnTypeText reading={held === null || held.format === 'duckdb-export-file' ? { kind: 'detected' } : columnReading(held, column.name)} type={column.type} /></span>
              </li>
            ))}
          </ul>
      )}
      {shownTab === 'sql' && sql !== null && <pre className={cn(literal(), 'm-0 overflow-x-auto rounded-md border border-hair bg-well px-2.5 py-2 text-label leading-relaxed text-bone')} data-testid="block-sql">{sql}</pre>}
      {shownTab === 'settings' && <>
      <BlockSettings node={node} inputColumns={inputColumns} inputNames={inputNames} onChange={onChange} />
      {(outcome?.kind === 'ran' || outcome?.kind === 'failed') && outcome.stdout !== undefined && outcome.stdout.length > 0 && (
        <div>
          <span className={label('block text-muted')}>Printed</span>
          <pre className={cn(literal(), 'mt-1 max-h-48 overflow-auto rounded-md border border-hair bg-well px-2.5 py-2 text-label leading-relaxed text-bone')} data-testid="block-stdout">{outcome.stdout}</pre>
        </div>
      )}
      </>}
      {removable && (
        <div className={cn('grid gap-2 pt-3', node.block.kind === 'input' ? 'grid-cols-2' : 'grid-cols-1')}>
          {node.block.kind === 'input'
            ? (
              <button type="button" className={button(held === null ? 'signal' : 'quiet', 'whitespace-nowrap', 'sm')} disabled={choosing.kind === 'busy'} aria-busy={choosing.kind === 'busy'} onClick={() => picker.current?.click()}>
                <Icon name="upload_file" size={14} /> {held === null ? 'Choose file' : 'Replace file'}
              </button>
            )
            : null}
          {/* The same control family as the file button beside it: control radius, the small step, danger only in the ink. */}
          <button type="button" className={button('quiet', 'whitespace-nowrap text-danger hover:border-danger/50 hover:text-danger', 'sm')} onClick={onRemove}><Icon name="delete" size={14} /> Remove block</button>
        </div>
      )}
      {node.block.kind === 'input' && <input ref={picker} type="file" accept={FILE_ACCEPT} className="sr-only" aria-label="File for this card" onChange={(event) => { const file = event.target.files?.[0]; if (file !== undefined) onChooseFile(file); event.target.value = '' }} />}
    </div>
  )
}

const FILE_ACCEPT = '.csv,.tsv,.parquet,text/csv,text/tab-separated-values'

/** The file an input card holds, or the prompt to choose one; choosing and replacing are actions in the footer. */
/** One label column for every detail list in the block panel, so values align from list to list. */
const detailList = 'm-0 grid grid-cols-[4.5rem_minmax(0,1fr)] items-baseline gap-x-3 gap-y-1 text-body'

function InputFileField({ held, choosing }: {
  readonly held: SqlPreparationInput | null
  readonly choosing: { readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }
}) {
  return (
    <div className="space-y-2">
      {held === null
        ? <p className={caption('m-0')}>Select a CSV, TSV, or Parquet file, then connect it to the next block to continue processing.</p>
        : (
          <dl className={detailList}>
            <dt className="text-label text-faint">File</dt><dd className={literal('m-0 truncate text-ink')} title={held.fileName}>{held.fileName}</dd>
            <dt className="text-label text-faint">Size</dt><dd className={num('m-0 text-ink')}>{formatBytes(held.bytes)}</dd>
            <dt className="text-label text-faint">Table</dt><dd className={literal('m-0 text-ink')}>{held.alias}</dd>
          </dl>
        )}
      {choosing.kind === 'refused' && <Alert tone="danger"><p className="m-0">{choosing.detail}</p></Alert>}
    </div>
  )
}

/** One line under the title that says what the last run made of this block, in the same register as the card foot. */
function BlockStatus({ outcome, running }: { readonly outcome: BlockOutcome | undefined; readonly running: boolean }) {
  if (running) return <p className={caption('m-0')} data-testid="block-status">Running.</p>
  if (outcome === undefined) return <p className={caption('m-0')} data-testid="block-status">Not run yet.</p>
  switch (outcome.kind) {
    case 'ran': return (
      <dl className={detailList} data-testid="block-status" aria-label="Block output">
        <dt className="text-label text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{formatCount(outcome.rowCount).text}</dd>
        <dt className="text-label text-faint">Columns</dt><dd className={num('m-0 text-ink')}>{formatCount(outcome.columns.length).text}</dd>
      </dl>
    )
    case 'waiting': return <p className={caption('m-0')} data-testid="block-status">Not run yet: {outcome.detail}.</p>
    case 'skipped': return <p className={caption('m-0')} data-testid="block-status">Not run: a block before it failed.</p>
    case 'failed': return <Alert tone="danger" testId="block-status"><p className="m-0">{outcome.detail}</p></Alert>
    default: return assertNever(outcome)
  }
}
