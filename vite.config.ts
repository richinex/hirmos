import { createReadStream, existsSync, readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath, URL } from 'node:url'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig, type Plugin } from 'vite'

/** Serve every client-side workbench route from the single document entry in development and preview. */
const appRoute = (): Plugin => {
  const rewrite = (request: { url?: string }) => {
    if (request.url === undefined) return
    const url = new URL(request.url, 'http://hirmos.local')
    if (url.pathname === '/app' || url.pathname.startsWith('/app/')) request.url = `/index.html${url.search}`
  }
  return {
    name: 'hirmos:app-route',
    configureServer(server) {
      server.middlewares.use((request, _response, next) => { rewrite(request); next() })
    },
    configurePreviewServer(server) {
      server.middlewares.use((request, _response, next) => { rewrite(request); next() })
    },
  }
}

/**
 * Serve the DuckDB engine binaries at /duckdb/<version>/<file> from node_modules in development and
 * preview. The deployment answers the same path from R2 (functions/duckdb/[[path]].js) because the
 * files exceed the Pages 25 MiB limit, so they are not part of the build.
 */
const duckdbBinaries = (): Plugin => {
  const dist = dirname(createRequire(import.meta.url).resolve('@duckdb/duckdb-wasm'))
  const { version } = JSON.parse(readFileSync(join(dist, '..', 'package.json'), 'utf8')) as { version: string }
  const serve = (request: { url?: string }, response: { setHeader: (name: string, value: string) => void; statusCode: number; end: (body?: string) => void; write: (chunk: unknown) => boolean; on: (event: string, listener: () => void) => void }, next: () => void) => {
    const match = /^\/duckdb\/([^/]+)\/(duckdb-(?:eh|mvp)\.wasm)$/.exec(new URL(request.url ?? '/', 'http://hirmos.local').pathname)
    if (match === null) { next(); return }
    const file = join(dist, match[2])
    if (match[1] !== version || !existsSync(file)) { response.statusCode = 404; response.end('Not found'); return }
    response.setHeader('content-type', 'application/wasm')
    createReadStream(file).pipe(response as unknown as NodeJS.WritableStream)
  }
  return {
    name: 'hirmos:duckdb-binaries',
    configureServer(server) { server.middlewares.use(serve) },
    configurePreviewServer(server) { server.middlewares.use(serve) },
  }
}

export default defineConfig({
  define: { __APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? '0.0.0') },
  plugins: [react(), tailwindcss(), appRoute(), duckdbBinaries()],
  resolve: { alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) } },
  server: {
    port: 5179,
    // Tailwind's plugin reloads the page whenever a file it scans changes; keep non-app files out of the watcher.
    watch: { ignored: ['**/tests/**', '**/docs/**', '**/test-results/**', '**/playwright-report/**', '**/*.md'] },
  },
})
