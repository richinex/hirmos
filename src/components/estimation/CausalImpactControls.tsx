import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { RadioList } from '@/components/ui/RadioList'
import { field, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import type { PhysicalColumnProfile } from '@/domain/dataset'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { type StudySpecification } from '@/domain/study'
import { type ReactNode } from 'react'
import { CausalImpactInference } from './CausalImpactInference'

type Configuration = Extract<EstimatorConfiguration, { kind: 'causal-impact' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly prepared: PreparedDatasetArtifact
  readonly study: StudySpecification | null
  readonly controlCandidates: readonly PhysicalColumnProfile[]
}

export function CausalImpactControls({
  configuration,
  configure,
  prepared,
  study,
  controlCandidates,
}: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Choose the model">
        <CausalImpactInference configuration={configuration} onChange={configure} />
      </SettingsStep>
      <SettingsStep number={2} title="Mark the intervention">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Intervention start"
            help={ESTIMATION_PARAMETER_HELP.causalImpact.interventionStart}
          />
          <div className="mt-1 flex flex-wrap items-center gap-2">
            <RadioList
              legend="Intervention start"
              legendHidden
              value={configuration.start.kind}
              onChange={(kind) =>
                configure({
                  ...configuration,
                  start:
                    kind === 'from-treatment'
                      ? { kind: 'from-treatment' }
                      : {
                          kind: 'row',
                          row: Math.max(9, Math.floor(prepared.observations / 2)),
                        },
                })
              }
              options={[
                {
                  value: 'from-treatment',
                  label: `Where ${study?.treatment.name ?? 'the treatment'} turns on`,
                },
                { value: 'row', label: 'At a row' },
              ]}
            />
            {configuration.start.kind === 'row' && (
              <label className="text-body text-ink">
                First post-intervention row
                <NumberInput
                  min={9}
                  max={prepared.observations}
                  aria-label="First post-intervention row"
                  className={field('text', 'ml-2 w-28')}
                  value={configuration.start.row}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      start: {
                        kind: 'row',
                        row: Math.max(
                          9,
                          Math.min(prepared.observations, Number(event.target.value) || 9),
                        ),
                      },
                    })
                  }
                />
              </label>
            )}
          </div>
        </div>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Evaluated window"
            help={ESTIMATION_PARAMETER_HELP.causalImpact.evaluationWindow}
          />
          <div className="mt-1 flex flex-wrap items-center gap-2">
            <RadioList
              legend="Evaluated window"
              legendHidden
              value={configuration.window.kind}
              onChange={(kind) =>
                configure({
                  ...configuration,
                  window:
                    kind === 'through-last-row'
                      ? { kind: 'through-last-row' }
                      : { kind: 'to-row', row: prepared.observations },
                })
              }
              options={[
                { value: 'through-last-row', label: 'Through the last row' },
                { value: 'to-row', label: 'To a row' },
              ]}
            />
            {configuration.window.kind === 'to-row' && (
              <label className="text-body text-ink">
                Last evaluated row
                <NumberInput
                  min={1}
                  max={prepared.observations}
                  aria-label="Last evaluated row"
                  className={field('text', 'ml-2 w-28')}
                  value={configuration.window.row}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      window: {
                        kind: 'to-row',
                        row: Math.max(
                          1,
                          Math.min(prepared.observations, Number(event.target.value) || 1),
                        ),
                      },
                    })
                  }
                />
              </label>
            )}
          </div>
        </div>
      </SettingsStep>
      <SettingsStep number={3} title="Choose control series">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Control series"
            help={ESTIMATION_PARAMETER_HELP.causalImpact.controlSeries}
          />
          <div className="mt-1 flex flex-wrap gap-2" role="group" aria-label="Control series">
            {controlCandidates.length === 0 && (
              <span className="text-body text-faint">
                No other prepared columns to use as controls.
              </span>
            )}
            {controlCandidates.map((column) => (
              <label key={column.id} className="flex items-center gap-1.5 text-body text-ink">
                <input
                  type="checkbox"
                  checked={configuration.controls.includes(column.id)}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      controls: event.target.checked
                        ? [...configuration.controls, column.id]
                        : configuration.controls.filter((candidate) => candidate !== column.id),
                    })
                  }
                />
                {column.name}
              </label>
            ))}
          </div>
        </div>
      </SettingsStep>
    </div>
  )
}
