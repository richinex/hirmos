import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldLabel, fieldRow, settingsStack } from '@/components/ui/recipes'
import type { CausalImpactConfiguration } from '@/domain/estimation'
import { StructuralImpactControls } from './StructuralImpactControls'

export function CausalImpactInference({
  configuration,
  onChange,
}: {
  readonly configuration: CausalImpactConfiguration
  readonly onChange: (configuration: CausalImpactConfiguration) => void
}) {
  const settings = configuration.inference
  return (
    <div className={settingsStack}>
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Inference"
          help="Maximum likelihood reports pointwise forecast bands. Bayesian inference samples the selected model and reports posterior effect intervals."
        />
        <SegmentedControl
          className="mt-1 max-w-2xl"
          fill
          ariaLabel="Causal impact inference"
          value={settings === undefined ? 'maximum-likelihood' : 'bayesian'}
          options={[
            { value: 'maximum-likelihood', label: 'Maximum likelihood' },
            { value: 'bayesian', label: 'Bayesian' },
          ]}
          onChange={(kind) => {
            const common = {
              kind: 'causal-impact' as const,
              start: configuration.start,
              window: configuration.window,
              controls: configuration.controls,
            }
            onChange(
              kind === 'bayesian'
                ? {
                    ...common,
                    inference: {
                      kind: 'bayesian',
                      draws: 2000,
                      warmup: 500,
                      seed: 1234,
                      priorLevelSd: 0.01,
                    },
                  }
                : { ...common, maxIter: 100 },
            )
          }}
        />
      </div>
      {settings !== undefined && (
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Bayesian model"
            help="CausalImpact uses a local level model. BSTS components supports local level, local linear trend and semilocal linear trend models. The specifications have different variance priors and prior inclusion probabilities."
          />
          <SegmentedControl
            className="mt-1 max-w-2xl"
            fill
            ariaLabel="Bayesian model"
            value={settings.kind}
            options={[
              { value: 'bayesian', label: 'CausalImpact specification' },
              { value: 'structural', label: 'BSTS components' },
            ]}
            onChange={(kind) =>
              onChange({
                ...configuration,
                inference:
                  kind === 'bayesian'
                    ? {
                        kind: 'bayesian',
                        draws: settings.draws,
                        warmup: settings.warmup,
                        seed: settings.seed,
                        priorLevelSd: 0.01,
                      }
                    : {
                        kind: 'structural',
                        draws: 4000,
                        warmup: 2000,
                        seed: settings.seed,
                        model: {
                          version: 'gaussian-components-v1',
                          trend: 'level',
                          seasonality: { kind: 'none' },
                        },
                      },
              })
            }
          />
        </div>
      )}
      {settings?.kind === 'structural' && (
        <StructuralImpactControls
          model={settings.model}
          onChange={(model) => onChange({ ...configuration, inference: { ...settings, model } })}
        />
      )}
      {settings !== undefined && (
        <div className={fieldRow.three}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Posterior draws"
              help="Draws retained after warmup. More draws can reduce Monte Carlo error; they do not correct an unsuitable model."
            />
            <input
              className={field('text', 'mt-1 w-full')}
              aria-label="Posterior draws"
              type="number"
              min={2}
              step={1}
              value={settings.draws}
              onChange={(event) =>
                onChange({
                  ...configuration,
                  inference: {
                    ...settings,
                    draws: Math.max(2, Math.trunc(Number(event.target.value) || 2)),
                  },
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Warmup iterations"
              help="Initial Gibbs iterations discarded before collecting posterior draws."
            />
            <input
              className={field('text', 'mt-1 w-full')}
              aria-label="Warmup iterations"
              type="number"
              min={0}
              step={1}
              value={settings.warmup}
              onChange={(event) =>
                onChange({
                  ...configuration,
                  inference: {
                    ...settings,
                    warmup: Math.max(0, Math.trunc(Number(event.target.value) || 0)),
                  },
                })
              }
            />
          </label>
          <label className="block">
            <span className={fieldLabel}>Seed</span>
            <input
              className={field('text', 'mt-1 w-full')}
              aria-label="Bayesian impact seed"
              type="number"
              min={0}
              max={4294967295}
              step={1}
              value={settings.seed}
              onChange={(event) =>
                onChange({
                  ...configuration,
                  inference: {
                    ...settings,
                    seed: Math.max(
                      0,
                      Math.min(4294967295, Math.trunc(Number(event.target.value) || 0)),
                    ),
                  },
                })
              }
            />
          </label>
          {settings.kind === 'bayesian' && (
            <label className="block">
              <ParameterLabel
                className={fieldLabel}
                label="Prior level scale"
                help="Prior scale of changes in the latent level, relative to the outcome's pre-intervention standard deviation. This is a modelling choice, not a significance threshold."
              />
              <input
                className={field('text', 'mt-1 w-full')}
                aria-label="Prior level scale"
                type="number"
                min={0.000001}
                step="any"
                value={settings.priorLevelSd}
                onChange={(event) =>
                  onChange({
                    ...configuration,
                    inference: {
                      ...settings,
                      priorLevelSd: Math.max(0.000001, Number(event.target.value) || 0.01),
                    },
                  })
                }
              />
            </label>
          )}
        </div>
      )}
    </div>
  )
}
