import { z } from 'zod'
import { brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { isNonEmpty } from './dop'
import { columnDeclarationsSchema, declarationsKey, fileReadingOf, type FileReading } from './fileReading'

export type SourceFingerprint = Brand<string, 'SourceFingerprint'>
export type DatasetProfileId = Brand<string, 'DatasetProfileId'>
export type ColumnId = Brand<string, 'ColumnId'>

export interface ColumnSelection {
  readonly id: ColumnId
  readonly name: string
}

/** A source column that has been verified as numeric before it enters a numerical matrix. */
export interface NumericColumnSelection extends ColumnSelection {}

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

export type TimeAxis =
  | { readonly kind: 'calendar'; readonly timestamps: Float64Array }
  | { readonly kind: 'ordinal'; readonly values: Float64Array }

/** Numeric values paired with one parsed, chronologically sorted time key per source row. */
export interface TimeOrderedNumericMatrix {
  readonly kind: 'time-ordered-numeric-matrix'
  readonly sourceFingerprint: SourceFingerprint
  readonly layout: 'column-major'
  readonly rowCount: number
  readonly timeColumn: NumericColumnSelection
  readonly timeAxis: TimeAxis
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

/** Whether the browser keeps a copy of the file: never by default; in the origin-private file system when the user asks. */
export type SourcePersistence = { readonly kind: 'ephemeral' } | { readonly kind: 'cached-locally' }

/** The file a profile was read from, and how its columns were read. */
export type SourceArtifact = {
  readonly fingerprint: SourceFingerprint
  readonly fileName: string
  readonly bytes: number
  readonly persistence: SourcePersistence
} & FileReading

export interface DatasetProfile {
  readonly id: DatasetProfileId
  readonly source: SourceArtifact
  readonly parser: {
    readonly kind: 'duckdb-wasm'
    readonly packageVersion: DataParserVersion
    readonly engineVersion: string
  }
  readonly rowCount: number
  readonly columns: NonEmptyArray<PhysicalColumnProfile>
  readonly preview: NonEmptyArray<NonEmptyArray<PreviewCell>>
}

export interface HistogramBins {
  /** `counts.length + 1` edges; the last bin includes its upper edge. */
  readonly edges: readonly number[]
  readonly counts: readonly number[]
}

/** One column described in full, computed by DuckDB on demand when the column is selected. */
export type ColumnProfile =
  | {
      readonly kind: 'numeric-column-profile'
      readonly column: ColumnId
      readonly sourceFingerprint: SourceFingerprint
      readonly count: number
      readonly nullCount: number
      readonly distinctCount: number
      readonly zeroCount: number
      readonly min: number
      readonly max: number
      readonly mean: number
      /** Sample standard deviation; null when fewer than two values exist. */
      readonly standardDeviation: number | null
      readonly quartiles: { readonly lower: number; readonly median: number; readonly upper: number }
      readonly histogram: HistogramBins
    }
  | {
      readonly kind: 'categorical-column-profile'
      readonly column: ColumnId
      readonly sourceFingerprint: SourceFingerprint
      readonly count: number
      readonly nullCount: number
      readonly distinctCount: number
      /** The most frequent values, descending, at most eight. */
      readonly top: readonly { readonly value: string; readonly count: number }[]
    }

export type ColumnProfileProblem =
  | { readonly kind: 'column-not-found'; readonly id: string }
  | { readonly kind: 'source-changed'; readonly expected: string; readonly actual: string }
  | { readonly kind: 'column-profile-failed'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }

export type ColumnProfileBoundaryProblem = { readonly kind: 'invalid-column-profile'; readonly detail: string }

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

export type TimeSeriesMaterializationProblem = NumericMaterializationProblem
  | { readonly kind: 'time-value-unparseable'; readonly name: string; readonly row: number }
  | { readonly kind: 'duplicate-time-value'; readonly name: string; readonly row: number }

export type NumericMatrixBoundaryProblem = {
  readonly kind: 'invalid-numeric-matrix'
  readonly detail: string
}

export const sourceFingerprint = (value: string): Result<SourceFingerprint, { readonly kind: 'invalid-fingerprint' }> =>
  /^[a-f0-9]{64}$/.test(value)
    ? ok(brand<string, 'SourceFingerprint'>(value))
    : err({ kind: 'invalid-fingerprint' })

/** One file read two ways gives two profiles, so the declarations are part of the id. */
export const datasetProfileId = (fingerprint: SourceFingerprint, reading: FileReading, version: DataParserVersion = DUCKDB_PACKAGE_VERSION): DatasetProfileId => {
  const declared = reading.format === 'parquet' ? '' : declarationsKey(reading.declared)
  return brand<string, 'DatasetProfileId'>(`duckdb-wasm:${version}:${fingerprint}${declared === '' ? '' : `:${declared}`}`)
}

export const columnId = (index: number, name: string): ColumnId =>
  brand<string, 'ColumnId'>(`${index}:${name}`)

/** The column's name, read back from its id, for text that has no profile to hand. */
export const columnNameOf = (id: ColumnId): string => id.slice(id.indexOf(':') + 1)

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
    declared: columnDeclarationsSchema.optional(),
    persistence: z.object({ kind: z.enum(['ephemeral', 'cached-locally']) }).strict(),
  }).strict(),
  parser: z.object({
    kind: z.literal('duckdb-wasm'),
    packageVersion: z.enum(DATA_PARSER_VERSIONS),
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

const timeOrderedNumericMatrixSchema = z.object({
  kind: z.literal('time-ordered-numeric-matrix'),
  sourceFingerprint: z.string(),
  layout: z.literal('column-major'),
  rowCount: z.number().int().positive(),
  timeColumn: z.object({ id: z.string(), name: z.string() }).strict(),
  timeAxis: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('calendar'), timestamps: z.instanceof(Float64Array) }).strict(),
    z.object({ kind: z.literal('ordinal'), values: z.instanceof(Float64Array) }).strict(),
  ]),
  columns: z.array(z.object({ id: z.string(), name: z.string() }).strict()),
  values: z.instanceof(Float64Array),
  validity: z.instanceof(Uint8Array),
  missingCells: z.number().int().nonnegative(),
}).strict()

const columnProfileSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('numeric-column-profile'),
    column: z.string(),
    sourceFingerprint: z.string(),
    count: z.number().int().nonnegative(),
    nullCount: z.number().int().nonnegative(),
    distinctCount: z.number().int().nonnegative(),
    zeroCount: z.number().int().nonnegative(),
    min: z.number().finite(),
    max: z.number().finite(),
    mean: z.number().finite(),
    standardDeviation: z.number().finite().nonnegative().nullable(),
    quartiles: z.object({ lower: z.number().finite(), median: z.number().finite(), upper: z.number().finite() }).strict(),
    histogram: z.object({ edges: z.array(z.number().finite()).min(2), counts: z.array(z.number().int().nonnegative()).min(1) }).strict(),
  }).strict(),
  z.object({
    kind: z.literal('categorical-column-profile'),
    column: z.string(),
    sourceFingerprint: z.string(),
    count: z.number().int().nonnegative(),
    nullCount: z.number().int().nonnegative(),
    distinctCount: z.number().int().nonnegative(),
    top: z.array(z.object({ value: z.string(), count: z.number().int().nonnegative() }).strict()).max(8),
  }).strict(),
])

/** Shape-only parse at the worker boundary; `parseColumnProfile` binds the result to its dataset profile. */
export function parseColumnProfileShape(value: unknown): Result<ColumnProfile, ColumnProfileBoundaryProblem> {
  const parsed = columnProfileSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-column-profile', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'numeric-column-profile' && parsed.data.histogram.edges.length !== parsed.data.histogram.counts.length + 1) {
    return err({ kind: 'invalid-column-profile', detail: 'Histogram edges and counts disagree.' })
  }
  return ok({
    ...parsed.data,
    column: brand<string, 'ColumnId'>(parsed.data.column),
    sourceFingerprint: brand<string, 'SourceFingerprint'>(parsed.data.sourceFingerprint),
  })
}

/** Parse a worker-returned column profile against the dataset profile it must belong to. */
export function parseColumnProfile(value: unknown, profile: DatasetProfile): Result<ColumnProfile, ColumnProfileBoundaryProblem> {
  const parsed = columnProfileSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-column-profile', detail: z.prettifyError(parsed.error) })
  const column = profile.columns.find((candidate) => candidate.id === parsed.data.column)
  if (column === undefined) return err({ kind: 'invalid-column-profile', detail: 'The column profile names a column outside the dataset profile.' })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint) {
    return err({ kind: 'invalid-column-profile', detail: 'The column profile belongs to a different source fingerprint.' })
  }
  if (parsed.data.kind === 'numeric-column-profile') {
    if (parsed.data.histogram.edges.length !== parsed.data.histogram.counts.length + 1) {
      return err({ kind: 'invalid-column-profile', detail: 'Histogram edges and counts disagree.' })
    }
    return ok({ ...parsed.data, column: column.id, sourceFingerprint: profile.source.fingerprint })
  }
  return ok({ ...parsed.data, column: column.id, sourceFingerprint: profile.source.fingerprint })
}

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
  const reading = fileReadingOf(parsed.data.source.format, parsed.data.source.declared)
  if (!reading.ok) {
    return err({ kind: 'inconsistent-profile', detail: 'A Parquet file stores its column types, so a profile of one declares none.' })
  }
  const expectedProfileId = datasetProfileId(fingerprint.value, reading.value, parsed.data.parser.packageVersion)
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
    source: { fileName: parsed.data.source.fileName, bytes: parsed.data.source.bytes, persistence: parsed.data.source.persistence, fingerprint: fingerprint.value, ...reading.value },
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

export function parseTimeOrderedNumericMatrix(
  value: unknown,
): Result<TimeOrderedNumericMatrix, NumericMatrixBoundaryProblem> {
  const parsed = timeOrderedNumericMatrixSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-numeric-matrix', detail: z.prettifyError(parsed.error) })
  const { timeColumn, timeAxis, ...numeric } = parsed.data
  const base = parseNullableNumericMatrix({ ...numeric, kind: 'nullable-numeric-matrix' })
  if (!base.ok) return base
  const timeId = columnIdFromWire(timeColumn.id, timeColumn.name)
  if (!timeId.ok) return err({ kind: 'invalid-numeric-matrix', detail: 'The time column identity is invalid.' })
  const times = parsed.data.timeAxis.kind === 'calendar' ? parsed.data.timeAxis.timestamps : parsed.data.timeAxis.values
  if (times.length !== parsed.data.rowCount) return err({ kind: 'invalid-numeric-matrix', detail: 'The time axis length does not match the materialized rows.' })
  for (let row = 0; row < times.length; row += 1) {
    const time = times[row]
    if (!Number.isFinite(time)) return err({ kind: 'invalid-numeric-matrix', detail: `Parsed time ${row} is not finite.` })
    if (row > 0 && time <= times[row - 1]) return err({ kind: 'invalid-numeric-matrix', detail: 'Parsed times are not strictly increasing.' })
  }
  return ok({
    ...base.value,
    kind: 'time-ordered-numeric-matrix',
    timeColumn: { id: timeId.value, name: timeColumn.name },
    timeAxis,
  })
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

export function validateTimeOrderedMatrixAgainstProfile(
  matrix: TimeOrderedNumericMatrix,
  profile: DatasetProfile,
): Result<TimeOrderedNumericMatrix, NumericMatrixBoundaryProblem> {
  if (matrix.sourceFingerprint !== profile.source.fingerprint || matrix.rowCount !== profile.rowCount) {
    return err({ kind: 'invalid-numeric-matrix', detail: 'The time-ordered matrix does not match its requested profile.' })
  }
  const time = profile.columns.find((column) => column.id === matrix.timeColumn.id)
  if (time === undefined || time.name !== matrix.timeColumn.name) return err({ kind: 'invalid-numeric-matrix', detail: 'The parsed time column is outside its requested profile.' })
  const numeric = validateNumericMatrixAgainstProfile({ ...matrix, kind: 'nullable-numeric-matrix' }, profile)
  return numeric.ok ? ok(matrix) : err(numeric.error)
}

/** Per-column facts for the schema table, computed after the profile so the first paint never waits on them. */
export interface ColumnSummary {
  readonly column: ColumnId
  readonly distinctCount: number
  /** DuckDB's own text rendering of the extremes; null when the column is empty. */
  readonly min: string | null
  readonly max: string | null
  readonly histogram: HistogramBins | null
  /** Value counts for columns with few distinct values, most frequent first. */
  readonly categories: readonly { readonly value: string; readonly count: number }[] | null
}

export interface DatasetSummary {
  readonly kind: 'dataset-summary'
  readonly sourceFingerprint: SourceFingerprint
  readonly columns: readonly ColumnSummary[]
}

export type DatasetSummaryProblem =
  | { readonly kind: 'source-changed'; readonly expected: SourceFingerprint; readonly actual: SourceFingerprint }
  | { readonly kind: 'summary-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export type PreviewFilter =
  | { readonly column: ColumnId; readonly kind: 'range'; readonly min: number | null; readonly max: number | null }
  | { readonly column: ColumnId; readonly kind: 'contains'; readonly text: string }
  | { readonly column: ColumnId; readonly kind: 'one-of'; readonly values: readonly string[] }
  | { readonly column: ColumnId; readonly kind: 'missing'; readonly missing: boolean }

export interface PreviewSort {
  readonly column: ColumnId
  readonly direction: 'asc' | 'desc'
}

/** A window onto the whole file, ordered and filtered in DuckDB. */
export interface PreviewQuery {
  readonly offset: number
  readonly limit: number
  readonly sort: PreviewSort | null
  readonly filters: readonly PreviewFilter[]
  /** Case-insensitive substring over every column's text form; empty means no search. */
  readonly search: string
}

export interface PreviewRow {
  /** One-based position in the source file, before sorting and filtering. */
  readonly index: number
  readonly cells: readonly PreviewCell[]
}

export interface PreviewWindow {
  readonly kind: 'preview-window'
  readonly sourceFingerprint: SourceFingerprint
  readonly offset: number
  /** Rows matching the filters and search over the whole file. */
  readonly total: number
  readonly rows: readonly PreviewRow[]
}

export type PreviewWindowProblem =
  | { readonly kind: 'source-changed'; readonly expected: SourceFingerprint; readonly actual: SourceFingerprint }
  | { readonly kind: 'preview-failed'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export const PREVIEW_WINDOW_LIMIT = 200

export const previewFilterSchema = z.discriminatedUnion('kind', [
  z.object({ column: z.string(), kind: z.literal('range'), min: z.number().finite().nullable(), max: z.number().finite().nullable() }).strict(),
  z.object({ column: z.string(), kind: z.literal('contains'), text: z.string() }).strict(),
  z.object({ column: z.string(), kind: z.literal('one-of'), values: z.array(z.string()).max(200) }).strict(),
  z.object({ column: z.string(), kind: z.literal('missing'), missing: z.boolean() }).strict(),
])

export const previewQuerySchema = z.object({
  offset: z.number().int().nonnegative(),
  limit: z.number().int().positive().max(1000),
  sort: z.object({ column: z.string(), direction: z.enum(['asc', 'desc']) }).strict().nullable(),
  filters: z.array(previewFilterSchema).max(32),
  search: z.string().max(200),
}).strict()

const histogramBinsSchema = z.object({
  edges: z.array(z.number().finite()).min(2),
  counts: z.array(z.number().int().nonnegative()).min(1),
}).strict()

const datasetSummarySchema = z.object({
  kind: z.literal('dataset-summary'),
  sourceFingerprint: z.string(),
  columns: z.array(z.object({
    column: z.string(),
    distinctCount: z.number().int().nonnegative(),
    min: z.string().nullable(),
    max: z.string().nullable(),
    histogram: histogramBinsSchema.nullable(),
    categories: z.array(z.object({ value: z.string(), count: z.number().int().nonnegative() }).strict()).nullable(),
  }).strict()),
}).strict()

const previewWindowSchema = z.object({
  kind: z.literal('preview-window'),
  sourceFingerprint: z.string(),
  offset: z.number().int().nonnegative(),
  total: z.number().int().nonnegative(),
  rows: z.array(z.object({ index: z.number().int().positive(), cells: z.array(previewCellSchema) }).strict()),
}).strict()

export type DatasetSummaryBoundaryProblem = { readonly kind: 'invalid-dataset-summary'; readonly detail: string }
export type PreviewWindowBoundaryProblem = { readonly kind: 'invalid-preview-window'; readonly detail: string }

/** Bind a summary to a profile: every column id must belong to it, and the fingerprint must match. */
export function parseDatasetSummary(value: unknown, profile: DatasetProfile): Result<DatasetSummary, DatasetSummaryBoundaryProblem> {
  const parsed = datasetSummarySchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-dataset-summary', detail: z.prettifyError(parsed.error) })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint) return err({ kind: 'invalid-dataset-summary', detail: 'The summary belongs to another source.' })
  const known = new Map(profile.columns.map((column) => [column.id as string, column.id]))
  const columns: ColumnSummary[] = []
  for (const column of parsed.data.columns) {
    const id = known.get(column.column)
    if (id === undefined) return err({ kind: 'invalid-dataset-summary', detail: `Column ${column.column} is not in the profile.` })
    if (column.histogram !== null && column.histogram.edges.length !== column.histogram.counts.length + 1) {
      return err({ kind: 'invalid-dataset-summary', detail: `Histogram for ${column.column} has mismatched edges and counts.` })
    }
    columns.push({ ...column, column: id })
  }
  return ok({ kind: 'dataset-summary', sourceFingerprint: profile.source.fingerprint, columns })
}

export function parsePreviewWindow(value: unknown, profile: DatasetProfile): Result<PreviewWindow, PreviewWindowBoundaryProblem> {
  const parsed = previewWindowSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-preview-window', detail: z.prettifyError(parsed.error) })
  if (parsed.data.sourceFingerprint !== profile.source.fingerprint) return err({ kind: 'invalid-preview-window', detail: 'The window belongs to another source.' })
  if (parsed.data.rows.some((row) => row.cells.length !== profile.columns.length)) {
    return err({ kind: 'invalid-preview-window', detail: 'A preview row does not match the column count.' })
  }
  return ok({ ...parsed.data, sourceFingerprint: profile.source.fingerprint })
}
import { DATA_PARSER_VERSIONS, DUCKDB_PACKAGE_VERSION, type DataParserVersion } from './dataEngine'
