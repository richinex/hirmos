// The DAG recall's read side. session-writes keeps one task_record per answered question; this picks the
// records similar to an incoming question with the same MiniLM embedder that powers semantic_search, and
// the CodeAct engine injects them as already-answered priors: re-verify the cited ids with one query, do
// not re-explore. Like exemplars.ts, matching is an assist, never a blocker.
import { embedMany } from './embeddings'
import { nodesBySession, type DagNode } from './dagStore'
import type { PriorAnswer } from '../octopus/rlm/priorTasks'

// Vectors for recalled questions, computed once per session (a session's records are few and append-only).
const cache = new Map<string, Float32Array>()

function cosine(a: Float32Array, b: Float32Array): number {
  let dot = 0,
    na = 0,
    nb = 0
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i]
    na += a[i] * a[i]
    nb += b[i] * b[i]
  }
  const d = Math.sqrt(na) * Math.sqrt(nb)
  return d === 0 ? 0 : dot / d
}

/** Below this cosine the questions are not the same question. Stricter than the exemplar floor of 0.5: an
 * exemplar offers a shape to imitate, but this block claims "you already answered this", and a wrong claim
 * sends the model confirming the wrong ids. */
const MATCH_FLOOR = 0.6
/** First-ever embed can trigger the ~23MB model download; an ask must never wait on that. */
const TIMEOUT_MS = 4000

/** Parse one task_record node into the PriorAnswer shape, or null when its metadata cannot be trusted.
 * Records written by writeTaskRecord always fit; anything else is dropped rather than recalled
 * half-formed. */
export function asPriorAnswer(n: DagNode): PriorAnswer | null {
  if (n.kind !== 'task_record') return null
  const { task, answer, citations } = n.metadata
  if (typeof task !== 'string' || !task.trim()) return null
  if (typeof answer !== 'string' || !answer.trim()) return null
  return {
    question: task,
    answer,
    citations: Array.isArray(citations)
      ? citations.filter((c): c is string => typeof c === 'string' && c.trim() !== '')
      : [],
  }
}

/** The pure matching core, separated so the floor and cap are testable without a worker. Scores every
 * candidate against the question vector, drops the unalike, keeps the best `k`. */
export function matchPriorAnswers(
  qv: Float32Array,
  candidates: ReadonlyArray<{ record: PriorAnswer; vec: Float32Array }>,
  k = 2,
): PriorAnswer[] {
  return candidates
    .map((c) => ({ record: c.record, score: cosine(qv, c.vec) }))
    .filter((x) => x.score >= MATCH_FLOOR)
    .sort((a, b) => b.score - a.score)
    .slice(0, k)
    .map((x) => x.record)
}

export async function pickPriorAnswers(
  question: string,
  sessionId: string,
  k = 2,
): Promise<PriorAnswer[]> {
  try {
    // Scoped to THIS session, like the DAG tools: another session's answer is another conversation.
    const records = (await nodesBySession(sessionId))
      .map(asPriorAnswer)
      .filter((r): r is PriorAnswer => r !== null)
    if (records.length === 0) return []
    const missing = [...new Set(records.map((r) => r.question))].filter((q) => !cache.has(q))
    const rows = await Promise.race([
      embedMany([question, ...missing]),
      new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error('embed timeout')), TIMEOUT_MS),
      ),
    ])
    const qv = rows[0]
    missing.forEach((m, i) => cache.set(m, rows[i + 1]))
    // Say it in data, not with a `!`: a record whose vector is somehow missing drops out of the match.
    return matchPriorAnswers(
      qv,
      records.flatMap((r) => {
        const vec = cache.get(r.question)
        return vec ? [{ record: r, vec }] : []
      }),
      k,
    )
  } catch (error) {
    throw error
  }
}
