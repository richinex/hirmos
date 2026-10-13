/**
 * Serves the ONNX Runtime files the embedding worker loads from the R2 bucket bound as DUCKDB, at
 * /onnx/<version>/<file>. The Vite dev and preview servers answer the same path from node_modules,
 * so the app has one URL.
 */
const TYPES = { mjs: 'text/javascript', wasm: 'application/wasm' }

export async function onRequestGet({ env, params }) {
  const key = [].concat(params.path ?? []).join('/')
  const match = /^[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.-]+)?\/ort-wasm-simd-threaded(?:\.asyncify)?\.(mjs|wasm)$/.exec(key)
  if (match === null) return new Response('Not found', { status: 404 })
  const object = await env.DUCKDB.get(`onnx/${key}`)
  if (object === null) return new Response('Not found', { status: 404 })
  const headers = new Headers()
  object.writeHttpMetadata(headers)
  headers.set('content-type', TYPES[match[1]])
  headers.set('etag', object.httpEtag)
  headers.set('cache-control', 'public, max-age=31536000, immutable')
  return new Response(object.body, { headers })
}
