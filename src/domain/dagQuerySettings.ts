import { formatSetting } from '@/lib/format/number'
import type { ConditionalGaussianQuery } from './conditionalGaussianQuery'
import type { DagNodeId } from './dag'
import { assertNever } from './dop'
import type { ConfigurationSetting } from './estimation'
import type { NetworkQuery } from './networkQuery'

export type QueryRole = 'unused' | 'outcome' | 'observe' | 'intervene'

export const queryRoleLabels: Readonly<Record<QueryRole, string>> = {
  unused: 'Marginalise',
  outcome: 'Read outcome',
  observe: 'Observe',
  intervene: 'Intervene',
}

export type ParameterEstimation = 'mle' | 'bdeu'

export const parameterEstimationLabels: Readonly<Record<ParameterEstimation, string>> = {
  mle: 'Maximum likelihood',
  bdeu: 'BDeu',
}

export const noConditionLabel = 'No condition'

export type ConditionState =
  | { readonly kind: 'lowest' }
  | { readonly kind: 'highest' }
  | { readonly kind: 'index'; readonly state: number }

export const conditionStateLabels: Readonly<Record<ConditionState['kind'], string>> = {
  lowest: 'Lowest',
  highest: 'Highest',
  index: 'Specify state index',
}

export type GaussianVariableKind = ConditionalGaussianQuery['variables'][number]

export const gaussianVariableKindLabels: Readonly<Record<GaussianVariableKind, string>> = {
  continuous: 'Continuous',
  discrete: 'Discrete',
}

export const gaussianValueLabels: Readonly<Record<GaussianVariableKind, string>> = {
  continuous: 'Value',
  discrete: 'Category value',
}

export interface InterventionContrastConfiguration {
  readonly set: DagNodeId | null
  readonly read: DagNodeId | null
  readonly condition:
    | { readonly kind: 'none' }
    | { readonly kind: 'selected'; readonly node: DagNodeId; readonly state: ConditionState }
  readonly bins: number
  readonly equivalentSampleSize: number
}

export type NetworkQueryConfiguration = Pick<
  NetworkQuery,
  'outcomes' | 'observations' | 'interventions' | 'bins' | 'equivalentSampleSize'
>

export type ConditionalGaussianConfiguration = Pick<
  ConditionalGaussianQuery,
  'variables' | 'observations' | 'interventions'
> & { readonly outcome: number | null }

const setting = (key: string, label: string, value: string): ConfigurationSetting => ({
  key,
  label,
  value,
})

const number = (value: number): string => formatSetting(value).text

const withRole = <A extends { readonly variable: number }>(
  role: 'observe' | 'intervene',
  assignments: readonly A[],
) => assignments.map((assignment) => ({ ...assignment, role }))

/** An intervention contrast in the words the Intervene view uses, for proposals and records. */
export function interventionContrastSettings(
  configuration: InterventionContrastConfiguration,
  variables: readonly { readonly id: DagNodeId; readonly name: string }[],
  hasLatent: boolean,
): readonly ConfigurationSetting[] {
  const variable = (id: DagNodeId | null): string =>
    id === null ? 'Not selected' : (variables.find((entry) => entry.id === id)?.name ?? id)
  const { condition } = configuration
  const conditionSettings = (): readonly ConfigurationSetting[] => {
    switch (condition.kind) {
      case 'none':
        return [setting('condition', 'Condition on', noConditionLabel)]
      case 'selected':
        return [
          setting('condition', 'Condition on', variable(condition.node)),
          setting('condition.state', 'Condition state', conditionStateLabels[condition.state.kind]),
          ...(condition.state.kind === 'index'
            ? [setting('condition.state.state', 'State index', number(condition.state.state))]
            : []),
        ]
      default:
        return assertNever(condition)
    }
  }
  const identifiedRoute = hasLatent || condition.kind === 'selected'
  return [
    setting('set', 'Set', variable(configuration.set)),
    setting('read', 'Read', variable(configuration.read)),
    ...conditionSettings(),
    setting('bins', 'State budget', number(configuration.bins)),
    ...(identifiedRoute
      ? []
      : [
          setting(
            'equivalentSampleSize',
            'Equivalent sample size',
            number(configuration.equivalentSampleSize),
          ),
        ]),
  ]
}

/** A discrete distribution query in the words the Distribution view uses, for proposals and records. */
export function networkQuerySettings(
  configuration: NetworkQueryConfiguration,
  names: readonly string[],
): readonly ConfigurationSetting[] {
  const states = [
    ...withRole('observe', configuration.observations),
    ...withRole('intervene', configuration.interventions),
  ]
  const estimation: ParameterEstimation = configuration.equivalentSampleSize === 0 ? 'mle' : 'bdeu'
  return [
    ...names.flatMap((name, index) => {
      if (configuration.outcomes.includes(index))
        return [setting(`roles.${index}`, name, queryRoleLabels.outcome)]
      const entry = states.find((candidate) => candidate.variable === index)
      return entry === undefined
        ? [setting(`roles.${index}`, name, queryRoleLabels.unused)]
        : [
            setting(`roles.${index}`, name, queryRoleLabels[entry.role]),
            setting(`roles.${index}.state`, `${name}, state index`, number(entry.state)),
          ]
    }),
    setting('bins', 'State budget', number(configuration.bins)),
    setting('estimation', 'Parameter estimation', parameterEstimationLabels[estimation]),
    ...(estimation === 'bdeu'
      ? [
          setting(
            'equivalentSampleSize',
            'Equivalent sample size',
            number(configuration.equivalentSampleSize),
          ),
        ]
      : []),
  ]
}

/** A conditional-Gaussian query in the words the Distribution view uses, for proposals and records. */
export function conditionalGaussianSettings(
  configuration: ConditionalGaussianConfiguration,
  names: readonly string[],
): readonly ConfigurationSetting[] {
  const name = (index: number): string => names[index] ?? String(index)
  const kindOf = (index: number): GaussianVariableKind =>
    configuration.variables[index] ?? 'continuous'
  const parents = [
    ...withRole('observe', configuration.observations),
    ...withRole('intervene', configuration.interventions),
  ].sort((a, b) => a.variable - b.variable)
  return [
    ...names.map((variable, index) =>
      setting(
        `variables.${index}`,
        `${variable}, variable type`,
        gaussianVariableKindLabels[kindOf(index)],
      ),
    ),
    setting(
      'outcome',
      'Continuous outcome',
      configuration.outcome === null ? 'Not selected' : name(configuration.outcome),
    ),
    ...parents.flatMap((parent) => [
      setting(`parents.${parent.variable}`, name(parent.variable), queryRoleLabels[parent.role]),
      setting(
        `parents.${parent.variable}.value`,
        `${name(parent.variable)}, ${gaussianValueLabels[kindOf(parent.variable)].toLowerCase()}`,
        number(parent.value),
      ),
    ]),
  ]
}
