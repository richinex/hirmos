import type { SwigDidDesign } from '@/domain/swigDid'
import { fieldHint } from '@/components/ui/recipes'
export function SwigDidSpecification({
  design,
  names,
}: {
  readonly design: SwigDidDesign
  readonly names: readonly string[]
}) {
  return (
    <div className="space-y-2 text-body">
      <p>
        Adoption period: {design.adoption}. Outcome period: {design.outcome}. Comparison group:{' '}
        {design.comparison.kind === 'neverTreated'
          ? 'never treated'
          : `not treated through period ${design.comparison.through}`}
        .
      </p>
      <p>
        Proposed controls:{' '}
        {design.selected.length ? design.selected.map((i) => names[i]).join(', ') : 'none'}.
      </p>
      <p className={fieldHint}>
        This assessment assumes that all units are untreated in period 0 and remain treated once
        treatment begins. Observed outcomes correspond to the treatment histories actually
        experienced. Disturbance terms are mutually independent, and the shared additive
        contribution of time-invariant unobserved causes cancels when outcomes are differenced.
        These assumptions were recorded with the analysis; they were not established by the graph or
        data.
      </p>
      <dl className="grid grid-cols-[minmax(0,1fr)_minmax(0,2fr)] gap-x-3 gap-y-2">
        {design.panelRoles.map((role, i) => (
          <div key={i} className="contents">
            <dt className="break-words">{names[i]}</dt>
            <dd className="m-0 break-words">
              {role.kind === 'confounder'
                ? 'Time-invariant common cause'
                : role.kind === 'disturbance'
                  ? `Exogenous disturbance for ${names[role.of]}`
                  : `${role.kind === 'covariate' ? 'Covariate' : role.kind === 'treatment' ? 'Treatment' : 'Outcome'}, period ${role.period}`}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  )
}
