import { createReadStream, existsSync, readFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { middlewarePlugin, requestPath } from './middleware'

/**
 * Serve the DuckDB engine binary at /duckdb/<version>/<file> from the local engine build in development
 * and preview. The deployment answers the same path from R2 (functions/duckdb/[[path]].js) because the
 * file exceeds the Pages 25 MiB limit, so it is not part of the build. The engine is built by
 * scripts/build-duckdb-engine.sh, which writes the manifest beside the binary.
 */
export const duckdbBinaries = () => {
  const engine = resolve(import.meta.dirname, '..', '.build', 'engine')
  const manifest = join(engine, 'manifest.json')
  const version = existsSync(manifest)
    ? (JSON.parse(readFileSync(manifest, 'utf8')) as { version: string }).version
    : null
  return middlewarePlugin('hirmos:duckdb-binaries', (request, response, next) => {
    const match =
      /^\/duckdb\/([^/]+)\/(duckdb-(?:base|threads)\.wasm|duckdb-browser-(?:base|threads)(?:\.pthread)?\.worker\.js)$/.exec(
        requestPath(request),
      )
    if (match === null) return next()
    const file = join(engine, match[2])
    if (match[1] !== version || !existsSync(file)) {
      response.statusCode = 404
      response.end('Not found')
      return
    }
    response.setHeader(
      'content-type',
      match[2].endsWith('.wasm') ? 'application/wasm' : 'text/javascript',
    )
    response.setHeader('cross-origin-embedder-policy', 'require-corp')
    response.setHeader('cross-origin-resource-policy', 'same-origin')
    createReadStream(file).pipe(response)
  })
}
