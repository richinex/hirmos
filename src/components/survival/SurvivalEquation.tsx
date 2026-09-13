import { Formula } from '@/components/ui/Formula'
import { survivalEquations, survivalEquationVariables } from '@/domain/survivalEquations'
import type { SurvivalRunArtifact } from '@/domain/survival'

export function SurvivalEquation({ run }: { readonly run: SurvivalRunArtifact }) {
  const equations = survivalEquations(run)
  const variables = survivalEquationVariables(run)
  const explanations = equations.definitions.filter((definition) => typeof definition === 'string')
  const details = equations.definitions.filter((definition) => typeof definition !== 'string')
  // Reserve space for italic glyph overhang at a formula's right edge.
  return <details className="@container mt-4 min-w-0 border-t border-hair pt-3 [&_.formula]:pr-2" data-testid="survival-equation">
    <summary className="cursor-pointer text-label font-medium text-ink">Model equation</summary>
    <div className="equation-columns mt-5" data-paired={equations.fitted.length > 0} data-testid="survival-equation-layout">
      <div className="equation-group" data-testid="survival-equation-formulas">
        <h5 className="m-0 text-label font-medium text-ink">General model</h5>
        <div className="space-y-4 py-2">
          {equations.general.map((equation, i) => <Formula key={`general-${i}`} {...equation} />)}
        </div>
        <div className="max-w-prose space-y-3 text-body text-muted" data-testid="survival-equation-definitions">
          <p className="m-0">t is follow-up time. S(t) is the probability of remaining event-free beyond t.</p>
          {explanations.map((definition, index) => <p key={index} className="m-0">{definition}</p>)}
        </div>
      </div>
      {equations.fitted.length > 0 && <div className="equation-group" data-testid="survival-equation-fitted">
        <h5 className="m-0 text-label font-medium text-ink">Fitted model</h5>
        <div className="space-y-4 py-2">
        {equations.fitted.map((equation, i) => <Formula key={`fitted-${i}`} {...equation} />)}
        </div>
        <div className="space-y-3"><p className="m-0 text-label text-faint">Values are rounded for display.</p>
        {variables.length > 0 && <div className="space-y-3 border-t border-hair pt-3 text-body text-muted">
          <h6 className="m-0 text-label font-medium text-ink">Variables in this model</h6>
          <dl className="m-0 grid grid-cols-[max-content_minmax(0,1fr)] gap-x-4 gap-y-2">
            {variables.map((name, i) => <Variable key={i} index={i + 1} name={name} />)}
          </dl>
          <p className="m-0">These are the numeric values in the prepared data, including any transformations or indicator coding.</p>
        </div>}</div>
      </div>}
    </div>
    {details.map((detail, index) => <details key={index} className="mt-5 min-w-0 border-t border-hair pt-3" data-testid="survival-equation-detail">
      <summary className="cursor-pointer text-label font-medium text-muted">{detail.title}</summary>
      <div className="mt-4 grid min-w-0 items-start gap-4 @min-[44rem]:grid-cols-[2fr_3fr] @min-[44rem]:gap-6">
        <Formula tex={`\\displaystyle ${detail.formula.tex}`} plain={detail.formula.plain} />
        <p className="m-0 max-w-prose text-body text-muted">{detail.description}</p>
      </div>
    </details>)}
  </details>
}

function Variable({ index, name }: { readonly index: number; readonly name: string }) {
  return <>
    <dt className="text-ink">x<sub>{index}</sub></dt>
    <dd className="m-0 [overflow-wrap:anywhere]">{name}</dd>
  </>
}
