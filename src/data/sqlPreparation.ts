import * as duckdb from '@duckdb/duckdb-wasm'
import { RecordBatchStreamWriter } from 'apache-arrow'
import { fingerprintFile } from './fingerprint'
import { DETECT, fileRelation } from './fileRelation'
import { declareColumn, fileReading, type DeclaredType } from '@/domain/fileReading'
import {
  detectLayout,
  isolatedDuckDbEngine,
  previewCell,
  summarizeRelation,
  type DuckDbEngine,
} from './duckdb'
import { rememberInputFiles } from './inputFiles'
import { exportDirectory, type SqlInputDescriptor } from '@/domain/sourceInputs'
import { err, isNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { selectSource, type SelectedSource } from '@/domain/workflow'
import {
  sqlViewName,
  sqlDerivedRecipe,
  uniqueSqlInputAlias,
  type SqlPreparationInput,
  type SqlViewName,
} from '@/domain/sqlPreparation'
import {
  sqlCatalog,
  type SqlCatalogRow,
  type SqlRelation,
  type SqlRelationName,
  type SqlRelationRows,
  type SqlRelationSummary,
} from '@/domain/sqlCatalog'

export type SqlPreparationProblem =
  | { readonly kind: 'no-input-files' }
  | { readonly kind: 'unsupported-input'; readonly fileName: string; readonly detail: string }
  | { readonly kind: 'fingerprint-failed'; readonly fileName: string; readonly detail: string }
  | { readonly kind: 'engine-unavailable'; readonly detail: string }
  | {
      readonly kind: 'input-registration-failed'
      readonly fileName: string
      readonly detail: string
    }
  | { readonly kind: 'view-list-failed'; readonly detail: string }
  | { readonly kind: 'catalog-failed'; readonly detail: string }
  | {
      readonly kind: 'relation-read-failed'
      readonly relation: SqlRelationName
      readonly detail: string
    }
  | {
      readonly kind: 'relation-summary-failed'
      readonly relation: SqlRelationName
      readonly detail: string
    }
  | { readonly kind: 'prepared-view-missing'; readonly viewName: string }
  | { readonly kind: 'prepared-view-invalid'; readonly detail: string }
  | { readonly kind: 'prepared-view-empty' }
  | { readonly kind: 'materialization-failed'; readonly detail: string }
  | { readonly kind: 'cancellation-failed'; readonly detail: string }
  | { readonly kind: 'session-close-failed'; readonly detail: string }
  /** A replay was given files that do not include every input the recipe was built from, by fingerprint. */
  | { readonly kind: 'replay-inputs-missing'; readonly fileNames: readonly string[] }

export interface SqlPreparationSession {
  readonly shellDatabase: duckdb.AsyncDuckDB
  readonly database: duckdb.AsyncDuckDB
  readonly inputs: readonly SqlPreparationInput[]
  readonly shellConnection: { current: number | null }
  readonly lifecycle: { current: 'open' | 'closed' }
}

export interface SqlPreparedOutput {
  readonly file: File
  readonly recipe: Extract<SelectedSource['recipe'], { readonly kind: 'sql-derived' }>
  readonly rowCount: number
  readonly columns: NonEmptyArray<{ readonly name: string; readonly type: string }>
}

const detailOf = (cause: unknown): string =>
  cause instanceof Error ? cause.message : String(cause)
const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const sqlIdentifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const scalarCount = (value: unknown): Result<number, SqlPreparationProblem> => {
  if (typeof value === 'bigint' && value <= BigInt(Number.MAX_SAFE_INTEGER))
    return ok(Number(value))
  if (typeof value === 'number' && Number.isSafeInteger(value) && value >= 0) return ok(value)
  return err({
    kind: 'prepared-view-invalid',
    detail: `DuckDB returned an invalid row count: ${String(value)}`,
  })
}

export async function registerInputs(
  engine: DuckDbEngine,
  inputs: readonly SqlPreparationInput[],
  namespace: string,
): Promise<Result<null, SqlPreparationProblem>> {
  const connection = await engine.db.connect()
  try {
    for (const input of inputs) {
      const path =
        input.format === 'duckdb-export-file'
          ? input.path
          : `${namespace}-${input.fingerprint}.${input.format}`
      try {
        await engine.db.registerFileHandle(
          path,
          input.file,
          duckdb.DuckDBDataProtocol.BROWSER_FILEREADER,
          true,
        )
        if (input.format !== 'duckdb-export-file') {
          const layout =
            input.format === 'parquet' ? DETECT : await detectLayout(connection, input, path)
          await connection.query(
            `CREATE OR REPLACE VIEW ${sqlIdentifier(input.alias)} AS SELECT * FROM ${fileRelation(input, path, undefined, layout)}`,
          )
        }
      } catch (cause) {
        return err({
          kind: 'input-registration-failed',
          fileName: input.fileName,
          detail: detailOf(cause),
        })
      }
    }
    const exported = inputs.filter((input) => input.format === 'duckdb-export-file')
    if (exported.length > 0) {
      const directory = exportDirectory(exported.map((input) => input.path))
      if (!directory.ok)
        return err({ kind: 'prepared-view-invalid', detail: directory.error.detail })
      try {
        await connection.query(`IMPORT DATABASE ${sqlString(directory.value)}`)
      } catch (cause) {
        return err({
          kind: 'input-registration-failed',
          fileName: directory.value,
          detail: detailOf(cause),
        })
      }
    }
    return ok(null)
  } finally {
    await connection.close()
  }
}

/** The input read again with one column's declaration changed: its view is created again under the same alias. */
export async function redeclareInput(
  engine: DuckDbEngine,
  namespace: string,
  input: SqlPreparationInput,
  column: string,
  type: DeclaredType | null,
): Promise<Result<SqlPreparationInput, SqlPreparationProblem>> {
  if (input.format === 'duckdb-export-file' || input.format === 'parquet') {
    return err({
      kind: 'input-registration-failed',
      fileName: input.fileName,
      detail: 'This file stores its column types, so none can be declared.',
    })
  }
  const updated: SqlPreparationInput = {
    ...input,
    declared: declareColumn(input.declared, column, type),
  }
  const registered = await registerInputs(engine, [updated], namespace)
  return registered.ok ? ok(updated) : registered
}

export async function prepareDatabaseExport(
  files: readonly File[],
): Promise<Result<readonly SqlPreparationInput[], SqlPreparationProblem>> {
  const paths = files.map((file) => file.webkitRelativePath)
  const directory = exportDirectory(paths)
  if (!directory.ok) return err({ kind: 'prepared-view-invalid', detail: directory.error.detail })
  const inputs: SqlPreparationInput[] = []
  const occupied = new Set<string>()
  for (const file of files) {
    const fingerprint = await fingerprintFile(file)
    if (!fingerprint.ok)
      return err({
        kind: 'fingerprint-failed',
        fileName: file.name,
        detail: fingerprint.error.detail,
      })
    const alias = uniqueSqlInputAlias(`export_${file.webkitRelativePath}`, occupied)
    occupied.add(alias)
    inputs.push({
      alias,
      file,
      fileName: file.name,
      bytes: file.size,
      fingerprint: fingerprint.value,
      format: 'duckdb-export-file',
      path: file.webkitRelativePath,
    })
  }
  rememberInputFiles(inputs)
  return ok(inputs)
}

/** Match original bytes first, then restore their recorded role and path. */
export async function recoverSqlInputs(
  descriptors: readonly SqlInputDescriptor[],
  files: readonly File[],
): Promise<Result<readonly SqlPreparationInput[], SqlPreparationProblem>> {
  const offered = new Map<string, File>()
  for (const file of files) {
    const fingerprint = await fingerprintFile(file)
    if (!fingerprint.ok)
      return err({
        kind: 'fingerprint-failed',
        fileName: file.name,
        detail: fingerprint.error.detail,
      })
    offered.set(fingerprint.value, file)
  }
  const matched: SqlPreparationInput[] = []
  const missing: string[] = []
  for (const descriptor of descriptors) {
    const file = offered.get(descriptor.fingerprint)
    if (file === undefined) missing.push(descriptor.fileName)
    else matched.push({ ...descriptor, file })
  }
  if (missing.length > 0) return err({ kind: 'replay-inputs-missing', fileNames: missing })
  rememberInputFiles(matched)
  return ok(matched)
}

/** Each file gets an alias no other file in the batch, nor any in `taken`, already uses. */
export async function prepareSqlInputs(
  files: readonly File[],
  taken: ReadonlySet<string> = new Set(),
): Promise<Result<NonEmptyArray<SqlPreparationInput>, SqlPreparationProblem>> {
  if (!isNonEmpty(files)) return err({ kind: 'no-input-files' })
  const prepared: SqlPreparationInput[] = []
  const occupied = new Set<string>(taken)
  for (const file of files) {
    const source = selectSource(file)
    if (!source.ok) {
      return err({ kind: 'unsupported-input', fileName: file.name, detail: source.error.kind })
    }
    const fingerprint = await fingerprintFile(file)
    if (!fingerprint.ok)
      return err({
        kind: 'fingerprint-failed',
        fileName: file.name,
        detail: fingerprint.error.detail,
      })
    const alias = uniqueSqlInputAlias(file.name, occupied)
    occupied.add(alias)
    prepared.push({
      alias,
      file,
      fileName: source.value.name,
      bytes: source.value.bytes,
      fingerprint: fingerprint.value,
      ...fileReading(source.value),
    })
  }
  if (!isNonEmpty(prepared)) return err({ kind: 'no-input-files' })
  rememberInputFiles(prepared)
  return ok(prepared)
}

/**
 * Runs a recipe's recorded view definitions on the shell's database, so an editor reopened on the
 * recipe starts with the views it had made. Each line is one definition, in creation order.
 */
export async function replayDefinitionsInShell(
  session: SqlPreparationSession,
  statement: string,
): Promise<Result<null, SqlPreparationProblem>> {
  const connection = await session.shellDatabase.connect()
  try {
    for (const line of statement
      .split('\n')
      .map((text) => text.trim())
      .filter((text) => text.length > 0)) {
      await connection.query(line)
    }
    return ok(null)
  } catch (cause) {
    return err({ kind: 'prepared-view-invalid', detail: detailOf(cause) })
  } finally {
    await connection.close()
  }
}

/**
 * The shell creates its own connection through `connectInternal`. The proxy records that public
 * connection handle so Hirmos can route a visible cancel action to DuckDB without implementing a
 * second query runner.
 *
 * The shell keeps that connection until the next console opens and only then disconnects it. By
 * then a closed session has terminated its worker, and a call reaching it logs an error, so the
 * proxy answers the late disconnect itself.
 */
const shellDatabase = (
  database: duckdb.AsyncDuckDB,
  connection: { current: number | null },
  lifecycle: { current: 'open' | 'closed' },
  onQuery: (running: boolean) => void,
): duckdb.AsyncDuckDB =>
  new Proxy(database, {
    get(target, property) {
      if (property === 'disconnect' && lifecycle.current === 'closed') return async () => {}
      if (property === 'runQuery') {
        return async (id: number, sql: string) => {
          onQuery(true)
          try {
            const live = new duckdb.AsyncDuckDBConnection(target, id)
            // DuckDB executes batches and returns their final statement's result.
            const reader = await live.send(sql)
            // The official shell expects an Arrow IPC stream. Arrow preserves the schema
            // and batches while DuckDB's pending API leaves the query cancellable.
            const writer = await RecordBatchStreamWriter.writeAll(reader)
            return await writer.toUint8Array()
          } finally {
            onQuery(false)
          }
        }
      }
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
  inputs: readonly SqlPreparationInput[],
  onQuery: (running: boolean) => void = () => {},
): Promise<Result<SqlPreparationSession, SqlPreparationProblem>> {
  let engine: DuckDbEngine
  try {
    // This engine only feeds the console and its previews; the chosen view is materialised by
    // verifyAndMaterialize on a lossless engine. The lossy export gives the shell a decimal it
    // can print for HUGEINT, which every sum over an integer column returns.
    engine = await isolatedDuckDbEngine({ arrowLosslessConversion: false })
  } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
  const registered = await registerInputs(engine, inputs, `sql-shell-${crypto.randomUUID()}`)
  if (!registered.ok) {
    await engine.db.terminate()
    return registered
  }
  const shellConnection = { current: null as number | null }
  const lifecycle: SqlPreparationSession['lifecycle'] = { current: 'open' }
  return ok({
    shellDatabase: shellDatabase(engine.db, shellConnection, lifecycle, onQuery),
    database: engine.db,
    inputs,
    shellConnection,
    lifecycle,
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

export interface SqlViewDefinition {
  readonly name: SqlViewName
  readonly statement: string
}

async function viewDefinitions(
  session: SqlPreparationSession,
): Promise<Result<readonly SqlViewDefinition[], SqlPreparationProblem>> {
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
        return err({
          kind: 'prepared-view-invalid',
          detail: 'DuckDB returned an invalid view definition.',
        })
      }
      // Catalog SQL is generated by DuckDB. Replacement also replays views included in an export.
      definitions.push({
        name: name.value,
        statement: statement.replace(/^CREATE VIEW\b/i, 'CREATE OR REPLACE VIEW'),
      })
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

const catalogRows = (
  table: Awaited<ReturnType<duckdb.AsyncDuckDBConnection['query']>>,
): Result<readonly SqlCatalogRow[], SqlPreparationProblem> => {
  const rows: SqlCatalogRow[] = []
  for (let row = 0; row < table.numRows; row += 1) {
    const relation = table.getChild('relation')?.get(row)
    const view = table.getChild('is_view')?.get(row)
    const column = table.getChild('column_name')?.get(row)
    const type = table.getChild('column_type')?.get(row)
    if (
      typeof relation !== 'string' ||
      typeof view !== 'boolean' ||
      typeof column !== 'string' ||
      typeof type !== 'string'
    )
      return err({ kind: 'catalog-failed', detail: 'DuckDB returned an invalid column entry.' })
    rows.push({ relation, view, column, type })
  }
  return ok(rows)
}

/** Every table and view in the console's database, with its columns, read from DuckDB's catalog. */
export async function readSqlCatalog(
  session: SqlPreparationSession,
): Promise<Result<readonly SqlRelation[], SqlPreparationProblem>> {
  const connection = await session.database.connect()
  try {
    const table = await connection.query(`
      SELECT
        c.table_name AS relation,
        v.view_name IS NOT NULL AS is_view,
        c.column_name AS column_name,
        c.data_type AS column_type
      FROM duckdb_columns() c
      LEFT JOIN duckdb_views() v
        ON v.database_name = c.database_name
        AND v.schema_name = c.schema_name
        AND v.view_name = c.table_name
      WHERE c.database_name = current_database() AND c.schema_name = 'main' AND NOT c.internal
      ORDER BY c.table_oid, c.column_index
    `)
    const rows = catalogRows(table)
    if (!rows.ok) return rows
    const inputs = new Set(
      session.inputs
        .filter((input) => input.format !== 'duckdb-export-file')
        .map((input) => input.alias as string),
    )
    return ok(sqlCatalog(rows.value, inputs))
  } catch (cause) {
    return err({ kind: 'catalog-failed', detail: detailOf(cause) })
  } finally {
    await connection.close()
  }
}

const RELATION_PREVIEW_ROWS = 100

/** The relation's row count and its first rows, each cell read as the data preview reads it. */
export async function readSqlRelation(
  session: SqlPreparationSession,
  relation: SqlRelation,
): Promise<Result<SqlRelationRows, SqlPreparationProblem>> {
  const connection = await session.database.connect()
  const name = sqlIdentifier(relation.name)
  try {
    const counted = await connection.query(
      `SELECT count(*)::UBIGINT AS n, ${relation.columns
        .map(
          (column, index) =>
            `(count(*) - count(${sqlIdentifier(column.name)}))::UBIGINT AS ${sqlIdentifier(`null_${index}`)}`,
        )
        .join(', ')} FROM ${name}`,
    )
    const rowCount = scalarCount(counted.getChild('n')?.get(0))
    const nullCounts = relation.columns.map((column, index) => ({
      column: column.id,
      count: scalarCount(counted.getChild(`null_${index}`)?.get(0)),
    }))
    if (!rowCount.ok || nullCounts.some((entry) => !entry.count.ok))
      return err({
        kind: 'relation-read-failed',
        relation: relation.name,
        detail: 'DuckDB returned an invalid row count.',
      })
    const table = await connection.query(`SELECT * FROM ${name} LIMIT ${RELATION_PREVIEW_ROWS}`)
    const rows = Array.from({ length: table.numRows }, (_, row) =>
      relation.columns.map((column, index) =>
        previewCell(table.getChildAt(index)?.get(row), column.type),
      ),
    )
    return ok({
      rowCount: rowCount.value,
      nullCounts: new Map(
        nullCounts.flatMap((entry) => (entry.count.ok ? [[entry.column, entry.count.value]] : [])),
      ),
      rows,
    })
  } catch (cause) {
    return err({ kind: 'relation-read-failed', relation: relation.name, detail: detailOf(cause) })
  } finally {
    await connection.close()
  }
}

/** The relation's column summaries, as the data inspector's schema table shows them. */
export async function summarizeSqlRelation(
  session: SqlPreparationSession,
  relation: SqlRelation,
  rowCount: number,
): Promise<Result<SqlRelationSummary, SqlPreparationProblem>> {
  const connection = await session.database.connect()
  try {
    const summaries = await summarizeRelation(
      connection,
      sqlIdentifier(relation.name),
      relation.columns.map((column) => ({
        id: column.id,
        name: column.name,
        duckdbType: column.type,
      })),
      rowCount,
    )
    if (!summaries.ok)
      return err({
        kind: 'relation-summary-failed',
        relation: relation.name,
        detail:
          summaries.error.kind === 'summary-failed' ? summaries.error.detail : summaries.error.kind,
      })
    return ok(new Map(summaries.value.map((summary) => [summary.column, summary])))
  } catch (cause) {
    return err({
      kind: 'relation-summary-failed',
      relation: relation.name,
      detail: detailOf(cause),
    })
  } finally {
    await connection.close()
  }
}

/**
 * The output view's columns, row count and Parquet bytes, read on an open connection whose views are
 * in place. Shared by the SQL path, which rebuilds its views first, and the pipeline path, which has
 * run its steps on the connection already.
 */
export async function materializeView(
  engine: Pick<DuckDbEngine, 'db'>,
  connection: duckdb.AsyncDuckDBConnection,
  outputView: string,
  fileName: string,
): Promise<Result<Omit<SqlPreparedOutput, 'recipe'>, SqlPreparationProblem>> {
  const outputPath = `prepared-${crypto.randomUUID()}.parquet`
  try {
    const description = await connection.query(
      `DESCRIBE SELECT * FROM ${sqlIdentifier(outputView)}`,
    )
    const columns = Array.from({ length: description.numRows }, (_, index) => ({
      name: String(description.getChild('column_name')?.get(index) ?? ''),
      type: String(description.getChild('column_type')?.get(index) ?? ''),
    }))
    if (
      !isNonEmpty(columns) ||
      columns.some((column) => column.name.length === 0 || column.type.length === 0)
    ) {
      return err({
        kind: 'prepared-view-invalid',
        detail: 'The prepared view has no readable columns.',
      })
    }
    const names = new Set(columns.map((column) => column.name))
    if (names.size !== columns.length)
      return err({
        kind: 'prepared-view-invalid',
        detail: 'The prepared view has duplicate column names.',
      })
    const countTable = await connection.query(
      `SELECT count(*)::UBIGINT AS rows FROM ${sqlIdentifier(outputView)}`,
    )
    const count = scalarCount(countTable.getChild('rows')?.get(0))
    if (!count.ok) return count
    if (count.value === 0) return err({ kind: 'prepared-view-empty' })
    await connection.query(
      `COPY (SELECT * FROM ${sqlIdentifier(outputView)}) TO ${sqlString(outputPath)} (FORMAT PARQUET)`,
    )
    const bytes = await engine.db.copyFileToBuffer(outputPath)
    const file = new File([Uint8Array.from(bytes)], fileName, {
      type: 'application/vnd.apache.parquet',
      lastModified: Date.now(),
    })
    return ok({ file, rowCount: count.value, columns })
  } catch (cause) {
    return err({ kind: 'materialization-failed', detail: detailOf(cause) })
  }
}

async function verifyAndMaterialize(
  definitions: readonly SqlViewDefinition[],
  outputView: SqlViewName,
  inputs: readonly SqlPreparationInput[],
): Promise<Result<Omit<SqlPreparedOutput, 'recipe'>, SqlPreparationProblem>> {
  let engine: DuckDbEngine
  try {
    engine = await isolatedDuckDbEngine()
  } catch (cause) {
    return err({ kind: 'engine-unavailable', detail: detailOf(cause) })
  }
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
      return await materializeView(engine, connection, outputView, `${outputView}.parquet`)
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
  if (!recipe.ok)
    return err({
      kind: 'prepared-view-invalid',
      detail: 'The prepared view has an incomplete source recipe.',
    })
  const materialized = await verifyAndMaterialize(definitions.value, outputView, session.inputs)
  if (materialized.ok && session.inputs.some((input) => input.format === 'duckdb-export-file')) {
    const connection = await session.database.connect()
    try {
      const live = await materializeView(
        { db: session.database },
        connection,
        outputView,
        `${outputView}.parquet`,
      )
      if (!live.ok) return live
      const [expected, actual] = await Promise.all([
        fingerprintFile(materialized.value.file),
        fingerprintFile(live.value.file),
      ])
      if (!expected.ok || !actual.ok || expected.value !== actual.value)
        return err({
          kind: 'prepared-view-invalid',
          detail:
            'The selected view differs from the saved import and view definitions. Changes to imported table data are not saved by this recipe. Express the transformation as a view, or export the changed data.',
        })
    } finally {
      await connection.close()
    }
  }
  return materialized.ok ? ok({ ...materialized.value, recipe: recipe.value }) : materialized
}

/**
 * Builds the recorded source again from the recipe: the same input files, matched by fingerprint and
 * given the aliases the statement was written against, the recorded view definitions run on them, and
 * the recorded output view materialised. The caller checks the result against the recorded source
 * fingerprint, as it does for a file chosen by hand.
 */
export async function replaySqlRecipe(
  recipe: Extract<SelectedSource['recipe'], { readonly kind: 'sql-derived' }>,
  files: readonly File[],
): Promise<Result<File, SqlPreparationProblem>> {
  const recovered = await recoverSqlInputs(recipe.inputs, files)
  if (!recovered.ok) return recovered
  // The recorded statement is the view definitions DuckDB reported, one per line, in creation order.
  const definitions: SqlViewDefinition[] = recipe.statement
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .map((statement) => ({ name: recipe.outputView, statement }))
  const materialized = await verifyAndMaterialize(definitions, recipe.outputView, recovered.value)
  return materialized.ok ? ok(materialized.value.file) : materialized
}

export function describeSqlPreparationProblem(problem: SqlPreparationProblem): string {
  switch (problem.kind) {
    case 'no-input-files':
      return 'Choose at least one input file.'
    case 'unsupported-input':
      return `${problem.fileName} cannot be used: ${problem.detail}.`
    case 'fingerprint-failed':
      return `${problem.fileName} could not be fingerprinted: ${problem.detail}`
    case 'engine-unavailable':
      return `DuckDB could not start: ${problem.detail}`
    case 'input-registration-failed':
      return `${problem.fileName} could not be registered: ${problem.detail}`
    case 'view-list-failed':
      return `The SQL views could not be listed: ${problem.detail}`
    case 'catalog-failed':
      return `The tables could not be listed: ${problem.detail}`
    case 'relation-read-failed':
      return `The rows of ${problem.relation} could not be read: ${problem.detail}`
    case 'relation-summary-failed':
      return `The columns of ${problem.relation} could not be summarised: ${problem.detail}`
    case 'prepared-view-missing':
      return `The selected view ${problem.viewName} no longer exists. Refresh the view list and choose another.`
    case 'prepared-view-invalid':
      return `The prepared view was refused: ${problem.detail}`
    case 'prepared-view-empty':
      return 'The prepared view contains no rows.'
    case 'materialization-failed':
      return `The prepared view could not be materialized: ${problem.detail}`
    case 'cancellation-failed':
      return `The query could not be cancelled: ${problem.detail}`
    case 'session-close-failed':
      return `The SQL workspace could not release its database: ${problem.detail}`
    case 'replay-inputs-missing':
      return `The files chosen do not include ${problem.fileNames.join(', ')}, unchanged. The SQL step needs every input it was built from.`
    default: {
      const exhaustive: never = problem
      return exhaustive
    }
  }
}
