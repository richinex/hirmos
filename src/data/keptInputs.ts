import * as duckdb from '@duckdb/duckdb-wasm'
import type { SourceFingerprint } from '@/domain/dataset'
import { err, ok, type Result } from '@/domain/dop'
import {
  dialectRecordSchema,
  NOT_RECORDED,
  type DialectRecord,
  type FileReading,
} from '@/domain/fileReading'
import { KEPT_META, KEPT_TABLE, type KeptName } from '@/domain/keptInput'
import { detectLayout, type DuckDbEngine } from './duckdb'
import { dialectRecord, fileRelation } from './fileRelation'
import { sourceCacheAvailable } from './sourceCache'

export interface KeptRelation {
  readonly relation: string
  readonly dialect: DialectRecord
}

export type KeptProblem = { readonly kind: 'kept-unavailable'; readonly detail: string }

const DIRECTORY = ['duckdb', 'fs'] as const

const detailOf = (cause: unknown): string =>
  cause instanceof Error ? cause.message : String(cause)
const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`
const sqlIdentifier = (value: string): string => `"${value.replaceAll('"', '""')}"`

const databasePath = (name: KeptName): string => `opfs://${name}.duckdb`
const writePaths = (name: KeptName): readonly string[] => {
  const path = databasePath(name)
  return [path, `${path}.wal`, `${path}.wal.checkpoint`, `${path}.wal.recovery`]
}
const alias = (name: KeptName): string => sqlIdentifier(`k_${name}`)

type Attaching = Map<KeptName, Promise<Result<KeptRelation, KeptProblem>>>

const attached = new WeakMap<duckdb.AsyncDuckDB, Map<KeptName, KeptRelation>>()
const attaching = new WeakMap<duckdb.AsyncDuckDB, Attaching>()

const attachedTo = (db: duckdb.AsyncDuckDB): Map<KeptName, KeptRelation> => {
  const existing = attached.get(db)
  if (existing !== undefined) return existing
  const created = new Map<KeptName, KeptRelation>()
  attached.set(db, created)
  return created
}

const attachingTo = (db: duckdb.AsyncDuckDB): Attaching => {
  const existing = attaching.get(db)
  if (existing !== undefined) return existing
  const created: Attaching = new Map()
  attaching.set(db, created)
  return created
}

const keptDirectory = async (): Promise<FileSystemDirectoryHandle> => {
  let directory = await navigator.storage.getDirectory()
  for (const part of DIRECTORY) directory = await directory.getDirectoryHandle(part)
  return directory
}

const exists = async (name: KeptName): Promise<boolean> => {
  const directory = await keptDirectory().catch(() => null)
  if (directory === null) return false
  return directory.getFileHandle(`${name}.duckdb`).then(
    () => true,
    () => false,
  )
}

async function attachRead(
  engine: DuckDbEngine,
  name: KeptName,
  fingerprint: SourceFingerprint,
): Promise<Result<KeptRelation, KeptProblem>> {
  const path = databasePath(name)
  const connection = await engine.db.connect()
  let attachedHere = false
  try {
    await engine.db.registerOPFSFileName(path, 'read')
    await connection.query(`ATTACH ${sqlString(path)} AS ${alias(name)} (READ_ONLY)`)
    attachedHere = true
    const meta = await connection.query(
      `SELECT fingerprint, dialect FROM ${alias(name)}.${KEPT_META} LIMIT 1`,
    )
    if (meta.numRows !== 1 || meta.getChild('fingerprint')?.get(0) !== fingerprint)
      throw new Error('The kept table is not the file it was built from.')
    const parsed = dialectRecordSchema.safeParse(
      JSON.parse(String(meta.getChild('dialect')?.get(0))),
    )
    return ok({
      relation: `${alias(name)}.${KEPT_TABLE}`,
      dialect: parsed.success ? parsed.data : NOT_RECORDED,
    })
  } catch (cause) {
    if (attachedHere) await connection.query(`DETACH ${alias(name)}`).catch(() => undefined)
    await engine.db.dropFiles([path]).catch(() => undefined)
    return err({ kind: 'kept-unavailable', detail: detailOf(cause) })
  } finally {
    await connection.close()
  }
}

async function removeFiles(name: KeptName): Promise<void> {
  const directory = await keptDirectory().catch(() => null)
  if (directory === null) return
  for (const file of [`${name}.duckdb`, `${name}.duckdb.wal`])
    await directory.removeEntry(file).catch(() => undefined)
}

async function build(
  engine: DuckDbEngine,
  name: KeptName,
  file: File,
  reading: FileReading,
  fingerprint: SourceFingerprint,
): Promise<Result<null, KeptProblem>> {
  await removeFiles(name)
  const path = databasePath(name)
  const source = `kept-source-${crypto.randomUUID()}.${reading.format}`
  const connection = await engine.db.connect()
  let attachedHere = false
  try {
    for (const write of writePaths(name)) await engine.db.registerOPFSFileName(write, 'write')
    await engine.db.registerFileHandle(
      source,
      file,
      duckdb.DuckDBDataProtocol.BROWSER_FILEREADER,
      true,
    )
    await connection.query(`ATTACH ${sqlString(path)} AS ${alias(name)}`)
    attachedHere = true
    let layout = await detectLayout(connection, reading, source, 'default')
    try {
      await connection.query(
        `CREATE TABLE ${alias(name)}.${KEPT_TABLE} AS SELECT * FROM ${fileRelation(reading, source, undefined, layout)}`,
      )
    } catch (cause) {
      if (!/could not convert|conversion error/i.test(detailOf(cause))) throw cause
      layout = await detectLayout(connection, reading, source, 'whole-file')
      await connection.query(
        `CREATE TABLE ${alias(name)}.${KEPT_TABLE} AS SELECT * FROM ${fileRelation(reading, source, undefined, layout)}`,
      )
    }
    await connection.query(
      `CREATE TABLE ${alias(name)}.${KEPT_META} AS SELECT ${sqlString(fingerprint)} AS fingerprint, ${sqlString(JSON.stringify(dialectRecord(layout)))} AS dialect, count(*)::UBIGINT AS rows FROM ${alias(name)}.${KEPT_TABLE}`,
    )
    await connection.query(`DETACH ${alias(name)}`)
    attachedHere = false
    return ok(null)
  } catch (cause) {
    if (attachedHere) await connection.query(`DETACH ${alias(name)}`).catch(() => undefined)
    return err({ kind: 'kept-unavailable', detail: detailOf(cause) })
  } finally {
    await connection.close()
    await engine.db.dropFile(source).catch(() => undefined)
    await engine.db.dropFiles([...writePaths(name)]).catch(() => undefined)
  }
}

const locked = <Value>(name: KeptName, work: () => Promise<Value>): Promise<Value> =>
  typeof navigator.locks === 'undefined'
    ? work()
    : (navigator.locks.request(`hirmos-kept-${name}`, work) as Promise<Value>)

const attachOrBuild = (
  engine: DuckDbEngine,
  name: KeptName,
  file: File,
  reading: FileReading,
  fingerprint: SourceFingerprint,
): Promise<Result<KeptRelation, KeptProblem>> =>
  locked(name, async () => {
    if (await exists(name)) {
      const read = await attachRead(engine, name, fingerprint)
      if (read.ok) {
        attachedTo(engine.db).set(name, read.value)
        return read
      }
    }
    const built = await build(engine, name, file, reading, fingerprint)
    if (!built.ok) return built
    const reread = await attachRead(engine, name, fingerprint)
    if (reread.ok) attachedTo(engine.db).set(name, reread.value)
    return reread
  })

const attachExisting = (
  engine: DuckDbEngine,
  name: KeptName,
  fingerprint: SourceFingerprint,
): Promise<Result<KeptRelation, KeptProblem>> =>
  locked(name, async () => {
    if (!(await exists(name)))
      return err({ kind: 'kept-unavailable', detail: 'The kept table has not been built.' })
    const read = await attachRead(engine, name, fingerprint)
    if (read.ok) attachedTo(engine.db).set(name, read.value)
    return read
  })

const once = (
  engine: DuckDbEngine,
  name: KeptName,
  work: () => Promise<Result<KeptRelation, KeptProblem>>,
): Promise<Result<KeptRelation, KeptProblem>> => {
  if (!sourceCacheAvailable())
    return Promise.resolve(
      err({ kind: 'kept-unavailable', detail: 'This browser has no private file system.' }),
    )
  const known = attachedTo(engine.db).get(name)
  if (known !== undefined) return Promise.resolve(ok(known))
  const inFlight = attachingTo(engine.db)
  const pending = inFlight.get(name)
  if (pending !== undefined) return pending
  const task = work().finally(() => {
    if (inFlight.get(name) === task) inFlight.delete(name)
  })
  inFlight.set(name, task)
  return task
}

export const attachKept = (
  engine: DuckDbEngine,
  name: KeptName,
  fingerprint: SourceFingerprint,
): Promise<Result<KeptRelation, KeptProblem>> =>
  once(engine, name, () => attachExisting(engine, name, fingerprint))

export const keptRelation = (
  engine: DuckDbEngine,
  name: KeptName,
  file: File,
  reading: FileReading,
  fingerprint: SourceFingerprint,
): Promise<Result<KeptRelation, KeptProblem>> =>
  once(engine, name, () => attachOrBuild(engine, name, file, reading, fingerprint))

const keptFingerprint = (entry: string): string | null =>
  /^kept-[^-]+-([a-f0-9]{64})-/.exec(entry)?.[1] ?? null

async function removeKeptWhere(discard: (fingerprint: string) => boolean): Promise<void> {
  if (!sourceCacheAvailable()) return
  const directory = await keptDirectory().catch(() => null)
  if (directory === null) return
  const entries = directory as unknown as { keys(): AsyncIterable<string> }
  const removable: string[] = []
  for await (const entry of entries.keys()) {
    const fingerprint = keptFingerprint(entry)
    if (fingerprint !== null && discard(fingerprint)) removable.push(entry)
  }
  for (const entry of removable) await directory.removeEntry(entry).catch(() => undefined)
}

export const removeKeptInputs = (fingerprint: SourceFingerprint): Promise<void> =>
  removeKeptWhere((kept) => kept === fingerprint)

export const sweepKeptInputs = (cached: ReadonlySet<string>): Promise<void> =>
  removeKeptWhere((kept) => !cached.has(kept))
