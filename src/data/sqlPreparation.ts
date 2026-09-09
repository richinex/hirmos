import * as duckdb from '@duckdb/duckdb-wasm'
import { fingerprintFile } from './fingerprint'
import { isolatedDuckDbEngine, type DuckDbEngine } from './duckdb'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { selectSource, type SelectedSource } from '@/domain/workflow'
import {
  PREPARED_VIEW,
  sqlViewName,
  sqlDerivedRecipe,
  uniqueSqlInputAlias,
  type SqlPreparationInput,
  type SqlViewName,
} from '@/domain/sqlPreparation'

export type SqlPreparationProblem =
  | { readonly kind: 'no-input-files' }
  | { readonly kind: 'unsupported-input'; readonly fileName: string; readonly detail: string }
  | { readonly kind: 'fingerprint-failed'; readonly fileName: string; readonly detail: string }
  | { readonly kind: 'engine-unavailable'; readonly detail: string }
  | { readonly kind: 'input-registration-failed'; readonly fileName: string; readonly detail: string }
  | { readonly kind: 'view-list-failed'; readonly detail: string }
  | { readonly kind: 'prepared-view-missing'; readonly viewName: string }
  | { readonly kind: 'prepared-view-invalid'; readonly detail: string }
  | { readonly kind: 'prepared-view-empty' }
  | { readonly kind: 'materialization-failed'; readonly detail: string }
  | { readonly kind: 'cancellation-failed'; readonly detail: string }
  | { readonly kind: 'session-close-failed'; readonly detail: string }

export interface SqlPreparationSession {
  readonly shellDatabase: duckdb.AsyncDuckDB
  readonly database: duckdb.AsyncDuckDB
  readonly inputs: NonEmptyArray<SqlPreparationInput>
  readonly shellConnection: { current: number | null }
  readonly lifecycle: { current: 'open' | 'closed' }
}

export interface SqlPreparedOutput {
  readonly file: File
  readonly recipe: Extract<SelectedSource['recipe'], { readonly kind: 'sql-derived' }>
  readonly rowCount: number
  readonly columns: NonEmptyArray<{ readonly name: string; readonly type: string }>
}

const detailOf = (cause: unknown): string => cause instanceof Error ? cause.message : String(cause)
const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const sqlIdentifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const scalarCount = (value: unknown): Result<number, SqlPreparationProblem> => {
  if (typeof value === 'bigint' && value <= BigInt(Number.MAX_SAFE_INTEGER)) return ok(Number(value))
  if (typeof value === 'number' && Number.isSafeInteger(value) && value >= 0) return ok(value)
  return err({ kind: 'prepared-view-invalid', detail: `DuckDB returned an invalid row count: ${String(value)}` })
}

const relationFor = (input: SqlPreparationInput, path: string): string => input.format === 'parquet'
  ? `read_parquet(${sqlString(path)})`
  : `read_csv_auto(${sqlString(path)}, header = true, sample_size = 20480)`

async function registerInputs(
  engine: DuckDbEngine,
  inputs: NonEmptyArray<SqlPreparationInput>,
  namespace: string,
): Promise<Result<null, SqlPreparationProblem>> {
  const connection = await engine.db.connect()
  try {
    for (const input of inputs) {
      const path = `${namespace}-${input.fingerprint}.${input.format}`
      try {
        await engine.db.registerFileHandle(path, input.file, duckdb.DuckDBDataProtocol.BROWSER_FILEREADER, true)
        await connection.query(`CREATE OR REPLACE VIEW ${sqlIdentifier(input.alias)} AS SELECT * FROM ${relationFor(input, path)}`)
      } catch (cause) {
        return err({ kind: 'input-registration-failed', fileName: input.fileName, detail: detailOf(cause) })
      }
    }
    return ok(null)
  } finally {
    await connection.close()
  }
}

export async function prepareSqlInputs(files: readonly File[]): Promise<Result<NonEmptyArray<SqlPreparationInput>, SqlPreparationProblem>> {
  if (!isNonEmpty(files)) return err({ kind: 'no-input-files' })
  const prepared: SqlPreparationInput[] = []
  const occupied = new Set<string>()
  for (const file of files) {
    const source = selectSource(file)
    if (!source.ok) {
      return err({ kind: 'unsupported-input', fileName: file.name, detail: source.error.kind })
    }
    const fingerprint = await fingerprintFile(file)
    if (!fingerprint.ok) return err({ kind: 'fingerprint-failed', fileName: file.name, detail: fingerprint.error.detail })
    const alias = uniqueSqlInputAlias(file.name, occupied)
    occupied.add(alias)
    prepared.push({
      alias,
      file,
      fileName: source.value.name,
      bytes: source.value.bytes,
      format: source.value.format,
      fingerprint: fingerprint.value,
    })
  }
  return isNonEmpty(prepared) ? ok(prepared) : err({ kind: 'no-input-files' })
}

/**
 * The shell creates its own connection through `connectInternal`. The proxy records that public
 * connection handle so Hirmos can route a visible cancel action to DuckDB without implementing a
 * second query runner.
 */
const shellDatabase = (
  database: duckdb.AsyncDuckDB,
  connection: { current: number | null },
): duckdb.AsyncDuckDB => new Proxy(database, {
  get(target, property) {
    if (property === 'connectInternal') {
      return async () => {
        const id = await target.connectInternal()
        connection.current = id
        return id
      }
    }
    const value: unknown = Reflect.get(target, property, target)
    return typeof value === 'function' ? value.bind(target) : value
  },
})

export async function openSqlPreparation(
  inputs: NonEmptyArray<SqlPreparationInput>,
): Promise<Result<SqlPreparationSession, SqlPreparationProblem>> {
  let engine: DuckDbEngine
  try { engine = await isolatedDuckDbEngine() } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
  const registered = await registerInputs(engine, inputs, `sql-shell-${crypto.randomUUID()}`)
  if (!registered.ok) return registered
  const shellConnection = { current: null as number | null }
  return ok({
    shellDatabase: shellDatabase(engine.db, shellConnection),
    database: engine.db,
    inputs,
    shellConnection,
    lifecycle: { current: 'open' },
  })
}

export async function closeSqlPreparation(
  session: SqlPreparationSession,
): Promise<Result<null, SqlPreparationProblem>> {
  if (session.lifecycle.current === 'closed') return ok(null)
  session.lifecycle.current = 'closed'
  try {
    await session.database.terminate()
    return ok(null)
  } catch (cause) {
    return err({ kind: 'session-close-failed', detail: detailOf(cause) })
  }
}

export async function cancelSqlPreparationQuery(
  session: SqlPreparationSession,
): Promise<Result<boolean, SqlPreparationProblem>> {
  if (session.shellConnection.current === null) return ok(false)
  try {
    return ok(await session.database.cancelPendingQuery(session.shellConnection.current))
  } catch (cause) {
    return err({ kind: 'cancellation-failed', detail: detailOf(cause) })
  }
}

interface SqlViewDefinition {
  readonly name: SqlViewName
  readonly statement: string
}

async function viewDefinitions(session: SqlPreparationSession): Promise<Result<readonly SqlViewDefinition[], SqlPreparationProblem>> {
  const connection = await session.database.connect()
  try {
    const table = await connection.query(`
      SELECT view_name, sql
      FROM duckdb_views()
      WHERE schema_name = 'main' AND NOT internal
      ORDER BY view_oid
    `)
    const inputAliases = new Set(session.inputs.map((input) => input.alias as string))
    const definitions: SqlViewDefinition[] = []
    for (let row = 0; row < table.numRows; row += 1) {
      const rawName = table.getChild('view_name')?.get(row)
      const statement = table.getChild('sql')?.get(row)
      if (typeof rawName !== 'string' || inputAliases.has(rawName)) continue
      const name = sqlViewName(rawName)
      if (!name.ok || typeof statement !== 'string' || statement.trim().length === 0) {
        return err({ kind: 'prepared-view-invalid', detail: 'DuckDB returned an invalid view definition.' })
      }
      definitions.push({ name: name.value, statement })
    }
    return ok(definitions)
  } catch (cause) {
    return err({ kind: 'view-list-failed', detail: detailOf(cause) })
  } finally {
    await connection.close()
  }
}

export async function listSqlPreparationViews(
  session: SqlPreparationSession,
): Promise<Result<readonly SqlViewName[], SqlPreparationProblem>> {
  const definitions = await viewDefinitions(session)
  return definitions.ok ? ok(definitions.value.map((definition) => definition.name)) : definitions
}

async function verifyAndMaterialize(
  definitions: readonly SqlViewDefinition[],
  outputView: SqlViewName,
  inputs: NonEmptyArray<SqlPreparationInput>,
): Promise<Result<Omit<SqlPreparedOutput, 'recipe'>, SqlPreparationProblem>> {
  let engine: DuckDbEngine
  try { engine = await isolatedDuckDbEngine() } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
  const outputPath = `sql-prepared-${crypto.randomUUID()}.parquet`
  try {
    const registered = await registerInputs(engine, inputs, `sql-verify-${crypto.randomUUID()}`)
    if (!registered.ok) return registered
    const connection = await engine.db.connect()
    try {
      let pending = [...definitions]
      while (pending.length > 0) {
        const failed: SqlViewDefinition[] = []
        for (const definition of pending) {
          try {
            await connection.query(definition.statement)
          } catch {
            failed.push(definition)
          }
        }
        if (failed.length === pending.length) break
        pending = failed
      }
      const description = await connection.query(`DESCRIBE SELECT * FROM ${sqlIdentifier(outputView)}`)
      const columns = Array.from({ length: description.numRows }, (_, index) => ({
        name: String(description.getChild('column_name')?.get(index) ?? ''),
        type: String(description.getChild('column_type')?.get(index) ?? ''),
      }))
      if (!isNonEmpty(columns) || columns.some((column) => column.name.length === 0 || column.type.length === 0)) {
        return err({ kind: 'prepared-view-invalid', detail: 'The prepared view has no readable columns.' })
      }
      const names = new Set(columns.map((column) => column.name))
      if (names.size !== columns.length) return err({ kind: 'prepared-view-invalid', detail: 'The prepared view has duplicate column names.' })
      const countTable = await connection.query(`SELECT count(*)::UBIGINT AS rows FROM ${sqlIdentifier(outputView)}`)
      const count = scalarCount(countTable.getChild('rows')?.get(0))
      if (!count.ok) return count
      if (count.value === 0) return err({ kind: 'prepared-view-empty' })
      await connection.query(`COPY (SELECT * FROM ${sqlIdentifier(outputView)}) TO ${sqlString(outputPath)} (FORMAT PARQUET)`)
      const bytes = await engine.db.copyFileToBuffer(outputPath)
      const file = new File([Uint8Array.from(bytes)], `${outputView}.parquet`, { type: 'application/vnd.apache.parquet', lastModified: Date.now() })
      return ok({ file, rowCount: count.value, columns })
    } catch (cause) {
      return err({ kind: 'materialization-failed', detail: detailOf(cause) })
    } finally {
      await connection.close()
    }
  } finally {
    await engine.db.terminate()
  }
}

export async function materializePreparedView(
  session: SqlPreparationSession,
  outputView: SqlViewName,
): Promise<Result<SqlPreparedOutput, SqlPreparationProblem>> {
  const definitions = await viewDefinitions(session)
  if (!definitions.ok) return definitions
  if (!definitions.value.some((definition) => definition.name === outputView)) {
    return err({ kind: 'prepared-view-missing', viewName: outputView })
  }
  const statement = definitions.value.map((definition) => definition.statement.trim()).join('\n')
  const recipe = sqlDerivedRecipe(statement, outputView, session.inputs)
  if (!recipe.ok) return err({ kind: 'prepared-view-invalid', detail: 'The prepared view has an incomplete source recipe.' })
  const materialized = await verifyAndMaterialize(definitions.value, outputView, session.inputs)
  return materialized.ok
    ? ok({ ...materialized.value, recipe: recipe.value })
    : materialized
}

export function describeSqlPreparationProblem(problem: SqlPreparationProblem): string {
  switch (problem.kind) {
    case 'no-input-files': return 'Choose at least one input file.'
    case 'unsupported-input': return `${problem.fileName} cannot be used: ${problem.detail}.`
    case 'fingerprint-failed': return `${problem.fileName} could not be fingerprinted: ${problem.detail}`
    case 'engine-unavailable': return `DuckDB could not start: ${problem.detail}`
    case 'input-registration-failed': return `${problem.fileName} could not be registered: ${problem.detail}`
    case 'view-list-failed': return `The SQL views could not be listed: ${problem.detail}`
    case 'prepared-view-missing': return `The selected view ${problem.viewName} no longer exists. Refresh the view list and choose another.`
    case 'prepared-view-invalid': return `The prepared view was refused: ${problem.detail}`
    case 'prepared-view-empty': return 'The prepared view contains no rows.'
    case 'materialization-failed': return `The prepared view could not be materialized: ${problem.detail}`
    case 'cancellation-failed': return `The query could not be cancelled: ${problem.detail}`
    case 'session-close-failed': return `The SQL workspace could not release its database: ${problem.detail}`
    default: { const exhaustive: never = problem; return exhaustive }
  }
}
