import { isNumericDuckDbType, type ColumnId, type ColumnSelection, type NumericColumnSelection } from './dataset'
import { assertNever } from './dop'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { ParametricSurvivalFamily, ProportionalHazardsFamily, CoxTies, PenalizedAftFamily, SurvivalRunArtifact } from './survival'
import type { ForestSettings } from './survivalRegression'
import type { Workflow } from './workflow'

export type RowFrequencyDraft =
  | { readonly kind: 'one-observation-per-row' }
  | { readonly kind: 'frequency-column'; readonly column: ColumnId | null }

export type CoxEntryDraft =
  | { readonly kind: 'not-used' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

export type CoxWeightsDraft =
  | { readonly kind: 'equal' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

export type CoxStrataDraft =
  | { readonly kind: 'unstratified' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

export type CoxStandardErrorsDraft =
  | { readonly kind: 'model-based' }
  | { readonly kind: 'robust' }
  | { readonly kind: 'clustered'; readonly column: ColumnId | null }

export type CoxPenaltyDraft =
  | { readonly kind: 'unpenalized' }
  | { readonly kind: 'elastic-net'; readonly strength: number; readonly l1Ratio: number }

export type CoxFrailtyDraft =
  | { readonly kind: 'none' }
  | { readonly kind: 'gamma'; readonly column: ColumnId | null; readonly ties: CoxTies }

export type CoxObservationDraft =
  | {
      readonly kind: 'right-censored'
      readonly duration: ColumnId | null
      readonly event: ColumnId | null
      readonly entry: CoxEntryDraft
      readonly standardErrors: CoxStandardErrorsDraft
      readonly frailty: CoxFrailtyDraft
    }
  | {
      readonly kind: 'start-stop'
      readonly subject: ColumnId | null
      readonly start: ColumnId | null
      readonly stop: ColumnId | null
      readonly event: ColumnId | null
    }

export type Draft =
  | { readonly kind: 'aalen'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[] }
  | { readonly kind: 'survival-forest'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly categorical: readonly ColumnId[]; readonly settings: ForestSettings; readonly predictionRow: number }
  | { readonly kind: 'right-censored'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly covariates: readonly ColumnId[]; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'nonparametric'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly horizon: number; readonly ties: 'discrete' | 'smoothed' }
  | { readonly kind: 'start-stop'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly covariates: readonly ColumnId[]; readonly family: ProportionalHazardsFamily; readonly horizon: number }
  | { readonly kind: 'cox-regression'; readonly observation: CoxObservationDraft; readonly weights: CoxWeightsDraft; readonly strata: CoxStrataDraft; readonly covariates: readonly ColumnId[]; readonly penalty: CoxPenaltyDraft; readonly confidenceLevel: number }
  | { readonly kind: 'penalized-aft'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly family: PenalizedAftFamily; readonly penalizer: number; readonly confidenceLevel: number; readonly horizon: number }
  | { readonly kind: 'two-group'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly group: ColumnId | null; readonly truncationTime: number; readonly permutations: number; readonly seed: number }
  | { readonly kind: 'multi-state'; readonly input: MultiStateDraftInput; readonly family: ProportionalHazardsFamily; readonly horizon: number }

export interface TransitionDraft { readonly from: number; readonly to: number }
export type WideStateDraft =
  | { readonly kind: 'not-applicable' }
  | { readonly kind: 'recorded'; readonly time: ColumnId | null; readonly status: ColumnId | null }
export type MultiStateDraftInput =
  | { readonly kind: 'prepared-rows'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly from: ColumnId | null; readonly to: ColumnId | null }
  | { readonly kind: 'longitudinal-states'; readonly subject: ColumnId | null; readonly time: ColumnId | null; readonly state: ColumnId | null; readonly stateCount: number; readonly transitions: readonly TransitionDraft[] }
  | {
      readonly kind: 'wide-events'
      readonly states: readonly WideStateDraft[]
      readonly transitions: readonly TransitionDraft[]
      readonly entry: { readonly kind: 'shared'; readonly state: number; readonly time: number } | { readonly kind: 'columns'; readonly state: ColumnId | null; readonly time: ColumnId | null }
    }

export const columnLike = (columns: readonly ColumnSelection[], pattern: RegExp): ColumnId | null =>
  columns.find((column) => pattern.test(column.name))?.id ?? null

export const defaultTransitions = (): readonly TransitionDraft[] => [
  { from: 1, to: 2 },
  { from: 1, to: 3 },
  { from: 2, to: 3 },
]

export const preparedMultiStateInput = (columns: readonly NumericColumnSelection[]): Extract<MultiStateDraftInput, { readonly kind: 'prepared-rows' }> => ({
  kind: 'prepared-rows',
  start: columnLike(columns, /^(start|tstart)$/i),
  stop: columnLike(columns, /^(stop|tstop)$/i),
  event: columnLike(columns, /^(event|status)$/i),
  from: columnLike(columns, /^from$/i),
  to: columnLike(columns, /^to$/i),
})

export const draftFor = (kind: Draft['kind'], columns: readonly NumericColumnSelection[]): Draft => {
  const duration = columnLike(columns, /^(time|duration|years?|months?|recyrs)$/i)
  const event = columnLike(columns, /^(event|status|death|censrec)$/i)
  switch (kind) {
    case 'aalen': return { kind, duration, event, covariates: [] }
    case 'survival-forest': return { kind, duration, event, covariates: [], categorical: [], settings: { trees: 500, mtry: 1, seed: 43, minNodeSize: 3, minBucket: 3, splitRule: 'logRank' }, predictionRow: 0 }
    case 'right-censored': return { kind, duration, event, rowFrequency: { kind: 'one-observation-per-row' }, covariates: [], family: 'weibull', horizon: 10 }
    case 'nonparametric': return { kind, duration, event, rowFrequency: { kind: 'one-observation-per-row' }, horizon: 10, ties: 'discrete' }
    case 'start-stop': return { kind, start: columnLike(columns, /^(start|tstart)$/i), stop: columnLike(columns, /^(stop|tstop)$/i), event, rowFrequency: { kind: 'one-observation-per-row' }, covariates: [], family: 'weibullPh', horizon: 10 }
    case 'cox-regression': return {
      kind,
      observation: { kind: 'right-censored', duration, event, entry: { kind: 'not-used' }, standardErrors: { kind: 'model-based' }, frailty: { kind: 'none' } },
      weights: { kind: 'equal' },
      strata: { kind: 'unstratified' },
      covariates: [],
      penalty: { kind: 'unpenalized' },
      confidenceLevel: 0.95,
    }
    case 'penalized-aft': return { kind, duration, event, covariates: [], family: 'weibull', penalizer: 0.1, confidenceLevel: 0.95, horizon: 10 }
    case 'two-group': return { kind, duration, event, group: columnLike(columns, /^(group|arm|treatment|treat)$/i), truncationTime: 10, permutations: 1_000, seed: 43 }
    case 'multi-state': return { kind, input: preparedMultiStateInput(columns), family: 'weibullPh', horizon: 10 }
    default: return assertNever(kind)
  }
}

/**
 * The controls as the latest run set them, so a reopened project or example shows the analysis it
 * recorded rather than the defaults. A column the prepared dataset no longer has is left unchosen.
 */
export const draftFromRun = (run: SurvivalRunArtifact, columns: readonly NumericColumnSelection[], sourceColumns: readonly ColumnSelection[]): Draft => {
  const present = (column: ColumnSelection): ColumnId | null => columns.some((candidate) => candidate.id === column.id) ? column.id : null
  const presentSource = (column: ColumnSelection): ColumnId | null => sourceColumns.some((candidate) => candidate.id === column.id) ? column.id : null
  const presentAll = (selected: readonly NumericColumnSelection[]): readonly ColumnId[] => selected.map(present).filter((id): id is ColumnId => id !== null)
  const horizon = (times: readonly number[]): number => times.at(-1) ?? 10
  const configuration = run.configuration
  switch (configuration.kind) {
    case 'aalen': return { kind: 'aalen', duration: present(configuration.duration), event: present(configuration.event), covariates: presentAll(configuration.covariates) }
    case 'survival-forest': return { kind: 'survival-forest', duration: present(configuration.duration), event: present(configuration.event), covariates: presentAll(configuration.covariates), categorical: presentAll(configuration.categorical), settings: configuration.settings, predictionRow: configuration.predictionRow }
    case 'right-censored-parametric':
      return { kind: 'right-censored', duration: present(configuration.duration), event: present(configuration.event), rowFrequency: configuration.rowFrequency.kind === 'one-observation-per-row' ? configuration.rowFrequency : { kind: 'frequency-column', column: present(configuration.rowFrequency.column) }, covariates: presentAll(configuration.covariates), family: configuration.family, horizon: horizon(configuration.predictionTimes) }
    case 'right-censored-nonparametric':
      return { kind: 'nonparametric', duration: present(configuration.duration), event: present(configuration.event), rowFrequency: configuration.rowFrequency.kind === 'one-observation-per-row' ? configuration.rowFrequency : { kind: 'frequency-column', column: present(configuration.rowFrequency.column) }, horizon: horizon(configuration.predictionTimes), ties: configuration.ties }
    case 'start-stop-proportional-hazards':
      return { kind: 'start-stop', start: present(configuration.start), stop: present(configuration.stop), event: present(configuration.event), rowFrequency: configuration.rowFrequency.kind === 'one-observation-per-row' ? configuration.rowFrequency : { kind: 'frequency-column', column: present(configuration.rowFrequency.column) }, covariates: presentAll(configuration.covariates), family: configuration.family, horizon: horizon(configuration.predictionTimes) }
    case 'penalized-aft':
      return { kind: 'penalized-aft', duration: present(configuration.duration), event: present(configuration.event), covariates: presentAll(configuration.covariates), family: configuration.family, penalizer: configuration.penalizer, confidenceLevel: configuration.confidenceLevel, horizon: horizon(configuration.predictionTimes) }
    case 'cox-regression': {
      const observation: CoxObservationDraft = (() => {
        switch (configuration.observation.kind) {
          case 'right-censored': return {
            kind: 'right-censored',
            duration: present(configuration.observation.duration),
            event: present(configuration.observation.event),
            entry: configuration.observation.entry.kind === 'not-used'
              ? configuration.observation.entry
              : { kind: 'column', column: present(configuration.observation.entry.column) },
            standardErrors: configuration.observation.standardErrors.kind === 'clustered'
              ? { kind: 'clustered', column: present(configuration.observation.standardErrors.cluster) }
              : configuration.observation.standardErrors,
            frailty: configuration.observation.frailty.kind === 'gamma'
              ? { kind: 'gamma', column: present(configuration.observation.frailty.group), ties: configuration.observation.frailty.ties }
              : { kind: 'none' },
          }
          case 'start-stop': return {
            kind: 'start-stop',
            subject: present(configuration.observation.subject),
            start: present(configuration.observation.start),
            stop: present(configuration.observation.stop),
            event: present(configuration.observation.event),
          }
          default: return assertNever(configuration.observation)
        }
      })()
      return {
        kind: 'cox-regression',
        observation,
        weights: configuration.weights.kind === 'equal' ? configuration.weights : { kind: 'column', column: present(configuration.weights.column) },
        strata: configuration.strata.kind === 'unstratified' ? configuration.strata : { kind: 'column', column: present(configuration.strata.column) },
        covariates: presentAll(configuration.covariates),
        penalty: configuration.penalty.kind === 'none'
          ? { kind: 'unpenalized' }
          : { kind: 'elastic-net', strength: configuration.penalty.strength, l1Ratio: configuration.penalty.l1Ratio },
        confidenceLevel: configuration.confidenceLevel,
      }
    }
    case 'two-group-comparison':
      return { kind: 'two-group', duration: present(configuration.duration), event: present(configuration.event), group: present(configuration.group), truncationTime: configuration.truncationTime, permutations: configuration.permutations, seed: configuration.seed }
    case 'multi-state-proportional-hazards': {
      const input = configuration.input
      const restored: MultiStateDraftInput = (() => {
        switch (input.kind) {
          case 'prepared-transition-rows': return { kind: 'prepared-rows', start: present(input.start), stop: present(input.stop), event: present(input.event), from: present(input.from), to: present(input.to) }
          case 'longitudinal-state-observations': return { kind: 'longitudinal-states', subject: presentSource(input.subject), time: present(input.time), state: present(input.state), stateCount: input.stateCount, transitions: input.transitions.map(([from, to]) => ({ from, to })) }
          case 'wide-state-events': return {
            kind: 'wide-events',
            states: input.states.map((state) => state.kind === 'not-applicable' ? state : { kind: 'recorded', time: present(state.time), status: present(state.status) }),
            transitions: input.transitions.map(([from, to]) => ({ from, to })),
            entry: input.entry.kind === 'shared' ? input.entry : { kind: 'columns', state: present(input.entry.state), time: present(input.entry.time) },
          }
          default: return assertNever(input)
        }
      })()
      return { kind: 'multi-state', input: restored, family: configuration.family, horizon: horizon(configuration.predictionTimes) }
    }
    default: return assertNever(configuration)
  }
}

export interface SurvivalSession {
  readonly prepared: PreparedDatasetVersionId
  readonly draft: Draft
}

export function retainSurvivalDraft(current: SurvivalSession | null, workflow: Workflow): SurvivalSession | null {
  if (workflow.kind !== 'profiled' || workflow.prepared === null) return null
  const prepared = workflow.prepared
  if (current?.prepared === prepared.id) return current
  const columns = workflow.profile.columns.filter(column => prepared.columns.includes(column.id) && isNumericDuckDbType(column.duckdbType)).map(column => ({ id: column.id, name: column.name }))
  const sourceColumns = workflow.profile.columns.map(column => ({ id: column.id, name: column.name }))
  const recorded = workflow.survivalRuns.at(-1)
  return { prepared: workflow.prepared.id, draft: recorded === undefined ? draftFor('right-censored', columns) : draftFromRun(recorded, columns, sourceColumns) }
}
