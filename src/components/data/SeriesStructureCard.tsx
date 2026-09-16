import { Metadata } from '@/components/ui/Metadata'
import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import type { SeriesFacts } from '@/domain/diagnostics'
import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { lagCorrelationOption } from '@/charts/data/lagCorrelation'
import { changePointsOption } from '@/charts/sensitivity/changePoints'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { MethodCaveats } from '@/components/MethodCaveats'
import { MetricTile } from '@/components/ui/figures'
import { button, field, figureGrid, label, num, panel, sectionTitle, well } from '@/components/ui/recipes'
import type { DatasetProfile, TimeAxis } from '@/domain/dataset'
import { SERIES_STRUCTURE_METHODS } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { seasonalPeriodOf } from '@/domain/seasonal'
import { defaultPeltPenalty } from '@/domain/sensitivity'
import type { SelectedSource } from '@/domain/workflow'
import { formatDay } from '@/lib/format/date'
import { formatAbsent, formatCount, formatStatistic } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { cn } from '@/lib/utils'

/** List at most this many change points inline; the rest are counted. */
const NAMED_CHANGE_POINTS = 6

/**
 * Name each change point by its date when the series has a calendar axis, and by its row otherwise.
 *
 * A long list is summarised: the positions are already drawn on the plot beside it, and a wall of
 * figures hides the count, which is the part worth reading.
 */
function describeChangePoints(changePoints: readonly number[], timeAxis: TimeAxis | null): string {
  if (changePoints.length === 0) return 'no change points'
  const name = (index: number): string => {
    if (timeAxis?.kind !== 'calendar') return String(index + 1)
    const stamp = timeAxis.timestamps[index]
    return stamp === undefined ? String(index + 1) : formatDay(stamp)
  }
  const named = changePoints.slice(0, NAMED_CHANGE_POINTS).map(name).join(', ')
  const remaining = changePoints.length - NAMED_CHANGE_POINTS
  return remaining > 0 ? `change points at ${named} and ${formatCount(remaining).text} more` : `change points at ${named}`
}

function SeriesRow({ facts, period }: { readonly facts: SeriesFacts; readonly period: number | null }) {
  const theme = useChartTheme()
  const option = useMemo(() => changePointsOption({ name: facts.name, values: facts.values, changePoints: facts.evidence.changePoints, stepLabel: 'observation' }, theme), [facts, theme])
  const correlation = useMemo(() => lagCorrelationOption({ name: facts.name, ...facts.evidence }, theme), [facts, theme])
  const strength = (value: number | null) => (value === null ? formatAbsent('notApplicable', period === null ? 'no seasonal period for yearly rows' : 'too few rows for two seasons') : formatStatistic('score', value))
  return (
    <li className={well('p-(--panel-space)')}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className="text-body font-medium text-ink">{facts.name}</span>
        <span className={num('text-micro text-faint')}><Metadata><span>{describeChangePoints(facts.evidence.changePoints, facts.timeAxis)}</span><span>penalty {formatStatistic('raw', facts.evidence.peltPenalty).text}</span></Metadata></span>
      </div>
      <div className={figureGrid('mt-2 @2xl/panel:grid-cols-3')}>
        <MetricTile label="Trend strength" size="compact" frame="cell" value={strength(facts.evidence.trendStrength)} context="seasonal-trend decomposition using loess (STL), 0 to 1" />
        <MetricTile label="Seasonal strength" size="compact" frame="cell" value={strength(facts.evidence.seasonalStrength)} context={period === null ? 'no period' : `period ${period}`} />
        <MetricTile label="Change points" size="compact" frame="cell" value={formatCount(facts.evidence.changePoints.length)} context="pruned exact linear time (PELT), L2 cost" />
      </div>
      <p className="mb-0 mt-2 text-label text-muted">Strengths near 1 mean that the fitted trend or seasonal component accounts for most of the variation remaining after the other component is removed; values near 0 indicate little such structure. PELT locations are the optimum for this penalty and minimum-segment choice, not hypothesis-test rejections.</p>
      <ExpandableChart option={option} label={`${facts.name} with PELT change points`} className="mt-2 h-[180px]" testId="change-points" />
      <div className="mt-3 border-t border-hair pt-3">
        <span className="text-body font-medium text-ink">Lag correlation</span>
        <p className="mb-0 mt-1 text-label text-muted">ACF compares the series with its earlier values. PACF measures the remaining relation at each lag after shorter lags are accounted for. The shaded regions show approximate 95% reference bands. These plots describe temporal dependence; they do not establish causal arrows.</p>
        <ExpandableChart option={correlation} label={`${facts.name} ACF and PACF`} className="mt-2 h-[430px]" testId="acf-pacf" />
      </div>
    </li>
  )
}

/** PELT, STL strengths, and notebook-compatible ACF/PACF for every prepared series. */
export function SeriesStructureCard({ source, profile, prepared, embedded = false, onResult }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
  /** Inside the Diagnostics card: no border of its own, body-weight heading. */
  readonly embedded?: boolean
  readonly onResult?: () => void
}) {
  const session = useJob('series-structure')
  const { job } = session
  const result = useWorkflow(state => state.temporalStructure?.prepared === prepared.id ? state.temporalStructure : null)
  const record = useWorkflow(state => state.recordTemporalStructure)
  const [minSize, setMinSize] = useState(4)
  const [maxLag, setMaxLag] = useState(40)
  const period = seasonalPeriodOf(prepared.sampling.frequency)

  const run = async () => {
    const id = session.start('analysis', 'Temporal structure')
    if (id === null) return
    session.progress(id, 'Temporal structure', { completed: 0, total: prepared.columns.length })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runSeriesStructure }] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(id)) return
      const matrix = await materialisePrepared(source, profile, prepared, prepared.columns)
      if (!session.current(id)) return
      if (!matrix.ok) { session.fail(id, describePreparedMaterialisationProblem(matrix.error)); return }
      const series: SeriesFacts[] = []
      const correlationMaxLag = Math.min(maxLag, Math.floor((matrix.value.rowCount - 1) / 2))
      if (correlationMaxLag < 1) { session.fail(id, 'At least three prepared observations are required for ACF and PACF.'); return }
      for (const [index, column] of matrix.value.columns.entries()) {
        const values = Array.from(matrix.value.values.subarray(index * matrix.value.rowCount, (index + 1) * matrix.value.rowCount))
        const result = await runSeriesStructure(Float64Array.from(values), matrix.value.rowCount, 1, { period, robust: false, correlationMaxLag, peltMinSize: minSize, peltJump: 1, peltPenalty: defaultPeltPenalty(values) })
        if (!session.current(id)) return
        if (!result.ok) { session.fail(id, `${column.name}: ${describeAnalysisWorkerProblem(result.error)}`); return }
        const evidence = result.value.series[0]
        if (evidence === undefined) { session.fail(id, `${column.name}: no structure evidence returned.`); return }
        series.push({ column: column.id, name: column.name, values, evidence, timeAxis: matrix.value.timeAxis })
        session.progress(id, 'Temporal structure', { completed: index + 1, total: prepared.columns.length })
      }
      record({ prepared: prepared.id, period, minSize, maxLag: correlationMaxLag, series })
      session.finish(id)
      onResult?.()
    } catch (cause: unknown) {
      session.fail(id, cause instanceof Error ? cause.message : String(cause))
    }
  }

  return (
    <section className={embedded ? undefined : panel('mt-4 p-(--panel-space)')} aria-labelledby="structure-title">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h3 id="structure-title" className={embedded ? 'm-0 text-body font-medium text-ink' : cn(sectionTitle, 'm-0')}>Temporal structure</h3>
          <p className="mb-0 mt-1 text-body text-faint">PELT estimates changes in the series mean. STL reports trend and seasonal strength{period === null ? ' (no seasonal period for yearly rows)' : ` at period ${period}`}. ACF and PACF show dependence across lags. This analysis does not change the prepared dataset.</p>
        </div>
        <div className="flex flex-wrap items-end gap-2">
          <label className="text-body text-ink"><span className={label('block text-faint')}>Min segment</span><input type="number" min={1} max={200} className={field('text', 'mt-1 w-20')} value={minSize} onChange={(event) => setMinSize(Math.max(1, Math.min(200, Number(event.target.value) || 1)))} /></label>
          <label className="text-body text-ink"><span className={label('block text-faint')}>Max lag</span><input type="number" min={1} max={400} className={field('text', 'mt-1 w-20')} value={maxLag} onChange={(event) => setMaxLag(Math.max(1, Math.min(400, Number(event.target.value) || 1)))} /></label>
          <button type="button" className={button('quiet')} disabled={session.blocked || job.kind === 'running'} aria-busy={job.kind === 'running'} onClick={() => void run()}>
            Analyse temporal structure
          </button>
          {job.kind === 'running' && <button type="button" className={button('quiet')} onClick={session.cancel}>Cancel analysis</button>}
        </div>
      </div>
      <MethodCaveats methods={SERIES_STRUCTURE_METHODS} />
      <JobNotice job={job} />
      {result !== null && (
        <>
          <p role="status" className="mb-2 mt-4 flex items-center gap-2 text-body text-muted"><Metadata><span><Icon name="check_circle" size={16} className="text-ok" /> {result.series.length} series checked</span><span>min segment {result.minSize}</span></Metadata></p>
          <ul className="m-0 list-none space-y-2 p-0" aria-label="Series structure">
            {result.series.map((facts) => <SeriesRow key={facts.column} facts={facts} period={result.period} />)}
          </ul>
        </>
      )}
    </section>
  )
}
