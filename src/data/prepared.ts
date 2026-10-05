import type {
  ColumnId,
  DatasetProfile,
  NullableNumericMatrix,
  NumericColumnSelection,
  TimeAxis,
  TimeOrderedNumericMatrix,
} from '@/domain/dataset'
import { assertNever, err, mapNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { resolutionCommandFor } from '@/domain/missingness'
import {
  seriesTransformFor,
  transformSeries,
  transformWarmup,
  type ColumnSeriesTransform,
  type DenseReadyMissingness,
  type PreparedDatasetArtifact,
} from '@/domain/preprocessing'
import {
  aggregationsForColumns,
  describeResamplingProblem,
  resampledMatrixFromEvidence,
  sameResamplingRecord,
} from '@/domain/resampling'
import type { MissingnessResolutionRecord } from '@/domain/missingness'
import type { SelectedSource } from '@/domain/workflow'
import type { SeasonalAdjustedEvidence } from '@/domain/seasonal'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

/** A dense column-major matrix over the prepared version's retained rows. */
export interface PreparedMatrix {
  readonly values: Float64Array
  readonly rowCount: number
  readonly columns: NonEmptyArray<NumericColumnSelection>
  /** Cells the resolution imputed, as [row, column] over the retained rows. */
  readonly imputedCells: readonly (readonly [number, number])[]
  /** Leading resolved rows excluded to align columns after their recorded transformations. */
  readonly leadingRowsRemoved: number
  /** Parsed temporal order for a regular series; absent for cross-sections and panels. */
  readonly timeAxis: TimeAxis | null
}

/** Nullable time-series values for discovery methods that construct samples per CI-test role. */
export interface RoleAwarePreparedMatrix {
  readonly values: Float64Array
  readonly validity: Uint8Array
  readonly analysisMask: Uint8Array
  readonly timeAxis: TimeAxis
  readonly rowCount: number
  readonly columns: NonEmptyArray<NumericColumnSelection>
}

export type PreparedMaterialisationProblem =
  | { readonly kind: 'materialization-refused'; readonly detail: string }
  | { readonly kind: 'seasonal-adjustment-refused'; readonly detail: string }
  | { readonly kind: 'missing-values-remain'; readonly cells: number }
  | { readonly kind: 'resolution-refused'; readonly detail: string }
  | { readonly kind: 'resampling-refused'; readonly detail: string }
  | { readonly kind: 'resolution-record-mismatch' }
  | { readonly kind: 'column-outside-prepared'; readonly column: ColumnId }
  | { readonly kind: 'role-aware-policy-required' }

/** Unpack the bit-packed validity into one byte per cell, column-major like the values. */
const unpackValidity = (validity: Uint8Array, cells: number): Uint8Array => {
  const bytes = new Uint8Array(cells)
  for (let index = 0; index < cells; index += 1)
    bytes[index] = (validity[index >> 3] >> (index & 7)) & 1
  return bytes
}

/**
 * Materialise prepared columns and apply the version's recorded missingness resolution, so every
 * chapter reads the same rows: the complete-interval window, or the imputed cells, never a refusal
 * on nulls the user has already resolved.
 */
export async function materialisePrepared(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<Result<PreparedMatrix, PreparedMaterialisationProblem>> {
  const stages = await materialisePreparedStages(source, profile, prepared, columnIds)
  return stages.ok ? ok(stages.value.final) : stages
}

/** Materialise the unchanged nullable grid used by PCMCI+ and LPCMCI role-aware projection. */
export async function materialiseRoleAwarePrepared(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: Extract<PreparedDatasetArtifact, { readonly kind: 'prepared-time-series' }>,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<Result<RoleAwarePreparedMatrix, PreparedMaterialisationProblem>> {
  if (prepared.missingness.kind !== 'lag-aware-exclusion')
    return err({ kind: 'role-aware-policy-required' })
  for (const column of columnIds) {
    if (!prepared.columns.includes(column)) return err({ kind: 'column-outside-prepared', column })
  }
  const { materializeTimeSeriesColumnsInWorker } = await import('./client')
  const sourceMatrix = await materializeTimeSeriesColumnsInWorker(
    source.file,
    profile,
    prepared.sampling.timeColumn,
    prepared.columns,
    prepared.sampling.interpretation,
  )
  if (!sourceMatrix.ok)
    return err({ kind: 'materialization-refused', detail: sourceMatrix.error.kind })
  if (sourceMatrix.value.missingCells !== prepared.missingness.cells) {
    return err({ kind: 'resolution-record-mismatch' })
  }

  const rows = sourceMatrix.value.rowCount
  const columns = mapNonEmpty(columnIds, (id) => {
    const column = sourceMatrix.value.columns.find((candidate) => candidate.id === id)
    if (column === undefined)
      throw new Error(`Prepared column ${id} is absent from the source matrix.`)
    return column
  })
  const values = new Float64Array(rows * columns.length)
  const validity = new Uint8Array(rows * columns.length)
  const unpacked = unpackValidity(
    sourceMatrix.value.validity,
    rows * sourceMatrix.value.columns.length,
  )
  columns.forEach((column, target) => {
    const sourceIndex = sourceMatrix.value.columns.findIndex(
      (candidate) => candidate.id === column.id,
    )
    values.set(
      sourceMatrix.value.values.subarray(sourceIndex * rows, (sourceIndex + 1) * rows),
      target * rows,
    )
    validity.set(unpacked.subarray(sourceIndex * rows, (sourceIndex + 1) * rows), target * rows)
  })
  return ok({
    values,
    validity,
    analysisMask: new Uint8Array(rows * columns.length),
    timeAxis: sourceMatrix.value.timeAxis,
    rowCount: rows,
    columns,
  })
}

/** The pipeline's intermediate matrices, for before-and-after inspection of the recipe. */
export interface PreparedStageMatrices {
  readonly resolved: PreparedMatrix
  /** After the recorded calendar aggregation; null when the version keeps its source frequency. */
  readonly resampled: PreparedMatrix | null
  /** After the recorded STL adjustment; null when no requested column is adjusted. */
  readonly adjusted: PreparedMatrix | null
  /** STL components for the adjusted columns; null when this materialisation did not run STL. */
  readonly stl: SeasonalAdjustedEvidence | null
  readonly final: PreparedMatrix
}

export async function materialisePreparedStages(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<Result<PreparedStageMatrices, PreparedMaterialisationProblem>> {
  const resolved = await materialiseResolved(source, profile, prepared, columnIds)
  if (!resolved.ok) return resolved
  let resampled: PreparedMatrix | null = null
  if (prepared.kind === 'prepared-time-series' && prepared.resampling.kind === 'daily-downsample') {
    if (resolved.value.timeAxis?.kind !== 'calendar')
      return err({
        kind: 'resampling-refused',
        detail:
          'Calendar resampling requires a date or timestamp column; an ordinal time key only defines row order.',
      })
    const input = { ...resolved.value, timestamps: resolved.value.timeAxis.timestamps }
    const aggregations = aggregationsForColumns(resolved.value.columns, prepared.resampling)
    if (!aggregations.ok)
      return err({
        kind: 'resampling-refused',
        detail: describeResamplingProblem(aggregations.error),
      })
    const { runPandasResampling } = await import('@/analysis/client')
    const evidence = await runPandasResampling(
      input.timestamps,
      input.values,
      input.rowCount,
      input.columns.length,
      prepared.resampling.targetFrequency,
      prepared.resampling.incompleteBins,
      aggregations.value,
      input.imputedCells,
    )
    if (!evidence.ok)
      return err({
        kind: 'resampling-refused',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
    const result = resampledMatrixFromEvidence(input, prepared.resampling, evidence.value)
    if (!result.ok)
      return err({ kind: 'resampling-refused', detail: describeResamplingProblem(result.error) })
    if (!sameResamplingRecord(result.value.record, prepared.resampling))
      return err({ kind: 'resolution-record-mismatch' })
    resampled = {
      ...result.value,
      leadingRowsRemoved: resolved.value.leadingRowsRemoved,
      timeAxis: { kind: 'calendar', timestamps: result.value.timestamps },
    }
  }
  let adjusted: PreparedMatrix | null = null
  let stl: SeasonalAdjustedEvidence | null = null
  if (prepared.seasonalAdjustment.kind === 'stl') {
    const matrix = resampled ?? resolved.value
    const adjust = matrix.columns.flatMap((column, index) =>
      prepared.seasonalAdjustment.kind === 'stl' &&
      prepared.seasonalAdjustment.columns.includes(column.id)
        ? [index]
        : [],
    )
    if (adjust.length > 0) {
      const { seasonalAdjustInWorker } = await import('@/analysis/client')
      const result = await seasonalAdjustInWorker(
        matrix.values,
        matrix.rowCount,
        matrix.columns.length,
        {
          period: prepared.seasonalAdjustment.period,
          robust: prepared.seasonalAdjustment.robust,
          adjust,
        },
      )
      if (!result.ok)
        return err({
          kind: 'seasonal-adjustment-refused',
          detail: describeAnalysisWorkerProblem(result.error),
        })
      stl = result.value
      adjusted = { ...matrix, values: Float64Array.from(result.value.values) }
    }
  }
  const base = adjusted ?? resampled ?? resolved.value
  const final =
    prepared.kind === 'prepared-time-series'
      ? applySeriesTransforms(base, prepared.seriesTransforms)
      : base
  return ok({ resolved: resolved.value, resampled, adjusted, stl, final })
}

/**
 * Apply each column's recorded transform and align every output to one common time grid. If any
 * prepared column is differenced, all materialisations of that version start at the second retained
 * source row—even when a caller requests only an unchanged column.
 */
export function applySeriesTransforms(
  matrix: PreparedMatrix,
  transforms: readonly ColumnSeriesTransform[],
): PreparedMatrix {
  const leadingRowsRemoved = transformWarmup(transforms)
  const outputRows = Math.max(0, matrix.rowCount - leadingRowsRemoved)
  const values = new Float64Array(outputRows * matrix.columns.length)
  const imputedSource = new Set(matrix.imputedCells.map(([row, column]) => `${row}:${column}`))
  const imputedCells: (readonly [number, number])[] = []

  for (let columnIndex = 0; columnIndex < matrix.columns.length; columnIndex += 1) {
    const start = columnIndex * matrix.rowCount
    const source = matrix.values.slice(start, start + matrix.rowCount)
    const transform = seriesTransformFor(transforms, matrix.columns[columnIndex].id)
    const transformed = transformSeries(source, transform)
    const transformedOffset = transform.kind === 'difference' ? 0 : leadingRowsRemoved
    values.set(
      transformed.subarray(transformedOffset, transformedOffset + outputRows),
      columnIndex * outputRows,
    )

    for (let outputRow = 0; outputRow < outputRows; outputRow += 1) {
      const sourceRow = outputRow + leadingRowsRemoved
      const directlyImputed = imputedSource.has(`${sourceRow}:${columnIndex}`)
      const priorImputed =
        transform.kind === 'difference' && imputedSource.has(`${sourceRow - 1}:${columnIndex}`)
      if (directlyImputed || priorImputed) imputedCells.push([outputRow, columnIndex])
    }
  }

  return {
    ...matrix,
    values,
    rowCount: outputRows,
    imputedCells,
    leadingRowsRemoved: matrix.leadingRowsRemoved + leadingRowsRemoved,
    timeAxis:
      matrix.timeAxis === null
        ? null
        : matrix.timeAxis.kind === 'calendar'
          ? { kind: 'calendar', timestamps: matrix.timeAxis.timestamps.slice(leadingRowsRemoved) }
          : { kind: 'ordinal', values: matrix.timeAxis.values.slice(leadingRowsRemoved) },
  }
}

/** The retained rows with the missingness resolution applied and nothing else. */
async function materialiseResolved(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<Result<PreparedMatrix, PreparedMaterialisationProblem>> {
  if (prepared.missingness.kind === 'lag-aware-exclusion') {
    return err({ kind: 'missing-values-remain', cells: prepared.missingness.cells })
  }
  const missingness: DenseReadyMissingness = prepared.missingness
  for (const column of columnIds) {
    if (!prepared.columns.includes(column)) return err({ kind: 'column-outside-prepared', column })
  }
  const { materializeNumericColumnsInWorker, materializeTimeSeriesColumnsInWorker } =
    await import('./client')
  const matrix =
    prepared.kind === 'prepared-time-series'
      ? await materializeTimeSeriesColumnsInWorker(
          source.file,
          profile,
          prepared.sampling.timeColumn,
          prepared.columns,
          prepared.sampling.interpretation,
        )
      : await materializeNumericColumnsInWorker(source.file, profile, prepared.columns)
  if (!matrix.ok) return err({ kind: 'materialization-refused', detail: matrix.error.kind })
  const resolved = await resolveNullableInput(matrix.value, missingness)
  if (!resolved.ok) return resolved
  if (!sameResolution(resolved.value.resolution, prepared.resolution))
    return err({ kind: 'resolution-record-mismatch' })
  return ok(selectColumns(resolved.value.matrix, columnIds))
}

const sameResolution = (
  left: MissingnessResolutionRecord,
  right: MissingnessResolutionRecord,
): boolean => {
  if (left.kind !== right.kind) return false
  switch (left.kind) {
    case 'none':
      return true
    case 'lag-aware-exclusion':
      return right.kind === 'lag-aware-exclusion' && left.cells === right.cells
    case 'window':
      return (
        right.kind === 'window' &&
        left.start === right.start &&
        left.endExclusive === right.endExclusive &&
        left.sourceRows === right.sourceRows
      )
    case 'imputed':
      return (
        right.kind === 'imputed' &&
        left.method === right.method &&
        left.maxGap === right.maxGap &&
        left.cells === right.cells
      )
    default:
      return assertNever(left)
  }
}

const selectColumns = (matrix: PreparedMatrix, ids: NonEmptyArray<ColumnId>): PreparedMatrix => {
  const rows = matrix.rowCount
  const columns = mapNonEmpty(ids, (id) => {
    const column = matrix.columns.find((candidate) => candidate.id === id)
    if (column === undefined) throw new Error(`Prepared column ${id} is absent after resolution.`)
    return column
  })
  const values = new Float64Array(rows * columns.length)
  const imputedCells: (readonly [number, number])[] = []

  columns.forEach((column, target) => {
    const source = matrix.columns.findIndex((candidate) => candidate.id === column.id)
    values.set(matrix.values.subarray(source * rows, (source + 1) * rows), target * rows)
    for (const [row, index] of matrix.imputedCells)
      if (index === source) imputedCells.push([row, target])
  })

  return { ...matrix, values, columns, imputedCells }
}

export interface ResolvedPreparedInput {
  readonly matrix: PreparedMatrix
  readonly resolution: MissingnessResolutionRecord
}

/** Apply one missingness policy to either ordinary numeric rows or a chronologically sorted series. */
export async function resolveNullableInput(
  input: NullableNumericMatrix | TimeOrderedNumericMatrix,
  missingness: DenseReadyMissingness,
): Promise<Result<ResolvedPreparedInput, PreparedMaterialisationProblem>> {
  const { values, validity, rowCount, columns, missingCells } = input
  const timeAxis = input.kind === 'time-ordered-numeric-matrix' ? input.timeAxis : null
  const command = resolutionCommandFor(missingness)
  if (missingCells > 0 && command === null)
    return err({ kind: 'missing-values-remain', cells: missingCells })
  if (command === null)
    return ok({
      matrix: { values, rowCount, columns, imputedCells: [], leadingRowsRemoved: 0, timeAxis },
      resolution: { kind: 'none' },
    })

  const { resolveMissingnessInWorker } = await import('@/analysis/client')
  const resolved = await resolveMissingnessInWorker(
    values,
    rowCount,
    columns.length,
    unpackValidity(validity, rowCount * columns.length),
    command,
  )
  if (!resolved.ok)
    return err({
      kind: 'resolution-refused',
      detail: describeAnalysisWorkerProblem(resolved.error),
    })
  const outcome = resolved.value.outcome
  if (outcome.kind === 'refused') {
    const { describeMissingnessRefusal } = await import('@/domain/missingness')
    return err({
      kind: 'resolution-refused',
      detail: outcome.reasons.map(describeMissingnessRefusal).join(' '),
    })
  }

  if (command.kind === 'completeInterval') {
    const rows = outcome.windowEnd - outcome.windowStart
    const sliced = new Float64Array(rows * columns.length)
    columns.forEach((_, column) => {
      const start = column * rowCount + outcome.windowStart
      sliced.set(values.subarray(start, start + rows), column * rows)
    })
    return ok({
      matrix: {
        values: sliced,
        rowCount: rows,
        columns,
        imputedCells: [],
        leadingRowsRemoved: 0,
        timeAxis:
          timeAxis === null
            ? null
            : timeAxis.kind === 'calendar'
              ? {
                  kind: 'calendar',
                  timestamps: timeAxis.timestamps.slice(outcome.windowStart, outcome.windowEnd),
                }
              : {
                  kind: 'ordinal',
                  values: timeAxis.values.slice(outcome.windowStart, outcome.windowEnd),
                },
      },
      resolution: {
        kind: 'window',
        start: outcome.windowStart,
        endExclusive: outcome.windowEnd,
        sourceRows: rowCount,
      },
    })
  }

  return ok({
    matrix: {
      values: Float64Array.from(outcome.values),
      rowCount,
      columns,
      imputedCells: outcome.imputedCells,
      leadingRowsRemoved: 0,
      timeAxis,
    },
    resolution: {
      kind: 'imputed',
      method: command.method,
      maxGap: command.maxGap,
      cells: outcome.imputedCells.length,
    },
  })
}

export function describePreparedMaterialisationProblem(
  problem: PreparedMaterialisationProblem,
): string {
  switch (problem.kind) {
    case 'materialization-refused':
      return `The numeric columns could not be read: ${problem.detail}. Check their types and missing-value settings.`
    case 'seasonal-adjustment-refused':
      return `The recorded seasonal adjustment could not be applied: ${problem.detail}`
    case 'missing-values-remain':
      return `${problem.cells} values are missing. Choose a missing-value policy in Data studio.`
    case 'resolution-refused':
      return `The recorded missingness resolution could not be applied: ${problem.detail}`
    case 'resampling-refused':
      return `The recorded resampling could not be applied: ${problem.detail}`
    case 'resolution-record-mismatch':
      return 'The source no longer reproduces the saved preparation record. Recreate the prepared dataset version.'
    case 'column-outside-prepared':
      return 'A requested column is not part of the prepared dataset version. Select another column.'
    case 'role-aware-policy-required':
      return 'This operation requires a time-series version prepared with lag-aware sample exclusion.'
    default:
      return assertNever(problem)
  }
}
