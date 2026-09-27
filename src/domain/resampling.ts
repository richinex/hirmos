import type { ColumnId, NumericColumnSelection } from './dataset'
import { assertNever, err, isNonEmpty, mapNonEmpty, ok, type NonEmptyArray, type Result } from './dop'
import type { Frequency } from './preprocessing'
import { z } from 'zod'

/** Calendar aggregation offered by the first resampling increment. */
export type ResamplingAggregation = 'mean' | 'sum' | 'median' | 'minimum' | 'maximum' | 'first' | 'last'

export interface ColumnResamplingAggregation {
  readonly column: ColumnId
  readonly aggregation: ResamplingAggregation
}

/**
 * Resampling is a saved data transformation, not a display setting. The first release deliberately
 * supports the unambiguous downsampling lane used in the chapter notebooks: daily observations to
 * Monday-based calendar weeks or calendar months.
 */
export type ResamplingDraft =
  | { readonly kind: 'none' }
  | {
      readonly kind: 'daily-downsample'
      readonly targetFrequency: 'weekly' | 'monthly'
      readonly incompleteBins: 'keep' | 'drop'
      readonly aggregations: readonly ColumnResamplingAggregation[]
    }

export type ResamplingRecipe =
  | { readonly kind: 'none' }
  | {
      readonly kind: 'daily-downsample'
      readonly sourceFrequency: 'daily'
      readonly targetFrequency: 'weekly' | 'monthly'
      readonly weekStartsOn: 'monday'
      readonly calendar: 'utc'
      readonly incompleteBins: 'keep' | 'drop'
      readonly aggregations: NonEmptyArray<ColumnResamplingAggregation>
    }

export type ResamplingRecord =
  | { readonly kind: 'none' }
  | {
      readonly kind: 'daily-downsample'
      readonly sourceFrequency: 'daily'
      readonly targetFrequency: 'weekly' | 'monthly'
      readonly weekStartsOn: 'monday'
      readonly calendar: 'utc'
      readonly incompleteBins: 'keep' | 'drop'
      readonly aggregations: NonEmptyArray<ColumnResamplingAggregation>
      readonly sourceRows: number
      readonly outputRows: number
      readonly incompleteBinsFound: number
      readonly binsDropped: number
      readonly sourceRowsDropped: number
    }

export interface ResamplingInputMatrix {
  readonly values: Float64Array
  readonly rowCount: number
  readonly columns: NonEmptyArray<NumericColumnSelection>
  readonly imputedCells: readonly (readonly [number, number])[]
  /** Parsed instants in ascending order, one per row. */
  readonly timestamps: Float64Array
}

export interface ResampledMatrix extends ResamplingInputMatrix {
  readonly record: ResamplingRecord
}

export type ResamplingProblem =
  | { readonly kind: 'kernel-refused'; readonly detail: string }

export const aggregationFor = (
  aggregations: readonly ColumnResamplingAggregation[],
  column: ColumnId,
): ResamplingAggregation | null => aggregations.find((candidate) => candidate.column === column)?.aggregation ?? null

export const aggregationsForColumns = (
  columns: NonEmptyArray<NumericColumnSelection>,
  recipe: Extract<ResamplingRecipe | ResamplingRecord, { readonly kind: 'daily-downsample' }>,
): Result<NonEmptyArray<ResamplingAggregation>, ResamplingProblem> => {
  const ordered: ResamplingAggregation[] = []
  for (const column of columns) {
    const aggregation = aggregationFor(recipe.aggregations, column.id)
    if (aggregation === null) return err({ kind: 'kernel-refused', detail: `The saved recipe has no aggregation for ${column.name}.` })
    ordered.push(aggregation)
  }
  return isNonEmpty(ordered)
    ? ok(ordered)
    : err({ kind: 'kernel-refused', detail: 'No analysis columns were supplied for resampling.' })
}

export const effectiveFrequency = (source: Frequency, resampling: ResamplingDraft | ResamplingRecipe | ResamplingRecord): Frequency => {
  switch (resampling.kind) {
    case 'none': return source
    case 'daily-downsample': return resampling.targetFrequency
    default: return assertNever(resampling)
  }
}

export type ResamplingReadinessProblem = { readonly kind: 'aggregations-required'; readonly columns: NonEmptyArray<ColumnId> }

export const readyResamplingRecipe = (
  source: Frequency,
  columns: NonEmptyArray<ColumnId>,
  draft: ResamplingDraft,
): Result<ResamplingRecipe, ResamplingReadinessProblem> => {
  switch (draft.kind) {
    case 'none': return ok({ kind: 'none' })
    case 'daily-downsample': {
      // The reducer resets this state when the declared source frequency ceases to be daily.
      if (source !== 'daily') return ok({ kind: 'none' })
      const missing = columns.filter((column) => aggregationFor(draft.aggregations, column) === null)
      if (isNonEmpty(missing)) return err({ kind: 'aggregations-required', columns: missing })
      return ok({
        kind: 'daily-downsample',
        sourceFrequency: 'daily',
        targetFrequency: draft.targetFrequency,
        weekStartsOn: 'monday',
        calendar: 'utc',
        incompleteBins: draft.incompleteBins,
        aggregations: mapNonEmpty(columns, (column) => {
          const aggregation = aggregationFor(draft.aggregations, column)
          if (aggregation === null) throw new Error(`Aggregation for ${column} disappeared after validation.`)
          return { column, aggregation }
        }),
      })
    }
    default: return assertNever(draft)
  }
}

export const pandasResamplingEvidenceSchema = z.object({
  kind: z.literal('pandasResampled'),
  values: z.array(z.number().finite()),
  timestampsMs: z.array(z.number().int()),
  imputedCells: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
  sourceRows: z.number().int().positive(),
  outputRows: z.number().int().positive(),
  incompleteBins: z.number().int().nonnegative(),
  binsDropped: z.number().int().nonnegative(),
  sourceRowsDropped: z.number().int().nonnegative(),
}).strict()

export type PandasResamplingEvidence = z.infer<typeof pandasResamplingEvidenceSchema>

export const parsePandasResamplingEvidence = (value: unknown): Result<PandasResamplingEvidence, ResamplingProblem> => {
  const parsed = pandasResamplingEvidenceSchema.safeParse(value)
  return parsed.success
    ? ok(parsed.data)
    : err({ kind: 'kernel-refused', detail: z.prettifyError(parsed.error) })
}

/** Reattach browser column identities and the saved recipe to a validated Rust result. */
export const resampledMatrixFromEvidence = (
  matrix: ResamplingInputMatrix,
  recipe: Extract<ResamplingRecipe | ResamplingRecord, { readonly kind: 'daily-downsample' }>,
  evidence: PandasResamplingEvidence,
): Result<ResampledMatrix, ResamplingProblem> => {
  const cells = evidence.outputRows * matrix.columns.length
  if (evidence.sourceRows !== matrix.rowCount
    || evidence.values.length !== cells
    || evidence.timestampsMs.length !== evidence.outputRows
    || evidence.imputedCells.some(([row, column]) => row >= evidence.outputRows || column >= matrix.columns.length)) {
    return err({ kind: 'kernel-refused', detail: 'The resampled data has unexpected dimensions. It cannot be used to prepare the dataset.' })
  }
  return ok({
    values: Float64Array.from(evidence.values),
    rowCount: evidence.outputRows,
    columns: matrix.columns,
    imputedCells: evidence.imputedCells,
    timestamps: Float64Array.from(evidence.timestampsMs),
    record: {
      kind: 'daily-downsample',
      sourceFrequency: 'daily',
      targetFrequency: recipe.targetFrequency,
      weekStartsOn: 'monday',
      calendar: 'utc',
      incompleteBins: recipe.incompleteBins,
      aggregations: recipe.aggregations,
      sourceRows: evidence.sourceRows,
      outputRows: evidence.outputRows,
      incompleteBinsFound: evidence.incompleteBins,
      binsDropped: evidence.binsDropped,
      sourceRowsDropped: evidence.sourceRowsDropped,
    },
  })
}

export function sameResamplingRecord(left: ResamplingRecord, right: ResamplingRecord): boolean {
  if (left.kind !== right.kind) return false
  if (left.kind === 'none' || right.kind === 'none') return true
  return left.sourceFrequency === right.sourceFrequency
    && left.targetFrequency === right.targetFrequency
    && left.weekStartsOn === right.weekStartsOn
    && left.calendar === right.calendar
    && left.incompleteBins === right.incompleteBins
    && left.sourceRows === right.sourceRows
    && left.outputRows === right.outputRows
    && left.incompleteBinsFound === right.incompleteBinsFound
    && left.binsDropped === right.binsDropped
    && left.sourceRowsDropped === right.sourceRowsDropped
    && left.aggregations.length === right.aggregations.length
    && left.aggregations.every((entry, i) => entry.column === right.aggregations[i]?.column && entry.aggregation === right.aggregations[i]?.aggregation)
}

export function describeResampling(record: ResamplingRecord, name: (column: ColumnId) => string): string | null {
  switch (record.kind) {
    case 'none': return null
    case 'daily-downsample': {
      const rules = record.aggregations.map((entry) => `${name(entry.column)}: ${entry.aggregation}`).join(', ')
      const dropped = record.binsDropped > 0 ? `; dropped ${record.binsDropped} incomplete ${record.binsDropped === 1 ? 'bin' : 'bins'}` : ''
      return `Daily to ${record.targetFrequency}, ${rules}${dropped}`
    }
    default: return assertNever(record)
  }
}

export function describeResamplingProblem(problem: ResamplingProblem): string {
  return problem.detail
}
