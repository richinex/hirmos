import { z } from 'zod'
import { brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { isNonEmpty } from './dop'

export type SourceFingerprint = Brand<string, 'SourceFingerprint'>
export type DatasetProfileId = Brand<string, 'DatasetProfileId'>
export type ColumnId = Brand<string, 'ColumnId'>

export interface NumericColumnSelection {
  readonly id: ColumnId
  readonly name: string
}

/** A nullable, column-major scientific matrix. Invalid cells contain NaN as a fail-safe and their
 * logical state is carried by the bit-packed validity map; consumers must never infer validity
 * from the numeric payload. */
export interface NullableNumericMatrix {
  readonly kind: 'nullable-numeric-matrix'
  readonly sourceFingerprint: SourceFingerprint
  readonly layout: 'column-major'
  readonly rowCount: number
  readonly columns: NonEmptyArray<NumericColumnSelection>
  readonly values: Float64Array
  readonly validity: Uint8Array
  readonly missingCells: number
}

export type PreviewCell =
  | { readonly kind: 'null' }
  | { readonly kind: 'number'; readonly value: number }
  | { readonly kind: 'integer'; readonly value: string }
  | { readonly kind: 'boolean'; readonly value: boolean }
  | { readonly kind: 'temporal'; readonly value: string }
  | { readonly kind: 'text'; readonly value: string }

export interface PhysicalColumnProfile {
  readonly id: ColumnId
  readonly name: string
  readonly duckdbType: string
  readonly nullable: boolean
  readonly nullCount: number
}

export interface SourceArtifact {
  readonly fingerprint: SourceFingerprint
  readonly fileName: string
  readonly bytes: number
  readonly format: 'csv' | 'tsv' | 'parquet'
  readonly persistence: { readonly kind: 'ephemeral' }
}

export interface DatasetProfile {
  readonly id: DatasetProfileId
  readonly source: SourceArtifact
  readonly parser: {
    readonly kind: 'duckdb-wasm'
    readonly packageVersion: '1.30.0'
    readonly engineVersion: string
  }
  readonly rowCount: number
  readonly columns: NonEmptyArray<PhysicalColumnProfile>
  readonly preview: NonEmptyArray<NonEmptyArray<PreviewCell>>
}

export type DatasetProfileProblem =
  | { readonly kind: 'fingerprint-failed'; readonly detail: string }
  | { readonly kind: 'engine-unavailable'; readonly detail: string }
  | { readonly kind: 'registration-failed'; readonly detail: string }
  | { readonly kind: 'parse-failed'; readonly detail: string }
  | { readonly kind: 'cleanup-failed'; readonly detail: string }
  | { readonly kind: 'empty-dataset' }
  | { readonly kind: 'no-columns' }
  | { readonly kind: 'unsafe-row-count'; readonly value: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export type DatasetProfileBoundaryProblem =
  | { readonly kind: 'invalid-profile-shape'; readonly detail: string }
  | { readonly kind: 'inconsistent-profile'; readonly detail: string }

export type NumericMaterializationProblem =
  | { readonly kind: 'source-changed'; readonly expected: string; readonly actual: string }
  | { readonly kind: 'column-not-found'; readonly id: string }
  | { readonly kind: 'duplicate-column'; readonly id: string }
  | { readonly kind: 'non-numeric-column'; readonly name: string; readonly duckdbType: string }
  | { readonly kind: 'non-finite-value'; readonly name: string; readonly row: number }
  | { readonly kind: 'materialization-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export type NumericMatrixBoundaryProblem = {
  readonly kind: 'invalid-numeric-matrix'
  readonly detail: string
}

export const sourceFingerprint = (value: string): Result<SourceFingerprint, { readonly kind: 'invalid-fingerprint' }> =>
  /^[a-f0-9]{64}$/.test(value)
    ? ok(brand<string, 'SourceFingerprint'>(value))
    : err({ kind: 'invalid-fingerprint' })

export const datasetProfileId = (fingerprint: SourceFingerprint): DatasetProfileId =>
  brand<string, 'DatasetProfileId'>(`duckdb-wasm:1.30.0:${fingerprint}`)

export const columnId = (index: number, name: string): ColumnId =>
  brand<string, 'ColumnId'>(`${index}:${name}`)

const NUMERIC_DUCKDB_TYPE = /^(?:U?TINYINT|U?SMALLINT|U?INTEGER|U?BIGINT|UHUGEINT|HUGEINT|FLOAT|REAL|DOUBLE|DECIMAL\(\d+,\d+\))$/

export const isNumericDuckDbType = (duckdbType: string): boolean => NUMERIC_DUCKDB_TYPE.test(duckdbType)

const columnIdFromWire = (value: string, name: string): Result<ColumnId, { readonly kind: 'invalid-column-id' }> => {
  const separator = value.indexOf(':')
  if (separator < 1 || value.slice(separator + 1) !== name) return err({ kind: 'invalid-column-id' })
  const index = Number(value.slice(0, separator))
  return Number.isSafeInteger(index) && index >= 0 && value === columnId(index, name)
    ? ok(columnId(index, name))
    : err({ kind: 'invalid-column-id' })
}

const previewCellSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('null') }).strict(),
  z.object({ kind: z.literal('number'), value: z.number().finite() }).strict(),
  z.object({ kind: z.literal('integer'), value: z.string() }).strict(),
  z.object({ kind: z.literal('boolean'), value: z.boolean() }).strict(),
  z.object({ kind: z.literal('temporal'), value: z.string() }).strict(),
  z.object({ kind: z.literal('text'), value: z.string() }).strict(),
])

const datasetProfileSchema = z.object({
  id: z.string(),
  source: z.object({
    fingerprint: z.string(),
    fileName: z.string().min(1),
    bytes: z.number().int().nonnegative(),
    format: z.enum(['csv', 'tsv', 'parquet']),
    persistence: z.object({ kind: z.literal('ephemeral') }).strict(),
  }).strict(),
  parser: z.object({
    kind: z.literal('duckdb-wasm'),
    packageVersion: z.literal('1.30.0'),
    engineVersion: z.string().min(1),
  }).strict(),
  rowCount: z.number().int().positive(),
  columns: z.array(z.object({
    id: z.string(),
    name: z.string(),
    duckdbType: z.string().min(1),
    nullable: z.boolean(),
    nullCount: z.number().int().nonnegative(),
  }).strict()),
  preview: z.array(z.array(previewCellSchema)),
}).strict()

const nullableNumericMatrixSchema = z.object({
  kind: z.literal('nullable-numeric-matrix'),
  sourceFingerprint: z.string(),
  layout: z.literal('column-major'),
  rowCount: z.number().int().positive(),
  columns: z.array(z.object({ id: z.string(), name: z.string() }).strict()),
  values: z.instanceof(Float64Array),
  validity: z.instanceof(Uint8Array),
  missingCells: z.number().int().nonnegative(),
}).strict()

/** Parse a worker-returned profile once, rebuilding every identity through its smart constructor. */
export function parseDatasetProfile(value: unknown): Result<DatasetProfile, DatasetProfileBoundaryProblem> {
  const parsed = datasetProfileSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-profile-shape', detail: z.prettifyError(parsed.error) })
  }

  const fingerprint = sourceFingerprint(parsed.data.source.fingerprint)
  if (!fingerprint.ok) {
    return err({ kind: 'inconsistent-profile', detail: 'The source fingerprint is not a SHA-256 digest.' })
  }
  const expectedProfileId = datasetProfileId(fingerprint.value)
  if (parsed.data.id !== expectedProfileId) {
    return err({ kind: 'inconsistent-profile', detail: 'The dataset profile identity does not match its source fingerprint.' })
  }
  if (!isNonEmpty(parsed.data.columns)) {
    return err({ kind: 'inconsistent-profile', detail: 'A dataset profile must contain at least one column.' })
  }

  const columns: PhysicalColumnProfile[] = []
  for (const [index, raw] of parsed.data.columns.entries()) {
    const id = columnId(index, raw.name)
    if (raw.id !== id) {
      return err({ kind: 'inconsistent-profile', detail: `Column ${raw.name} has an inconsistent identity.` })
    }
    if (raw.nullCount > parsed.data.rowCount || raw.nullable !== (raw.nullCount > 0)) {
      return err({ kind: 'inconsistent-profile', detail: `Column ${raw.name} has inconsistent null metadata.` })
    }
    columns.push({ ...raw, id })
  }
  if (!isNonEmpty(columns)) {
    return err({ kind: 'inconsistent-profile', detail: 'A dataset profile must contain at least one column.' })
  }

  const preview: NonEmptyArray<PreviewCell>[] = []
  for (const rawRow of parsed.data.preview) {
    if (!isNonEmpty(rawRow) || rawRow.length !== columns.length) {
      return err({ kind: 'inconsistent-profile', detail: 'A preview row does not match the physical schema.' })
    }
    preview.push(rawRow)
  }
  if (!isNonEmpty(preview)) {
    return err({ kind: 'inconsistent-profile', detail: 'A non-empty dataset profile must contain a bounded preview.' })
  }

  return ok({
    id: expectedProfileId,
    source: { ...parsed.data.source, fingerprint: fingerprint.value },
    parser: parsed.data.parser,
    rowCount: parsed.data.rowCount,
    columns,
    preview,
  })
}

const cellIsValid = (validity: Uint8Array, index: number): boolean =>
  (validity[index >> 3] & (1 << (index & 7))) !== 0

export function parseNullableNumericMatrix(
  value: unknown,
): Result<NullableNumericMatrix, NumericMatrixBoundaryProblem> {
  const parsed = nullableNumericMatrixSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-numeric-matrix', detail: z.prettifyError(parsed.error) })
  }
  const fingerprint = sourceFingerprint(parsed.data.sourceFingerprint)
  if (!fingerprint.ok) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The materialized matrix fingerprint is invalid.' })
  }
  if (!isNonEmpty(parsed.data.columns)) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The materialized matrix has no columns.' })
  }

  const columns: NumericColumnSelection[] = []
  const seen = new Set<string>()
  for (const raw of parsed.data.columns) {
    const id = columnIdFromWire(raw.id, raw.name)
    if (!id.ok || seen.has(raw.id)) {
      return err({ kind: 'invalid-numeric-matrix', detail: `The materialized column ${raw.name} has an invalid or duplicate identity.` })
    }
    seen.add(id.value)
    columns.push({ id: id.value, name: raw.name })
  }
  if (!isNonEmpty(columns)) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The materialized matrix has no columns.' })
  }

  const cellCount = parsed.data.rowCount * columns.length
  if (!Number.isSafeInteger(cellCount)
    || parsed.data.values.length !== cellCount
    || parsed.data.validity.length !== Math.ceil(cellCount / 8)) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The materialized matrix buffer lengths are inconsistent.' })
  }

  let missingCells = 0
  for (let index = 0; index < cellCount; index += 1) {
    if (cellIsValid(parsed.data.validity, index)) {
      if (!Number.isFinite(parsed.data.values[index])) {
        return err({ kind: 'invalid-numeric-matrix', detail: `Valid numeric cell ${index} is not finite.` })
      }
    } else {
      missingCells += 1
      if (!Number.isNaN(parsed.data.values[index])) {
        return err({ kind: 'invalid-numeric-matrix', detail: `Invalid numeric cell ${index} does not contain the fail-safe NaN.` })
      }
    }
  }
  for (let index = cellCount; index < parsed.data.validity.length * 8; index += 1) {
    if (cellIsValid(parsed.data.validity, index)) {
      return err({ kind: 'invalid-numeric-matrix', detail: 'The validity map has nonzero padding bits.' })
    }
  }
  if (missingCells !== parsed.data.missingCells) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The missing-cell count does not match the validity map.' })
  }

  return ok({ ...parsed.data, sourceFingerprint: fingerprint.value, columns })
}

export function validateNumericMatrixAgainstProfile(
  matrix: NullableNumericMatrix,
  profile: DatasetProfile,
): Result<NullableNumericMatrix, NumericMatrixBoundaryProblem> {
  if (matrix.sourceFingerprint !== profile.source.fingerprint || matrix.rowCount !== profile.rowCount) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The materialized matrix does not match its requested profile.' })
  }
  const profileColumns = new Map<string, PhysicalColumnProfile>(profile.columns.map((column) => [column.id, column]))
  for (const column of matrix.columns) {
    const matched = profileColumns.get(column.id)
    if (!matched || matched.name !== column.name) {
      return err({ kind: 'invalid-numeric-matrix', detail: `The materialized column ${column.name} is not in its requested profile.` })
    }
  }
  return ok(matrix)
}
