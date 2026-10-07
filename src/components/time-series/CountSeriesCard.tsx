import { NumberInput } from '@/components/ui/NumberInput'
import { RunActions } from '@/components/ui/RunActions'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useTimeSeriesDraft } from './useTimeSeriesDraft'
import type { CountDraft } from '@/domain/timeSeriesDraft'
import { Metadata } from '@/components/ui/Metadata'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
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
import {
  actionGap,
  button,
  field,
  fieldLabel,
  fieldRow,
  label,
  num,
  panel,
  sectionTitle,
  stepsStack,
  well,
} from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { cn } from '@/lib/utils'
import {
  describeCountSeriesReadiness,
  newCountSeriesModelId,
  readyCountSeriesSpecification,
  type CountSeriesModelArtifact,
} from '@/domain/countSeries'
import { isNumericDuckDbType, type ColumnId, type DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic, formatWords } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

function ResultCharts({ artifact }: { readonly artifact: CountSeriesModelArtifact }) {
  const theme = useChartTheme()
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const observed = useMemo(
    () =>
      artifact.result.fittedMeans.map(
        (mean, index) => mean + (artifact.result.residuals[index] ?? 0),
      ),
    [artifact.result.fittedMeans, artifact.result.residuals],
  )
  const fit = useMemo(
    () =>
      countSeriesFitOption(
        {
          name: artifact.outcome.name,
          observed,
          fitted: artifact.result.fittedMeans,
          strongestReferencePoint: artifact.result.strongestReferencePoint,
        },
        theme,
      ),
    [
      artifact.outcome.name,
      artifact.result.fittedMeans,
      artifact.result.strongestReferencePoint,
      observed,
      theme,
    ],
  )
  const scan = useMemo(
    () =>
      interventionScoreOption(
        { candidates: artifact.result.candidates, observations: observed.length },
        theme,
      ),
    [artifact.result.candidates, observed.length, theme],
  )
  return (
    <div className="mt-3 grid min-w-0 gap-3">
      <div
        data-testid="count-fit-plot"
        className="min-w-0 rounded-lg border border-hair bg-panel p-3"
      >
        <span className={label('text-faint')}>Observed counts and fitted conditional mean</span>
        <ExpandableChart
          option={fit}
          window={window}
          onWindow={setWindow}
          label={`${artifact.outcome.name}, observed and fitted counts`}
          className="mt-1 h-72"
        />
      </div>
      <div
        data-testid="count-score-plot"
        className="min-w-0 rounded-lg border border-hair bg-panel p-3"
      >
        <span className={label('text-faint')}>Unknown-date score profile</span>
        <ExpandableChart
          option={scan}
          window={window}
          onWindow={setWindow}
          label={`${artifact.outcome.name}, intervention score by candidate date`}
          className="mt-1 h-64"
        />
      </div>
    </div>
  )
}

export function CountSeriesRecord({
  artifact,
  open,
}: {
  readonly artifact: CountSeriesModelArtifact
  readonly open: boolean
}) {
  const strongest = artifact.result.candidates.find(
    (candidate) => candidate.referencePoint === artifact.result.strongestReferencePoint,
  )
  return (
    <li>
      <details className={well()} open={open}>
        <DisclosureSummary className="cursor-pointer px-3 py-2 text-body text-ink">
          <span className="font-medium">{artifact.outcome.name}</span>
          <span className={num('ml-3 text-label text-faint')}>
            <Metadata>
              <span>{artifact.result.link === 'identity' ? 'additive' : 'multiplicative'}</span>
              <span>count lags {artifact.result.pastObservationLags.join(', ')}</span>
              <span>mean lags {artifact.result.pastMeanLags.join(', ')}</span>
            </Metadata>
          </span>
          <span className={num('float-right text-micro text-faint')}>
            {formatTime(artifact.createdAt)}
          </span>
        </DisclosureSummary>
        <div className="border-t border-hair px-3 py-3">
          <MetricGrid label="Count-model summary">
            <MetricTile
              label="Strongest candidate"
              value={formatWords(`row ${artifact.result.strongestReferencePoint + 1}`)}
            />
            <MetricTile
              label="Score statistic"
              value={formatStatistic('raw', strongest?.scoreStatistic ?? Number.NaN)}
            />
            <MetricTile
              label="Negative-binomial size"
              value={formatStatistic('raw', artifact.result.size)}
            />
            <MetricTile
              label="Log likelihood"
              value={formatStatistic('raw', artifact.result.logLikelihood)}
            />
          </MetricGrid>
          <p className="mb-0 mt-3 text-body text-muted">
            The maximum locates the date most compatible with the selected intervention shape under
            this fitted count model. The non-bootstrap scan does not report a p-value, and the date
            is not evidence of a causal intervention.
          </p>
          <ResultCharts artifact={artifact} />
          <TimeSeriesEquation run={artifact} />
          <p className={num('mb-0 mt-3 text-label text-faint')}>
            <Metadata>
              <span>
                Parameters:{' '}
                {artifact.result.parameters
                  .map((value) => formatStatistic('raw', value).text)
                  .join('; ')}
              </span>
              <span>dispersion {formatStatistic('raw', artifact.result.dispersion).text}</span>
              <span>{formatCount(artifact.result.observations).text} observations</span>
            </Metadata>
          </p>
        </div>
      </details>
    </li>
  )
}

export function CountSeriesCard({
  source,
  profile,
  prepared,
  artifacts,
  onArtifact,
  onActivity,
  selector,
}: {
  readonly selector: ReactNode
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
  readonly artifacts: readonly CountSeriesModelArtifact[]
  readonly onArtifact: (artifact: CountSeriesModelArtifact) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}) {
  const columns = profile.columns.filter(
    (column) => prepared.columns.includes(column.id) && isNumericDuckDbType(column.duckdbType),
  )
  const { outcome, link, pastObservationLags, pastMeanLags, candidateStart, candidateEnd, delta } =
    useTimeSeriesDraft(prepared.id, (state) => state.count)
  const change = useWorkflow((state) => state.changeTimeSeries)
  const setOutcome = (value: CountDraft['outcome']) =>
    change(prepared.id, { type: 'count', field: 'outcome', value })
  const setLink = (value: CountDraft['link']) =>
    change(prepared.id, { type: 'count', field: 'link', value })
  const setPastObservationLags = (value: CountDraft['pastObservationLags']) =>
    change(prepared.id, { type: 'count', field: 'pastObservationLags', value })
  const setPastMeanLags = (value: CountDraft['pastMeanLags']) =>
    change(prepared.id, { type: 'count', field: 'pastMeanLags', value })
  const setCandidateStart = (value: CountDraft['candidateStart']) =>
    change(prepared.id, { type: 'count', field: 'candidateStart', value })
  const setCandidateEnd = (value: CountDraft['candidateEnd']) =>
    change(prepared.id, { type: 'count', field: 'candidateEnd', value })
  const setDelta = (value: CountDraft['delta']) =>
    change(prepared.id, { type: 'count', field: 'delta', value })
  const session = useJob('time-series:counts')
  const { job } = session
  useRunActivity(
    onActivity,
    job.kind === 'running' ? { label: 'Count-series scan', progress: null } : null,
  )

  const run = async () => {
    const current = session.start('analysis', 'Preparing count series')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    if (outcome === null) {
      fail('Choose the count series to model.')
      return
    }
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] =
        await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const matrix = await materialisePrepared(source, profile, prepared, [outcome])
      if (!session.current(current)) return
      if (!matrix.ok) {
        fail(describePreparedMaterialisationProblem(matrix.error))
        return
      }
      const values = Array.from(matrix.value.values)
      const specification = readyCountSeriesSpecification(
        { outcome, link, pastObservationLags, pastMeanLags, candidateStart, candidateEnd, delta },
        matrix.value.rowCount,
        values,
      )
      if (!specification.ok) {
        fail(describeCountSeriesReadiness(specification.error))
        return
      }
      const candidateReferencePoints = Array.from(
        { length: specification.value.candidateEnd - specification.value.candidateStart + 1 },
        (_, index) => specification.value.candidateStart + index,
      )
      const result = await analysis.runCountSeriesInterventionScan(
        matrix.value.values,
        matrix.value.rowCount,
        matrix.value.columns.length,
        {
          outcome: 0,
          link,
          pastObservationLags,
          pastMeanLags,
          candidateReferencePoints,
          delta,
        },
        (progress) => session.progress(current, progress.stage),
      )
      if (!session.current(current)) return
      if (!result.ok) {
        const detail = describeAnalysisWorkerProblem(result.error)
        fail(
          detail.includes('DispersionNotEstimable')
            ? 'The negative-binomial dispersion could not be estimated for this specification. Review the count variation and selected lags.'
            : detail,
        )
        return
      }
      const selected = matrix.value.columns[0]
      if (selected === undefined) {
        fail('The prepared matrix omitted the selected count series.')
        return
      }
      onArtifact({
        kind: 'count-series-model',
        id: newCountSeriesModelId(),
        preparedDataset: prepared.id,
        createdAt: new Date().toISOString(),
        outcome: selected,
        specification: {
          link,
          pastObservationLags,
          pastMeanLags,
          candidateStart: specification.value.candidateStart,
          candidateEnd: specification.value.candidateEnd,
          delta,
        },
        result: result.value,
      })
      session.finish(current)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  return (
    <section className="flex flex-col gap-5" aria-label="Negative-binomial count model">
      <section className={panel('p-(--panel-space)')} aria-label="Time-series setup">
        {selector}
        <fieldset disabled={job.kind === 'running'} className="m-0 mt-6 min-w-0 border-0 p-0">
          <legend className="sr-only">Count model specification</legend>
          <div className={stepsStack}>
            <SettingsStep number={1} title="Fit the count model">
              <label className="block max-w-md">
                <span className={fieldLabel}>Count series</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={outcome ?? ''}
                  onChange={(event) =>
                    setOutcome(event.target.value === '' ? null : (event.target.value as ColumnId))
                  }
                >
                  <option value="">Choose series</option>
                  {columns.map((column) => (
                    <option key={column.id} value={column.id}>
                      {column.name}
                    </option>
                  ))}
                </Select>
              </label>
              <div>
                <span className={fieldLabel}>Mean link</span>
                <SegmentedControl
                  className="mt-1"
                  ariaLabel="Count-series mean link"
                  value={link}
                  onChange={setLink}
                  options={[
                    { value: 'identity', label: 'Additive' },
                    { value: 'log', label: 'Multiplicative' },
                  ]}
                />
              </div>
              <div className={fieldRow.two}>
                <LagListField
                  label="Past count lags"
                  lags={pastObservationLags}
                  onChange={setPastObservationLags}
                />
                <LagListField
                  label="Past mean lags"
                  lags={pastMeanLags}
                  onChange={setPastMeanLags}
                />
              </div>
            </SettingsStep>
            <SettingsStep number={2} title="Search for a change">
              <div className={fieldRow.two}>
                <label className="block">
                  <span className={fieldLabel}>Candidate start row</span>
                  <NumberInput
                    min={2}
                    max={prepared.observations}
                    className={field('text', 'mt-1')}
                    value={candidateStart + 1}
                    onChange={(event) =>
                      setCandidateStart(
                        Math.max(1, Math.floor(Number(event.target.value) || 2) - 1),
                      )
                    }
                  />
                </label>
                <label className="block">
                  <span className={fieldLabel}>Candidate end row</span>
                  <NumberInput
                    min={2}
                    max={prepared.observations}
                    className={field('text', 'mt-1')}
                    value={candidateEnd + 1}
                    onChange={(event) =>
                      setCandidateEnd(Math.max(1, Math.floor(Number(event.target.value) || 2) - 1))
                    }
                  />
                </label>
              </div>
              <div>
                <span className={fieldLabel}>Intervention shape</span>
                <SegmentedControl
                  className="mt-1"
                  ariaLabel="Count-series intervention shape"
                  value={delta === 0 ? 'point' : delta === 1 ? 'persistent' : 'decaying'}
                  onChange={(kind) =>
                    setDelta(kind === 'point' ? 0 : kind === 'persistent' ? 1 : 0.8)
                  }
                  options={[
                    { value: 'point', label: 'Point' },
                    { value: 'decaying', label: 'Decaying' },
                    { value: 'persistent', label: 'Persistent' },
                  ]}
                />
              </div>
              {delta > 0 && delta < 1 && (
                <label className="block max-w-xs">
                  <span className={fieldLabel}>Decay δ</span>
                  <NumberInput
                    min={0.01}
                    max={0.99}
                    step={0.01}
                    className={field('text', 'mt-1')}
                    value={delta}
                    onChange={(event) =>
                      setDelta(Math.max(0.01, Math.min(0.99, Number(event.target.value) || 0.8)))
                    }
                  />
                </label>
              )}
            </SettingsStep>
          </div>
        </fieldset>
        <RunActions
          className={actionGap}
          running={job.kind === 'running'}
          onCancel={session.cancel}
          orbLabel="Count model running"
        >
          <button
            type="button"
            className={button('signal')}
            aria-busy={job.kind === 'running'}
            disabled={outcome === null || session.blocked}
            onClick={job.kind === 'running' ? undefined : () => void run()}
          >
            Fit and scan
          </button>
          <span role="status" className="sr-only">
            {job.kind === 'running' ? job.stage : ''}
          </span>
        </RunActions>
        <JobNotice job={job} />
      </section>
      {artifacts.length > 0 && <h3 className={`${sectionTitle} m-0`}>Results</h3>}
      {artifacts.slice(-1).map((artifact) => (
        <ul key={artifact.id} className="m-0 list-none p-0">
          <CountSeriesRecord artifact={artifact} open />
        </ul>
      ))}
    </section>
  )
}
