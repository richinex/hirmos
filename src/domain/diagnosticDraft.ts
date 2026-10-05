import type { ColumnId } from './dataset'
import { assertNever } from './dop'
import { GRANGER_LAG_OPTIONS, type GrangerLag } from './granger'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'

interface RedundancyDraft {
  readonly correlation: number
  readonly vif: number
  readonly pair: readonly [number, number]
}

interface Identity {
  readonly prepared: PreparedDatasetVersionId
  readonly redundancy: RedundancyDraft
}

export type DiagnosticDraft = Identity &
  (
    | { readonly kind: 'observations'; readonly view: 'multicollinearity' }
    | {
        readonly kind: 'series'
        readonly view: 'multicollinearity' | 'stationarity' | 'structure' | 'granger'
        readonly stationarity: readonly ColumnId[]
        readonly temporal: { readonly minSize: number; readonly maxLag: number }
        readonly granger: {
          readonly cause: ColumnId | null
          readonly target: ColumnId | null
          readonly maxLag: GrangerLag
        }
      }
  )

export type DiagnosticEvent =
  | { readonly type: 'view'; readonly view: DiagnosticDraft['view'] }
  | { readonly type: 'stationarity'; readonly columns: readonly ColumnId[] }
  | { readonly type: 'min-size'; readonly value: number }
  | { readonly type: 'max-lag'; readonly value: number }
  | { readonly type: 'correlation'; readonly value: number }
  | { readonly type: 'vif'; readonly value: number }
  | { readonly type: 'pair'; readonly value: readonly [number, number] }
  | {
      readonly type: 'granger-column'
      readonly role: 'cause' | 'target'
      readonly column: ColumnId | null
    }
  | { readonly type: 'granger-lag'; readonly value: GrangerLag }

export function initialDiagnosticDraft(prepared: PreparedDatasetArtifact): DiagnosticDraft {
  const common = {
    prepared: prepared.id,
    redundancy: {
      correlation: 0.9,
      vif: 10,
      pair: [0, Math.min(1, prepared.columns.length - 1)] as const,
    },
  }
  return prepared.kind === 'prepared-time-series'
    ? {
        ...common,
        kind: 'series',
        view: 'multicollinearity',
        stationarity: [],
        temporal: { minSize: 4, maxLag: 40 },
        granger: { cause: null, target: null, maxLag: 4 },
      }
    : { ...common, kind: 'observations', view: 'multicollinearity' }
}

export function stepDiagnosticDraft(
  state: DiagnosticDraft,
  event: DiagnosticEvent,
  prepared: PreparedDatasetArtifact,
): DiagnosticDraft {
  if (state.prepared !== prepared.id) return state
  switch (event.type) {
    case 'granger-column':
      return state.kind === 'series' &&
        (event.column === null || prepared.columns.includes(event.column))
        ? { ...state, granger: { ...state.granger, [event.role]: event.column } }
        : state
    case 'granger-lag':
      return state.kind === 'series' && GRANGER_LAG_OPTIONS.includes(event.value)
        ? { ...state, granger: { ...state.granger, maxLag: event.value } }
        : state
    case 'view':
      return state.kind === 'series' ? { ...state, view: event.view } : state
    case 'stationarity':
      return state.kind === 'series'
        ? {
            ...state,
            stationarity: [...new Set(event.columns)].filter((column) =>
              prepared.columns.includes(column),
            ),
          }
        : state
    case 'min-size':
      return state.kind === 'series' &&
        Number.isInteger(event.value) &&
        event.value >= 1 &&
        event.value <= 200
        ? { ...state, temporal: { ...state.temporal, minSize: event.value } }
        : state
    case 'max-lag':
      return state.kind === 'series' &&
        Number.isInteger(event.value) &&
        event.value >= 1 &&
        event.value <= 400
        ? { ...state, temporal: { ...state.temporal, maxLag: event.value } }
        : state
    case 'correlation':
      return Number.isFinite(event.value) && event.value >= 0.01 && event.value <= 1
        ? { ...state, redundancy: { ...state.redundancy, correlation: event.value } }
        : state
    case 'vif':
      return Number.isFinite(event.value) && event.value >= 1.01
        ? { ...state, redundancy: { ...state.redundancy, vif: event.value } }
        : state
    case 'pair':
      return event.value.every(
        (index) => Number.isInteger(index) && index >= 0 && index < prepared.columns.length,
      )
        ? { ...state, redundancy: { ...state.redundancy, pair: event.value } }
        : state
    default:
      return assertNever(event)
  }
}
