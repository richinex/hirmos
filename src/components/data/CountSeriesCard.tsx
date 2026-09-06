import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { countSeriesFitOption, interventionScoreOption } from '@/charts/data/countSeriesModel'
import { useChartTheme } from '@/charts/theme'
import { MethodCaveats } from '@/components/MethodCaveats'
import { LagListField } from '@/components/ui/LagListField'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { button, field, fieldLabel, label, num, prose, well } from '@/components/ui/recipes'
import {
  describeCountSeriesReadiness,
  newCountSeriesModelId,
  readyCountSeriesSpecification,
  type CountSeriesLink,
  type CountSeriesModelArtifact,
} from '@/domain/countSeries'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import type { NonEmptyArray } from '@/domain/dop'
import { COUNT_SERIES_DIAGNOSTIC_METHODS } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly phase: string }
  | { readonly kind: 'failed'; readonly detail: string }

function ResultCharts({ artifact }: { readonly artifact: CountSeriesModelArtifact }) {
  const theme = useChartTheme()
  const observed = useMemo(
    () => artifact.result.fittedMeans.map((mean, index) => mean + (artifact.result.residuals[index] ?? 0)),
    [artifact.result.fittedMeans, artifact.result.residuals],
  )
  const fit = useMemo(
    () => countSeriesFitOption({ name: artifact.outcome.name, observed, fitted: artifact.result.fittedMeans, strongestReferencePoint: artifact.result.strongestReferencePoint }, theme),
    [artifact.outcome.name, artifact.result.fittedMeans, artifact.result.strongestReferencePoint, observed, theme],
  )
  const scan = useMemo(() => interventionScoreOption({ candidates: artifact.result.candidates }, theme), [artifact.result.candidates, theme])
  return (
    <div className="mt-3 grid gap-3 @3xl/panel:grid-cols-2">
      <div className="rounded-lg border border-hair bg-panel p-3"><span className={label('text-faint')}>Observed counts and fitted conditional mean</span><EChart option={fit} label={`${artifact.outcome.name}, observed and fitted counts`} className="mt-1 h-56" /></div>
      <div className="rounded-lg border border-hair bg-panel p-3"><span className={label('text-faint')}>Unknown-date score profile</span><EChart option={scan} label={`${artifact.outcome.name}, intervention score by candidate date`} className="mt-1 h-56" /></div>
    </div>
  )
}

function CountSeriesRecord({ artifact, open }: { readonly artifact: CountSeriesModelArtifact; readonly open: boolean }) {
  const strongest = artifact.result.candidates.find((candidate) => candidate.referencePoint === artifact.result.strongestReferencePoint)
  return (
    <li>
      <details className={well()} open={open}>
        <summary className="cursor-pointer px-3 py-2 text-body text-ink">
          <span className="font-medium">{artifact.outcome.name}</span>
          <span className={num('ml-3 text-label text-faint')}>{artifact.result.link === 'identity' ? 'additive' : 'multiplicative'} · count lags {artifact.result.pastObservationLags.join(', ')} · mean lags {artifact.result.pastMeanLags.join(', ')}</span>
          <span className={num('float-right text-micro text-faint')}>{formatTime(artifact.createdAt)}</span>
        </summary>
        <div className="border-t border-hair px-3 py-3">
          <dl className="m-0 grid gap-3 @md/panel:grid-cols-2 @3xl/panel:grid-cols-4">
            <div><dt className={label('text-faint')}>Strongest candidate</dt><dd className={num('m-0 mt-1 text-title text-ink')}>row {artifact.result.strongestReferencePoint + 1}</dd></div>
            <div><dt className={label('text-faint')}>Score statistic</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{formatStatistic('raw', strongest?.scoreStatistic ?? Number.NaN).text}</dd></div>
            <div><dt className={label('text-faint')}>Negative-binomial size</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{formatStatistic('raw', artifact.result.size).text}</dd></div>
            <div><dt className={label('text-faint')}>Log likelihood</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{formatStatistic('raw', artifact.result.logLikelihood).text}</dd></div>
          </dl>
          <p className="mb-0 mt-3 text-body text-muted">The maximum locates the date most compatible with the selected intervention shape under this fitted count model. The non-bootstrap scan does not report a p-value, and the date is not evidence of a causal intervention.</p>
          <ResultCharts artifact={artifact} />
          <p className={num('mb-0 mt-3 text-label text-faint')}>Parameters: {artifact.result.parameters.map((value) => formatStatistic('raw', value).text).join(' · ')} · dispersion {formatStatistic('raw', artifact.result.dispersion).text} · {formatCount(artifact.result.observations).text} observations</p>
        </div>
      </details>
    </li>
  )
}

export function CountSeriesCard({ source, profile, prepared, artifacts, onArtifact }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
  readonly artifacts: readonly CountSeriesModelArtifact[]
  readonly onArtifact: (artifact: CountSeriesModelArtifact) => void
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
      if (!result.ok) { setJob({ kind: 'failed', detail: describeAnalysisWorkerProblem(result.error) }); return }
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
    <section aria-labelledby="count-series-title">
      <h4 id="count-series-title" className="m-0 text-body font-medium text-ink">Count-series model and intervention scan</h4>
      <p className={prose('mb-0 mt-1 text-faint')}>Fit a negative-binomial INGARCH model, then scan a specified point, decaying, or persistent intervention shape over a candidate date range. This is model assessment and event detection, not causal effect estimation.</p>
      <div className="mt-3 grid items-start gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
        <label className="block"><span className={fieldLabel}>Count series</span><Select className={field('text', 'mt-1')} value={outcome ?? ''} onChange={(event) => setOutcome(event.target.value === '' ? null : event.target.value as ColumnId)}><option value="">Choose series</option>{columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}</Select></label>
        <div><span className={fieldLabel}>Mean link</span><SegmentedControl className="mt-1" ariaLabel="Count-series mean link" value={link} onChange={setLink} options={[{ value: 'identity', label: 'Additive' }, { value: 'log', label: 'Multiplicative' }]} /></div>
        <LagListField label="Past count lags" lags={pastObservationLags} onChange={setPastObservationLags} />
        <LagListField label="Past mean lags" lags={pastMeanLags} onChange={setPastMeanLags} />
        <label className="block"><span className={fieldLabel}>Candidate start row</span><input type="number" min={2} max={prepared.observations} className={field('text', 'mt-1')} value={candidateStart + 1} onChange={(event) => setCandidateStart(Math.max(1, Math.floor(Number(event.target.value) || 2) - 1))} /></label>
        <label className="block"><span className={fieldLabel}>Candidate end row</span><input type="number" min={2} max={prepared.observations} className={field('text', 'mt-1')} value={candidateEnd + 1} onChange={(event) => setCandidateEnd(Math.max(1, Math.floor(Number(event.target.value) || 2) - 1))} /></label>
        <div><span className={fieldLabel}>Intervention shape</span><SegmentedControl className="mt-1" ariaLabel="Count-series intervention shape" value={delta === 0 ? 'point' : delta === 1 ? 'persistent' : 'decaying'} onChange={(kind) => setDelta(kind === 'point' ? 0 : kind === 'persistent' ? 1 : 0.8)} options={[{ value: 'point', label: 'Point' }, { value: 'decaying', label: 'Decaying' }, { value: 'persistent', label: 'Persistent' }]} /></div>
        {delta > 0 && delta < 1 && <label className="block"><span className={fieldLabel}>Decay δ</span><input type="number" min={0.01} max={0.99} step={0.01} className={field('text', 'mt-1')} value={delta} onChange={(event) => setDelta(Math.max(0.01, Math.min(0.99, Number(event.target.value) || 0.8)))} /></label>}
      </div>
      <div className="mt-3 flex flex-wrap items-center gap-3">
        <button type="button" className={button('quiet')} aria-busy={job.kind === 'running'} disabled={outcome === null || job.kind === 'running'} onClick={() => void run()}>Fit and scan</button>
        {job.kind === 'running' && <span role="status" className="text-body text-faint">{job.phase}</span>}
        {job.kind === 'failed' && <span role="alert" className="text-body text-danger">{job.detail}</span>}
      </div>
      <p className="mb-0 mt-2 text-body text-faint">The scan follows tscount’s non-bootstrap score procedure. It reports the maximum statistic and no p-value; use it to inspect a fitted model, not to assert that an intervention occurred.</p>
      <MethodCaveats methods={COUNT_SERIES_DIAGNOSTIC_METHODS} />
      {artifacts.length > 0 && <ul className="m-0 mt-4 list-none space-y-2 p-0">{[...artifacts].reverse().map((artifact, index) => <CountSeriesRecord key={artifact.id} artifact={artifact} open={index === 0} />)}</ul>}
    </section>
  )
}
