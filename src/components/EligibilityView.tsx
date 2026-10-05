import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { useOpenStationarityTests } from '@/components/data/useOpenStationarityTests'
import { assertNever } from '@/domain/dop'
import { stageGroups, type MethodEligibility } from '@/domain/methods'
import type { LevelIssueAction } from '@/domain/stationarityAssessment'

const actionLabel = (action: LevelIssueAction): string => {
  switch (action) {
    case 'run-stationarity-tests':
      return 'Run stationarity tests'
    case 'test-first-difference':
      return 'Test the first difference'
    default:
      return assertNever(action)
  }
}

/** Concise pre-run status. Detailed conditions and evidence are centralised in MethodCaveats. */
export function EligibilityView({
  eligibility,
  subject = 'this prepared dataset',
}: {
  readonly eligibility: MethodEligibility
  /** What the refusal is about, for the headline: "this prepared dataset", "this study". */
  readonly subject?: string
}) {
  const openStationarityTests = useOpenStationarityTests()
  switch (eligibility.kind) {
    case 'eligible':
      return (
        <Alert tone="ok" live={false} className="mt-4">
          <p className="m-0">Available; all pre-run checks completed</p>
        </Alert>
      )
    case 'caution': {
      // One line per group of series that share a reason; the series are listed in the requirements panel,
      // and the one action is the test that is missing.
      const groups = eligibility.unresolved.flatMap(stageGroups)
      const action =
        groups.map((group) => group.action).find((candidate) => candidate !== null) ?? null
      return (
        <Alert tone="warn" live={false} className="mt-4" testId="eligibility-notice">
          <p className="m-0">Review required; the estimator remains runnable</p>
          {groups.map((group) => (
            <p key={group.summary} className="mb-0 mt-1 text-muted">
              {group.summary}
            </p>
          ))}
          {action !== null && openStationarityTests !== null && (
            <button
              type="button"
              className="mt-2 inline-flex items-center gap-1 text-body font-medium text-link"
              onClick={openStationarityTests}
            >
              {actionLabel(action)}
              <Icon name="arrow_forward" size={16} />
            </button>
          )}
        </Alert>
      )
    }
    case 'refused':
      return (
        <Alert tone="danger" className="mt-4">
          <p className="m-0">Requirements not met for {subject}</p>
          <ul className="mb-0 mt-2 space-y-2 pl-4 text-muted">
            {eligibility.violations.map((evaluation) => (
              <li key={evaluation.caveat.id}>{evaluation.evidence}</li>
            ))}
          </ul>
        </Alert>
      )
    default:
      return assertNever(eligibility)
  }
}
