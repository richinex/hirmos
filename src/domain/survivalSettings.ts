import { formatSetPercent, formatSetting } from '@/lib/format/number'
import type { ColumnId } from './dataset'
import { assertNever } from './dop'
import type { ConfigurationSetting } from './estimation'
import type {
  CoxTies,
  ParametricSurvivalFamily,
  PenalizedAftFamily,
  ProportionalHazardsFamily,
} from './survival'
import type {
  CoxEntryDraft,
  CoxFrailtyDraft,
  CoxObservationDraft,
  CoxPenaltyDraft,
  CoxStandardErrorsDraft,
  CoxStrataDraft,
  CoxWeightsDraft,
  Draft,
  MultiStateDraftInput,
  RowFrequencyDraft,
  TransitionDraft,
} from './survivalDraft'
import type { ForestSettings } from './survivalRegression'

export const survivalAnalysisLabels: Readonly<Record<Draft['kind'], string>> = {
  'right-censored': 'Parametric',
  nonparametric: 'Kaplan–Meier',
  'start-stop': 'Start–stop',
  'cox-regression': 'Cox regression',
  aalen: 'Aalen regression',
  'survival-forest': 'Survival forest',
  'penalized-aft': 'Penalised AFT',
  'two-group': 'Compare groups',
  'multi-state': 'Multi-state',
}

export const survivalFamilyLabels: Readonly<Record<ParametricSurvivalFamily, string>> = {
  exponential: 'Exponential',
  weibull: 'Weibull AFT',
  weibullPh: 'Weibull PH',
  logNormal: 'Log-normal',
  gamma: 'Gamma',
  gompertz: 'Gompertz',
  logLogistic: 'Log-logistic',
  generalizedGamma: 'Generalized gamma',
  generalizedF: 'Generalized F',
}

export const penalizedAftFamilyLabels: Readonly<Record<PenalizedAftFamily, string>> = {
  weibull: 'Weibull AFT',
  logLogistic: 'Log-logistic AFT',
}

export const rowFrequencyLabels: Readonly<Record<RowFrequencyDraft['kind'], string>> = {
  'one-observation-per-row': 'One observation',
  'frequency-column': 'Grouped count',
}

export const nelsonAalenTiesLabels: Readonly<Record<'discrete' | 'smoothed', string>> = {
  discrete: 'Discrete',
  smoothed: 'Smoothed',
}

export const forestSplitRuleLabels: Readonly<Record<ForestSettings['splitRule'], string>> = {
  logRank: 'Log-rank',
  extraTrees: 'Extra trees',
}

export const forestSettingLabels: Readonly<
  Record<Exclude<keyof ForestSettings, 'splitRule'>, string>
> = {
  trees: 'Trees',
  mtry: 'Candidate covariates per split',
  seed: 'Random seed',
  minNodeSize: 'Minimum node size',
  minBucket: 'Minimum terminal size',
}

export const coxObservationLabels: Readonly<Record<CoxObservationDraft['kind'], string>> = {
  'right-censored': 'Right-censored',
  'start-stop': 'Start–stop',
}

export const coxFrailtyLabels: Readonly<Record<CoxFrailtyDraft['kind'], string>> = {
  none: 'None',
  gamma: 'Gamma by group',
}

export const coxTiesLabels: Readonly<Record<CoxTies, string>> = {
  efron: 'Efron',
  breslow: 'Breslow',
}

export const coxEntryLabels: Readonly<Record<CoxEntryDraft['kind'], string>> = {
  'not-used': 'Not used',
  column: 'Entry column',
}

/** Clustered-Breslow is shown as clustered errors with Breslow ties, as the page's two controls show it. */
export const coxStandardErrorLabels: Readonly<
  Record<Exclude<CoxStandardErrorsDraft['kind'], 'clustered-breslow'>, string>
> = {
  'model-based': 'Model-based',
  robust: 'Robust',
  clustered: 'Clustered',
}

export const coxWeightLabels: Readonly<Record<CoxWeightsDraft['kind'], string>> = {
  equal: 'Equal',
  column: 'Weight column',
}

export const coxStrataLabels: Readonly<Record<CoxStrataDraft['kind'], string>> = {
  unstratified: 'Unstratified',
  column: 'Stratum column',
}

export const coxPenaltyLabels: Readonly<Record<CoxPenaltyDraft['kind'], string>> = {
  unpenalized: 'Unpenalized',
  'elastic-net': 'Elastic net',
}

export const multiStateInputLabels: Readonly<Record<MultiStateDraftInput['kind'], string>> = {
  'prepared-rows': 'Prepared rows',
  'longitudinal-states': 'State observations',
  'wide-events': 'Wide event history',
}

export const wideEntryLabels: Readonly<
  Record<Extract<MultiStateDraftInput, { readonly kind: 'wide-events' }>['entry']['kind'], string>
> = {
  shared: 'Same entry',
  columns: 'Entry columns',
}

const setting = (key: string, label: string, value: string): ConfigurationSetting => ({
  key,
  label,
  value,
})

const number = (value: number): string => formatSetting(value).text

const percent = (proportion: number): string => formatSetPercent(proportion).text

const transitions = (list: readonly TransitionDraft[]): string =>
  list.length === 0 ? 'None' : list.map(({ from, to }) => `${from} → ${to}`).join(', ')

/** A survival draft in the words the Survival page uses, for proposals and records. */
export function survivalSettings(
  draft: Draft,
  columns: readonly { readonly id: ColumnId; readonly name: string }[],
): readonly ConfigurationSetting[] {
  const column = (id: ColumnId | null): string =>
    id === null ? 'Not selected' : (columns.find((entry) => entry.id === id)?.name ?? id)
  const names = (ids: readonly ColumnId[]): string =>
    ids.length === 0 ? 'None' : ids.map(column).join(', ')
  const analysis = setting('kind', 'Analysis type', survivalAnalysisLabels[draft.kind])
  const rows = (frequency: RowFrequencyDraft): readonly ConfigurationSetting[] => [
    setting('rowFrequency', 'Rows represent', rowFrequencyLabels[frequency.kind]),
    ...(frequency.kind === 'frequency-column'
      ? [setting('rowFrequency.column', 'Frequency', column(frequency.column))]
      : []),
  ]
  const family = (value: ParametricSurvivalFamily | ProportionalHazardsFamily) =>
    setting('family', 'Distribution', survivalFamilyLabels[value])
  const horizon = (value: number) => setting('horizon', 'Prediction horizon', number(value))
  const covariates = (ids: readonly ColumnId[]) => setting('covariates', 'Covariates', names(ids))
  switch (draft.kind) {
    case 'right-censored':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        ...rows(draft.rowFrequency),
        family(draft.family),
        horizon(draft.horizon),
        covariates(draft.covariates),
      ]
    case 'nonparametric':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        ...rows(draft.rowFrequency),
        setting('ties', 'Tied events', nelsonAalenTiesLabels[draft.ties]),
        horizon(draft.horizon),
      ]
    case 'start-stop':
      return [
        analysis,
        setting('start', 'Start time', column(draft.start)),
        setting('stop', 'Stop time', column(draft.stop)),
        setting('event', 'Event', column(draft.event)),
        ...rows(draft.rowFrequency),
        family(draft.family),
        horizon(draft.horizon),
        covariates(draft.covariates),
      ]
    case 'cox-regression':
      return [analysis, ...coxSettings(draft, column), covariates(draft.covariates)]
    case 'aalen':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        covariates(draft.covariates),
      ]
    case 'survival-forest':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        covariates(draft.covariates),
        setting('categorical', 'Categorical covariates', names(draft.categorical)),
        setting(
          'settings.splitRule',
          'Split rule',
          forestSplitRuleLabels[draft.settings.splitRule],
        ),
        ...(Object.keys(forestSettingLabels) as (keyof typeof forestSettingLabels)[]).map((key) =>
          setting(`settings.${key}`, forestSettingLabels[key], number(draft.settings[key])),
        ),
        setting('predictionRow', 'Prediction row', number(draft.predictionRow + 1)),
      ]
    case 'penalized-aft':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        setting('family', 'Distribution', penalizedAftFamilyLabels[draft.family]),
        setting('penalizer', 'Penalty', number(draft.penalizer)),
        setting('confidenceLevel', 'Confidence level', percent(draft.confidenceLevel)),
        horizon(draft.horizon),
        covariates(draft.covariates),
      ]
    case 'two-group':
      return [
        analysis,
        setting('duration', 'Duration', column(draft.duration)),
        setting('event', 'Event', column(draft.event)),
        setting('group', 'Group', column(draft.group)),
        setting('truncationTime', 'Compare through time', number(draft.truncationTime)),
      ]
    case 'multi-state':
      return [
        analysis,
        ...multiStateSettings(draft.input, column),
        family(draft.family),
        horizon(draft.horizon),
      ]
    default:
      return assertNever(draft)
  }
}

function coxSettings(
  draft: Extract<Draft, { readonly kind: 'cox-regression' }>,
  column: (id: ColumnId | null) => string,
): readonly ConfigurationSetting[] {
  const { observation, weights, strata, penalty } = draft
  const shared = [
    setting('weights', 'Observation weights', coxWeightLabels[weights.kind]),
    ...(weights.kind === 'column'
      ? [setting('weights.column', 'Weight', column(weights.column))]
      : []),
    setting('strata', 'Strata', coxStrataLabels[strata.kind]),
    ...(strata.kind === 'column'
      ? [setting('strata.column', 'Stratum', column(strata.column))]
      : []),
    setting('confidenceLevel', 'Confidence level', percent(draft.confidenceLevel)),
  ]
  const penaltySettings = [
    setting('penalty', 'Penalty', coxPenaltyLabels[penalty.kind]),
    ...(penalty.kind === 'elastic-net'
      ? [
          setting('penalty.strength', 'Penalty strength', number(penalty.strength)),
          setting('penalty.l1Ratio', 'L1 ratio', number(penalty.l1Ratio)),
        ]
      : []),
  ]
  const kind = setting(
    'observation',
    'Observation structure',
    coxObservationLabels[observation.kind],
  )
  switch (observation.kind) {
    case 'start-stop':
      return [
        kind,
        setting('observation.subject', 'Subject', column(observation.subject)),
        setting('observation.start', 'Start time', column(observation.start)),
        setting('observation.stop', 'Stop time', column(observation.stop)),
        setting('observation.event', 'Event', column(observation.event)),
        ...penaltySettings,
        ...shared,
      ]
    case 'right-censored': {
      const { frailty, entry, standardErrors } = observation
      return [
        kind,
        setting('observation.duration', 'Duration', column(observation.duration)),
        setting('observation.event', 'Event', column(observation.event)),
        setting('observation.frailty', 'Shared frailty', coxFrailtyLabels[frailty.kind]),
        ...(frailty.kind === 'gamma'
          ? [
              setting('observation.frailty.column', 'Frailty group', column(frailty.column)),
              setting('observation.frailty.ties', 'Tied event times', coxTiesLabels[frailty.ties]),
            ]
          : [
              setting('observation.entry', 'Delayed entry', coxEntryLabels[entry.kind]),
              ...(entry.kind === 'column'
                ? [setting('observation.entry.column', 'Entry time', column(entry.column))]
                : []),
              ...standardErrorSettings(standardErrors, column),
              ...penaltySettings,
            ]),
        ...shared,
      ]
    }
    default:
      return assertNever(observation)
  }
}

function standardErrorSettings(
  errors: CoxStandardErrorsDraft,
  column: (id: ColumnId | null) => string,
): readonly ConfigurationSetting[] {
  switch (errors.kind) {
    case 'model-based':
    case 'robust':
      return [
        setting(
          'observation.standardErrors',
          'Standard errors',
          coxStandardErrorLabels[errors.kind],
        ),
      ]
    case 'clustered':
    case 'clustered-breslow':
      return [
        setting('observation.standardErrors', 'Standard errors', coxStandardErrorLabels.clustered),
        setting('observation.standardErrors.column', 'Cluster', column(errors.column)),
        setting(
          'observation.standardErrors.ties',
          'Tied event times',
          coxTiesLabels[errors.kind === 'clustered' ? 'efron' : 'breslow'],
        ),
      ]
    default:
      return assertNever(errors)
  }
}

function multiStateSettings(
  input: MultiStateDraftInput,
  column: (id: ColumnId | null) => string,
): readonly ConfigurationSetting[] {
  const kind = setting('input', 'Input rows', multiStateInputLabels[input.kind])
  switch (input.kind) {
    case 'prepared-rows':
      return [
        kind,
        setting('input.start', 'Start time', column(input.start)),
        setting('input.stop', 'Stop time', column(input.stop)),
        setting('input.event', 'Event', column(input.event)),
        setting('input.from', 'Origin state', column(input.from)),
        setting('input.to', 'Destination state', column(input.to)),
      ]
    case 'longitudinal-states':
      return [
        kind,
        setting('input.subject', 'Subject', column(input.subject)),
        setting('input.time', 'Observation time', column(input.time)),
        setting('input.state', 'Observed state', column(input.state)),
        setting('input.stateCount', 'Number of states', number(input.stateCount)),
        setting('input.transitions', 'Allowed state changes', transitions(input.transitions)),
      ]
    case 'wide-events': {
      const { entry } = input
      return [
        kind,
        setting('input.states', 'Number of states', number(input.states.length)),
        ...input.states.flatMap((state, i) =>
          state.kind === 'not-applicable'
            ? [
                setting(
                  `input.states.${i}`,
                  `State ${i + 1}`,
                  'No time/status fields for this state',
                ),
              ]
            : [
                setting(
                  `input.states.${i}.time`,
                  `State ${i + 1}, time reached or last followed`,
                  column(state.time),
                ),
                setting(
                  `input.states.${i}.status`,
                  `State ${i + 1}, reached`,
                  column(state.status),
                ),
              ],
        ),
        setting('input.transitions', 'Allowed state changes', transitions(input.transitions)),
        setting('input.entry', 'Entry for each subject', wideEntryLabels[entry.kind]),
        ...(entry.kind === 'shared'
          ? [
              setting('input.entry.state', 'Entry state', number(entry.state)),
              setting('input.entry.time', 'Entry time', number(entry.time)),
            ]
          : [
              setting('input.entry.state', 'Entry state', column(entry.state)),
              setting('input.entry.time', 'Entry time', column(entry.time)),
            ]),
      ]
    }
    default:
      return assertNever(input)
  }
}
