import * as duckdb from '@duckdb/duckdb-wasm'
import { fileRelation } from './fileRelation'
import { fileReading } from '@/domain/fileReading'
import { inspectCalendar } from './calendar'
import { timestampSql, type TimeInterpretation, type TimePreview, type TimeSpacing } from '@/domain/timeInterpretation'
import { DUCKDB_PACKAGE_VERSION, DUCKDB_ENGINE_VERSION } from '@/domain/dataEngine'
import duckdbEhWorker from '@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url'
import duckdbMvpWorker from '@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url'
import { fingerprintFile } from './fingerprint'
import { assertNever, err, isNonEmpty, mapNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import {
  columnId,
  datasetProfileId,
  isNumericDuckDbType,
  type ColumnProfile,
  type ColumnProfileProblem,
  type DatasetProfile,
  type DatasetProfileProblem,
  type NullableNumericMatrix,
  type NumericMaterializationProblem,
  type TimeOrderedNumericMatrix,
  type TimeSeriesMaterializationProblem,
  type PhysicalColumnProfile,
  type PreviewCell,
  type SourceFingerprint,
  type ColumnSummary,
  type DatasetSummary,
  type DatasetSummaryProblem,
  type PreviewFilter,
  type PreviewQuery,
  type PreviewRow,
  type PreviewWindow,
  type PreviewWindowProblem,
  type ColumnId,
} from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'
import type { PanelDataProblem, PanelKeyMatrix, PanelLongMatrix, PanelPeriod, PanelStructureEvidence } from '@/domain/panel'

const MILLISECONDS_PER_DAY = 86_400_000

const PREVIEW_ROWS = 12

/** The engine binaries are served under the app's own origin at this path: from node_modules by the dev and preview servers, from R2 by the deployment. */
const engineBinary = (file: string): string => new URL(`/duckdb/${DUCKDB_PACKAGE_VERSION}/${file}`, self.location.origin).href

const LOCAL_BUNDLES: duckdb.DuckDBBundles = {
  mvp: { mainModule: engineBinary('duckdb-mvp.wasm'), mainWorker: duckdbMvpWorker },
  eh: { mainModule: engineBinary('duckdb-eh.wasm'), mainWorker: duckdbEhWorker },
}

export interface DuckDbEngine {
  readonly db: duckdb.AsyncDuckDB
  readonly version: string
}

let enginePromise: Promise<DuckDbEngine> | null = null

const localExtensionRepository = (): string =>
  new URL('/duckdb-extensions', self.location.origin).href.replace(/\/$/, '')

const scalarBigInt = (value: unknown, label: string): bigint => {
  if (typeof value === 'bigint') return value
  if (typeof value === 'number' && Number.isSafeInteger(value)) return BigInt(value)
  throw new Error(`DuckDB returned an invalid ${label}: ${String(value)}`)
}

async function configureOfflineEngine(db: duckdb.AsyncDuckDB, version: string): Promise<void> {
  if (version !== DUCKDB_ENGINE_VERSION) {
    throw new Error(`DuckDB engine ${version} does not match the self-hosted ${DUCKDB_ENGINE_VERSION} extensions.`)
  }

  const connection = await db.connect()
  try {
    const repository = sqlString(localExtensionRepository())
    await connection.query(`SET custom_extension_repository = ${repository}`)
    await connection.query('SET allow_community_extensions = false')
    await connection.query('SET autoinstall_known_extensions = false')
    await connection.query('SET autoload_known_extensions = false')
    await connection.query('LOAD parquet')
    await connection.query('LOAD icu')
    await connection.query("SET TimeZone = 'UTC'")

    // This parses named-zone timestamps on both sides of Amsterdam's 2024 spring DST transition.
    // If ICU is missing, mismatched, unsigned, or not actually loaded, engine startup is refused.
    const dst = await connection.query(`
      SELECT
        epoch(TIMESTAMPTZ '2024-03-31 00:30:00 Europe/Amsterdam')::BIGINT AS before_dst,
        epoch(TIMESTAMPTZ '2024-03-31 03:30:00 Europe/Amsterdam')::BIGINT AS after_dst
    `)
    const before = scalarBigInt(dst.getChild('before_dst')?.get(0), 'pre-DST epoch')
    const after = scalarBigInt(dst.getChild('after_dst')?.get(0), 'post-DST epoch')
    if (before !== 1_711_841_400n || after !== 1_711_848_600n) {
      throw new Error(`ICU DST probe returned ${before}/${after}; expected 1711841400/1711848600.`)
    }
  } finally {
    await connection.close()
  }
}

const startEngine = async (): Promise<DuckDbEngine> => {
  const bundle = await duckdb.selectBundle(LOCAL_BUNDLES)
  if (!bundle.mainWorker) throw new Error('This browser did not select a DuckDB worker bundle.')
  const worker = new Worker(bundle.mainWorker)
  const db = new duckdb.AsyncDuckDB(new duckdb.VoidLogger(), worker)
  try {
    await db.instantiate(bundle.mainModule, bundle.pthreadWorker)
    await db.open({ allowUnsignedExtensions: false, arrowLosslessConversion: true, maximumThreads: 1 })
    const version = await db.getVersion()
    await configureOfflineEngine(db, version)
    return { db, version }
  } catch (cause) {
    try {
      await db.terminate()
    } catch (terminationCause) {
      throw new Error(`${detailOf(cause)}; worker termination also failed: ${detailOf(terminationCause)}`)
    }
    throw cause
  }
}

const engine = (): Promise<DuckDbEngine> => {
  enginePromise ??= startEngine().catch((cause) => {
    enginePromise = null
    throw cause
  })
  return enginePromise
}

/** A separate database for work whose catalog and registered files must end with that work. */
export const isolatedDuckDbEngine = (): Promise<DuckDbEngine> => startEngine()

const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const sqlIdentifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const temporalPreview = (value: number, duckdbType: string): PreviewCell | null => {
  if (!/^(DATE|TIMESTAMP)/i.test(duckdbType)) return null
  const date = new Date(value)
  return Number.isNaN(date.valueOf()) ? null : { kind: 'temporal', value: date.toISOString() }
}

/** Arrow hands 128-bit integers and decimals over as sixteen little-endian bytes; this turns them back into a number. */
const wideInteger = (view: ArrayBufferView, signed: boolean): bigint => {
  const bytes = new Uint8Array(view.buffer, view.byteOffset, view.byteLength)
  let value = 0n
  for (let index = bytes.length - 1; index >= 0; index -= 1) value = (value << 8n) | BigInt(bytes[index]!)
  if (signed && (bytes[bytes.length - 1]! & 0x80) !== 0) value -= 1n << BigInt(bytes.length * 8)
  return value
}

/** A 128-bit integer or decimal as the digits it stands for; the decimal point is placed in the text, so wide values keep every digit. */
const wideValue = (value: ArrayBufferView, duckdbType: string): PreviewCell => {
  const integer = wideInteger(value, !duckdbType.startsWith('U'))
  const decimal = /^DECIMAL\(\d+,(\d+)\)$/.exec(duckdbType)
  const scale = decimal === null ? 0 : Number(decimal[1])
  if (scale === 0) return { kind: 'integer', value: integer.toString() }
  const magnitude = (integer < 0n ? -integer : integer).toString().padStart(scale + 1, '0')
  const text = `${integer < 0n ? '-' : ''}${magnitude.slice(0, -scale)}.${magnitude.slice(-scale)}`
  // Past fifteen digits a double would round the value, so the digits are shown as written instead.
  return magnitude.length > 15 ? { kind: 'text', value: text } : { kind: 'number', value: Number(text) }
}

const isWideNumberType = (duckdbType: string): boolean => /^(U?HUGEINT|DECIMAL\()/.test(duckdbType)

export const previewCell = (value: unknown, duckdbType: string): PreviewCell => {
  if (value === null || value === undefined) return { kind: 'null' }
  if (ArrayBuffer.isView(value)) {
    if (isWideNumberType(duckdbType) && value.byteLength === 16) return wideValue(value, duckdbType)
    return { kind: 'text', value: `${value.byteLength} bytes` }
  }
  if (typeof value === 'number') return temporalPreview(value, duckdbType) ?? { kind: 'number', value }
  if (typeof value === 'bigint') return { kind: 'integer', value: value.toString() }
  if (typeof value === 'boolean') return { kind: 'boolean', value }
  if (value instanceof Date) return { kind: 'temporal', value: value.toISOString() }
  return { kind: 'text', value: String(value) }
}

type CountProblem =
  | { readonly kind: 'unsafe-row-count'; readonly value: string }
  | { readonly kind: 'parse-failed'; readonly detail: string }

const safeCount = (value: unknown): Result<number, CountProblem> => {
  if (typeof value === 'bigint') {
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) return err({ kind: 'unsafe-row-count', value: value.toString() })
    return ok(Number(value))
  }
  if (typeof value === 'number' && Number.isSafeInteger(value) && value >= 0) return ok(value)
  return err({ kind: 'parse-failed', detail: `DuckDB returned an invalid row count: ${String(value)}` })
}

const detailOf = (cause: unknown): string => cause instanceof Error ? cause.message : String(cause)

const requiredString = (value: unknown, field: string): Result<string, DatasetProfileProblem> =>
  typeof value === 'string' && value.length > 0
    ? ok(value)
    : err({ kind: 'parse-failed', detail: `DuckDB returned an invalid ${field}: ${String(value)}` })

async function readProfile(
  connection: duckdb.AsyncDuckDBConnection,
  relation: string,
  source: SelectedSource,
  fingerprint: SourceFingerprint,
  engineVersion: string,
): Promise<Result<DatasetProfile, DatasetProfileProblem>> {
  const previewTable = await connection.query(`SELECT * FROM ${relation} LIMIT ${PREVIEW_ROWS}`)
  const fields = [...previewTable.schema.fields]
  if (!isNonEmpty(fields)) return err({ kind: 'no-columns' })

  const described = await connection.query(`DESCRIBE SELECT * FROM ${relation}`)
  const describedNames = described.getChild('column_name')
  const describedTypes = described.getChild('column_type')
  if (!describedNames || !describedTypes || described.numRows !== fields.length) {
    return err({ kind: 'parse-failed', detail: 'DuckDB physical schema did not match the Arrow preview schema.' })
  }

  const countExpressions = fields.map((field, index) =>
    `(count(*) - count(${sqlIdentifier(field.name)}))::UBIGINT AS ${sqlIdentifier(`null_${index}`)}`,
  )
  const counts = await connection.query(
    `SELECT count(*)::UBIGINT AS total_rows, ${countExpressions.join(', ')} FROM ${relation}`,
  )
  const parsedRowCount = safeCount(counts.getChild('total_rows')?.get(0))
  if (!parsedRowCount.ok) return parsedRowCount
  if (parsedRowCount.value === 0) return err({ kind: 'empty-dataset' })

  const builtColumns: PhysicalColumnProfile[] = []
  for (const [index, field] of fields.entries()) {
    const describedName = requiredString(describedNames.get(index), 'column name')
    if (!describedName.ok) return describedName
    if (describedName.value !== field.name) {
      return err({ kind: 'parse-failed', detail: `DuckDB described ${describedName.value} where Arrow returned ${field.name}.` })
    }
    const describedType = requiredString(describedTypes.get(index), `physical type for ${field.name}`)
    if (!describedType.ok) return describedType
    const parsedNullCount = safeCount(counts.getChild(`null_${index}`)?.get(0))
    if (!parsedNullCount.ok) return parsedNullCount
    builtColumns.push({
      id: columnId(index, field.name),
      name: field.name,
      duckdbType: describedType.value,
      nullable: parsedNullCount.value > 0,
      nullCount: parsedNullCount.value,
    })
  }
  if (!isNonEmpty(builtColumns)) return err({ kind: 'no-columns' })

  const builtPreview: NonEmptyArray<PreviewCell>[] = []
  for (let rowIndex = 0; rowIndex < previewTable.numRows; rowIndex += 1) {
    const row = builtColumns.map((_, columnIndex) =>
      previewCell(previewTable.getChildAt(columnIndex)?.get(rowIndex), builtColumns[columnIndex].duckdbType),
    )
    if (!isNonEmpty(row)) return err({ kind: 'no-columns' })
    builtPreview.push(row)
  }
  if (!isNonEmpty(builtPreview)) {
    return err({ kind: 'parse-failed', detail: 'DuckDB counted data rows but returned an empty bounded preview.' })
  }

  return ok({
    id: datasetProfileId(fingerprint, source),
    source: {
      fingerprint,
      fileName: source.name,
      bytes: source.bytes,
      persistence: { kind: 'ephemeral' },
      ...fileReading(source),
    },
    parser: {
      kind: 'duckdb-wasm',
      packageVersion: DUCKDB_PACKAGE_VERSION,
      engineVersion,
    },
    rowCount: parsedRowCount.value,
    columns: builtColumns,
    preview: builtPreview,
  })
}

/** Canonical CSV/TSV/Parquet profile. DuckDB reads from the browser File handle. Arrow vectors are
 * accessed with `get(row)`, which applies logical validity; `toArray()` is intentionally forbidden. */
export async function profileSource(source: SelectedSource): Promise<Result<DatasetProfile, DatasetProfileProblem>> {
  const fingerprint = await fingerprintFile(source.file)
  if (!fingerprint.ok) return fingerprint

  let running: DuckDbEngine
  try {
    running = await engine()
  } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }

  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.db.registerFileHandle(
      registeredPath,
      source.file,
      duckdb.DuckDBDataProtocol.BROWSER_FILEREADER,
      true,
    )
  } catch (cause) {
    return err({ kind: 'registration-failed', detail: detailOf(cause) })
  }

  const relation = fileRelation(source, registeredPath)
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<DatasetProfile, DatasetProfileProblem>
  try {
    connection = await running.db.connect()
    outcome = await readProfile(connection, relation, source, fingerprint.value, running.version)
  } catch (cause) {
    outcome = err({ kind: 'parse-failed', detail: detailOf(cause) })
  }

  const cleanupFailures: string[] = []
  if (connection) {
    try {
      await connection.close()
    } catch (cause) {
      cleanupFailures.push(`connection: ${detailOf(cause)}`)
    }
  }
  try {
    await running.db.dropFile(registeredPath)
  } catch (cause) {
    cleanupFailures.push(`source handle: ${detailOf(cause)}`)
  }
  return cleanupFailures.length > 0
    ? err({ kind: 'cleanup-failed', detail: cleanupFailures.join('; ') })
    : outcome
}

const setValid = (validity: Uint8Array, index: number): void => {
  validity[index >> 3] |= 1 << (index & 7)
}

const selectedNumericColumns = (
  profile: DatasetProfile,
  requestedColumnIds: NonEmptyArray<string>,
): Result<NonEmptyArray<PhysicalColumnProfile>, NumericMaterializationProblem> => {
  const selected: PhysicalColumnProfile[] = []
  const seen = new Set<string>()
  for (const id of requestedColumnIds) {
    if (seen.has(id)) return err({ kind: 'duplicate-column', id })
    seen.add(id)
    const column = profile.columns.find((candidate) => candidate.id === id)
    if (!column) return err({ kind: 'column-not-found', id })
    if (!isNumericDuckDbType(column.duckdbType)) return err({ kind: 'non-numeric-column', name: column.name, duckdbType: column.duckdbType })
    selected.push(column)
  }
  return isNonEmpty(selected) ? ok(selected) : err({ kind: 'materialization-failed', detail: 'No numeric columns were selected.' })
}

interface NumericQueryTable {
  readonly numRows: number
  getChild(name: string): { get(row: number): unknown } | null
}

const numericMatrixFromTable = (
  table: NumericQueryTable,
  profile: DatasetProfile,
  selected: NonEmptyArray<PhysicalColumnProfile>,
): Result<NullableNumericMatrix, NumericMaterializationProblem> => {
  if (table.numRows !== profile.rowCount) return err({ kind: 'materialization-failed', detail: 'The materialized row count changed since profiling.' })
  const cellCount = profile.rowCount * selected.length
  if (!Number.isSafeInteger(cellCount)) return err({ kind: 'materialization-failed', detail: 'The selected matrix is too large for browser-safe indexing.' })
  const values = new Float64Array(cellCount)
  values.fill(Number.NaN)
  const validity = new Uint8Array(Math.ceil(cellCount / 8))
  let missingCells = 0
  for (const [columnIndex, column] of selected.entries()) {
    const vector = table.getChild(`hirmos_numeric_${columnIndex}`)
    if (!vector) return err({ kind: 'materialization-failed', detail: `DuckDB omitted numeric column ${column.name}.` })
    for (let row = 0; row < profile.rowCount; row += 1) {
      const value = vector.get(row)
      const target = columnIndex * profile.rowCount + row
      if (value === null || value === undefined) {
        missingCells += 1
      } else if (typeof value !== 'number' || !Number.isFinite(value)) {
        return err({ kind: 'non-finite-value', name: column.name, row })
      } else {
        values[target] = value
        setValid(validity, target)
      }
    }
  }
  return ok({
    kind: 'nullable-numeric-matrix',
    sourceFingerprint: profile.source.fingerprint,
    layout: 'column-major',
    rowCount: profile.rowCount,
    columns: mapNonEmpty(selected, ({ id, name }) => ({ id, name })),
    values,
    validity,
    missingCells,
  })
}

export async function materializeNumericColumns(
  source: SelectedSource,
  profile: DatasetProfile,
  requestedColumnIds: NonEmptyArray<string>,
): Promise<Result<NullableNumericMatrix, NumericMaterializationProblem>> {
  const selected = selectedNumericColumns(profile, requestedColumnIds)
  if (!selected.ok) return selected
  const projections = selected.value.map((column, index) =>
    `CAST(${sqlIdentifier(column.name)} AS DOUBLE) AS ${sqlIdentifier(`hirmos_numeric_${index}`)}`,
  )
  return withSource(
    source,
    profile,
    (detail): NumericMaterializationProblem => ({ kind: 'materialization-failed', detail }),
    (expected, actual): NumericMaterializationProblem => ({ kind: 'source-changed', expected, actual }),
    async (connection, relation) => numericMatrixFromTable(await connection.query(`SELECT ${projections.join(', ')} FROM ${relation}`), profile, selected.value),
  )
}

/** DuckDB owns calendar arithmetic; the round trip rejects overflowing ISO week numbers. */
function timeExpression(column: string, numeric: boolean, interpretation: TimeInterpretation): { readonly kind: 'ordinal' | 'calendar'; readonly sql: string } {
  switch (interpretation.kind) {
    case 'source-type': return timeExpression(column, numeric, { kind: numeric ? 'ordinal' : 'timestamp' })
    case 'ordinal': return { kind: 'ordinal', sql: `TRY_CAST(${column} AS DOUBLE)` }
    case 'timestamp':
    case 'date-format':
    case 'iso-week': return { kind: 'calendar', sql: `epoch_ms(${timestampSql(column, interpretation, sqlString)})` }
    default: return assertNever(interpretation)
  }
}

export async function previewTimeColumn(source: SelectedSource, profile: DatasetProfile, columnId: ColumnId, interpretation: TimeInterpretation, calendar?: import('@/domain/calendar').CalendarRequest): Promise<Result<TimePreview, TimeSeriesMaterializationProblem>> {
  const column = profile.columns.find((candidate) => candidate.id === columnId)
  if (column === undefined) return err({ kind: 'column-not-found', id: columnId })
  const time = sqlIdentifier(column.name)
  const parsed = timeExpression(time, isNumericDuckDbType(column.duckdbType), interpretation)
  return withSource(
    source, profile,
    (detail): TimeSeriesMaterializationProblem => ({ kind: 'materialization-failed', detail }),
    (expected, actual): TimeSeriesMaterializationProblem => ({ kind: 'source-changed', expected, actual }),
    async (connection, relation) => {
      const result = await connection.query(`SELECT CAST(${time} AS VARCHAR) AS original, ${parsed.sql} AS parsed FROM ${relation} LIMIT 12`)
      const rows = Array.from({ length: result.numRows }, (_, index) => {
        const original = result.getChild('original')?.get(index)
        const value = result.getChild('parsed')?.get(index)
        const number = value == null ? NaN : Number(value)
        return { original: original == null ? null : String(original), parsed: Number.isFinite(number) ? number : null }
      })
      const spacing = await ((): Promise<TimeSpacing> => {
        if (parsed.kind === 'ordinal') return Promise.resolve({ kind: 'ordinal' })
        return connection.query(`
          WITH distinct_times AS (SELECT DISTINCT ${parsed.sql} AS time FROM ${relation} WHERE ${parsed.sql} IS NOT NULL),
               gaps AS (SELECT time - lag(time) OVER (ORDER BY time) AS gap FROM distinct_times)
          SELECT gap FROM gaps WHERE gap IS NOT NULL GROUP BY gap ORDER BY count(*) DESC, gap ASC LIMIT 1
        `).then((gaps) => {
          const gap = gaps.numRows === 0 ? null : gaps.getChild('gap')?.get(0)
          return gap == null ? { kind: 'single-period' } : { kind: 'days', modal: Number(gap) / MILLISECONDS_PER_DAY }
        })
      })()
      if (calendar === undefined || parsed.kind === 'ordinal') return ok({ kind: parsed.kind, rows, spacing })
      const unit = calendar.unitColumn === undefined ? undefined : profile.columns.find((candidate) => candidate.id === calendar.unitColumn)
      if (calendar.unitColumn !== undefined && unit === undefined) return err({ kind: 'materialization-failed', detail: 'The unit column is outside the supplied profile.' })
      const unitSql = unit === undefined ? "'Series'" : `CAST(${sqlIdentifier(unit.name)} AS VARCHAR)`
      const axis = await connection.query(`SELECT ${unitSql} AS unit, ${parsed.sql} AS time FROM ${relation} ORDER BY unit, time`)
      const groups = new Map<string, number[]>()
      for (let index = 0; index < axis.numRows; index++) {
        const name = axis.getChild('unit')?.get(index)
        const value = axis.getChild('time')?.get(index)
        if (name == null || value == null || !Number.isSafeInteger(Number(value))) return err({ kind: 'materialization-failed', detail: 'Calendar coverage needs valid dates and non-missing unit identifiers on every row.' })
        const key = String(name)
        const times = groups.get(key)
        if (times === undefined) groups.set(key, [Number(value)])
        else times.push(Number(value))
      }
      const report = await inspectCalendar(calendar.schedule, Array.from(groups, ([name, times]) => ({ name, times })))
      return ok({ kind: parsed.kind, rows, spacing, calendar: report })
    },
    interpretation.kind === 'source-type' ? undefined : column.name,
  )
}

/** Read the analysis columns together with one parsed temporal key and sort them chronologically. */
export async function materializeTimeSeriesColumns(
  source: SelectedSource,
  profile: DatasetProfile,
  timeColumnId: ColumnId,
  requestedColumnIds: NonEmptyArray<string>,
  interpretation: TimeInterpretation = { kind: 'source-type' },
): Promise<Result<TimeOrderedNumericMatrix, TimeSeriesMaterializationProblem>> {
  const timeColumn = profile.columns.find((candidate) => candidate.id === timeColumnId)
  if (timeColumn === undefined) return err({ kind: 'column-not-found', id: timeColumnId })
  const selected = selectedNumericColumns(profile, requestedColumnIds)
  if (!selected.ok) return selected
  const projections = selected.value.map((column, index) =>
    `CAST(${sqlIdentifier(column.name)} AS DOUBLE) AS ${sqlIdentifier(`hirmos_numeric_${index}`)}`,
  )
  const time = sqlIdentifier(timeColumn.name)
  const parsedTime = timeExpression(time, isNumericDuckDbType(timeColumn.duckdbType), interpretation)
  const ordinal = parsedTime.kind === 'ordinal'
  const timeProjection = parsedTime.sql
  return withSource(
    source,
    profile,
    (detail): TimeSeriesMaterializationProblem => ({ kind: 'materialization-failed', detail }),
    (expected, actual): TimeSeriesMaterializationProblem => ({ kind: 'source-changed', expected, actual }),
    async (connection, relation) => {
      const table = await connection.query(`
        SELECT ${timeProjection} AS hirmos_time, ${projections.join(', ')}
        FROM ${relation}
        ORDER BY hirmos_time ASC NULLS LAST
      `)
      const base = numericMatrixFromTable(table, profile, selected.value)
      if (!base.ok) return base
      const vector = table.getChild('hirmos_time')
      if (vector === null) return err({ kind: 'materialization-failed', detail: `DuckDB omitted parsed time column ${timeColumn.name}.` })
      const times = new Float64Array(profile.rowCount)
      for (let row = 0; row < profile.rowCount; row += 1) {
        const value = vector.get(row)
        if (value === null || value === undefined) return err({ kind: 'time-value-unparseable', name: timeColumn.name, row })
        const timestamp = scalarNumber(value, 'time value')
        if (!Number.isFinite(timestamp)) return err({ kind: 'time-value-unparseable', name: timeColumn.name, row })
        if (row > 0 && timestamp === times[row - 1]) return err({ kind: 'duplicate-time-value', name: timeColumn.name, row })
        times[row] = timestamp
      }
      return ok({
        ...base.value,
        kind: 'time-ordered-numeric-matrix',
        timeColumn: { id: timeColumn.id, name: timeColumn.name },
        timeAxis: ordinal ? { kind: 'ordinal', values: times } : { kind: 'calendar', timestamps: times },
      })
    },
    interpretation.kind === 'source-type' ? undefined : timeColumn.name,
  )
}

const panelColumn = (profile: DatasetProfile, id: ColumnId): Result<PhysicalColumnProfile, PanelDataProblem> => {
  const column = profile.columns.find((candidate) => candidate.id === id)
  return column === undefined ? err({ kind: 'column-not-found', id }) : ok(column)
}

const firstDuplicate = (requested: readonly ColumnId[]): ColumnId | null => {
  const seen = new Set<ColumnId>()
  for (const id of requested) {
    if (seen.has(id)) return id
    seen.add(id)
  }
  return null
}

async function verifiedPanelSource(
  source: SelectedSource,
  profile: DatasetProfile,
): Promise<Result<DuckDbEngine, PanelDataProblem>> {
  const fingerprint = await fingerprintFile(source.file)
  if (!fingerprint.ok) return err({ kind: 'panel-data-failed', detail: fingerprint.error.detail })
  if (fingerprint.value !== profile.source.fingerprint) {
    return err({ kind: 'source-changed', expected: profile.source.fingerprint, actual: fingerprint.value })
  }
  try { return ok(await engine()) } catch (cause) { return err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
}


/** Validate the observational panel keys without reading the scientific values. */
export async function inspectPanelStructure(
  source: SelectedSource,
  profile: DatasetProfile,
  unitColumnId: ColumnId,
  timeColumnId: ColumnId,
): Promise<Result<PanelStructureEvidence, PanelDataProblem>> {
  const duplicate = firstDuplicate([unitColumnId, timeColumnId])
  if (duplicate !== null) return err({ kind: 'duplicate-column', id: duplicate })
  const boundUnit = panelColumn(profile, unitColumnId)
  if (!boundUnit.ok) return boundUnit
  const boundTime = panelColumn(profile, timeColumnId)
  if (!boundTime.ok) return boundTime
  const unitColumn = boundUnit.value
  const timeColumn = boundTime.value
  const running = await verifiedPanelSource(source, profile)
  if (!running.ok) return running
  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.value.db.registerFileHandle(registeredPath, source.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
  } catch (cause) { return err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  const relation = fileRelation(profile.source, registeredPath)
  const unit = sqlIdentifier(unitColumn.name)
  const time = sqlIdentifier(timeColumn.name)
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<PanelStructureEvidence, PanelDataProblem>
  try {
    connection = await running.value.db.connect()
    const summary = await connection.query(`
      WITH panel AS (SELECT ${unit} AS unit_key, ${time} AS time_key FROM ${relation}),
      duplicates AS (
        SELECT count(*)::UBIGINT AS duplicate_keys
        FROM (SELECT unit_key, time_key FROM panel WHERE unit_key IS NOT NULL AND time_key IS NOT NULL GROUP BY unit_key, time_key HAVING count(*) > 1)
      )
      SELECT
        count(*)::UBIGINT AS observations,
        count(DISTINCT unit_key)::UBIGINT AS units,
        count(DISTINCT time_key)::UBIGINT AS periods,
        (count(*) - count(unit_key))::UBIGINT AS missing_units,
        (count(*) - count(time_key))::UBIGINT AS missing_times,
        (SELECT duplicate_keys FROM duplicates)::UBIGINT AS duplicate_keys
      FROM panel
    `)
    const number = (name: string) => scalarNumber(summary.getChild(name)?.get(0), name)
    const observations = number('observations')
    const units = number('units')
    const periods = number('periods')
    const duplicateKeys = number('duplicate_keys')
    const missingUnitKeys = number('missing_units')
    const missingTimeKeys = number('missing_times')
    outcome = ok({
      kind: 'panel-structure', sourceFingerprint: profile.source.fingerprint,
      unitColumn: unitColumn.id, timeColumn: timeColumn.id, observations, units, periods,
      duplicateKeys, missingUnitKeys, missingTimeKeys,
      balanced: duplicateKeys === 0 && missingUnitKeys === 0 && missingTimeKeys === 0 && observations === units * periods,
    })
  } catch (cause) { outcome = err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  try { if (connection !== null) await connection.close(); await running.value.db.dropFile(registeredPath) } catch (cause) {
    return err({ kind: 'panel-data-failed', detail: `Panel cleanup failed: ${detailOf(cause)}` })
  }
  return outcome
}

/** Materialize only panel keys, so imputed analysis columns do not make structural ordering fail. */
export async function materializePanelKeys(
  source: SelectedSource,
  profile: DatasetProfile,
  unitColumnId: ColumnId,
  timeColumnId: ColumnId,
): Promise<Result<PanelKeyMatrix, PanelDataProblem>> {
  if (unitColumnId === timeColumnId) return err({ kind: 'duplicate-column', id: unitColumnId })
  const boundUnit = panelColumn(profile, unitColumnId)
  if (!boundUnit.ok) return boundUnit
  const boundTime = panelColumn(profile, timeColumnId)
  if (!boundTime.ok) return boundTime
  const unitColumn = boundUnit.value
  const timeColumn = boundTime.value
  const running = await verifiedPanelSource(source, profile)
  if (!running.ok) return running
  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.value.db.registerFileHandle(registeredPath, source.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
  } catch (cause) { return err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  const relation = fileRelation(profile.source, registeredPath)
  const unit = sqlIdentifier(unitColumn.name)
  const time = sqlIdentifier(timeColumn.name)
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<PanelKeyMatrix, PanelDataProblem>
  try {
    connection = await running.value.db.connect()
    // Source row order, as the value matrix is read: the rank window alone would return the rows sorted by time.
    const table = await connection.query(`
      WITH source AS (SELECT row_number() OVER () AS __row, ${unit} AS unit_value, ${time} AS time_value FROM ${relation})
      SELECT CAST(unit_value AS VARCHAR) AS unit_label,
             (dense_rank() OVER (ORDER BY time_value) - 1)::INTEGER AS time_code,
             CAST(time_value AS VARCHAR) AS time_label
      FROM source
      ORDER BY __row
    `)
    const unitVector = table.getChild('unit_label')
    const timeVector = table.getChild('time_code')
    const timeLabelVector = table.getChild('time_label')
    const units: string[] = []
    const periodCodes: number[] = []
    const periodsByCode = new Map<number, string>()
    let problem: PanelDataProblem | null = null
    for (let row = 0; row < table.numRows; row += 1) {
      const unitValue = unitVector?.get(row)
      const timeValue = timeVector?.get(row)
      const timeLabel = timeLabelVector?.get(row)
      if (unitValue === null || unitValue === undefined || String(unitValue) === '') { problem = { kind: 'missing-key', name: unitColumn.name, row }; break }
      if (timeValue === null || timeValue === undefined || timeLabel === null || timeLabel === undefined) { problem = { kind: 'missing-key', name: timeColumn.name, row }; break }
      const periodCode = scalarNumber(timeValue, 'panel time code')
      const periodLabel = String(timeLabel)
      const recordedLabel = periodsByCode.get(periodCode)
      if (recordedLabel !== undefined && recordedLabel !== periodLabel) {
        problem = { kind: 'panel-data-failed', detail: `Panel period code ${periodCode} is associated with both ${recordedLabel} and ${periodLabel}.` }
        break
      }
      units.push(String(unitValue))
      periodCodes.push(periodCode)
      periodsByCode.set(periodCode, periodLabel)
    }
    const periods = [...periodsByCode].sort(([left], [right]) => left - right).map(([code, label]): PanelPeriod => ({ code, label }))
    outcome = problem !== null || !isNonEmpty(units) || !isNonEmpty(periodCodes) || !isNonEmpty(periods)
      ? err(problem ?? { kind: 'panel-data-failed', detail: 'The panel-key query returned no rows.' })
      : ok({ kind: 'panel-key-matrix', sourceFingerprint: profile.source.fingerprint, rowCount: table.numRows, units, periodCodes, periods })
  } catch (cause) { outcome = err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  try { if (connection !== null) await connection.close(); await running.value.db.dropFile(registeredPath) } catch (cause) {
    return err({ kind: 'panel-data-failed', detail: `Panel cleanup failed: ${detailOf(cause)}` })
  }
  return outcome
}

/** Materialize one long panel for the Rust estimator: outcome then treatment, column-major. */
export async function materializePanelLong(
  source: SelectedSource,
  profile: DatasetProfile,
  unitColumnId: ColumnId,
  timeColumnId: ColumnId,
  outcomeColumnId: ColumnId,
  treatmentColumnId: ColumnId,
  covariateIds: readonly ColumnId[] = [],
): Promise<Result<PanelLongMatrix, PanelDataProblem>> {
  const duplicate = firstDuplicate([unitColumnId, timeColumnId, outcomeColumnId, treatmentColumnId, ...covariateIds])
  if (duplicate !== null) return err({ kind: 'duplicate-column', id: duplicate })
  const boundUnit = panelColumn(profile, unitColumnId)
  if (!boundUnit.ok) return boundUnit
  const boundTime = panelColumn(profile, timeColumnId)
  if (!boundTime.ok) return boundTime
  const boundOutcome = panelColumn(profile, outcomeColumnId)
  if (!boundOutcome.ok) return boundOutcome
  const boundTreatment = panelColumn(profile, treatmentColumnId)
  if (!boundTreatment.ok) return boundTreatment
  const unitColumn = boundUnit.value
  const timeColumn = boundTime.value
  const outcomeColumn = boundOutcome.value
  const treatmentColumn = boundTreatment.value
  const numericColumns = [outcomeColumn, treatmentColumn]
  for (const id of covariateIds) {
    const bound = panelColumn(profile, id)
    if (!bound.ok) return bound
    numericColumns.push(bound.value)
  }
  for (const column of numericColumns) {
    if (!isNumericDuckDbType(column.duckdbType)) return err({ kind: 'non-numeric-column', name: column.name, duckdbType: column.duckdbType })
  }
  const running = await verifiedPanelSource(source, profile)
  if (!running.ok) return running
  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.value.db.registerFileHandle(registeredPath, source.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
  } catch (cause) { return err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  const relation = fileRelation(profile.source, registeredPath)
  const unit = sqlIdentifier(unitColumn.name)
  const time = sqlIdentifier(timeColumn.name)
  const numericProjection = numericColumns.map((column, index) => `CAST(${sqlIdentifier(column.name)} AS DOUBLE) AS value_${index}`).join(', ')
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<PanelLongMatrix, PanelDataProblem>
  try {
    connection = await running.value.db.connect()
    const table = await connection.query(`
      SELECT CAST(${unit} AS VARCHAR) AS unit_label,
             (dense_rank() OVER (ORDER BY ${time}) - 1)::INTEGER AS time_code,
             CAST(${time} AS VARCHAR) AS time_label,
             ${numericProjection}
      FROM ${relation}
    `)
    const unitVector = table.getChild('unit_label')
    const timeVector = table.getChild('time_code')
    const timeLabelVector = table.getChild('time_label')
    const numericVectors = numericColumns.map((column, index) => ({ column, vector: table.getChild(`value_${index}`) }))
    const units: string[] = []
    const periodCodes: number[] = []
    const periodsByCode = new Map<number, string>()
    const values = new Float64Array(table.numRows * numericColumns.length)
    let problem: PanelDataProblem | null = null
    for (let row = 0; row < table.numRows; row += 1) {
      const unitValue = unitVector?.get(row)
      const timeValue = timeVector?.get(row)
      const timeLabel = timeLabelVector?.get(row)
      if (unitValue === null || unitValue === undefined || String(unitValue) === '') { problem = { kind: 'missing-key', name: unitColumn.name, row }; break }
      if (timeValue === null || timeValue === undefined || timeLabel === null || timeLabel === undefined) { problem = { kind: 'missing-key', name: timeColumn.name, row }; break }
      const periodCode = scalarNumber(timeValue, 'panel time code')
      const periodLabel = String(timeLabel)
      const recordedLabel = periodsByCode.get(periodCode)
      if (recordedLabel !== undefined && recordedLabel !== periodLabel) {
        problem = { kind: 'panel-data-failed', detail: `Panel period code ${periodCode} is associated with both ${recordedLabel} and ${periodLabel}.` }
        break
      }
      units.push(String(unitValue))
      periodCodes.push(periodCode)
      periodsByCode.set(periodCode, periodLabel)
      for (const [columnIndex, pair] of numericVectors.entries()) {
        const value = pair.vector?.get(row)
        if (value === null || value === undefined) { problem = { kind: 'missing-value', name: pair.column.name, row }; break }
        if (typeof value !== 'number' || !Number.isFinite(value)) { problem = { kind: 'non-finite-value', name: pair.column.name, row }; break }
        values[columnIndex * table.numRows + row] = value
      }
      if (problem !== null) break
    }
    const periods = [...periodsByCode]
      .sort(([left], [right]) => left - right)
      .map(([code, label]): PanelPeriod => ({ code, label }))
    outcome = problem !== null || !isNonEmpty(units) || !isNonEmpty(periodCodes) || !isNonEmpty(periods)
      ? err(problem ?? { kind: 'panel-data-failed', detail: 'The panel query returned no rows.' })
      : ok({ kind: 'panel-long-matrix', sourceFingerprint: profile.source.fingerprint, rowCount: table.numRows, units, periodCodes, periods, values, covariates: covariateIds })
  } catch (cause) { outcome = err({ kind: 'panel-data-failed', detail: detailOf(cause) }) }
  try { if (connection !== null) await connection.close(); await running.value.db.dropFile(registeredPath) } catch (cause) {
    return err({ kind: 'panel-data-failed', detail: `Panel cleanup failed: ${detailOf(cause)}` })
  }
  return outcome
}

const scalarNumber = (value: unknown, label: string): number => {
  if (typeof value === 'number') return value
  if (typeof value === 'bigint') return Number(value)
  throw new Error(`DuckDB returned an invalid ${label}: ${String(value)}`)
}

const optionalNumber = (value: unknown): number | null => (value === null || value === undefined ? null : scalarNumber(value, 'statistic'))

const MAX_HISTOGRAM_BINS = 32
const TOP_VALUES = 8

/** Describe one column on demand: summary statistics and a histogram for numeric types, frequencies otherwise. */
export async function profileColumn(
  source: SelectedSource,
  profile: DatasetProfile,
  requestedColumnId: string,
): Promise<Result<ColumnProfile, ColumnProfileProblem>> {
  const fingerprint = await fingerprintFile(source.file)
  if (!fingerprint.ok) return err({ kind: 'column-profile-failed', detail: fingerprint.error.detail })
  if (fingerprint.value !== profile.source.fingerprint) {
    return err({ kind: 'source-changed', expected: profile.source.fingerprint, actual: fingerprint.value })
  }
  const column = profile.columns.find((candidate) => candidate.id === requestedColumnId)
  if (!column) return err({ kind: 'column-not-found', id: requestedColumnId })

  let running: DuckDbEngine
  try {
    running = await engine()
  } catch (cause) {
    return err({ kind: 'column-profile-failed', detail: detailOf(cause) })
  }
  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.db.registerFileHandle(registeredPath, source.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
  } catch (cause) {
    return err({ kind: 'column-profile-failed', detail: detailOf(cause) })
  }
  const relation = fileRelation(profile.source, registeredPath)
  const name = sqlIdentifier(column.name)

  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<ColumnProfile, ColumnProfileProblem>
  try {
    connection = await running.db.connect()
    if (isNumericDuckDbType(column.duckdbType)) {
      const summary = await connection.query(`
        SELECT
          count(${name}) AS present,
          count(*) - count(${name}) AS nulls,
          count(DISTINCT ${name}) AS distinct_values,
          count(*) FILTER (WHERE ${name} = 0) AS zeros,
          min(CAST(${name} AS DOUBLE)) AS minimum,
          max(CAST(${name} AS DOUBLE)) AS maximum,
          avg(CAST(${name} AS DOUBLE)) AS mean,
          stddev_samp(CAST(${name} AS DOUBLE)) AS deviation,
          quantile_cont(CAST(${name} AS DOUBLE), 0.25) AS lower_quartile,
          quantile_cont(CAST(${name} AS DOUBLE), 0.5) AS median,
          quantile_cont(CAST(${name} AS DOUBLE), 0.75) AS upper_quartile
        FROM ${relation}
      `)
      const row = (field: string) => summary.getChild(field)?.get(0)
      const present = scalarNumber(row('present'), 'count')
      if (present === 0) {
        outcome = err({ kind: 'column-profile-failed', detail: `${column.name} has no non-null values to describe.` })
      } else {
        const minimum = scalarNumber(row('minimum'), 'minimum')
        const maximum = scalarNumber(row('maximum'), 'maximum')
        const bins = minimum === maximum ? 1 : Math.max(4, Math.min(MAX_HISTOGRAM_BINS, Math.ceil(Math.sqrt(present))))
        const width = (maximum - minimum) / bins
        const counts = new Array<number>(bins).fill(0)
        if (bins === 1) {
          counts[0] = present
        } else {
          const buckets = await connection.query(`
            SELECT
              least(greatest(floor((CAST(${name} AS DOUBLE) - ${minimum}) / ${width}), 0), ${bins - 1})::INTEGER AS bucket,
              count(*) AS n
            FROM ${relation}
            WHERE ${name} IS NOT NULL
            GROUP BY bucket
            ORDER BY bucket
          `)
          const bucketVector = buckets.getChild('bucket')
          const countVector = buckets.getChild('n')
          for (let index = 0; index < buckets.numRows; index += 1) {
            const bucket = scalarNumber(bucketVector?.get(index), 'bucket')
            counts[bucket] = scalarNumber(countVector?.get(index), 'bucket count')
          }
        }
        const edges = Array.from({ length: bins + 1 }, (_, index) => (index === bins ? maximum : minimum + width * index))
        outcome = ok({
          kind: 'numeric-column-profile',
          column: column.id,
          sourceFingerprint: profile.source.fingerprint,
          count: present,
          nullCount: scalarNumber(row('nulls'), 'null count'),
          distinctCount: scalarNumber(row('distinct_values'), 'distinct count'),
          zeroCount: scalarNumber(row('zeros') ?? 0, 'zero count'),
          min: minimum,
          max: maximum,
          mean: scalarNumber(row('mean'), 'mean'),
          standardDeviation: optionalNumber(row('deviation')),
          quartiles: {
            lower: scalarNumber(row('lower_quartile'), 'lower quartile'),
            median: scalarNumber(row('median'), 'median'),
            upper: scalarNumber(row('upper_quartile'), 'upper quartile'),
          },
          histogram: { edges, counts },
        })
      }
    } else {
      const summary = await connection.query(`
        SELECT count(${name}) AS present, count(*) - count(${name}) AS nulls, count(DISTINCT ${name}) AS distinct_values
        FROM ${relation}
      `)
      const top = await connection.query(`
        SELECT CAST(${name} AS VARCHAR) AS value, count(*) AS n
        FROM ${relation}
        WHERE ${name} IS NOT NULL
        GROUP BY value
        ORDER BY n DESC, value ASC
        LIMIT ${TOP_VALUES}
      `)
      const valueVector = top.getChild('value')
      const countVector = top.getChild('n')
      outcome = ok({
        kind: 'categorical-column-profile',
        column: column.id,
        sourceFingerprint: profile.source.fingerprint,
        count: scalarNumber(summary.getChild('present')?.get(0), 'count'),
        nullCount: scalarNumber(summary.getChild('nulls')?.get(0), 'null count'),
        distinctCount: scalarNumber(summary.getChild('distinct_values')?.get(0), 'distinct count'),
        top: Array.from({ length: top.numRows }, (_, index) => ({
          value: String(valueVector?.get(index)),
          count: scalarNumber(countVector?.get(index), 'value count'),
        })),
      })
    }
  } catch (cause) {
    outcome = err({ kind: 'column-profile-failed', detail: detailOf(cause) })
  }

  const cleanupFailures: string[] = []
  if (connection) {
    try {
      await connection.close()
    } catch (cause) {
      cleanupFailures.push(`connection: ${detailOf(cause)}`)
    }
  }
  try {
    await running.db.dropFile(registeredPath)
  } catch (cause) {
    cleanupFailures.push(`source handle: ${detailOf(cause)}`)
  }
  return cleanupFailures.length > 0
    ? err({ kind: 'column-profile-failed', detail: cleanupFailures.join('; ') })
    : outcome
}

/** Register the source, open a connection, run `work`, and release both; the relation expression is passed in. */
async function withSource<Value, Problem>(
  source: SelectedSource,
  profile: DatasetProfile,
  failure: (detail: string) => Problem,
  changed: (expected: SourceFingerprint, actual: SourceFingerprint) => Problem,
  work: (connection: duckdb.AsyncDuckDBConnection, relation: string) => Promise<Result<Value, Problem>>,
  textColumn?: string,
): Promise<Result<Value, Problem>> {
  const fingerprint = await fingerprintFile(source.file)
  if (!fingerprint.ok) return err(failure(fingerprint.error.detail))
  if (fingerprint.value !== profile.source.fingerprint) return err(changed(profile.source.fingerprint, fingerprint.value))
  let running: DuckDbEngine
  try {
    running = await engine()
  } catch (cause) {
    return err(failure(detailOf(cause)))
  }
  const registeredPath = `hirmos-${crypto.randomUUID()}.${source.format}`
  try {
    await running.db.registerFileHandle(registeredPath, source.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
  } catch (cause) {
    return err(failure(detailOf(cause)))
  }
  const relation = fileRelation(profile.source, registeredPath, textColumn)
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<Value, Problem>
  try {
    connection = await running.db.connect()
    // TimeZone is connection-local: the startup connection's setting does not carry over.
    // Interpret unzoned calendar input in UTC while preserving explicitly supplied zones.
    await connection.query("SET TimeZone = 'UTC'")
    outcome = await work(connection, relation)
  } catch (cause) {
    outcome = err(failure(detailOf(cause)))
  }
  try {
    await connection?.close()
    await running.db.dropFile(registeredPath)
  } catch (cause) {
    return err(failure(`cleanup: ${detailOf(cause)}`))
  }
  return outcome
}

const MAX_SUMMARY_HISTOGRAMS = 64
const EXACT_DISTINCT_ROWS = 200_000
const CATEGORY_LIMIT = 30
const SUMMARY_BINS = 16

/** One `SUMMARIZE` pass for distinct counts and extremes, then a bucket query per numeric column and a value count per low-cardinality column. */
export async function summarizeColumns(source: SelectedSource, profile: DatasetProfile): Promise<Result<DatasetSummary, DatasetSummaryProblem>> {
  return withSource<DatasetSummary, DatasetSummaryProblem>(
    source,
    profile,
    (detail) => ({ kind: 'summary-failed', detail }),
    (expected, actual) => ({ kind: 'source-changed', expected, actual }),
    async (connection, relation) => {
      const summarized = await connection.query(`SUMMARIZE SELECT * FROM ${relation}`)
      const names = summarized.getChild('column_name')
      const mins = summarized.getChild('min')
      const maxes = summarized.getChild('max')
      const uniques = summarized.getChild('approx_unique')
      const counts = summarized.getChild('count')
      const byName = new Map<string, number>()
      for (let index = 0; index < summarized.numRows; index += 1) byName.set(String(names?.get(index)), index)
      // `approx_unique` can exceed the row count on small files; below the threshold the exact count is cheap.
      const exact = profile.rowCount <= EXACT_DISTINCT_ROWS
        ? await connection.query(`SELECT ${profile.columns.map((column, index) => `count(DISTINCT ${sqlIdentifier(column.name)})::UBIGINT AS ${sqlIdentifier(`d_${index}`)}`).join(', ')} FROM ${relation}`)
        : null
      const columns: ColumnSummary[] = []
      let histograms = 0
      for (const column of profile.columns) {
        const row = byName.get(column.name)
        if (row === undefined) return err({ kind: 'summary-failed', detail: `SUMMARIZE returned no row for ${column.name}.` })
        const present = scalarNumber(counts?.get(row) ?? 0, 'count')
        const columnIndex = profile.columns.indexOf(column)
        const distinctCount = exact !== null
          ? scalarNumber(exact.getChild(`d_${columnIndex}`)?.get(0) ?? 0, 'distinct count')
          : Math.min(present, scalarNumber(uniques?.get(row) ?? 0, 'distinct count'))
        const minRaw = mins?.get(row)
        const maxRaw = maxes?.get(row)
        const min = minRaw === null || minRaw === undefined ? null : String(minRaw)
        const max = maxRaw === null || maxRaw === undefined ? null : String(maxRaw)
        const name = sqlIdentifier(column.name)
        let histogram: ColumnSummary['histogram'] = null
        let categories: ColumnSummary['categories'] = null
        if (isNumericDuckDbType(column.duckdbType) && present > 0 && histograms < MAX_SUMMARY_HISTOGRAMS) {
          histograms += 1
          const minimum = Number(min)
          const maximum = Number(max)
          if (Number.isFinite(minimum) && Number.isFinite(maximum)) {
            const bins = minimum === maximum ? 1 : Math.max(1, Math.min(distinctCount, SUMMARY_BINS, Math.max(4, Math.ceil(Math.sqrt(present)))))
            const width = (maximum - minimum) / bins
            const binCounts = new Array<number>(bins).fill(0)
            if (bins === 1) {
              binCounts[0] = present
            } else {
              const buckets = await connection.query(`
                SELECT least(greatest(floor((CAST(${name} AS DOUBLE) - ${minimum}) / ${width}), 0), ${bins - 1})::INTEGER AS bucket, count(*) AS n
                FROM ${relation}
                WHERE ${name} IS NOT NULL
                GROUP BY bucket
              `)
              const bucketVector = buckets.getChild('bucket')
              const countVector = buckets.getChild('n')
              for (let index = 0; index < buckets.numRows; index += 1) {
                binCounts[scalarNumber(bucketVector?.get(index), 'bucket')] = scalarNumber(countVector?.get(index), 'bucket count')
              }
            }
            histogram = {
              edges: Array.from({ length: bins + 1 }, (_, index) => (index === bins ? maximum : minimum + width * index)),
              counts: binCounts,
            }
          }
        } else if (!isNumericDuckDbType(column.duckdbType) && present > 0 && distinctCount <= CATEGORY_LIMIT) {
          const top = await connection.query(`
            SELECT CAST(${name} AS VARCHAR) AS value, count(*) AS n
            FROM ${relation}
            WHERE ${name} IS NOT NULL
            GROUP BY value
            ORDER BY n DESC, value ASC
            LIMIT ${CATEGORY_LIMIT}
          `)
          const valueVector = top.getChild('value')
          const countVector = top.getChild('n')
          categories = Array.from({ length: top.numRows }, (_, index) => ({
            value: String(valueVector?.get(index)),
            count: scalarNumber(countVector?.get(index), 'value count'),
          }))
        }
        columns.push({ column: column.id, distinctCount, min, max, histogram, categories })
      }
      return ok({ kind: 'dataset-summary', sourceFingerprint: profile.source.fingerprint, columns })
    },
  )
}

const sqlNumber = (value: number): string => (Number.isFinite(value) ? String(value) : 'NULL')

const filterClause = (columnName: string, filter: PreviewFilter): string => {
  const name = sqlIdentifier(columnName)
  switch (filter.kind) {
    case 'range': {
      const parts: string[] = []
      if (filter.min !== null) parts.push(`CAST(${name} AS DOUBLE) >= ${sqlNumber(filter.min)}`)
      if (filter.max !== null) parts.push(`CAST(${name} AS DOUBLE) <= ${sqlNumber(filter.max)}`)
      return parts.length === 0 ? 'TRUE' : `(${parts.join(' AND ')})`
    }
    case 'contains': return `CAST(${name} AS VARCHAR) ILIKE ${sqlString(`%${filter.text.replaceAll('%', '\\%').replaceAll('_', '\\_')}%`)} ESCAPE '\\'`
    case 'one-of': return filter.values.length === 0 ? 'TRUE' : `CAST(${name} AS VARCHAR) IN (${filter.values.map(sqlString).join(', ')})`
    case 'missing': return filter.missing ? `${name} IS NULL` : `${name} IS NOT NULL`
    default: return assertNever(filter)
  }
}

/** A sorted, filtered window onto the whole file; the row index is the file position before either. */
export async function previewWindow(source: SelectedSource, profile: DatasetProfile, query: PreviewQuery): Promise<Result<PreviewWindow, PreviewWindowProblem>> {
  return withSource<PreviewWindow, PreviewWindowProblem>(
    source,
    profile,
    (detail) => ({ kind: 'preview-failed', detail }),
    (expected, actual) => ({ kind: 'source-changed', expected, actual }),
    async (connection, relation) => {
      const columnName = (id: string): string | null => profile.columns.find((column) => column.id === id)?.name ?? null
      const clauses: string[] = []
      for (const filter of query.filters) {
        const name = columnName(filter.column)
        if (name === null) return err({ kind: 'preview-failed', detail: `Filter names an unknown column ${filter.column}.` })
        clauses.push(filterClause(name, filter))
      }
      const search = query.search.trim()
      if (search.length > 0) {
        const pattern = sqlString(`%${search.replaceAll('%', '\\%').replaceAll('_', '\\_')}%`)
        clauses.push(`(${profile.columns.map((column) => `CAST(${sqlIdentifier(column.name)} AS VARCHAR) ILIKE ${pattern} ESCAPE '\\'`).join(' OR ')})`)
      }
      const where = clauses.length === 0 ? '' : `WHERE ${clauses.join(' AND ')}`
      let order = 'ORDER BY __row'
      if (query.sort !== null) {
        const name = columnName(query.sort.column)
        if (name === null) return err({ kind: 'preview-failed', detail: `Sort names an unknown column ${query.sort.column}.` })
        order = `ORDER BY ${sqlIdentifier(name)} ${query.sort.direction === 'asc' ? 'ASC' : 'DESC'} NULLS LAST, __row`
      }
      const projection = profile.columns.map((column) => sqlIdentifier(column.name)).join(', ')
      const base = `WITH source AS (SELECT row_number() OVER () AS __row, ${projection} FROM ${relation})`
      const counted = await connection.query(`${base} SELECT count(*)::UBIGINT AS total FROM source ${where}`)
      const total = safeCount(counted.getChild('total')?.get(0))
      if (!total.ok) return err({ kind: 'preview-failed', detail: total.error.kind === 'unsafe-row-count' ? 'The row count is outside the safe range.' : total.error.detail })
      const table = await connection.query(`${base} SELECT * FROM source ${where} ${order} LIMIT ${query.limit} OFFSET ${query.offset}`)
      const indexVector = table.getChild('__row')
      const rows: PreviewRow[] = []
      for (let rowIndex = 0; rowIndex < table.numRows; rowIndex += 1) {
        rows.push({
          index: scalarNumber(indexVector?.get(rowIndex), 'row index'),
          cells: profile.columns.map((column, columnIndex) => previewCell(table.getChildAt(columnIndex + 1)?.get(rowIndex), column.duckdbType)),
        })
      }
      return ok({ kind: 'preview-window', sourceFingerprint: profile.source.fingerprint, offset: query.offset, total: total.value, rows })
    },
  )
}
