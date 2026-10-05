import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Formula } from '@/components/ui/Formula'
import { timeSeriesEquations } from '@/domain/timeSeriesEquations'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'

export function TimeSeriesEquation({
  run,
}: {
  readonly run: TimeSeriesRun | CountSeriesModelArtifact
}) {
  const equations = timeSeriesEquations(run)
  return (
    <details
      className="@container mt-4 min-w-0 border-t border-hair pt-3 [&_.formula]:pr-2"
      data-testid="time-series-equation"
    >
      <DisclosureSummary className="cursor-pointer text-label font-medium text-ink">
        Model equation
      </DisclosureSummary>
      <div className="equation-columns mt-5" data-paired="true">
        <div className="equation-group">
          <h5 className="m-0 text-label font-medium text-ink">General model</h5>
          <div className="min-w-0 space-y-3 py-2">
            {equations.general.map((formula, i) => (
              <Formula key={i} {...formula} />
            ))}
          </div>
          <div className="space-y-3">
            {equations.definitions.map((text, i) => (
              <p key={i} className="m-0 max-w-prose text-body text-muted">
                {text}
              </p>
            ))}
          </div>
        </div>
        <div className="equation-group">
          <h5 className="m-0 text-label font-medium text-ink">
            {equations.fitted.kind === 'available' ? equations.fitted.title : 'Fitted model'}
          </h5>
          <div className="min-w-0 space-y-3 py-2">
            {equations.fitted.kind === 'available' &&
              equations.fitted.expressions.map((formula, i) => <Formula key={i} {...formula} />)}
          </div>
          <div className="space-y-3">
            <p className="m-0 max-w-prose text-body text-muted">{equations.fitted.explanation}</p>
            {equations.fitted.kind === 'available' && (
              <p className="m-0 text-label text-faint">Values are rounded for display.</p>
            )}
          </div>
        </div>
      </div>
      <p className="mb-0 mt-5 text-label text-faint">{equations.reference}</p>
    </details>
  )
}
