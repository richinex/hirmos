/**
 * Session short-term memory, the recourse pattern in a serverless browser app: after each answered
 * question the host keeps a compact TASK RECORD (question, answer, citations, surviving REPL variables),
 * and the next question's system prompt carries the recent records. Raw transcripts never ride along.
 */
export interface PriorTask {
  question: string
  answer: string
  record?: { turnId: string; kind: 'task_record' | 'user_message' }
  /** Cited node ids, when the answer was grounded. */
  citations?: readonly string[]
  /** name -> short repr of variables the task left in the (persistent) REPL namespace. */
  vars?: Readonly<Record<string, string>>
}

/** True when some records predate a namespace clear (their variables no longer exist). */
export function anyVarsLost(tasks: readonly PriorTask[]): boolean {
  return tasks.some((t) => !t.vars)
}

const clip = (s: string, n: number) => (s.length > n ? `${s.slice(0, n)}…` : s)

export function priorTaskRecords(tasks: readonly PriorTask[]): string {
  return tasks
    .map((t) => {
      const lines = [`[Previous Task] Q: ${clip(t.question, 300)}`, `A: ${clip(t.answer, 800)}`]
      if (t.question.length > 300 || t.answer.length > 800) {
        lines.push('This recap is shortened; omitted details may remain in the original record.')
      }
      if (t.record) {
        lines.push(
          `Read the original: await DAG_HISTORY(kind=${JSON.stringify(t.record.kind)}, turn_id=${JSON.stringify(t.record.turnId)})`,
        )
      }
      if (t.citations?.length) lines.push(`Cited nodes: ${t.citations.slice(0, 12).join(', ')}`)
      const vars = Object.entries(t.vars ?? {})
      if (vars.length)
        lines.push(
          `Variables still in the namespace: ${vars.map(([k, v]) => `${k}=${clip(v, 120)}`).join('; ')}`,
        )
      return lines.join('\n')
    })
    .join('\n\n')
}

export function priorTaskBlock(tasks: readonly PriorTask[] | undefined): string {
  if (!tasks || tasks.length === 0) return ''
  const records = priorTaskRecords(tasks)
  return `\n\n## Conversation so far
This is a continuing session. Earlier questions in this chat, newest last:

${records}

Rules for using this history:
- The REPL namespace PERSISTS across questions: variables listed above still exist and you may reuse
  them (a dataframe you built earlier is still there).
- When the user refers to something from earlier ("the previous result", "that table", "what did I
  ask"), ground your answer in the records above or re-inspect the variable; never guess.
- Numbers must come from the live graph or a live variable, never from memory of prose.${
    anyVarsLost(tasks)
      ? `
- [REPL Reset] Some earlier tasks list no variables: the namespace was cleared since (the graph
  changed or the session restarted). Those objects no longer exist; recompute what you need.`
      : ''
  }`
}

/** CodeAct-only: documents the DAG_* retrieval tools (recourse's conversation_memory block). The cypher
 *  and mcp engines have no Python surface, so they get records only. */
export function dagToolsBlock(): string {
  return `

## Session memory tools (call these from Python)
- DAG_HISTORY(kind=None, limit=None): chronological records of this chat. Each item has node_id,
  kind, content. kind 'task_record' gives one compact record per earlier question with 'task' and
  'answer' as top-level fields; kind 'user_message' items hold the user's verbatim words in content.
  Ordered oldest first; limit keeps the last N (the most recent). The newest user_message is the
  question you are answering right now.
- DAG_GREP(pattern): case-insensitive regex over all records when you need to FIND something said
  or computed earlier.
- DAG_EXPAND(node_id): recover originals under a summary (use when content shows [source_ids: ...]).
- DAG_DESCRIBE(): the shape of this session's memory.

When the user references anything from earlier in the conversation (the previous result, that
table, what did I ask), you MUST call DAG_HISTORY or DAG_GREP before answering. Never speculate
about past results you have not retrieved.

Before your FIRST exploratory query: if the question could repeat something this session already
answered, call DAG_HISTORY(kind='task_record') first. On a match, reuse the record's cited node ids
and CONFIRM with ONE live query over those ids instead of re-exploring. The grounding contract still
holds: never carry a number forward from a record — every figure must come from a row the confirming
query returned.

Rule evaluation takes precedence over this recall recipe. When evaluate_rule exposes a matching enabled
rule, call it directly. Do not call DAG_HISTORY or confirm a recalled answer with Cypher first.`
}

/** A task record recalled because the incoming question matches one this session already answered. The
 *  host picks these with the same MiniLM embedder exemplars use, and the model's job shrinks from
 *  exploring to confirming. */
export interface PriorAnswer {
  readonly question: string
  readonly answer: string
  /** The node ids the recalled answer cited: what the confirming query should read. */
  readonly citations: readonly string[]
}

export function priorAnswerBlock(hits: readonly PriorAnswer[] | undefined): string {
  if (!hits || hits.length === 0) return ''
  const items = hits
    .map((h) => {
      const lines = [`Q: ${clip(h.question, 300)}`, `A: ${clip(h.answer, 800)}`]
      if (h.citations.length) lines.push(`Cited nodes: ${h.citations.slice(0, 12).join(', ')}`)
      return lines.join('\n')
    })
    .join('\n\n')
  return `\n\n## You already answered this in this session
${items}

Re-verify against the live graph with ONE query on the cited ids, then answer. Do not re-explore.
Numbers must come from that query's rows, never from the recalled text.

Exception: when evaluate_rule exposes a matching enabled rule, ignore the confirmation instruction above
and call evaluate_rule directly. A recalled answer is not policy evidence.`
}
