import type { ColumnId, DatasetProfile, NumericColumnSelection } from '@/domain/dataset'
import { assertNever, err, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { resolutionCommandFor } from '@/domain/missingness'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'

/** A dense column-major matrix over the prepared version's retained rows. */
export interface PreparedMatrix {
  readonly values: Float64Array
  readonly rowCount: number
  readonly columns: NonEmptyArray<NumericColumnSelection>
  /** Cells the resolution imputed, as [row, column] over the retained rows. */
  readonly imputedCells: readonly (readonly [number, number])[]
}

export type PreparedMaterialisationProblem =
  | { readonly kind: 'materialization-refused'; readonly detail: string }
  | { readonly kind: 'seasonal-adjustment-refused'; readonly detail: string }
  | { readonly kind: 'missing-values-remain'; readonly cells: number }
  | { readonly kind: 'resolution-refused'; readonly detail: string }
  | { readonly kind: 'column-outside-prepared'; readonly column: ColumnId }

/** Unpack the bit-packed validity into one byte per cell, column-major like the values. */
const unpackValidity = (validity: Uint8Array, cells: number): Uint8Array => {
  const bytes = new Uint8Array(cells)
  for (let index = 0; index < cells; index += 1) bytes[index] = (validity[index >> 3] >> (index & 7)) & 1
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
  const resolved = await materialiseResolved(source, profile, prepared, columnIds)
  if (!resolved.ok || prepared.seasonalAdjustment.kind === 'none') return resolved
  const adjust = resolved.value.columns.flatMap((column, index) => (prepared.seasonalAdjustment.kind === 'stl' && prepared.seasonalAdjustment.columns.includes(column.id) ? [index] : []))
  if (adjust.length === 0) return resolved
  const { seasonalAdjustInWorker } = await import('@/analysis/client')
  const adjusted = await seasonalAdjustInWorker(resolved.value.values, resolved.value.rowCount, resolved.value.columns.length, { period: prepared.seasonalAdjustment.period, robust: prepared.seasonalAdjustment.robust, adjust })
  if (!adjusted.ok) return err({ kind: 'seasonal-adjustment-refused', detail: adjusted.error.detail })
  return ok({ ...resolved.value, values: Float64Array.from(adjusted.value.values) })
}

/** The retained rows with the missingness resolution applied and nothing else. */
async function materialiseResolved(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
  columnIds: NonEmptyArray<ColumnId>,
): Promise<Result<PreparedMatrix, PreparedMaterialisationProblem>> {
  for (const column of columnIds) {
    if (!prepared.columns.includes(column)) return err({ kind: 'column-outside-prepared', column })
  }
  const { materializeNumericColumnsInWorker } = await import('./client')
  const matrix = await materializeNumericColumnsInWorker(source.file, profile, columnIds)
  if (!matrix.ok) return err({ kind: 'materialization-refused', detail: matrix.error.kind })
  const { values, validity, rowCount, columns, missingCells } = matrix.value
  const record = prepared.resolution
  switch (record.kind) {
    case 'none':
      return missingCells > 0 ? err({ kind: 'missing-values-remain', cells: missingCells }) : ok({ values, rowCount, columns, imputedCells: [] })
    case 'window': {
      const rows = record.endExclusive - record.start
      const sliced = new Float64Array(rows * columns.length)
      let missing = 0
      columns.forEach((_, columnIndex) => {
        for (let row = 0; row < rows; row += 1) {
          const sourceIndex = columnIndex * rowCount + record.start + row
          const valid = (validity[sourceIndex >> 3] >> (sourceIndex & 7)) & 1
          if (valid === 0) missing += 1
          sliced[columnIndex * rows + row] = values[sourceIndex]
        }
      })
      return missing > 0 ? err({ kind: 'missing-values-remain', cells: missing }) : ok({ values: sliced, rowCount: rows, columns, imputedCells: [] })
    }
    case 'imputed': {
      if (missingCells === 0) return ok({ values, rowCount, columns, imputedCells: [] })
      const command = resolutionCommandFor(prepared.missingness)
      if (command === null || command.kind !== 'imputation') return err({ kind: 'resolution-refused', detail: 'The prepared version records an imputation its policy does not describe.' })
      const { resolveMissingnessInWorker } = await import('@/analysis/client')
      const resolved = await resolveMissingnessInWorker(values, rowCount, columns.length, unpackValidity(validity, rowCount * columns.length), command)
      if (!resolved.ok) return err({ kind: 'resolution-refused', detail: resolved.error.detail })
      if (resolved.value.outcome.kind === 'refused') {
        const { describeMissingnessRefusal } = await import('@/domain/missingness')
        return err({ kind: 'resolution-refused', detail: resolved.value.outcome.reasons.map(describeMissingnessRefusal).join(' ') })
      }
      return ok({ values: Float64Array.from(resolved.value.outcome.values), rowCount, columns, imputedCells: resolved.value.outcome.imputedCells })
    }
    default:
      return assertNever(record)
  }
}

export function describePreparedMaterialisationProblem(problem: PreparedMaterialisationProblem): string {
  switch (problem.kind) {
    case 'materialization-refused': return `The numeric columns could not be read: ${problem.detail}. Check their types and missing-value settings.`
    case 'seasonal-adjustment-refused': return `The recorded seasonal adjustment could not be applied: ${problem.detail}`
    case 'missing-values-remain': return `${problem.cells} values are missing. Choose a missing-value policy in Data studio.`
    case 'resolution-refused': return `The recorded missingness resolution could not be applied: ${problem.detail}`
    case 'column-outside-prepared': return 'A requested column is not part of the prepared dataset version. Select another column.'
    default: return assertNever(problem)
  }
}
