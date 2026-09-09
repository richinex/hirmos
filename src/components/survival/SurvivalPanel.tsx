import { useMemo, useReducer, useState } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { SurvivalRunResult, survivalFamilyLabel, survivalRunSummary } from '@/components/survival/SurvivalRunResult'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { EmptyState } from '@/components/ui/EmptyState'
import { Orb } from '@/components/ui/Orb'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { button, chapterIntro, field, fieldHint, fieldLabel, label, num, panel, sectionTitle } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { RunActivity } from '@/domain/activity'
import { isNumericDuckDbType, type ColumnId, type ColumnSelection, type DatasetProfile, type NumericColumnSelection } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { chapterLabel } from '@/domain/navigation'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
  describeSurvivalRefusal,
  multiStateSurvivalRun,
  newSurvivalRunId,
  parametricSurvivalFamilySchema,
  proportionalHazardsFamilySchema,
  rightCensoredSurvivalRun,
  startStopSurvivalRun,
  type ParametricSurvivalFamily,
  type ProportionalHazardsFamily,
  type MultiStateInputConfiguration,
  type SurvivalRunArtifact,
} from '@/domain/survival'
import type { SelectedSource } from '@/domain/workflow'
import { useRunActivity } from '@/lib/useRunActivity'
import { formatCount } from '@/lib/format/number'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type Draft =
  | { readonly kind: 'right-censored'; readonly duration: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'start-stop'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly covariates: readonly ColumnId[]; readonly family: ProportionalHazardsFamily; readonly horizon: number }
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

type ReadyDraft =
  | { readonly kind: 'right-censored'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'start-stop'; readonly start: NumericColumnSelection; readonly stop: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ProportionalHazardsFamily; readonly horizon: number }
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

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running' }
  | { readonly kind: 'failed'; readonly detail: string }

interface State { readonly draft: Draft; readonly job: Job }
type Event =
  | { readonly type: 'draft-changed'; readonly draft: Draft }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'draft-changed': return { ...state, draft: event.draft, job: { kind: 'idle' } }
    case 'run-started': return { ...state, job: { kind: 'running' } }
    case 'run-failed': return { ...state, job: { kind: 'failed', detail: event.detail } }
    case 'run-finished': return { ...state, job: { kind: 'idle' } }
    default: return assertNever(event)
  }
}

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
    case 'right-censored': return { kind, duration, event, covariates: [], family: 'weibull', horizon: 10 }
    case 'start-stop': return { kind, start: columnLike(columns, /^(start|tstart)$/i), stop: columnLike(columns, /^(stop|tstop)$/i), event, covariates: [], family: 'weibullPh', horizon: 10 }
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
    case 'right-censored-parametric':
      return { kind: 'right-censored', duration: present(configuration.duration), event: present(configuration.event), covariates: presentAll(configuration.covariates), family: configuration.family, horizon: horizon(configuration.predictionTimes) }
    case 'start-stop-proportional-hazards':
      return { kind: 'start-stop', start: present(configuration.start), stop: present(configuration.stop), event: present(configuration.event), covariates: presentAll(configuration.covariates), family: configuration.family, horizon: horizon(configuration.predictionTimes) }
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

  switch (draft.kind) {
    case 'right-censored': {
      const durationId = draft.duration
      const eventId = draft.event
      if (durationId === null) return invalid('Choose the duration column.')
      if (eventId === null) return invalid('Choose the event column.')
      if (new Set([durationId, eventId, ...draft.covariates]).size !== 2 + draft.covariates.length) {
        return invalid('The duration, the event and each covariate must be different columns.')
      }
      const ids: NonEmptyArray<ColumnId> = [durationId, eventId, ...draft.covariates]
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [duration, event, ...covariates] = selected.value
      if (duration === undefined || event === undefined) return invalid('Choose duration and event columns.')
      return ok({ ...draft, duration, event, covariates, columns: ids })
    }
    case 'start-stop': {
      const startId = draft.start
      const stopId = draft.stop
      const eventId = draft.event
      if (startId === null) return invalid('Choose the start time column.')
      if (stopId === null) return invalid('Choose the stop time column.')
      if (eventId === null) return invalid('Choose the event column.')
      if (new Set([startId, stopId, eventId, ...draft.covariates]).size !== 3 + draft.covariates.length) {
        return invalid('The start, the stop, the event and each covariate must be different columns.')
      }
      const ids: NonEmptyArray<ColumnId> = [startId, stopId, eventId, ...draft.covariates]
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [start, stop, event, ...covariates] = selected.value
      if (start === undefined || stop === undefined || event === undefined) {
        return invalid('Choose start, stop, and event columns.')
      }
      return ok({ ...draft, start, stop, event, covariates, columns: ids })
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
    case 'right-censored': return [draft.duration, draft.event]
    case 'start-stop': return [draft.start, draft.stop, draft.event]
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

interface AnalysisType {
  readonly name: string
  readonly summary: string
  readonly requirements: readonly { readonly holds: string; readonly otherwise: string }[]
}

const analysisType = (kind: Draft['kind']): AnalysisType => {
  switch (kind) {
    case 'right-censored': return {
      name: 'Parametric survival',
      summary: 'One duration and one event indicator per row. The chosen family gives the event-time distribution, and each covariate shifts it; the fitted curve, hazard and median follow from the parameters.',
      requirements: [
        { holds: 'Each row is one observation, with its duration measured from its own time zero and an event flag of 1 for observed or 0 for censored.', otherwise: 'a row that is really several spells, or a flag coded the other way round, fits the wrong likelihood without any warning.' },
        { holds: 'Censoring is unrelated to the event beyond the covariates in the model.', otherwise: 'rows that leave because they are about to fail make the fitted curve too optimistic.' },
        { holds: 'The family describes how the hazard changes over follow-up.', otherwise: 'a constant-hazard family on a rising risk misplaces the median and the tail; compare AIC across families.' },
        { holds: 'The intervals are for the parameters, not for the curve.', otherwise: 'the curve is read as more certain than it is; a band around it is not yet drawn.' },
      ],
    }
    case 'start-stop': return {
      name: 'Start–stop survival',
      summary: 'Each row covers one interval during which its covariates stay fixed, and the row had already survived to its start. A covariate that changes mid-spell becomes several rows. Covariates multiply the hazard.',
      requirements: [
        { holds: 'Each row is one interval of one observation, with start before stop, and the covariates constant inside it.', otherwise: 'the left-truncation term credits survival that did not happen, or a covariate is read at the wrong value.' },
        { holds: 'The intervals of one observation do not overlap and the event flag is 1 only on the interval where the event occurred.', otherwise: 'the observation is counted more than once and its event is double-counted or lost.' },
        { holds: 'Covariates multiply the hazard, which is the proportional-hazards form; accelerated-time families are not offered on start–stop rows.', otherwise: 'a covariate that stretches time rather than scaling risk is misread as a hazard ratio.' },
        { holds: 'Censoring is unrelated to the event beyond the covariates in the model.', otherwise: 'rows that leave because they are about to fail make the fitted curve too optimistic.' },
      ],
    }
    case 'two-group': return {
      name: 'Two-group comparison',
      summary: 'The observed event-free curves of two groups, compared across follow-up with tests that stay valid when the curves cross, and the difference in average event-free time through a chosen horizon.',
      requirements: [
        { holds: 'The group column holds 0 and 1 only, and each row is an independent observation.', otherwise: 'a third value or a repeated unit leaves the tests without their sampling model.' },
        { holds: 'Censoring is comparable between the groups.', otherwise: 'a group that drops out earlier looks better or worse than it is.' },
        { holds: 'The comparison time sits inside the follow-up both groups reach.', otherwise: 'the restricted mean is extrapolated past the last observed event.' },
        { holds: 'The result describes two groups; it is an effect only when group assignment and the study design support that reading.', otherwise: 'a difference that comes from who ended up in each group is reported as if the group caused it.' },
      ],
    }
    case 'multi-state': return {
      name: 'Multi-state survival',
      summary: 'Each observed transition receives its own cause-specific proportional-hazards model, then the transition system is combined into the chance of occupying each state over follow-up.',
      requirements: [
        { holds: 'Every permitted move appears as a row with its origin and destination states, and the event flag is 1 only for the transition that happened.', otherwise: 'a move the data never records cannot be estimated and the occupancy curves omit it.' },
        { holds: 'Future movement depends on the current state and the time since entry as the clock-forward Markov model specifies.', otherwise: 'a history the model cannot see shifts the estimated occupancy.' },
        { holds: 'Transition hazards follow the chosen proportional-hazards family.', otherwise: 'a constant-hazard family on a rising transition risk misplaces the occupancy curves.' },
      ],
    }
    default: return assertNever(kind)
  }
}

function ColumnSelect({ title, value, columns, onChange }: { readonly title: string; readonly value: ColumnId | null; readonly columns: readonly ColumnSelection[]; readonly onChange: (value: ColumnId | null) => void }) {
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
            <label><span className={fieldHint}>From</span><Select className={field('text', 'mt-1 w-20')} value={candidate.from} onChange={(event) => setCandidate({ ...candidate, from: Number(event.target.value) })}>{Array.from({ length: stateCount }, (_, index) => <option key={index + 1} value={index + 1}>{index + 1}</option>)}</Select></label>
            <label><span className={fieldHint}>To</span><Select className={field('text', 'mt-1 w-20')} value={candidate.to} onChange={(event) => setCandidate({ ...candidate, to: Number(event.target.value) })}>{Array.from({ length: stateCount }, (_, index) => <option key={index + 1} value={index + 1}>{index + 1}</option>)}</Select></label>
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

export function SurvivalPanel({ source, profile, prepared, runs, onRun, onDeleteRun, onActivity }: {
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
  const [state, dispatch] = useReducer(step, columns, (available): State => {
    const recorded = runs.at(-1)
    return { draft: recorded === undefined ? draftFor('right-censored', available) : draftFromRun(recorded, available, sourceColumns), job: { kind: 'idle' } }
  })
  const [pendingDelete, setPendingDelete] = useState<SurvivalRunArtifact | null>(null)
  const latest = runs.at(-1) ?? null
  useRunActivity(onActivity, state.job.kind === 'running' ? { label: 'Survival analysis', progress: null } : null)
  const configure = (draft: Draft) => dispatch({ type: 'draft-changed', draft })

  const execute = async () => {
    if (state.job.kind === 'running') return
    const validated = validateDraft(state.draft, columns, sourceColumns)
    if (!validated.ok) {
      dispatch({ type: 'run-failed', detail: validated.error.detail })
      return
    }
    dispatch({ type: 'run-started' })
    const fail = (detail: string) => dispatch({ type: 'run-failed', detail })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const identity = (matrixColumns: NonEmptyArray<ColumnSelection>) => ({ id: newSurvivalRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), columns: matrixColumns })
      const materialise = async (ids: NonEmptyArray<ColumnId>) => {
        const matrix = await materialisePrepared(source, profile, prepared, ids)
        if (!matrix.ok) { fail(describePreparedMaterialisationProblem(matrix.error)); return null }
        return matrix.value
      }
      switch (validated.value.kind) {
        case 'right-censored': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const result = await analysis.runFlexSurv(matrix.values, matrix.rowCount, matrix.columns.length, { observation: { kind: 'rightCensored', duration: 0, event: 1 }, covariates: draft.covariates.map((_, index) => index + 2), family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = rightCensoredSurvivalRun(identity(matrix.columns), { kind: 'right-censored-parametric', duration: draft.duration, event: draft.event, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        case 'start-stop': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const result = await analysis.runFlexSurv(matrix.values, matrix.rowCount, matrix.columns.length, { observation: { kind: 'startStop', start: 0, stop: 1, event: 2 }, covariates: draft.covariates.map((_, index) => index + 3), family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeSurvivalRefusal(describeAnalysisWorkerProblem(result.error))); return }
          const recorded = startStopSurvivalRun(identity(matrix.columns), { kind: 'start-stop-proportional-hazards', start: draft.start, stop: draft.stop, event: draft.event, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
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
              const keys = await materializePanelKeysInWorker(source.file, profile, input.subject.id, input.time.id)
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
      dispatch({ type: 'run-finished' })
    } catch (cause: unknown) {
      console.error('Unexpected survival analysis failure', cause)
      fail('Survival analysis stopped unexpectedly. Try the run again; if it continues, report the problem.')
    }
  }

  const draft = state.draft
  const selectDraft = (kind: Draft['kind']) => configure(draftFor(kind, columns))
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
      case 'right-censored': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration, covariates: withoutCovariate(draft.covariates, duration) })} />
        <ColumnSelect title="Event · 1 observed, 0 censored" value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        {familyOptions(draft.family, ['exponential', 'weibull', 'weibullPh', 'logNormal', 'gamma', 'gompertz', 'logLogistic', 'generalizedGamma', 'generalizedF'])}
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'start-stop': return <>
        <ColumnSelect title="Start time" value={draft.start} columns={columns} onChange={(start) => configure({ ...draft, start, covariates: withoutCovariate(draft.covariates, start) })} />
        <ColumnSelect title="Stop time" value={draft.stop} columns={columns} onChange={(stop) => configure({ ...draft, stop, covariates: withoutCovariate(draft.covariates, stop) })} />
        <ColumnSelect title="Event · 1 observed, 0 censored" value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event, covariates: withoutCovariate(draft.covariates, event) })} />
        {familyOptions(draft.family, ['exponential', 'weibullPh', 'gompertz'])}
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
      case 'two-group': return <>
        <ColumnSelect title="Duration" value={draft.duration} columns={columns} onChange={(duration) => configure({ ...draft, duration })} />
        <ColumnSelect title="Event · 1 observed, 0 censored" value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event })} />
        <ColumnSelect title="Group · 0 or 1" value={draft.group} columns={columns} onChange={(group) => configure({ ...draft, group })} />
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
              <ColumnSelect title="Event · 1 transition, 0 censored" value={input.event} columns={columns} onChange={(event) => configure({ ...draft, input: { ...input, event } })} />
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
                      <ColumnSelect title="Reached · 1 yes, 0 censored" value={state.status} columns={columns} onChange={(status) => configure({ ...draft, input: { ...input, states: input.states.map((candidate, candidateIndex) => candidateIndex === index && candidate.kind === 'recorded' ? { ...candidate, status } : candidate) } })} />
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
            <SegmentedControl wrap size="sm" ariaLabel="Multi-state input rows" value={input.kind} onChange={chooseInput} options={[{ value: 'prepared-rows', label: 'Prepared rows' }, { value: 'longitudinal-states', label: 'State observations' }, { value: 'wide-events', label: 'Wide event history' }]} />
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
      case 'start-stop': {
        const roles = observationColumns(draft)
        return (
          <fieldset className="m-0 border-0 p-0">
            <legend className={fieldLabel}>Covariates</legend>
            <p className={cn(fieldHint, 'mb-2 max-w-[65ch]')}>Each enters the model on the log hazard or log time scale. The columns already chosen as times or the event cannot be covariates.</p>
            <div className="flex flex-wrap gap-x-4 gap-y-1.5">{columns.map((column) => {
              const reserved = roles.includes(column.id)
              return <label key={column.id} className={cn('flex items-center gap-2 text-body', reserved ? 'text-faint' : 'text-ink')}><input type="checkbox" disabled={reserved} checked={draft.covariates.includes(column.id)} onChange={() => {
                if (reserved) return
                configure({ ...draft, covariates: draft.covariates.includes(column.id) ? draft.covariates.filter((id) => id !== column.id) : [...draft.covariates, column.id] })
              }} />{column.name}</label>
            })}</div>
          </fieldset>
        )
      }
      case 'two-group':
      case 'multi-state': return null
      default: return assertNever(draft)
    }
  })()

  const type = analysisType(draft.kind)
  const stage = (
    <section aria-labelledby="survival-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>{chapterLabel('survival')}</span>
        <h2 id="survival-title" className="mb-2 mt-2 text-heading text-ink">Time until an event</h2>
        <p className={chapterIntro}>Some outcomes are not a level but a wait: how long until a team drops a tool, a ticket closes, a customer leaves. In this chapter, fit the distribution of that time, compare two observed groups across follow-up, or estimate movement between states. Rows still waiting when the data ends are used as far as they go, never counted as failures. These analyses do not need a DAG and are not added to the causal-estimation ledger.</p>
      </div>

      <section className={panel('p-(--panel-space)')} aria-labelledby="survival-setup-title">
        <h3 id="survival-setup-title" className={cn(sectionTitle, 'mb-3 mt-0')}>{type.name}</h3>
        <div className="grid grid-cols-1 gap-4">
          <div>
            <SegmentedControl wrap size="sm" ariaLabel="Survival analysis type" value={draft.kind} onChange={selectDraft} options={[{ value: 'right-censored', label: 'Parametric' }, { value: 'start-stop', label: 'Start–stop' }, { value: 'two-group', label: 'Compare groups' }, { value: 'multi-state', label: 'Multi-state' }]} />
            <p className={cn(fieldHint, 'mt-3 max-w-[65ch]')}>{type.summary}</p>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">{draftControls}</div>
          {covariateControls}
        </div>
        {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The analysis could not run: {state.job.detail}</p></Alert>}
        <div className="mt-4 flex items-center gap-3">
          <button type="button" className={button('signal')} disabled={state.job.kind === 'running'} onClick={() => void execute()}>{state.job.kind === 'running' ? 'Running…' : 'Run survival analysis'}</button>
          {state.job.kind === 'running' && <><Orb state="solving" aria-label="Survival analysis running" /><button type="button" className={button('quiet')} onClick={() => void import('@/analysis/client').then(({ cancelAnalysisRuns }) => cancelAnalysisRuns()).then(() => dispatch({ type: 'run-failed', detail: 'The survival run was cancelled.' }))}>Cancel run</button></>}
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
    <div className="space-y-4">
      <section aria-labelledby="survival-type-title">
        <h3 id="survival-type-title" className="mb-2 mt-1 text-body font-medium text-ink">{type.name}</h3>
        <p className={cn(fieldHint, 'mt-0')}>{type.summary}</p>
      </section>
      <section className="border-t border-hair pt-3" aria-label="Method requirements">
        <h3 className="m-0 text-body font-medium text-ink">Requirements</h3>
        <ul className="m-0 mt-2 list-none space-y-2 p-0">
          {type.requirements.map((requirement) => (
            <li key={requirement.holds} className="text-body">
              <p className="m-0 text-ink">{requirement.holds}</p>
              <p className="m-0 font-serif text-faint">If this is not met: {requirement.otherwise}</p>
            </li>
          ))}
        </ul>
      </section>
      <section className="border-t border-hair pt-3">
        <h3 className="m-0 text-body font-medium text-ink">Prepared data</h3>
        <dl className="m-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
          <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{formatCount(prepared.observations).text}</dd>
          <dt className="text-faint">Columns</dt><dd className={num('m-0 text-ink')}>{formatCount(columns.length).text}</dd>
          <dt className="text-faint">Source</dt><dd className="m-0 truncate text-ink">{source.name}</dd>
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
      <WorkbenchLayout id="survival" stage={stage} inspector={{ title: 'Data and method requirements', body: inspector }} bottom={{ title: `Survival runs · ${runs.length}`, body: ledger, defaultSize: 150 }} />
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
