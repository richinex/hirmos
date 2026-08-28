import * as duckdb from '@duckdb/duckdb-wasm'
import duckdbEhWasm from '@duckdb/duckdb-wasm/dist/duckdb-eh.wasm?url'
import duckdbEhWorker from '@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url'
import duckdbMvpWasm from '@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url'
import duckdbMvpWorker from '@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url'
import { fingerprintFile } from './fingerprint'
import { err, isNonEmpty, mapNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import {
  columnId,
  datasetProfileId,
  isNumericDuckDbType,
  type DatasetProfile,
  type DatasetProfileProblem,
  type NullableNumericMatrix,
  type NumericMaterializationProblem,
  type PhysicalColumnProfile,
  type PreviewCell,
  type SourceFingerprint,
} from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'

const DUCKDB_PACKAGE_VERSION = '1.30.0'
const DUCKDB_ENGINE_VERSION = 'v1.3.2'
const PREVIEW_ROWS = 12

const LOCAL_BUNDLES: duckdb.DuckDBBundles = {
  mvp: { mainModule: duckdbMvpWasm, mainWorker: duckdbMvpWorker },
  eh: { mainModule: duckdbEhWasm, mainWorker: duckdbEhWorker },
}

interface Engine {
  readonly db: duckdb.AsyncDuckDB
  readonly version: string
}

let enginePromise: Promise<Engine> | null = null

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

const startEngine = async (): Promise<Engine> => {
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

const engine = (): Promise<Engine> => {
  enginePromise ??= startEngine().catch((cause) => {
    enginePromise = null
    throw cause
  })
  return enginePromise
}

const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const sqlIdentifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const previewCell = (value: unknown): PreviewCell => {
  if (value === null || value === undefined) return { kind: 'null' }
  if (typeof value === 'number') return { kind: 'number', value }
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
      previewCell(previewTable.getChildAt(columnIndex)?.get(rowIndex)),
    )
    if (!isNonEmpty(row)) return err({ kind: 'no-columns' })
    builtPreview.push(row)
  }
  if (!isNonEmpty(builtPreview)) {
    return err({ kind: 'parse-failed', detail: 'DuckDB counted data rows but returned an empty bounded preview.' })
  }

  return ok({
    id: datasetProfileId(fingerprint),
    source: {
      fingerprint,
      fileName: source.name,
      bytes: source.bytes,
      format: source.format,
      persistence: { kind: 'ephemeral' },
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

  let running: Engine
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

  const relation = source.format === 'parquet'
    ? `read_parquet(${sqlString(registeredPath)})`
    : `read_csv_auto(${sqlString(registeredPath)}, header = true, sample_size = 20480)`
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

export async function materializeNumericColumns(
  source: SelectedSource,
  profile: DatasetProfile,
  requestedColumnIds: NonEmptyArray<string>,
): Promise<Result<NullableNumericMatrix, NumericMaterializationProblem>> {
  const fingerprint = await fingerprintFile(source.file)
  if (!fingerprint.ok) return err({ kind: 'materialization-failed', detail: fingerprint.error.detail })
  if (fingerprint.value !== profile.source.fingerprint) {
    return err({
      kind: 'source-changed',
      expected: profile.source.fingerprint,
      actual: fingerprint.value,
    })
  }

  const selected: PhysicalColumnProfile[] = []
  const seen = new Set<string>()
  for (const id of requestedColumnIds) {
    if (seen.has(id)) return err({ kind: 'duplicate-column', id })
    seen.add(id)
    const column = profile.columns.find((candidate) => candidate.id === id)
    if (!column) return err({ kind: 'column-not-found', id })
    if (!isNumericDuckDbType(column.duckdbType)) {
      return err({ kind: 'non-numeric-column', name: column.name, duckdbType: column.duckdbType })
    }
    selected.push(column)
  }
  if (!isNonEmpty(selected)) return err({ kind: 'materialization-failed', detail: 'No numeric columns were selected.' })

  let running: Engine
  try {
    running = await engine()
  } catch (cause) {
    return err({ kind: 'materialization-failed', detail: detailOf(cause) })
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
    return err({ kind: 'materialization-failed', detail: detailOf(cause) })
  }

  const relation = source.format === 'parquet'
    ? `read_parquet(${sqlString(registeredPath)})`
    : `read_csv_auto(${sqlString(registeredPath)}, header = true, sample_size = 20480)`
  const projections = selected.map((column, index) =>
    `CAST(${sqlIdentifier(column.name)} AS DOUBLE) AS ${sqlIdentifier(`hirmos_numeric_${index}`)}`,
  )
  let connection: duckdb.AsyncDuckDBConnection | null = null
  let outcome: Result<NullableNumericMatrix, NumericMaterializationProblem>
  try {
    connection = await running.db.connect()
    const table = await connection.query(`SELECT ${projections.join(', ')} FROM ${relation}`)
    if (table.numRows !== profile.rowCount) {
      outcome = err({ kind: 'materialization-failed', detail: 'The materialized row count changed since profiling.' })
    } else {
      const cellCount = profile.rowCount * selected.length
      if (!Number.isSafeInteger(cellCount)) {
        outcome = err({ kind: 'materialization-failed', detail: 'The selected matrix is too large for browser-safe indexing.' })
      } else {
        const values = new Float64Array(cellCount)
        values.fill(Number.NaN)
        const validity = new Uint8Array(Math.ceil(cellCount / 8))
        let missingCells = 0
        let problem: NumericMaterializationProblem | null = null
        for (const [columnIndex, column] of selected.entries()) {
          const vector = table.getChild(`hirmos_numeric_${columnIndex}`)
          if (!vector) {
            problem = { kind: 'materialization-failed', detail: `DuckDB omitted numeric column ${column.name}.` }
            break
          }
          for (let row = 0; row < profile.rowCount; row += 1) {
            const value = vector.get(row)
            const target = columnIndex * profile.rowCount + row
            if (value === null || value === undefined) {
              missingCells += 1
            } else if (typeof value !== 'number' || !Number.isFinite(value)) {
              problem = { kind: 'non-finite-value', name: column.name, row }
              break
            } else {
              values[target] = value
              setValid(validity, target)
            }
          }
          if (problem) break
        }
        outcome = problem
          ? err(problem)
          : ok({
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
    }
  } catch (cause) {
    outcome = err({ kind: 'materialization-failed', detail: detailOf(cause) })
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
    ? err({ kind: 'materialization-failed', detail: cleanupFailures.join('; ') })
    : outcome
}
