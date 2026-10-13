// The conversation DAG, lifted from recourse's browser tier (`recourse-dag`): one IndexedDB
// database holding sessions, nodes, and edges. Octopus has no server, so unlike recourse there is
// no write-only proxy: this store is both the writer (useAsk hooks) and the reader (the DAG_*
// tools the model calls from Python). Node kinds, the superseded_by invariant (active iff null)
// and close_task supersession follow recourse exactly.

export type NodeKind =
  | 'user_message'
  | 'assistant_message'
  | 'code_execution'
  | 'system_message'
  | 'task_record'
  | 'iteration_summary'
  | 'task_summary'
  | 'session_summary'

export interface DagNode {
  node_id: string
  session_id: string
  kind: NodeKind
  content: string
  token_count: number
  created_at: string // ISO
  metadata: Record<string, unknown>
  is_summary: boolean
  compaction_level: number | null
  superseded_by: string | null // active iff null
  depth: number
}

export interface DagEdge {
  parent_id: string
  child_id: string
  ordinal: number
}

const DB = 'hirmos-assistant-memory'

function open(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const r = indexedDB.open(DB, 1)
    r.onupgradeneeded = () => {
      const db = r.result
      const sessions = db.createObjectStore('sessions', { keyPath: 'session_id' })
      sessions.createIndex('updated_at', 'updated_at')
      const nodes = db.createObjectStore('dag_nodes', { keyPath: 'node_id' })
      nodes.createIndex('session_id', 'session_id')
      nodes.createIndex('session_kind', ['session_id', 'kind'])
      const edges = db.createObjectStore('dag_edges', { keyPath: ['parent_id', 'child_id'] })
      edges.createIndex('parent_id', 'parent_id')
      edges.createIndex('child_id', 'child_id')
    }
    r.onsuccess = () => resolve(r.result)
    r.onerror = () => reject(r.error)
  })
}

async function tx<T>(
  store: string,
  mode: IDBTransactionMode,
  run: (s: IDBObjectStore) => IDBRequest<T>,
): Promise<T> {
  const db = await open()
  try {
    return await new Promise<T>((resolve, reject) => {
      const transaction = db.transaction(store, mode)
      const req = run(transaction.objectStore(store))
      transaction.oncomplete = () => resolve(req.result)
      transaction.onabort = () =>
        reject(transaction.error ?? new Error('Memory transaction aborted.'))
      transaction.onerror = () =>
        reject(transaction.error ?? new Error('Memory could not be stored.'))
    })
  } finally {
    db.close()
  }
}

/** Clear every session. The project UI uses clearProjectMemory instead. */
export async function clearDagMemory(): Promise<void> {
  await tx('dag_nodes', 'readwrite', (s) => s.clear())
  await tx('dag_edges', 'readwrite', (s) => s.clear())
  await tx('sessions', 'readwrite', (s) => s.clear())
}

/** ~4 chars per token: the browser stand-in for recourse's tiktoken count. */
export const estimateTokens = (s: string): number => Math.ceil(s.length / 4)

/** A row in the `sessions` store. It is NOT a DagNode — reading it as one forced a double cast to reach
 * `created_at`, which hid the fact that the two records have different shapes. */
export interface SessionRecord {
  readonly session_id: string
  readonly title: string | null
  readonly created_at: string
  readonly updated_at: string
}

/** Parse a stored session row; anything else is treated as absent, so `touchSession` starts it fresh. */
function asSessionRecord(v: unknown): SessionRecord | null {
  if (!v || typeof v !== 'object') return null
  const o = v as Record<string, unknown>
  if (typeof o.session_id !== 'string' || typeof o.created_at !== 'string') return null
  return {
    session_id: o.session_id,
    title: typeof o.title === 'string' ? o.title : null,
    created_at: o.created_at,
    updated_at: typeof o.updated_at === 'string' ? o.updated_at : o.created_at,
  }
}

export async function touchSession(sessionId: string): Promise<void> {
  const now = new Date().toISOString()
  const raw = await tx<unknown>(
    'sessions',
    'readonly',
    (s) => s.get(sessionId) as IDBRequest<unknown>,
  )
  const existing = asSessionRecord(raw)
  await tx('sessions', 'readwrite', (s) =>
    s.put({
      session_id: sessionId,
      title: existing?.title ?? null,
      // Keeping the ORIGINAL creation time is the point of the read: absent means the session is being
      // created now, which is the one honest default here.
      created_at: existing?.created_at ?? now,
      updated_at: now,
    } satisfies SessionRecord),
  )
}

export async function addNode(
  n: Pick<DagNode, 'session_id' | 'kind' | 'content'> & Partial<DagNode>,
): Promise<string> {
  const node: DagNode = {
    node_id: n.node_id ?? crypto.randomUUID(),
    created_at: n.created_at ?? new Date().toISOString(),
    token_count: n.token_count ?? estimateTokens(n.content),
    metadata: n.metadata ?? {},
    is_summary: n.is_summary ?? false,
    compaction_level: n.compaction_level ?? null,
    superseded_by: n.superseded_by ?? null,
    depth: n.depth ?? 0,
    session_id: n.session_id,
    kind: n.kind,
    content: n.content,
  }
  await tx('dag_nodes', 'readwrite', (s) => s.put(node))
  return node.node_id
}

export async function nodesBySession(sessionId: string): Promise<DagNode[]> {
  const rows = await tx<DagNode[]>('dag_nodes', 'readonly', (s) =>
    s.index('session_id').getAll(sessionId),
  )
  return (rows ?? []).sort((a, b) => a.created_at.localeCompare(b.created_at))
}

/** recourse close_task: everything conversational since the last task record is superseded by it,
 * so the active window collapses to the task-record chain. */
const WITHIN_TASK_KINDS = new Set<NodeKind>([
  'system_message',
  'user_message',
  'assistant_message',
  'code_execution',
  'iteration_summary',
])

export async function closeTask(sessionId: string, taskRecordId: string): Promise<void> {
  const all = await nodesBySession(sessionId)
  for (const n of all) {
    if (n.superseded_by === null && WITHIN_TASK_KINDS.has(n.kind) && n.node_id !== taskRecordId) {
      await tx('dag_nodes', 'readwrite', (s) => s.put({ ...n, superseded_by: taskRecordId }))
    }
  }
  await touchSession(sessionId)
}

export async function edgesByParent(parentId: string): Promise<DagEdge[]> {
  const rows = await tx<DagEdge[]>('dag_edges', 'readonly', (s) =>
    s.index('parent_id').getAll(parentId),
  )
  return (rows ?? []).sort((a, b) => a.ordinal - b.ordinal)
}

// ---- The DAG_* tool backends (what the model reaches from Python) ------------------------------

export interface DagToolItem {
  node_id: string
  kind: NodeKind
  content: string
  token_count: number
  created_at: string
  metadata?: Record<string, unknown>
  /** task_record items only: the question and answer, top-level for easy access. */
  task?: unknown
  answer?: unknown
}

const toItem = (n: DagNode): DagToolItem => ({
  node_id: n.node_id,
  kind: n.kind,
  content: n.content,
  token_count: n.token_count,
  created_at: n.created_at,
  metadata: n.metadata,
  // Task records also surface their essentials TOP-LEVEL: in live runs models reached for
  // record['task'] rather than record['metadata']['task'], and a memory tool that punishes the
  // obvious access pattern is a bad tool.
  ...(n.kind === 'task_record' ? { task: n.metadata.task, answer: n.metadata.answer } : {}),
})

/** The pure selection behind dagHistory, separated so the limit semantics are testable without
 * IndexedDB. `limit` keeps the LAST N: a capped history call is always "what happened most
 * recently", and the previous slice(0, limit) returned the session's OPENING instead — the one
 * window that can never contain the record the caller wants. Order stays chronological. */
export function selectHistory(
  rows: readonly DagNode[],
  kind?: string,
  limit?: number,
  turnId?: string,
): DagNode[] {
  let out = rows.filter((n) => !n.is_summary)
  if (kind) out = out.filter((n) => n.kind === kind)
  if (turnId) out = out.filter((n) => n.metadata.turnId === turnId)
  if (limit && limit > 0) out = out.slice(-limit)
  return out
}

/** Leaves (non-summary originals), chronological: superseded originals stay visible, per recourse. */
export async function dagHistory(
  sessionId: string,
  kind?: string,
  limit?: number,
  turnId?: string,
): Promise<DagToolItem[]> {
  return selectHistory(await nodesBySession(sessionId), kind, limit, turnId).map(toItem)
}

export async function dagGrep(sessionId: string, pattern: string): Promise<DagToolItem[]> {
  let re: RegExp
  try {
    re = new RegExp(pattern, 'i')
  } catch {
    re = new RegExp(pattern.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i')
  }
  return (await nodesBySession(sessionId))
    .filter(
      (n) => !n.is_summary && (re.test(n.content) || re.test(JSON.stringify(n.metadata ?? {}))),
    )
    .map(toItem)
}

/** Recover originals under a summary via edges; a leaf expands to itself (recourse DAG_EXPAND). */
export async function dagExpand(sessionId: string, nodeId: string): Promise<DagToolItem[]> {
  const all = new Map((await nodesBySession(sessionId)).map((n) => [n.node_id, n]))
  const out: DagToolItem[] = []
  const visited = new Set<string>()
  const walk = async (id: string) => {
    if (visited.has(id) || !all.has(id)) return
    visited.add(id)
    const kids = await edgesByParent(id)
    if (kids.length === 0) {
      const n = all.get(id)
      if (n) out.push(toItem(n))
      return
    }
    for (const k of kids) await walk(k.child_id)
  }
  await walk(nodeId)
  return out.sort((a, b) => a.created_at.localeCompare(b.created_at))
}

export async function clearProjectMemory(project: string): Promise<void> {
  const sessions = await tx<SessionRecord[]>('sessions', 'readonly', (s) => s.getAll())
  const db = await open()
  try {
    for (const session of sessions.filter((s) => s.session_id.startsWith(`${project}:`))) {
      const nodes = await nodesBySession(session.session_id)
      const ids = new Set(nodes.map((node) => node.node_id))
      const edges = await tx<DagEdge[]>('dag_edges', 'readonly', (s) => s.getAll())
      await new Promise<void>((resolve, reject) => {
        const transaction = db.transaction(['sessions', 'dag_nodes', 'dag_edges'], 'readwrite')
        for (const id of ids) transaction.objectStore('dag_nodes').delete(id)
        for (const edge of edges)
          if (ids.has(edge.parent_id) || ids.has(edge.child_id))
            transaction.objectStore('dag_edges').delete([edge.parent_id, edge.child_id])
        transaction.objectStore('sessions').delete(session.session_id)
        transaction.oncomplete = () => resolve()
        transaction.onabort = () =>
          reject(transaction.error ?? new Error('Memory deletion aborted.'))
        transaction.onerror = () =>
          reject(transaction.error ?? new Error('Memory could not be deleted.'))
      })
    }
  } finally {
    db.close()
  }
}

export async function dagDescribe(sessionId: string): Promise<Record<string, unknown>> {
  const all = await nodesBySession(sessionId)
  const byKind: Record<string, number> = {}
  for (const n of all) byKind[n.kind] = (byKind[n.kind] ?? 0) + 1
  return {
    session_id: sessionId,
    nodes: all.length,
    active: all.filter((n) => n.superseded_by === null).length,
    by_kind: byKind,
    tokens: all.reduce((t, n) => t + n.token_count, 0),
  }
}
