import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'vecm' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly prepared: PreparedDatasetArtifact
}

const VECM_TERMS = [
  ['n', 'None'],
  ['co', 'Constant outside'],
  ['ci', 'Constant inside'],
  ['coli', 'Constant and trend'],
] as const

const TRACE_LEVELS: readonly (90 | 95 | 99)[] = [90, 95, 99]

export function VecmControls({ configuration, configure, prepared }: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Fit the model">
        <label className="block max-w-xs">
          <ParameterLabel
            className={fieldLabel}
            label="Maximum lags"
            help={ESTIMATION_PARAMETER_HELP.vecm.maximumLags}
          />
          <NumberInput
            min={1}
            max={24}
            aria-label="Maximum lags"
            className={field('text', 'mt-1 w-full')}
            value={configuration.maxLags}
            onChange={(event) =>
              configure({
                ...configuration,
                maxLags: Math.max(1, Math.min(24, Number(event.target.value) || 1)),
              })
            }
          />
        </label>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Deterministic terms"
            help={ESTIMATION_PARAMETER_HELP.vecm.deterministicTerms}
          />
          <Select
            className={field('text', 'mt-1')}
            aria-label="VECM deterministic terms"
            value={configuration.deterministic}
            onChange={(event) => {
              const term = VECM_TERMS.find(([value]) => value === event.target.value)
              if (term !== undefined) configure({ ...configuration, deterministic: term[0] })
            }}
          >
            {VECM_TERMS.map(([value, name]) => (
              <option key={value} value={value}>
                {name}
              </option>
            ))}
          </Select>
        </div>
      </SettingsStep>
      <SettingsStep number={2} title="Test for cointegration">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Trace significance"
            help={ESTIMATION_PARAMETER_HELP.vecm.traceSignificance}
          />
          <SegmentedControl
            className="mt-1"
            fill
            ariaLabel="Trace significance"
            value={String(configuration.significance)}
            onChange={(chosen) => {
              const level = TRACE_LEVELS.find((item) => String(item) === chosen)
              if (level !== undefined) configure({ ...configuration, significance: level })
            }}
            options={TRACE_LEVELS.map((level) => ({
              value: String(level),
              label: `${level}%`,
            }))}
          />
        </div>
      </SettingsStep>
      <SettingsStep number={3} title="Test for a break">
        <label className="block max-w-xs">
          <ParameterLabel
            className={fieldLabel}
            label="Chow split after row"
            help={ESTIMATION_PARAMETER_HELP.vecm.chowBreakRow}
          />
          <NumberInput
            min={4}
            max={prepared.observations - 4}
            aria-label="Chow split after row"
            placeholder="none"
            className={field('text', 'mt-1 w-full')}
            value={configuration.breakIndex ?? ''}
            onChange={(event) =>
              configure({
                ...configuration,
                breakIndex:
                  event.target.value === ''
                    ? null
                    : Math.max(
                        4,
                        Math.min(
                          prepared.observations - 4,
                          Math.floor(Number(event.target.value) || 4),
                        ),
                      ),
              })
            }
          />
        </label>
      </SettingsStep>
    </div>
  )
}
