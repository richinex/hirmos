import type { MissingnessResolutionRecord } from './missingness'
import { assertNever, brand, err, isNonEmpty, mapNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { ColumnId, DatasetProfile } from './dataset'
import { seasonalPeriodOf, type SeasonalAdjustmentRecord } from './seasonal'
import type { StationarityBattery } from './stationarity'
import type { StationarityAssessment } from './stationarityAssessment'
import type { PanelStructureEvidence } from './panel'
import {
  effectiveFrequency,
  readyResamplingRecipe,
  type ResamplingAggregation,
  type ResamplingDraft,
  type ResamplingRecipe,
  type ResamplingRecord,
} from './resampling'

export type PreparedDatasetVersionId = Brand<string, 'PreparedDatasetVersionId'>
export type TransformRecipeId = Brand<string, 'TransformRecipeId'>
export type StationarityEvidenceId = Brand<string, 'StationarityEvidenceId'>

export type Frequency = 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly'

export type SamplingDraft =
  | { readonly kind: 'unconfigured' }
  | { readonly kind: 'cross-sectional' }
  | { readonly kind: 'regular-series-awaiting-time'; readonly frequency: Frequency }
  | { readonly kind: 'regular-series'; readonly timeColumn: ColumnId; readonly frequency: Frequency }
  | { readonly kind: 'regular-panel-awaiting-keys'; readonly unitColumn: ColumnId | null; readonly timeColumn: ColumnId | null; readonly frequency: Frequency }
  | { readonly kind: 'regular-panel'; readonly unitColumn: ColumnId; readonly timeColumn: ColumnId; readonly frequency: Frequency }

export type VariableDraft =
  | { readonly kind: 'empty' }
  | { readonly kind: 'selected'; readonly columns: NonEmptyArray<ColumnId> }

export type MissingnessDraft =
  | { readonly kind: 'not-present' }
  | { readonly kind: 'unresolved'; readonly cells: number }
  | {
      readonly kind: 'lag-aware-exclusion'
      readonly cells: number
      readonly history:
        | { readonly kind: 'method-default' }
        | { readonly kind: 'minimum-for-features' }
        | { readonly kind: 'fixed-warmup'; readonly observations: number }
      readonly gapInfluence:
        | { readonly kind: 'direct-only' }
        | { readonly kind: 'following-guard'; readonly steps: number }
      readonly analysisExclusions:
        | { readonly kind: 'ignore' }
        | {
            readonly kind: 'roles'
            readonly roles: NonEmptyArray<'candidate-cause' | 'tested-outcome' | 'conditioner'>
          }
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

/** The transformation applied to one analysis column when a time-series version is materialised. */
export interface ColumnSeriesTransform {
  readonly column: ColumnId
  readonly transform: SeriesTransform
}

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

/** Which selected columns lose their STL seasonal component when the version is created. */
export type SeasonalAdjustmentDraft =
  | { readonly kind: 'none' }
  | { readonly kind: 'stl'; readonly columns: readonly ColumnId[]; readonly robust: boolean }

export interface PreprocessingDraft {
  readonly sampling: SamplingDraft
  readonly variables: VariableDraft
  readonly missingness: MissingnessDraft
  /** A saved calendar aggregation for regular daily series; never a chart-only grouping. */
  readonly resampling: ResamplingDraft
  readonly seasonal: SeasonalAdjustmentDraft
  /** Preparation choices. Missing entries are levels until the recipe is made. */
  readonly seriesTransforms: readonly ColumnSeriesTransform[]
  /** A display-only scale for the stationarity table; it never changes the prepared matrix. */
  readonly diagnosticTransform: SeriesTransform
  readonly preparation: PreparationJob
  readonly stationarity: StationarityJob
}

export type DenseReadyMissingness = Exclude<MissingnessDraft, { readonly kind: 'unresolved' | 'lag-aware-exclusion' }>

export type ReadyPreprocessingRecipe =
  | {
      readonly kind: 'regular-series'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-series' }>
      readonly columns: NonEmptyArray<ColumnId>
      readonly missingness: DenseReadyMissingness
      readonly resampling: ResamplingRecipe
      readonly seasonalAdjustment: SeasonalAdjustmentRecord
      readonly seriesTransforms: NonEmptyArray<ColumnSeriesTransform>
    }
  | {
      readonly kind: 'cross-sectional'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'cross-sectional' }>
      readonly columns: NonEmptyArray<ColumnId>
      readonly missingness: DenseReadyMissingness
    }
  | {
      readonly kind: 'regular-panel'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-panel' }>
      readonly columns: NonEmptyArray<ColumnId>
      readonly missingness: DenseReadyMissingness
    }

export interface VariableStationarityEvidence {
  readonly column: ColumnId
  /** The battery under the chosen diagnostic scale. */
  readonly result: StationarityBattery
  readonly levels: StationarityBattery
  readonly differenced: StationarityBattery | null
  readonly assessment: StationarityAssessment
}

interface PreparedDatasetIdentity {
  readonly id: PreparedDatasetVersionId
  readonly recipe: TransformRecipeId
  readonly sourceProfile: DatasetProfile['id']
  /** Rows every chapter reads after missingness resolution and the shared transformation warm-up. */
  readonly observations: number
  readonly columns: NonEmptyArray<ColumnId>
  /** What the data-preparation core did about missing cells when this version was created. */
  readonly resolution: MissingnessResolutionRecord
  /** The STL seasonal adjustment every chapter applies after the resolution; none for a cross-section. */
  readonly seasonalAdjustment: SeasonalAdjustmentRecord
}

export type PreparedDatasetArtifact =
  | PreparedDatasetIdentity & {
      readonly kind: 'prepared-time-series'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-series' }>
      readonly missingness: DenseReadyMissingness
      /** Calendar aggregation and its realised row counts. */
      readonly resampling: ResamplingRecord
      /** One record per analysis column. Materialisation applies these after missingness and STL. */
      readonly seriesTransforms: NonEmptyArray<ColumnSeriesTransform>
    }
  | PreparedDatasetIdentity & {
      readonly kind: 'prepared-panel'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'regular-panel' }>
      readonly missingness: DenseReadyMissingness
      readonly panel: PanelStructureEvidence
    }
  | PreparedDatasetIdentity & {
      readonly kind: 'prepared-cross-section'
      readonly sampling: Extract<SamplingDraft, { readonly kind: 'cross-sectional' }>
      readonly missingness: DenseReadyMissingness
    }

export interface StationarityEvidenceArtifact {
  readonly kind: 'stationarity-evidence'
  readonly id: StationarityEvidenceId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly observations: number
  /** The scale shown in `result`; `levels` and `differenced` always describe prepared values. */
  readonly diagnosticTransform: SeriesTransform
  readonly variables: NonEmptyArray<VariableStationarityEvidence>
}

export type PreprocessingEvent =
  | { readonly type: 'regular-series-selected' }
  | { readonly type: 'cross-section-selected' }
  | { readonly type: 'regular-panel-selected' }
  | { readonly type: 'unit-column-selected'; readonly unitColumn: ColumnId }
  | { readonly type: 'time-column-selected'; readonly timeColumn: ColumnId }
  | { readonly type: 'frequency-selected'; readonly frequency: Frequency }
  | { readonly type: 'variable-toggled'; readonly column: ColumnId }
  | { readonly type: 'missingness-selected'; readonly resolution: MissingnessDraft }
  | { readonly type: 'resampling-selected'; readonly resampling: ResamplingDraft }
  | { readonly type: 'resampling-aggregation-selected'; readonly column: ColumnId; readonly aggregation: ResamplingAggregation }
  | { readonly type: 'seasonal-adjustment-selected'; readonly seasonal: SeasonalAdjustmentDraft }
  | { readonly type: 'series-transform-selected'; readonly column: ColumnId; readonly transform: SeriesTransform }
  | { readonly type: 'all-series-transforms-selected'; readonly transform: SeriesTransform }
  | { readonly type: 'diagnostic-transform-selected'; readonly transform: SeriesTransform }
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
  | { readonly kind: 'unit-column-required' }
  | { readonly kind: 'variables-required' }
  | { readonly kind: 'missingness-unresolved'; readonly cells: number }
  | { readonly kind: 'lag-exclusion-needs-compatible-method'; readonly cells: number }
  | { readonly kind: 'panel-missingness-unsupported'; readonly cells: number }
  | { readonly kind: 'structural-zero-unconfirmed' }
  | { readonly kind: 'seasonal-period-unavailable' }
  | { readonly kind: 'seasonal-columns-required' }
  | { readonly kind: 'resampling-aggregations-required'; readonly columns: NonEmptyArray<ColumnId> }

export const initialPreprocessingDraft = (profile: DatasetProfile): PreprocessingDraft => {
  const missingCells = profile.columns.reduce((sum, column) => sum + column.nullCount, 0)
  return {
    sampling: { kind: 'unconfigured' },
    variables: { kind: 'empty' },
    missingness: missingCells === 0
      ? { kind: 'not-present' }
      : { kind: 'unresolved', cells: missingCells },
    resampling: { kind: 'none' },
    seasonal: { kind: 'none' },
    seriesTransforms: [],
    diagnosticTransform: { kind: 'levels' },
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

export const seriesTransformFor = (
  transforms: readonly ColumnSeriesTransform[],
  column: ColumnId,
): SeriesTransform => transforms.find((candidate) => candidate.column === column)?.transform ?? { kind: 'levels' }

export const transformWarmup = (transforms: readonly ColumnSeriesTransform[]): number =>
  transforms.some((record) => record.transform.kind === 'difference') ? 1 : 0

export function describeSeriesTransform(transform: SeriesTransform): string {
  switch (transform.kind) {
    case 'levels': return 'levels'
    case 'difference': return 'first difference'
    case 'linear-detrend': return 'linear detrend'
    default: return assertNever(transform)
  }
}

const setSeriesTransform = (
  transforms: readonly ColumnSeriesTransform[],
  column: ColumnId,
  transform: SeriesTransform,
): readonly ColumnSeriesTransform[] => [
  ...transforms.filter((candidate) => candidate.column !== column),
  { column, transform },
]

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
        seasonal: { kind: 'none' },
        resampling: { kind: 'none' },
        seriesTransforms: [],
        diagnosticTransform: { kind: 'levels' },
        ...resetStructuralWork(),
      }
    case 'regular-panel-selected':
      return {
        ...state,
        sampling: state.sampling.kind === 'regular-panel' || state.sampling.kind === 'regular-panel-awaiting-keys'
          ? state.sampling
          : { kind: 'regular-panel-awaiting-keys', unitColumn: null, timeColumn: null, frequency: 'yearly' },
        seasonal: { kind: 'none' }, resampling: { kind: 'none' }, seriesTransforms: [], diagnosticTransform: { kind: 'levels' }, ...resetStructuralWork(),
      }
    case 'unit-column-selected': {
      if (state.sampling.kind !== 'regular-panel' && state.sampling.kind !== 'regular-panel-awaiting-keys') return state
      const timeColumn = state.sampling.timeColumn === event.unitColumn ? null : state.sampling.timeColumn
      const sampling: SamplingDraft = timeColumn === null
        ? { kind: 'regular-panel-awaiting-keys', unitColumn: event.unitColumn, timeColumn, frequency: state.sampling.frequency }
        : { kind: 'regular-panel', unitColumn: event.unitColumn, timeColumn, frequency: state.sampling.frequency }
      const variables = state.variables.kind === 'selected' && state.variables.columns.includes(event.unitColumn) ? toggleColumn(state.variables, event.unitColumn) : state.variables
      return { ...state, sampling, variables, ...resetStructuralWork() }
    }
    case 'time-column-selected': {
      if (state.sampling.kind === 'regular-panel' || state.sampling.kind === 'regular-panel-awaiting-keys') {
        const unitColumn = state.sampling.unitColumn === event.timeColumn ? null : state.sampling.unitColumn
        const sampling: SamplingDraft = unitColumn === null
          ? { kind: 'regular-panel-awaiting-keys', unitColumn, timeColumn: event.timeColumn, frequency: state.sampling.frequency }
          : { kind: 'regular-panel', unitColumn, timeColumn: event.timeColumn, frequency: state.sampling.frequency }
        const variables = state.variables.kind === 'selected' && state.variables.columns.includes(event.timeColumn) ? toggleColumn(state.variables, event.timeColumn) : state.variables
        return { ...state, sampling, variables, ...resetStructuralWork() }
      }
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
        return { ...state, sampling: { ...state.sampling, frequency: event.frequency }, resampling: event.frequency === 'daily' ? state.resampling : { kind: 'none' }, ...resetStructuralWork() }
      }
      if (state.sampling.kind === 'regular-series-awaiting-time') {
        return { ...state, sampling: { kind: 'regular-series-awaiting-time', frequency: event.frequency }, resampling: event.frequency === 'daily' ? state.resampling : { kind: 'none' }, ...resetStructuralWork() }
      }
      if (state.sampling.kind === 'regular-panel') return { ...state, sampling: { ...state.sampling, frequency: event.frequency }, ...resetStructuralWork() }
      if (state.sampling.kind === 'regular-panel-awaiting-keys') return { ...state, sampling: { ...state.sampling, frequency: event.frequency }, ...resetStructuralWork() }
      return state
    case 'variable-toggled': {
      const variables = toggleColumn(state.variables, event.column)
      const kept: readonly ColumnId[] = variables.kind === 'selected' ? variables.columns : []
      const seasonal: SeasonalAdjustmentDraft = state.seasonal.kind === 'stl' ? { ...state.seasonal, columns: state.seasonal.columns.filter((column) => kept.includes(column)) } : state.seasonal
      const seriesTransforms = state.seriesTransforms.filter((candidate) => kept.includes(candidate.column))
      const resampling: ResamplingDraft = state.resampling.kind === 'daily-downsample'
        ? { ...state.resampling, aggregations: state.resampling.aggregations.filter((candidate) => kept.includes(candidate.column)) }
        : state.resampling
      return { ...state, variables, seasonal, resampling, seriesTransforms, ...resetStructuralWork() }
    }
    case 'seasonal-adjustment-selected':
      return { ...state, seasonal: event.seasonal, ...resetStructuralWork() }
    case 'missingness-selected':
      return { ...state, missingness: event.resolution, ...resetStructuralWork() }
    case 'resampling-selected':
      if (state.sampling.kind !== 'regular-series' || state.sampling.frequency !== 'daily') return state
      return { ...state, resampling: event.resampling, ...resetStructuralWork() }
    case 'resampling-aggregation-selected':
      if (state.resampling.kind !== 'daily-downsample' || state.variables.kind !== 'selected' || !state.variables.columns.includes(event.column)) return state
      return {
        ...state,
        resampling: {
          ...state.resampling,
          aggregations: [
            ...state.resampling.aggregations.filter((candidate) => candidate.column !== event.column),
            { column: event.column, aggregation: event.aggregation },
          ],
        },
        ...resetStructuralWork(),
      }
    case 'series-transform-selected':
      if (state.variables.kind !== 'selected' || !state.variables.columns.includes(event.column)) return state
      return { ...state, seriesTransforms: setSeriesTransform(state.seriesTransforms, event.column, event.transform), ...resetStructuralWork() }
    case 'all-series-transforms-selected':
      return state.variables.kind === 'selected'
        ? { ...state, seriesTransforms: mapNonEmpty(state.variables.columns, (column) => ({ column, transform: event.transform })), ...resetStructuralWork() }
        : state
    case 'diagnostic-transform-selected':
      return { ...state, diagnosticTransform: event.transform, stationarity: { kind: 'idle' } }
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
  if (state.sampling.kind === 'regular-panel-awaiting-keys' && state.sampling.unitColumn === null) return err({ kind: 'unit-column-required' })
  if (state.sampling.kind === 'regular-panel-awaiting-keys') return err({ kind: 'time-column-required' })
  if (state.variables.kind === 'empty') return err({ kind: 'variables-required' })
  if (state.sampling.kind === 'regular-panel' && state.missingness.kind !== 'not-present') {
    return err({ kind: 'panel-missingness-unsupported', cells: state.missingness.cells })
  }
  switch (state.missingness.kind) {
    case 'unresolved': return err({ kind: 'missingness-unresolved', cells: state.missingness.cells })
    case 'lag-aware-exclusion': return err({ kind: 'lag-exclusion-needs-compatible-method', cells: state.missingness.cells })
    case 'complete-interval':
    case 'imputation':
    case 'not-present': {
      if (state.missingness.kind === 'imputation' && state.missingness.method === 'structuralZero' && !state.missingness.confirmedStructuralZero) {
        return err({ kind: 'structural-zero-unconfirmed' })
      }
      switch (state.sampling.kind) {
        case 'cross-sectional':
          return ok({
            kind: 'cross-sectional',
            sampling: state.sampling,
            columns: state.variables.columns,
            missingness: state.missingness,
          })
        case 'regular-series': {
          const resampling = readyResamplingRecipe(state.sampling.frequency, state.variables.columns, state.resampling)
          if (!resampling.ok) return err({ kind: 'resampling-aggregations-required', columns: resampling.error.columns })
          const period = seasonalPeriodOf(effectiveFrequency(state.sampling.frequency, resampling.value))
          let seasonalAdjustment: SeasonalAdjustmentRecord = { kind: 'none' }
          if (state.seasonal.kind === 'stl') {
            if (period === null) return err({ kind: 'seasonal-period-unavailable' })
            const columns = state.seasonal.columns.filter((column) => state.variables.kind === 'selected' && state.variables.columns.includes(column))
            if (!isNonEmpty(columns)) return err({ kind: 'seasonal-columns-required' })
            seasonalAdjustment = { kind: 'stl', period, robust: state.seasonal.robust, columns }
          }
          return ok({
            kind: 'regular-series',
            sampling: state.sampling,
            columns: state.variables.columns,
            missingness: state.missingness,
            resampling: resampling.value,
            seasonalAdjustment,
            seriesTransforms: mapNonEmpty(state.variables.columns, (column) => ({
              column,
              transform: seriesTransformFor(state.seriesTransforms, column),
            })),
          })
        }
        case 'regular-panel':
          return ok({ kind: 'regular-panel', sampling: state.sampling, columns: state.variables.columns, missingness: state.missingness })
        default: return assertNever(state.sampling)
      }
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
    case 'observational-structure-required': return 'Choose a time series, a panel, or independent observations.'
    case 'time-column-required': return 'Choose the time column.'
    case 'unit-column-required': return 'Choose the unit column for this panel.'
    case 'variables-required': return 'Choose at least one numeric analysis variable.'
    case 'missingness-unresolved': return `${problem.cells} missing values still need a policy.`
    case 'lag-exclusion-needs-compatible-method': return 'Choose a complete interval or approved imputation for dense diagnostics. Lag-aware exclusion keeps the original time grid.'
    case 'panel-missingness-unsupported': return `This panel has ${problem.cells} missing values. Complete them separately within each unit before preparing the panel.`
    case 'structural-zero-unconfirmed': return 'Confirm that each missing value represents a true zero.'
    case 'seasonal-period-unavailable': return 'Yearly rows have no seasonal period, so seasonal-trend decomposition using loess (STL) does not apply.'
    case 'seasonal-columns-required': return 'Choose at least one selected column to adjust seasonally, or switch the adjustment off.'
    case 'resampling-aggregations-required': return `Choose how to aggregate ${problem.columns.length === 1 ? 'the selected column' : 'every selected column'} into the new calendar interval.`
    default: return assertNever(problem)
  }
}
