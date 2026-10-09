import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { middlewarePlugin, requestPath } from './middleware'

const TYPES: Record<string, string> = {
  mjs: 'text/javascript',
  wasm: 'application/wasm',
  json: 'application/json',
  zip: 'application/zip',
  whl: 'application/octet-stream',
}

/**
 * Serve the Python runtime at /pyodide/<version>/<file> in development and preview by fetching it
 * from the Pyodide CDN. The deployment answers the same path from R2 (functions/pyodide/[[path]].js),
 * so the worker loads the runtime from the app's own origin in both.
 */
export const pyodideRuntime = () => {
  const cache = join(process.cwd(), 'node_modules', '.cache', 'pyodide')
  return middlewarePlugin('hirmos:pyodide-runtime', (request, response, next) => {
    const match = /^\/pyodide\/([0-9.]+)\/([A-Za-z0-9_.-]+\.(mjs|wasm|json|zip|whl))$/.exec(requestPath(request))
    if (match === null) return next()
    const [, version, file, extension] = match
    const cached = join(cache, version, file)
    const send = (body: Uint8Array) => {
      response.setHeader('content-type', TYPES[extension] ?? 'application/octet-stream')
      response.setHeader('cache-control', 'public, max-age=31536000, immutable')
      response.end(body)
    }
    if (existsSync(cached)) return send(readFileSync(cached))
    // Fetched once from the Pyodide CDN and kept under node_modules/.cache, so the runtime loads in dev as fast as it does from R2.
    void fetch(`https://cdn.jsdelivr.net/pyodide/v${version}/full/${file}`)
      .then(async (upstream) => {
        if (!upstream.ok) {
          response.statusCode = upstream.status
          response.end('Not found')
          return
        }
        const body = new Uint8Array(await upstream.arrayBuffer())
        mkdirSync(dirname(cached), { recursive: true })
        writeFileSync(cached, body)
        send(body)
      })
      .catch(() => {
        response.statusCode = 502
        response.end('The Pyodide CDN could not be reached')
      })
  })
}
