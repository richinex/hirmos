import { adjustmentLabels, type EstimationRunArtifact } from '@/domain/estimation'

/** Display the fitted comparison covariates, not an unrelated DAG adjustment record. */
export function SensitivityAdjustment({ run }: { readonly run: EstimationRunArtifact }) {
  const panelCovariates =
    run.kind === 'panel-intervention-run' &&
    (run.evidence.kind === 'panelAdjusted' || run.evidence.kind === 'staggeredDid')
  const names = panelCovariates
    ? run.columns.slice(2).map((column) => column.name)
    : adjustmentLabels(run.estimate.adjustment)
  return (
    <>
      <dt className="text-faint">{panelCovariates ? 'Baseline covariates' : 'Adjustment set'}</dt>
      <dd className="m-0 text-ink">{names.join(', ') || 'None'}</dd>
    </>
  )
}
