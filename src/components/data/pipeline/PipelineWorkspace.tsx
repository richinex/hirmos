import { Metadata } from '@/components/ui/Metadata'
import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { Icon } from '@/components/Icon'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { button, caption, chromeAction, label, literal, num, prose } from '@/components/ui/recipes'
import { isNumericDuckDbType, type PreviewCell } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { blockLabel, describePipelineProblem, type PipelineBlock, type PipelineBlockId, type PipelineEdge, type PipelineGraph, type PipelineNode, type RowTest } from '@/domain/pipeline'
import type { SqlPreparationInput } from '@/domain/sourceInputs'
import { selectDerivedSource, type PipelineResume, type SelectedSource } from '@/domain/workflow'
import {
  addPipelineInput,
  closePipeline,
  describePipelineRunProblem,
  materializePipeline,
  openPipeline,
  previewBlock,
  removePipelineInput,
  runPipeline,
  type BlockOutcome,
  type BlockPreview,
  type PipelineRun,
  type PipelineSession,
} from '@/data/pipeline'
import { pythonScriptRuntime } from '@/data/pythonRuntime'
import { formatBytes, formatCount } from '@/lib/format/number'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'
import { EvidenceTable, type EvidenceColumn, type EvidenceValue } from '@/components/table/EvidenceTable'
import { columnDecimals, PreviewCellText } from '../previewCell'
import { BlockSettings, blockSqlText, preloadPythonEditor } from './BlockSettings'
import { usePythonRun } from './usePythonRun'
import { BLOCK_DRAG_TYPE, blockIcon, PALETTE_GROUPS, type PaletteKind } from './blockIcons'
import { PipelineCanvas } from './PipelineCanvas'
import { addBlock, addInputBlock, configureBlock, connect, describeConnectionRefusal, indexGraph, initialGraph, inputsOf, moveBlock, placeFor, removeBlock, removeEdge, tidyGraph, type ConnectionRefusal, type GraphIndex } from './pipelineWorkspaceModel'

type SessionState =
  | { readonly kind: 'opening' }
  | { readonly kind: 'ready'; readonly live: PipelineSession }
  | { readonly kind: 'failed'; readonly detail: string }

type RunState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'ran'; readonly result: PipelineRun }
  | { readonly kind: 'refused'; readonly detail: string }


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
    case 'failed': return `This block failed: ${outcome.detail}`
    case 'waiting': return `Not run yet: ${outcome.detail}.`
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
    case 'join': return `${block.how}${block.keys.length === 0 ? '' : ` on ${block.keys.map((k) => k.left === k.right ? k.left : `${k.left} = ${k.right}`).join(', ')}`}`
    case 'union': return `by ${block.by}${block.distinct ? ', distinct' : ''}`
    case 'aggregate': return `${block.groupBy.length === 0 ? 'all rows' : `by ${block.groupBy.join(', ')}`}, ${block.measures.map((m) => m.as || m.function).join(', ') || 'no measures yet'}`
    case 'sort-limit': return `${block.sort.map((s) => `${s.column} ${s.direction === 'ascending' ? '↑' : '↓'}`).join(', ') || 'unsorted'}${block.limit === null ? '' : `, first ${block.limit}`}`
    case 'script': return `${block.code.split('\n').filter((line) => line.trim().length > 0).length} lines`
    case 'output': return 'the source'
    default: return assertNever(block)
  }
}

export function PipelineWorkspace({ resume, onPrepared }: {
  /** A graph and its files to start from, when the canvas reopens on a source it made. */
  readonly resume: PipelineResume | null
  readonly onPrepared: (source: SelectedSource) => void
}) {
  const [graph, setGraph] = useState<PipelineGraph>(() => resume === null ? initialGraph([]) : resume.graph)
  // The files the session holds, mirrored into state so the inspector re-renders when a card takes or drops one.
  const [files, setFiles] = useState<readonly SqlPreparationInput[]>(() => resume === null ? [] : resume.inputs)
  const [selected, setSelected] = useState<PipelineBlockId | null>(null)
  const [session, setSession] = useState<SessionState>({ kind: 'opening' })
  const [run, setRun] = useState<RunState>({ kind: 'idle' })
  const [preview, setPreview] = useState<BlockPreview | null>(null)
  const [refusal, setRefusal] = useState<string | null>(null)
  const [materializing, setMaterializing] = useState<{ readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }>({ kind: 'idle' })
  const index = useMemo(() => indexGraph(graph), [graph])
  const isMobile = useIsMobile()

  const sessionReady = useRef<Promise<PipelineSession | null>>(Promise.resolve(null))
  useEffect(() => {
    let cancelled = false
    let opened: PipelineSession | null = null
    sessionReady.current = openPipeline(resume === null ? [] : resume.inputs, pythonScriptRuntime()).then((result) => {
      if (cancelled) { if (result.ok) void closePipeline(result.value); return null }
      if (!result.ok) { setSession({ kind: 'failed', detail: describePipelineRunProblem(result.error, String) }); return null }
      opened = result.value
      setSession({ kind: 'ready', live: result.value })
      return result.value
    })
    return () => { cancelled = true; if (opened !== null) void closePipeline(opened) }
  }, [resume])

  const nameOf = useCallback((id: PipelineBlockId): string => {
    const node = index.nodes.get(id)
    if (node === undefined) return id
    return node.block.kind === 'input' && node.block.file.kind === 'chosen' ? `${blockLabel('input')} ${node.block.file.alias}` : blockLabel(node.block.kind)
  }, [index])

  // What the pipeline computes: the blocks and the arrows, not where the cards sit. Moving a card must not rerun DuckDB.
  const semantics = useMemo(() => JSON.stringify({ nodes: graph.nodes.map((node) => [node.id, node.block]), edges: graph.edges }), [graph])
  const latest = useRef({ graph, nameOf })
  useEffect(() => { latest.current = { graph, nameOf } }, [graph, nameOf])

  // A change to the semantics reruns the pipeline after a short pause, so typing in a field does not run it per keystroke.
  // Runs queue behind one another and a run overtaken while it waited is dropped, so two edits close together never
  // race on the same view names in DuckDB.
  const runVersion = useRef(0)
  const runQueue = useRef<Promise<void>>(Promise.resolve())
  useEffect(() => {
    if (session.kind !== 'ready') return
    const live = session.live
    const version = runVersion.current + 1
    runVersion.current = version
    const handle = window.setTimeout(() => {
      runQueue.current = runQueue.current.then(async () => {
        if (runVersion.current !== version) return
        const result = await runPipeline(live, latest.current.graph)
        if (runVersion.current !== version) return
        setRun(result.ok ? { kind: 'ran', result: result.value } : { kind: 'refused', detail: describePipelineRunProblem(result.error, latest.current.nameOf) })
      })
    }, 250)
    return () => window.clearTimeout(handle)
  }, [semantics, session])

  const outcomes = useMemo(() => run.kind === 'ran' ? run.result.outcomes : new Map<PipelineBlockId, BlockOutcome>(), [run])
  const viewOf = useCallback((id: PipelineBlockId): string | null => run.kind === 'ran' ? run.result.views.get(id) ?? null : null, [run])

  // The rows of the selected block, read once per run of it; the run already knows its columns and count.
  const selectedOutcome = selected === null ? undefined : outcomes.get(selected)
  const selectedView = selected === null ? null : viewOf(selected)
  useEffect(() => {
    if (session.kind !== 'ready' || selected === null || selectedView === null || selectedOutcome?.kind !== 'ran') { setPreview(null); return }
    let cancelled = false
    void previewBlock(session.live, selectedView, selected, selectedOutcome).then((result) => {
      if (!cancelled) setPreview(result.ok ? result.value : null)
    })
    return () => { cancelled = true }
  }, [selected, selectedOutcome, selectedView, session])

  const selectedNode = selected === null ? null : index.nodes.get(selected) ?? null
  const summaries = useMemo(() => new Map(graph.nodes.map((node) => [node.id, summarise(node.block)])), [graph.nodes])

  const add = useCallback((kind: PaletteKind, position?: { readonly x: number; readonly y: number }) => {
    const current = latest.current.graph
    const added = kind === 'input'
      ? addInputBlock(position === undefined ? current : { ...current })
      : addBlock(current, kind, position ?? placeFor(current, selected))
    setGraph(position === undefined || kind !== 'input' ? added.graph : moveBlock(added.graph, added.id, position))
    setSelected(added.id)
  }, [selected])

  // A card takes a file: the session registers it, and a file the card held before is dropped once the new one is in.
  const [choosing, setChoosing] = useState<{ readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }>({ kind: 'idle' })
  const chooseFile = useCallback(async (id: PipelineBlockId, file: File) => {
    setChoosing({ kind: 'busy' })
    // A file chosen before DuckDB has opened waits for it rather than being dropped.
    const live = await sessionReady.current
    if (live === null) { setChoosing({ kind: 'refused', detail: 'DuckDB could not start, so the file cannot be read.' }); return }
    const previous = latest.current.graph.nodes.find((node) => node.id === id)?.block
    const added = await addPipelineInput(live, file)
    if (!added.ok) { setChoosing({ kind: 'refused', detail: describePipelineRunProblem(added.error, latest.current.nameOf) }); return }
    setGraph((current) => configureBlock(current, id, { kind: 'input', file: { kind: 'chosen', alias: added.value.alias } }))
    if (previous?.kind === 'input' && previous.file.kind === 'chosen') await removePipelineInput(live, previous.file.alias)
    setFiles([...live.inputs])
    setChoosing({ kind: 'idle' })
  }, [])
  const dropBlock = useCallback((id: PipelineBlockId) => {
    const node = latest.current.graph.nodes.find((candidate) => candidate.id === id)
    setGraph((current) => removeBlock(current, id))
    setSelected((current) => current === id ? null : current)
    if (session.kind === 'ready' && node?.block.kind === 'input' && node.block.file.kind === 'chosen') {
      void removePipelineInput(session.live, node.block.file.alias).then(() => setFiles([...session.live.inputs]))
    }
  }, [session])
  const tidy = useCallback(() => { void tidyGraph(latest.current.graph).then((tidied) => setGraph(tidied)) }, [])
  const refuse = (refusal: ConnectionRefusal | null) => setRefusal(refusal === null ? null : describeConnectionRefusal(refusal))
  useEffect(() => {
    if (refusal === null) return
    const handle = window.setTimeout(() => setRefusal(null), 8000)
    return () => window.clearTimeout(handle)
  }, [refusal])

  const adoptAsSource = async () => {
    if (session.kind !== 'ready') return
    setMaterializing({ kind: 'busy' })
    const output = await materializePipeline(session.live, graph)
    if (!output.ok) { setMaterializing({ kind: 'refused', detail: describePipelineRunProblem(output.error, nameOf) }); return }
    const source = selectDerivedSource(output.value.file, output.value.recipe)
    if (!source.ok) { setMaterializing({ kind: 'refused', detail: `The output could not be used as a source: ${source.error.kind}` }); return }
    setMaterializing({ kind: 'idle' })
    onPrepared(source.value)
  }

  const completeProblem = run.kind === 'ran' && !run.result.complete.ok ? describePipelineProblem(run.result.complete.error, nameOf) : null
  const outputReady = run.kind === 'ran' && run.result.complete.ok && [...run.result.outcomes.values()].every((outcome) => outcome.kind === 'ran')

  const stage = (
    <div className="flex min-h-0 flex-1 flex-col">
      {/* One row, never a stack: on a phone the chips show their glyph alone so all nine fit; wider than that they carry their labels and scroll sideways if the stage is narrower than the row. The row has no heading of its own: the chip names say what it adds. */}
      <div className="flex min-w-0 max-w-full flex-nowrap items-center gap-1 overflow-x-auto border-b border-line px-2 py-1.5 [scrollbar-width:thin]" role="toolbar" aria-label="Add a block">
        {PALETTE_GROUPS.map((group, index) => (
          <div key={group.label} className="flex shrink-0 items-center gap-0.5" role="group" aria-label={group.label}>
            {index > 0 && <span aria-hidden className="mx-0.5 h-4 w-px bg-hair" />}
            {group.kinds.map((kind) => (
              <button
                key={kind}
                type="button"
                className={chromeAction('quiet', 'shrink-0 cursor-grab whitespace-nowrap active:cursor-grabbing')}
                aria-label={blockLabel(kind)}
                title={`Click to add ${blockLabel(kind).toLowerCase()} after the selected block, or drag it onto the canvas`}
                draggable
                onDragStart={(event) => { event.dataTransfer.setData(BLOCK_DRAG_TYPE, kind); event.dataTransfer.effectAllowed = 'copy' }}
                onPointerEnter={kind === 'script' ? preloadPythonEditor : undefined}
                onFocus={kind === 'script' ? preloadPythonEditor : undefined}
                onClick={() => add(kind)}
              >
                <Icon name={blockIcon(kind)} size={16} />{isMobile ? null : <span>{blockLabel(kind)}</span>}
              </button>
            ))}
          </div>
        ))}
        <span className="ml-auto flex items-center gap-2" role="status">
          {session.kind === 'opening' && <span className={caption('m-0')}>Opening DuckDB</span>}
          {session.kind === 'failed' && <span className="text-label text-danger">{session.detail}</span>}
        </span>
      </div>
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

  const inspector = selectedNode === null
    ? (
      <div className="space-y-3">
        <p className={prose('m-0 text-muted')}>Each block is one operation on a table. Select an Input file card and choose its file, wire it into blocks, and wire the last block into "Use as source". Select a block to set it up and to see its rows below.</p>
        <p className={caption('m-0')}><Metadata><span>{formatCount(graph.nodes.length).text} blocks</span><span>{formatCount(graph.edges.length).text} arrows</span></Metadata></p>
        {completeProblem !== null && <p className={caption('m-0 text-warn')} data-testid="pipeline-incomplete">{completeProblem}</p>}
        {files.length > 0 && (
          <div className="border-t border-hair pt-3">
            <span className={label('block text-muted')}>Files</span>
            <ul className="m-0 mt-1 list-none space-y-0.5 p-0">
              {files.map((input) => <li key={input.alias} className="flex items-baseline gap-2 text-body text-ink"><span className={literal('truncate')}>{input.alias}</span><span className="ml-auto shrink-0 text-micro text-faint"><Metadata><span>{input.fileName}</span><span>{formatBytes(input.bytes)}</span></Metadata></span></li>)}
            </ul>
          </div>
        )}
      </div>
    )
    : <Inspector node={selectedNode} index={index} outcomes={outcomes} viewOf={viewOf} files={files} choosing={choosing} onChooseFile={(file) => void chooseFile(selectedNode.id, file)} onChange={(block) => setGraph((current) => configureBlock(current, selectedNode.id, block))} onRemove={() => dropBlock(selectedNode.id)} />

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
  const bottom = (
    <div className="flex h-full min-h-0 flex-col">
      {preview === null
        ? <p className={caption('m-3')}>{previewNotice(selectedNode === null ? undefined : outcomes.get(selectedNode.id), selectedNode === null)}</p>
        : (
          <div className="min-h-0 flex-1 overflow-auto p-3">
            <EvidenceTable
              appearance="data"
              title={selectedNode === null ? 'Block' : blockLabel(selectedNode.block.kind)}
              rows={previewRows}
              columns={previewColumns}
              rowKey={(row) => String(row.index)}
              noun="row"
              empty="The block produced no rows."
              total={preview.rowCount}
              exportName={selectedNode === null ? undefined : `${selectedNode.id}-preview`}
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
          title: selectedNode === null ? 'Pipeline' : blockLabel(selectedNode.block.kind),
          body: inspector,
          controls: (
            <div className="flex items-center gap-2">
              {materializing.kind === 'refused' && <span className="max-w-[16rem] truncate text-label text-danger" title={materializing.detail}>{materializing.detail}</span>}
              <button type="button" className={button('signal', 'h-7 px-3 text-label', 'sm')} disabled={!outputReady || materializing.kind === 'busy'} aria-busy={materializing.kind === 'busy'} title={completeProblem ?? undefined} onClick={() => void adoptAsSource()}>Use as source</button>
            </div>
          ),
        }}
        bottom={{ title: previewTitle, body: bottom, defaultSize: 220 }}
      />
    </section>
  )
}

function Inspector({ node, index, outcomes, viewOf, files, choosing, onChooseFile, onChange, onRemove }: {
  readonly node: PipelineNode
  readonly index: GraphIndex
  readonly outcomes: ReadonlyMap<PipelineBlockId, BlockOutcome>
  readonly viewOf: (id: PipelineBlockId) => string | null
  readonly files: readonly SqlPreparationInput[]
  readonly choosing: { readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }
  readonly onChooseFile: (file: File) => void
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
  return (
    <div className="space-y-4">
      {node.block.kind === 'input' && <InputFileField held={held} choosing={choosing} />}
      {!(node.block.kind === 'input' && node.block.file.kind === 'empty') && <BlockStatus outcome={outcome} running={running !== null} />}
      {outcome?.kind === 'ran' && (
        <details className="group/schema">
          <summary className={label('flex cursor-pointer list-none items-center gap-1 text-muted hover:text-ink')}><Metadata><span><Icon name="chevron_right" size={14} className="transition-transform group-open/schema:rotate-90" /> Columns</span><span>{outcome.columns.length}</span></Metadata></summary>
          <ul className="m-0 mt-1.5 list-none space-y-0.5 p-0" data-testid="block-schema">
            {outcome.columns.map((column) => <li key={column.name} className="flex items-baseline gap-2 text-body"><span className={literal('truncate text-ink')}>{column.name}</span><span className="ml-auto shrink-0 text-micro text-faint">{column.type}</span></li>)}
          </ul>
        </details>
      )}
      <BlockSettings node={node} inputColumns={inputColumns} inputNames={inputNames} onChange={onChange} />
      {(outcome?.kind === 'ran' || outcome?.kind === 'failed') && outcome.stdout !== undefined && outcome.stdout.length > 0 && (
        <div>
          <span className={label('block text-muted')}>Printed</span>
          <pre className={cn(literal(), 'mt-1 max-h-48 overflow-auto rounded-md border border-hair bg-well px-2.5 py-2 text-[11px] leading-relaxed text-bone')} data-testid="block-stdout">{outcome.stdout}</pre>
        </div>
      )}
      {sql !== null && (
        <details className="group/sql">
          <summary className={label('flex cursor-pointer list-none items-center gap-1 text-muted hover:text-ink')}><Icon name="chevron_right" size={14} className="transition-transform group-open/sql:rotate-90" /> As SQL</summary>
          <pre className={cn(literal(), 'mt-2 overflow-x-auto rounded-md border border-hair bg-well px-2.5 py-2 text-[11px] leading-relaxed text-bone')} data-testid="block-sql">{sql}</pre>
        </details>
      )}
      {removable && (
        <div className="flex flex-wrap items-center justify-between gap-2 border-t border-hair pt-3">
          {node.block.kind === 'input'
            ? (
              <button type="button" className={button(held === null ? 'signal' : 'quiet', 'whitespace-nowrap', 'sm')} disabled={choosing.kind === 'busy'} aria-busy={choosing.kind === 'busy'} onClick={() => picker.current?.click()}>
                <Icon name="upload_file" size={14} /> {held === null ? 'Choose file' : 'Replace file'}
              </button>
            )
            : <span />}
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
function InputFileField({ held, choosing }: {
  readonly held: SqlPreparationInput | null
  readonly choosing: { readonly kind: 'idle' } | { readonly kind: 'busy' } | { readonly kind: 'refused'; readonly detail: string }
}) {
  return (
    <div className="space-y-2">
      {held === null
        ? <p className={caption('m-0')}>Choose a CSV, TSV or Parquet file, then connect to the next block to continue processing.</p>
        : (
          <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-0.5 text-body">
            <dt className="text-muted">File</dt><dd className={literal('m-0 truncate text-ink')} title={held.fileName}>{held.fileName}</dd>
            <dt className="text-muted">Size</dt><dd className={num('m-0 text-ink')}>{formatBytes(held.bytes)}</dd>
            <dt className="text-muted">Table</dt><dd className={literal('m-0 text-ink')}>{held.alias}</dd>
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
    case 'ran': return <p className={cn(label('m-0 text-muted'), num())} data-testid="block-status"><Metadata><span>{formatCount(outcome.rowCount).text} rows</span><span>{outcome.columns.length} {outcome.columns.length === 1 ? 'column' : 'columns'}</span></Metadata></p>
    case 'waiting': return <p className={caption('m-0')} data-testid="block-status">Not run yet: {outcome.detail}.</p>
    case 'skipped': return <p className={caption('m-0')} data-testid="block-status">Not run: a block before it failed.</p>
    case 'failed': return <Alert tone="danger" testId="block-status"><p className="m-0">{outcome.detail}</p></Alert>
    default: return assertNever(outcome)
  }
}
