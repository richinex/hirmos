// In-browser text embeddings (all-MiniLM-L6-v2, 384-dim) in a dedicated worker (workers/embed.worker.ts),
// because indexing a large ingest on the main thread janked the UI for tens of seconds. WebGPU when
// available, WASM otherwise. Same model and dims as the devops MCP's fastembed layer, so semantic matching
// agrees across both. The worker and its ~23MB model load lazily on first embed.
import { EMBED_DIM, type Embedder, type EmbedStatus } from './embeddingTypes'
export { EMBED_DIM }

// ---- the embed status bus ----
// The session's onEmbed (composed in runtime.ts) reports here; useCompile turns each status into a log
// line. The bus lives here rather than in runtime.ts because runtime imports this module, and the self-test
// below must report without a circular import.
const listeners = new Set<(s: EmbedStatus) => void>()

/** Hear semantic-index progress/failure/completion. Returns the unsubscribe. */
export function onEmbedStatus(fn: (s: EmbedStatus) => void): () => void {
  listeners.add(fn)
  return () => listeners.delete(fn)
}

/** Fan one status out to every subscriber (runtime.ts passes this as the session's onEmbed). */
export function reportEmbedStatus(s: EmbedStatus): void {
  for (const l of listeners) l(s)
}

let _worker: Worker | null = null
let _seq = 0

/** Whether embedding has been requested at least once (drives a "still loading" hint in the UI). */
export function embeddingsRequested(): boolean {
  return _worker !== null
}

// One in-flight request at a time is the natural usage, so an id-matched round-trip suffices. Same protocol
// as the whisper worker.
function roundTrip(texts: string[]): Promise<Float32Array[]> {
  _worker ??= new Worker(new URL('./embed.worker.ts', import.meta.url), { type: 'module' })
  const w = _worker
  const id = ++_seq
  return new Promise((resolve, reject) => {
    const onMessage = (
      e: MessageEvent<{
        id: number
        ok: boolean
        dim?: number
        flat?: Float32Array
        error?: string
      }>,
    ) => {
      if (e.data.id !== id) return
      w.removeEventListener('message', onMessage)
      if (!e.data.ok || !e.data.flat || !e.data.dim) {
        reject(new Error(e.data.error ?? 'embedding failed'))
        return
      }
      const { flat, dim } = e.data
      const rows: Float32Array[] = []
      for (let i = 0; i < texts.length; i++) rows.push(flat.subarray(i * dim, (i + 1) * dim))
      resolve(rows)
    }
    w.addEventListener('message', onMessage)
    w.postMessage({ id, texts })
  })
}

// ---- boot self-test ----
// The first embed of the session embeds a constant and checks for 384 finite floats. The q8+webgpu combo
// once produced structurally valid but malformed vectors, and nothing downstream could tell: searches just
// scored garbage. Fail here, once, through the same status bus the indexer reports on.
let _selfTest: Promise<void> | null = null
let _selfTestReported = false
function selfTest(): Promise<void> {
  return (_selfTest ??= (async () => {
    const [v] = await roundTrip(['octopus embedding self-test'])
    const healthy =
      v instanceof Float32Array && v.length === EMBED_DIM && v.every((x) => Number.isFinite(x))
    if (!healthy)
      throw new Error(
        `the embedder produced a malformed vector (expected ${EMBED_DIM} finite floats)`,
      )
  })().catch((e: unknown) => {
    if (!_selfTestReported) {
      _selfTestReported = true
      reportEmbedStatus({
        kind: 'error',
        message: `embedding self-test failed: ${e instanceof Error ? e.message : String(e)}`,
      })
    }
    throw e
  }))
}

/** Embed a batch to normalised 384-float vectors (one worker round-trip; cosine == dot product afterwards). */
export async function embedMany(texts: string[]): Promise<Float32Array[]> {
  if (texts.length === 0) return []
  await selfTest()
  return roundTrip(texts)
}

/** Embed one text to a normalised 384-float vector. */
export async function embed(text: string): Promise<Float32Array> {
  await selfTest()
  return (await roundTrip([text]))[0]
}

/** This host's Embedder port: the worker round-trip behind the core interface. */
export const browserEmbedder: Embedder = { dim: EMBED_DIM, embedMany }
