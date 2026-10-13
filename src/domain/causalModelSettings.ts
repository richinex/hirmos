import { formatSetPercent, formatSetting } from '@/lib/format/number'
import type { AttributionDraft, CausalAnalysis, CausalInputs } from './causalModelDraft'
import type { ObservedDagNode } from './dag'
import { assertNever } from './dop'
import type { ConfigurationSetting } from './estimation'
import type { GcmEffectsRequest, GcmMechanism } from './gcmEffects'
import type { GcmInfluenceRequest } from './gcmInfluence'

type InfluenceQuery = GcmInfluenceRequest['query']
type IntrinsicQuery = Extract<InfluenceQuery, { readonly kind: 'intrinsic' }>
type ArrowsQuery = Extract<InfluenceQuery, { readonly kind: 'arrows' }>
type GroupScope = Extract<GcmEffectsRequest['grouping'], { readonly kind: 'quantiles' }>['scope']

export const causalAnalysisLabels: Readonly<Record<CausalAnalysis, string>> = {
  anomaly: 'Unusual observation',
  change: 'Distribution change',
  intervention: 'Shift intervention',
  effects: 'Intervention effects',
  intrinsic: 'Variance contributions',
  arrows: 'Arrow strengths',
}

export const conditionalModelLabels: Readonly<Record<AttributionDraft['changeFitting'], string>> = {
  halfNormalLinear: 'Linear models',
  automaticFull: 'Automatic selection',
}

export const observationInputLabels: Readonly<Record<CausalInputs['observationMode'], string>> = {
  values: 'Enter values',
  file: 'Upload file',
}

export const intrinsicSampleLabels: Readonly<
  Record<Exclude<keyof IntrinsicQuery, 'kind'>, string>
> = {
  training: 'Prediction training samples',
  randomization: 'Randomization samples',
  baseline: 'Baseline samples',
}

export const arrowSampleLabels: Readonly<Record<Exclude<keyof ArrowsQuery, 'kind'>, string>> = {
  conditional: 'Conditional samples',
  maxRuns: 'Maximum iterations',
  tolerance: 'Convergence tolerance',
}

export const effectRoleLabels: Readonly<Record<'treatment' | 'outcome', string>> = {
  treatment: 'Treatment',
  outcome: 'Outcome',
}

export const ungroupedEffectsLabel = 'All rows'

export const groupInclusionLabels: Readonly<Record<GroupScope['kind'], string>> = {
  all: 'All rows',
  lowerBound: 'Lower bound',
}

export const gcmMechanismLabels: Readonly<Record<GcmMechanism['kind'], string>> = {
  empirical: 'Observed distribution',
  regression: 'Continuous',
  classifier: 'Categorical',
}

export const effectsSamplingLabels: Readonly<
  Record<
    'trees' | 'minLeaf' | 'fitSeed' | 'simulationSeed' | 'repetitions' | 'upperQuantile',
    string
  >
> = {
  trees: 'Trees per model',
  minLeaf: 'Minimum leaf size',
  fitSeed: 'Fitting seed',
  simulationSeed: 'Simulation seed',
  repetitions: 'Simulations',
  upperQuantile: 'Upper percentile',
}

export interface RandomSequenceOption {
  readonly value: string
  readonly label: string
}

/** The random sequences an attribution can start from: the seed, or the saved state of an earlier check or analysis. */
export function randomSequenceOptions(
  checks: readonly { readonly id: string }[],
  analyses: readonly { readonly id: string }[],
): readonly RandomSequenceOption[] {
  return [
    { value: 'seed', label: 'Start from the random seed' },
    ...checks.map((record, index) => ({
      value: record.id,
      label: `Continue after model check ${index + 1}`,
    })),
    ...analyses.map((record, index) => ({
      value: record.id,
      label: `Continue after analysis ${index + 1}`,
    })),
  ]
}

export type AttributionConfiguration = AttributionDraft &
  Pick<CausalInputs, 'observationMode' | 'observationDraft' | 'shifts'>

export interface InfluenceConfiguration {
  readonly target: string
  readonly query: InfluenceQuery
  readonly seed: number
}

const setting = (key: string, label: string, value: string): ConfigurationSetting => ({
  key,
  label,
  value,
})

const number = (value: number): string => formatSetting(value).text

const percent = (proportion: number): string => formatSetPercent(proportion).text

const variable = (names: readonly string[], index: string): string =>
  (index === '' ? undefined : names[Number(index)]) ?? 'Not selected'

/** The chosen causal model analysis in the words the page's analysis tabs use. */
export function causalAnalysisSettings(value: {
  readonly analysis: CausalAnalysis
}): readonly ConfigurationSetting[] {
  return [setting('analysis', 'Analysis type', causalAnalysisLabels[value.analysis])]
}

/** An attribution draft in the words the causal model analysis page uses for the chosen analysis. */
export function attributionSettings(
  value: AttributionConfiguration,
  analysis: CausalAnalysis,
  nodes: readonly Pick<ObservedDagNode, 'id' | 'name'>[],
  sequences: readonly RandomSequenceOption[],
): readonly ConfigurationSetting[] {
  const names = nodes.map((node) => node.name)
  return [
    ...(analysis === 'change'
      ? [
          setting(
            'changeFitting',
            'Conditional models',
            conditionalModelLabels[value.changeFitting],
          ),
        ]
      : []),
    ...(analysis === 'anomaly'
      ? [
          setting(
            'observationMode',
            'Observation input',
            observationInputLabels[value.observationMode],
          ),
        ]
      : []),
    setting('target', 'Target variable', variable(names, value.target)),
    ...(analysis === 'anomaly' && value.observationMode === 'values'
      ? nodes.map((node) =>
          setting(
            `observationDraft.${node.id}`,
            `Observed ${node.name}`,
            number(Number(value.observationDraft[node.id])),
          ),
        )
      : []),
    ...(analysis === 'intervention'
      ? nodes.map((node) => {
          const shift = value.shifts[node.id]?.trim() ?? ''
          return setting(
            `shifts.${node.id}`,
            `Shift in ${node.name}`,
            shift === '' ? 'Leave unchanged' : number(Number(shift)),
          )
        })
      : []),
    setting('repetitions', 'Refitted estimates', number(value.repetitions)),
    setting('seed', 'Random seed', number(value.seed)),
    setting(
      'randomSource',
      'Analysis random sequence',
      sequences.find((sequence) => sequence.value === value.randomSource)?.label ??
        value.randomSource,
    ),
    ...(analysis !== 'intervention'
      ? [setting('samples', 'Distribution samples', number(value.samples))]
      : []),
  ]
}

/** A variance-contribution or arrow-strength draft in the words the causal influence page uses. */
export function influenceSettings(
  value: InfluenceConfiguration,
  names: readonly string[],
): readonly ConfigurationSetting[] {
  const { query } = value
  const samples = (): readonly ConfigurationSetting[] => {
    switch (query.kind) {
      case 'intrinsic':
        return (Object.keys(intrinsicSampleLabels) as (keyof typeof intrinsicSampleLabels)[]).map(
          (key) => setting(`query.${key}`, intrinsicSampleLabels[key], number(query[key])),
        )
      case 'arrows':
        return (Object.keys(arrowSampleLabels) as (keyof typeof arrowSampleLabels)[]).map((key) =>
          setting(`query.${key}`, arrowSampleLabels[key], number(query[key])),
        )
      default:
        return assertNever(query)
    }
  }
  return [
    setting('query.kind', 'Analysis type', causalAnalysisLabels[query.kind]),
    setting('target', 'Target variable', variable(names, value.target)),
    ...samples(),
    setting('seed', 'Random seed', number(value.seed)),
  ]
}

/** An intervention-effects request in the words the intervention effects page uses. */
export function effectsSettings(value: GcmEffectsRequest): readonly ConfigurationSetting[] {
  const { names, grouping } = value
  const name = (index: number): string => names[index] ?? 'Not selected'
  const groups = (): readonly ConfigurationSetting[] => {
    switch (grouping.kind) {
      case 'none':
        return [setting('grouping', 'Group effects by', ungroupedEffectsLabel)]
      case 'quantiles':
        return [
          setting('grouping', 'Group effects by', name(grouping.column)),
          setting('grouping.groups', 'Quantile groups', number(grouping.groups)),
          setting('grouping.scope', 'Group inclusion', groupInclusionLabels[grouping.scope.kind]),
          ...(grouping.scope.kind === 'lowerBound'
            ? [
                setting(
                  'grouping.scope.minimum',
                  'Minimum group value',
                  number(grouping.scope.minimum),
                ),
              ]
            : []),
        ]
      default:
        return assertNever(grouping)
    }
  }
  return [
    setting('treatment', effectRoleLabels.treatment, name(value.treatment)),
    setting('outcome', effectRoleLabels.outcome, name(value.outcome)),
    ...groups(),
    ...value.mechanisms.flatMap((mechanism, i) => [
      setting(`mechanisms.${i}`, `Model for ${name(i)}`, gcmMechanismLabels[mechanism.kind]),
      ...(mechanism.kind === 'classifier'
        ? [
            setting(
              `mechanisms.${i}.classes`,
              `Categories for ${name(i)}`,
              number(mechanism.classes),
            ),
          ]
        : []),
    ]),
    setting('trees', effectsSamplingLabels.trees, number(value.trees)),
    setting('minLeaf', effectsSamplingLabels.minLeaf, number(value.minLeaf)),
    setting('fitSeed', effectsSamplingLabels.fitSeed, number(value.fitSeed)),
    setting('simulationSeed', effectsSamplingLabels.simulationSeed, number(value.simulationSeed)),
    setting('repetitions', effectsSamplingLabels.repetitions, number(value.repetitions)),
    setting('upperQuantile', effectsSamplingLabels.upperQuantile, percent(value.upperQuantile)),
  ]
}
