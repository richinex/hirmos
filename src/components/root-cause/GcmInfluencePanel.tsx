import { SettingsDisclosure } from '@/components/ui/SettingsDisclosure'
import { useMemo, useState, type ReactNode } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { sameCausalSelection } from '@/domain/causalModelDraft'
import { WorkbenchLayout, useClosePane } from '@/components/shell/WorkbenchLayout'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { button, chapterIntroSingle, field, fieldLabel, fieldHint, iconControl, panel, sectionTitle, resultSurface, resultTitle } from '@/components/ui/recipes'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { absoluteShares, influenceBars, influenceGraph } from '@/charts/gcmInfluence'
import { useChartTheme } from '@/charts/theme'
import { downloadText } from '@/data/bundleFiles'
import { Icon } from '@/components/Icon'
import { ActionRow } from './ActionRow'
import { GraphDetails } from './GraphDetails'
import { mapNonEmpty } from '@/domain/dop'
import { gcmInfluenceRequestSchema, gcmInfluenceRunSchema, type GcmInfluenceRequest, type GcmInfluenceRun } from '@/domain/gcmInfluence'
import type { RootCauseGraph } from '@/domain/rootCause'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'

type Analysis = GcmInfluenceRequest['query']['kind']
const titles: Record<Analysis, string> = { intrinsic: 'Intrinsic variance contributions', arrows: 'Incoming-arrow strengths' }
const descriptions: Record<Analysis, string> = {
  intrinsic: 'Estimate how each variable’s own variation contributes to differences in the target, including contributions transmitted through downstream variables.',
  arrows: 'Estimate how variation in each direct parent contributes to target variation when the other parents are held fixed.',
}
const number = (value: number) => formatStatistic('raw', value).text
interface Props {
  readonly source: SelectedSource; readonly profile: DatasetProfile; readonly prepared: PreparedDatasetArtifact
  readonly graph: RootCauseGraph; readonly name: string; readonly navigation: ReactNode; readonly analysis: Analysis
  readonly runs: readonly GcmInfluenceRun[]; readonly onRun: (run: GcmInfluenceRun) => void
  readonly onDelete: (id: string) => void; readonly onGraph: () => void
}
interface Row { readonly node: number; readonly name: string; readonly value: number; readonly share: number | null }

export function GcmInfluenceResult({ run }: { readonly run: GcmInfluenceRun }) {
  const theme = useChartTheme()
  const [scale, setScale] = useState<'signed' | 'absoluteShare'>('absoluteShare')
  const [view, setView] = useState<'graph' | 'bars'>('graph')
  const { outcome } = run.evidence
  const shares = useMemo(() => absoluteShares(outcome.values), [outcome.values])
  const rows = useMemo(() => outcome.nodes.map((node, i): Row => ({ node, name: run.model.names[node]!, value: outcome.values[i]!, share: shares?.[i] ?? null })), [outcome, run.model.names, shares])
  const chart = useMemo(() => outcome.kind === 'arrows' && view === 'graph' ? influenceGraph(run, theme) : influenceBars(run, outcome.kind === 'intrinsic' ? scale : 'signed', theme), [outcome.kind, run, scale, theme, view])
  const columns: readonly EvidenceColumn<Row>[] = [
    { id: 'name', header: 'Variable', value: row => row.name },
    { id: 'value', header: outcome.kind === 'intrinsic' ? 'Variance contribution' : 'Strength', align: 'right', value: row => row.value, format: (_, row) => number(row.value) },
    ...(outcome.kind === 'intrinsic' ? [{ id: 'share', header: 'Share of absolute contributions', align: 'right' as const, value: (row: Row) => row.share ?? '—', format: (_: unknown, row: Row) => row.share === null ? '—' : `${number(row.share)}%` }] : []),
  ]
  return <section className={resultSurface()} aria-label="Causal influence result">
    <header className="flex flex-wrap items-start justify-between gap-3">
      <div className="min-w-0 flex-1 basis-64">
        <h3 className={`${resultTitle} m-0`}>{titles[outcome.kind]}</h3>
        <p className="mb-0 mt-1 text-body text-faint">{descriptions[outcome.kind]}</p>
      </div>
      <button type="button" className={button('quiet')} onClick={() => downloadText(`causal-influence-${run.id}.json`, JSON.stringify(run, null, 2))}>Export analysis record</button>
    </header>
    <div className="min-w-0 space-y-3">
      <div className="w-fit max-w-full">{outcome.kind === 'intrinsic' ? <SegmentedControl ariaLabel="Contribution scale" value={shares === null ? 'signed' : scale} options={[{ value: 'absoluteShare', label: 'Percentage shares', disabled: shares === null }, { value: 'signed', label: 'Signed estimates' }]} onChange={setScale} /> : <SegmentedControl ariaLabel="Strength plot" value={view} options={[
        { value: 'graph', label: <span title="Graph" className="flex items-center gap-1.5"><Icon name="account_tree" size={18} /><span className="sr-only sm:not-sr-only">Graph</span></span> },
        { value: 'bars', label: <span title="Bars" className="flex items-center gap-1.5"><Icon name="bar_chart" size={18} /><span className="sr-only sm:not-sr-only">Bars</span></span> },
      ]} className="[&_[data-segment-option]]:min-w-11" onChange={setView} />}</div>
      <ExpandableChart option={chart} label={titles[outcome.kind]} testId="gcm-influence-chart" style={{ height: Math.max(outcome.kind === 'arrows' ? 380 : 260, rows.length * 34 + 90) }} />
    </div>
    {outcome.kind === 'intrinsic' && <p className={fieldHint}>{shares === null ? 'All contributions are zero; percentage shares are undefined.' : 'Percentage shares divide each contribution’s absolute value by the sum of absolute values. The table retains the signed estimates.'}</p>}
    {outcome.kind === 'arrows' && view === 'graph' && <p className={fieldHint}>Coloured arrows show measured strengths. Dashed relationships were not measured in this run. Thickness represents magnitude, not whether a variable increases or decreases the target.</p>}
    <EvidenceTable title="Influence estimates" rows={rows} columns={columns} rowKey={row => String(row.node)} noun="variable" empty="No estimates." exportName="causal-influence" frame="none" />
  </section>
}

function History({ runs, selected, onSelect, onDelete }: { readonly runs: readonly GcmInfluenceRun[]; readonly selected: string | undefined; readonly onSelect: (id: string) => void; readonly onDelete: (id: string) => void }) {
  const close = useClosePane()
  return <ul className="m-0 list-none p-1 text-body" aria-label="Causal influence runs">
    {runs.length === 0 && <li className="px-3 py-2 text-faint">No influence runs yet.</li>}
    {[...runs].reverse().map(run => <li key={run.id} className={`flex min-w-0 items-center gap-2 rounded-md px-2 ${selected === run.id ? 'bg-well' : ''}`}>
      <button type="button" className="flex min-h-10 min-w-0 flex-1 items-center gap-3 rounded-md text-left focus-visible:outline-2 focus-visible:outline-signal" aria-pressed={selected === run.id} onClick={() => { onSelect(run.id); close() }}><span className="min-w-0 flex-1 truncate text-body text-ink">{titles[run.model.query.kind]}: {run.model.names[run.model.target]}</span><time className="shrink-0 text-label tabular-nums text-faint">{formatTime(run.createdAt)}</time></button>
      <button type="button" className={iconControl('danger')} aria-label="Delete influence run" onClick={() => onDelete(run.id)}><Icon name="delete" size={16} /></button>
    </li>)}
  </ul>
}

export function GcmInfluencePanel(props: Props) {
  const previous = props.runs.filter(run => run.graph.dagRevision === props.graph.dagRevision && run.model.query.kind === props.analysis).at(-1)
  const draft = useWorkflow(state => state.causalModelDrafts.find(candidate => sameCausalSelection(candidate.graph, props.graph))?.[props.analysis])
  const change = useWorkflow(state => state.changeCausalModel)
  const session = useJob(`influence:${props.graph.dagRevision}:${props.analysis}`)
  const { job } = session
  const [selected, setSelected] = useState<string | null>(null)
  if (draft === undefined) throw new Error('Causal influence requires settings for the selected graph revision.')
  const { target, query, seed } = draft
  const update = (target: string, query: GcmInfluenceRequest['query'], seed: number) => {
    switch (query.kind) {
      case 'intrinsic': change(props.graph, { type: 'intrinsic', draft: { target, query, seed } }); return
      case 'arrows': change(props.graph, { type: 'arrows', draft: { target, query, seed } }); return
    }
  }
  const setTarget = (value: string) => update(value, query, seed)
  const setQuery = (value: GcmInfluenceRequest['query']) => update(target, value, seed)
  const setSeed = (value: number) => update(target, query, value)
  const latest = props.runs.find(run => run.id === selected) ?? previous
  const busy = job.kind === 'running'
  const run = async () => {
    const current = session.start('analysis', 'Preparing causal influence')
    if (current === null) return
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runGcmInfluence }] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const data = await materialisePrepared(props.source, props.profile, props.prepared, mapNonEmpty(props.graph.nodes, node => node.column))
      if (!session.current(current)) return
      if (!data.ok) throw new Error(describePreparedMaterialisationProblem(data.error))
      const request = gcmInfluenceRequestSchema.parse({ names: props.graph.nodes.map(node => node.name), edges: props.graph.edges, rows: data.value.rowCount, target: Number(target), random: { kind: 'seed', seed }, query })
      if (!session.current(current)) return
      const result = await runGcmInfluence(data.value.values, request, progress => session.progress(current, progress.stage))
      if (!session.current(current)) return
      if (!result.ok) throw new Error(describeAnalysisWorkerProblem(result.error))
      const record = gcmInfluenceRunSchema.parse({ id: crypto.randomUUID(), createdAt: new Date().toISOString(), graph: { dagDocument: props.graph.dagDocument, dagRevision: props.graph.dagRevision, preparedDataset: props.graph.preparedDataset }, model: request, evidence: result.value })
      props.onRun(record); setSelected(record.id); session.finish(current)
    } catch (error) { session.fail(current, error instanceof Error ? error.message : String(error)) }
  }
  const cancel = session.cancel
  const settings = query.kind === 'intrinsic' ? [
    { label: 'Prediction training samples', value: query.training, change: (training: number) => setQuery({ ...query, training }) },
    { label: 'Randomization samples', value: query.randomization, change: (randomization: number) => setQuery({ ...query, randomization }) },
    { label: 'Baseline samples', value: query.baseline, change: (baseline: number) => setQuery({ ...query, baseline }) },
  ] : [
    { label: 'Conditional samples', value: query.conditional, change: (conditional: number) => setQuery({ ...query, conditional }) },
    { label: 'Maximum iterations', value: query.maxRuns, change: (maxRuns: number) => setQuery({ ...query, maxRuns }) },
    { label: 'Convergence tolerance', value: query.tolerance, change: (tolerance: number) => setQuery({ ...query, tolerance }) },
  ]
  const requirements = <div className="space-y-6">
    <section><h3 className="m-0 text-body font-medium">Variable models</h3><p className={fieldHint}>Variables without parents use their observed distributions. Other variables use five-fold model selection over linear regression, quadratic regression where applicable, and gradient boosting. Whole-number outcomes use discrete additive noise.</p></section>
    <section><h3 className="m-0 text-body font-medium">Interpretation</h3><p className={fieldHint}>These estimates concern target variance, not changes in its average. A causal interpretation depends on the recorded graph, adequate fitted relationships and independent model noise.</p></section>
    <section><h3 className="m-0 text-body font-medium">Sampling</h3><p className={fieldHint}>The model generates observations to approximate each contribution. Sampling can produce small negative estimates. This analysis does not report confidence intervals.</p></section>
  </div>
  return <WorkbenchLayout id="root-cause" inspector={{ trigger: { label: 'Requirements', icon: 'contract' }, title: 'Data and method requirements', body: requirements }} bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Run history (${props.runs.length})`, defaultCollapsed: true, body: <History runs={props.runs} selected={latest?.id} onSelect={setSelected} onDelete={props.onDelete} /> }} stage={<section className="@container/panel flex flex-col gap-5">
    <div><ChapterHeading className="mb-2">Causal model analysis</ChapterHeading><p className={chapterIntroSingle}>Explain variation in an outcome using the recorded causal graph.</p></div>
    <section className={panel('p-(--panel-space)')} aria-label="Causal influence setup">
      <div className="mb-6"><GraphDetails name={props.name} graph={props.graph} disabled={busy} onOpen={props.onGraph} /></div>
      <div className="mb-4">{props.navigation}</div>
      <fieldset disabled={busy} className="m-0 grid min-w-0 gap-6 border-0 p-0"><legend className="sr-only">Causal influence settings</legend>
        <div><h3 className={`${sectionTitle} m-0`}>{titles[props.analysis]}</h3><p className={fieldHint}>{descriptions[props.analysis]}</p></div>
        <label className="block max-w-md"><span className={fieldLabel}>Target variable</span><Select aria-label="Influence target" className={field('text', 'mt-1')} value={target} onChange={event => setTarget(event.target.value)}><option value="">Choose target</option>{props.graph.nodes.map((node, index) => <option key={node.id} value={index} disabled={props.analysis === 'arrows' && !props.graph.edges.some(([, child]) => child === index)}>{node.name}</option>)}</Select></label>
        <SettingsDisclosure title="Sampling settings" items={[...settings.map(setting => ({ icon: 'tune', text: `${setting.label} ${setting.value}` })), { icon: 'tag', text: `seed ${seed}` }]}><div className="grid gap-4 @lg/panel:grid-cols-2">
          {settings.map(setting => <label key={setting.label}><span className={fieldLabel}>{setting.label}</span><input aria-label={setting.label} type="number" min={0} step="any" value={setting.value} className={field('text', 'mt-1')} onChange={event => setting.change(Number(event.target.value))} /></label>)}
          <label><span className={fieldLabel}>Random seed</span><input aria-label="Random seed" type="number" min={0} value={seed} className={field('text', 'mt-1')} onChange={event => setSeed(Number(event.target.value))} /></label>
        </div></SettingsDisclosure>
      </fieldset><ActionRow job={job} action="analysis" progress="accessible" disabled={target === '' || session.blocked} onRun={() => void run()} onCancel={cancel} />
    </section>{latest && <GcmInfluenceResult run={latest} />}
  </section>} />
}
