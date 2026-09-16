import { Metadata } from '@/components/ui/Metadata'
import { useMemo, useState, type ReactNode } from 'react'
import { useRunActivity } from '@/lib/useRunActivity'
import type { RunActivity } from '@/domain/activity'
import { ExpandableChart } from '@/charts/ExpandableChart'
import type { VisibleWindow } from '@/charts/window'
import { TimeSeriesEquation } from './TimeSeriesEquation'
import { Orb } from '@/components/ui/Orb'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { countSeriesFitOption, interventionScoreOption } from '@/charts/data/countSeriesModel'
import { useChartTheme } from '@/charts/theme'
import { LagListField } from '@/components/ui/LagListField'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { button, field, fieldLabel, fieldHint, label, num, panel, sectionTitle, well } from '@/components/ui/recipes'
import {
  describeCountSeriesReadiness,
  newCountSeriesModelId,
  readyCountSeriesSpecification,
  type CountSeriesLink,
  type CountSeriesModelArtifact,
} from '@/domain/countSeries'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import type { NonEmptyArray } from '@/domain/dop'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic, formatWords } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly phase: string }
  | { readonly kind: 'failed'; readonly detail: string }

function ResultCharts({ artifact }: { readonly artifact: CountSeriesModelArtifact }) {
  const theme = useChartTheme()
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const observed = useMemo(
    () => artifact.result.fittedMeans.map((mean, index) => mean + (artifact.result.residuals[index] ?? 0)),
    [artifact.result.fittedMeans, artifact.result.residuals],
  )
  const fit = useMemo(
    () => countSeriesFitOption({ name: artifact.outcome.name, observed, fitted: artifact.result.fittedMeans, strongestReferencePoint: artifact.result.strongestReferencePoint }, theme),
    [artifact.outcome.name, artifact.result.fittedMeans, artifact.result.strongestReferencePoint, observed, theme],
  )
  const scan = useMemo(() => interventionScoreOption({ candidates: artifact.result.candidates, observations: observed.length }, theme), [artifact.result.candidates, observed.length, theme])
  return (
    <div className="mt-3 grid min-w-0 gap-3">
      <div data-testid="count-fit-plot" className="min-w-0 rounded-lg border border-hair bg-panel p-3"><span className={label('text-faint')}>Observed counts and fitted conditional mean</span><ExpandableChart option={fit} window={window} onWindow={setWindow} label={`${artifact.outcome.name}, observed and fitted counts`} className="mt-1 h-72" /></div>
      <div data-testid="count-score-plot" className="min-w-0 rounded-lg border border-hair bg-panel p-3"><span className={label('text-faint')}>Unknown-date score profile</span><ExpandableChart option={scan} window={window} onWindow={setWindow} label={`${artifact.outcome.name}, intervention score by candidate date`} className="mt-1 h-64" /></div>
    </div>
  )
}

export function CountSeriesRecord({ artifact, open }: { readonly artifact: CountSeriesModelArtifact; readonly open: boolean }) {
  const strongest = artifact.result.candidates.find((candidate) => candidate.referencePoint === artifact.result.strongestReferencePoint)
  return (
    <li>
      <details className={well()} open={open}>
        <summary className="cursor-pointer px-3 py-2 text-body text-ink">
          <span className="font-medium">{artifact.outcome.name}</span>
          <span className={num('ml-3 text-label text-faint')}><Metadata><span>{artifact.result.link === 'identity' ? 'additive' : 'multiplicative'}</span><span>count lags {artifact.result.pastObservationLags.join(', ')}</span><span>mean lags {artifact.result.pastMeanLags.join(', ')}</span></Metadata></span>
          <span className={num('float-right text-micro text-faint')}>{formatTime(artifact.createdAt)}</span>
        </summary>
        <div className="border-t border-hair px-3 py-3">
          <MetricGrid label="Count-model summary">
            <MetricTile label="Strongest candidate" value={formatWords(`row ${artifact.result.strongestReferencePoint + 1}`)} />
            <MetricTile label="Score statistic" value={formatStatistic('raw', strongest?.scoreStatistic ?? Number.NaN)} />
            <MetricTile label="Negative-binomial size" value={formatStatistic('raw', artifact.result.size)} />
            <MetricTile label="Log likelihood" value={formatStatistic('raw', artifact.result.logLikelihood)} />
          </MetricGrid>
          <p className="mb-0 mt-3 text-body text-muted">The maximum locates the date most compatible with the selected intervention shape under this fitted count model. The non-bootstrap scan does not report a p-value, and the date is not evidence of a causal intervention.</p>
          <ResultCharts artifact={artifact} />
          <TimeSeriesEquation run={artifact} />
          <p className={num('mb-0 mt-3 text-label text-faint')}><Metadata><span>Parameters: {artifact.result.parameters.map((value) => formatStatistic('raw', value).text).join('; ')}</span><span>dispersion {formatStatistic('raw', artifact.result.dispersion).text}</span><span>{formatCount(artifact.result.observations).text} observations</span></Metadata></p>
        </div>
      </details>
    </li>
  )
}

export function CountSeriesCard({ source, profile, prepared, artifacts, onArtifact, onActivity, selector }: {
  readonly selector: ReactNode
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
  readonly artifacts: readonly CountSeriesModelArtifact[]
  readonly onArtifact: (artifact: CountSeriesModelArtifact) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}) {
  const columns = profile.columns.filter((column) => prepared.columns.includes(column.id) && isNumericDuckDbType(column.duckdbType))
  const [outcome, setOutcome] = useState<ColumnId | null>(null)
  const [link, setLink] = useState<CountSeriesLink>('identity')
  const [pastObservationLags, setPastObservationLags] = useState<NonEmptyArray<number>>([1])
  const [pastMeanLags, setPastMeanLags] = useState<NonEmptyArray<number>>([1])
  const [candidateStart, setCandidateStart] = useState(Math.max(1, Math.floor(prepared.observations * 0.2)))
  const [candidateEnd, setCandidateEnd] = useState(Math.max(1, Math.floor(prepared.observations * 0.8)))
  const [delta, setDelta] = useState(1)
  const [job, setJob] = useState<Job>({ kind: 'idle' })
  useRunActivity(onActivity, job.kind === 'running' ? { label: 'Count-series scan', progress: null } : null)

  const run = async () => {
    if (outcome === null) { setJob({ kind: 'failed', detail: 'Choose the count series to model.' }); return }
    setJob({ kind: 'running', phase: 'Preparing count series' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(source, profile, prepared, [outcome])
      if (!matrix.ok) { setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(matrix.error) }); return }
      const values = Array.from(matrix.value.values)
      const specification = readyCountSeriesSpecification({ outcome, link, pastObservationLags, pastMeanLags, candidateStart, candidateEnd, delta }, matrix.value.rowCount, values)
      if (!specification.ok) { setJob({ kind: 'failed', detail: describeCountSeriesReadiness(specification.error) }); return }
      const candidateReferencePoints = Array.from({ length: specification.value.candidateEnd - specification.value.candidateStart + 1 }, (_, index) => specification.value.candidateStart + index)
      const result = await analysis.runCountSeriesInterventionScan(matrix.value.values, matrix.value.rowCount, matrix.value.columns.length, {
        outcome: 0,
        link,
        pastObservationLags,
        pastMeanLags,
        candidateReferencePoints,
        delta,
      }, (progress) => setJob({ kind: 'running', phase: progress.stage }))
      if (!result.ok) {
        const detail = describeAnalysisWorkerProblem(result.error)
        setJob({ kind: 'failed', detail: detail.includes('DispersionNotEstimable') ? 'The negative-binomial dispersion could not be estimated for this specification. Review the count variation and selected lags.' : detail })
        return
      }
      const selected = matrix.value.columns[0]
      if (selected === undefined) { setJob({ kind: 'failed', detail: 'The prepared matrix omitted the selected count series.' }); return }
      onArtifact({
        kind: 'count-series-model',
        id: newCountSeriesModelId(),
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        outcome: selected,
        specification: { link, pastObservationLags, pastMeanLags, candidateStart: specification.value.candidateStart, candidateEnd: specification.value.candidateEnd, delta },
        result: result.value,
      })
      setJob({ kind: 'idle' })
    } catch (cause: unknown) {
      setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  return (
    <section className="flex flex-col gap-5" aria-labelledby="count-series-title">
      <section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
      {selector}
      <h3 id="count-series-title" className="mb-0 mt-3 text-body font-medium text-ink">Negative-binomial count model</h3>
      <p className={`${fieldHint} mb-0 mt-1 max-w-[65ch]`}>Fit a count model and search the selected range for a temporary, fading or persistent change.</p>
      <fieldset disabled={job.kind === 'running'} className="m-0 mt-4 grid min-w-0 items-start gap-4 border-0 p-0 @lg/panel:grid-cols-2"><legend className="sr-only">Count model specification</legend>
        <label className="block"><span className={fieldLabel}>Count series</span><Select className={field('text', 'mt-1')} value={outcome ?? ''} onChange={(event) => setOutcome(event.target.value === '' ? null : event.target.value as ColumnId)}><option value="">Choose series</option>{columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}</Select></label>
        <div><span className={fieldLabel}>Mean link</span><SegmentedControl className="mt-1" ariaLabel="Count-series mean link" value={link} onChange={setLink} options={[{ value: 'identity', label: 'Additive' }, { value: 'log', label: 'Multiplicative' }]} /></div>
        <LagListField label="Past count lags" lags={pastObservationLags} onChange={setPastObservationLags} />
        <LagListField label="Past mean lags" lags={pastMeanLags} onChange={setPastMeanLags} />
        <label className="block"><span className={fieldLabel}>Candidate start row</span><input type="number" min={2} max={prepared.observations} className={field('text', 'mt-1')} value={candidateStart + 1} onChange={(event) => setCandidateStart(Math.max(1, Math.floor(Number(event.target.value) || 2) - 1))} /></label>
        <label className="block"><span className={fieldLabel}>Candidate end row</span><input type="number" min={2} max={prepared.observations} className={field('text', 'mt-1')} value={candidateEnd + 1} onChange={(event) => setCandidateEnd(Math.max(1, Math.floor(Number(event.target.value) || 2) - 1))} /></label>
        <div><span className={fieldLabel}>Intervention shape</span><SegmentedControl className="mt-1" ariaLabel="Count-series intervention shape" value={delta === 0 ? 'point' : delta === 1 ? 'persistent' : 'decaying'} onChange={(kind) => setDelta(kind === 'point' ? 0 : kind === 'persistent' ? 1 : 0.8)} options={[{ value: 'point', label: 'Point' }, { value: 'decaying', label: 'Decaying' }, { value: 'persistent', label: 'Persistent' }]} /></div>
        {delta > 0 && delta < 1 && <label className="block"><span className={fieldLabel}>Decay δ</span><input type="number" min={0.01} max={0.99} step={0.01} className={field('text', 'mt-1')} value={delta} onChange={(event) => setDelta(Math.max(0.01, Math.min(0.99, Number(event.target.value) || 0.8)))} /></label>}
      </fieldset>
      <div className="mt-3 flex flex-wrap items-center gap-3">
        <button type="button" className={button('signal')} aria-busy={job.kind === 'running'} disabled={outcome === null} onClick={job.kind === 'running' ? undefined : () => void run()}>Fit and scan</button>
        <span className="inline-flex h-5 w-5 items-center">{job.kind === 'running' && <Orb state="solving" aria-label="Count model running" />}</span>
        <span role="status" className="sr-only">{job.kind === 'running' ? job.phase : ''}</span>
        {job.kind === 'failed' && <span role="alert" className="text-body text-danger">{job.detail}</span>}
      </div>
      </section>
      {artifacts.length > 0 && <h3 className={`${sectionTitle} m-0`}>Results</h3>}
      {artifacts.slice(-1).map((artifact) => <ul key={artifact.id} className="m-0 list-none p-0"><CountSeriesRecord artifact={artifact} open /></ul>)}
    </section>
  )
}
