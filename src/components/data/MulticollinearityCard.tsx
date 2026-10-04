import { RunActions } from '@/components/ui/RunActions'
import { Metadata } from '@/components/ui/Metadata'
import { useJob } from '@/analysis/JobsProvider'
import { useWorkflow } from '@/components/WorkflowProvider'
import { JobNotice } from '@/components/ui/JobNotice'
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
  type MulticollinearitySelection,
} from '@/domain/multicollinearity'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Availability =
  | { readonly kind: 'available' }
  | { readonly kind: 'unavailable'; readonly reason: string }

const nameAt = (matrix: PreparedMatrix, index: number): string => matrix.columns[index]?.name ?? `Variable ${index + 1}`

export function MulticollinearityCard({ source, profile, prepared, onSelection }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly onSelection: (selection: MulticollinearitySelection) => void
}) {
  const theme = useChartTheme()
  const settings = useWorkflow(state => state.diagnosticDraft?.prepared === prepared.id ? state.diagnosticDraft.redundancy : null)
  const change = useWorkflow(state => state.changeDiagnostic)
  const session = useJob('redundancy')
  const { job } = session
  const result = useWorkflow(state => state.redundancy?.prepared === prepared.id ? state.redundancy : null)
  const record = useWorkflow(state => state.recordRedundancy)
  const [selectionProblem, setSelectionProblem] = useState<string | null>(null)
  if (settings === null) throw new Error('Redundancy diagnostics require the current prepared dataset.')
  const { correlation: correlationThreshold, vif: vifThreshold, pair } = settings
  const setCorrelationThreshold = (value: number) => change(prepared.id, { type: 'correlation', value })
  const setVifThreshold = (value: number) => change(prepared.id, { type: 'vif', value })
  const setPair = (value: readonly [number, number]) => change(prepared.id, { type: 'pair', value })
  const availability: Availability = prepared.observations < 3
    ? { kind: 'unavailable', reason: 'At least three prepared rows are required.' }
    : prepared.columns.length < 2 || prepared.columns.length > 64
      ? { kind: 'unavailable', reason: 'Select between two and 64 analysis columns.' }
      : { kind: 'available' }

  const run = async () => {
    if (availability.kind === 'unavailable') return
    const id = session.start('analysis', 'Predictor redundancy')
    if (id === null) return
    setSelectionProblem(null)
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runMulticollinearity }] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      if (!session.current(id)) return
      const matrix = await materialisePrepared(source, profile, prepared, prepared.columns)
      if (!session.current(id)) return
      if (!matrix.ok) {
        session.fail(id, describePreparedMaterialisationProblem(matrix.error))
        return
      }
      const result = await runMulticollinearity(
        matrix.value.values.slice(),
        matrix.value.rowCount,
        matrix.value.columns.length,
        { correlation: correlationThreshold, vif: vifThreshold },
      )
      if (!session.current(id)) return
      if (!result.ok) {
        session.fail(id, describeAnalysisWorkerProblem(result.error))
        return
      }
      const firstCluster = result.value.correlationClusters.find((cluster) => cluster.length > 1)
      setPair(firstCluster === undefined ? [0, 1] : [firstCluster[0] ?? 0, firstCluster[1] ?? 1])
      record({ prepared: prepared.id, matrix: matrix.value, evidence: result.value })
      session.finish(id)
    } catch (cause: unknown) {
      session.fail(id, cause instanceof Error ? cause.message : String(cause))
    }
  }

  const apply = (method: 'correlation' | 'vif') => {
    if (result === null) return
    const selection = multicollinearitySelection(result.evidence, result.matrix.columns, method)
    if (!selection.ok) {
      setSelectionProblem('The diagnostic result does not match the columns currently prepared. Rerun the diagnostic.')
      return
    }
    onSelection(selection.value)
  }

  const heatmap = useMemo(() => result !== null
    ? matrixHeatmapOption({
      title: 'Pairwise Pearson correlations',
      sources: result.matrix.columns.map((column) => column.name),
      targets: result.matrix.columns.map((column) => column.name),
      values: result.evidence.correlation,
      scale: 'signed',
      quantity: 'Pearson r',
      relation: 'symmetric',
      threshold: result.evidence.correlationThreshold,
    }, theme)
    : null, [result, theme])

  const scatter = useMemo(() => {
    if (result === null) return null
    const [xIndex, yIndex] = pair
    const rows = result.matrix.rowCount
    const x = Array.from(result.matrix.values.subarray(xIndex * rows, (xIndex + 1) * rows))
    const y = Array.from(result.matrix.values.subarray(yIndex * rows, (yIndex + 1) * rows))
    return pairwiseScatterOption({
      xName: nameAt(result.matrix, xIndex),
      yName: nameAt(result.matrix, yIndex),
      x,
      y,
      correlation: result.evidence.correlation[xIndex]?.[yIndex] ?? 0,
    }, theme)
  }, [result, pair, theme])

  return (
    <section aria-labelledby="multicollinearity-title">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <div className="flex items-center gap-1">
            <h4 id="multicollinearity-title" className="m-0 text-body font-medium text-ink">Predictor redundancy</h4>
            <ParameterHelp label="predictor redundancy" help="These diagnostics describe numerical redundancy, not confounding or a causal adjustment set; do not remove a required causal variable solely because it is correlated with another predictor." />
          </div>
          <p className="mb-0 mt-1 max-w-[78ch] text-body text-faint">Pairwise correlation groups variables with similar or opposite linear variation. The variance inflation factor (VIF) measures how much linear dependence on the other predictors inflates a coefficient’s variance. The selection procedure repeatedly removes the variable with the highest VIF until the threshold is met.</p>
        </div>
        {/* The thresholds are ordinary fields; on a narrow panel they fill the width and the button takes the row beneath. */}
        <div className="flex w-full flex-col gap-3 @xl/panel:w-auto @xl/panel:flex-row @xl/panel:items-end">
          <div className="grid grid-cols-2 gap-3">
            <label className="block"><span className={fieldLabel}>|r| threshold</span><input type="number" min={0.01} max={1} step={0.01} className={field('text', 'mt-1 w-full @xl/panel:w-28')} value={correlationThreshold} onChange={(event) => setCorrelationThreshold(Math.max(0.01, Math.min(1, Number(event.target.value) || 0.9)))} /></label>
            <label className="block"><span className={fieldLabel}>VIF threshold</span><input type="number" min={1.01} step={0.5} className={field('text', 'mt-1 w-full @xl/panel:w-28')} value={vifThreshold} onChange={(event) => setVifThreshold(Math.max(1.01, Number(event.target.value) || 10))} /></label>
          </div>
          <RunActions running={job.kind === 'running'} onCancel={session.cancel} orbLabel="Redundancy analysis running">
            <button type="button" className={button('quiet')} disabled={availability.kind === 'unavailable' || session.blocked || job.kind === 'running'} aria-busy={job.kind === 'running'} onClick={() => void run()}>Analyse redundancy</button>
          </RunActions>
        </div>
      </div>

      {availability.kind === 'unavailable' && <p role="status" className="mb-0 mt-3 text-body text-faint">{availability.reason}</p>}
      <JobNotice job={job} />
      {selectionProblem !== null && <Alert tone="danger" className="mt-3"><p className="m-0">{selectionProblem}</p></Alert>}
      {result !== null && heatmap !== null && scatter !== null && (
        <div className="mt-4 space-y-4">
          <p role="status" className="m-0 flex items-center gap-2 text-body text-muted"><Metadata><span><Icon name="check_circle" size={16} className="text-ok" /> {result.evidence.variables} variables</span><span>{formatCount(result.evidence.observations).text} rows</span></Metadata></p>
          <div className="grid gap-4 @3xl/panel:grid-cols-2">
            <div className={well('p-(--panel-space)')}>
              <h5 className="m-0 text-body font-medium text-ink">Correlation groups</h5>
              <p className="mb-2 mt-1 text-label text-muted">Variables are grouped using complete linkage with distance 1 − |r|. The recommendation retains the first selected variable in each group. Review that representative before applying the selection.</p>
              <ul className="m-0 space-y-1 pl-5 text-body text-muted">
                {result.evidence.correlationClusters.filter((cluster) => cluster.length > 1).map((cluster) => <li key={cluster.join('-')}>{cluster.map((index) => nameAt(result.matrix, index)).join(', ')}</li>)}
                {result.evidence.correlationDrop.length === 0 && <li>No group crossed |r| ≥ {result.evidence.correlationThreshold}.</li>}
              </ul>
              {result.evidence.correlationDrop.length > 0 && <button type="button" className={button('quiet', 'mt-3')} onClick={() => apply('correlation')}>Use correlation selection</button>}
            </div>
            <div className={well('p-(--panel-space)')}>
              <h5 className="m-0 text-body font-medium text-ink">VIF elimination path</h5>
              <p className="mb-2 mt-1 text-label text-muted">At each step, the variable with the highest VIF is removed. If there is a tie, the later-selected column is removed.</p>
              {result.evidence.vifHistory.length === 0
                ? <p className="m-0 text-body text-muted">Every VIF is below {result.evidence.vifThreshold}.</p>
                : <div className="overflow-x-auto"><table className={table}><thead><tr><th className={th()}>Step</th><th className={th()}>Removed</th><th className={th('text-right')}>VIF</th></tr></thead><tbody>{result.evidence.vifHistory.map((entry, index) => <tr key={`${entry.column}-${index}`} className={tr()}><td className={td()}>{index + 1}</td><td className={td()}>{nameAt(result.matrix, entry.column)}</td><td className={td(num('text-right'))}>{entry.vif === null ? '∞' : formatStatistic('raw', entry.vif).text}</td></tr>)}</tbody></table></div>}
              {result.evidence.vifDrop.length > 0 && <button type="button" className={button('quiet', 'mt-3')} onClick={() => apply('vif')}>Use VIF selection</button>}
            </div>
          </div>
          <ExpandableChart option={heatmap} label="Pairwise correlation matrix" className="h-[360px]" testId="correlation-matrix" />
          <div>
            <div className="mb-2 flex flex-wrap items-end gap-2">
              <label><span className={fieldLabel}>X axis</span><Select value={pair[0]} className={field('text', 'mt-1 w-48')} onChange={(event) => setPair([Number(event.target.value), pair[1]])}>{result.matrix.columns.map((column, index) => <option key={column.id} value={index}>{column.name}</option>)}</Select></label>
              <label><span className={fieldLabel}>Y axis</span><Select value={pair[1]} className={field('text', 'mt-1 w-48')} onChange={(event) => setPair([pair[0], Number(event.target.value)])}>{result.matrix.columns.map((column, index) => <option key={column.id} value={index}>{column.name}</option>)}</Select></label>
            </div>
            <ExpandableChart option={scatter} label={`${nameAt(result.matrix, pair[1])} against ${nameAt(result.matrix, pair[0])}`} className="h-[320px]" testId="pairwise-scatter" />
          </div>
        </div>
      )}
    </section>
  )
}
