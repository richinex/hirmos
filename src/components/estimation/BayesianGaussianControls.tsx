import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'bayesian-gaussian' }>

type Props = {
  readonly covariatesStep: (number: number) => ReactNode
  readonly categoricalChecklist: ReactNode
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
}

export function BayesianGaussianControls({
  covariatesStep,
  categoricalChecklist,
  configuration,
  configure,
}: Props): ReactNode {
  return (
    <div className={stepsStack}>
      {covariatesStep(1)}
      <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Sampler">
        <div className={fieldRow.three}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Warmup"
              help={ESTIMATION_PARAMETER_HELP.nuts.warmup}
            />
            <NumberInput
              min={10}
              max={5000}
              aria-label="Warmup"
              className={field('text', 'mt-1')}
              value={configuration.warmup}
              onChange={(event) =>
                configure({
                  ...configuration,
                  warmup: Math.max(
                    10,
                    Math.min(5000, Math.floor(Number(event.target.value) || 10)),
                  ),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Draws per chain"
              help={ESTIMATION_PARAMETER_HELP.nuts.draws}
            />
            <NumberInput
              min={10}
              max={5000}
              aria-label="Draws per chain"
              className={field('text', 'mt-1')}
              value={configuration.samples}
              onChange={(event) =>
                configure({
                  ...configuration,
                  samples: Math.max(
                    10,
                    Math.min(5000, Math.floor(Number(event.target.value) || 10)),
                  ),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Seed"
              help={ESTIMATION_PARAMETER_HELP.nuts.seed}
            />
            <NumberInput
              min={0}
              aria-label="Sampler seed"
              className={field('text', 'mt-1')}
              value={configuration.seed}
              onChange={(event) =>
                configure({
                  ...configuration,
                  seed: Math.max(0, Math.floor(Number(event.target.value) || 0)),
                })
              }
            />
          </label>
        </div>
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
          Normal(0, 10) intercepts, Normal(0, 1) slopes, half-normal(10) residual scale; non-binary
          adjustment columns are standardised; three NUTS chains with step-size and diagonal mass
          adaptation at target acceptance 0.8.
        </p>
      </SettingsStep>
    </div>
  )
}
