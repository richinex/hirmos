import { createReadStream, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { execFileSync } from 'node:child_process'
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

/**
 * Serve the Python runtime at /pyodide/<version>/<file> in development and preview by fetching it
 * from the Pyodide CDN. The deployment answers the same path from R2 (functions/pyodide/[[path]].js),
 * so the worker loads the runtime from the app's own origin in both.
 */
const pyodideRuntime = (): Plugin => {
  const cache = join(process.cwd(), 'node_modules', '.cache', 'pyodide')
  const types: Record<string, string> = { mjs: 'text/javascript', wasm: 'application/wasm', json: 'application/json', zip: 'application/zip', whl: 'application/octet-stream' }
  const serve = (request: { url?: string }, response: { setHeader: (name: string, value: string) => void; statusCode: number; end: (body?: Uint8Array | string) => void }, next: () => void) => {
    const match = /^\/pyodide\/([0-9.]+)\/([A-Za-z0-9_.-]+\.(mjs|wasm|json|zip|whl))$/.exec(new URL(request.url ?? '/', 'http://hirmos.local').pathname)
    if (match === null) { next(); return }
    const [, version, file, extension] = match
    const cached = join(cache, version, file)
    const send = (body: Uint8Array) => {
      response.setHeader('content-type', types[extension] ?? 'application/octet-stream')
      response.setHeader('cache-control', 'public, max-age=31536000, immutable')
      response.end(body)
    }
    if (existsSync(cached)) { send(readFileSync(cached)); return }
    // Fetched once from the Pyodide CDN and kept under node_modules/.cache, so the runtime loads in dev as fast as it does from R2.
    void fetch(`https://cdn.jsdelivr.net/pyodide/v${version}/full/${file}`).then(async (upstream) => {
      if (!upstream.ok) { response.statusCode = upstream.status; response.end('Not found'); return }
      const body = new Uint8Array(await upstream.arrayBuffer())
      mkdirSync(dirname(cached), { recursive: true })
      writeFileSync(cached, body)
      send(body)
    }).catch(() => { response.statusCode = 502; response.end('The Pyodide CDN could not be reached') })
  }
  return {
    name: 'hirmos:pyodide-runtime',
    configureServer(server) { server.middlewares.use(serve) },
    configurePreviewServer(server) { server.middlewares.use(serve) },
  }
}

export default defineConfig({
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? '0.0.0'),
    __SOURCE_REVISION__: JSON.stringify(
      process.env.CF_PAGES_COMMIT_SHA ?? execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
    ),
  },
  plugins: [react(), tailwindcss(), appRoute(), duckdbBinaries(), pyodideRuntime()],
  resolve: { alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) } },
  // WebCola is CommonJS reached only through a lazily loaded module, so the dev server would otherwise find it mid-session and serve a stale copy.
  optimizeDeps: { include: ['webcola/dist/src/layout'] },
  server: {
    port: 5179,
    // Tailwind's plugin reloads the page whenever a file it scans changes; keep non-app files out of the watcher.
    watch: { ignored: ['**/tests/**', '**/docs/**', '**/test-results/**', '**/playwright-report/**', '**/*.md'] },
  },
})
