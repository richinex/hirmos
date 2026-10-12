import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { boostedCandidateCount, type EstimatorConfiguration } from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'
import { BoostedSearchFields } from './BoostedSearchFields'
import { BootstrapIntervalFields } from './BootstrapIntervalFields'

type Configuration = Extract<
  EstimatorConfiguration,
  { kind: 'propensity-weighting' | 'propensity-matching' | 'doubly-robust' }
>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly categoricalChecklist: ReactNode
}

export function PropensityControls({
  configuration,
  configure,
  categoricalChecklist,
}: Props): ReactNode {
  const uncertainty =
    configuration.kind === 'propensity-matching' || configuration.model === 'boosted'
      ? null
      : configuration.uncertainty
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Fit the propensity score">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Treatment model"
            help={ESTIMATION_PARAMETER_HELP.propensity.treatmentModel}
          />
          <SegmentedControl
            className="mt-1 max-w-md"
            fill
            ariaLabel="Treatment model"
            value={configuration.model}
            options={
              configuration.kind === 'doubly-robust'
                ? [
                    { value: 'newton', label: 'Newton' },
                    { value: 'lbfgsb', label: 'L-BFGS-B' },
                  ]
                : [
                    { value: 'newton', label: 'Newton' },
                    { value: 'lbfgsb', label: 'L-BFGS-B' },
                    { value: 'boosted', label: 'Boosted' },
                  ]
            }
            onChange={(model) =>
              configure(
                configuration.kind === 'doubly-robust'
                  ? { ...configuration, model: model === 'newton' ? 'newton' : 'lbfgsb' }
                  : {
                      ...configuration,
                      model:
                        model === 'newton' || model === 'lbfgsb' || model === 'boosted'
                          ? model
                          : 'newton',
                    },
              )
            }
          />
        </div>
        {configuration.model === 'lbfgsb' && (
          <label className="block max-w-xs">
            <ParameterLabel
              className={fieldLabel}
              label="Iteration limit"
              help={ESTIMATION_PARAMETER_HELP.propensity.maxIter}
            />
            <NumberInput
              min={1}
              max={100000}
              aria-label="Iteration limit"
              className={field('text', 'mt-1 w-full')}
              value={configuration.maxIter}
              onChange={(event) =>
                configure({
                  ...configuration,
                  maxIter: Math.max(
                    1,
                    Math.min(100000, Math.floor(Number(event.target.value) || 1)),
                  ),
                })
              }
            />
          </label>
        )}
        {configuration.kind !== 'doubly-robust' && configuration.model === 'boosted' && (
          <BoostedSearchFields
            grid={configuration.boosted}
            seedHelp={ESTIMATION_PARAMETER_HELP.propensity.treeSeed}
            onChange={(grid) =>
              configure({ ...configuration, boosted: { ...configuration.boosted, ...grid } })
            }
            scoring={{
              summary: {
                icon: 'target',
                text:
                  configuration.boosted.scoring === 'cross-fitted' ? 'cross-fitted' : 'one model',
              },
              control: (
                <div>
                  <ParameterLabel
                    className={fieldLabel}
                    label="Scoring"
                    help={ESTIMATION_PARAMETER_HELP.propensity.crossFitted}
                  />
                  <SegmentedControl
                    className="mt-1"
                    fill
                    ariaLabel="Scoring"
                    value={configuration.boosted.scoring}
                    options={[
                      { value: 'one-model', label: 'One model' },
                      { value: 'cross-fitted', label: 'Cross-fitted' },
                    ]}
                    onChange={(scoring) =>
                      configure({
                        ...configuration,
                        boosted: { ...configuration.boosted, scoring },
                      })
                    }
                  />
                </div>
              ),
            }}
          />
        )}
        {categoricalChecklist}
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
          {`The score is fitted on the identified adjustment set as supplied. Declare categorical covariates before the run, and read the fitted scores with the result: extreme scores can indicate unstable weights or poor matches, so inspect treatment overlap.${configuration.model === 'boosted' ? ` The search scores ${boostedCandidateCount(configuration.boosted)} candidates by held-out ROC AUC across ${configuration.boosted.splits} folds, then refits the one it chose, and reports no bootstrap interval.` : ''}`}
        </p>
      </SettingsStep>
      {configuration.kind === 'propensity-weighting' && (
        <SettingsStep number={2} title="Weight the sample">
          <div>
            <ParameterLabel
              className={fieldLabel}
              label="Weights"
              help={ESTIMATION_PARAMETER_HELP.propensity.scale}
            />
            <SegmentedControl
              className="mt-1 max-w-md"
              fill
              ariaLabel="Weights"
              value={configuration.scale}
              options={[
                { value: 'inverseProbability', label: 'Inverse probability' },
                { value: 'stabilized', label: 'Stabilized' },
              ]}
              onChange={(scale) =>
                configure({
                  ...configuration,
                  scale: scale === 'stabilized' ? 'stabilized' : 'inverseProbability',
                })
              }
            />
          </div>
        </SettingsStep>
      )}
      {uncertainty !== null && configuration.kind !== 'propensity-matching' && (
        <SettingsStep
          number={configuration.kind === 'propensity-weighting' ? 3 : 2}
          title="Report uncertainty"
        >
          <BootstrapIntervalFields
            value={uncertainty}
            help={ESTIMATION_PARAMETER_HELP.propensity.uncertainty}
            onChange={(next) => configure({ ...configuration, uncertainty: next })}
          />
        </SettingsStep>
      )}
    </div>
  )
}
