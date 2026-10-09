import { createReadStream, existsSync, readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { middlewarePlugin, requestPath } from './middleware'

/**
 * Serve the DuckDB engine binaries at /duckdb/<version>/<file> from node_modules in development and
 * preview. The deployment answers the same path from R2 (functions/duckdb/[[path]].js) because the
 * files exceed the Pages 25 MiB limit, so they are not part of the build.
 */
export const duckdbBinaries = () => {
  const dist = dirname(createRequire(import.meta.url).resolve('@duckdb/duckdb-wasm'))
  const { version } = JSON.parse(readFileSync(join(dist, '..', 'package.json'), 'utf8')) as { version: string }
  return middlewarePlugin('hirmos:duckdb-binaries', (request, response, next) => {
    const match = /^\/duckdb\/([^/]+)\/(duckdb-(?:eh|mvp)\.wasm)$/.exec(requestPath(request))
    if (match === null) return next()
    const file = join(dist, match[2])
    if (match[1] !== version || !existsSync(file)) {
      response.statusCode = 404
      response.end('Not found')
      return
    }
    response.setHeader('content-type', 'application/wasm')
    createReadStream(file).pipe(response)
  })
}
