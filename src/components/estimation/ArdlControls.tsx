import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { PSS_CASES } from '@/domain/timeSeriesEstimatorSettings'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'ardl-pss' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
}

export function ArdlControls({ configuration, configure }: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Fit the error-correction model">
        <label className="block max-w-xs">
          <ParameterLabel
            className={fieldLabel}
            label="Maximum lag"
            help={ESTIMATION_PARAMETER_HELP.ardl.maximumLag}
          />
          <NumberInput
            min={1}
            max={24}
            aria-label="Maximum lag"
            className={field('text', 'mt-1 w-full')}
            value={configuration.maxLag}
            onChange={(event) =>
              configure({
                ...configuration,
                maxLag: Math.max(1, Math.min(24, Number(event.target.value) || 1)),
              })
            }
          />
        </label>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Deterministic terms"
            help={ESTIMATION_PARAMETER_HELP.ardl.deterministicTerms}
          />
          <SegmentedControl
            className="mt-1"
            fill
            ariaLabel="Deterministic terms"
            value={configuration.trend}
            onChange={(trend) =>
              configure({ ...configuration, trend, case: trend === 'c' ? 3 : 4 })
            }
            options={[
              { value: 'c', label: 'Constant' },
              { value: 'ct', label: 'Constant and trend' },
            ]}
          />
        </div>
      </SettingsStep>
      <SettingsStep number={2} title="Bounds test">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="PSS case"
            help={ESTIMATION_PARAMETER_HELP.ardl.pssCase}
          />
          <SegmentedControl
            className="mt-1"
            fill
            ariaLabel="PSS case"
            value={String(configuration.case)}
            onChange={(chosen) => {
              const candidate = PSS_CASES[configuration.trend].find(
                (item) => String(item) === chosen,
              )
              if (candidate !== undefined) configure({ ...configuration, case: candidate })
            }}
            options={PSS_CASES[configuration.trend].map((candidate) => ({
              value: String(candidate),
              label: `Case ${candidate}`,
            }))}
          />
        </div>
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
          AIC lag search, error-correction fit, delta-method interval on the long-run effect, bounds
          test at the recorded case.
        </p>
      </SettingsStep>
    </div>
  )
}
