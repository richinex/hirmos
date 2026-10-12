import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, fieldRow } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'negbin-nuts' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
}

export function NegbinNutsControls({ configuration, configure }: Props): ReactNode {
  return (
    <SettingsStep title="Sampler">
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
            className={field('text', 'mt-1 w-full')}
            value={configuration.warmup}
            onChange={(event) =>
              configure({
                ...configuration,
                warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))),
              })
            }
          />
        </label>
        <label className="block">
          <ParameterLabel
            className={fieldLabel}
            label="Draws"
            help={ESTIMATION_PARAMETER_HELP.nuts.draws}
          />
          <NumberInput
            min={10}
            max={5000}
            aria-label="Draws"
            className={field('text', 'mt-1 w-full')}
            value={configuration.samples}
            onChange={(event) =>
              configure({
                ...configuration,
                samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))),
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
        Gamma-Poisson likelihood, standardised treatment and confounder, NUTS with step-size and
        diagonal mass adaptation at target acceptance 0.8.
      </p>
    </SettingsStep>
  )
}
