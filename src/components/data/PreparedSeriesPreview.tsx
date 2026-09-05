import { useCallback, useEffect, useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { decompositionOption } from '@/charts/data/decomposition'
import { changePointsOption } from '@/charts/sensitivity/changePoints'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { button, caption, label, panel, well } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { mapNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { describeSeriesTransform, seriesTransformFor, type PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SeasonalAdjustedEvidence } from '@/domain/seasonal'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount } from '@/lib/format/number'

interface PreparedStage {
  readonly label: string
  readonly values: readonly number[]
}

interface PreparedSeries {
  readonly column: ColumnId
  readonly name: string
  readonly transform: string
  readonly decomposition: SeasonalAdjustedEvidence['adjusted'][number] | null
  readonly time: readonly number[]
  readonly calendar: boolean
  /** The recipe's stations for this column, ending with the values every chapter reads. */
  readonly stages: NonEmptyArray<PreparedStage>
}

type PreviewJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'ready'; readonly rows: number; readonly leadingRowsRemoved: number; readonly series: NonEmptyArray<PreparedSeries> }

function StagePlot({ name, stage, final, labelled }: { readonly name: string; readonly stage: PreparedStage; readonly final: boolean; readonly labelled: boolean }) {
  const theme = useChartTheme()
  const option = useMemo(() => changePointsOption({
    name,
    values: stage.values,
    changePoints: [],
    stepLabel: 'row',
    zoom: false,
  }, theme), [name, stage, theme])
  return (
    <li>
      {/* The stage name separates one step of the preparation from the next, so a lone stage is left unlabelled. */}
      {labelled && <span className={label('text-faint')}>{stage.label}</span>}
      <ExpandableChart option={option} label={`${name}, ${stage.label}`} className={`${labelled ? 'mt-1' : ''} h-[110px]`} testId={final ? 'prepared-series' : undefined} />
    </li>
  )
}

function DecompositionPlot({ series }: { readonly series: PreparedSeries & { readonly decomposition: NonNullable<PreparedSeries['decomposition']> } }) {
  const theme = useChartTheme()
  const option = useMemo(() => decompositionOption({
    name: series.name,
    time: series.time,
    calendar: series.calendar,
    observed: series.decomposition.observed,
    trend: series.decomposition.trend,
    seasonal: series.decomposition.seasonal,
    remainder: series.decomposition.remainder,
  }, theme), [series, theme])
  return (
    <div className="mt-3 border-t border-hair pt-3">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className="text-body font-medium text-ink">STL decomposition</span>
        <span className={caption()}>trend strength {series.decomposition.trendStrength.toFixed(3)} · seasonal strength {series.decomposition.seasonalStrengthBefore.toFixed(3)}</span>
      </div>
      <p className="mb-0 mt-1 text-label text-muted">Observed = trend + seasonal + remainder at every retained time point. The components describe temporal structure; the decomposition does not assign causal meaning.</p>
      <ExpandableChart option={option} label={`${series.name} STL decomposition`} className="mt-2 h-[520px]" testId="stl-decomposition" />
    </div>
  )
}

function PreparedSeriesCell({ series }: { readonly series: PreparedSeries }) {
  return (
    <li className={well('p-(--panel-space)')}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className="text-body font-medium text-ink">{series.name}</span>
        <span className={caption()}>{series.transform}{series.decomposition !== null ? ' · STL-adjusted' : ''}</span>
      </div>
      <ul className="m-0 mt-2 grid list-none gap-2 p-0" aria-label={`${series.name} preparation stages`}>
        {series.stages.map((stage, index) => (
          <StagePlot key={stage.label} name={series.name} stage={stage} final={index === series.stages.length - 1} labelled={series.stages.length > 1} />
        ))}
      </ul>
      {series.decomposition !== null && <DecompositionPlot series={{ ...series, decomposition: series.decomposition }} />}
    </li>
  )
}

const columnValues = (values: Float64Array, rowCount: number, index: number): readonly number[] =>
  Array.from(values.subarray(index * rowCount, (index + 1) * rowCount))

/** Exact post-recipe values, drawn as aligned small multiples; an adjusted column stacks each station of its recipe. */
export function PreparedSeriesPreview({ source, profile, prepared }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>
}) {
  const [job, setJob] = useState<PreviewJob>({ kind: 'idle' })
  const [open, setOpen] = useState(true)
  const [busy, setBusy] = useState(false)

  const load = useCallback(async () => {
    setBusy(true)
    try {
      const { materialisePreparedStages, describePreparedMaterialisationProblem } = await import('@/data/prepared')
      const stages = await materialisePreparedStages(source, profile, prepared, prepared.columns)
      if (!stages.ok) { setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(stages.error) }); return }
      const { resolved, resampled, adjusted, stl, final } = stages.value
      const series = mapNonEmpty(final.columns, (column, index): PreparedSeries => {
        const transform = seriesTransformFor(prepared.seriesTransforms, column.id)
        const seasonallyAdjusted = prepared.seasonalAdjustment.kind === 'stl' && prepared.seasonalAdjustment.columns.includes(column.id)
        const finalStage = { label: transform.kind === 'levels' ? 'Final' : `Final · ${describeSeriesTransform(transform)}`, values: columnValues(final.values, final.rowCount, index) }
        const prior = adjusted ?? resampled ?? resolved
        const timeAxis = prior.timeAxis
        const stack: NonEmptyArray<PreparedStage> = seasonallyAdjusted || transform.kind !== 'levels' || resampled !== null
          ? [
            { label: 'Resolved source', values: columnValues(resolved.values, resolved.rowCount, index) },
            ...(resampled !== null ? [{ label: `After ${prepared.sampling.frequency} resampling`, values: columnValues(resampled.values, resampled.rowCount, index) }] : []),
            ...(seasonallyAdjusted && adjusted !== null ? [{ label: 'After STL', values: columnValues(adjusted.values, adjusted.rowCount, index) }] : []),
            ...(transform.kind === 'levels' ? [] : [finalStage]),
          ]
          : [finalStage]
        const decomposition = stl?.adjusted.find(({ column: adjustedColumn }) => adjustedColumn === index) ?? null
        return {
          column: column.id,
          name: column.name,
          transform: describeSeriesTransform(transform),
          decomposition,
          time: timeAxis === null ? Array.from({ length: prior.rowCount }, (_, row) => row + 1) : Array.from(timeAxis.kind === 'calendar' ? timeAxis.timestamps : timeAxis.values),
          calendar: timeAxis?.kind === 'calendar',
          stages: stack,
        }
      })
      setJob({ kind: 'ready', rows: final.rowCount, leadingRowsRemoved: final.leadingRowsRemoved, series })
    } catch (cause: unknown) {
      setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) })
    } finally {
      setBusy(false)
    }
  }, [prepared, profile, source])

  useEffect(() => { void load() }, [load])

  return (
    <section className={panel('mt-4 p-(--panel-space)')} aria-labelledby="prepared-preview-title">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h3 id="prepared-preview-title" className="m-0 text-title font-medium text-ink">Prepared values</h3>
          <p className="mb-0 mt-1 text-body text-faint">Inspect the exact values passed to diagnostics, discovery methods, and estimators. An adjusted column shows each station of its recipe.</p>
        </div>
        <button type="button" className={button('quiet')} aria-busy={busy} onClick={busy ? undefined : () => void load()}>
          {job.kind === 'ready' ? 'Refresh preview' : 'Loading preview'}
        </button>
      </div>
      {job.kind === 'failed' && <p role="alert" className="mb-0 mt-3 text-body text-danger">{job.detail}</p>}
      {job.kind === 'ready' && (
        <div className="mt-4 rounded-md border border-line">
          <button
            type="button"
            onClick={() => setOpen((value) => !value)}
            aria-expanded={open}
            className={'flex w-full items-center gap-1.5 px-2.5 py-1.5 text-label text-muted transition-colors hover:text-ink'}
          >
            <Icon name="expand_more" size={14} className={`shrink-0 transition-transform duration-(--motion-fast) ${open ? 'rotate-180' : ''}`} />
            <span>Prepared series · {formatCount(job.rows, { noun: 'aligned rows' }).text}{job.leadingRowsRemoved > 0 ? ' · first source row removed for alignment' : ''}</span>
          </button>
          <div className="grid transition-[grid-template-rows] duration-(--motion-base)" style={{ gridTemplateRows: open ? '1fr' : '0fr' }}>
            <div className="overflow-hidden">
              <ul className="m-0 grid list-none grid-cols-[repeat(auto-fit,minmax(min(28rem,100%),1fr))] gap-2 border-t border-hair p-2" aria-label="Prepared series plots">
                {job.series.map((series) => <PreparedSeriesCell key={series.column} series={series} />)}
              </ul>
            </div>
          </div>
        </div>
      )}
    </section>
  )
}
