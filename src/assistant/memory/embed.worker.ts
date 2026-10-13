/// <reference lib="webworker" />
// Text embeddings in a dedicated module worker (the same split as whisper.worker.ts): MiniLM inference off
// the main thread, so indexing a big ingest never janks the UI — a 664-node compile used to saturate the
// main thread for tens of seconds in half-second slices. WebGPU when the browser has it, WASM otherwise.
// Input: a batch of texts. Output: one flat Float32Array (batch × dim), transferred, not copied.
import { env, pipeline } from '@huggingface/transformers'
import { embeddingBackend } from './embeddingBackend'

const MODEL = 'Xenova/all-MiniLM-L6-v2'

const onnx = env.backends.onnx
const cdnPaths = onnx.wasm?.wasmPaths
if (onnx.wasm && onnx.versions?.web && typeof cdnPaths === 'object') {
  const local = (url: string | URL | undefined) =>
    new URL(`/onnx/${onnx.versions!.web}/${String(url).split('/').at(-1)}`, self.location.origin).href
  onnx.wasm.wasmPaths = { mjs: local(cdnPaths.mjs), wasm: local(cdnPaths.wasm) }
}

interface ExtractOutput {
  data: Float32Array
  dims: number[]
}
type ExtractPipe = (
  texts: string[],
  opts: { pooling: 'mean'; normalize: boolean },
) => Promise<ExtractOutput>

let _pipe: Promise<ExtractPipe> | null = null
function getPipe(): Promise<ExtractPipe> {
  return (_pipe ??= (async () => {
    const { device, dtype } = await embeddingBackend(
      (navigator as { gpu?: { requestAdapter(): Promise<unknown | null> } }).gpu,
    )
    const p = await pipeline('feature-extraction', MODEL, { dtype, device } as never)
    return p as unknown as ExtractPipe
  })())
}

interface InMsg {
  id: number
  texts: string[]
}
type OutMsg =
  | { id: number; ok: true; dim: number; flat: Float32Array }
  | { id: number; ok: false; error: string }

self.onmessage = async (e: MessageEvent<InMsg>) => {
  const { id, texts } = e.data
  try {
    const pipe = await getPipe()
    const out = await pipe(texts, { pooling: 'mean', normalize: true })
    const dim = out.dims[out.dims.length - 1]
    const flat = Float32Array.from(out.data)
    const msg: OutMsg = { id, ok: true, dim, flat }
    self.postMessage(msg, { transfer: [flat.buffer] })
  } catch (err) {
    const msg: OutMsg = { id, ok: false, error: err instanceof Error ? err.message : String(err) }
    self.postMessage(msg)
  }
}
