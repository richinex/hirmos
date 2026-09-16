import { useId, useRef, useState } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { WorkbenchLayout, useClosePane } from '@/components/shell/WorkbenchLayout'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ActionRow } from './ActionRow'
import { GcmEffectsPanel } from './GcmEffectsPanel'
import { useWorkflow } from '@/components/WorkflowProvider'
import { emptyCausalInputs, sameCausalSelection, type CausalAnalysis, type CausalModelEvent } from '@/domain/causalModelDraft'
import { GcmInfluencePanel } from './GcmInfluencePanel'
import type { GcmInfluenceRun } from '@/domain/gcmInfluence'
import type { GcmEffectsRun } from '@/domain/gcmEffects'
import { Icon } from '@/components/Icon'
import { formatTime } from '@/lib/format/date'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { RootCauseRunResult } from './RootCauseRunResult'
import { RootCauseChecks } from './RootCauseChecks'
import { RootCauseSettings } from './RootCauseSettings'
import { RootCauseData } from './RootCauseData'
import { GraphDetails } from './GraphDetails'
import { readObservation } from '@/domain/observation'
import { button, caption, chapterIntro, field, fieldLabel, fieldHint, panel, sectionTitle } from '@/components/ui/recipes'
import { mapNonEmpty, isNonEmpty } from '@/domain/dop'
import { describeRootCauseGraphProblem, selectedRootCauseGraph } from '@/domain/rootCause'
import { rootCauseRequestSchema, rootCauseRunSchema, type RootCauseRun, type RootCauseWorkspace, type RootCauseRequest } from '@/domain/rootCauseAnalysis'
import { rootCauseCheckRequestSchema, rootCauseCheckRecordSchema, type RootCauseCheckRecord } from '@/domain/rootCauseAnalysis'
import type { DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { selectSource, describeSourceSelectionProblem, describeDatasetProfileProblem, type SelectedSource } from '@/domain/workflow'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Analysis = CausalAnalysis
interface Props {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly documents: readonly DagDocument[]
  readonly workspace: RootCauseWorkspace
  readonly onGraph: () => void
  readonly onRun: (run: RootCauseRun) => void
  readonly onChecks: (record: RootCauseCheckRecord) => void
  readonly onDelete: (id: string) => void
  readonly onEffects: (run: GcmEffectsRun) => void
  readonly onDeleteEffects: (id: string) => void
  readonly onInfluence: (run: GcmInfluenceRun) => void
  readonly onDeleteInfluence: (id: string) => void
}

const analyses = [
  { value: 'anomaly', label: 'Unusual observation' },
  { value: 'change', label: 'Distribution change' },
  { value: 'intervention', label: 'Shift intervention' },
  { value: 'effects', label: 'Intervention effects' },
  { value: 'intrinsic', label: 'Variance contributions' },
  { value: 'arrows', label: 'Arrow strengths' },
] as const

function RunHistory({ runs, selected, onSelect, onDelete }: {
  readonly runs: readonly RootCauseRun[]
  readonly selected: string | undefined
  readonly onSelect: (id: string) => void
  readonly onDelete: (id: string) => void
}) {
  const close = useClosePane()
  return <ul className="m-0 list-none p-1 text-body" aria-label="Root-cause runs">
    {runs.length === 0 && <li className="px-3 py-2 text-faint">No root-cause runs yet.</li>}
    {[...runs].reverse().map((run) => {
      const name = analyses.find((entry) => entry.value === run.model.query.kind)!.label
      return <li key={run.id} className={`flex min-w-0 items-center gap-2 rounded-md px-2 ${selected === run.id ? 'bg-well' : ''}`}>
        <button type="button" className="flex min-h-10 min-w-0 flex-1 items-center gap-3 rounded-md text-left focus-visible:outline-2 focus-visible:outline-signal" aria-label={`${name}: ${run.comparison.name}, ${formatTime(run.createdAt)}`} aria-pressed={selected === run.id} title={`${name}: ${run.comparison.name}`} onClick={() => { onSelect(run.id); close() }}>
          <span className={caption('min-w-0 flex-1 truncate text-ink')}>{name}</span>
          <time className="shrink-0 text-label tabular-nums text-faint" dateTime={run.createdAt}>{formatTime(run.createdAt)}</time>
        </button>
        <button type="button" className="grid h-10 w-10 shrink-0 place-items-center rounded-md text-faint hover:bg-well hover:text-danger" aria-label={`Delete ${run.comparison.name} run`} title="Delete this run" onClick={() => onDelete(run.id)}><Icon name="delete" size={16} /></button>
      </li>
    })}
  </ul>
}

const descriptions: Record<'anomaly' | 'change' | 'intervention', { readonly title: string; readonly summary: string; readonly dataLabel: string; readonly dataHint: string }> = {
  anomaly: {
    title: 'Explain an unusual observation',
    summary: 'Estimate how each variable contributes to an unusual target value. The prepared data provides the baseline for comparison.',
    dataLabel: 'Unusual observation file',
    dataHint: 'Supply one row, with a column for every graph variable.',
  },
  change: {
    title: 'Explain a change between datasets',
    summary: 'Compare the new observations with the prepared baseline data. Estimate how changes in each variable’s behaviour contribute to the change in the target’s average value.',
    dataLabel: 'Comparison dataset',
    dataHint: 'Supply observations from the changed conditions, with a column for every graph variable.',
  },
  intervention: {
    title: 'Estimate outcomes after specified changes',
    summary: 'Fit the model to the selected file. Increase or decrease specified variables and estimate the resulting average target value.',
    dataLabel: 'Intervention model data',
    dataHint: 'Supply observations from the conditions you want to change, with a column for every graph variable.',
  },
}

export function RootCausePanel(props: Props) {
  const fields = useId()
  const graph = selectedRootCauseGraph(props.workspace.selection, props.documents, props.prepared)
  const draft = useWorkflow(state => state.causalModelDrafts.find(candidate => props.workspace.selection !== null && sameCausalSelection(candidate.graph, props.workspace.selection)))
  const analysis = draft?.analysis ?? 'anomaly'
  const change = useWorkflow(state => state.changeCausalModel)
  const update = (event: CausalModelEvent) => {
    if (props.workspace.selection !== null) change(props.workspace.selection, event)
  }
  const setAnalysis = (analysis: Analysis) => {
    if (props.workspace.selection !== null) change(props.workspace.selection, { type: 'analysis', analysis })
  }
  const target = draft?.attribution.target ?? ''
  const setTarget = (value: string) => update({ type: 'attribution', field: 'target', value })
  const { file, observationMode, observationDraft, shifts, confirmed, replay } = draft?.inputs ?? emptyCausalInputs
  const setFile = (file: File) => update({ type: 'file', file })
  const setObservationMode = (mode: 'values' | 'file') => update({ type: 'observation-mode', mode })
  const setConfirmed = (confirmed: boolean) => update({ type: 'confirm', confirmed })
  const setReplay = (model: RootCauseRequest | null) => update({ type: 'replay', model })
  const enteringValues = analysis === 'anomaly' && observationMode === 'values'
  const fileInput = useRef<HTMLInputElement>(null)
  const repetitions = draft?.attribution.repetitions ?? 10
  const samples = draft?.attribution.samples ?? 3000
  const changeFitting = draft?.attribution.changeFitting ?? 'halfNormalLinear'
  const seed = draft?.attribution.seed ?? 0
  const randomSource = draft?.attribution.randomSource ?? 'seed'
  const setRepetitions = (value: number) => update({ type: 'attribution', field: 'repetitions', value })
  const setSamples = (value: number) => update({ type: 'attribution', field: 'samples', value })
  const setChangeFitting = (value: 'halfNormalLinear' | 'automaticFull') => update({ type: 'attribution', field: 'changeFitting', value })
  const setSeed = (value: number) => update({ type: 'attribution', field: 'seed', value })
  const setRandomSource = (value: string) => update({ type: 'attribution', field: 'randomSource', value })
  const session = useJob(`root-cause:${props.workspace.selection?.dagRevision ?? ''}`)
  const { job } = session
  const [selected, setSelected] = useState<string | null>(null)
  const latest = props.workspace.runs.find((run) => run.id === selected) ?? props.workspace.runs.at(-1)
  const effectiveFitting = replay?.query.kind === 'change' ? replay.query.fitting.kind : changeFitting
  const checkFitting = analysis === 'change' && effectiveFitting === 'automaticFull' ? 'automatic' : 'halfNormalLinear'
  const checks = props.workspace.checks.filter((record) => record.graph.dagDocument === props.workspace.selection?.dagDocument
    && record.graph.dagRevision === props.workspace.selection?.dagRevision
    && record.graph.preparedDataset === props.prepared.id && record.model.fitting === checkFitting)
  const checked = checks.at(-1)
  const description = descriptions[analysis === 'effects' || analysis === 'intrinsic' || analysis === 'arrows' ? 'anomaly' : analysis]
  const document = props.documents.find((entry) => entry.id === props.workspace.selection?.dagDocument)
  const savedRandom = [...checks, ...props.workspace.runs.filter((record) => record.graph.dagRevision === props.workspace.selection?.dagRevision)].find((record) => record.id === randomSource)?.evidence.random

  const check = async () => {
    if (!graph.ok) return
    const current = session.start('checks', 'Preparing model checks')
    if (current === null) return
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { checkRootCause }] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const data = await materialisePrepared(props.source, props.profile, props.prepared, mapNonEmpty(graph.value.nodes, (node) => node.column))
      if (!session.current(current)) return
      if (!data.ok) throw new Error(describePreparedMaterialisationProblem(data.error))
      const model = rootCauseCheckRequestSchema.parse({ names: graph.value.nodes.map((node) => node.name), edges: graph.value.edges, rows: data.value.rowCount, seed, scope: 'fitted', fitting: checkFitting })
      if (!session.current(current)) return
      const result = await checkRootCause(data.value.values, model, progress => session.progress(current, progress.stage))
      if (!session.current(current)) return
      if (!result.ok) throw new Error(describeAnalysisWorkerProblem(result.error))
      const record = rootCauseCheckRecordSchema.parse({ id: crypto.randomUUID(), createdAt: new Date().toISOString(), graph: { dagDocument: graph.value.dagDocument, dagRevision: graph.value.dagRevision, preparedDataset: graph.value.preparedDataset }, model, evidence: result.value })
      props.onChecks(record)
      session.finish(current)
    } catch (error: unknown) {
      session.fail(current, error instanceof Error ? error.message : String(error))
    }
  }

  const run = async () => {
    if (analysis === 'effects' || analysis === 'intrinsic' || analysis === 'arrows') return
    if (!graph.ok || (!enteringValues && file === null) || !confirmed || target === '') return
    const current = session.start('analysis', 'Preparing causal model analysis')
    if (current === null) return
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { profileSource, materializeNumericColumns }, { runRootCause }] = await Promise.all([
        import('@/data/prepared'), import('@/data/duckdb'), import('@/analysis/client'),
      ])
      if (!session.current(current)) return
      const entered = enteringValues ? readObservation(graph.value, observationDraft) : null
      if (entered !== null && !entered.ok) throw new Error(entered.error)
      const inputFile = entered?.ok ? new File([entered.value.csv], 'entered-observation.csv', { type: 'text/csv' }) : file
      if (inputFile === null) throw new Error('Choose an analysis file.')
      const source = selectSource(inputFile)
      if (!source.ok) throw new Error(describeSourceSelectionProblem(source.error))
      const profile = await profileSource(source.value)
      if (!session.current(current)) return
      if (!profile.ok) throw new Error(describeDatasetProfileProblem(profile.error))
      const columns = graph.value.nodes.map((node) => profile.value.columns.find((column) => column.name === node.name)?.id)
      if (columns.some((column) => column === undefined)) throw new Error('The analysis file must contain every graph variable, with the same column names.')
      const included = columns.filter((column) => column !== undefined)
      if (!isNonEmpty(included)) throw new Error('Choose at least one graph variable.')
      const observed = await materializeNumericColumns(source.value, profile.value, included)
      if (!session.current(current)) return
      if (!observed.ok) throw new Error('The analysis columns could not be read as numeric values.')
      if (observed.value.missingCells > 0) throw new Error('Resolve missing values in the analysis file before running this model.')
      if (analysis === 'anomaly' && observed.value.rowCount !== 1) throw new Error('For an unusual-observation analysis, supply exactly one observation.')
      const baseline = await materialisePrepared(props.source, props.profile, props.prepared, mapNonEmpty(graph.value.nodes, (node) => node.column))
      if (!session.current(current)) return
      if (!baseline.ok) throw new Error(describePreparedMaterialisationProblem(baseline.error))
      const fitting = analysis === 'intervention' ? observed.value : baseline.value
      const query = analysis === 'anomaly' ? { kind: analysis, samples } : analysis === 'change'
        ? { kind: analysis, rows: observed.value.rowCount, samples, execution: { kind: 'independentJobs' }, fitting: { kind: changeFitting } }
        : { kind: analysis, rows: observed.value.rowCount, order: profile.value.columns.flatMap((column) => {
          const index = graph.value.nodes.findIndex((node) => node.name === column.name)
          return index < 0 ? [] : [index]
        }), shifts: graph.value.nodes.flatMap((node, index) => {
          const value = shifts[node.id]?.trim() ?? ''
          return value === '' ? [] : [{ node: index, amount: Number(value) }]
        }) }
      const model = rootCauseRequestSchema.safeParse(replay ?? {
        names: graph.value.nodes.map((node) => node.name), edges: graph.value.edges,
        rows: fitting.rowCount, target: Number(target), repetitions,
        upperQuantile: 0.95, fraction: analysis === 'change' ? (changeFitting === 'automaticFull' ? 1 : 0.6) : 0.75,
        random: randomSource === 'seed' ? { kind: 'seed', seed } : { kind: 'resume', state: savedRandom }, query,
      })
      if (!model.success) throw new Error(model.error.issues.map((issue) => issue.message).join(' '))
      const comparisonRows = model.data.query.kind === 'anomaly' ? 1 : model.data.query.rows
      if (model.data.rows !== fitting.rowCount || comparisonRows !== observed.value.rowCount) throw new Error('The fitting and analysis row counts must match the recorded settings.')
      const values = new Float64Array(fitting.values.length + observed.value.values.length)
      values.set(fitting.values)
      values.set(observed.value.values, fitting.values.length)
      if (!session.current(current)) return
      const result = await runRootCause(values, model.data, progress => session.progress(current, `${progress.stage}, ${progress.completed} of ${progress.total}`))
      if (!session.current(current)) return
      if (!result.ok) throw new Error(describeAnalysisWorkerProblem(result.error))
      const recorded = rootCauseRunSchema.safeParse({
        id: crypto.randomUUID(), createdAt: new Date().toISOString(),
        graph: { dagDocument: graph.value.dagDocument, dagRevision: graph.value.dagRevision, preparedDataset: graph.value.preparedDataset },
        model: model.data, evidence: result.value,
        comparison: { name: inputFile.name, fingerprint: profile.value.source.fingerprint, rows: observed.value.rowCount },
        ...(entered?.ok ? { observation: entered.value.values } : {}),
      })
      if (!recorded.success) throw new Error(recorded.error.issues.map((issue) => issue.message).join(' '))
      props.onRun(recorded.data)
      setSelected(recorded.data.id)
      session.finish(current)
    } catch (error: unknown) {
      session.fail(current, error instanceof Error ? error.message : String(error))
    }
  }

  const cancel = session.cancel

  const requirements = <div className="space-y-6">
    <section><h3 className="m-0 text-body font-medium text-ink">Understanding model checks</h3><p className={fieldHint}>KL divergence measures the difference between observed and model-generated distributions. Lower values indicate closer agreement. CRPS assesses predictive distributions; lower values indicate better predictions.</p><p className={fieldHint}>Noise-independence checks test whether a variable’s estimated noise is independent of its parent variables. A rejected check suggests that the fitted relationship needs review. Passing a graph check does not remove that concern.</p></section>
    <section><h3 className="m-0 text-body font-medium text-ink">Causal assumptions</h3><p className={fieldHint}>The graph represents the assumed causal relationships. Every variable must be observed, and the model assumes that its noise terms are independent.</p></section>
    <section><h3 className="m-0 text-body font-medium text-ink">Fitted model</h3><p className={fieldHint}>{analysis === 'change' && changeFitting === 'automaticFull' ? 'Variables without parents use their observed distributions. For each remaining variable, cross-validation compares linear regression, quadratic regression where applicable, and gradient boosting. Whole-number outcomes use discrete additive noise. The selected model classes are retained across repeated estimates, using all observations.' : 'Variables without parents use half-normal distributions. Other variables use linear regressions with empirical noise.'}</p></section>
    <section><h3 className="m-0 text-body font-medium text-ink">Analysis data</h3><p className={fieldHint}>{analysis === 'intervention' ? 'The model is fitted to the analysis file. A shift changes the selected variable by the specified amount and samples its downstream variables again.' : 'The prepared dataset represents baseline behaviour. The analysis file must use the same variable definitions, units and transformations.'}</p></section>
    <section><h3 className="m-0 text-body font-medium text-ink">Uncertainty</h3><p className={fieldHint}>Intervals show the {Number(((1 - (replay?.upperQuantile ?? 0.95)) * 100).toPrecision(12))}th and {(replay?.upperQuantile ?? 0.95) * 100}th percentiles across refitted estimates. Anomaly contributions are scores, not changes in the target’s units.</p></section>
    <section><h3 className="m-0 text-body font-medium text-ink">Prepared data</h3><dl className="m-0 mt-2 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-1 text-body"><dt className="text-faint">Source</dt><dd className="m-0 break-all text-ink">{props.source.name}</dd><dt className="text-faint">Rows</dt><dd className="m-0 tabular-nums text-ink">{props.prepared.observations.toLocaleString()}</dd></dl></section>
  </div>
  if ((analysis === 'intrinsic' || analysis === 'arrows') && graph.ok) return <GcmInfluencePanel
    key={`${graph.value.dagRevision}-${analysis}`} analysis={analysis}
    source={props.source} profile={props.profile} prepared={props.prepared}
    graph={graph.value} name={document?.name ?? 'Causal graph'} runs={props.workspace.influences}
    onRun={props.onInfluence} onDelete={props.onDeleteInfluence} onGraph={props.onGraph}
    navigation={<SegmentedControl<Analysis> ariaLabel="Analysis type" variant="line" size="sm" value={analysis} options={analyses} onChange={setAnalysis} />}
  />
  if (analysis === 'effects' && graph.ok) return <GcmEffectsPanel
    source={props.source} profile={props.profile} prepared={props.prepared}
    graph={graph.value} name={document?.name ?? 'Causal graph'} runs={props.workspace.effects}
    onRun={props.onEffects} onDelete={props.onDeleteEffects} onGraph={props.onGraph}
    navigation={<SegmentedControl<Analysis> ariaLabel="Analysis type" variant="line" size="sm" value={analysis} options={analyses} onChange={setAnalysis} />}
  />
  return <WorkbenchLayout id="root-cause" inspector={{ trigger: { label: 'Requirements', icon: 'fact_check' }, title: 'Data and method requirements', body: requirements }}
    bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Run history (${props.workspace.runs.length})`, defaultCollapsed: true, body: <RunHistory runs={props.workspace.runs} selected={latest?.id} onSelect={setSelected} onDelete={props.onDelete} /> }}
    stage={<section aria-labelledby="root-cause-title" className="@container/panel flex flex-col gap-5">
      <div><ChapterHeading id="root-cause-title" className="mb-2">Causal model analysis</ChapterHeading><p className={chapterIntro}>Explain an unusual observation, attribute a change between datasets, or estimate outcomes after specified changes to variables.</p></div>
      {!graph.ok ? <section className={panel('space-y-4 p-(--panel-space)')} aria-label="Root-cause graph selection">
        <h3 className={`${sectionTitle} m-0`}>{graph.error.kind === 'no-selection' ? 'Choose the causal graph' : 'Review the selected graph'}</h3>
        <p className={`${fieldHint} max-w-[65ch]`}>{describeRootCauseGraphProblem(graph.error)} In the editor, select <strong className="font-medium text-ink">Use for causal model analysis</strong> to return here with that revision selected.</p>
        <button type="button" className={button('signal')} onClick={props.onGraph}>Open DAG workspace</button>
      </section> : <>
        <section className={panel('p-(--panel-space)')} aria-label="Root-cause setup">
        <div className="mb-6">
          <GraphDetails name={document?.name ?? 'Analysis graph'} graph={graph.value} disabled={job.kind === 'running'} onOpen={props.onGraph} />
        </div>
        <div className="mb-4" hidden={replay !== null}><SegmentedControl<Analysis> ariaLabel="Analysis type" variant="line" size="sm" value={analysis} options={analyses} onChange={(value) => { setAnalysis(value); setSamples(value === 'change' ? 2000 : 3000) }} /></div>
        <fieldset disabled={job.kind === 'running'} className="m-0 min-w-0 space-y-4 border-0 p-0">
          <legend className="sr-only">Root-cause specification</legend>
          <div><h3 className={`${sectionTitle} m-0`}>{description.title}</h3><p className={`${fieldHint} max-w-[65ch]`}>{description.summary}</p></div>
          {analysis === 'change' && replay === null && <label><ParameterLabel className={fieldLabel} label="Conditional models" help="Automatic selection compares prediction errors across five held-out folds and retains the selected model classes. Linear models use half-normal root distributions and refit sampled subsets." /><Select aria-label="Conditional models" className={field('text', 'mt-1')} value={changeFitting} onChange={event => setChangeFitting(event.target.value as typeof changeFitting)}><option value="halfNormalLinear">Linear models</option><option value="automaticFull">Automatic selection</option></Select></label>}
          {analysis === 'anomaly' && <SegmentedControl ariaLabel="Observation input" value={observationMode} options={[{ value: 'values', label: 'Enter values' }, { value: 'file', label: 'Upload file' }]} onChange={setObservationMode} size="md" />}
          <div data-testid="root-cause-inputs" className="grid min-w-0 grid-cols-1 items-start gap-4 @lg/panel:grid-cols-2">
            <label hidden={replay !== null}><span className={fieldLabel}>Target variable</span><Select className={field('text', 'mt-1')} value={target} onChange={(event) => setTarget(event.target.value)}><option value="">Choose target</option>{graph.value.nodes.map((node, index) => <option key={node.id} value={index}>{node.name}</option>)}</Select></label>
            <div hidden={enteringValues} className="min-w-0"><span className={fieldLabel}>{description.dataLabel}</span><input ref={fileInput} aria-label={description.dataLabel} className="hidden" type="file" accept=".csv,.tsv,.parquet" onChange={(event) => { const chosen = event.target.files?.[0]; if (chosen !== undefined) { setFile(chosen) } }} /><div className="mt-1 flex flex-wrap items-center gap-2"><button type="button" className={button('outline')} onClick={() => fileInput.current?.click()}>{file === null ? 'Choose file' : 'Replace file'}</button>{file !== null && <span className="min-w-0 break-all text-label text-muted">{file.name}</span>}</div><p className={fieldHint}>{description.dataHint} Use the graph’s column names.</p></div>
          </div>
          {enteringValues && <fieldset className="m-0 min-w-0 space-y-3 border-0 p-0"><legend className={fieldLabel}>Observed values</legend><p className={fieldHint}>Enter the values recorded for the observation you want to explain, including the target.</p><div className="observation-fields">{graph.value.nodes.map((node, index) => <label key={node.id} className="flex min-w-0 flex-col justify-end gap-1.5"><span className="min-w-0 break-words text-body">{node.name}{String(index) === target && <span className="ml-2 text-label text-muted">Target</span>}</span><input aria-label={`Observed ${node.name}`} className={field('text', 'min-w-0 w-full tabular-nums')} type="text" inputMode="decimal" value={observationDraft[node.id] ?? ''} onChange={event => update({ type: 'observation', node: node.id, value: event.target.value })} /></label>)}</div></fieldset>}
          {analysis === 'intervention' && replay === null && <fieldset className="m-0 min-w-0 space-y-3 border-0 p-0"><legend className={fieldLabel}>Changes to simulate</legend><p className={`${fieldHint} max-w-[65ch]`}>Enter a positive amount to increase a variable or a negative amount to decrease it. Leave other variables blank.</p><div className="grid gap-4 @lg/panel:grid-cols-2">{graph.value.nodes.map((node) => <label key={node.id}><span className={fieldLabel}>Shift in {node.name}</span><input className={field('text', 'mt-1')} type="number" step="any" value={shifts[node.id] ?? ''} placeholder="Leave unchanged" onChange={event => update({ type: 'shift', node: node.id, value: event.target.value })} /></label>)}</div></fieldset>}
          <details hidden={replay !== null} className="pt-2 text-body"><summary className="cursor-pointer font-medium text-ink">Sampling settings</summary><div className="mt-3 grid gap-4 @lg/panel:grid-cols-2">
            <div><ParameterLabel className={fieldLabel} htmlFor={`${fields}-repetitions`} label="Refitted estimates" help="Number of times the model is refitted to sampled observations to calculate the summary and percentile bounds." /><input id={`${fields}-repetitions`} className={field('text', 'mt-1')} type="number" min={1} value={repetitions} onChange={(event) => setRepetitions(Number(event.target.value))} /></div>
            <div><ParameterLabel className={fieldLabel} htmlFor={`${fields}-seed`} label="Random seed" help="Starting value for random sampling when starting a new random sequence." /><input id={`${fields}-seed`} className={field('text', 'mt-1')} type="number" min={0} value={seed} onChange={(event) => setSeed(Number(event.target.value))} /></div>
            <div><ParameterLabel className={fieldLabel} htmlFor={`${fields}-sequence`} label="Analysis random sequence" help="Start fresh from the seed, or continue sampling from the saved random state of an earlier check or analysis." /><Select id={`${fields}-sequence`} className={field('text', 'mt-1')} value={randomSource} onChange={(event) => setRandomSource(event.target.value)}><option value="seed">Start from the random seed</option>{checks.map((record, index) => <option key={record.id} value={record.id}>Continue after model check {index + 1}</option>)}{props.workspace.runs.filter((record) => record.graph.dagRevision === props.workspace.selection?.dagRevision).map((record, index) => <option key={record.id} value={record.id}>Continue after analysis {index + 1}</option>)}</Select></div>
            {analysis !== 'intervention' && <div><ParameterLabel className={fieldLabel} htmlFor={`${fields}-samples`} label="Distribution samples" help="Number of generated observations used to approximate the model distributions in each attribution calculation." /><input id={`${fields}-samples`} className={field('text', 'mt-1')} type="number" min={1} value={samples} onChange={(event) => setSamples(Number(event.target.value))} /></div>}
          </div></details>
          <RootCauseSettings graph={graph.value} value={replay} onChange={setReplay} />
          <label className="flex max-w-[75ch] items-start gap-2 text-body"><input className="mt-0.5 shrink-0" type="checkbox" checked={confirmed} onChange={(event) => setConfirmed(event.target.checked)} />{enteringValues ? 'These values use the same units and transformations as the prepared data.' : 'The selected file uses the same variable definitions, units and transformations as the prepared data.'}</label>
        </fieldset>
        <ActionRow job={job} action="analysis" disabled={session.blocked || (!enteringValues && file === null) || !confirmed || target === ''} onRun={() => void run()} onCancel={cancel} />
        </section>
        {latest !== undefined && <div className="space-y-4"><h3 className={`${sectionTitle} m-0`}>Results</h3><RootCauseRunResult key={latest.id} run={latest} /></div>}
        <section className={panel('space-y-4 p-(--panel-space)')} aria-label="Model assessment">
          <div><h3 className={`${sectionTitle} m-0`}>Model assessment</h3><p className={`${fieldHint} max-w-[65ch]`}>Check prediction performance, fitted distributions and noise independence against the prepared data.</p></div>
          <RootCauseData graph={graph.value} source={props.source} profile={props.profile} prepared={props.prepared} />
          <ActionRow job={job} action="checks" disabled={session.blocked} onRun={() => void check()} onCancel={cancel} />
          {checked !== undefined && <RootCauseChecks record={checked} />}
        </section>
      </>}
    </section>} />
}
