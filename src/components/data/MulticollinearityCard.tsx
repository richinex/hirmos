import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { matrixHeatmapOption } from '@/charts/discovery/matrixHeatmap'
import { pairwiseScatterOption } from '@/charts/data/pairwiseScatter'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { Select } from '@/components/ui/Select'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { button, field, fieldLabel, num, table, td, th, tr, well } from '@/components/ui/recipes'
import type { PreparedMatrix } from '@/data/prepared'
import type { DatasetProfile } from '@/domain/dataset'
import {
  multicollinearitySelection,
  type MulticollinearityEvidence,
  type MulticollinearitySelection,
} from '@/domain/multicollinearity'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatStatistic } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running' }
  | { readonly kind: 'ready'; readonly matrix: PreparedMatrix; readonly evidence: MulticollinearityEvidence }
  | { readonly kind: 'failed'; readonly detail: string }

type Availability =
  | { readonly kind: 'available' }
  | { readonly kind: 'unavailable'; readonly reason: string }

const nameAt = (matrix: PreparedMatrix, index: number): string => matrix.columns[index]?.name ?? `Variable ${index + 1}`

export function MulticollinearityCard({ source, profile, prepared, onSelection, onResult }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly onSelection: (selection: MulticollinearitySelection) => void
  readonly onResult?: () => void
}) {
  const theme = useChartTheme()
  const [correlationThreshold, setCorrelationThreshold] = useState(0.9)
  const [vifThreshold, setVifThreshold] = useState(10)
  const [pair, setPair] = useState<readonly [number, number]>([0, 1])
  const [job, setJob] = useState<Job>({ kind: 'idle' })
  const availability: Availability = prepared.observations < 3
    ? { kind: 'unavailable', reason: 'At least three prepared rows are required.' }
    : prepared.columns.length < 2 || prepared.columns.length > 64
      ? { kind: 'unavailable', reason: 'Select between two and 64 analysis columns.' }
      : { kind: 'available' }

  const run = async () => {
    if (availability.kind === 'unavailable') return
    setJob({ kind: 'running' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runMulticollinearity }] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      const matrix = await materialisePrepared(source, profile, prepared, prepared.columns)
      if (!matrix.ok) {
        setJob({ kind: 'failed', detail: describePreparedMaterialisationProblem(matrix.error) })
        return
      }
      const result = await runMulticollinearity(
        matrix.value.values.slice(),
        matrix.value.rowCount,
        matrix.value.columns.length,
        { correlation: correlationThreshold, vif: vifThreshold },
      )
      if (!result.ok) {
        setJob({ kind: 'failed', detail: describeAnalysisWorkerProblem(result.error) })
        return
      }
      const firstCluster = result.value.correlationClusters.find((cluster) => cluster.length > 1)
      setPair(firstCluster === undefined ? [0, 1] : [firstCluster[0] ?? 0, firstCluster[1] ?? 1])
      setJob({ kind: 'ready', matrix: matrix.value, evidence: result.value })
      onResult?.()
    } catch (cause: unknown) {
      setJob({ kind: 'failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  const apply = (method: 'correlation' | 'vif') => {
    if (job.kind !== 'ready') return
    const selection = multicollinearitySelection(job.evidence, job.matrix.columns, method)
    if (!selection.ok) {
      setJob({ kind: 'failed', detail: 'The diagnostic result no longer matches the prepared columns. Run it again.' })
      return
    }
    onSelection(selection.value)
  }

  const heatmap = useMemo(() => job.kind === 'ready'
    ? matrixHeatmapOption({
      title: 'Pairwise Pearson correlations',
      sources: job.matrix.columns.map((column) => column.name),
      targets: job.matrix.columns.map((column) => column.name),
      values: job.evidence.correlation,
      scale: 'signed',
      quantity: 'Pearson r',
      relation: 'symmetric',
    }, theme)
    : null, [job, theme])

  const scatter = useMemo(() => {
    if (job.kind !== 'ready') return null
    const [xIndex, yIndex] = pair
    const rows = job.matrix.rowCount
    const x = Array.from(job.matrix.values.subarray(xIndex * rows, (xIndex + 1) * rows))
    const y = Array.from(job.matrix.values.subarray(yIndex * rows, (yIndex + 1) * rows))
    return pairwiseScatterOption({
      xName: nameAt(job.matrix, xIndex),
      yName: nameAt(job.matrix, yIndex),
      x,
      y,
      correlation: job.evidence.correlation[xIndex]?.[yIndex] ?? 0,
    }, theme)
  }, [job, pair, theme])

  return (
    <section aria-labelledby="multicollinearity-title">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <div className="flex items-center gap-1">
            <h4 id="multicollinearity-title" className="m-0 text-body font-medium text-ink">Predictor redundancy</h4>
            <ParameterHelp label="predictor redundancy" help="These diagnostics describe numerical redundancy, not confounding or a causal adjustment set; do not remove a required causal variable solely because it is correlated with another predictor." />
          </div>
          <p className="mb-0 mt-1 max-w-[78ch] text-body text-faint">Pairwise correlation groups variables with similar linear variation. Variance inflation factor (VIF) measures how well each variable is explained by all the others, then removes the largest value until the threshold is met.</p>
        </div>
        <div className="flex flex-wrap items-end gap-2">
          <label className="text-body text-ink"><span className={fieldLabel}>|r| threshold</span><input type="number" min={0.01} max={1} step={0.01} className={field('text', 'mt-1 w-24')} value={correlationThreshold} onChange={(event) => setCorrelationThreshold(Math.max(0.01, Math.min(1, Number(event.target.value) || 0.9)))} /></label>
          <label className="text-body text-ink"><span className={fieldLabel}>VIF threshold</span><input type="number" min={1.01} step={0.5} className={field('text', 'mt-1 w-24')} value={vifThreshold} onChange={(event) => setVifThreshold(Math.max(1.01, Number(event.target.value) || 10))} /></label>
          <button type="button" className={button('quiet')} disabled={availability.kind === 'unavailable'} aria-busy={job.kind === 'running'} onClick={job.kind === 'running' ? undefined : () => void run()}>Analyse redundancy</button>
        </div>
      </div>

      {availability.kind === 'unavailable' && <p role="status" className="mb-0 mt-3 text-body text-faint">{availability.reason}</p>}
      {job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{job.detail}</p></Alert>}
      {job.kind === 'ready' && heatmap !== null && scatter !== null && (
        <div className="mt-4 space-y-4">
          <p role="status" className="m-0 flex items-center gap-2 text-body text-muted"><Icon name="check_circle" size={16} className="text-ok" /> {job.evidence.variables} variables · {job.evidence.observations.toLocaleString()} rows</p>
          <div className="grid gap-4 @3xl/panel:grid-cols-2">
            <div className={well('p-3')}>
              <h5 className="m-0 text-body font-medium text-ink">Correlation groups</h5>
              <p className="mb-2 mt-1 text-label text-muted">Complete linkage over 1 − |r|. The deterministic recommendation retains the first selected variable in each group; review that representative before applying it.</p>
              <ul className="m-0 space-y-1 pl-5 text-body text-muted">
                {job.evidence.correlationClusters.filter((cluster) => cluster.length > 1).map((cluster) => <li key={cluster.join('-')}>{cluster.map((index) => nameAt(job.matrix, index)).join(', ')}</li>)}
                {job.evidence.correlationDrop.length === 0 && <li>No group crossed |r| ≥ {job.evidence.correlationThreshold}.</li>}
              </ul>
              {job.evidence.correlationDrop.length > 0 && <button type="button" className={button('quiet', 'mt-3')} onClick={() => apply('correlation')}>Use correlation selection</button>}
            </div>
            <div className={well('p-3')}>
              <h5 className="m-0 text-body font-medium text-ink">VIF elimination path</h5>
              <p className="mb-2 mt-1 text-label text-muted">At each step the variable with the largest VIF is removed; ties are resolved by the later selected column.</p>
              {job.evidence.vifHistory.length === 0
                ? <p className="m-0 text-body text-muted">Every VIF is below {job.evidence.vifThreshold}.</p>
                : <div className="overflow-x-auto"><table className={table}><thead><tr><th className={th()}>Step</th><th className={th()}>Removed</th><th className={th('text-right')}>VIF</th></tr></thead><tbody>{job.evidence.vifHistory.map((entry, index) => <tr key={`${entry.column}-${index}`} className={tr()}><td className={td()}>{index + 1}</td><td className={td()}>{nameAt(job.matrix, entry.column)}</td><td className={td(num('text-right'))}>{entry.vif === null ? '∞' : formatStatistic('raw', entry.vif).text}</td></tr>)}</tbody></table></div>}
              {job.evidence.vifDrop.length > 0 && <button type="button" className={button('quiet', 'mt-3')} onClick={() => apply('vif')}>Use VIF selection</button>}
            </div>
          </div>
          <ExpandableChart option={heatmap} label="Pairwise correlation matrix" className="h-[360px]" testId="correlation-matrix" />
          <div>
            <div className="mb-2 flex flex-wrap items-end gap-2">
              <label><span className={fieldLabel}>X axis</span><Select value={pair[0]} className={field('text', 'mt-1 w-48')} onChange={(event) => setPair([Number(event.target.value), pair[1]])}>{job.matrix.columns.map((column, index) => <option key={column.id} value={index}>{column.name}</option>)}</Select></label>
              <label><span className={fieldLabel}>Y axis</span><Select value={pair[1]} className={field('text', 'mt-1 w-48')} onChange={(event) => setPair([pair[0], Number(event.target.value)])}>{job.matrix.columns.map((column, index) => <option key={column.id} value={index}>{column.name}</option>)}</Select></label>
            </div>
            <ExpandableChart option={scatter} label={`${nameAt(job.matrix, pair[1])} against ${nameAt(job.matrix, pair[0])}`} className="h-[320px]" testId="pairwise-scatter" />
          </div>
        </div>
      )}
    </section>
  )
}
