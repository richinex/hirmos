import { err, ok, type Result } from '@/domain/dop'
import {
  headerOf,
  parseSnapshot,
  samePersistedProjectContent,
  serialiseSnapshot,
  type PersistedProject,
  type SavedProjectHeader,
  type SnapshotProblem,
} from '@/domain/persistence'
import type { ProjectId } from '@/domain/workflow'
import { reportStorageFailure, reportStorageRecovered } from './storageHealth'

/** One IndexedDB database with one object store: project id → { header, body }. The body is the tagged JSON text. */

const DB_NAME = 'hirmos'
const STORE = 'projects'

interface StoredRecord {
  readonly header: SavedProjectHeader
  readonly body: string
}

export type ProjectStoreProblem =
  | { readonly kind: 'storage-unavailable'; readonly detail: string }
  | { readonly kind: 'not-found'; readonly id: ProjectId }
  | SnapshotProblem

export type SaveProjectOutcome =
  | { readonly kind: 'saved'; readonly header: SavedProjectHeader }
  | { readonly kind: 'unchanged'; readonly header: SavedProjectHeader }

const open = (): Promise<IDBDatabase> =>
  new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, 1)
    request.onupgradeneeded = () => {
      request.result.createObjectStore(STORE)
    }
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error ?? new Error('IndexedDB could not open.'))
  })

const detailOf = (cause: unknown): string =>
  cause instanceof Error ? cause.message : String(cause)

const transact = <T>(
  mode: IDBTransactionMode,
  run: (store: IDBObjectStore) => IDBRequest<T>,
): Promise<T> =>
  open().then(
    (db) =>
      new Promise<T>((resolve, reject) => {
        const tx = db.transaction(STORE, mode)
        const request = run(tx.objectStore(STORE))
        request.onsuccess = () => resolve(request.result)
        request.onerror = () => reject(request.error ?? new Error('IndexedDB request failed.'))
        tx.oncomplete = () => db.close()
        tx.onabort = () => {
          db.close()
          reject(tx.error ?? new Error('IndexedDB transaction aborted.'))
        }
      }),
  )

export async function saveProject(
  snapshot: PersistedProject,
): Promise<Result<SavedProjectHeader, ProjectStoreProblem>> {
  const record: StoredRecord = { header: headerOf(snapshot), body: serialiseSnapshot(snapshot) }
  try {
    await transact('readwrite', (store) => store.put(record, snapshot.project.id))
    reportStorageRecovered()
    return ok(record.header)
  } catch (cause) {
    const detail = detailOf(cause)
    reportStorageFailure({ store: DB_NAME, reason: detail, at: new Date().toISOString() })
    return err({ kind: 'storage-unavailable', detail })
  }
}

/** Preserve the previous write time when the analysis itself has not changed. */
export async function saveProjectIfChanged(
  snapshot: PersistedProject,
): Promise<Result<SaveProjectOutcome, ProjectStoreProblem>> {
  const existing = await loadProject(snapshot.project.id)
  if (existing.ok && samePersistedProjectContent(existing.value, snapshot)) {
    return ok({ kind: 'unchanged', header: headerOf(existing.value) })
  }

  const saved = await saveProject(snapshot)
  return saved.ok ? ok({ kind: 'saved', header: saved.value }) : saved
}

export async function listProjects(): Promise<readonly SavedProjectHeader[]> {
  try {
    const records = await transact<StoredRecord[]>(
      'readonly',
      (store) => store.getAll() as IDBRequest<StoredRecord[]>,
    )
    return records.map((record) => record.header).sort((a, b) => b.savedAt.localeCompare(a.savedAt))
  } catch {
    return []
  }
}

export async function loadProject(
  id: ProjectId,
): Promise<Result<PersistedProject, ProjectStoreProblem>> {
  let record: StoredRecord | undefined
  try {
    record = await transact<StoredRecord | undefined>(
      'readonly',
      (store) => store.get(id) as IDBRequest<StoredRecord | undefined>,
    )
  } catch (cause) {
    return err({ kind: 'storage-unavailable', detail: detailOf(cause) })
  }
  if (record === undefined) return err({ kind: 'not-found', id })
  return parseSnapshot(record.body)
}

export async function deleteProject(id: ProjectId): Promise<Result<null, ProjectStoreProblem>> {
  try {
    await transact('readwrite', (store) => store.delete(id))
    return ok(null)
  } catch (cause) {
    return err({ kind: 'storage-unavailable', detail: detailOf(cause) })
  }
}
