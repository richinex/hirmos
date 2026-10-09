import { execFileSync } from 'node:child_process'
import { fileURLToPath, URL } from 'node:url'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'
import { appPreload } from './vite-plugins/appPreload'
import { appRoute } from './vite-plugins/appRoute'
import { duckdbBinaries } from './vite-plugins/duckdbBinaries'
import { iconFont } from './vite-plugins/iconFont'
import { pyodideRuntime } from './vite-plugins/pyodideRuntime'

export default defineConfig({
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? '0.0.0'),
    __SOURCE_REVISION__: JSON.stringify(
      process.env.CF_PAGES_COMMIT_SHA ?? execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
    ),
  },
  plugins: [react(), tailwindcss(), appRoute(), appPreload(), iconFont(), duckdbBinaries(), pyodideRuntime()],
  resolve: { alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) } },
  // WebCola is CommonJS reached only through a lazily loaded module, so the dev server would otherwise find it mid-session and serve a stale copy.
  optimizeDeps: { include: ['webcola/dist/src/layout'] },
  server: {
    port: 5179,
    // Tailwind's plugin reloads the page whenever a file it scans changes; keep non-app files out of the watcher.
    watch: { ignored: ['**/tests/**', '**/docs/**', '**/test-results/**', '**/playwright-report/**', '**/*.md'] },
  },
})
