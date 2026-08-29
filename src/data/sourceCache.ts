import { err, ok, type Result } from '@/domain/dop'
import type { SourceFingerprint } from '@/domain/dataset'
import { reportStorageFailure } from './storageHealth'

/**
 * The origin-private file system holds a source the user chose to cache locally (DESIGN.md §5.6), keyed by
 * its fingerprint so the same file is stored once however many projects use it. Nothing is cached unless
 * asked; a project whose source is not cached asks for the file again on reopening.
 */

const DIRECTORY = 'sources'

export type SourceCacheProblem =
  | { readonly kind: 'cache-unavailable' }
  | { readonly kind: 'cache-write-failed'; readonly detail: string }

export const sourceCacheAvailable = (): boolean => typeof navigator !== 'undefined' && 'storage' in navigator && typeof navigator.storage.getDirectory === 'function'

const directory = async (create: boolean): Promise<FileSystemDirectoryHandle | null> => {
  if (!sourceCacheAvailable()) return null
  try {
    const root = await navigator.storage.getDirectory()
    return await root.getDirectoryHandle(DIRECTORY, { create })
  } catch {
    return null
  }
}

interface CachedMeta {
  readonly name: string
  readonly type: string
  readonly lastModified: number
}

const detailOf = (cause: unknown): string => (cause instanceof Error ? cause.message : String(cause))

export async function cacheSource(fingerprint: SourceFingerprint, file: File): Promise<Result<null, SourceCacheProblem>> {
  const dir = await directory(true)
  if (dir === null) return err({ kind: 'cache-unavailable' })
  try {
    const handle = await dir.getFileHandle(fingerprint, { create: true })
    const writable = await handle.createWritable()
    await file.stream().pipeTo(writable)
    const metaHandle = await dir.getFileHandle(`${fingerprint}.json`, { create: true })
    const metaWritable = await metaHandle.createWritable()
    const meta: CachedMeta = { name: file.name, type: file.type, lastModified: file.lastModified }
    await metaWritable.write(JSON.stringify(meta))
    await metaWritable.close()
    return ok(null)
  } catch (cause) {
    const detail = detailOf(cause)
    reportStorageFailure({ store: 'opfs/sources', reason: detail, at: new Date().toISOString() })
    return err({ kind: 'cache-write-failed', detail })
  }
}

/** The cached file with its original name, type and timestamp, or null when nothing is cached under this fingerprint. */
export async function readCachedSource(fingerprint: SourceFingerprint): Promise<File | null> {
  const dir = await directory(false)
  if (dir === null) return null
  try {
    const handle = await dir.getFileHandle(fingerprint)
    const blob = await handle.getFile()
    let meta: CachedMeta = { name: fingerprint, type: '', lastModified: blob.lastModified }
    try {
      const metaHandle = await dir.getFileHandle(`${fingerprint}.json`)
      meta = JSON.parse(await (await metaHandle.getFile()).text()) as CachedMeta
    } catch { /* the data file alone is still usable */ }
    return new File([blob], meta.name, { type: meta.type, lastModified: meta.lastModified })
  } catch {
    return null
  }
}

export async function removeCachedSource(fingerprint: SourceFingerprint): Promise<void> {
  const dir = await directory(false)
  if (dir === null) return
  for (const name of [fingerprint, `${fingerprint}.json`]) {
    try { await dir.removeEntry(name) } catch { /* already gone */ }
  }
}
