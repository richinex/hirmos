import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { RadioList } from '@/components/ui/RadioList'
import { field, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { SettingsStep } from '@/components/ui/SettingsStep'
import type { DatasetProfile, PhysicalColumnProfile } from '@/domain/dataset'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { type PanelInterventionPreflight } from '@/domain/panel'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { defaultPredictorSyntheticConfiguration } from '@/domain/predictorSyntheticControl'
import { type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { ridgeConfigurationForPanel } from '@/domain/ridgeAugmented'
import { type StudySpecification } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { type ReactNode } from 'react'
import { PredictorSyntheticControls } from './PredictorSyntheticControls'
import { RidgeAugmentedControls } from './RidgeAugmentedControls'

type Configuration = Extract<EstimatorConfiguration, { kind: 'synthetic-control' }>

type Props = {
  readonly prepared: PreparedDatasetArtifact
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly panelPreflight: PanelInterventionPreflight
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly study: StudySpecification | null
  readonly controlCandidates: readonly PhysicalColumnProfile[]
}

export function OutcomeHistorySyntheticControls({
  prepared,
  configuration,
  configure,
  panelPreflight,
  source,
  profile,
  study,
  controlCandidates,
}: Props): ReactNode {
  const selector =
    prepared.kind === 'prepared-panel' ? (
      <SegmentedControl
        className="justify-self-start"
        ariaLabel="Synthetic-control specification"
        value={configuration.specification === 'ridge-augmented' ? 'ridge-augmented' : 'predictors'}
        onChange={(specification) =>
          configure(
            specification === 'ridge-augmented'
              ? ridgeConfigurationForPanel(
                  panelPreflight.kind === 'ready' ? panelPreflight.layout : null,
                )
              : defaultPredictorSyntheticConfiguration(),
          )
        }
        options={[
          { value: 'predictors', label: 'Predictor balance' },
          {
            value: 'ridge-augmented',
            label: 'Ridge augmentation',
            disabled: panelPreflight.kind === 'pending',
            title:
              panelPreflight.kind === 'pending'
                ? 'Checking the treatment pattern before preparing the comparison.'
                : undefined,
          },
        ]}
      />
    ) : null
  if (configuration.specification === 'ridge-augmented')
    return (
      <div className="grid gap-6">
        {selector}
        <RidgeAugmentedControls
          configuration={configuration}
          onChange={configure}
          source={source}
          profile={profile}
          prepared={prepared}
        />
      </div>
    )
  if (configuration.specification === 'predictors')
    return (
      <div className="grid gap-6">
        {selector}
        <PredictorSyntheticControls
          configuration={configuration}
          onChange={configure}
          source={source}
          profile={profile}
          prepared={prepared}
          treatment={study?.treatment.column ?? null}
        />
      </div>
    )
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Mark the intervention">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Intervention start"
            help={ESTIMATION_PARAMETER_HELP.syntheticControl.interventionStart}
          />
          <div className="mt-1 flex flex-wrap items-center gap-2">
            <RadioList
              legend="Synthetic intervention start"
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
                          row: Math.max(3, Math.floor(prepared.observations / 2)),
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
                  min={3}
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
                          3,
                          Math.min(prepared.observations, Number(event.target.value) || 3),
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
      <SettingsStep number={2} title="Choose donor series">
        <div className="max-w-3xl">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <ParameterLabel
              className={fieldLabel}
              label="Donor series"
              help={ESTIMATION_PARAMETER_HELP.syntheticControl.donorSeries}
            />
            {controlCandidates.length > 0 && (
              <SelectionActions
                selectLabel="Select all donor series"
                clearLabel="Clear selected donor series"
                onSelectAll={() =>
                  configure({
                    ...configuration,
                    donors: controlCandidates.map((column) => column.id),
                  })
                }
                onClear={() => configure({ ...configuration, donors: [] })}
              />
            )}
          </div>
          <div className="mt-1 flex flex-wrap gap-2" role="group" aria-label="Donor series">
            {controlCandidates.length === 0 && (
              <span className="text-body text-faint">
                No other prepared columns to use as donors.
              </span>
            )}
            {controlCandidates.map((column) => (
              <label key={column.id} className="flex items-center gap-1.5 text-body text-ink">
                <input
                  type="checkbox"
                  checked={configuration.donors.includes(column.id)}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      donors: event.target.checked
                        ? [...configuration.donors, column.id]
                        : configuration.donors.filter((candidate) => candidate !== column.id),
                    })
                  }
                />
                {column.name}
              </label>
            ))}
          </div>
        </div>
      </SettingsStep>
      <SettingsStep number={3} title="Report uncertainty">
        <div className={fieldRow.two}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Cross-fit folds"
              help={ESTIMATION_PARAMETER_HELP.syntheticControl.crossFitFolds}
            />
            <NumberInput
              min={2}
              max={20}
              aria-label="Cross-fit folds"
              className={field('text', 'mt-1 w-full')}
              value={configuration.crossFitFolds}
              onChange={(event) =>
                configure({
                  ...configuration,
                  crossFitFolds: Math.max(
                    2,
                    Math.min(20, Math.floor(Number(event.target.value) || 2)),
                  ),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Inference alpha"
              help={ESTIMATION_PARAMETER_HELP.syntheticControl.inferenceAlpha}
            />
            <NumberInput
              min={0.001}
              max={0.5}
              step={0.01}
              aria-label="Synthetic-control inference alpha"
              className={field('text', 'mt-1 w-full')}
              value={configuration.alpha}
              onChange={(event) =>
                configure({
                  ...configuration,
                  alpha: Math.max(0.001, Math.min(0.5, Number(event.target.value) || 0.05)),
                })
              }
            />
          </label>
        </div>
      </SettingsStep>
    </div>
  )
}
