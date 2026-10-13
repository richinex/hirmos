import { createReadStream, existsSync, readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import type { Plugin } from 'vite'
import { middlewarePlugin, requestPath } from './middleware'

const RUNTIME_FILE = /ort-wasm-simd-threaded[^/]*\.wasm$/

/**
 * Serve the ONNX Runtime files the embedding worker loads at /onnx/<version>/<file> from node_modules in
 * development and preview. The deployment answers the same path from R2 (functions/onnx/[[path]].js)
 * because the asyncify wasm exceeds the Pages 25 MiB limit, so the build drops the copies Vite emits.
 */
export const onnxRuntime = (): Plugin => {
  const dist = dirname(createRequire(import.meta.url).resolve('onnxruntime-web'))
  const { version } = JSON.parse(readFileSync(join(dist, '..', 'package.json'), 'utf8')) as { version: string }
  const server = middlewarePlugin('hirmos:onnx-runtime', (request, response, next) => {
    const match = /^\/onnx\/([^/]+)\/(ort-wasm-simd-threaded(?:\.asyncify)?\.(mjs|wasm))$/.exec(requestPath(request))
    if (match === null) return next()
    const file = join(dist, match[2])
    if (match[1] !== version || !existsSync(file)) {
      response.statusCode = 404
      response.end('Not found')
      return
    }
    response.setHeader('content-type', match[3] === 'wasm' ? 'application/wasm' : 'text/javascript')
    createReadStream(file).pipe(response)
  })
  return {
    ...server,
    generateBundle(_, bundle) {
      for (const name of Object.keys(bundle)) if (RUNTIME_FILE.test(name)) delete bundle[name]
    },
  }
}
