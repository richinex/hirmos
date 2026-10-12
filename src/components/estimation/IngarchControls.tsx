import { LagListField } from '@/components/ui/LagListField'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'negative-binomial-ingarch' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
}

export function IngarchControls({ configuration, configure }: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Fit the count model">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Mean link"
            help={ESTIMATION_PARAMETER_HELP.ingarch.meanLink}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="INGARCH mean link"
            value={configuration.link}
            onChange={(link) => configure({ ...configuration, link })}
            options={[
              { value: 'identity', label: 'Additive' },
              { value: 'log', label: 'Multiplicative' },
            ]}
          />
        </div>
        <div className={fieldRow.two}>
          <LagListField
            label="Past count lags"
            help={ESTIMATION_PARAMETER_HELP.ingarch.pastCountLags}
            lags={configuration.pastObservationLags}
            onChange={(pastObservationLags) => configure({ ...configuration, pastObservationLags })}
          />
          <LagListField
            label="Past mean lags"
            help={ESTIMATION_PARAMETER_HELP.ingarch.pastMeanLags}
            lags={configuration.pastMeanLags}
            onChange={(pastMeanLags) => configure({ ...configuration, pastMeanLags })}
          />
        </div>
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
          The additive link expresses effects in expected counts and requires non-negative
          regressors. The multiplicative link expresses effects on the log expected count and
          permits signed regressors. Both use the treatment and identified same-period adjustment
          variables with the selected count and mean lags; no sampling interval is reported.
        </p>
      </SettingsStep>
      <SettingsStep number={2} title="Forecast the contrast">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Treatment schedule"
            help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentSchedule}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="INGARCH treatment schedule"
            value={configuration.schedule.kind}
            onChange={(kind) =>
              configure({
                ...configuration,
                schedule: kind === 'decaying' ? { kind: 'decaying', delta: 0.6 } : { kind },
              })
            }
            options={[
              { value: 'point', label: 'One period' },
              { value: 'persistent', label: 'Persistent' },
              { value: 'decaying', label: 'Decaying' },
            ]}
          />
        </div>
        <div className={fieldRow.three}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Forecast periods"
              help={ESTIMATION_PARAMETER_HELP.ingarch.forecastPeriods}
            />
            <NumberInput
              min={1}
              max={240}
              className={field('text', 'mt-1 w-full')}
              value={configuration.horizon}
              onChange={(event) =>
                configure({
                  ...configuration,
                  horizon: Math.max(1, Math.min(240, Math.floor(Number(event.target.value) || 1))),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Control value"
              help={ESTIMATION_PARAMETER_HELP.ingarch.controlValue}
            />
            <NumberInput
              step="any"
              className={field('text', 'mt-1 w-full')}
              value={configuration.controlValue}
              onChange={(event) =>
                configure({ ...configuration, controlValue: Number(event.target.value) || 0 })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Treatment value"
              help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentValue}
            />
            <NumberInput
              step="any"
              className={field('text', 'mt-1 w-full')}
              value={configuration.treatmentValue}
              onChange={(event) =>
                configure({
                  ...configuration,
                  treatmentValue: Number(event.target.value) || 0,
                })
              }
            />
          </label>
          {configuration.schedule.kind === 'decaying' && (
            <label className="block">
              <ParameterLabel
                className={fieldLabel}
                label="Decay δ"
                help={ESTIMATION_PARAMETER_HELP.ingarch.decay}
              />
              <NumberInput
                min={0}
                max={1}
                step={0.05}
                className={field('text', 'mt-1 w-full')}
                value={configuration.schedule.delta}
                onChange={(event) =>
                  configure({
                    ...configuration,
                    schedule: {
                      kind: 'decaying',
                      delta: Math.max(0, Math.min(1, Number(event.target.value) || 0)),
                    },
                  })
                }
              />
            </label>
          )}
        </div>
      </SettingsStep>
    </div>
  )
}
