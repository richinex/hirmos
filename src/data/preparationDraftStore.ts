import { z } from 'zod'
import { parsePipelineDraft, type PipelineGraph } from '@/domain/pipeline'
import {
  inputDescriptorSchema,
  parseInputDescriptors,
  type SqlPreparationInput,
} from '@/domain/sourceInputs'

export type PreparationDraft =
  | { kind: 'pipeline'; graph: PipelineGraph; inputs: readonly SqlPreparationInput[] }
  | { kind: 'sql'; name: string; text: string; inputs: readonly SqlPreparationInput[] }

const envelope = z.object({
  version: z.literal(1),
  draft: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('pipeline'), graph: z.unknown(), inputs: z.array(z.unknown()) }),
    z.object({
      kind: z.literal('sql'),
      name: z.string(),
      text: z.string(),
      inputs: z.array(z.unknown()),
    }),
  ]),
})

async function access<T>(
  mode: IDBTransactionMode,
  operation: (store: IDBObjectStore) => IDBRequest<T>,
): Promise<T> {
  const db = await new Promise<IDBDatabase>((resolve, reject) => {
    const request = indexedDB.open('hirmos-preparation-drafts', 1)
    request.onupgradeneeded = () => request.result.createObjectStore('drafts')
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
  try {
    return await new Promise<T>((resolve, reject) => {
      const transaction = db.transaction('drafts', mode)
      const request = operation(transaction.objectStore('drafts'))
      transaction.oncomplete = () => resolve(request.result)
      transaction.onabort = () => reject(transaction.error)
      transaction.onerror = () => reject(transaction.error)
    })
  } finally {
    db.close()
  }
}

let writes: Promise<unknown> = Promise.resolve()
const enqueue = <T>(operation: () => Promise<T>): Promise<T> => {
  const pending = writes.then(operation)
  writes = pending.catch(() => undefined)
  return pending
}

export const savePreparationDraft = (
  project: string,
  kind: PreparationDraft['kind'],
  basis: string,
  draft: PreparationDraft,
) =>
  enqueue(() =>
    access('readwrite', (store) => store.put({ version: 1, draft }, [project, kind, basis])),
  )

export const removePreparationDraft = (
  project: string,
  kind: PreparationDraft['kind'],
  basis: string,
) => enqueue(() => access('readwrite', (store) => store.delete([project, kind, basis])))

export async function loadPreparationDraft(
  project: string,
  kind: PreparationDraft['kind'],
  basis: string,
): Promise<PreparationDraft | null> {
  await writes
  const raw = await access('readonly', (store) => store.get([project, kind, basis]))
  if (raw === undefined) return null
  const { draft } = envelope.parse(raw)
  if (draft.kind !== kind) throw new Error('The saved draft belongs to another editor.')
  const files = draft.inputs.map((input) =>
    z
      .object({ file: z.instanceof(File) })
      .passthrough()
      .parse(input),
  )
  const descriptors = parseInputDescriptors(
    files.map(({ file: _file, ...rest }) => inputDescriptorSchema.parse(rest)),
  )
  if (!descriptors.ok) throw new Error('The saved draft has invalid input files.')
  const inputs = descriptors.value.map((input, index) => ({ ...input, file: files[index]!.file }))
  if (draft.kind === 'sql') return { ...draft, inputs }
  return { kind: 'pipeline', graph: parsePipelineDraft(draft.graph), inputs }
}

export async function deletePreparationDrafts(project: string): Promise<void> {
  await writes
  const keys = await access('readonly', (store) => store.getAllKeys())
  await Promise.all(
    keys
      .filter((key) => Array.isArray(key) && key[0] === project)
      .map((key) => access('readwrite', (store) => store.delete(key))),
  )
}
