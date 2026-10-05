import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { field, fieldHint, fieldLabel, fieldRow, settingsStack } from '@/components/ui/recipes'
import type { StructuralModel } from '@/domain/structuralImpact'

export function StructuralImpactControls({
  model,
  onChange,
}: {
  readonly model: StructuralModel
  readonly onChange: (model: StructuralModel) => void
}) {
  const season = model.seasonality
  return (
    <div className={settingsStack}>
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Trend"
          help="Local level lets the baseline drift. Local linear also lets the slope drift. Semilocal lets the slope revert toward an estimated long-run drift. Each is a modelling assumption."
        />
        <SegmentedControl
          className="mt-1 max-w-2xl"
          fill
          ariaLabel="Structural trend"
          value={model.trend}
          options={[
            { value: 'level', label: 'Local level' },
            { value: 'linear', label: 'Local linear' },
            { value: 'semilocal', label: 'Semilocal' },
          ]}
          onChange={(trend) => onChange({ ...model, trend })}
        />
      </div>
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Seasonality"
          help="Seasonal states represent positions in a repeated cycle. Harmonics represent smooth repeated cycles. Both can evolve over time. Periods are measured in observations, not automatically inferred calendar units."
        />
        <SegmentedControl
          className="mt-1 max-w-2xl"
          fill
          ariaLabel="Structural seasonality"
          value={season.kind}
          options={[
            { value: 'none', label: 'None' },
            { value: 'seasonal', label: 'Seasonal states' },
            { value: 'harmonic', label: 'Harmonics' },
          ]}
          onChange={(kind) =>
            onChange({
              ...model,
              seasonality:
                kind === 'none'
                  ? { kind }
                  : kind === 'seasonal'
                    ? { kind, seasons: 12, duration: 1 }
                    : { kind, period: 12, pairs: 2 },
            })
          }
        />
      </div>
      {season.kind === 'seasonal' && (
        <div className={fieldRow.two}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Seasons"
              help="Number of positions in a cycle. For monthly observations, 12 represents an annual cycle."
            />
            <input
              aria-label="Structural seasons"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={2}
              step={1}
              value={season.seasons}
              onChange={(e) =>
                onChange({ ...model, seasonality: { ...season, seasons: Number(e.target.value) } })
              }
            />
          </label>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Season duration"
              help="Number of observations spent at each seasonal position."
            />
            <input
              aria-label="Season duration"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={1}
              step={1}
              value={season.duration}
              onChange={(e) =>
                onChange({ ...model, seasonality: { ...season, duration: Number(e.target.value) } })
              }
            />
          </label>
        </div>
      )}
      {season.kind === 'harmonic' && (
        <div className={fieldRow.two}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Period"
              help="Enter the number of observations in one cycle. It can be non-integer, for example an approximate annual cycle in weekly data."
            />
            <input
              aria-label="Structural period"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={0}
              step="any"
              value={season.period}
              onChange={(e) =>
                onChange({ ...model, seasonality: { ...season, period: Number(e.target.value) } })
              }
            />
          </label>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Harmonic pairs"
              help="Number of sine and cosine pairs. It must be below half the period."
            />
            <input
              aria-label="Structural harmonic pairs"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={1}
              step={1}
              value={season.pairs}
              onChange={(e) =>
                onChange({ ...model, seasonality: { ...season, pairs: Number(e.target.value) } })
              }
            />
          </label>
        </div>
      )}
      <p className={`${fieldHint} m-0 max-w-[65ch]`}>
        Calendar and other predictors come from the control-series selection below. Calendar event
        columns enter the same static regression. Each predictor needs variation before the
        intervention. Contributions describe the fitted model, not separately identified causal
        effects.
      </p>
    </div>
  )
}
