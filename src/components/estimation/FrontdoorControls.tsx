import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'frontdoor-two-stage' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
}

export function FrontdoorControls({ configuration, configure }: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Contrast">
        <div className={fieldRow.two}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Control value"
              help={ESTIMATION_PARAMETER_HELP.frontdoor.controlValue}
            />
            <NumberInput
              step="any"
              aria-label="Front-door control value"
              className={field('text', 'mt-1 w-full')}
              value={configuration.interventions[0]}
              onChange={(event) =>
                configure({
                  ...configuration,
                  interventions: [Number(event.target.value) || 0, configuration.interventions[1]],
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Treatment value"
              help={ESTIMATION_PARAMETER_HELP.frontdoor.treatmentValue}
            />
            <NumberInput
              step="any"
              aria-label="Front-door treatment value"
              className={field('text', 'mt-1 w-full')}
              value={configuration.interventions[1]}
              onChange={(event) =>
                configure({
                  ...configuration,
                  interventions: [configuration.interventions[0], Number(event.target.value) || 0],
                })
              }
            />
          </label>
        </div>
      </SettingsStep>
      <SettingsStep number={2} title="Report uncertainty">
        <div className={fieldRow.two}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Bootstrap resamples"
              help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapResamples}
            />
            <NumberInput
              min={20}
              max={5000}
              aria-label="Front-door bootstrap resamples"
              className={field('text', 'mt-1 w-full')}
              value={configuration.simulations}
              onChange={(event) =>
                configure({
                  ...configuration,
                  simulations: Math.max(
                    20,
                    Math.min(5000, Math.floor(Number(event.target.value) || 20)),
                  ),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Bootstrap seed"
              help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapSeed}
            />
            <NumberInput
              min={0}
              aria-label="Front-door bootstrap seed"
              className={field('text', 'mt-1 w-full')}
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
          The first regression estimates treatment → mediator. The second estimates mediator →
          outcome while adjusting for treatment. Their product gives the linear front-door contrast;
          the interval uses a seeded row bootstrap.
        </p>
      </SettingsStep>
    </div>
  )
}
