import { Metadata } from '@/components/ui/Metadata'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { useMemo, useState } from 'react'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { SurvivalRunResult, survivalFamilyLabel, survivalRunSummary } from '@/components/survival/SurvivalRunResult'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { Orb } from '@/components/ui/Orb'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { button, chapterIntro, field, fieldHint, fieldLabel, num, panel, sectionTitle } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { RunActivity } from '@/domain/activity'
import { isNumericDuckDbType, type ColumnId, type ColumnSelection, type DatasetProfile, type NumericColumnSelection } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
  describeSurvivalRefusal,
  coxRegressionRun,
  multiStateSurvivalRun,
  newSurvivalRunId,
  parametricSurvivalFamilySchema,
  proportionalHazardsFamilySchema,
  rightCensoredSurvivalRun,
  startStopSurvivalRun,
  type ParametricSurvivalFamily,
  type ProportionalHazardsFamily,
  type CoxFrailty,
  type CoxPenalty,
  type CoxTies,
  type CoxStandardErrors,
  type CoxObservationConfiguration,
  type MultiStateInputConfiguration,
  type PenalizedAftFamily,
  penalizedAftRun,
  type SurvivalRowFrequency,
  type SurvivalRunArtifact,
} from '@/domain/survival'
import type { SelectedSource } from '@/domain/workflow'
import { useRunActivity } from '@/lib/useRunActivity'
import { formatCount } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { forestSettingsSchema, type ForestSettings } from '@/domain/survivalRegression'

type RowFrequencyDraft =
  | { readonly kind: 'one-observation-per-row' }
  | { readonly kind: 'frequency-column'; readonly column: ColumnId | null }

type CoxEntryDraft =
  | { readonly kind: 'not-used' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

type CoxWeightsDraft =
  | { readonly kind: 'equal' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

type CoxStrataDraft =
  | { readonly kind: 'unstratified' }
  | { readonly kind: 'column'; readonly column: ColumnId | null }

type CoxStandardErrorsDraft =
  | { readonly kind: 'model-based' }
  | { readonly kind: 'robust' }
  | { readonly kind: 'clustered'; readonly column: ColumnId | null }

type CoxPenaltyDraft =
  | { readonly kind: 'unpenalized' }
  | { readonly kind: 'elastic-net'; readonly strength: number; readonly l1Ratio: number }

type CoxFrailtyDraft =
  | { readonly kind: 'none' }
  | { readonly kind: 'gamma'; readonly column: ColumnId | null; readonly ties: CoxTies }

type CoxObservationDraft =
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

type Draft =
  | { readonly kind: 'aalen'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[] }
  | { readonly kind: 'survival-forest'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly categorical: readonly ColumnId[]; readonly settings: ForestSettings; readonly predictionRow: number }
  | { readonly kind: 'right-censored'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly covariates: readonly ColumnId[]; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'nonparametric'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly horizon: number; readonly ties: 'discrete' | 'smoothed' }
  | { readonly kind: 'start-stop'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly rowFrequency: RowFrequencyDraft; readonly covariates: readonly ColumnId[]; readonly family: ProportionalHazardsFamily; readonly horizon: number }
  | { readonly kind: 'cox-regression'; readonly observation: CoxObservationDraft; readonly weights: CoxWeightsDraft; readonly strata: CoxStrataDraft; readonly covariates: readonly ColumnId[]; readonly penalty: CoxPenaltyDraft; readonly confidenceLevel: number }
  | { readonly kind: 'penalized-aft'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly family: PenalizedAftFamily; readonly penalizer: number; readonly confidenceLevel: number; readonly horizon: number }
  | { readonly kind: 'two-group'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly group: ColumnId | null; readonly truncationTime: number; readonly permutations: number; readonly seed: number }
  | { readonly kind: 'multi-state'; readonly input: MultiStateDraftInput; readonly family: ProportionalHazardsFamily; readonly horizon: number }

interface TransitionDraft { readonly from: number; readonly to: number }
type WideStateDraft =
  | { readonly kind: 'not-applicable' }
  | { readonly kind: 'recorded'; readonly time: ColumnId | null; readonly status: ColumnId | null }
type MultiStateDraftInput =
  | { readonly kind: 'prepared-rows'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly from: ColumnId | null; readonly to: ColumnId | null }
  | { readonly kind: 'longitudinal-states'; readonly subject: ColumnId | null; readonly time: ColumnId | null; readonly state: ColumnId | null; readonly stateCount: number; readonly transitions: readonly TransitionDraft[] }
  | {
      readonly kind: 'wide-events'
      readonly states: readonly WideStateDraft[]
      readonly transitions: readonly TransitionDraft[]
      readonly entry: { readonly kind: 'shared'; readonly state: number; readonly time: number } | { readonly kind: 'columns'; readonly state: ColumnId | null; readonly time: ColumnId | null }
    }

type CoxReadyDraft = {
  readonly kind: 'cox-regression'
  readonly observation: CoxObservationConfiguration
  readonly weights: { readonly kind: 'equal' } | { readonly kind: 'column'; readonly column: NumericColumnSelection }
  readonly strata: { readonly kind: 'unstratified' } | { readonly kind: 'column'; readonly column: NumericColumnSelection }
  readonly covariates: NonEmptyArray<NumericColumnSelection>
  readonly penalty: CoxPenalty
  readonly confidenceLevel: number
  readonly columns: NonEmptyArray<ColumnId>
}

type ReadyDraft =
  | { readonly kind: 'aalen'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: NonEmptyArray<NumericColumnSelection>; readonly columns: NonEmptyArray<ColumnId> }
  | { readonly kind: 'survival-forest'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: NonEmptyArray<NumericColumnSelection>; readonly categorical: readonly NumericColumnSelection[]; readonly settings: ForestSettings; readonly predictionRow: number; readonly columns: NonEmptyArray<ColumnId> }
  | { readonly kind: 'right-censored'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly rowFrequency: SurvivalRowFrequency; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'nonparametric'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly rowFrequency: SurvivalRowFrequency; readonly columns: NonEmptyArray<ColumnId>; readonly horizon: number; readonly ties: 'discrete' | 'smoothed' }
  | { readonly kind: 'start-stop'; readonly start: NumericColumnSelection; readonly stop: NumericColumnSelection; readonly event: NumericColumnSelection; readonly rowFrequency: SurvivalRowFrequency; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ProportionalHazardsFamily; readonly horizon: number }
  | CoxReadyDraft
  | { readonly kind: 'penalized-aft'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: NonEmptyArray<NumericColumnSelection>; readonly family: PenalizedAftFamily; readonly penalizer: number; readonly confidenceLevel: number; readonly horizon: number; readonly columns: NonEmptyArray<ColumnId> }
  | { readonly kind: 'two-group'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly group: NumericColumnSelection; readonly columns: NonEmptyArray<ColumnId>; readonly truncationTime: number; readonly permutations: number; readonly seed: number }
  | { readonly kind: 'multi-state'; readonly input: MultiStateReadyInput; readonly family: ProportionalHazardsFamily; readonly horizon: number }

type MultiStateReadyInput =
  | { readonly kind: 'prepared-rows'; readonly start: NumericColumnSelection; readonly stop: NumericColumnSelection; readonly event: NumericColumnSelection; readonly from: NumericColumnSelection; readonly to: NumericColumnSelection; readonly columns: NonEmptyArray<ColumnId> }
  | { readonly kind: 'longitudinal-states'; readonly subject: ColumnSelection; readonly time: NumericColumnSelection; readonly state: NumericColumnSelection; readonly stateCount: number; readonly transitions: NonEmptyArray<TransitionDraft>; readonly columns: NonEmptyArray<ColumnId> }
  | {
      readonly kind: 'wide-events'
      readonly states: NonEmptyArray<{ readonly kind: 'not-applicable' } | { readonly kind: 'recorded'; readonly time: NumericColumnSelection; readonly status: NumericColumnSelection }>
      readonly transitions: NonEmptyArray<TransitionDraft>
      readonly entry: { readonly kind: 'shared'; readonly state: number; readonly time: number } | { readonly kind: 'columns'; readonly state: NumericColumnSelection; readonly time: NumericColumnSelection }
      readonly columns: NonEmptyArray<ColumnId>
    }
type WideReadyInput = Extract<MultiStateReadyInput, { readonly kind: 'wide-events' }>

type DraftProblem = { readonly kind: 'invalid-survival-draft'; readonly detail: string }

const FOREST_PARAMETERS = [
  { key: 'trees', title: 'Trees', help: 'Number of trees. Each uses a bootstrap sample of the observations.' },
  { key: 'mtry', title: 'Candidate covariates per split', help: 'Number of covariates considered at each split. Cannot exceed the number selected below.' },
  { key: 'seed', title: 'Random seed', help: 'A positive seed makes the same inputs and settings reproducible.' },
  { key: 'minNodeSize', title: 'Minimum node size', help: 'Do not split a node with fewer than this many observations.' },
  { key: 'minBucket', title: 'Minimum terminal size', help: 'Require at least this many observations in each child node.' },
] as const

/** A column whose name says what it is; otherwise the field waits for a choice rather than guessing by position. */
const columnLike = (columns: readonly ColumnSelection[], pattern: RegExp): ColumnId | null =>
  columns.find((column) => pattern.test(column.name))?.id ?? null

const defaultTransitions = (): readonly TransitionDraft[] => [
  { from: 1, to: 2 },
  { from: 1, to: 3 },
  { from: 2, to: 3 },
]

const preparedMultiStateInput = (columns: readonly NumericColumnSelection[]): Extract<MultiStateDraftInput, { readonly kind: 'prepared-rows' }> => ({
  kind: 'prepared-rows',
  start: columnLike(columns, /^(start|tstart)$/i),
  stop: columnLike(columns, /^(stop|tstop)$/i),
  event: columnLike(columns, /^(event|status)$/i),
  from: columnLike(columns, /^from$/i),
  to: columnLike(columns, /^to$/i),
})

const draftFor = (kind: Draft['kind'], columns: readonly NumericColumnSelection[]): Draft => {
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
const draftFromRun = (run: SurvivalRunArtifact, columns: readonly NumericColumnSelection[], sourceColumns: readonly ColumnSelection[]): Draft => {
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

const predictionTimes = (horizon: number): NonEmptyArray<number> =>
  [0, ...Array.from({ length: 50 }, (_, index) => horizon * (index + 1) / 50)]

const nonparametricTimes = (horizon: number): NonEmptyArray<number> =>
  Number.isInteger(horizon) && horizon <= 499
    ? [0, ...Array.from({ length: horizon }, (_, index) => index + 1)]
    : predictionTimes(horizon)

const selectColumn = (
  columns: readonly NumericColumnSelection[],
  id: ColumnId,
): NumericColumnSelection | null => columns.find((column) => column.id === id) ?? null

const selectColumns = (
  columns: readonly NumericColumnSelection[],
  ids: readonly ColumnId[],
): readonly NumericColumnSelection[] | null => {
  const selected = ids.map((id) => selectColumn(columns, id))
  return selected.every((column): column is NumericColumnSelection => column !== null) ? selected : null
}

const validateDraft = (
  draft: Draft,
  columns: readonly NumericColumnSelection[],
  sourceColumns: readonly ColumnSelection[],
): Result<ReadyDraft, DraftProblem> => {
  const invalid = (detail: string): Result<never, DraftProblem> =>
    err({ kind: 'invalid-survival-draft', detail })
  const selectedOrProblem = (ids: readonly ColumnId[]) => {
    const selected = selectColumns(columns, ids)
    return selected === null
      ? invalid('A selected column is no longer in the prepared dataset.')
      : ok(selected)
  }
  const transitionsOrProblem = (transitions: readonly TransitionDraft[], stateCount: number): Result<NonEmptyArray<TransitionDraft>, DraftProblem> => {
    if (!Number.isInteger(stateCount) || stateCount < 2 || stateCount > 32) return invalid('Use between 2 and 32 states.')
    if (!isNonEmpty(transitions)) return invalid('Add at least one allowed transition.')
    const keys = transitions.map(({ from, to }) => `${from}:${to}`)
    if (new Set(keys).size !== keys.length) return invalid('Each allowed transition must appear once.')
    if (transitions.some(({ from, to }) => !Number.isInteger(from) || !Number.isInteger(to) || from < 1 || to < 1 || from > stateCount || to > stateCount || from === to)) {
      return invalid(`Each transition must connect two different states numbered 1 through ${stateCount}.`)
    }
    return ok(transitions)
  }
  const rowFrequencyOrProblem = (rowFrequency: RowFrequencyDraft): Result<{
    readonly configuration: SurvivalRowFrequency
    readonly columns: readonly ColumnId[]
  }, DraftProblem> => {
    switch (rowFrequency.kind) {
      case 'one-observation-per-row':
        return ok({ configuration: rowFrequency, columns: [] })
      case 'frequency-column': {
        if (rowFrequency.column === null) return invalid('Choose the frequency column.')
        const column = selectColumn(columns, rowFrequency.column)
        return column === null
          ? invalid('The frequency column is no longer in the prepared dataset.')
          : ok({ configuration: { kind: 'frequency-column', column }, columns: [column.id] })
      }
      default: return assertNever(rowFrequency)
    }
  }

  switch (draft.kind) {
    case 'right-censored': {
      const durationId = draft.duration
      const eventId = draft.event
      if (durationId === null) return invalid('Choose the duration column.')
      if (eventId === null) return invalid('Choose the event column.')
      const rowFrequency = rowFrequencyOrProblem(draft.rowFrequency)
      if (!rowFrequency.ok) return rowFrequency
      const ids: NonEmptyArray<ColumnId> = [durationId, eventId, ...rowFrequency.value.columns, ...draft.covariates]
      if (new Set(ids).size !== ids.length) {
        return invalid('The duration, event, frequency, and each covariate must use different columns.')
      }
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event] = selected.value
      if (duration === undefined || event === undefined) return invalid('Choose duration and event columns.')
      const covariates = selectColumns(columns, draft.covariates)
      if (covariates === null) return invalid('A selected covariate is no longer in the prepared dataset.')
      return ok({ ...draft, duration, event, rowFrequency: rowFrequency.value.configuration, covariates, columns: ids })
    }
    case 'nonparametric': {
      const durationId = draft.duration
      const eventId = draft.event
      if (durationId === null) return invalid('Choose the duration column.')
      if (eventId === null) return invalid('Choose the event column.')
      const rowFrequency = rowFrequencyOrProblem(draft.rowFrequency)
      if (!rowFrequency.ok) return rowFrequency
      const ids: NonEmptyArray<ColumnId> = [durationId, eventId, ...rowFrequency.value.columns]
      if (new Set(ids).size !== ids.length) return invalid('Duration, event, and frequency must use different columns.')
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event] = selected.value
      if (duration === undefined || event === undefined) return invalid('Choose duration and event columns.')
      return ok({ ...draft, duration, event, rowFrequency: rowFrequency.value.configuration, columns: ids })
    }
    case 'start-stop': {
      const startId = draft.start
      const stopId = draft.stop
      const eventId = draft.event
      if (startId === null) return invalid('Choose the start time column.')
      if (stopId === null) return invalid('Choose the stop time column.')
      if (eventId === null) return invalid('Choose the event column.')
      const rowFrequency = rowFrequencyOrProblem(draft.rowFrequency)
      if (!rowFrequency.ok) return rowFrequency
      const ids: NonEmptyArray<ColumnId> = [startId, stopId, eventId, ...rowFrequency.value.columns, ...draft.covariates]
      if (new Set(ids).size !== ids.length) {
        return invalid('The start, stop, event, frequency, and each covariate must use different columns.')
      }
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [start, stop, event] = selected.value
      if (start === undefined || stop === undefined || event === undefined) {
        return invalid('Choose start, stop, and event columns.')
      }
      const covariates = selectColumns(columns, draft.covariates)
      if (covariates === null) return invalid('A selected covariate is no longer in the prepared dataset.')
      return ok({ ...draft, start, stop, event, rowFrequency: rowFrequency.value.configuration, covariates, columns: ids })
    }
    case 'aalen':
    case 'survival-forest': {
      if (draft.duration === null) return invalid('Choose the duration column.')
      if (draft.event === null) return invalid('Choose the event column.')
      const ids: NonEmptyArray<ColumnId> = [draft.duration, draft.event, ...draft.covariates]
      if (new Set(ids).size !== ids.length) return invalid('Duration, event and each covariate must use different columns.')
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event] = selected.value
      if (duration === undefined || event === undefined) return invalid('Choose duration and event columns.')
      const covariates = selectColumns(columns, draft.covariates)
      if (covariates === null || !isNonEmpty(covariates)) return invalid('Choose at least one available covariate.')
      if (draft.kind === 'aalen') return ok({ kind: draft.kind, duration, event, covariates, columns: ids })
      const settings = forestSettingsSchema.safeParse(draft.settings)
      if (!settings.success) return invalid('Use positive whole numbers for tree count, candidate covariates, node sizes and seed. At most 5,000 trees are allowed.')
      if (settings.data.mtry > covariates.length) return invalid('Candidate covariates per split cannot exceed the number of selected covariates.')
      if (!Number.isInteger(draft.predictionRow) || draft.predictionRow < 0) return invalid('Choose a prediction row of 1 or more.')
      const categorical = covariates.filter((column) => draft.categorical.includes(column.id))
      return ok({ kind: draft.kind, duration, event, covariates, categorical, settings: settings.data, predictionRow: draft.predictionRow, columns: ids })
    }
    case 'penalized-aft': {
      const durationId = draft.duration
      const eventId = draft.event
      if (durationId === null) return invalid('Choose the duration column.')
      if (eventId === null) return invalid('Choose the event column.')
      if (!isNonEmpty(draft.covariates)) return invalid('Choose at least one covariate.')
      if (!(Number.isFinite(draft.penalizer) && draft.penalizer >= 0)) return invalid('Use a penalty of zero or more.')
      if (!(draft.confidenceLevel > 0 && draft.confidenceLevel < 1)) return invalid('Use a confidence level between 0 and 100 percent.')
      const ids: NonEmptyArray<ColumnId> = [durationId, eventId, ...draft.covariates]
      if (new Set(ids).size !== ids.length) return invalid('The duration, event, and each covariate must use different columns.')
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event] = selected.value
      if (duration === undefined || event === undefined) return invalid('Choose duration and event columns.')
      const covariates = selectColumns(columns, draft.covariates)
      if (covariates === null || !isNonEmpty(covariates)) return invalid('A selected covariate is no longer in the prepared dataset.')
      return ok({ ...draft, duration, event, covariates, columns: ids })
    }
    case 'cox-regression': {
      const covariateIds = draft.covariates
      if (!isNonEmpty(covariateIds)) return invalid('Choose at least one covariate.')
      if (!(draft.confidenceLevel > 0 && draft.confidenceLevel < 1)) return invalid('Use a confidence level between 0 and 100 percent.')

      const observationIds: readonly ColumnId[] | null = (() => {
        switch (draft.observation.kind) {
          case 'right-censored': {
            const { duration, event, entry, standardErrors, frailty } = draft.observation
            if (duration === null || event === null) return null
            if (entry.kind === 'column' && entry.column === null) return null
            if (standardErrors.kind === 'clustered' && standardErrors.column === null) return null
            if (frailty.kind === 'gamma' && frailty.column === null) return null
            const ids: ColumnId[] = [duration, event]
            if (entry.kind === 'column' && entry.column !== null) ids.push(entry.column)
            if (standardErrors.kind === 'clustered' && standardErrors.column !== null) ids.push(standardErrors.column)
            if (frailty.kind === 'gamma' && frailty.column !== null) ids.push(frailty.column)
            return ids
          }
          case 'start-stop': {
            const { subject, start, stop, event } = draft.observation
            return subject === null || start === null || stop === null || event === null
              ? null
              : [subject, start, stop, event]
          }
          default: return assertNever(draft.observation)
        }
      })()
      if (observationIds === null) return invalid('Choose every column required by the selected Cox observation, standard-error and frailty settings.')
      if (draft.observation.kind === 'right-censored' && draft.observation.frailty.kind === 'gamma') {
        if (draft.observation.entry.kind !== 'not-used') return invalid('A shared frailty model does not take a delayed-entry column.')
        if (draft.observation.standardErrors.kind !== 'model-based') return invalid('A shared frailty model reports model-based standard errors only.')
        if (draft.penalty.kind !== 'unpenalized') return invalid('A shared frailty model cannot also carry an elastic-net penalty.')
      }
      if (draft.weights.kind === 'column' && draft.weights.column === null) return invalid('Choose the observation-weight column.')
      if (draft.strata.kind === 'column' && draft.strata.column === null) return invalid('Choose the stratum column.')

      const roleIds: ColumnId[] = [...observationIds]
      if (draft.weights.kind === 'column' && draft.weights.column !== null) roleIds.push(draft.weights.column)
      if (draft.strata.kind === 'column' && draft.strata.column !== null) roleIds.push(draft.strata.column)
      if (!isNonEmpty(roleIds)) return invalid('Choose the columns that define the Cox observation rows.')
      const ids: NonEmptyArray<ColumnId> = [roleIds[0], ...roleIds.slice(1), ...covariateIds]
      if (new Set(ids).size !== ids.length) return invalid('Each Cox row role and covariate must use a different column.')
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const byId = new Map(selected.value.map((column) => [column.id, column]))
      const chosen = (id: ColumnId): NumericColumnSelection | null => byId.get(id) ?? null

      const observation: CoxObservationConfiguration | null = (() => {
        switch (draft.observation.kind) {
          case 'right-censored': {
            const duration = draft.observation.duration === null ? null : chosen(draft.observation.duration)
            const event = draft.observation.event === null ? null : chosen(draft.observation.event)
            if (duration === null || event === null) return null
            const entry: Extract<CoxObservationConfiguration, { readonly kind: 'right-censored' }>['entry'] | null = (() => {
              switch (draft.observation.entry.kind) {
                case 'not-used': return draft.observation.entry
                case 'column': {
                  if (draft.observation.entry.column === null) return null
                  const column = chosen(draft.observation.entry.column)
                  return column === null ? null : { kind: 'column', column }
                }
                default: return assertNever(draft.observation.entry)
              }
            })()
            const standardErrors: CoxStandardErrors | null = (() => {
              switch (draft.observation.standardErrors.kind) {
                case 'model-based': return draft.observation.standardErrors
                case 'robust': return draft.observation.standardErrors
                case 'clustered': {
                  if (draft.observation.standardErrors.column === null) return null
                  const cluster = chosen(draft.observation.standardErrors.column)
                  return cluster === null ? null : { kind: 'clustered', cluster }
                }
                default: return assertNever(draft.observation.standardErrors)
              }
            })()
            const frailty: CoxFrailty | null = (() => {
              switch (draft.observation.frailty.kind) {
                case 'none': return draft.observation.frailty
                case 'gamma': {
                  if (draft.observation.frailty.column === null) return null
                  const group = chosen(draft.observation.frailty.column)
                  return group === null ? null : { kind: 'gamma', group, ties: draft.observation.frailty.ties }
                }
                default: return assertNever(draft.observation.frailty)
              }
            })()
            return entry === null || standardErrors === null || frailty === null
              ? null
              : { kind: 'right-censored', duration, event, entry, standardErrors, frailty }
          }
          case 'start-stop': {
            const subject = draft.observation.subject === null ? null : chosen(draft.observation.subject)
            const start = draft.observation.start === null ? null : chosen(draft.observation.start)
            const stop = draft.observation.stop === null ? null : chosen(draft.observation.stop)
            const event = draft.observation.event === null ? null : chosen(draft.observation.event)
            return subject === null || start === null || stop === null || event === null
              ? null
              : { kind: 'start-stop', subject, start, stop, event, standardErrors: { kind: 'model-based' } }
          }
          default: return assertNever(draft.observation)
        }
      })()
      if (observation === null) return invalid('A selected Cox observation column is no longer in the prepared dataset.')

      const weights: CoxReadyDraft['weights'] = (() => {
        switch (draft.weights.kind) {
          case 'equal': return draft.weights
          case 'column': {
            const column = draft.weights.column === null ? null : chosen(draft.weights.column)
            return column === null ? { kind: 'equal' } : { kind: 'column', column }
          }
          default: return assertNever(draft.weights)
        }
      })()
      if (draft.weights.kind === 'column' && weights.kind === 'equal') return invalid('The observation-weight column is no longer in the prepared dataset.')
      const strata: CoxReadyDraft['strata'] = (() => {
        switch (draft.strata.kind) {
          case 'unstratified': return draft.strata
          case 'column': {
            const column = draft.strata.column === null ? null : chosen(draft.strata.column)
            return column === null ? { kind: 'unstratified' } : { kind: 'column', column }
          }
          default: return assertNever(draft.strata)
        }
      })()
      if (draft.strata.kind === 'column' && strata.kind === 'unstratified') return invalid('The stratum column is no longer in the prepared dataset.')

      const covariates = covariateIds.map(chosen)
      if (!covariates.every((column): column is NumericColumnSelection => column !== null) || !isNonEmpty(covariates)) {
        return invalid('A selected Cox covariate is no longer in the prepared dataset.')
      }
      const penalty: CoxPenalty = (() => {
        switch (draft.penalty.kind) {
          case 'unpenalized': return { kind: 'none' }
          case 'elastic-net': return { kind: 'uniform', strength: draft.penalty.strength, l1Ratio: draft.penalty.l1Ratio }
          default: return assertNever(draft.penalty)
        }
      })()
      if (penalty.kind === 'uniform' && (!Number.isFinite(penalty.strength) || !(penalty.strength > 0) || !Number.isFinite(penalty.l1Ratio) || penalty.l1Ratio < 0 || penalty.l1Ratio > 1)) {
        return invalid('Penalty strength must be above zero and the L1 ratio must be between 0 and 1.')
      }
      return ok({ kind: draft.kind, observation, weights, strata, covariates, penalty, confidenceLevel: draft.confidenceLevel, columns: ids })
    }
    case 'two-group': {
      const durationId = draft.duration
      const eventId = draft.event
      const groupId = draft.group
      if (durationId === null) return invalid('Choose the duration column.')
      if (eventId === null) return invalid('Choose the event column.')
      if (groupId === null) return invalid('Choose the group column.')
      if (new Set([durationId, eventId, groupId]).size !== 3) return invalid('The duration, the event and the group must be different columns.')
      const ids: NonEmptyArray<ColumnId> = [durationId, eventId, groupId]
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event, group] = selected.value
      if (duration === undefined || event === undefined || group === undefined) {
        return invalid('Choose duration, event, and group columns.')
      }
      return ok({ ...draft, duration, event, group, columns: ids })
    }
    case 'multi-state': {
      const input = draft.input
      switch (input.kind) {
        case 'prepared-rows': {
          const { start: startId, stop: stopId, event: eventId, from: fromId, to: toId } = input
          if (startId === null) return invalid('Choose the start time column.')
          if (stopId === null) return invalid('Choose the stop time column.')
          if (eventId === null) return invalid('Choose the event column.')
          if (fromId === null) return invalid('Choose the origin state column.')
          if (toId === null) return invalid('Choose the destination state column.')
          if (new Set([startId, stopId, eventId, fromId, toId]).size !== 5) return invalid('The five prepared-row roles must use different columns.')
          const ids: NonEmptyArray<ColumnId> = [startId, stopId, eventId, fromId, toId]
          const selected = selectedOrProblem(ids)
          if (!selected.ok) return selected
          const [start, stop, event, from, to] = selected.value
          if (start === undefined || stop === undefined || event === undefined || from === undefined || to === undefined) return invalid('Choose all five prepared-row columns.')
          return ok({ ...draft, input: { kind: input.kind, start, stop, event, from, to, columns: ids } })
        }
        case 'longitudinal-states': {
          if (input.subject === null) return invalid('Choose the subject column.')
          if (input.time === null) return invalid('Choose the numeric observation-time column.')
          if (input.state === null) return invalid('Choose the observed state column.')
          const transitions = transitionsOrProblem(input.transitions, input.stateCount)
          if (!transitions.ok) return transitions
          const subject = sourceColumns.find((column) => column.id === input.subject) ?? null
          const time = selectColumn(columns, input.time)
          const state = selectColumn(columns, input.state)
          if (subject === null || time === null || state === null) return invalid('The subject, numeric observation time, or state column is no longer available.')
          if (new Set([subject.id, time.id, state.id]).size !== 3) return invalid('Subject, time, and state must be different columns.')
          return ok({ ...draft, input: { ...input, subject, time, state, transitions: transitions.value, columns: [time.id, state.id] } })
        }
        case 'wide-events': {
          if (!isNonEmpty(input.states)) return invalid('Define at least two states.')
          const transitions = transitionsOrProblem(input.transitions, input.states.length)
          if (!transitions.ok) return transitions
          const recordedIds: ColumnId[] = []
          for (const state of input.states) {
            if (state.kind === 'not-applicable') continue
            if (state.time === null || state.status === null) return invalid('Choose both a time and status column for every recorded state.')
            if (state.time === state.status) return invalid('A state time and its event status must use different columns.')
            recordedIds.push(state.time, state.status)
          }
          const entryIds: ColumnId[] = input.entry.kind === 'columns'
            ? input.entry.state === null || input.entry.time === null
              ? []
              : [input.entry.state, input.entry.time]
            : []
          if (input.entry.kind === 'columns' && entryIds.length === 0) return invalid('Choose the entry state and entry time columns.')
          if (input.entry.kind === 'shared' && (!Number.isInteger(input.entry.state) || input.entry.state < 1 || input.entry.state > input.states.length || !Number.isFinite(input.entry.time))) {
            return invalid(`The shared entry state must be numbered 1 through ${input.states.length}, and its time must be finite.`)
          }
          const uniqueIds = [...new Set([...recordedIds, ...entryIds])]
          if (!isNonEmpty(uniqueIds)) return invalid('Choose at least one state time and status pair.')
          const selected = selectedOrProblem(uniqueIds)
          if (!selected.ok) return selected
          const byId = new Map(selected.value.map((column) => [column.id, column]))
          const states: Array<{ readonly kind: 'not-applicable' } | { readonly kind: 'recorded'; readonly time: NumericColumnSelection; readonly status: NumericColumnSelection }> = []
          for (const state of input.states) {
            if (state.kind === 'not-applicable') {
              states.push(state)
              continue
            }
            if (state.time === null || state.status === null) return invalid('Choose both a time and status column for every recorded state.')
            const time = byId.get(state.time)
            const status = byId.get(state.status)
            if (time === undefined || status === undefined) return invalid('A selected wide-event column is no longer in the prepared dataset.')
            states.push({ kind: 'recorded', time, status })
          }
          if (!isNonEmpty(states)) return invalid('Define at least two states.')
          const entry: WideReadyInput['entry'] | null = (() => {
            if (input.entry.kind === 'shared') return input.entry
            if (input.entry.state === null || input.entry.time === null) return null
            const state = byId.get(input.entry.state)
            const time = byId.get(input.entry.time)
            return state === undefined || time === undefined ? null : { kind: 'columns' as const, state, time }
          })()
          if (entry === null) return invalid('The selected entry-state or entry-time column is no longer in the prepared dataset.')
          return ok({ ...draft, input: { kind: input.kind, states, transitions: transitions.value, entry, columns: uniqueIds } })
        }
        default: return assertNever(input)
      }
    }
    default: return assertNever(draft)
  }
}

const observationColumns = (draft: Draft): readonly (ColumnId | null)[] => {
  switch (draft.kind) {
    case 'right-censored': return [draft.duration, draft.event, draft.rowFrequency.kind === 'frequency-column' ? draft.rowFrequency.column : null]
    case 'nonparametric': return [draft.duration, draft.event, draft.rowFrequency.kind === 'frequency-column' ? draft.rowFrequency.column : null]
    case 'start-stop': return [draft.start, draft.stop, draft.event, draft.rowFrequency.kind === 'frequency-column' ? draft.rowFrequency.column : null]
    case 'aalen':
    case 'survival-forest':
    case 'penalized-aft': return [draft.duration, draft.event]
    case 'cox-regression': {
      const observation = draft.observation
      const observationRoles = (() => {
        switch (observation.kind) {
          case 'right-censored': return [
            observation.duration,
            observation.event,
            ...(observation.entry.kind === 'column' ? [observation.entry.column] : []),
            ...(observation.standardErrors.kind === 'clustered' ? [observation.standardErrors.column] : []),
            ...(observation.frailty.kind === 'gamma' ? [observation.frailty.column] : []),
          ]
          case 'start-stop': return [observation.subject, observation.start, observation.stop, observation.event]
          default: return assertNever(observation)
        }
      })()
      return [
        ...observationRoles,
        ...(draft.weights.kind === 'column' ? [draft.weights.column] : []),
        ...(draft.strata.kind === 'column' ? [draft.strata.column] : []),
      ]
    }
    case 'two-group': return [draft.duration, draft.event, draft.group]
    case 'multi-state': {
      const input = draft.input
      switch (input.kind) {
        case 'prepared-rows': return [input.start, input.stop, input.event, input.from, input.to]
        case 'longitudinal-states': return [input.subject, input.time, input.state]
        case 'wide-events': return [
          ...input.states.flatMap((state) => state.kind === 'recorded' ? [state.time, state.status] : []),
          ...(input.entry.kind === 'columns' ? [input.entry.state, input.entry.time] : []),
        ]
        default: return assertNever(input)
      }
    }
    default: return assertNever(draft)
  }
}

const withoutCovariate = (
  covariates: readonly ColumnId[],
  role: ColumnId | null,
): readonly ColumnId[] => role === null ? covariates : covariates.filter((id) => id !== role)

/** Carry the user's predictor choices across regression methods; reserve the new observation roles. */
const retainCovariates = (previous: Draft, next: Draft): Draft => {
  const selected = (() => {
    switch (previous.kind) {
      case 'right-censored':
      case 'start-stop':
      case 'cox-regression':
      case 'penalized-aft':
      case 'aalen':
      case 'survival-forest': return previous.covariates
      case 'nonparametric':
      case 'two-group':
      case 'multi-state': return []
      default: return assertNever(previous)
    }
  })()
  switch (next.kind) {
    case 'right-censored':
    case 'start-stop':
    case 'cox-regression':
    case 'penalized-aft':
    case 'aalen':
    case 'survival-forest': {
      const reserved = new Set(observationColumns(next))
      return { ...next, covariates: selected.filter((id) => !reserved.has(id)) }
    }
    case 'nonparametric':
    case 'two-group':
    case 'multi-state': return next
    default: return assertNever(next)
  }
}

interface AnalysisType {
  readonly name: string
  readonly summary: string
  readonly requirements: readonly { readonly holds: string; readonly otherwise: string }[]
}

const analysisType = (kind: Draft['kind']): AnalysisType => {
  switch (kind) {
    case 'aalen': return {
      name: 'Aalen additive regression',
      summary: 'Estimate how covariates add to or subtract from the event rate, allowing their associations to change during follow-up.',
      requirements: [
        { holds: 'Each row describes one independent observation, with a positive duration and an event flag of 0 or 1.', otherwise: 'the risk sets or the model-based uncertainty are incorrect.' },
        { holds: 'Covariates contribute additively to the event rate and remain constant during each observation.', otherwise: 'the fitted coefficient curves may not describe the associations.' },
        { holds: 'Censoring is independent of the event after accounting for the covariates.', otherwise: 'estimated associations may be biased.' },
        { holds: 'Enough observations remain at risk to estimate the selected covariates separately.', otherwise: 'fitting stops before the end of follow-up or is refused. The result records the last fitted event time.' },
      ],
    }
    case 'survival-forest': return {
      name: 'Random survival forest',
      summary: 'Predict event-free probability from an ensemble of survival trees. Estimate predictive importance by shuffling each covariate in observations excluded from a tree’s training sample.',
      requirements: [
        { holds: 'Rows are independent observations with non-negative durations and an event flag of 0 or 1.', otherwise: 'the training samples and prediction checks do not represent independent observations.' },
        { holds: 'Categorical covariates use whole-number codes from 1 to 53 and are explicitly marked categorical.', otherwise: 'their numeric ordering affects how the trees split.' },
        { holds: 'Censoring is independent of the event conditional on the covariates.', otherwise: 'survival predictions may be biased.' },
        { holds: 'Out-of-bag concordance assesses ranking within this dataset; predictive importance describes the fitted forest.', otherwise: 'these results can be mistaken for external validation or causal effects.' },
      ],
    }
    case 'right-censored': return {
      name: 'Parametric survival',
      summary: 'Fit a parametric distribution to the observed durations and event indicators. The result includes a survival curve, hazard curve and median survival time.',
      requirements: [
        { holds: 'Each row is one observation, with duration measured from that observation’s time zero and an event flag of 1 for observed or 0 for censored.', otherwise: 'the likelihood receives the wrong duration or event status.' },
        { holds: 'Censoring is independent of the event after accounting for the model covariates.', otherwise: 'the estimated survival function can be biased.' },
        { holds: 'The selected distribution describes how the hazard changes during follow-up.', otherwise: 'the estimated survival function, hazard and median can be biased; compare the fitted families and their AIC values.' },
        { holds: 'The reported intervals apply to the fitted parameters.', otherwise: 'the parameter intervals are incorrectly interpreted as confidence bands for the survival or hazard curve.' },
      ],
    }
    case 'nonparametric': return {
      name: 'Kaplan–Meier and Nelson–Aalen',
      summary: 'Estimate the survival function and cumulative hazard from the observed durations and event indicators, without choosing a parametric distribution.',
      requirements: [
        { holds: 'Each row is one observation, or a grouped row with a positive whole-number frequency.', otherwise: 'the risk set and event totals do not represent the observed population.' },
        { holds: 'The event flag is 1 for an observed event and 0 when follow-up ended first.', otherwise: 'events and censoring are reversed.' },
        { holds: 'Censoring is independent of the event process.', otherwise: 'the estimated survival function and cumulative hazard can be biased.' },
        { holds: 'The chosen time grid uses the same units as duration.', otherwise: 'the increments describe the wrong reporting periods.' },
      ],
    }
    case 'start-stop': return {
      name: 'Start–stop survival',
      summary: 'Each row records one start–stop interval during which the covariates remain constant. An observation with a changing covariate requires multiple rows. The model uses a proportional-hazards parameterisation.',
      requirements: [
        { holds: 'Each row is one interval of one observation, with start before stop and constant covariates within the interval.', otherwise: 'the start–stop likelihood or the covariate value for that interval is incorrect.' },
        { holds: 'The intervals of one observation do not overlap and the event flag is 1 only on the interval where the event occurred.', otherwise: 'the observation is counted more than once and its event is double-counted or lost.' },
        { holds: 'Covariates multiply the hazard. Accelerated failure-time families are not available for start–stop rows in this analysis.', otherwise: 'the covariate coefficient does not have the reported hazard-ratio interpretation.' },
        { holds: 'Censoring is independent of the event after accounting for the model covariates.', otherwise: 'the estimated survival function can be biased.' },
      ],
    }
    case 'penalized-aft': return {
      name: 'Penalised AFT',
      summary: 'Estimate how covariates are associated with shorter or longer time to an event, using a Weibull or log-logistic model. A penalty shrinks the coefficients toward zero. Results are reported as time ratios.',
      requirements: [
        { holds: 'Every duration is above zero; the model takes the logarithm of each one.', otherwise: 'the fit is refused.' },
        { holds: 'The event flag is 1 when the event occurred and 0 when follow-up ended first.', otherwise: 'events and right-censoring are reversed.' },
        { holds: 'The log of the duration follows the chosen family, shifted by the covariates.', otherwise: 'the time ratios do not describe the covariate associations.' },
        { holds: 'Use the penalty specified for the study. It is applied after covariates are scaled by their sample standard deviations.', otherwise: 'the estimates are shrunk by a different amount than the study reports.' },
      ],
    }
    case 'cox-regression': return {
      name: 'Cox regression',
      summary: 'Estimate how one or more covariates are associated with the event rate while leaving the baseline event rate unspecified. Use right-censored rows or start–stop rows for covariates that change during follow-up.',
      requirements: [
        { holds: 'The event flag is 1 when the event occurred and 0 when follow-up ended first.', otherwise: 'events and right-censoring are reversed.' },
        { holds: 'The covariate hazard ratios remain constant during follow-up.', otherwise: 'one hazard ratio does not describe the covariate association over time.' },
        { holds: 'Censoring is independent of the event after accounting for the selected covariates.', otherwise: 'the fitted coefficients and survival estimates can be biased.' },
        { holds: 'Start–stop rows identify the same subject and contain non-overlapping intervals with constant covariates.', otherwise: 'a subject can be counted in the wrong risk sets.' },
      ],
    }
    case 'two-group': return {
      name: 'Two-group comparison',
      summary: 'The observed event-free curves of two groups, compared across follow-up with tests that stay valid when the curves cross, and the difference in average event-free time through a chosen horizon.',
      requirements: [
        { holds: 'The group column contains only 0 and 1, and each row is an independent observation.', otherwise: 'the two-independent-group sampling model does not apply.' },
        { holds: 'Censoring is comparable between the groups.', otherwise: 'differential censoring can bias the group comparison.' },
        { holds: 'The comparison time is within the follow-up supported by both groups.', otherwise: 'the restricted mean requires extrapolation beyond the observed event times.' },
        { holds: 'Group assignment and the study design support a causal interpretation.', otherwise: 'the result is an observed group difference rather than an estimated effect of group assignment.' },
      ],
    }
    case 'multi-state': return {
      name: 'Multi-state survival',
      summary: 'Fit a cause-specific proportional-hazards model for each observed transition, then calculate the probability of occupying each state during follow-up.',
      requirements: [
        { holds: 'Every permitted move appears as a row with its origin and destination states, and the event flag is 1 only for the transition that happened.', otherwise: 'a move the data never records cannot be estimated and the occupancy curves omit it.' },
        { holds: 'Future transitions depend on the current state and the time since entry, as required by the clock-forward Markov model.', otherwise: 'dependence on earlier states can bias the estimated state-occupancy probabilities.' },
        { holds: 'Transition hazards follow the selected proportional-hazards distribution.', otherwise: 'misspecified transition hazards can bias the estimated state-occupancy probabilities.' },
      ],
    }
    default: return assertNever(kind)
  }
}

function ColumnSelect({ title, value, columns, onChange }: { readonly title: React.ReactNode; readonly value: ColumnId | null; readonly columns: readonly ColumnSelection[]; readonly onChange: (value: ColumnId | null) => void }) {
  return (
    <label className="block">
      <span className={fieldLabel}>{title}</span>
      <Select className={field('text', 'mt-1')} value={value ?? ''} onChange={(event) => onChange(event.target.value === '' ? null : event.target.value as ColumnId)}>
        <option value="">Choose a column</option>
        {columns.map((column) => <option key={column.id} value={column.id}>{column.name}</option>)}
      </Select>
    </label>
  )
}

function RowFrequencyControls({ value, columns, onChange }: {
  readonly value: RowFrequencyDraft
  readonly columns: readonly NumericColumnSelection[]
  readonly onChange: (value: RowFrequencyDraft) => void
}) {
  const selectKind = (kind: RowFrequencyDraft['kind']) => {
    switch (kind) {
      case 'one-observation-per-row': onChange({ kind }); return
      case 'frequency-column': onChange({ kind, column: columnLike(columns, /^(freq|frequency|count|weight)$/i) }); return
      default: assertNever(kind)
    }
  }
  return (
    <div className="sm:col-span-2">
      <ParameterLabel label="Rows represent" help="Choose grouped counts when one row summarizes several observations. The frequency must be a positive whole-number count." />
      <SegmentedControl size="sm" ariaLabel="Survival row representation" value={value.kind} onChange={selectKind} options={[
        { value: 'one-observation-per-row', label: 'One observation' },
        { value: 'frequency-column', label: 'Grouped count' },
      ]} />
      {value.kind === 'frequency-column' && (
        <div className="mt-3 max-w-sm">
          <ColumnSelect title="Frequency" value={value.column} columns={columns} onChange={(column) => onChange({ kind: 'frequency-column', column })} />
        </div>
      )}
    </div>
  )
}

function TransitionControls({ stateCount, transitions, onStateCount, onTransitions }: {
  readonly stateCount: number
  readonly transitions: readonly TransitionDraft[]
  readonly onStateCount: (value: number) => void
  readonly onTransitions: (value: readonly TransitionDraft[]) => void
}) {
  const [candidate, setCandidate] = useState<TransitionDraft>({ from: 1, to: 2 })
  const addTransition = () => {
    if (candidate.from === candidate.to) return
    if (candidate.from > stateCount || candidate.to > stateCount) return
    if (transitions.some((transition) => transition.from === candidate.from && transition.to === candidate.to)) return
    onTransitions([...transitions, candidate])
  }
  return (
    <fieldset className="m-0 border-0 p-0 sm:col-span-2">
      <legend className={fieldLabel}>Allowed state changes</legend>
      <div className="mt-1 grid gap-3 sm:grid-cols-[minmax(8rem,12rem)_1fr]">
        <label className="block">
          <span className={fieldHint}>Number of states</span>
          <input className={field('text', 'mt-1 w-full')} type="number" min={2} max={32} step={1} value={stateCount} aria-label="Number of states" onChange={(event) => onStateCount(Math.min(32, Math.max(2, Number(event.target.value) || 2)))} />
        </label>
        <div>
          <div className="flex flex-wrap gap-2">
            {transitions.map((transition, index) => (
              <button key={`${transition.from}-${transition.to}-${index}`} type="button" className={button('quiet')} onClick={() => onTransitions(transitions.filter((_, candidate) => candidate !== index))} aria-label={`Remove transition ${transition.from} to ${transition.to}`}>
                {transition.from} → {transition.to} ×
              </button>
            ))}
          </div>
          <div className="mt-2 flex flex-wrap items-end gap-2">
            <label className="flex flex-col"><span className={fieldHint}>From</span><Select className={field('text', 'mt-1 w-20')} value={candidate.from} onChange={(event) => setCandidate({ ...candidate, from: Number(event.target.value) })}>{Array.from({ length: stateCount }, (_, index) => <option key={index + 1} value={index + 1}>{index + 1}</option>)}</Select></label>
            <label className="flex flex-col"><span className={fieldHint}>To</span><Select className={field('text', 'mt-1 w-20')} value={candidate.to} onChange={(event) => setCandidate({ ...candidate, to: Number(event.target.value) })}>{Array.from({ length: stateCount }, (_, index) => <option key={index + 1} value={index + 1}>{index + 1}</option>)}</Select></label>
            <button type="button" className={button('quiet')} onClick={addTransition} disabled={candidate.from === candidate.to || transitions.some((transition) => transition.from === candidate.from && transition.to === candidate.to)}>Add transition</button>
          </div>
          <p className={cn(fieldHint, 'mb-0 mt-2')}>Transitions are numbered in the order shown for wide event histories. Select a listed transition to remove it.</p>
        </div>
      </div>
    </fieldset>
  )
}

const allowedMatrix = (stateCount: number, transitions: readonly TransitionDraft[]): readonly (readonly boolean[])[] =>
  Array.from({ length: stateCount }, (_, from) => Array.from({ length: stateCount }, (_, to) => transitions.some((transition) => transition.from === from + 1 && transition.to === to + 1)))

const numberedMatrix = (stateCount: number, transitions: readonly TransitionDraft[]): readonly (readonly (number | null)[])[] => {
  const numbers = new Map(transitions.map((transition, index) => [`${transition.from}:${transition.to}`, index + 1]))
  return Array.from({ length: stateCount }, (_, from) => Array.from({ length: stateCount }, (_, to) => numbers.get(`${from + 1}:${to + 1}`) ?? null))
}

const columnPosition = (columns: readonly NumericColumnSelection[], selected: NumericColumnSelection): number =>
  columns.findIndex((column) => column.id === selected.id)

const transitionPairs = (transitions: NonEmptyArray<TransitionDraft>): NonEmptyArray<readonly [number, number]> => {
  const [first, ...rest] = transitions
  return [[first.from, first.to], ...rest.map(({ from, to }) => [from, to] as const)]
}

export function SurvivalPanel({ source, profile, prepared, runs, onRun: recordRun, onDeleteRun, onActivity }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly runs: readonly SurvivalRunArtifact[]
  readonly onRun: (run: SurvivalRunArtifact) => void
  readonly onDeleteRun: (run: SurvivalRunArtifact['id']) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}) {
  const columns = useMemo(() => profile.columns.filter((column) => prepared.columns.includes(column.id) && isNumericDuckDbType(column.duckdbType)).map((column) => ({ id: column.id, name: column.name })), [prepared.columns, profile.columns])
  const sourceColumns = useMemo(() => profile.columns.map((column) => ({ id: column.id, name: column.name })), [profile.columns])
  const [draft, configure] = useState<Draft>(() => {
    const recorded = runs.at(-1)
    return recorded === undefined ? draftFor('right-censored', columns) : draftFromRun(recorded, columns, sourceColumns)
  })
  const session = useJob('survival')
  const { job } = session
  const [pendingDelete, setPendingDelete] = useState<SurvivalRunArtifact | null>(null)
  const latest = runs.at(-1) ?? null
  useRunActivity(onActivity, job.kind === 'running' ? { label: 'Survival analysis', progress: null } : null)

  const execute = async () => {
    const current = session.start('analysis', 'Survival analysis')
    if (current === null) return
    const fail = (detail: string) => session.fail(current, detail)
    const onRun = (run: SurvivalRunArtifact) => {
      if (session.current(current)) recordRun(run)
    }
    const validated = validateDraft(draft, columns, sourceColumns)
    if (!validated.ok) {
      fail(validated.error.detail)
      return
    }
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const identity = (matrixColumns: NonEmptyArray<ColumnSelection>) => ({ id: newSurvivalRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), columns: matrixColumns })
      const materialise = async (ids: NonEmptyArray<ColumnId>) => {
        const matrix = await materialisePrepared(source, profile, prepared, ids)
        if (!session.current(current)) return null
        if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return null }
        return matrix.value
      }
      switch (validated.value.kind) {
        case 'aalen': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const result = await analysis.runAalen(matrix.values, matrix.rowCount, matrix.columns.length, { duration: 0, event: 1, covariates: draft.covariates.map((_, i) => i + 2) })
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          if (result.value.coefficients.length !== draft.covariates.length + 1) { fail('The coefficient count does not match the selected covariates.'); return }
          onRun({ kind: 'aalen-run', ...identity(matrix.columns), configuration: { kind: 'aalen', duration: draft.duration, event: draft.event, covariates: draft.covariates }, evidence: result.value })
          break
        }
        case 'survival-forest': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          if (draft.predictionRow >= matrix.rowCount) { fail(`Choose a prediction row between 1 and ${matrix.rowCount}.`); return }
          const result = await analysis.runSurvivalForest(matrix.values, matrix.rowCount, matrix.columns.length, { duration: 0, event: 1, covariates: draft.covariates.map((_, i) => i + 2), categorical: draft.categorical.map((column) => columnPosition(matrix.columns, column)), ...draft.settings, predictionRow: draft.predictionRow })
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          if (result.value.importance.length !== draft.covariates.length) { fail('The importance count does not match the selected covariates.'); return }
          onRun({ kind: 'survival-forest-run', ...identity(matrix.columns), configuration: { kind: 'survival-forest', duration: draft.duration, event: draft.event, covariates: draft.covariates, categorical: draft.categorical, settings: draft.settings, predictionRow: draft.predictionRow }, evidence: result.value })
          break
        }
        case 'right-censored': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const grouped = draft.rowFrequency.kind === 'frequency-column'
          const rowFrequency = grouped
            ? { kind: 'frequencyColumn' as const, column: 2 }
            : { kind: 'oneObservationPerRow' as const }
          const result = await analysis.runFlexSurv(matrix.values, matrix.rowCount, matrix.columns.length, { observation: { kind: 'rightCensored', duration: 0, event: 1 }, rowFrequency, covariates: draft.covariates.map((_, index) => index + (grouped ? 3 : 2)), family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = rightCensoredSurvivalRun(identity(matrix.columns), { kind: 'right-censored-parametric', duration: draft.duration, event: draft.event, rowFrequency: draft.rowFrequency, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        case 'nonparametric': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const grouped = draft.rowFrequency.kind === 'frequency-column'
          const rowFrequency = grouped
            ? { kind: 'frequencyColumn' as const, column: 2 }
            : { kind: 'oneObservationPerRow' as const }
          const times = nonparametricTimes(draft.horizon)
          const result = await analysis.runNonparametricSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { duration: 0, event: 1, rowFrequency, predictionTimes: times, ties: draft.ties })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          onRun({ kind: 'nonparametric-survival-run', ...identity(matrix.columns), configuration: { kind: 'right-censored-nonparametric', duration: draft.duration, event: draft.event, rowFrequency: draft.rowFrequency, predictionTimes: times, ties: draft.ties }, evidence: result.value })
          break
        }
        case 'start-stop': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const grouped = draft.rowFrequency.kind === 'frequency-column'
          const rowFrequency = grouped
            ? { kind: 'frequencyColumn' as const, column: 3 }
            : { kind: 'oneObservationPerRow' as const }
          const result = await analysis.runFlexSurv(matrix.values, matrix.rowCount, matrix.columns.length, { observation: { kind: 'startStop', start: 0, stop: 1, event: 2 }, rowFrequency, covariates: draft.covariates.map((_, index) => index + (grouped ? 4 : 3)), family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = startStopSurvivalRun(identity(matrix.columns), { kind: 'start-stop-proportional-hazards', start: draft.start, stop: draft.stop, event: draft.event, rowFrequency: draft.rowFrequency, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        case 'penalized-aft': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          // lifelines' formula transformer sorts the covariates by name before appending the intercept.
          const ordered = [...draft.covariates].sort((left, right) => left.name.localeCompare(right.name, 'en'))
          if (!isNonEmpty(ordered)) { fail('Choose at least one covariate.'); return }
          const times = predictionTimes(draft.horizon)
          const result = await analysis.runPenalizedAft(matrix.values, matrix.rowCount, matrix.columns.length, {
            duration: columnPosition(matrix.columns, draft.duration),
            event: columnPosition(matrix.columns, draft.event),
            covariates: ordered.map((covariate) => columnPosition(matrix.columns, covariate)),
            family: draft.family,
            penalizer: draft.penalizer,
            confidenceLevel: draft.confidenceLevel,
            predictionTimes: times,
          })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = penalizedAftRun(identity(matrix.columns), { kind: 'penalized-aft', duration: draft.duration, event: draft.event, covariates: ordered, family: draft.family, penalizer: draft.penalizer, confidenceLevel: draft.confidenceLevel, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The AFT result does not match the requested family or covariates.'); return }
          onRun(recorded.value); break
        }
        case 'cox-regression': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const observation = (() => {
            switch (draft.observation.kind) {
              case 'right-censored': {
                const entry = draft.observation.entry.kind === 'not-used'
                  ? { kind: 'notUsed' as const }
                  : { kind: 'column' as const, column: columnPosition(matrix.columns, draft.observation.entry.column) }
                const standardErrors = (() => {
                  switch (draft.observation.standardErrors.kind) {
                    case 'model-based': return { kind: 'modelBased' as const }
                    case 'robust': return { kind: 'robust' as const }
                    case 'clustered': return { kind: 'clustered' as const, column: columnPosition(matrix.columns, draft.observation.standardErrors.cluster) }
                    default: return assertNever(draft.observation.standardErrors)
                  }
                })()
                const frailty = draft.observation.frailty.kind === 'none'
                  ? { kind: 'none' as const }
                  : { kind: 'gamma' as const, column: columnPosition(matrix.columns, draft.observation.frailty.group), ties: draft.observation.frailty.ties }
                return {
                  kind: 'rightCensored' as const,
                  duration: columnPosition(matrix.columns, draft.observation.duration),
                  event: columnPosition(matrix.columns, draft.observation.event),
                  entry,
                  standardErrors,
                  frailty,
                }
              }
              case 'start-stop': return {
                kind: 'startStop' as const,
                subject: columnPosition(matrix.columns, draft.observation.subject),
                start: columnPosition(matrix.columns, draft.observation.start),
                stop: columnPosition(matrix.columns, draft.observation.stop),
                event: columnPosition(matrix.columns, draft.observation.event),
              }
              default: return assertNever(draft.observation)
            }
          })()
          const weights = draft.weights.kind === 'equal'
            ? draft.weights
            : { kind: 'column' as const, column: columnPosition(matrix.columns, draft.weights.column) }
          const strata = draft.strata.kind === 'unstratified'
            ? draft.strata
            : { kind: 'column' as const, column: columnPosition(matrix.columns, draft.strata.column) }
          const penalty = draft.penalty.kind === 'none'
            ? { kind: 'unpenalized' as const }
            : { kind: 'elasticNet' as const, penalizer: draft.penalty.strength, l1Ratio: draft.penalty.l1Ratio }
          const result = await analysis.runCoxRegression(matrix.values, matrix.rowCount, matrix.columns.length, {
            observation,
            weights,
            strata,
            covariates: draft.covariates.map((covariate) => columnPosition(matrix.columns, covariate)),
            penalty,
            confidenceLevel: draft.confidenceLevel,
          })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = coxRegressionRun(identity(matrix.columns), {
            kind: 'cox-regression',
            observation: draft.observation,
            weights: draft.weights,
            strata: draft.strata,
            covariates: draft.covariates,
            penalty: draft.penalty,
            confidenceLevel: draft.confidenceLevel,
          }, result.value)
          if (!recorded.ok) { fail('The Cox result does not match the selected observation structure or covariates.'); return }
          onRun(recorded.value); break
        }
        case 'two-group': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const result = await analysis.runComparisonSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { duration: 0, event: 1, group: 2, truncationTime: draft.truncationTime, permutations: draft.permutations, seed: draft.seed })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          onRun({ kind: 'two-group-survival-run', ...identity(matrix.columns), configuration: { kind: 'two-group-comparison', duration: draft.duration, event: draft.event, group: draft.group, truncationTime: draft.truncationTime, permutations: draft.permutations, seed: draft.seed }, evidence: result.value }); break
        }
        case 'multi-state': {
          const draft = validated.value
          const times = predictionTimes(draft.horizon)
          const input = draft.input
          let result: Awaited<ReturnType<typeof analysis.runMultiStateSurvival>>
          let configuration: MultiStateInputConfiguration
          let runColumns: NonEmptyArray<NumericColumnSelection>
          switch (input.kind) {
            case 'prepared-rows': {
              const matrix = await materialise(input.columns); if (matrix === null) return
              result = await analysis.runMultiStateSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { input: { kind: 'preparedRows', start: 0, stop: 1, event: 2, from: 3, to: 4 }, family: draft.family, predictionTimes: times })
              configuration = { kind: 'prepared-transition-rows', start: input.start, stop: input.stop, event: input.event, from: input.from, to: input.to }
              runColumns = matrix.columns
              break
            }
            case 'longitudinal-states': {
              const matrix = await materialise(input.columns); if (matrix === null) return
              const { materializePanelKeysInWorker } = await import('@/data/client')
              if (!session.current(current)) return
              const keys = await materializePanelKeysInWorker(source.file, profile, input.subject.id, input.time.id)
              if (!session.current(current)) return
              if (!keys.ok) { fail(`The subject and time keys could not be prepared: ${keys.error.kind}.`); return }
              if (keys.value.rowCount !== matrix.rowCount) { fail('The subject keys and prepared time/state columns contain different rows. Use a preparation that retains every source row.'); return }
              const subjectCodes = new Map<string, number>()
              const values = new Float64Array(matrix.rowCount * 3)
              for (let row = 0; row < matrix.rowCount; row += 1) {
                const label = keys.value.units[row]!
                const code = subjectCodes.get(label) ?? subjectCodes.size + 1
                subjectCodes.set(label, code)
                values[row] = code
                values[matrix.rowCount + row] = matrix.values[row]!
                values[2 * matrix.rowCount + row] = matrix.values[matrix.rowCount + row]!
              }
              result = await analysis.runMultiStateSurvival(values, matrix.rowCount, 3, { input: { kind: 'longitudinalStates', subject: 0, time: 1, state: 2, allowed: allowedMatrix(input.stateCount, input.transitions) }, family: draft.family, predictionTimes: times })
              configuration = { kind: 'longitudinal-state-observations', subject: input.subject, time: input.time, state: input.state, stateCount: input.stateCount, transitions: transitionPairs(input.transitions) }
              runColumns = [input.subject, input.time, input.state]
              break
            }
            case 'wide-events': {
              const matrix = await materialise(input.columns); if (matrix === null) return
              const states = input.states.map((state) => state.kind === 'not-applicable'
                ? { kind: 'notApplicable' as const }
                : { kind: 'recorded' as const, time: columnPosition(matrix.columns, state.time), status: columnPosition(matrix.columns, state.status) })
              const entry = input.entry.kind === 'shared'
                ? input.entry
                : { kind: 'columns' as const, state: columnPosition(matrix.columns, input.entry.state), time: columnPosition(matrix.columns, input.entry.time) }
              result = await analysis.runMultiStateSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { input: { kind: 'wideEvents', states, transitions: numberedMatrix(input.states.length, input.transitions), entry }, family: draft.family, predictionTimes: times })
              const configuredStates = input.states.map((state) => state.kind === 'not-applicable' ? state : { kind: 'recorded' as const, time: state.time, status: state.status })
              if (!isNonEmpty(configuredStates)) { fail('Wide event preparation needs at least two states.'); return }
              configuration = { kind: 'wide-state-events', states: configuredStates, transitions: transitionPairs(input.transitions), entry: input.entry }
              runColumns = matrix.columns
              break
            }
            default: assertNever(input)
          }
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = multiStateSurvivalRun(identity(runColumns), { kind: 'multi-state-proportional-hazards', input: configuration, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        default: assertNever(validated.value)
      }
      session.finish(current)
    } catch (cause: unknown) {
      console.error('Unexpected survival analysis failure', cause)
      fail('Survival analysis stopped unexpectedly. Try the run again; if it continues, report the problem.')
    }
  }

  const selectDraft = (kind: Draft['kind']) => configure(retainCovariates(draft, draftFor(kind, columns)))
  const changeFamily = (value: string) => {
    switch (draft.kind) {
      case 'right-censored': {
        const family = parametricSurvivalFamilySchema.safeParse(value)
        if (family.success) configure({ ...draft, family: family.data })
        return
      }
      case 'start-stop':
      case 'multi-state': {
        const family = proportionalHazardsFamilySchema.safeParse(value)
        if (family.success) configure({ ...draft, family: family.data })
        return
      }
      case 'two-group': return
      case 'aalen': return
      case 'survival-forest': return
      case 'nonparametric': return
      case 'cox-regression': return
      case 'penalized-aft': {
        if (value === 'weibull' || value === 'logLogistic') configure({ ...draft, family: value })
        return
      }
      default: return assertNever(draft)
    }
  }
  const familyOptions = (
    value: ParametricSurvivalFamily,
    families: readonly ParametricSurvivalFamily[],
  ) => (
    <label className="block">
      <ParameterLabel label="Distribution" help="The shape the hazard takes over follow-up. Weibull and Gompertz let risk rise or fall with age; the exponential holds it constant. PH families multiply the hazard by a covariate's effect, AFT families stretch time." />
      <Select className={field('text', 'mt-1')} value={value} onChange={(event) => changeFamily(event.target.value)}>
        {families.map((family) => <option key={family} value={family}>{survivalFamilyLabel(family)}</option>)}
      </Select>
    </label>
  )
  const horizonControl = (value: number, onChange: (value: number) => void) => (
    <label className="block">
      <ParameterLabel label="Prediction horizon" help="The last follow-up time the fitted curve is drawn to, in the duration column's units." />
      <input className={field('text', 'mt-1 w-full')} type="number" min={0.001} step="any" value={value} aria-label="Prediction horizon" onChange={(event) => onChange(Math.max(0.001, Number(event.target.value) || 10))} />
    </label>
  )

  const draftControls = (() => {
    switch (draft.kind) {
      case 'aalen':
      case 'survival-forest': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration, covariates: withoutCovariate(draft.covariates, duration) })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        {draft.kind === 'survival-forest' ? <>
          <label className="block"><ParameterLabel label="Split rule" help="Log-rank selects the best eligible split. Extra trees tests one randomly chosen threshold per candidate covariate." /><Select className={field('text', 'mt-1')} value={draft.settings.splitRule} onChange={(event) => { const splitRule = event.target.value; if (splitRule === 'logRank' || splitRule === 'extraTrees') configure({ ...draft, settings: { ...draft.settings, splitRule } }) }}><option value="logRank">Log-rank</option><option value="extraTrees">Extra trees</option></Select></label>
          {FOREST_PARAMETERS.map(({ key, title, help }) => <label className="block" key={key}><ParameterLabel label={title} help={help} /><input aria-label={title} className={field('text', 'mt-1 w-full')} type="number" min={1} step={1} value={draft.settings[key]} onChange={(event) => configure({ ...draft, settings: { ...draft.settings, [key]: Number(event.target.value) } })} /></label>)}
          <label className="block"><ParameterLabel label="Prediction row" help="Draw the fitted survival curve for the covariate values in this prepared row. Row numbering begins at 1. This is a fitted prediction, not an out-of-bag prediction." /><input aria-label="Prediction row" className={field('text', 'mt-1 w-full')} type="number" min={1} max={prepared.observations} step={1} value={draft.predictionRow + 1} onChange={(event) => configure({ ...draft, predictionRow: Number(event.target.value) - 1 })} /></label>
        </> : null}
      </>
      case 'right-censored': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration, covariates: withoutCovariate(draft.covariates, duration) })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        <RowFrequencyControls value={draft.rowFrequency} columns={columns} onChange={(rowFrequency) => configure({ ...draft, rowFrequency, covariates: withoutCovariate(draft.covariates, rowFrequency.kind === 'frequency-column' ? rowFrequency.column : null) })} />
        {familyOptions(draft.family, ['exponential', 'weibull', 'weibullPh', 'logNormal', 'gamma', 'gompertz', 'logLogistic', 'generalizedGamma', 'generalizedF'])}
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'nonparametric': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event })} />
        <RowFrequencyControls value={draft.rowFrequency} columns={columns} onChange={(rowFrequency) => configure({ ...draft, rowFrequency })} />
        <div>
          <ParameterLabel label="Tied events" help="Use discrete handling when event times are recorded in discrete units such as years or minutes. Smoothed handling accounts for tied events one at a time, reducing the number at risk after each event." />
          <SegmentedControl size="sm" ariaLabel="Nelson-Aalen tied events" value={draft.ties} onChange={(ties) => configure({ ...draft, ties })} options={[{ value: 'discrete', label: 'Discrete' }, { value: 'smoothed', label: 'Smoothed' }]} />
        </div>
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'start-stop': return <>
        <ColumnSelect title="Start time" value={draft.start} columns={columns} onChange={(start) => configure({ ...draft, start, covariates: withoutCovariate(draft.covariates, start) })} />
        <ColumnSelect title="Stop time" value={draft.stop} columns={columns} onChange={(stop) => configure({ ...draft, stop, covariates: withoutCovariate(draft.covariates, stop) })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        <RowFrequencyControls value={draft.rowFrequency} columns={columns} onChange={(rowFrequency) => configure({ ...draft, rowFrequency, covariates: withoutCovariate(draft.covariates, rowFrequency.kind === 'frequency-column' ? rowFrequency.column : null) })} />
        {familyOptions(draft.family, ['exponential', 'weibullPh', 'gompertz'])}
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'penalized-aft': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration, covariates: withoutCovariate(draft.covariates, duration) })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        <label className="block">
          <ParameterLabel label="Distribution" help="Weibull AFT: the log duration has a Gumbel-type error with a shape parameter; log-logistic AFT: a logistic error. Both report a time ratio per covariate, the multiplier on the duration for a one-unit increase." />
          <Select className={field('text', 'mt-1')} value={draft.family} onChange={(event) => changeFamily(event.target.value)}>
            <option value="weibull">Weibull AFT</option>
            <option value="logLogistic">Log-logistic AFT</option>
          </Select>
        </label>
        <label className="block">
          <ParameterLabel label="Penalty" help="An L2 penalty shrinks coefficients toward zero after covariates are scaled by their sample standard deviations. Use zero to fit without a penalty." />
          <input className={field('text', 'mt-1 w-full')} type="number" min={0} step="any" value={draft.penalizer} aria-label="AFT penalizer" onChange={(event) => configure({ ...draft, penalizer: Number(event.target.value) })} />
        </label>
        <label className="block">
          <ParameterLabel label="Confidence level" help="The percentage used for coefficient and time-ratio intervals." />
          <input className={field('text', 'mt-1 w-full')} type="number" min={1} max={99.9} step="any" value={draft.confidenceLevel * 100} onChange={(event) => configure({ ...draft, confidenceLevel: Number(event.target.value) / 100 })} />
        </label>
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'cox-regression': {
        const penalty = draft.penalty
        const setObservation = (observation: CoxObservationDraft) => {
          const changed: Draft = { ...draft, observation }
          const reserved = observationColumns(changed)
          configure({ ...draft, observation, covariates: draft.covariates.filter((id) => !reserved.includes(id)) })
        }
        const setWeights = (weights: CoxWeightsDraft) => {
          const changed: Draft = { ...draft, weights }
          const reserved = observationColumns(changed)
          configure({ ...draft, weights, covariates: draft.covariates.filter((id) => !reserved.includes(id)) })
        }
        const setStrata = (strata: CoxStrataDraft) => {
          const changed: Draft = { ...draft, strata }
          const reserved = observationColumns(changed)
          configure({ ...draft, strata, covariates: draft.covariates.filter((id) => !reserved.includes(id)) })
        }
        const chooseObservation = (kind: CoxObservationDraft['kind']) => {
          switch (kind) {
            case 'right-censored': setObservation({ kind, duration: columnLike(columns, /^(time|duration|years?|months?|recyrs)$/i), event: columnLike(columns, /^(event|status|death|censrec)$/i), entry: { kind: 'not-used' }, standardErrors: { kind: 'model-based' }, frailty: { kind: 'none' } }); return
            case 'start-stop': setObservation({ kind, subject: columnLike(columns, /^(subject|id|team|unit)$/i), start: columnLike(columns, /^(start|tstart)$/i), stop: columnLike(columns, /^(stop|tstop)$/i), event: columnLike(columns, /^(event|status|death)$/i) }); return
            default: return assertNever(kind)
          }
        }
        const observationControls = (() => {
          switch (draft.observation.kind) {
            case 'right-censored': {
              const observation = draft.observation
              const chooseEntry = (kind: CoxEntryDraft['kind']) => {
                switch (kind) {
                  case 'not-used': setObservation({ ...observation, entry: { kind } }); return
                  case 'column': setObservation({ ...observation, entry: { kind, column: columnLike(columns, /^(entry|entry_time)$/i) } }); return
                  default: return assertNever(kind)
                }
              }
              const chooseFrailty = (kind: CoxFrailtyDraft['kind']) => {
                switch (kind) {
                  case 'none': setObservation({ ...observation, frailty: { kind } }); return
                  case 'gamma':
                    // survival's frailty() takes model-based errors, no delayed entry and no ridge penalty.
                    configure({
                      ...draft,
                      observation: { ...observation, entry: { kind: 'not-used' }, standardErrors: { kind: 'model-based' }, frailty: { kind, column: columnLike(columns, /^(group|cluster|subject|id|repo|team|unit)$/i), ties: 'efron' } },
                      penalty: { kind: 'unpenalized' },
                      covariates: draft.covariates.filter((id) => !observationColumns({ ...draft, observation: { ...observation, frailty: { kind, column: columnLike(columns, /^(group|cluster|subject|id|repo|team|unit)$/i), ties: 'efron' } } }).includes(id)),
                    })
                    return
                  default: return assertNever(kind)
                }
              }
              const chooseStandardErrors = (kind: CoxStandardErrorsDraft['kind']) => {
                switch (kind) {
                  case 'model-based': setObservation({ ...observation, standardErrors: { kind } }); return
                  case 'robust': setObservation({ ...observation, standardErrors: { kind } }); return
                  case 'clustered': setObservation({ ...observation, standardErrors: { kind, column: columnLike(columns, /^(cluster|subject|id|team|unit)$/i) } }); return
                  default: return assertNever(kind)
                }
              }
              return <>
                <ColumnSelect title="Duration" value={observation.duration} columns={columns} onChange={(duration) => setObservation({ ...observation, duration })} />
                <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={observation.event} columns={columns} onChange={(event) => setObservation({ ...observation, event })} />
                <div>
                  <ParameterLabel label="Shared frailty" help="A gamma frailty gives every observation in a group the same unobserved multiplier on its hazard, fitted as survival's frailty(group, distribution = 'gamma') with its penalised likelihood. Use it when observations are grouped, for example lines within a repository, and the group's own risk is not a covariate." />
                  <SegmentedControl size="sm" ariaLabel="Cox shared frailty" value={observation.frailty.kind} onChange={chooseFrailty} options={[{ value: 'none', label: 'None' }, { value: 'gamma', label: 'Gamma by group' }]} />
                </div>
                {observation.frailty.kind === 'gamma' && <ColumnSelect title="Frailty group" value={observation.frailty.column} columns={columns} onChange={(column) => setObservation({ ...observation, frailty: { kind: 'gamma', column, ties: observation.frailty.kind === 'gamma' ? observation.frailty.ties : 'efron' } })} />}
                {observation.frailty.kind === 'gamma' && <div>
                  <ParameterLabel label="Tied event times" help="Efron's approximation is coxph's default. Breslow's treats every tied event as if it came after the others at that time; choose it to match a study that fitted with ties = 'breslow'." />
                  <SegmentedControl size="sm" ariaLabel="Cox tied event times" value={observation.frailty.ties} onChange={(ties) => setObservation({ ...observation, frailty: { kind: 'gamma', column: observation.frailty.kind === 'gamma' ? observation.frailty.column : null, ties } })} options={[{ value: 'efron', label: 'Efron' }, { value: 'breslow', label: 'Breslow' }]} />
                </div>}
                {observation.frailty.kind === 'none' && <>
                  <div>
                    <ParameterLabel label="Delayed entry" help="Select an entry-time column when an observation joined the risk set after time zero." />
                    <SegmentedControl size="sm" ariaLabel="Cox delayed entry" value={observation.entry.kind} onChange={chooseEntry} options={[{ value: 'not-used', label: 'Not used' }, { value: 'column', label: 'Entry column' }]} />
                  </div>
                  {observation.entry.kind === 'column' && <ColumnSelect title="Entry time" value={observation.entry.column} columns={columns} onChange={(column) => setObservation({ ...observation, entry: { kind: 'column', column } })} />}
                  <div className="sm:col-span-2">
                    <ParameterLabel label="Standard errors" help="Use robust errors for weighted or misspecified models. Use clustered errors when rows within the same cluster may be related." />
                    <SegmentedControl size="sm" ariaLabel="Cox standard errors" value={observation.standardErrors.kind} onChange={chooseStandardErrors} options={[{ value: 'model-based', label: 'Model-based' }, { value: 'robust', label: 'Robust' }, { value: 'clustered', label: 'Clustered' }]} />
                  </div>
                  {observation.standardErrors.kind === 'clustered' && <ColumnSelect title="Cluster" value={observation.standardErrors.column} columns={columns} onChange={(column) => setObservation({ ...observation, standardErrors: { kind: 'clustered', column } })} />}
                </>}
                {observation.frailty.kind === 'gamma' && <p className={cn(fieldHint, 'm-0 sm:col-span-2')}>A shared frailty model uses model-based standard errors, no delayed entry and no penalty.</p>}
              </>
            }
            case 'start-stop': {
              const observation = draft.observation
              return <>
                <ColumnSelect title="Subject" value={observation.subject} columns={columns} onChange={(subject) => setObservation({ ...observation, subject })} />
                <ColumnSelect title="Start time" value={observation.start} columns={columns} onChange={(start) => setObservation({ ...observation, start })} />
                <ColumnSelect title="Stop time" value={observation.stop} columns={columns} onChange={(stop) => setObservation({ ...observation, stop })} />
                <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={observation.event} columns={columns} onChange={(event) => setObservation({ ...observation, event })} />
                <p className={cn(fieldHint, 'm-0 sm:col-span-2')}>Model-based standard errors are used for start–stop Cox regression.</p>
              </>
            }
            default: return assertNever(draft.observation)
          }
        })()
        return <>
          <div className="sm:col-span-2">
            <ParameterLabel label="Observation structure" help="Use right-censored rows when each observation has one duration. Use start–stop rows when a subject contributes intervals with covariates that can change." />
            <SegmentedControl size="sm" ariaLabel="Cox observation structure" value={draft.observation.kind} onChange={chooseObservation} options={[{ value: 'right-censored', label: 'Right-censored' }, { value: 'start-stop', label: 'Start–stop' }]} />
          </div>
          {observationControls}
          <div>
            <ParameterLabel label="Observation weights" help="Select a positive weight column when rows contribute different weights to the partial likelihood." />
            <SegmentedControl size="sm" ariaLabel="Cox observation weights" value={draft.weights.kind} onChange={(kind) => setWeights(kind === 'equal' ? { kind } : { kind, column: columnLike(columns, /^(weight|weights)$/i) })} options={[{ value: 'equal', label: 'Equal' }, { value: 'column', label: 'Weight column' }]} />
          </div>
          {draft.weights.kind === 'column' && <ColumnSelect title="Weight" value={draft.weights.column} columns={columns} onChange={(column) => setWeights({ kind: 'column', column })} />}
          <div>
            <ParameterLabel label="Strata" help="Select a stratum column when groups may have different baseline hazards but share the same covariate coefficients." />
            <SegmentedControl size="sm" ariaLabel="Cox strata" value={draft.strata.kind} onChange={(kind) => setStrata(kind === 'unstratified' ? { kind } : { kind, column: columnLike(columns, /^(strata|stratum|group)$/i) })} options={[{ value: 'unstratified', label: 'Unstratified' }, { value: 'column', label: 'Stratum column' }]} />
          </div>
          {draft.strata.kind === 'column' && <ColumnSelect title="Stratum" value={draft.strata.column} columns={columns} onChange={(column) => setStrata({ kind: 'column', column })} />}
          {!(draft.observation.kind === 'right-censored' && draft.observation.frailty.kind === 'gamma') && <div className="sm:col-span-2">
            <ParameterLabel label="Penalty" help="An elastic-net penalty can stabilize a model with many or strongly related covariates. Leave the model unpenalized unless the study specifies a penalty." />
            <SegmentedControl size="sm" ariaLabel="Cox penalty" value={penalty.kind} onChange={(kind) => configure({ ...draft, penalty: kind === 'unpenalized' ? { kind } : { kind, strength: 0.1, l1Ratio: 0 } })} options={[{ value: 'unpenalized', label: 'Unpenalized' }, { value: 'elastic-net', label: 'Elastic net' }]} />
          </div>}
          {penalty.kind === 'elastic-net' && <>
            <label className="block"><span className={fieldLabel}>Penalty strength</span><input className={field('text', 'mt-1 w-full')} type="number" min={Number.EPSILON} step="any" value={penalty.strength} onChange={(event) => configure({ ...draft, penalty: { ...penalty, strength: Number(event.target.value) } })} /></label>
            <label className="block"><span className={fieldLabel}>L1 ratio</span><input className={field('text', 'mt-1 w-full')} type="number" min={0} max={1} step="any" value={penalty.l1Ratio} onChange={(event) => configure({ ...draft, penalty: { ...penalty, l1Ratio: Number(event.target.value) } })} /></label>
          </>}
          <label className="block">
            <ParameterLabel label="Confidence level" help="The percentage used for coefficient and hazard-ratio intervals." />
            <input className={field('text', 'mt-1 w-full')} type="number" min={1} max={99.9} step="any" value={draft.confidenceLevel * 100} onChange={(event) => configure({ ...draft, confidenceLevel: Number(event.target.value) / 100 })} />
          </label>
        </>
      }
      case 'two-group': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration })} />
        <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 observed, 0 censored</span></Metadata>} value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event })} />
        <ColumnSelect title={<Metadata><span>Group</span><span className="font-normal text-faint">0 or 1</span></Metadata>} value={draft.group} columns={columns} onChange={(group) => configure({ ...draft, group })} />
        <label className="block">
          <ParameterLabel label="Compare through time" help="The follow-up time the restricted mean is taken to. Event-free time is averaged up to here, so the difference is in the duration column's units." />
          <input className={field('text', 'mt-1 w-full')} type="number" min={0.001} step="any" value={draft.truncationTime} aria-label="Compare through time" onChange={(event) => configure({ ...draft, truncationTime: Math.max(0.001, Number(event.target.value) || 10) })} />
        </label>
      </>
      case 'multi-state': {
        const input = draft.input
        const chooseInput = (kind: MultiStateDraftInput['kind']) => {
          switch (kind) {
            case 'prepared-rows': configure({ ...draft, input: preparedMultiStateInput(columns) }); return
            case 'longitudinal-states': configure({ ...draft, input: {
              kind,
              subject: prepared.kind === 'prepared-panel' ? prepared.sampling.unitColumn : columnLike(sourceColumns, /^(subject|id|team|unit)$/i),
              time: prepared.kind === 'prepared-panel' && columns.some((column) => column.id === prepared.sampling.timeColumn) ? prepared.sampling.timeColumn : columnLike(columns, /^(time|month|week|day|year)$/i),
              state: columnLike(columns, /^(state|status)$/i),
              stateCount: 3,
              transitions: defaultTransitions(),
            } }); return
            case 'wide-events': configure({ ...draft, input: { kind, states: [{ kind: 'not-applicable' }, { kind: 'recorded', time: null, status: null }, { kind: 'recorded', time: null, status: null }], transitions: defaultTransitions(), entry: { kind: 'shared', state: 1, time: 0 } } }); return
            default: assertNever(kind)
          }
        }
        const inputControls = (() => {
          switch (input.kind) {
            case 'prepared-rows': return <>
              <ColumnSelect title="Start time" value={input.start} columns={columns} onChange={(start) => configure({ ...draft, input: { ...input, start } })} />
              <ColumnSelect title="Stop time" value={input.stop} columns={columns} onChange={(stop) => configure({ ...draft, input: { ...input, stop } })} />
              <ColumnSelect title={<Metadata><span>Event</span><span className="font-normal text-faint">1 transition, 0 censored</span></Metadata>} value={input.event} columns={columns} onChange={(event) => configure({ ...draft, input: { ...input, event } })} />
              <ColumnSelect title="Origin state" value={input.from} columns={columns} onChange={(from) => configure({ ...draft, input: { ...input, from } })} />
              <ColumnSelect title="Destination state" value={input.to} columns={columns} onChange={(to) => configure({ ...draft, input: { ...input, to } })} />
            </>
            case 'longitudinal-states': return <>
              <p className={cn(fieldHint, 'm-0 sm:col-span-2')}>Use one row per exact state observation. Time keeps the numeric units in the source data; irregular observation times are allowed.</p>
              <ColumnSelect title="Subject" value={input.subject} columns={sourceColumns} onChange={(subject) => configure({ ...draft, input: { ...input, subject } })} />
              <ColumnSelect title="Observation time" value={input.time} columns={columns} onChange={(time) => configure({ ...draft, input: { ...input, time } })} />
              <ColumnSelect title="Observed state" value={input.state} columns={columns} onChange={(state) => configure({ ...draft, input: { ...input, state } })} />
              <TransitionControls stateCount={input.stateCount} transitions={input.transitions} onStateCount={(stateCount) => configure({ ...draft, input: { ...input, stateCount, transitions: input.transitions.filter(({ from, to }) => from <= stateCount && to <= stateCount) } })} onTransitions={(transitions) => configure({ ...draft, input: { ...input, transitions } })} />
            </>
            case 'wide-events': return <>
              <TransitionControls stateCount={input.states.length} transitions={input.transitions} onStateCount={(stateCount) => {
                const states = Array.from({ length: stateCount }, (_, index) => input.states[index] ?? { kind: 'recorded' as const, time: null, status: null })
                configure({ ...draft, input: { ...input, states, transitions: input.transitions.filter(({ from, to }) => from <= stateCount && to <= stateCount) } })
              }} onTransitions={(transitions) => configure({ ...draft, input: { ...input, transitions } })} />
              <div className="grid gap-3 sm:col-span-2">
                {input.states.map((state, index) => (
                  <fieldset key={index} className="m-0 grid gap-3 rounded-md border border-hair p-3 sm:grid-cols-2">
                    <legend className="px-1 text-body font-medium text-ink">State {index + 1}</legend>
                    <label className="flex items-center gap-2 text-body sm:col-span-2"><input type="checkbox" checked={state.kind === 'not-applicable'} onChange={(event) => {
                      const states = input.states.map((candidate, candidateIndex) => candidateIndex === index ? event.target.checked ? { kind: 'not-applicable' as const } : { kind: 'recorded' as const, time: null, status: null } : candidate)
                      configure({ ...draft, input: { ...input, states } })
                    }} />No time/status fields for this state</label>
                    {state.kind === 'recorded' && <>
                      <ColumnSelect title="Time reached or last followed" value={state.time} columns={columns} onChange={(time) => configure({ ...draft, input: { ...input, states: input.states.map((candidate, candidateIndex) => candidateIndex === index && candidate.kind === 'recorded' ? { ...candidate, time } : candidate) } })} />
                      <ColumnSelect title={<Metadata><span>Reached</span><span className="font-normal text-faint">1 yes, 0 censored</span></Metadata>} value={state.status} columns={columns} onChange={(status) => configure({ ...draft, input: { ...input, states: input.states.map((candidate, candidateIndex) => candidateIndex === index && candidate.kind === 'recorded' ? { ...candidate, status } : candidate) } })} />
                    </>}
                  </fieldset>
                ))}
              </div>
              <div className="sm:col-span-2">
                <span className={fieldLabel}>Entry for each subject</span>
                <SegmentedControl size="sm" ariaLabel="Wide event entry mode" value={input.entry.kind} onChange={(kind) => configure({ ...draft, input: { ...input, entry: kind === 'shared' ? { kind, state: 1, time: 0 } : { kind, state: null, time: null } } })} options={[{ value: 'shared', label: 'Same entry' }, { value: 'columns', label: 'Entry columns' }]} />
              </div>
              {input.entry.kind === 'shared' ? <>
                <label className="block"><span className={fieldLabel}>Entry state</span><input className={field('text', 'mt-1 w-full')} type="number" min={1} max={input.states.length} step={1} value={input.entry.state} onChange={(event) => { if (input.entry.kind === 'shared') configure({ ...draft, input: { ...input, entry: { kind: 'shared', state: Number(event.target.value) || 1, time: input.entry.time } } }) }} /></label>
                <label className="block"><span className={fieldLabel}>Entry time</span><input className={field('text', 'mt-1 w-full')} type="number" step="any" value={input.entry.time} onChange={(event) => { if (input.entry.kind === 'shared') configure({ ...draft, input: { ...input, entry: { kind: 'shared', state: input.entry.state, time: Number(event.target.value) || 0 } } }) }} /></label>
              </> : <>
                <ColumnSelect title="Entry state" value={input.entry.state} columns={columns} onChange={(state) => { if (input.entry.kind === 'columns') configure({ ...draft, input: { ...input, entry: { kind: 'columns', state, time: input.entry.time } } }) }} />
                <ColumnSelect title="Entry time" value={input.entry.time} columns={columns} onChange={(time) => { if (input.entry.kind === 'columns') configure({ ...draft, input: { ...input, entry: { kind: 'columns', state: input.entry.state, time } } }) }} />
              </>}
            </>
            default: return assertNever(input)
          }
        })()
        return <>
          <div className="sm:col-span-2">
            <ParameterLabel label="Input rows" help="Use existing transition-risk rows, exact state observations recorded over time, or one wide event-history row per subject. Hirmos validates and expands the last two forms before fitting." />
            <SegmentedControl size="sm" ariaLabel="Multi-state input rows" value={input.kind} onChange={chooseInput} options={[{ value: 'prepared-rows', label: 'Prepared rows' }, { value: 'longitudinal-states', label: 'State observations' }, { value: 'wide-events', label: 'Wide event history' }]} />
          </div>
          {inputControls}
          {familyOptions(draft.family, ['exponential', 'weibullPh', 'gompertz'])}
          {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
        </>
      }
      default: return assertNever(draft)
    }
  })()
  const covariateControls = (() => {
    switch (draft.kind) {
      case 'right-censored':
      case 'start-stop':
      case 'cox-regression':
      case 'aalen':
      case 'survival-forest':
      case 'penalized-aft': {
        const roles = observationColumns(draft)
        const help = draft.kind === 'aalen' || draft.kind === 'survival-forest'
          ? 'Choose at least one covariate. Times and event status cannot also be covariates. For Aalen, encode categories as separate indicator columns with one reference category omitted.'
          : draft.kind === 'cox-regression'
          ? 'Choose at least one covariate. The columns selected for the event-time row roles cannot also be covariates.'
          : draft.kind === 'penalized-aft'
            ? 'Choose at least one covariate. Covariates affect the time to an event; the shape parameter is shared across observations.'
            : 'Each enters the model on the log hazard or log time scale. The columns already chosen as times or the event cannot be covariates.'
        return (
          <div>
            <span className={fieldLabel}>Covariates</span>
            <p className={cn(fieldHint, 'mb-2 max-w-[65ch]')}>{help}</p>
            <div className="flex flex-wrap items-center gap-x-4 gap-y-1.5" role="group" aria-label="Covariates">
              <SelectionActions selectLabel="Select all covariates" clearLabel="Clear selected covariates" onSelectAll={() => configure({ ...draft, covariates: columns.filter((column) => !roles.includes(column.id)).map((column) => column.id) })} onClear={() => configure({ ...draft, covariates: [] })} />
              {columns.map((column) => {
              const reserved = roles.includes(column.id)
              return <label key={column.id} className={cn('flex items-center gap-2 text-body', reserved ? 'text-faint' : 'text-ink')}><input type="checkbox" disabled={reserved} checked={draft.covariates.includes(column.id)} onChange={() => {
                if (reserved) return
                configure({ ...draft, covariates: draft.covariates.includes(column.id) ? draft.covariates.filter((id) => id !== column.id) : [...draft.covariates, column.id] })
              }} />{column.name}</label>
            })}</div>
            {draft.kind === 'survival-forest' ? <fieldset className="mt-3 border-0 p-0"><legend className={fieldLabel}>Categorical covariates</legend><p className={fieldHint}>Mark categorical columns coded with whole numbers from 1 to 53. Unmarked columns use numeric thresholds.</p><div className="flex flex-wrap gap-3">{columns.filter((column) => draft.covariates.includes(column.id)).map((column) => <label key={column.id} className="flex items-center gap-2 text-body"><input type="checkbox" checked={draft.categorical.includes(column.id)} onChange={() => configure({ ...draft, categorical: draft.categorical.includes(column.id) ? draft.categorical.filter((id) => id !== column.id) : [...draft.categorical, column.id] })} />{column.name}</label>)}</div></fieldset> : null}
          </div>
        )
      }
      case 'two-group':
      case 'nonparametric':
      case 'multi-state': return null
      default: return assertNever(draft)
    }
  })()

  const type = analysisType(draft.kind)
  const stage = (
    <section aria-labelledby="survival-title" className="@container/panel flex flex-col gap-5">
      <div>
        <ChapterHeading id="survival-title" className="mb-2">Survival analysis</ChapterHeading>
        <p className={chapterIntro}>Use survival analysis when the outcome is the time until an event. Fit an event-time distribution, compare two observed groups during follow-up, or estimate transitions between states. A row with no observed event by the end of follow-up is right-censored at its recorded duration. These analyses do not require a DAG and are not added to the causal-estimation ledger.</p>
      </div>

      <section className={panel('p-(--panel-space)')} aria-labelledby="survival-setup-title">
        <h3 id="survival-setup-title" className={cn(sectionTitle, 'mb-3 mt-0')}>{type.name}</h3>
        <div className="grid grid-cols-1 gap-4">
          <div>
            <SegmentedControl variant="line" size="sm" ariaLabel="Survival analysis type" value={draft.kind} onChange={selectDraft} options={[{ value: 'right-censored', label: 'Parametric' }, { value: 'nonparametric', label: 'Kaplan–Meier' }, { value: 'start-stop', label: 'Start–stop' }, { value: 'cox-regression', label: 'Cox regression' }, { value: 'aalen', label: 'Aalen regression' }, { value: 'survival-forest', label: 'Survival forest' }, { value: 'penalized-aft', label: 'Penalised AFT' }, { value: 'two-group', label: 'Compare groups' }, { value: 'multi-state', label: 'Multi-state' }]} />
            <p className={cn(fieldHint, 'mt-3 max-w-[65ch]')}>{type.summary}</p>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">{draftControls}</div>
          {covariateControls}
        </div>
        <JobNotice job={job} />
        <div className="mt-4 flex items-center gap-3">
          <button type="button" className={button('signal')} disabled={job.kind === 'running' || session.blocked} onClick={() => void execute()}>{job.kind === 'running' ? 'Running…' : 'Run survival analysis'}</button>
          {job.kind === 'running' && <><Orb state="solving" aria-label="Survival analysis running" /><button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button></>}
        </div>
      </section>

      {latest !== null ? (
        <section aria-labelledby="survival-results-title" className="grid grid-cols-1 gap-4">
          <h2 id="survival-results-title" className={cn(sectionTitle, 'm-0')}>Results</h2>
          <SurvivalRunResult run={latest} current />
        </section>
      ) : (
        <EmptyState>Choose the event-time columns and run an analysis.</EmptyState>
      )}
    </section>
  )

  const inspector = (
    <div className="space-y-6">
      <section aria-labelledby="survival-type-title">
        <h3 id="survival-type-title" className="mb-2 mt-1 text-body font-medium text-ink">{type.name}</h3>
        <p className={cn(fieldHint, 'mt-0')}>{type.summary}</p>
      </section>
      <section aria-label="Method requirements">
        <h3 className="m-0 text-body font-medium text-ink">Requirements</h3>
        <ul className="m-0 mt-2 list-none space-y-4 p-0">
          {type.requirements.map((requirement) => (
            <li key={requirement.holds} className="text-body">
              <p className="m-0 text-ink">{requirement.holds}</p>
              <p className="m-0 font-serif text-faint">If this is not met: {requirement.otherwise}</p>
            </li>
          ))}
        </ul>
      </section>
      <section>
        <h3 className="m-0 text-body font-medium text-ink">Prepared data</h3>
        <dl className="m-0 mt-2 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-1 text-body">
          <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{formatCount(prepared.observations).text}</dd>
          <dt className="text-faint">Columns</dt><dd className={num('m-0 text-ink')}>{formatCount(columns.length).text}</dd>
          <dt className="text-faint">Source</dt><dd className="m-0 break-words text-ink">{source.name}</dd>
        </dl>
      </section>
    </div>
  )

  const ledger = (
    <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Survival runs">
      {runs.length === 0 && <li className="px-3 py-2 text-faint">No survival runs yet.</li>}
      {[...runs].reverse().map((run) => (
        <li key={run.id} className="px-3 py-2">
          <SurvivalRunResult run={run} current={false} open={false} onDelete={() => setPendingDelete(run)} />
        </li>
      ))}
    </ul>
  )

  return (
    <>
      <WorkbenchLayout id="survival" stage={stage} inspector={{ trigger: { label: 'Requirements', icon: 'fact_check' }, title: 'Data and method requirements', body: inspector }} bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Survival runs (${runs.length})`, body: ledger, defaultSize: 150 }} />
      <ConfirmDialog
        open={pendingDelete !== null}
        title="Delete this survival run?"
        danger
        confirmLabel="Delete run"
        message={pendingDelete === null ? '' : `Removes the ${survivalRunSummary(pendingDelete).method} run. It cannot be restored.`}
        onConfirm={() => { if (pendingDelete !== null) onDeleteRun(pendingDelete.id); setPendingDelete(null) }}
        onClose={() => setPendingDelete(null)}
      />
    </>
  )
}
