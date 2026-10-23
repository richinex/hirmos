import { mkdirSync, copyFileSync, writeFileSync, existsSync, readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import esbuild from 'esbuild'

const here = dirname(fileURLToPath(import.meta.url))
const features = process.argv[2] ?? 'base'
const pkg = resolve(here, '..', '..', '.build', 'duckdb-wasm', 'packages', 'duckdb-wasm')
const out = resolve(here, '..', '..', '.build', 'engine')
mkdirSync(out, { recursive: true })
writeFileSync(resolve(out, '.gitignore'), '*\n')

await esbuild.build({
  platform: 'browser',
  format: 'iife',
  target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
  bundle: true,
  minify: true,
  sourcemap: false,
  external: ['module', 'node:*'],
  define: { 'process.release.name': '"browser"' },
  absWorkingDir: pkg,
  nodePaths: [
    resolve(here, '..', '..', '.build', 'tools', 'node_modules'),
    resolve(here, '..', '..', 'node_modules'),
  ],
  entryPoints: [`./src/targets/duckdb-browser-${features}.worker.ts`],
  outfile: resolve(out, `duckdb-browser-${features}.worker.js`),
  globalName: 'duckdb',
})
if (features === 'threads') {
  await esbuild.build({
    platform: 'browser',
    format: 'iife',
    target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
    bundle: true,
    minify: true,
    sourcemap: false,
    external: ['module', 'node:*'],
    define: { 'process.release.name': '"browser"' },
    absWorkingDir: pkg,
    nodePaths: [
      resolve(here, '..', '..', '.build', 'tools', 'node_modules'),
      resolve(here, '..', '..', 'node_modules'),
    ],
    entryPoints: ['./src/targets/duckdb-browser-threads.worker.ts'],
    outfile: resolve(out, 'duckdb-browser-threads.pthread.worker.js'),
  })
}
copyFileSync(
  resolve(pkg, `src/bindings/duckdb-${features}.wasm`),
  resolve(out, `duckdb-${features}.wasm`),
)
const version = /DUCKDB_PACKAGE_VERSION = '([^']+)'/.exec(
  readFileSync(resolve(here, '..', '..', 'src', 'domain', 'dataEngine.ts'), 'utf8'),
)[1]
writeFileSync(resolve(out, 'manifest.json'), JSON.stringify({ version }, null, 2) + '\n')
const workerHome = resolve(here, '..', '..', 'src', 'data', 'engine')
mkdirSync(workerHome, { recursive: true })
copyFileSync(
  resolve(out, `duckdb-browser-${features}.worker.js`),
  resolve(workerHome, `duckdb-browser-${features}.worker.js`),
)
if (features === 'threads')
  copyFileSync(
    resolve(out, 'duckdb-browser-threads.pthread.worker.js'),
    resolve(workerHome, 'duckdb-browser-threads.pthread.worker.js'),
  )
console.log('bundled into', out, 'as', version, 'and the worker into', workerHome)
