import { z } from 'zod'
import type { ColumnId, NumericColumnSelection } from './dataset'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from './dop'

const indexSchema = z.number().int().nonnegative()
const nonEmptyIndicesSchema = z.tuple([indexSchema]).rest(indexSchema)
const correlationRowSchema = z
  .tuple([z.number().finite(), z.number().finite()])
  .rest(z.number().finite())

export const multicollinearityEvidenceSchema = z
  .object({
    kind: z.literal('multicollinearity'),
    observations: z.number().int().min(3),
    variables: z.number().int().min(2).max(64),
    correlationThreshold: z.number().gt(0).max(1),
    vifThreshold: z.number().gt(1),
    correlation: z.tuple([correlationRowSchema, correlationRowSchema]).rest(correlationRowSchema),
    correlationKeep: nonEmptyIndicesSchema,
    correlationDrop: z.array(indexSchema),
    correlationClusters: z.tuple([nonEmptyIndicesSchema]).rest(nonEmptyIndicesSchema),
    vifKeep: nonEmptyIndicesSchema,
    vifDrop: z.array(indexSchema),
    vifHistory: z.array(
      z
        .object({
          column: indexSchema,
          /** null denotes the mathematically infinite VIF produced by exact collinearity. */
          vif: z.number().finite().nullable(),
        })
        .strict(),
    ),
  })
  .strict()
  .superRefine((evidence, context) => {
    const indices = [
      ...evidence.correlationKeep,
      ...evidence.correlationDrop,
      ...evidence.correlationClusters.flat(),
      ...evidence.vifKeep,
      ...evidence.vifDrop,
      ...evidence.vifHistory.map((entry) => entry.column),
    ]
    if (
      evidence.correlation.length !== evidence.variables ||
      evidence.correlation.some((row) => row.length !== evidence.variables)
    ) {
      context.addIssue({
        code: 'custom',
        message: 'The correlation matrix dimensions must match the variable count.',
      })
    }
    if (indices.some((index) => index >= evidence.variables)) {
      context.addIssue({
        code: 'custom',
        message: 'A multicollinearity result refers to a variable outside its matrix.',
      })
    }
    const asymmetric = evidence.correlation.some((row, rowIndex) =>
      row.some((value, columnIndex) => {
        const reflected = evidence.correlation[columnIndex]?.[rowIndex]
        return reflected === undefined || Math.abs(value - reflected) > 1e-10
      }),
    )
    const invalidDiagonal = evidence.correlation.some(
      (row, index) => Math.abs((row[index] ?? 0) - 1) > 1e-10,
    )
    if (asymmetric || invalidDiagonal) {
      context.addIssue({
        code: 'custom',
        message: 'A correlation matrix must be symmetric with ones on its diagonal.',
      })
    }
    const partitions = [
      [...evidence.correlationKeep, ...evidence.correlationDrop],
      [...evidence.vifKeep, ...evidence.vifDrop],
      evidence.correlationClusters.flat(),
    ]
    if (
      partitions.some(
        (partition) =>
          partition.length !== evidence.variables || new Set(partition).size !== evidence.variables,
      )
    ) {
      context.addIssue({
        code: 'custom',
        message: 'Each retained/dropped result must partition all variables exactly once.',
      })
    }
    const clusterKeep = evidence.correlationClusters
      .map(([first]) => first)
      .sort((left, right) => left - right)
    const reportedKeep = [...evidence.correlationKeep].sort((left, right) => left - right)
    if (
      clusterKeep.length !== reportedKeep.length ||
      clusterKeep.some((index, position) => index !== reportedKeep[position])
    ) {
      context.addIssue({
        code: 'custom',
        message: 'Correlation retention must select the first variable in every reported cluster.',
      })
    }
    const vifHistory = evidence.vifHistory
      .map((entry) => entry.column)
      .sort((left, right) => left - right)
    const vifDrop = [...evidence.vifDrop].sort((left, right) => left - right)
    if (
      vifHistory.length !== vifDrop.length ||
      vifHistory.some((index, position) => index !== vifDrop[position])
    ) {
      context.addIssue({
        code: 'custom',
        message: 'The VIF history must account for every removed variable exactly once.',
      })
    }
  })

export type MulticollinearityEvidence = z.infer<typeof multicollinearityEvidenceSchema>

export type MulticollinearitySelection =
  | { readonly kind: 'correlation-selection'; readonly columns: NonEmptyArray<ColumnId> }
  | { readonly kind: 'vif-selection'; readonly columns: NonEmptyArray<ColumnId> }

export type MulticollinearityProblem =
  | { readonly kind: 'invalid-evidence'; readonly detail: string }
  | { readonly kind: 'column-shape-mismatch' }

export const parseMulticollinearityEvidence = (
  value: unknown,
): Result<MulticollinearityEvidence, MulticollinearityProblem> => {
  const parsed = multicollinearityEvidenceSchema.safeParse(value)
  return parsed.success
    ? ok(parsed.data)
    : err({ kind: 'invalid-evidence', detail: z.prettifyError(parsed.error) })
}

/** Translate the kernel's positional recommendation once, at the domain boundary. */
export const multicollinearitySelection = (
  evidence: MulticollinearityEvidence,
  columns: NonEmptyArray<NumericColumnSelection>,
  method: 'correlation' | 'vif',
): Result<MulticollinearitySelection, MulticollinearityProblem> => {
  if (columns.length !== evidence.variables) return err({ kind: 'column-shape-mismatch' })
  const indices = method === 'correlation' ? evidence.correlationKeep : evidence.vifKeep
  const selected = indices
    .map((index) => columns[index]?.id)
    .filter((column): column is ColumnId => column !== undefined)
  if (!isNonEmpty(selected) || selected.length !== indices.length)
    return err({ kind: 'column-shape-mismatch' })
  return ok(
    method === 'correlation'
      ? { kind: 'correlation-selection', columns: selected }
      : { kind: 'vif-selection', columns: selected },
  )
}
