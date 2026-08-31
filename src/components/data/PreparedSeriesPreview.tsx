import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { changePointsOption } from '@/charts/sensitivity/changePoints'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { button, label, num } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { describeSeriesTransform, seriesTransformFor, type PreparedDatasetArtifact } from '@/domain/preprocessing'
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
  readonly seasonallyAdjusted: boolean
  /** The recipe's stations for this column, ending with the values every chapter reads. */
  readonly stages: readonly PreparedStage[]
}

type PreviewJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'loading' }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'ready'; readonly rows: number; readonly leadingRowsRemoved: number; readonly series: readonly PreparedSeries[] }

function StagePlot({ name, stage, final }: { readonly name: string; readonly stage: PreparedStage; readonly final: boolean }) {
  const theme = useChartTheme()
  const option = useMemo(() => changePointsOption({
    name,
    values: stage.values,
    changePoints: [],
    stepLabel: 'row',
  }, theme), [name, stage, theme])
  return (
    <li>
      <span className={label('text-faint')}>{stage.label}</span>
      <EChart option={option} label={`${name}, ${stage.label}`} className="mt-1 h-[110px]" testId={final ? 'prepared-series' : undefined} />
    </li>
  )
}

function PreparedSeriesCell({ series }: { readonly series: PreparedSeries }) {
  return (
    <li className="rounded-lg border border-hair bg-well p-3">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className="text-body font-medium text-ink">{series.name}</span>
        <span className={label('text-faint')}>{series.transform}{series.seasonallyAdjusted ? ' · STL-adjusted' : ''}</span>
      </div>
      <ul className="m-0 mt-2 grid list-none gap-2 p-0" aria-label={`${series.name} preparation stages`}>
        {series.stages.map((stage, index) => (
          <StagePlot key={stage.label} name={series.name} stage={stage} final={index === series.stages.length - 1} />
        ))}
      </ul>
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

  const load = async () => {
    setJob({ kind: 'loading' })
    try {
      const { materialisePreparedStages, describePreparedMaterialisationProblem } = await import('@/data/prepared')
      const stages = await materialisePreparedStages(source, profile, prepared, prepared.columns)
      if (!stages.ok) { setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(stages.error) }); return }
      const { resolved, adjusted, final } = stages.value
      const series = final.columns.map((column, index): PreparedSeries => {
        const transform = seriesTransformFor(prepared.seriesTransforms, column.id)
        const seasonallyAdjusted = prepared.seasonalAdjustment.kind === 'stl' && prepared.seasonalAdjustment.columns.includes(column.id)
        const finalStage = { label: transform.kind === 'levels' ? 'Final' : `Final · ${describeSeriesTransform(transform)}`, values: columnValues(final.values, final.rowCount, index) }
        const stack = seasonallyAdjusted || transform.kind !== 'levels'
          ? [
            { label: 'Resolved source', values: columnValues(resolved.values, resolved.rowCount, index) },
            ...(seasonallyAdjusted && adjusted !== null ? [{ label: 'After STL', values: columnValues(adjusted.values, adjusted.rowCount, index) }] : []),
            finalStage,
          ]
          : [finalStage]
        return {
          column: column.id,
          name: column.name,
          transform: describeSeriesTransform(transform),
          seasonallyAdjusted,
          stages: stack,
        }
      })
      setJob({ kind: 'ready', rows: final.rowCount, leadingRowsRemoved: final.leadingRowsRemoved, series })
    } catch (cause: unknown) {
      setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  return (
    <section className="mt-4 rounded-xl border border-hair bg-panel p-4" aria-labelledby="prepared-preview-title">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h3 id="prepared-preview-title" className="m-0 text-title font-medium text-ink">Prepared values</h3>
          <p className="mb-0 mt-1 text-body text-faint">Inspect the exact values passed to diagnostics, discovery methods, and estimators. An adjusted column shows each station of its recipe.</p>
        </div>
        <button type="button" className={button('quiet')} aria-busy={job.kind === 'loading'} onClick={job.kind === 'loading' ? undefined : () => void load()}>
          {job.kind === 'ready' ? 'Refresh preview' : 'Preview prepared values'}
        </button>
      </div>
      {job.kind === 'failed' && <p role="alert" className="mb-0 mt-3 text-body text-danger">{job.detail}</p>}
      {job.kind === 'ready' && (
        <>
          <p role="status" className="mb-3 mt-4 flex flex-wrap items-center gap-2 text-body text-muted">
            <Icon name="check_circle" size={16} className="text-ok" />
            <span className={num()}>{formatCount(job.rows, { noun: 'aligned rows' }).text}</span>
            {job.leadingRowsRemoved > 0 && <span>· first source row removed for alignment</span>}
          </p>
          <ul className="m-0 grid list-none gap-2 p-0 @2xl/panel:grid-cols-2" aria-label="Prepared series plots">
            {job.series.map((series) => <PreparedSeriesCell key={series.column} series={series} />)}
          </ul>
        </>
      )}
    </section>
  )
}
