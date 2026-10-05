import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import type { SwigDidAssessment } from '@/domain/swigDid'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { Alert } from '@/components/ui/Alert'
export function SwigDidResult({
  assessment,
  names,
}: {
  readonly assessment: SwigDidAssessment
  readonly names: readonly string[]
}) {
  const name = (i: number) => {
    const value = names[i]
    if (value === undefined) throw Error('Unknown recorded DiD variable')
    return value
  }
  const list = (values: readonly number[]) =>
    values.length === 0 ? 'None' : values.map(name).join(', ')
  const decision = assessment.decision
  return (
    <section className="flex flex-col gap-3 text-body" aria-label="DiD adjustment assessment">
      {decision.kind === 'baselineIdentity' ? (
        <p className="m-0 text-body text-ink">
          The outcome period is the cohort’s baseline. Its difference from itself is identically
          zero.
        </p>
      ) : decision.kind === 'supportedSubjectToOverlap' ? (
        <>
          <Alert tone="info">
            The declared model supports an adjustment family, subject to overlap.
          </Alert>
          <RecordList>
            <RecordRow term="Required controls">{list(decision.required)}</RecordRow>
            <RecordRow term="Available observed controls">{list(decision.available)}</RecordRow>
            <RecordRow term="Proposed controls">
              {decision.selectedSupported
                ? 'Satisfy the subset requirements'
                : 'Do not satisfy the subset requirements'}
            </RecordRow>
          </RecordList>
        </>
      ) : (
        <>
          <Alert tone="warn">
            This adjustment rule is not established. Required potential covariates are not available
            in one or both groups.
          </Alert>
          <RecordList>
            <RecordRow term="Required potential covariates">
              {list(decision.potentialSet)}
            </RecordRow>
          </RecordList>
          <ul className="m-0 space-y-1 pl-5 text-body">
            {[
              ...decision.treatedBlockers.map((blocker) => ({ group: 'Treated group', blocker })),
              ...decision.comparisonBlockers.map((blocker) => ({
                group: 'Comparison group',
                blocker,
              })),
            ].map(({ group, blocker }, i) => (
              <li key={i}>
                {group}: {name(blocker.variable)}{' '}
                {blocker.kind === 'unmeasured'
                  ? 'is unmeasured.'
                  : blocker.kind === 'unestablishedAssignment'
                    ? `requires ${name(blocker.treatment)} = ${blocker.required}, which the declared treatment history does not establish.`
                    : `requires ${name(blocker.treatment)} = ${blocker.required}, but the declared history sets it to ${blocker.actual}.`}
              </li>
            ))}
          </ul>
        </>
      )}
      <details>
        <DisclosureSummary className="cursor-pointer text-body text-ink">
          Structural restrictions checked on the graph
        </DisclosureSummary>
        <ul className="mb-0 mt-2 space-y-1 pl-5 text-body">
          {(
            [
              [
                'No within-period treatment effect on covariates',
                assessment.noWithinPeriodTreatmentCovariateEffect,
              ],
              [
                'No treatment effect on later covariates',
                assessment.noFutureTreatmentCovariateEffect,
              ],
              [
                'No direct covariate effect on later outcomes',
                assessment.noDirectCovariateOutcomeDynamics,
              ],
              [
                'No within-period covariate effect on outcomes',
                assessment.noWithinPeriodCovariateOutcomeEffect,
              ],
            ] as const
          ).map(([label, holds]) => (
            <li key={label}>
              {label}: {holds ? 'satisfied' : 'not satisfied'}.
            </li>
          ))}
        </ul>
      </details>
      <p className="m-0 text-label text-faint">
        This is a sufficient rule for the declared model class, not a general identification
        algorithm, so a refusal does not rule out other strategies. Consistency, the declared
        mechanisms and overlap are not tested.
      </p>
    </section>
  )
}
