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
import type { ColumnId, DatasetProfile, NumericColumnSelection } from '@/domain/dataset'
import { assertNever, err, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { chapterLabel } from '@/domain/navigation'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import {
  multiStateSurvivalRun,
  newSurvivalRunId,
  parametricSurvivalFamilySchema,
  proportionalHazardsFamilySchema,
  rightCensoredSurvivalRun,
  startStopSurvivalRun,
  type ParametricSurvivalFamily,
  type ProportionalHazardsFamily,
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
  | { readonly kind: 'multi-state'; readonly start: ColumnId | null; readonly stop: ColumnId | null; readonly event: ColumnId | null; readonly from: ColumnId | null; readonly to: ColumnId | null; readonly family: ProportionalHazardsFamily; readonly horizon: number }

type ReadyDraft =
  | { readonly kind: 'right-censored'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ParametricSurvivalFamily; readonly horizon: number }
  | { readonly kind: 'start-stop'; readonly start: NumericColumnSelection; readonly stop: NumericColumnSelection; readonly event: NumericColumnSelection; readonly covariates: readonly NumericColumnSelection[]; readonly columns: NonEmptyArray<ColumnId>; readonly family: ProportionalHazardsFamily; readonly horizon: number }
  | { readonly kind: 'two-group'; readonly duration: NumericColumnSelection; readonly event: NumericColumnSelection; readonly group: NumericColumnSelection; readonly columns: NonEmptyArray<ColumnId>; readonly truncationTime: number; readonly permutations: number; readonly seed: number }
  | { readonly kind: 'multi-state'; readonly start: NumericColumnSelection; readonly stop: NumericColumnSelection; readonly event: NumericColumnSelection; readonly from: NumericColumnSelection; readonly to: NumericColumnSelection; readonly columns: NonEmptyArray<ColumnId>; readonly family: ProportionalHazardsFamily; readonly horizon: number }

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
const columnLike = (columns: readonly NumericColumnSelection[], pattern: RegExp): ColumnId | null =>
  columns.find((column) => pattern.test(column.name))?.id ?? null

const draftFor = (kind: Draft['kind'], columns: readonly NumericColumnSelection[]): Draft => {
  const duration = columnLike(columns, /^(time|duration|years?|months?|recyrs)$/i)
  const event = columnLike(columns, /^(event|status|death|censrec)$/i)
  switch (kind) {
    case 'right-censored': return { kind, duration, event, covariates: [], family: 'weibull', horizon: 10 }
    case 'start-stop': return { kind, start: columnLike(columns, /^(start|tstart)$/i), stop: columnLike(columns, /^(stop|tstop)$/i), event, covariates: [], family: 'weibullPh', horizon: 10 }
    case 'two-group': return { kind, duration, event, group: columnLike(columns, /^(group|arm|treatment|treat)$/i), truncationTime: 10, permutations: 1_000, seed: 43 }
    case 'multi-state': return { kind, start: columnLike(columns, /^(start|tstart)$/i), stop: columnLike(columns, /^(stop|tstop)$/i), event, from: columnLike(columns, /^from$/i), to: columnLike(columns, /^to$/i), family: 'weibullPh', horizon: 10 }
    default: return assertNever(kind)
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
): Result<ReadyDraft, DraftProblem> => {
  const invalid = (detail: string): Result<never, DraftProblem> =>
    err({ kind: 'invalid-survival-draft', detail })
  const selectedOrProblem = (ids: readonly ColumnId[]) => {
    const selected = selectColumns(columns, ids)
    return selected === null
      ? invalid('A selected column is no longer in the prepared dataset.')
      : ok(selected)
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
      const startId = draft.start
      const stopId = draft.stop
      const eventId = draft.event
      const fromId = draft.from
      const toId = draft.to
      if (startId === null) return invalid('Choose the start time column.')
      if (stopId === null) return invalid('Choose the stop time column.')
      if (eventId === null) return invalid('Choose the event column.')
      if (fromId === null) return invalid('Choose the origin state column.')
      if (toId === null) return invalid('Choose the destination state column.')
      if (new Set([startId, stopId, eventId, fromId, toId]).size !== 5) return invalid('The start, the stop, the event, the origin state and the destination state must be different columns.')
      const ids: NonEmptyArray<ColumnId> = [startId, stopId, eventId, fromId, toId]
      const selected = selectedOrProblem(ids)
      if (!selected.ok) return selected
      const [start, stop, event, from, to] = selected.value
      if (start === undefined || stop === undefined || event === undefined || from === undefined || to === undefined) {
        return invalid('Choose all five multi-state columns.')
      }
      return ok({ ...draft, start, stop, event, from, to, columns: ids })
    }
    default: return assertNever(draft)
  }
}

const observationColumns = (draft: Draft): readonly (ColumnId | null)[] => {
  switch (draft.kind) {
    case 'right-censored': return [draft.duration, draft.event]
    case 'start-stop': return [draft.start, draft.stop, draft.event]
    case 'two-group': return [draft.duration, draft.event, draft.group]
    case 'multi-state': return [draft.start, draft.stop, draft.event, draft.from, draft.to]
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

function ColumnSelect({ title, value, columns, onChange }: { readonly title: string; readonly value: ColumnId | null; readonly columns: readonly NumericColumnSelection[]; readonly onChange: (value: ColumnId | null) => void }) {
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

export function SurvivalPanel({ source, profile, prepared, runs, onRun, onDeleteRun, onActivity }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly runs: readonly SurvivalRunArtifact[]
  readonly onRun: (run: SurvivalRunArtifact) => void
  readonly onDeleteRun: (run: SurvivalRunArtifact['id']) => void
  readonly onActivity?: (activity: RunActivity | null) => void
}) {
  const columns = useMemo(() => profile.columns.filter((column) => prepared.columns.includes(column.id)).map((column) => ({ id: column.id, name: column.name })), [prepared.columns, profile.columns])
  const [state, dispatch] = useReducer(step, columns, (available): State => ({ draft: draftFor('right-censored', available), job: { kind: 'idle' } }))
  const [pendingDelete, setPendingDelete] = useState<SurvivalRunArtifact | null>(null)
  const latest = runs.at(-1) ?? null
  useRunActivity(onActivity, state.job.kind === 'running' ? { label: 'Survival analysis', progress: null } : null)
  const configure = (draft: Draft) => dispatch({ type: 'draft-changed', draft })

  const execute = async () => {
    if (state.job.kind === 'running') return
    const validated = validateDraft(state.draft, columns)
    if (!validated.ok) {
      dispatch({ type: 'run-failed', detail: validated.error.detail })
      return
    }
    dispatch({ type: 'run-started' })
    const fail = (detail: string) => dispatch({ type: 'run-failed', detail })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const identity = (matrixColumns: NonEmptyArray<NumericColumnSelection>) => ({ id: newSurvivalRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), columns: matrixColumns })
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
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          const recorded = rightCensoredSurvivalRun(identity(matrix.columns), { kind: 'right-censored-parametric', duration: draft.duration, event: draft.event, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        case 'start-stop': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const result = await analysis.runFlexSurv(matrix.values, matrix.rowCount, matrix.columns.length, { observation: { kind: 'startStop', start: 0, stop: 1, event: 2 }, covariates: draft.covariates.map((_, index) => index + 3), family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          const recorded = startStopSurvivalRun(identity(matrix.columns), { kind: 'start-stop-proportional-hazards', start: draft.start, stop: draft.stop, event: draft.event, covariates: draft.covariates, family: draft.family, predictionTimes: times }, result.value)
          if (!recorded.ok) { fail('The fitted family does not match the requested family.'); return }
          onRun(recorded.value); break
        }
        case 'two-group': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const result = await analysis.runComparisonSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { duration: 0, event: 1, group: 2, truncationTime: draft.truncationTime, permutations: draft.permutations, seed: draft.seed })
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          onRun({ kind: 'two-group-survival-run', ...identity(matrix.columns), configuration: { kind: 'two-group-comparison', duration: draft.duration, event: draft.event, group: draft.group, truncationTime: draft.truncationTime, permutations: draft.permutations, seed: draft.seed }, evidence: result.value }); break
        }
        case 'multi-state': {
          const draft = validated.value
          const matrix = await materialise(draft.columns); if (matrix === null) return
          const times = predictionTimes(draft.horizon)
          const result = await analysis.runMultiStateSurvival(matrix.values, matrix.rowCount, matrix.columns.length, { start: 0, stop: 1, event: 2, from: 3, to: 4, family: draft.family, predictionTimes: times })
          if (!result.ok) { fail(describeAnalysisWorkerProblem(result.error)); return }
          const recorded = multiStateSurvivalRun(identity(matrix.columns), { kind: 'multi-state-proportional-hazards', start: draft.start, stop: draft.stop, event: draft.event, from: draft.from, to: draft.to, family: draft.family, predictionTimes: times }, result.value)
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
      case 'multi-state': return <>
        <ColumnSelect title="Start time" value={draft.start} columns={columns} onChange={(start) => configure({ ...draft, start })} />
        <ColumnSelect title="Stop time" value={draft.stop} columns={columns} onChange={(stop) => configure({ ...draft, stop })} />
        <ColumnSelect title="Event · 1 transition, 0 censored" value={draft.event} columns={columns} onChange={(event) => configure({ ...draft, event })} />
        <ColumnSelect title="Origin state" value={draft.from} columns={columns} onChange={(from) => configure({ ...draft, from })} />
        <ColumnSelect title="Destination state" value={draft.to} columns={columns} onChange={(to) => configure({ ...draft, to })} />
        {familyOptions(draft.family, ['exponential', 'weibullPh', 'gompertz'])}
        {horizonControl(draft.horizon, (horizon) => configure({ ...draft, horizon }))}
      </>
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
