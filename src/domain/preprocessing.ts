import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { ColumnId, DatasetProfile } from './dataset'
import type { StationarityBattery } from './stationarity'

export type PreparedDatasetVersionId = Brand<string, 'PreparedDatasetVersionId'>
export type TransformRecipeId = Brand<string, 'TransformRecipeId'>
export type StationarityEvidenceId = Brand<string, 'StationarityEvidenceId'>

export type Frequency = 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly'

export type SamplingDraft =
  | { readonly kind: 'unconfigured' }
  | { readonly kind: 'cross-sectional' }
  | { readonly kind: 'regular-series-awaiting-time'; readonly frequency: Frequency }
  | { readonly kind: 'regular-series'; readonly timeColumn: ColumnId; readonly frequency: Frequency }

export type VariableDraft =
  | { readonly kind: 'empty' }
  | { readonly kind: 'selected'; readonly columns: NonEmptyArray<ColumnId> }

export type MissingnessDraft =
  | { readonly kind: 'not-present' }
  | { readonly kind: 'unresolved'; readonly cells: number }
  | {
      readonly kind: 'tigramite-mask'
      readonly cells: number
      readonly cutOff: 'methodDefault' | '2xtau_max' | 'tau_max' | 'max_lag' | 'max_lag_or_tau_max' | '2xtau_max_future'
      readonly propagateThroughMaxLag: boolean
      readonly maskType: 'none' | 'x' | 'y' | 'z' | 'xy' | 'xz' | 'yz' | 'xyz'
    }
  | { readonly kind: 'complete-interval'; readonly cells: number }
  | {
      readonly kind: 'imputation'
      readonly cells: number
      readonly method: 'linearInterior' | 'forwardFill' | 'structuralZero'
      readonly maxGap: number
      readonly confirmedStructuralZero: boolean
    }

export type SeriesTransform =
  | { readonly kind: 'levels' }
  | { readonly kind: 'difference'; readonly order: 1 }
  | { readonly kind: 'linear-detrend' }

export type PreparationJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running' }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'succeeded'; readonly artifact: PreparedDatasetArtifact }

export type StationarityJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly completed: number; readonly total: number }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'succeeded'; readonly evidence: StationarityEvidenceArtifact }

export interface PreprocessingDraft {
  readonly sampling: SamplingDraft
  readonly variables: VariableDraft
  readonly missingness: MissingnessDraft
  readonly transform: SeriesTransform
  readonly preparation: PreparationJob
  readonly stationarity: StationarityJob
}

type ReadyMissingness = Exclude<MissingnessDraft, { readonly kind: 'unresolved' }>

export type ReadyPreprocessingRecipe =
  | {
      readonly kind: 'regular-series'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-series' }>
      readonly columns: NonEmptyArray<ColumnId>
      readonly missingness: ReadyMissingness
    }
  | {
      readonly kind: 'cross-sectional'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'cross-sectional' }>
      readonly columns: NonEmptyArray<ColumnId>
      readonly missingness: ReadyMissingness
    }

export interface VariableStationarityEvidence {
  readonly column: ColumnId
  readonly result: StationarityBattery
}

interface PreparedDatasetIdentity {
  readonly id: PreparedDatasetVersionId
  readonly recipe: TransformRecipeId
  readonly sourceProfile: DatasetProfile['id']
  readonly observations: number
  readonly columns: NonEmptyArray<ColumnId>
}

export type PreparedDatasetArtifact =
  | PreparedDatasetIdentity & {
      readonly kind: 'prepared-time-series'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-series' }>
      readonly missingness: ReadyMissingness
    }
  | PreparedDatasetIdentity & {
      readonly kind: 'prepared-cross-section'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'cross-sectional' }>
      readonly missingness: ReadyMissingness
    }

export interface StationarityEvidenceArtifact {
  readonly kind: 'stationarity-evidence'
  readonly id: StationarityEvidenceId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly observations: number
  readonly transform: SeriesTransform
  readonly variables: NonEmptyArray<VariableStationarityEvidence>
}

export type PreprocessingEvent =
  | { readonly type: 'regular-series-selected' }
  | { readonly type: 'cross-section-selected' }
  | { readonly type: 'time-column-selected'; readonly timeColumn: ColumnId }
  | { readonly type: 'frequency-selected'; readonly frequency: Frequency }
  | { readonly type: 'variable-toggled'; readonly column: ColumnId }
  | { readonly type: 'missingness-selected'; readonly resolution: MissingnessDraft }
  | { readonly type: 'transform-selected'; readonly transform: SeriesTransform }
  | { readonly type: 'preparation-started' }
  | { readonly type: 'preparation-failed'; readonly detail: string }
  | { readonly type: 'preparation-succeeded'; readonly artifact: PreparedDatasetArtifact }
  | { readonly type: 'diagnostics-started'; readonly total: number }
  | { readonly type: 'diagnostic-completed' }
  | { readonly type: 'diagnostics-failed'; readonly detail: string }
  | { readonly type: 'diagnostics-succeeded'; readonly evidence: StationarityEvidenceArtifact }

export type PreprocessingReadinessProblem =
  | { readonly kind: 'observational-structure-required' }
  | { readonly kind: 'time-column-required' }
  | { readonly kind: 'variables-required' }
  | { readonly kind: 'missingness-unresolved'; readonly cells: number }
  | { readonly kind: 'mask-not-dense'; readonly cells: number }
  | { readonly kind: 'missingness-execution-pending'; readonly resolution: 'complete-interval' | 'imputation' }

export const initialPreprocessingDraft = (profile: DatasetProfile): PreprocessingDraft => {
  const missingCells = profile.columns.reduce((sum, column) => sum + column.nullCount, 0)
  return {
    sampling: { kind: 'unconfigured' },
    variables: { kind: 'empty' },
    missingness: missingCells === 0
      ? { kind: 'not-present' }
      : { kind: 'unresolved', cells: missingCells },
    transform: { kind: 'levels' },
    preparation: { kind: 'idle' },
    stationarity: { kind: 'idle' },
  }
}

const resetStructuralWork = (): Pick<PreprocessingDraft, 'preparation' | 'stationarity'> => ({
  preparation: { kind: 'idle' },
  stationarity: { kind: 'idle' },
})

const toggleColumn = (variables: VariableDraft, column: ColumnId): VariableDraft => {
  const current: readonly ColumnId[] = variables.kind === 'selected' ? variables.columns : []
  const next = current.includes(column)
    ? current.filter((candidate) => candidate !== column)
    : [...current, column]
  return isNonEmpty(next) ? { kind: 'selected', columns: next } : { kind: 'empty' }
}

export function stepPreprocessing(state: PreprocessingDraft, event: PreprocessingEvent): PreprocessingDraft {
  switch (event.type) {
    case 'regular-series-selected':
      return {
        ...state,
        sampling: state.sampling.kind === 'regular-series' || state.sampling.kind === 'regular-series-awaiting-time'
          ? state.sampling
          : { kind: 'regular-series-awaiting-time', frequency: 'monthly' },
        ...resetStructuralWork(),
      }
    case 'cross-section-selected':
      return {
        ...state,
        sampling: { kind: 'cross-sectional' },
        transform: { kind: 'levels' },
        ...resetStructuralWork(),
      }
    case 'time-column-selected': {
      if (state.sampling.kind !== 'regular-series' && state.sampling.kind !== 'regular-series-awaiting-time') return state
      const frequency = state.sampling.frequency
      return {
        ...state,
        sampling: { kind: 'regular-series', timeColumn: event.timeColumn, frequency },
        variables: state.variables.kind === 'selected' && state.variables.columns.includes(event.timeColumn)
          ? toggleColumn(state.variables, event.timeColumn)
          : state.variables,
        ...resetStructuralWork(),
      }
    }
    case 'frequency-selected':
      if (state.sampling.kind === 'regular-series') {
        return { ...state, sampling: { ...state.sampling, frequency: event.frequency }, ...resetStructuralWork() }
      }
      if (state.sampling.kind === 'regular-series-awaiting-time') {
        return { ...state, sampling: { kind: 'regular-series-awaiting-time', frequency: event.frequency }, ...resetStructuralWork() }
      }
      return state
    case 'variable-toggled':
      return { ...state, variables: toggleColumn(state.variables, event.column), ...resetStructuralWork() }
    case 'missingness-selected':
      return { ...state, missingness: event.resolution, ...resetStructuralWork() }
    case 'transform-selected':
      return { ...state, transform: event.transform, stationarity: { kind: 'idle' } }
    case 'preparation-started':
      return { ...state, preparation: { kind: 'running' }, stationarity: { kind: 'idle' } }
    case 'preparation-failed':
      return { ...state, preparation: { kind: 'failed', detail: event.detail }, stationarity: { kind: 'idle' } }
    case 'preparation-succeeded':
      return { ...state, preparation: { kind: 'succeeded', artifact: event.artifact }, stationarity: { kind: 'idle' } }
    case 'diagnostics-started':
      return { ...state, stationarity: { kind: 'running', completed: 0, total: event.total } }
    case 'diagnostic-completed':
      return state.stationarity.kind === 'running'
        ? { ...state, stationarity: { ...state.stationarity, completed: Math.min(state.stationarity.completed + 1, state.stationarity.total) } }
        : state
    case 'diagnostics-failed':
      return { ...state, stationarity: { kind: 'failed', detail: event.detail } }
    case 'diagnostics-succeeded':
      return { ...state, stationarity: { kind: 'succeeded', evidence: event.evidence } }
    default:
      return assertNever(event)
  }
}

export function readyPreprocessingRecipe(
  state: PreprocessingDraft,
): Result<ReadyPreprocessingRecipe, PreprocessingReadinessProblem> {
  if (state.sampling.kind === 'unconfigured') return err({ kind: 'observational-structure-required' })
  if (state.sampling.kind === 'regular-series-awaiting-time') return err({ kind: 'time-column-required' })
  if (state.variables.kind === 'empty') return err({ kind: 'variables-required' })
  switch (state.missingness.kind) {
    case 'unresolved': return err({ kind: 'missingness-unresolved', cells: state.missingness.cells })
    case 'tigramite-mask': return err({ kind: 'mask-not-dense', cells: state.missingness.cells })
    case 'complete-interval': return err({ kind: 'missingness-execution-pending', resolution: 'complete-interval' })
    case 'imputation': return err({ kind: 'missingness-execution-pending', resolution: 'imputation' })
    case 'not-present':
      switch (state.sampling.kind) {
        case 'cross-sectional':
          return ok({
            kind: 'cross-sectional',
            sampling: state.sampling,
            columns: state.variables.columns,
            missingness: state.missingness,
          })
        case 'regular-series':
          return ok({
            kind: 'regular-series',
            sampling: state.sampling,
            columns: state.variables.columns,
            missingness: state.missingness,
          })
        default: return assertNever(state.sampling)
      }
    default:
      return assertNever(state.missingness)
  }
}

export const newPreparedDatasetVersionId = (): PreparedDatasetVersionId =>
  brand<string, 'PreparedDatasetVersionId'>(crypto.randomUUID())

export const newTransformRecipeId = (): TransformRecipeId =>
  brand<string, 'TransformRecipeId'>(crypto.randomUUID())

export const newStationarityEvidenceId = (): StationarityEvidenceId =>
  brand<string, 'StationarityEvidenceId'>(crypto.randomUUID())

export function transformSeries(values: Float64Array, transform: SeriesTransform): Float64Array {
  switch (transform.kind) {
    case 'levels': return values.slice()
    case 'difference': {
      const output = new Float64Array(Math.max(0, values.length - 1))
      for (let index = 1; index < values.length; index += 1) output[index - 1] = values[index] - values[index - 1]
      return output
    }
    case 'linear-detrend': {
      const count = values.length
      const meanTime = (count - 1) / 2
      let meanValue = 0
      for (const value of values) meanValue += value / count
      let covariance = 0
      let timeVariance = 0
      for (let index = 0; index < count; index += 1) {
        const centeredTime = index - meanTime
        covariance += centeredTime * (values[index] - meanValue)
        timeVariance += centeredTime * centeredTime
      }
      const slope = timeVariance === 0 ? 0 : covariance / timeVariance
      const intercept = meanValue - slope * meanTime
      return Float64Array.from(values, (value, index) => value - (intercept + slope * index))
    }
    default:
      return assertNever(transform)
  }
}

export function describeReadinessProblem(problem: PreprocessingReadinessProblem): string {
  switch (problem.kind) {
    case 'observational-structure-required': return 'Choose whether rows form a time series or independent observations.'
    case 'time-column-required': return 'Choose the time column for this regular series.'
    case 'variables-required': return 'Choose at least one numeric analysis variable.'
    case 'missingness-unresolved': return `${problem.cells} missing cells still need an explicit policy.`
    case 'mask-not-dense': return 'Tigramite exclusion preserves missingness for discovery, but dense stationarity diagnostics need a complete interval or approved imputation.'
    case 'missingness-execution-pending': return `${problem.resolution === 'complete-interval' ? 'Complete-interval selection' : 'Imputation'} is configured but not executable in this UI slice yet.`
    default: return assertNever(problem)
  }
}
