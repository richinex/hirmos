/**
 * Serves the Python runtime from the R2 bucket bound as DUCKDB, at /pyodide/<version>/<file>: the
 * Pyodide loader, its wasm and standard library, the lock file, and the wheels for pandas and numpy.
 * The Vite dev server answers the same path from the Pyodide CDN, so the app has one URL.
 */
const TYPES = { mjs: 'text/javascript', wasm: 'application/wasm', json: 'application/json', zip: 'application/zip', whl: 'application/octet-stream' }

export async function onRequestGet({ env, params }) {
  const key = [].concat(params.path ?? []).join('/')
  const match = /^[0-9]+\.[0-9]+\.[0-9]+\/[A-Za-z0-9_.-]+\.(mjs|wasm|json|zip|whl)$/.exec(key)
  if (match === null) return new Response('Not found', { status: 404 })
  const object = await env.DUCKDB.get(`pyodide/${key}`)
  if (object === null) return new Response('Not found', { status: 404 })
  const headers = new Headers()
  object.writeHttpMetadata(headers)
  headers.set('content-type', TYPES[match[1]])
  headers.set('etag', object.httpEtag)
  headers.set('cache-control', 'public, max-age=31536000, immutable')
  return new Response(object.body, { headers })
}
