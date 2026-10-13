import { z } from 'zod'
import { answerSchema } from './agent'
import type { Turn } from './TurnView'
import { contextBasis } from './memory/access'

const base = { agent: z.string(), text: z.string(), at: z.number() }
const trace = z.discriminatedUnion('kind', [
  z.object({
    ...base,
    kind: z.enum(['task', 'request', 'writing', 'python', 'output', 'tool', 'answer', 'failed']),
  }),
  z.object({
    ...base,
    kind: z.literal('budget'),
    requestId: z.number(),
    budget: z.object({
      step: z.number(),
      maxSteps: z.number(),
      requests: z.number(),
      maxRequests: z.number(),
      elapsedMs: z.number(),
      deadlineMs: z.number(),
      outputTokenAllowance: z.number(),
    }),
  }),
  z.object({
    ...base,
    kind: z.literal('usage'),
    requestId: z.number(),
    usage: z.object({
      kind: z.enum(['reported', 'unavailable']),
      generationId: z.string().nullable().default(null),
      reason: z.enum(['early-stop', 'missing-usage']).nullable(),
      inputTokens: z.number().nullable(),
      outputTokens: z.number().nullable(),
      reasoningTokens: z.number().nullable(),
      cachedTokens: z.number().nullable(),
      costUsd: z.number().nullable(),
    }),
  }),
  z.object({
    ...base,
    kind: z.literal('provider-failure'),
    requestId: z.number(),
    failure: z.object({
      status: z.number().nullable(),
      limitSource: z.string().nullable(),
      resetAt: z.string().nullable(),
      retryAfterSeconds: z.number().nullable(),
      message: z.string(),
    }),
  }),
])
const storedTurn = z.object({
  version: z.literal(1),
  project: z.string(),
  session: z.string(),
  startedAt: z.number(),
  turn: z.object({
    id: z.string(),
    question: z.string(),
    context: z.string(),
    trace: z.array(trace),
    drafts: z.array(z.object({ title: z.string(), content: z.string() })).optional(),
    limits: z
      .object({ maxSteps: z.number(), maxRequests: z.number(), deadlineMs: z.number() })
      .optional(),
    outcome: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('answered'), answer: answerSchema }),
      z.object({ kind: z.literal('failed'), message: z.string() }),
      z.object({ kind: z.literal('cancelled') }),
    ]),
  }),
})

async function transact<T>(
  mode: IDBTransactionMode,
  action: (store: IDBObjectStore) => IDBRequest<T>,
): Promise<T> {
  const db = await new Promise<IDBDatabase>((resolve, reject) => {
    const request = indexedDB.open('hirmos-assistant-display', 1)
    request.onupgradeneeded = () => {
      const store = request.result.createObjectStore('turns', { keyPath: ['session', 'turn.id'] })
      store.createIndex('session', 'session')
      store.createIndex('project', 'project')
    }
    request.onsuccess = () => resolve(request.result)
    request.onerror = () => reject(request.error)
  })
  try {
    return await new Promise<T>((resolve, reject) => {
      const tx = db.transaction('turns', mode)
      const request = action(tx.objectStore('turns'))
      tx.oncomplete = () => resolve(request.result)
      tx.onabort = () => reject(tx.error ?? new Error('Display history transaction aborted.'))
      tx.onerror = () => reject(tx.error ?? new Error('Display history could not be stored.'))
    })
  } finally {
    db.close()
  }
}

export async function saveDisplayTurn(
  project: string,
  session: string,
  startedAt: number,
  turn: Turn,
) {
  const context = turn.context.startsWith('sha256:')
    ? turn.context
    : `sha256:${await contextBasis(turn.context)}`
  const record = storedTurn.parse({
    version: 1,
    project,
    session,
    startedAt,
    turn: { ...turn, context },
  })
  await transact('readwrite', (store) => store.put(record))
}

export async function loadDisplayTurns(session: string): Promise<Turn[]> {
  const records = await transact<unknown[]>('readonly', (store) =>
    store.index('session').getAll(session),
  )
  return records
    .map((record) => storedTurn.parse(record))
    .sort((a, b) => a.startedAt - b.startedAt)
    .map((record) => record.turn)
}

export function mergeTurnHistory(legacy: readonly Turn[], display: readonly Turn[]): Turn[] {
  const ids = new Set(display.map((turn) => turn.id))
  return [...legacy.filter((turn) => !ids.has(turn.id)), ...display]
}

export async function clearDisplayHistory(project: string) {
  await transact('readwrite', (store) => {
    const request = store.index('project').openCursor(IDBKeyRange.only(project))
    request.onsuccess = () => {
      const cursor = request.result
      if (cursor) {
        cursor.delete()
        cursor.continue()
      }
    }
    return request
  })
}
