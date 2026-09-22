import { Alert } from '@/components/ui/Alert'
import { armaUncertaintyWarning, type ArmaErrorEvidence } from '@/domain/interruptedSeries'

/** Missing diagnostics on older saved runs are not evidence of a full-rank fit. */
export function ArmaUncertaintyAlert({ evidence }: { readonly evidence: ArmaErrorEvidence | undefined }) {
  const warning = evidence === undefined ? null : armaUncertaintyWarning(evidence)
  return warning === null ? null : <Alert tone="warn" live={false} className="mt-3"><p className="m-0">{warning}</p></Alert>
}
