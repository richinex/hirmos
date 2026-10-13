import { z } from 'zod'
import { nodesBySession, type DagNode } from './dagStore'
import type { Answer, Trace } from '../agent'
import type { Turn } from '../TurnView'
import { addNode, touchSession, writeTaskRecord } from './session-writes'

const evidence = z.enum([
  'project-summary',
  'user-question',
  'approved-run',
  'conversation-memory',
  'data-summary',
  'study-record',
  'saved-run',
])
const record = z.object({
  task: z.string(),
  answer: z.string(),
  citations: z.array(evidence).min(1),
  basis: z.string(),
  turnId: z.string(),
})
const historyArgs = z
  .object({
    kind: z.string().max(80).nullish(),
    limit: z.number().int().min(1).max(100).nullish(),
    turn_id: z.string().min(1).max(200).nullish(),
  })
  .strict()
const searchArgs = z.object({ pattern: z.string().min(1).max(160) }).strict()
const expandArgs = z.object({ node_id: z.string().min(1).max(200) }).strict()
const semanticArgs = z.object({ query: z.string().min(1).max(2000) }).strict()

export const MEMORY_TOOLS = [
  'DAG_HISTORY',
  'DAG_GREP',
  'DAG_EXPAND',
  'DAG_DESCRIBE',
  'DAG_SEARCH',
] as const
export interface MemoryAccess {
  read(name: string, args: unknown, signal: AbortSignal): Promise<unknown>
}

export async function contextBasis(context: string): Promise<string> {
  const bytes = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(context))
  return Array.from(new Uint8Array(bytes), (b) => b.toString(16).padStart(2, '0')).join('')
}

export function restoreTurns(nodes: readonly DagNode[]): Turn[] {
  return nodes
    .filter((node) => node.kind === 'task_record')
    .flatMap((node) => {
      const parsed = record.safeParse(node.metadata)
      return parsed.success
        ? [
            {
              id: parsed.data.turnId,
              question: parsed.data.task,
              context: `sha256:${parsed.data.basis}`,
              trace: [],
              outcome: {
                kind: 'answered' as const,
                answer: { answer: parsed.data.answer, evidence: parsed.data.citations },
              },
            },
          ]
        : []
    })
}

function query(
  session: string,
  name: string,
  args: Record<string, unknown>,
  signal: AbortSignal,
): Promise<unknown> {
  signal.throwIfAborted()
  const worker = new Worker(new URL('./query.worker.ts', import.meta.url), { type: 'module' })
  return new Promise((resolve, reject) => {
    const close = () => {
      clearTimeout(timer)
      signal.removeEventListener('abort', abort)
      worker.terminate()
    }
    const abort = () => {
      close()
      reject(new Error('Memory lookup cancelled.'))
    }
    const timer = setTimeout(() => {
      close()
      reject(new Error('Memory lookup timed out. Try a simpler search.'))
    }, 5000)
    signal.addEventListener('abort', abort, { once: true })
    worker.onerror = () => {
      close()
      reject(new Error('Memory worker failed.'))
    }
    worker.onmessage = ({ data }: MessageEvent<unknown>) => {
      close()
      const parsed = z
        .discriminatedUnion('ok', [
          z.object({ ok: z.literal(true), result: z.unknown() }),
          z.object({ ok: z.literal(false), error: z.string() }),
        ])
        .safeParse(data)
      if (!parsed.success) reject(new Error('Invalid memory response.'))
      else if (parsed.data.ok) resolve(parsed.data.result)
      else reject(new Error(parsed.data.error))
    }
    worker.postMessage({ session, name, args })
  })
}

export async function beginMemoryTask(
  session: string,
  question: string,
  context: string,
  turnId: string,
  onError: (message: string) => void,
) {
  const basis = await contextBasis(context)
  const metadata = { basis, turnId }
  await touchSession(session)
  await addNode({ session_id: session, kind: 'user_message', content: question, metadata })
  let writes = Promise.resolve()
  const enqueue = (write: () => Promise<unknown>) => {
    writes = writes
      .then(write)
      .then(() => {})
      .catch(() =>
        onError('Conversation memory could not be saved. This answer may not survive a reload.'),
      )
  }
  return {
    access: {
      async read(name: string, args: unknown, signal: AbortSignal) {
        await writes
        signal.throwIfAborted()
        if (name === 'DAG_SEARCH') {
          const parsed = semanticArgs.safeParse(args)
          if (!parsed.success) return { kind: 'invalid', reason: 'Supply a short search query.' }
          try {
            const { pickPriorAnswers } = await import('./priorAnswers')
            const matches = await pickPriorAnswers(parsed.data.query, session)
            signal.throwIfAborted()
            return {
              kind: 'recalled',
              matches,
              status:
                'Conversation history, not verified results. Recheck claims using current application records.',
            }
          } catch {
            signal.throwIfAborted()
            return {
              kind: 'unavailable',
              reason:
                'Local semantic search is not ready. Use DAG_GREP or DAG_HISTORY; do not infer that memory is empty.',
            }
          }
        }
        const parsed =
          name === 'DAG_HISTORY'
            ? historyArgs.safeParse(args)
            : name === 'DAG_GREP'
              ? searchArgs.safeParse(args)
              : name === 'DAG_EXPAND'
                ? expandArgs.safeParse(args)
                : name === 'DAG_DESCRIBE'
                  ? z.object({}).strict().safeParse(args)
                  : null
        if (!parsed?.success) return { kind: 'invalid', reason: 'Invalid memory query.' }
        const result = await query(session, name, parsed.data, signal)
        if (!Array.isArray(result)) return result
        return result
          .slice(-100)
          .map((item: unknown) => {
            const row = z
              .object({
                node_id: z.string(),
                kind: z.string(),
                content: z.string(),
                metadata: z.record(z.string(), z.unknown()).optional(),
              })
              .passthrough()
              .safeParse(item)
            return row.success
              ? {
                  ...row.data,
                  currentSpecification: row.data.metadata?.basis === basis,
                  status:
                    'Conversation history, not verified evidence. Run approvals never carry forward.',
                }
              : null
          })
          .filter((item) => item !== null)
      },
    } satisfies MemoryAccess,
    trace(event: Trace) {
      if (
        event.kind === 'request' ||
        event.kind === 'tool' ||
        event.kind === 'answer' ||
        event.kind === 'writing' ||
        event.kind === 'budget' ||
        event.kind === 'usage' ||
        event.kind === 'provider-failure'
      )
        return
      enqueue(() =>
        addNode({
          session_id: session,
          kind:
            event.kind === 'python' || event.kind === 'output'
              ? 'code_execution'
              : 'assistant_message',
          content: event.text,
          metadata: { ...metadata, agent: event.agent, traceKind: event.kind },
        }),
      )
    },
    async finish(answer: Answer) {
      enqueue(() =>
        writeTaskRecord(session, question, answer.answer, answer.evidence, undefined, metadata),
      )
      await writes
    },
    async flush() {
      await writes
    },
  }
}

export async function loadMemoryTurns(session: string) {
  return restoreTurns(await nodesBySession(session))
}
