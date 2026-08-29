import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { assertNever } from '@/domain/dop'
import { isStageNote, type MethodEligibility } from '@/domain/methods'

/** Concise pre-run status. Detailed conditions and evidence are centralised in MethodCaveats. */
export function EligibilityView({ eligibility, subject = 'this prepared dataset' }: {
  readonly eligibility: MethodEligibility
  /** What the refusal is about, for the headline: "this prepared dataset", "this study". */
  readonly subject?: string
}) {
  // The one unassessed condition worth a sentence on the stage: series that are integrated in levels.
  const stationarityNote = eligibility.kind === 'caution'
    ? eligibility.unresolved.find(isStageNote)?.missingEvidence ?? null
    : null
  switch (eligibility.kind) {
    case 'eligible':
      return (
        <Alert tone="ok" live={false} className="mt-4">
          <p className="m-0 flex items-center gap-2"><Icon name="check_circle" size={16} /> Available; all requirements checked</p>
        </Alert>
      )
    case 'caution':
      return (
        <Alert tone="warn" live={false} className="mt-4">
          <p className="m-0 flex items-center gap-2"><Icon name="warning" size={16} /> Available; some requirements not checked</p>
          {stationarityNote !== null && <p className="mb-0 mt-1 text-muted">{stationarityNote}</p>}
        </Alert>
      )
    case 'refused':
      return (
        <Alert tone="danger" className="mt-4">
          <p className="m-0 flex items-center gap-2"><Icon name="block" size={16} /> Requirements not met for {subject}</p>
          <ul className="mb-0 mt-2 space-y-2 pl-4 text-muted">
            {eligibility.violations.map((evaluation) => (
              <li key={evaluation.caveat.id}>{evaluation.evidence}</li>
            ))}
          </ul>
        </Alert>
      )
    default: return assertNever(eligibility)
  }
}
