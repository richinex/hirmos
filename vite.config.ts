import { spawnSync } from 'node:child_process'
import { fileURLToPath, URL } from 'node:url'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'
import { appPreload } from './vite-plugins/appPreload'
import { appRoute } from './vite-plugins/appRoute'
import { duckdbBinaries } from './vite-plugins/duckdbBinaries'
import { iconFont } from './vite-plugins/iconFont'
import { pyodideRuntime } from './vite-plugins/pyodideRuntime'
import { onnxRuntime } from './vite-plugins/onnxRuntime'
import { localOpenRouter } from './vite-plugins/localOpenRouter'

export default defineConfig({
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? '0.0.0'),
    __SOURCE_REVISION__: JSON.stringify(
      process.env.CF_PAGES_COMMIT_SHA ??
        (() => {
          const revision = spawnSync('git', ['rev-parse', '--verify', 'HEAD'], { encoding: 'utf8' })
          return revision.status === 0 ? revision.stdout.trim() : 'uncommitted'
        })(),
    ),
  },
  plugins: [
    localOpenRouter(),
    react(),
    tailwindcss(),
    appRoute(),
    appPreload(),
    iconFont(),
    duckdbBinaries(),
    pyodideRuntime(),
    onnxRuntime(),
  ],
  resolve: { alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) } },
  // WebCola is CommonJS reached only through a lazily loaded module, so the dev server would otherwise find it mid-session and serve a stale copy.
  optimizeDeps: { include: ['webcola/dist/src/layout', 'ai', '@openrouter/ai-sdk-provider'] },
  preview: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  server: {
    port: 5179,
    strictPort: true,
    // Tailwind's plugin reloads the page whenever a file it scans changes; keep non-app files out of the watcher.
    watch: {
      ignored: [
        '**/tests/**',
        '**/docs/**',
        '**/test-results/**',
        '**/playwright-report/**',
        '**/*.md',
        '**/.build/**',
      ],
    },
  },
})
