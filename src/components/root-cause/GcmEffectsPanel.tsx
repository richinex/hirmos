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
import { button, chapterIntro, field, fieldLabel, fieldHint, iconControl, panel, sectionTitle, resultSurface, resultTitle } from '@/components/ui/recipes'
import { downloadText } from '@/data/bundleFiles'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { runComparisonOption } from '@/charts/estimation/runComparison'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { ActionRow } from './ActionRow'
import { GraphDetails } from './GraphDetails'
import { mapNonEmpty } from '@/domain/dop'
import { gcmEffectsRequestSchema, gcmEffectsRunSchema, type GcmEffectsRequest, type GcmEffectsRun } from '@/domain/gcmEffects'
import type { RootCauseGraph } from '@/domain/rootCause'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'

interface Props {
  readonly source: SelectedSource; readonly profile: DatasetProfile; readonly prepared: PreparedDatasetArtifact
  readonly graph: RootCauseGraph; readonly name: string; readonly navigation: ReactNode
  readonly runs: readonly GcmEffectsRun[]; readonly onRun: (run: GcmEffectsRun) => void
  readonly onDelete: (id: string) => void; readonly onGraph: () => void
}
const number = (value: number) => formatStatistic('raw', value).text
const optimizerMessages: Record<GcmEffectsRun['evidence']['optimizer']['status'], string> = {
  converged: 'The summary optimizer converged.',
  iterationLimit: 'The summary optimizer reached its iteration limit. Review the result before interpreting it.',
  precisionLoss: 'Desired error not necessarily achieved due to precision loss.',
}
interface EffectRow { readonly label: string; readonly estimate: number; readonly lower: number; readonly upper: number }
const columns: readonly EvidenceColumn<EffectRow>[] = [
  { id: 'label', header: 'Population', value: row => row.label },
  ...(['estimate', 'lower', 'upper'] as const).map((id): EvidenceColumn<EffectRow> => ({
    id, header: { estimate: 'Effect', lower: 'Lower percentile', upper: 'Upper percentile' }[id],
    value: row => row[id], align: 'right', format: (_, row) => number(row[id]),
  })),
]
export function GcmEffectResult({ run }: { readonly run: GcmEffectsRun }) {
  const theme = useChartTheme()
  const rows = useMemo(() => {
    const grouping = run.evidence.grouping
    const labels = grouping.kind === 'none' ? ['All rows'] : ['All rows', ...grouping.counts.map((_, i) => {
      const name = run.model.names[grouping.column]
      const low = i === 0 && grouping.scope.kind === 'lowerBound' ? grouping.scope.minimum : grouping.cutpoints[i - 1]
      const high = grouping.cutpoints[i]
      if (low === undefined) return `${name} below ${number(high!)}`
      if (high === undefined) return `${name} at least ${number(low)}`
      return `${name}: ${number(low)} to below ${number(high)}`
    })]
    return labels.map((label, i): EffectRow => ({ label, estimate: run.evidence.estimates[i]!, lower: run.evidence.bounds[i]![0], upper: run.evidence.bounds[i]![1] }))
  }, [run])
  const chart = useMemo(() => runComparisonOption(rows.map(row => ({ ...row, current: false })), theme), [rows, theme])
  const effect = run.evidence.estimates[0]!
  return <section className={resultSurface('space-y-5')} aria-label="Intervention effect results">
    <div className="flex flex-wrap items-start justify-between gap-3"><h3 className={`${resultTitle} m-0`}>Intervention effects</h3><button type="button" className={button('quiet')} onClick={() => downloadText(`intervention-effects-${run.id}.json`, JSON.stringify(run, null, 2))}>Export analysis record</button></div>
    <h3 className={`${sectionTitle} m-0`}>What this result means</h3>
    <div><h4 className="m-0 text-body font-medium">Bottom line</h4><p className={fieldHint}>In the simulated population, setting {run.model.names[run.model.treatment]} to 1 rather than 0 gives an estimated {number(Math.abs(effect))} {effect < 0 ? 'lower' : 'higher'} {run.model.names[run.model.outcome]} on average.</p></div>
    <div><h4 className="m-0 text-body font-medium">Uncertainty</h4><p className={fieldHint}>Across {run.model.repetitions} simulations, the {number(run.evidence.quantiles[0] * 100)}th to {number(run.evidence.quantiles[1] * 100)}th percentile range for the average difference is {number(rows[0]!.lower)} to {number(rows[0]!.upper)}. The models stay fixed during these simulations; this range does not include uncertainty from fitting them.</p></div>
    <div><h4 className="m-0 flex items-center gap-1.5 text-body font-medium"><Icon name="gavel" size={14} className="text-faint" />What must be true</h4><p className={fieldHint}>A causal interpretation depends on the recorded graph, the fitted relationships and independent model noise. The relationships must remain applicable under the simulated intervention.</p></div>
    <ExpandableChart option={chart} label="Simulated intervention effects and percentile ranges" className="h-[300px]" testId="gcm-effects-chart" />
    <EvidenceTable title="Effect estimates" rows={rows} columns={columns} rowKey={row => row.label} noun="estimate" empty="No estimates." exportName="gcm-intervention-effects" frame="none" />
    {run.evidence.grouping.kind === 'quantiles' && run.evidence.grouping.excluded > 0 && <p className={fieldHint}>{run.evidence.grouping.excluded} rows fall below the selected group boundary. They remain in the overall estimate.</p>}
    {run.evidence.optimizer.status !== 'converged' && <p className="text-body text-warn">{optimizerMessages[run.evidence.optimizer.status]}</p>}
  </section>
}
function History({ runs, selected, onSelect, onDelete }: { readonly runs: readonly GcmEffectsRun[]; readonly selected: string | undefined; readonly onSelect: (id: string) => void; readonly onDelete: (id: string) => void }) {
  const close = useClosePane()
  return <ul className="m-0 list-none p-1 text-body" aria-label="Intervention effect runs">
    {runs.length === 0 && <li className="px-3 py-2 text-faint">No intervention effect runs yet.</li>}
    {[...runs].reverse().map(run => <li key={run.id} className={`flex min-w-0 items-center gap-2 rounded-md px-2 ${selected === run.id ? 'bg-well' : ''}`}>
      <button type="button" className="flex min-h-10 min-w-0 flex-1 items-center gap-3 rounded-md text-left focus-visible:outline-2 focus-visible:outline-signal" aria-pressed={selected === run.id} onClick={() => { onSelect(run.id); close() }}>
        <span className="min-w-0 flex-1 truncate text-body text-ink">{run.model.names[run.model.treatment]} → {run.model.names[run.model.outcome]}</span><time className="shrink-0 text-label tabular-nums text-faint">{formatTime(run.createdAt)}</time>
      </button><button type="button" className={iconControl('danger')} aria-label="Delete intervention effect run" onClick={() => onDelete(run.id)}><Icon name="delete" size={16} /></button>
    </li>)}
  </ul>
}

type Quantiles = Extract<GcmEffectsRequest['grouping'], { kind: 'quantiles' }>
function QuantileGroups({ value, onChange }: { readonly value: Quantiles; readonly onChange: (value: Quantiles) => void }) {
  return <>
    <label><span className={fieldLabel}>Quantile groups</span><input aria-label="Quantile groups" type="number" min={2} max={10} className={field('text', 'mt-1')} value={value.groups} onChange={event => onChange({ ...value, groups: Number(event.target.value) })} /></label>
    <div><span className={fieldLabel}>Group inclusion</span><SegmentedControl className="mt-1" ariaLabel="Group inclusion" value={value.scope.kind} options={[{ value: 'all', label: 'All rows' }, { value: 'lowerBound', label: 'Lower bound' }]} onChange={kind => onChange({ ...value, scope: kind === 'all' ? { kind } : { kind, minimum: 0 } })} /></div>
    {value.scope.kind === 'lowerBound' && <label><span className={fieldLabel}>Minimum group value</span><input aria-label="Minimum group value" type="number" className={field('text', 'mt-1')} value={value.scope.minimum} onChange={event => onChange({ ...value, scope: { kind: 'lowerBound', minimum: Number(event.target.value) } })} /><p className={fieldHint}>Values below this boundary are excluded from groups, but remain in the overall estimate. Quantile boundaries use all rows.</p></label>}
  </>
}

export function GcmEffectsPanel(props: Props) {
  const previous = props.runs.filter(run => run.graph.dagRevision === props.graph.dagRevision).at(-1)
  const model = useWorkflow(state => state.causalModelDrafts.find(candidate => sameCausalSelection(candidate.graph, props.graph))?.effects)
  const change = useWorkflow(state => state.changeCausalModel)
  const setModel = (model: GcmEffectsRequest) => change(props.graph, { type: 'effects', model })
  const session = useJob(`effects:${props.graph.dagRevision}`)
  const { job } = session
  const [selected, setSelected] = useState<string | null>(null)
  if (model === undefined) throw new Error('Intervention effects require settings for the selected graph revision.')
  const latest = props.runs.find(run => run.id === selected) ?? previous
  const busy = job.kind === 'running'
  const run = async () => {
    const current = session.start('analysis', 'Preparing intervention effects')
    if (current === null) return
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runGcmEffects }] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const data = await materialisePrepared(props.source, props.profile, props.prepared, mapNonEmpty(props.graph.nodes, node => node.column))
      if (!session.current(current)) return
      if (!data.ok) throw new Error(describePreparedMaterialisationProblem(data.error))
      const request = gcmEffectsRequestSchema.safeParse({ ...model, rows: data.value.rowCount })
      if (!request.success) throw new Error(request.error.issues.map(issue => issue.message).join(' '))
      if (!session.current(current)) return
      const result = await runGcmEffects(data.value.values, request.data, progress => session.progress(current, `${progress.stage}: ${progress.completed} of ${progress.total}`))
      if (!session.current(current)) return
      if (!result.ok) throw new Error(describeAnalysisWorkerProblem(result.error))
      const record = gcmEffectsRunSchema.parse({ id: crypto.randomUUID(), createdAt: new Date().toISOString(),
        graph: { dagDocument: props.graph.dagDocument, dagRevision: props.graph.dagRevision, preparedDataset: props.graph.preparedDataset },
        model: request.data, evidence: result.value })
      props.onRun(record); setSelected(record.id); session.finish(current)
    } catch (error) { session.fail(current, error instanceof Error ? error.message : String(error)) }
  }
  const cancel = session.cancel
  const requirements = <div className="space-y-6">
    <section><h3 className="m-0 text-body font-medium">Variable models</h3><p className={fieldHint}>Variables without parents retain their observed distribution. Other variables use a random-forest regression with empirical noise, or a categorical classifier. Categories must be coded consecutively from 0; categorical parents use one-hot encoding.</p></section>
    <section><h3 className="m-0 text-body font-medium">Intervention</h3><p className={fieldHint}>The treatment is assigned 0 or 1 with equal probability in each simulation. Affected variables are sampled again; unaffected observed covariates remain together. Grouping is limited to variables without parents.</p></section>
    <section><h3 className="m-0 text-body font-medium">Simulation uncertainty</h3><p className={fieldHint}>The forests are fitted once. Percentile ranges describe repeated intervention simulations, not bootstrap refitting or simultaneous coverage. The central estimate is the geometric median of the simulation vectors.</p></section>
    {latest && <section><h3 className="m-0 text-body font-medium">Summary calculation</h3><p className={fieldHint}>{optimizerMessages[latest.evidence.optimizer.status]}</p></section>}
  </div>
  return <WorkbenchLayout id="root-cause" inspector={{ trigger: { label: 'Requirements', icon: 'contract' }, title: 'Data and method requirements', body: requirements }}
    bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Run history (${props.runs.length})`, defaultCollapsed: true, body: <History runs={props.runs} selected={latest?.id} onSelect={setSelected} onDelete={props.onDelete} /> }}
    stage={<section className="@container/panel flex flex-col gap-5">
      <div><ChapterHeading className="mb-2">Causal model analysis</ChapterHeading><p className={chapterIntro}>Estimate intervention effects using the prepared data and the recorded causal graph.</p></div>
      <section className={panel('p-(--panel-space)')} aria-label="Intervention effects setup">
        <div className="mb-6"><GraphDetails name={props.name} graph={props.graph} disabled={busy} onOpen={props.onGraph} /></div>
        <div className="mb-4">{props.navigation}</div>
        <fieldset disabled={busy} className="m-0 grid min-w-0 gap-6 border-0 p-0"><legend className="sr-only">Intervention effect settings</legend>
          <div><h3 className={`${sectionTitle} m-0`}>Estimate the effect of an intervention</h3><p className={fieldHint}>Compare simulated outcomes when the treatment is set to 1 rather than 0.</p></div>
          <div className="grid min-w-0 gap-4 @lg/panel:grid-cols-2">
            {(['treatment', 'outcome'] as const).map(role => <label key={role}><span className={fieldLabel}>{role === 'treatment' ? 'Treatment' : 'Outcome'}</span><Select aria-label={role === 'treatment' ? 'GCM treatment' : 'GCM outcome'} className={field('text', 'mt-1')} value={String(model[role])} onChange={event => setModel({ ...model, [role]: Number(event.target.value) })}>{model.names.map((name, i) => <option key={name} value={i}>{name}</option>)}</Select></label>)}
            <label><span className={fieldLabel}>Group effects by</span><Select aria-label="Group effects by" className={field('text', 'mt-1')} value={model.grouping.kind === 'none' ? '' : String(model.grouping.column)} onChange={event => setModel({ ...model, grouping: event.target.value === '' ? { kind: 'none' } : { kind: 'quantiles', column: Number(event.target.value), groups: 5, scope: { kind: 'all' } } })}><option value="">All rows</option>{model.names.map((name, i) => !model.edges.some(([, child]) => child === i) && i !== model.treatment && i !== model.outcome ? <option key={name} value={i}>{name}</option> : null)}</Select></label>
            {model.grouping.kind === 'quantiles' && <QuantileGroups value={model.grouping} onChange={grouping => setModel({ ...model, grouping })} />}
          </div>
          <SettingsDisclosure title="Variable models" items={[
            { icon: 'show_chart', text: `${model.mechanisms.filter(m => m.kind === 'regression').length} continuous` },
            { icon: 'category', text: `${model.mechanisms.filter(m => m.kind === 'classifier').length} categorical` },
            { icon: 'bar_chart', text: `${model.mechanisms.filter(m => m.kind === 'empirical').length} observed distributions` },
          ]}><div className="grid gap-4 @lg/panel:grid-cols-2">{model.mechanisms.map((mechanism, i) => <div key={model.names[i]}>
            <span className={fieldLabel}>{model.names[i]}</span>
            {mechanism.kind === 'empirical' ? <p className={fieldHint}>Observed distribution</p> : <>
              <SegmentedControl className="mt-1" fill ariaLabel={`Model for ${model.names[i]}`} value={mechanism.kind} options={[{ value: 'regression', label: 'Continuous' }, { value: 'classifier', label: 'Categorical' }]} onChange={kind => setModel({ ...model, mechanisms: model.mechanisms.map((entry, j) => j !== i ? entry : kind === 'regression' ? { kind } : { kind, classes: 2 }) })} />
              {mechanism.kind === 'classifier' && <label className="mt-2 block"><span className={fieldLabel}>Categories, coded from 0</span><input aria-label={`Categories for ${model.names[i]}`} type="number" min={2} max={7} value={mechanism.classes} className={field('text', 'mt-1')} onChange={event => setModel({ ...model, mechanisms: model.mechanisms.map((entry, j) => j !== i ? entry : { kind: 'classifier', classes: Number(event.target.value) }) })} /></label>}
            </>}
          </div>)}</div></SettingsDisclosure>
          <SettingsDisclosure title="Sampling settings" items={[
            { icon: 'forest', text: `${model.trees} trees per model` },
            { icon: 'repeat', text: `${model.repetitions} simulations` },
            { icon: 'tag', text: `seeds ${model.fitSeed} and ${model.simulationSeed}` },
          ]}><div className="grid gap-4 @lg/panel:grid-cols-2">
            {([{ key: 'trees', label: 'Trees per model', min: 1 }, { key: 'minLeaf', label: 'Minimum leaf size', min: 1 }, { key: 'fitSeed', label: 'Fitting seed', min: 0 }, { key: 'simulationSeed', label: 'Simulation seed', min: 0 }, { key: 'repetitions', label: 'Simulations', min: 1 }] as const).map(item => <label key={item.key}><span className={fieldLabel}>{item.label}</span><input aria-label={item.label} type="number" min={item.min} value={model[item.key]} className={field('text', 'mt-1')} onChange={event => setModel({ ...model, [item.key]: Number(event.target.value) })} /></label>)}
            <label><span className={fieldLabel}>Upper percentile</span><input aria-label="Upper percentile" type="number" min={51} max={99.9} step={0.1} value={model.upperQuantile * 100} className={field('text', 'mt-1')} onChange={event => setModel({ ...model, upperQuantile: Number(event.target.value) / 100 })} /></label>
          </div></SettingsDisclosure>
        </fieldset><ActionRow job={job} action="analysis" disabled={session.blocked} onRun={() => void run()} onCancel={cancel} />
      </section>
      {latest && <GcmEffectResult run={latest} />}
    </section>} />
}
