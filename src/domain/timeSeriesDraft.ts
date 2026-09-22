import type { ColumnId } from './dataset'
import { assertNever, type NonEmptyArray } from './dop'
import type { CountSeriesLink } from './countSeries'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { ArdlTerms, TimeSeriesRun } from './timeSeries'
import type { Workflow } from './workflow'

export type Role = { readonly kind: 'unused' } | { readonly kind: 'predictor'; readonly lag: string } | { readonly kind: 'fixed' }
export type Future = { readonly kind: 'none' } | { readonly kind: 'scenario'; readonly columns: Readonly<Record<string, string>> }
export type Mode = 'fixed' | 'search' | 'rFixed' | 'rHorizontal' | 'rGrid'
export interface ArdlDraft {
  readonly outcome: ColumnId | null
  readonly roles: Readonly<Record<string, Role>>
  readonly mode: Mode
  readonly starting: Readonly<Record<string, string>>
  readonly fixedOrders: Readonly<Record<string, string>>
  readonly minimum: string
  readonly outcomeLag: string
  readonly holdBack: string
  readonly terms: ArdlTerms
  readonly horizon: string
  readonly future: Future
}
export interface LongRunDraft {
  readonly selected: readonly ColumnId[]
  readonly maxLag: number
  readonly terms: ArdlTerms
  readonly deterministic: 'n' | 'co' | 'ci' | 'coli'
  readonly significance: 90 | 95 | 99
  readonly forecastSteps: string
}
export interface CountDraft {
  readonly outcome: ColumnId | null
  readonly link: CountSeriesLink
  readonly pastObservationLags: NonEmptyArray<number>
  readonly pastMeanLags: NonEmptyArray<number>
  readonly candidateStart: number
  readonly candidateEnd: number
  readonly delta: number
}
/** Field text as typed; the panel parses it when the run is requested. Each choice keeps only its own settings. */
export type ContinuousErrorsDraft =
  | { readonly kind: 'neweyWest'; readonly maxLags: string }
  | { readonly kind: 'arma'; readonly p: string; readonly q: string; readonly maxIter: string }
export type InterruptedModelDraft =
  | { readonly kind: 'continuous'; readonly errors: ContinuousErrorsDraft }
  | { readonly kind: 'count'; readonly exposure: ColumnId | null }
export type InterruptedImpactDraft =
  | { readonly kind: 'level' }
  | { readonly kind: 'levelAndSlope' }
  | { readonly kind: 'slope' }
  | { readonly kind: 'temporaryLevel'; readonly until: string }
export interface InterruptedDraft {
  readonly outcome: ColumnId | null
  readonly model: InterruptedModelDraft
  readonly interventionRow: string
  readonly lag: string
  readonly impact: InterruptedImpactDraft
  readonly harmonicPairs: string
}
export interface TimeSeriesDraft {
  readonly prepared: PreparedDatasetVersionId
  readonly analysis: 'count' | 'ardl' | 'vecm' | 'interrupted'
  readonly ardl: ArdlDraft
  readonly longRun: Readonly<Record<'ardl' | 'vecm', LongRunDraft>>
  readonly count: CountDraft
  readonly interrupted: InterruptedDraft
}
type FieldEvent<T, Kind extends string> = { [K in keyof T]: { readonly type: Kind; readonly field: K; readonly value: T[K] } }[keyof T]
export type TimeSeriesEvent =
  | { readonly type: 'analysis'; readonly analysis: TimeSeriesDraft['analysis'] }
  | FieldEvent<ArdlDraft, 'ardl'>
  | FieldEvent<CountDraft, 'count'>
  | FieldEvent<InterruptedDraft, 'interrupted'>
  | (FieldEvent<LongRunDraft, 'long-run'> & { readonly model: 'ardl' | 'vecm' })

function longRunDraft(model: 'ardl' | 'vecm', runs: readonly TimeSeriesRun[]): LongRunDraft {
  const previous = runs.filter((run): run is Extract<TimeSeriesRun, { kind: 'ardl' | 'vecm' }> => run.kind === model).at(-1)
  return {
    selected: previous === undefined ? [] : previous.kind === 'ardl' ? [previous.outcome.id, previous.predictor.id] : previous.variables.map(variable => variable.id),
    maxLag: previous === undefined ? 2 : previous.kind === 'ardl' ? previous.specification.maxLag : previous.specification.maxLags,
    terms: previous?.kind === 'ardl' ? previous.specification.terms : 'constant',
    deterministic: previous?.kind === 'vecm' ? previous.specification.deterministic : 'ci',
    significance: previous?.kind === 'vecm' ? previous.specification.significance : 95,
    forecastSteps: '',
  }
}

export function retainTimeSeriesDraft(current: TimeSeriesDraft | null, workflow: Workflow): TimeSeriesDraft | null {
  if (workflow.kind !== 'profiled' || workflow.prepared?.kind !== 'prepared-time-series') return null
  const prepared = workflow.prepared
  if (current?.prepared === prepared.id) return current
  return {
    prepared: prepared.id, analysis: 'count',
    ardl: { outcome: null, roles: {}, mode: 'search', starting: {}, fixedOrders: {}, minimum: '1', outcomeLag: '2', holdBack: '', terms: 'constant', horizon: '12', future: { kind: 'none' } },
    longRun: { ardl: longRunDraft('ardl', workflow.timeSeriesRuns), vecm: longRunDraft('vecm', workflow.timeSeriesRuns) },
    count: { outcome: null, link: 'identity', pastObservationLags: [1], pastMeanLags: [1], candidateStart: Math.max(1, Math.floor(prepared.observations * 0.2)), candidateEnd: Math.max(1, Math.floor(prepared.observations * 0.8)), delta: 1 },
    interrupted: { outcome: null, model: { kind: 'continuous', errors: { kind: 'neweyWest', maxLags: '' } }, interventionRow: '', lag: '0', impact: { kind: 'level' }, harmonicPairs: '2' },
  }
}

export function stepTimeSeriesDraft(state: TimeSeriesDraft, event: TimeSeriesEvent): TimeSeriesDraft {
  switch (event.type) {
    case 'analysis': return { ...state, analysis: event.analysis }
    case 'ardl': return { ...state, ardl: { ...state.ardl, [event.field]: event.value } }
    case 'count': return { ...state, count: { ...state.count, [event.field]: event.value } }
    case 'interrupted': return { ...state, interrupted: { ...state.interrupted, [event.field]: event.value } }
    case 'long-run': return { ...state, longRun: { ...state.longRun, [event.model]: { ...state.longRun[event.model], [event.field]: event.value } } }
    default: return assertNever(event)
  }
}
