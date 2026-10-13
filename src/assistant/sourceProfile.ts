import { DuckDBDataProtocol } from '@duckdb/duckdb-wasm'
import { isolatedDuckDbEngine } from '@/data/duckdb'
import { fileRelation } from '@/data/fileRelation'
import { fingerprintFile } from '@/data/fingerprint'
import type { SelectedSource } from '@/domain/workflow'
import { profileTable, type ProfileRequest } from './preparation'

export async function profileAcceptedSource(
  source: SelectedSource,
  expectedFingerprint: string | null,
  request: ProfileRequest,
  signal: AbortSignal,
) {
  signal.throwIfAborted()
  if (request.table !== 'source') throw new Error('Choose the accepted source table: source.')
  if (expectedFingerprint !== null) {
    const fingerprint = await fingerprintFile(source.file)
    if (!fingerprint.ok) throw new Error(fingerprint.error.detail)
    if (fingerprint.value !== expectedFingerprint)
      throw new Error('The source file changed. Inspect the current data before profiling it.')
  }
  signal.throwIfAborted()
  const { db } = await isolatedDuckDbEngine()
  try {
    signal.throwIfAborted()
    const path = `source.${source.format}`
    await db.registerFileHandle(path, source.file, DuckDBDataProtocol.BROWSER_FILEREADER, true)
    const connection = await db.connect()
    try {
      await connection.query(`CREATE VIEW source AS SELECT * FROM ${fileRelation(source, path)}`)
    } finally {
      await connection.close()
    }
    return await profileTable(db, ['source'], request, signal)
  } finally {
    await db.terminate()
  }
}
