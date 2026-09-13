/**
 * Serves the DuckDB engine binaries from the R2 bucket bound as DUCKDB, at /duckdb/<version>/<file>.
 * The Vite dev and preview servers answer the same path from node_modules, so the app has one URL.
 */
export async function onRequestGet({ env, params }) {
  const key = [].concat(params.path ?? []).join('/')
  if (!/^[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9]+(?:\.[A-Za-z0-9]+)*)?\/duckdb-(eh|mvp)\.wasm$/.test(key)) return new Response('Not found', { status: 404 })
  const object = await env.DUCKDB.get(key)
  if (object === null) return new Response('Not found', { status: 404 })
  const headers = new Headers()
  object.writeHttpMetadata(headers)
  headers.set('content-type', 'application/wasm')
  headers.set('etag', object.httpEtag)
  headers.set('cache-control', 'public, max-age=31536000, immutable')
  return new Response(object.body, { headers })
}
