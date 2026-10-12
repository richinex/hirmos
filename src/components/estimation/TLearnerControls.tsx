import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { assertNever } from '@/domain/dop'
import {
  boostedCandidateCount,
  DEFAULT_BOOSTED_SEARCH,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'
import { BoostedSearchFields } from './BoostedSearchFields'
import { TLearnerUncertainty } from './TLearnerUncertainty'

type Configuration = Extract<EstimatorConfiguration, { kind: 't-learner' }>

type Props = {
  readonly configuration: Configuration
  readonly covariatesStep: (number: number) => ReactNode
  readonly categoricalChecklist: ReactNode
  readonly configure: (next: Configuration) => void
}

export function TLearnerControls({
  configuration,
  covariatesStep,
  categoricalChecklist,
  configure,
}: Props): ReactNode {
  const model = configuration.model
  return (
    <div className={stepsStack}>
      {covariatesStep(1)}
      <SettingsStep
        number={categoricalChecklist === null ? undefined : 2}
        title="Fit the outcome models"
      >
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Outcome model"
            help={ESTIMATION_PARAMETER_HELP.tLearner.outcomeModel}
          />
          <SegmentedControl
            className="mt-1 max-w-md"
            fill
            ariaLabel="Outcome model"
            value={model.kind}
            options={[
              { value: 'forest', label: 'Random forest' },
              { value: 'boosted-cross-fitted', label: 'Boosted, cross-fitted' },
            ]}
            onChange={(kind) =>
              configure({
                ...configuration,
                model:
                  kind === 'boosted-cross-fitted'
                    ? { kind, grid: DEFAULT_BOOSTED_SEARCH }
                    : { kind: 'forest', seed: 7, uncertainty: { kind: 'none' } },
              })
            }
          />
        </div>
        {(() => {
          switch (model.kind) {
            case 'forest':
              return (
                <>
                  <label className="block max-w-xs">
                    <ParameterLabel
                      className={fieldLabel}
                      label="Learner seed"
                      help={ESTIMATION_PARAMETER_HELP.tLearner.learnerSeed}
                    />
                    <NumberInput
                      min={0}
                      aria-label="Learner seed"
                      className={field('text', 'mt-1')}
                      value={model.seed}
                      onChange={(event) =>
                        configure({
                          ...configuration,
                          model: {
                            ...model,
                            seed: Math.max(0, Math.floor(Number(event.target.value) || 0)),
                          },
                        })
                      }
                    />
                  </label>
                  <TLearnerUncertainty
                    value={model.uncertainty}
                    onChange={(uncertainty) =>
                      configure({ ...configuration, model: { ...model, uncertainty } })
                    }
                  />
                  <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
                    One random forest per treatment arm, with 200 trees and a minimum leaf size of
                    5. Bootstrap intervals refit both forests on resampled rows and take longer to
                    calculate.
                  </p>
                </>
              )
            case 'boosted-cross-fitted':
              return (
                <>
                  <BoostedSearchFields
                    grid={model.grid}
                    seedHelp={ESTIMATION_PARAMETER_HELP.tLearner.boostedSeed}
                    onChange={(grid) => configure({ ...configuration, model: { ...model, grid } })}
                  />
                  <p
                    className={cn(fieldHint, 'm-0 max-w-[65ch]')}
                  >{`Grid search compares ${boostedCandidateCount(model.grid)} candidates on all rows within each treatment arm before splitting the sample into two halves, stratified by outcome. Each half is predicted by models fitted on the other half. Hyperparameter selection is not repeated within the training halves. The outcome must be 0 or 1; no interval is calculated.`}</p>
                </>
              )
            default:
              return assertNever(model)
          }
        })()}
      </SettingsStep>
    </div>
  )
}
