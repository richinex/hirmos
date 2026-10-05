import type { ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { prose, well } from '@/components/ui/recipes'
import type { SurvivalRunArtifact } from '@/domain/survival'
import { SurvivalEquation } from './SurvivalEquation'

export function SurvivalInterpretation({
  run,
  bottomLine,
  uncertainty,
  mustBeTrue,
}: {
  readonly run: SurvivalRunArtifact
  readonly bottomLine: ReactNode
  readonly uncertainty: ReactNode
  readonly mustBeTrue: ReactNode
}) {
  return (
    <section className={well('mt-4 px-3 py-3')} aria-label="Interpretation">
      <h4 className="m-0 text-label font-medium text-faint">What this result means</h4>
      <h5 className="mb-0 mt-3 text-label font-medium text-bone">Bottom line</h5>
      <p className={prose('mb-0 mt-1 text-ink')}>{bottomLine}</p>
      <h5 className="mb-0 mt-3 border-t border-hair pt-3 text-label font-medium text-bone">
        Uncertainty
      </h5>
      <p className={prose('mb-0 mt-1 text-muted')}>{uncertainty}</p>
      <h5 className="mb-0 mt-3 flex items-center gap-1.5 border-t border-hair pt-3 text-label font-medium text-bone">
        <Icon name="gavel" size={14} className="text-faint" />
        What must be true
      </h5>
      <p className={prose('mb-0 mt-1 text-faint')}>{mustBeTrue}</p>
      <SurvivalEquation run={run} />
    </section>
  )
}
