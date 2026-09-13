import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useCallback, useState, type ReactNode } from 'react'
import { COUNT_SERIES_DIAGNOSTIC_METHODS, TIME_SERIES_METHODS } from '@/domain/methods'
import { TimeSeriesRequirements } from './TimeSeriesRequirements'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { CountSeriesCard } from './CountSeriesCard'
import { TimeSeriesRunResult } from './TimeSeriesRunResult'
import { TimeSeriesHistory } from './TimeSeriesHistory'
import { ArdlModelPanel } from './ArdlModelPanel'
import { Orb } from '@/components/ui/Orb'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, chapterIntro, field, fieldLabel, fieldHint, panel, sectionTitle } from '@/components/ui/recipes'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { ARDL_TERMS, newTimeSeriesRunId, parseTimeSeriesRun, timeSeriesRunLabel, type ArdlTerms, type TimeSeriesRun, type TimeSeriesRunId } from '@/domain/timeSeries'
import type { CountSeriesModelArtifact, CountSeriesModelId } from '@/domain/countSeries'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import type { RunActivity } from '@/domain/activity'
import { useRunActivity } from '@/lib/useRunActivity'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Model = 'ardl' | 'vecm'
type Job = { readonly kind: 'idle' } | { readonly kind: 'running' } | { readonly kind: 'failed'; readonly detail: string }
export type TimeSeriesPanelProps = {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { kind: 'prepared-time-series' }>
  readonly runs: readonly TimeSeriesRun[]
  readonly counts: readonly CountSeriesModelArtifact[]
  readonly onRun: (run: TimeSeriesRun) => void
  readonly onDeleteRun: (id: TimeSeriesRunId) => void
  readonly onCount: (run: CountSeriesModelArtifact) => void
  readonly onDeleteCount: (id: CountSeriesModelId) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}

type Props = TimeSeriesPanelProps
export function TimeSeriesHeading() {
  return <div>
    <ChapterHeading className="mb-2">Time-series analysis</ChapterHeading>
    <p className={chapterIntro} style={{ columnCount: 1 }}>Analyse how observations change over time and how current values relate to earlier values. Estimate short-run dynamics and long-run relationships between series.</p>
  </div>
}

function LongRunModel({ model, selector, ...props }: Props & { readonly model: Model; readonly selector: ReactNode }) {
  const columns = props.profile.columns.filter((c) => props.prepared.columns.includes(c.id) && isNumericDuckDbType(c.duckdbType))
  const previous = props.runs.filter((run): run is Exclude<TimeSeriesRun, {kind: 'ardl-model'}> => run.kind === model).at(-1)
  const [selected, setSelected] = useState<readonly ColumnId[]>(() => previous === undefined ? [] : previous.kind === 'ardl' ? [previous.outcome.id, previous.predictor.id] : previous.variables.map((variable) => variable.id))
  const [maxLag, setMaxLag] = useState(() => previous === undefined ? 2 : previous.kind === 'ardl' ? previous.specification.maxLag : previous.specification.maxLags)
  const [terms, setTerms] = useState<ArdlTerms>(() => previous?.kind === 'ardl' ? previous.specification.terms : 'constant')
  const [deterministic, setDeterministic] = useState<'n' | 'co' | 'ci' | 'coli'>(() => previous?.kind === 'vecm' ? previous.specification.deterministic : 'ci')
  const [significance, setSignificance] = useState<90 | 95 | 99>(() => previous?.kind === 'vecm' ? previous.specification.significance : 95)
  const [job, setJob] = useState<Job>({ kind: 'idle' })
  const [forecastSteps,setForecastSteps]=useState('')
  useRunActivity(props.onActivity, job.kind === 'running' ? { label: model.toUpperCase(), progress: null } : null)
  const runs = props.runs.filter((run) => run.kind === model)
  const minimumRows = model === 'ardl' ? 6 * (maxLag + 1) + 10 : (maxLag + 2) * selected.length * 3 + 10
  const ready = selected.length >= 2 && (model !== 'ardl' || selected.length === 2) && props.prepared.observations >= minimumRows

  const run = async () => {
    if (!ready || selected[0] === undefined) return
    setJob({ kind: 'running' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(props.source, props.profile, props.prepared, selected as NonEmptyArray<ColumnId>)
      if (!matrix.ok) { setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(matrix.error) }); return }
      const { values, rowCount, columns: used, timeAxis } = matrix.value
      if (timeAxis === null) { setJob({ kind: 'failed', detail: 'The prepared time series has no recorded time key.' }); return }
      const plotTime = { kind: timeAxis.kind, values: Array.from(timeAxis.kind === 'calendar' ? timeAxis.timestamps : timeAxis.values) }
      const variables = used.map(({ id, name }) => ({ id, name }))
      const identity = { id: newTimeSeriesRunId(), preparedDataset: props.prepared.id, createdAt: new Date().toISOString(), plotTime }
      const fitted = await (async () => {
        switch (model) {
          case 'ardl': {
            const specification = { maxLag, terms }
            const choice = ARDL_TERMS[terms]
            const evidence = await analysis.runArdlPss(values, rowCount, used.length, { outcome: 0, treatment: 1, maxLag, trend: choice.trend, case: choice.case })
            return evidence.ok ? parseTimeSeriesRun({ ...identity, kind: 'ardl', outcome: variables[0], predictor: variables[1], specification, evidence: evidence.value }) : { ok: false as const, error: describeAnalysisWorkerProblem(evidence.error) }
          }
          case 'vecm': {
            const specification = { maxLags: maxLag, deterministic, significance, forecastSteps:forecastSteps.trim()===''?null:Number(forecastSteps) }
            const evidence = await analysis.runVecm(values, rowCount, used.length, { endogenous: used.map((_, i) => i), maxLags: maxLag, deterministic, significance: [90, 95, 99].indexOf(significance), breakIndex: null, forecastSteps:specification.forecastSteps })
            return evidence.ok ? parseTimeSeriesRun({ ...identity, kind: 'vecm', variables, specification, evidence: evidence.value }) : { ok: false as const, error: describeAnalysisWorkerProblem(evidence.error) }
          }
          default: return assertNever(model)
        }
      })()
      if (!fitted.ok) { setJob({ kind: 'failed', detail: fitted.error }); return }
      props.onRun(fitted.value)
      setJob({ kind: 'idle' })
    } catch (cause: unknown) { setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) }) }
  }

  const controls = <><fieldset disabled={job.kind === 'running'} className="m-0 grid min-w-0 grid-cols-1 items-start gap-4 border-0 p-0 @lg/panel:grid-cols-2">
    <legend className="sr-only">Model specification</legend>
    {model === 'ardl' ? <>
      <label className="block"><span className={fieldLabel}>Outcome series</span><Select className={field('text', 'mt-1')} value={selected[0] ?? ''} onChange={(e) => { const id = columns.find((c) => c.id === e.target.value)?.id; setSelected(id === undefined ? [] : [id, ...selected.slice(1).filter((other) => other !== id)]) }}><option value="">Choose outcome</option>{columns.map((c) => <option key={c.id} value={c.id}>{c.name}</option>)}</Select></label>
      <label className="block"><span className={fieldLabel}>Predictor series</span><Select disabled={selected[0] === undefined} className={field('text', 'mt-1')} value={selected[1] ?? ''} onChange={(e) => { const id = columns.find((c) => c.id === e.target.value)?.id; if (selected[0] !== undefined) setSelected(id === undefined ? [selected[0]] : [selected[0], id]) }}><option value="">Choose predictor</option>{columns.filter((c) => c.id !== selected[0]).map((c) => <option key={c.id} value={c.id}>{c.name}</option>)}</Select></label>
      <label className="block"><span className={fieldLabel}>Deterministic terms</span><Select className={field('text', 'mt-1')} value={terms} onChange={(e) => { const entry = Object.keys(ARDL_TERMS).find((key) => key === e.target.value); if (entry !== undefined) setTerms(entry as ArdlTerms) }}>{Object.entries(ARDL_TERMS).map(([key, value]) => <option key={key} value={key}>{value.label}</option>)}</Select></label>
    </> : <>
      <div className="min-w-0 @lg/panel:col-span-2"><ColumnChecklist title="Series in the system" help="Choose at least two series. The model estimates their relationships jointly." columns={columns} selected={selected} onChange={setSelected} /></div>
      <label className="block"><span className={fieldLabel}>Deterministic terms</span><Select className={field('text', 'mt-1')} value={deterministic} onChange={(e) => { const value = e.target.value; if (value === 'n' || value === 'co' || value === 'ci' || value === 'coli') setDeterministic(value) }}><option value="n">None</option><option value="ci">Constant within equilibrium</option><option value="co">Constant outside equilibrium</option><option value="coli">Constant outside, trend within equilibrium</option></Select></label>
      <label className="block"><span className={fieldLabel}>Rank-test confidence level</span><Select className={field('text', 'mt-1')} value={significance} onChange={(e) => { const value = Number(e.target.value); if (value === 90 || value === 95 || value === 99) setSignificance(value) }}><option value={90}>90%</option><option value={95}>95%</option><option value={99}>99%</option></Select></label>
    </>}
    <label className="block"><span className={fieldLabel}>Maximum lag</span><input type="number" min={1} max={24} className={field('text', 'mt-1')} value={maxLag} onChange={(e) => setMaxLag(Math.max(1, Math.min(24, Math.floor(Number(e.target.value) || 1))))} /></label>
    {model==='vecm'&&<label className="block"><span className={fieldLabel}>Forecast periods</span><input type="number" min={1} max={200} className={field('text','mt-1')} placeholder="No forecast" value={forecastSteps} onChange={e=>setForecastSteps(e.target.value)} /><span className={fieldHint}>Leave blank to fit without forecasting.</span></label>}
    {selected.length >= 2 && props.prepared.observations < minimumRows && <p role="status" className="text-body text-muted">This specification needs at least {minimumRows} prepared observations.</p>}
  </fieldset>
  <div className="mt-4 flex items-center gap-3"><button type="button" className={button('signal')} disabled={!ready} aria-busy={job.kind === 'running'} onClick={job.kind === 'running' ? undefined : () => void run()}>Fit {model.toUpperCase()}</button><span className="inline-flex h-5 w-5 items-center">{job.kind === 'running' && <Orb state="solving" aria-label={`${model.toUpperCase()} running`} />}</span></div></>

  return <WorkbenchLayout id={`time-series-${model}`}
    bottom={{ title: `Time-series runs · ${runs.length}`, defaultSize: 150, body: <TimeSeriesHistory entries={runs} onDelete={(entry) => props.onDeleteRun(entry.id)} /> }}
    inspector={{ title: 'Data and method requirements', body: <TimeSeriesRequirements method={TIME_SERIES_METHODS[model]} prepared={props.prepared} source={props.source.name} /> }}
    stage={<section className="@container/panel flex flex-col gap-5">
      <TimeSeriesHeading />
      <section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
        {selector}
        <h3 className="mb-0 mt-3 text-body font-medium text-ink">{model === 'ardl' ? 'ARDL long-run relationship' : 'Vector error-correction model'}</h3>
        <p className={`${fieldHint} mb-4 mt-1 max-w-[65ch]`}>{model === 'ardl' ? 'Estimate how an outcome relates to its own earlier values and to current and earlier values of another series.' : 'Estimate long-run equilibrium relationships and how changes in the series respond to departures from them.'}</p>
        {controls}
        <span role="status" className="sr-only">{job.kind === 'running' ? `Fitting ${model.toUpperCase()}…` : ''}</span>
        {job.kind === 'failed' && <p role="alert" className="mt-3 text-body text-danger">{job.detail}</p>}
      </section>
      {runs.length === 0 && <p className="text-body text-muted">Choose series and a model specification to begin.</p>}
      {runs.length > 0 && <h3 className={`${sectionTitle} m-0`}>Results</h3>}
      {runs.slice(-1).map((saved) => <TimeSeriesRunResult key={saved.id} run={saved} />)}
    </section>} />
}

export function TimeSeriesPanel(props: Props) {
  const [analysis, setAnalysis] = useState<'count' | Model>('count')
  const [busy, setBusy] = useState(false)
  const report = props.onActivity
  const onActivity = useCallback((activity: RunActivity | null) => { setBusy(activity !== null); report?.(activity) }, [report])
  const selector = <SegmentedControl variant="line" size="sm" ariaLabel="Time-series analysis type" value={analysis} onChange={setAnalysis} disabled={busy} options={[{ value: 'count', label: 'Count models' }, { value: 'ardl', label: 'ARDL' }, { value: 'vecm', label: 'VECM' }]} />
  if (analysis === 'ardl') return <ArdlModelPanel {...props} selector={selector} onActivity={onActivity} />
  if (analysis === 'vecm') return <LongRunModel key={analysis} {...props} model={analysis} selector={selector} onActivity={onActivity} />
  return <WorkbenchLayout id="time-series-count" bottom={{ title: `Count-model runs · ${props.counts.length}`, defaultSize: 150, body: <TimeSeriesHistory entries={props.counts} onDelete={(entry) => props.onDeleteCount(entry.id)} /> }} inspector={{ title: 'Data and method requirements', body: <TimeSeriesRequirements method={COUNT_SERIES_DIAGNOSTIC_METHODS[0]} prepared={props.prepared} source={props.source.name} /> }} stage={<section className="@container/panel flex flex-col gap-5">
    <TimeSeriesHeading />
    <CountSeriesCard selector={selector} source={props.source} profile={props.profile} prepared={props.prepared} artifacts={props.counts} onArtifact={props.onCount} onActivity={onActivity} />
  </section>} />
}
