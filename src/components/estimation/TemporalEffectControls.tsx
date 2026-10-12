import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import {
  button,
  field,
  fieldHint,
  fieldLabel,
  fieldRow,
  stepsStack,
  well,
} from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { assertNever } from '@/domain/dop'
import {
  type CausalEffectsAdjustment,
  type EstimatorConfiguration,
  type TotalEffectEstimator,
} from '@/domain/estimation'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'causal-effects-total' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly setAdjustmentDraft: (draft: ExplicitAdjustmentMemberDraft) => void
  readonly temporalAdjustmentCandidates: readonly {
    readonly index: number
    readonly name: string
  }[]
  readonly adjustmentDraft: ExplicitAdjustmentMemberDraft
  readonly temporalAdjustmentMaxLag: number
}

const adjustmentFromEstimator = (estimator: TotalEffectEstimator): CausalEffectsAdjustment =>
  estimator.kind === 'wrightParents' ? { kind: 'optimal' } : estimator.adjustment

const totalEffectEstimatorFromKind = (
  kind: TotalEffectEstimator['kind'],
  current: TotalEffectEstimator,
): TotalEffectEstimator => {
  const adjustment = adjustmentFromEstimator(current)
  switch (kind) {
    case 'linear':
      return { kind: 'linear', adjustment }
    case 'knn':
      return { kind: 'knn', k: 15, adjustment }
    case 'wrightParents':
      return { kind: 'wrightParents' }
    default:
      return assertNever(kind)
  }
}

export type ExplicitAdjustmentMemberDraft =
  | { readonly kind: 'closed' }
  | { readonly kind: 'editing'; readonly variable: number | null; readonly lag: number }

export const CLOSED_ADJUSTMENT_DRAFT: ExplicitAdjustmentMemberDraft = { kind: 'closed' }

export function TemporalEffectControls({
  configuration,
  configure,
  setAdjustmentDraft,
  temporalAdjustmentCandidates,
  adjustmentDraft,
  temporalAdjustmentMaxLag,
}: Props): ReactNode {
  const bootstrap =
    configuration.uncertainty.kind === 'bootstrap' ? configuration.uncertainty : null
  const adjustedEstimator =
    configuration.estimator.kind === 'wrightParents' ? null : configuration.estimator
  const explicitAdjustment =
    adjustedEstimator?.adjustment.kind === 'explicit' ? adjustedEstimator.adjustment : null
  const setAdjustment = (adjustment: CausalEffectsAdjustment) => {
    if (adjustedEstimator === null) return
    configure({ ...configuration, estimator: { ...adjustedEstimator, adjustment } })
  }
  const setNeighbours = (k: number) => {
    if (adjustedEstimator?.kind !== 'knn') return
    configure({ ...configuration, estimator: { ...adjustedEstimator, k } })
  }
  const later = adjustedEstimator === null ? 2 : 3
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Choose the effect model">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Effect model"
            help={ESTIMATION_PARAMETER_HELP.causalEffects.effectModel}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="CausalEffects model"
            value={configuration.estimator.kind}
            onChange={(kind) =>
              configure({
                ...configuration,
                estimator: totalEffectEstimatorFromKind(kind, configuration.estimator),
              })
            }
            options={[
              { value: 'linear', label: 'Adjusted linear' },
              { value: 'knn', label: 'Adjusted k-NN' },
              { value: 'wrightParents', label: 'Wright paths' },
            ]}
          />
        </div>
        {configuration.estimator.kind === 'knn' && (
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Neighbours k"
              help={ESTIMATION_PARAMETER_HELP.causalEffects.neighbours}
            />
            <NumberInput
              min={1}
              max={100}
              className={field('text', 'mt-1 w-full')}
              value={configuration.estimator.k}
              onChange={(event) =>
                setNeighbours(Math.max(1, Math.min(100, Number(event.target.value) || 1)))
              }
            />
          </label>
        )}
        {configuration.estimator.kind === 'wrightParents' && (
          <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
            Fits each node on its time-indexed parents, then sums products of coefficients along
            directed treatment-to-outcome paths. The result separates direct and indirect path
            contributions.
          </p>
        )}
      </SettingsStep>
      {adjustedEstimator !== null && (
        <SettingsStep number={2} title="Choose the adjustment set">
          <div className="grid gap-2">
            <label className="block max-w-md">
              <ParameterLabel
                className={fieldLabel}
                label="Time-indexed adjustment set"
                help={ESTIMATION_PARAMETER_HELP.causalEffects.adjustmentSet}
              />
              <Select
                className={field('text', 'mt-1')}
                value={adjustedEstimator.adjustment.kind}
                onChange={(event) => {
                  setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
                  switch (event.target.value) {
                    case 'optimal':
                      setAdjustment({ kind: 'optimal' })
                      break
                    case 'minimizedOptimal':
                      setAdjustment({ kind: 'minimizedOptimal' })
                      break
                    case 'collidersMinimizedOptimal':
                      setAdjustment({ kind: 'collidersMinimizedOptimal' })
                      break
                    case 'explicit':
                      setAdjustment({ kind: 'explicit', nodes: [] })
                      break
                  }
                }}
              >
                <option value="optimal">Complete O-set</option>
                <option value="collidersMinimizedOptimal">Collider-minimized O-set</option>
                <option value="minimizedOptimal">Minimized O-set</option>
                <option value="explicit">User-supplied set</option>
              </Select>
            </label>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
              The generated choices are computed from the stationary graph. A user-supplied set is
              checked against every open path between treatment and outcome that is not causal
              before fitting.
            </p>
          </div>
          {explicitAdjustment !== null && (
            <div className="grid max-w-3xl gap-2">
              <div className="flex flex-wrap items-center justify-between gap-2">
                <ParameterLabel
                  className={fieldLabel}
                  label="Adjustment members"
                  help={ESTIMATION_PARAMETER_HELP.causalEffects.adjustmentMembers}
                />
                <button
                  type="button"
                  className={button('quiet')}
                  disabled={
                    temporalAdjustmentCandidates.length === 0 || adjustmentDraft.kind === 'editing'
                  }
                  onClick={() => setAdjustmentDraft({ kind: 'editing', variable: null, lag: 0 })}
                >
                  Add member
                </button>
              </div>
              {explicitAdjustment.nodes.length === 0 && (
                <p className="m-0 text-body text-faint">
                  The empty set will be tested. It is valid only when the graph has no open path
                  between treatment and outcome that is not causal.
                </p>
              )}
              {adjustmentDraft.kind === 'editing' && (
                <div
                  className={well(
                    'grid grid-cols-[minmax(0,1fr)_8rem_auto_auto] items-end gap-2 p-2',
                  )}
                >
                  <label className="block">
                    <ParameterLabel
                      className={fieldLabel}
                      label="Variable"
                      help={ESTIMATION_PARAMETER_HELP.causalEffects.variable}
                    />
                    <Select
                      className={field('text', 'mt-1')}
                      value={adjustmentDraft.variable ?? ''}
                      onChange={(event) =>
                        setAdjustmentDraft({
                          ...adjustmentDraft,
                          variable: event.target.value === '' ? null : Number(event.target.value),
                        })
                      }
                    >
                      <option value="">Choose a variable</option>
                      {temporalAdjustmentCandidates.map((candidate) => (
                        <option key={candidate.index} value={candidate.index}>
                          {candidate.name}
                        </option>
                      ))}
                    </Select>
                  </label>
                  <label className="block">
                    <ParameterLabel
                      className={fieldLabel}
                      label="Lag"
                      help={ESTIMATION_PARAMETER_HELP.causalEffects.lag}
                    />
                    <NumberInput
                      min={0}
                      max={temporalAdjustmentMaxLag}
                      className={field('text', 'mt-1')}
                      value={adjustmentDraft.lag}
                      onChange={(event) =>
                        setAdjustmentDraft({
                          ...adjustmentDraft,
                          lag: Math.max(
                            0,
                            Math.min(
                              temporalAdjustmentMaxLag,
                              Math.floor(Number(event.target.value) || 0),
                            ),
                          ),
                        })
                      }
                    />
                  </label>
                  <button
                    type="button"
                    className={button('quiet')}
                    disabled={
                      adjustmentDraft.variable === null ||
                      explicitAdjustment.nodes.some(
                        (node) =>
                          node[0] === adjustmentDraft.variable && node[1] === -adjustmentDraft.lag,
                      )
                    }
                    onClick={() => {
                      if (adjustmentDraft.variable === null) return
                      setAdjustment({
                        kind: 'explicit',
                        nodes: [
                          ...explicitAdjustment.nodes,
                          [adjustmentDraft.variable, -adjustmentDraft.lag],
                        ],
                      })
                      setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
                    }}
                  >
                    Add
                  </button>
                  <button
                    type="button"
                    className={button('quiet')}
                    onClick={() => setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)}
                  >
                    Cancel
                  </button>
                </div>
              )}
              {explicitAdjustment.nodes.map((node, index) => (
                <div
                  key={`${index}-${node[0]}-${node[1]}`}
                  className="grid grid-cols-[minmax(0,1fr)_8rem_auto] items-end gap-2"
                >
                  <label className="block">
                    <ParameterLabel
                      className={fieldLabel}
                      label="Variable"
                      help={ESTIMATION_PARAMETER_HELP.causalEffects.variable}
                    />
                    <Select
                      className={field('text', 'mt-1')}
                      value={node[0]}
                      onChange={(event) =>
                        setAdjustment({
                          kind: 'explicit',
                          nodes: explicitAdjustment.nodes.map((item, position) =>
                            position === index ? [Number(event.target.value), item[1]] : item,
                          ),
                        })
                      }
                    >
                      {temporalAdjustmentCandidates.map((candidate) => (
                        <option key={candidate.index} value={candidate.index}>
                          {candidate.name}
                        </option>
                      ))}
                    </Select>
                  </label>
                  <label className="block">
                    <ParameterLabel
                      className={fieldLabel}
                      label="Lag"
                      help={ESTIMATION_PARAMETER_HELP.causalEffects.lag}
                    />
                    <NumberInput
                      min={0}
                      max={temporalAdjustmentMaxLag}
                      className={field('text', 'mt-1')}
                      value={-node[1]}
                      onChange={(event) =>
                        setAdjustment({
                          kind: 'explicit',
                          nodes: explicitAdjustment.nodes.map((item, position) =>
                            position === index
                              ? [
                                  item[0],
                                  -Math.max(
                                    0,
                                    Math.min(
                                      temporalAdjustmentMaxLag,
                                      Math.floor(Number(event.target.value) || 0),
                                    ),
                                  ),
                                ]
                              : item,
                          ),
                        })
                      }
                    />
                  </label>
                  <button
                    type="button"
                    className={button('quiet')}
                    onClick={() =>
                      setAdjustment({
                        kind: 'explicit',
                        nodes: explicitAdjustment.nodes.filter((_, position) => position !== index),
                      })
                    }
                  >
                    Remove
                  </button>
                </div>
              ))}
            </div>
          )}
        </SettingsStep>
      )}
      <SettingsStep number={later} title="Contrast">
        <div className={fieldRow.three}>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="Treatment lag"
              help={ESTIMATION_PARAMETER_HELP.causalEffects.treatmentLag}
            />
            <NumberInput
              min={0}
              max={20}
              className={field('text', 'mt-1 w-full')}
              value={configuration.treatmentLag}
              onChange={(event) =>
                configure({
                  ...configuration,
                  treatmentLag: Math.max(0, Math.min(20, Number(event.target.value) || 0)),
                })
              }
            />
          </label>
          <label className="block">
            <ParameterLabel
              className={fieldLabel}
              label="From value"
              help={ESTIMATION_PARAMETER_HELP.causalEffects.fromValue}
            />
            <NumberInput
              step="any"
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
              label="To value"
              help={ESTIMATION_PARAMETER_HELP.causalEffects.toValue}
            />
            <NumberInput
              step="any"
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
      <SettingsStep number={later + 1} title="Report uncertainty">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Sampling uncertainty"
            help={ESTIMATION_PARAMETER_HELP.causalEffects.uncertainty}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="CausalEffects sampling uncertainty"
            value={configuration.uncertainty.kind}
            onChange={(kind) =>
              configure({
                ...configuration,
                uncertainty:
                  kind === 'none'
                    ? { kind: 'none' }
                    : {
                        kind: 'bootstrap',
                        samples: 100,
                        blockLength: { kind: 'fixed', length: 1 },
                        confidenceLevel: 0.9,
                        seed: 4,
                      },
              })
            }
            options={[
              { value: 'bootstrap', label: 'Block bootstrap' },
              { value: 'none', label: 'Point estimate' },
            ]}
          />
        </div>
        {bootstrap !== null && (
          <>
            <div className={fieldRow.three}>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Bootstrap samples"
                  help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSamples}
                />
                <NumberInput
                  min={20}
                  max={5000}
                  aria-label="CausalEffects bootstrap samples"
                  className={field('text', 'mt-1 w-full')}
                  value={bootstrap.samples}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      uncertainty: {
                        ...bootstrap,
                        samples: Math.max(
                          20,
                          Math.min(5000, Math.floor(Number(event.target.value) || 20)),
                        ),
                      },
                    })
                  }
                />
              </label>
              <div>
                <ParameterLabel
                  className={fieldLabel}
                  label="Block length"
                  help={ESTIMATION_PARAMETER_HELP.causalEffects.blockLength}
                />
                <SegmentedControl
                  className="mt-1"
                  ariaLabel="CausalEffects block length policy"
                  value={bootstrap.blockLength.kind}
                  onChange={(kind) =>
                    configure({
                      ...configuration,
                      uncertainty: {
                        ...bootstrap,
                        blockLength:
                          kind === 'fixed' ? { kind: 'fixed', length: 1 } : { kind: 'cubeRoot' },
                      },
                    })
                  }
                  options={[
                    { value: 'fixed', label: 'Fixed' },
                    { value: 'cubeRoot', label: 'Cube root' },
                  ]}
                />
              </div>
              {bootstrap.blockLength.kind === 'fixed' && (
                <label className="block">
                  <ParameterLabel
                    className={fieldLabel}
                    label="Observations per block"
                    help={ESTIMATION_PARAMETER_HELP.causalEffects.observationsPerBlock}
                  />
                  <NumberInput
                    min={1}
                    aria-label="CausalEffects observations per block"
                    className={field('text', 'mt-1 w-full')}
                    value={bootstrap.blockLength.length}
                    onChange={(event) =>
                      configure({
                        ...configuration,
                        uncertainty: {
                          ...bootstrap,
                          blockLength: {
                            kind: 'fixed',
                            length: Math.max(1, Math.floor(Number(event.target.value) || 1)),
                          },
                        },
                      })
                    }
                  />
                </label>
              )}
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Confidence level"
                  help={ESTIMATION_PARAMETER_HELP.causalEffects.confidenceLevel}
                />
                <NumberInput
                  min={50}
                  max={99.9}
                  step={0.1}
                  aria-label="CausalEffects confidence level"
                  className={field('text', 'mt-1 w-full')}
                  value={bootstrap.confidenceLevel * 100}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      uncertainty: {
                        ...bootstrap,
                        confidenceLevel: Math.max(
                          0.5,
                          Math.min(0.999, (Number(event.target.value) || 90) / 100),
                        ),
                      },
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Bootstrap seed"
                  help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSeed}
                />
                <NumberInput
                  min={0}
                  aria-label="CausalEffects bootstrap seed"
                  className={field('text', 'mt-1 w-full')}
                  value={bootstrap.seed}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      uncertainty: {
                        ...bootstrap,
                        seed: Math.max(0, Math.floor(Number(event.target.value) || 0)),
                      },
                    })
                  }
                />
              </label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
              Contiguous blocks preserve the lag alignment used by the fitted graph. Choose a block
              length that represents the series’ dependence; the cube-root option takes the cube
              root of the row count.
            </p>
          </>
        )}
      </SettingsStep>
    </div>
  )
}
