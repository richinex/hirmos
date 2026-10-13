import { loadEnv, type Plugin } from 'vite'

export function localOpenRouter(): Plugin {
  return {
    name: 'hirmos:local-openrouter',
    apply: 'serve',
    configureServer(server) {
      server.middlewares.use((request, response, next) => {
        if (request.url !== '/__hirmos/local-openrouter-key') return next()
        response.setHeader('Cache-Control', 'no-store')
        response.setHeader('Content-Type', 'application/json')
        const host = request.headers.host ?? ''
        const localHost = /^(localhost|127\.0\.0\.1|\[::1\]):5179$/.test(host)
        const localPeer = ['127.0.0.1', '::1', '::ffff:127.0.0.1'].includes(
          request.socket.remoteAddress ?? '',
        )
        if (
          !localHost ||
          !localPeer ||
          request.method !== 'POST' ||
          request.headers.origin !== `http://${host}` ||
          request.headers['sec-fetch-site'] !== 'same-origin' ||
          request.headers['x-hirmos-local-key'] !== 'load'
        ) {
          response.statusCode = 403
          response.end(JSON.stringify({ error: 'Local browser access required.' }))
          return
        }
        try {
          const key = loadEnv(
            server.config.mode,
            server.config.envDir,
            'OPENROUTER_',
          ).OPENROUTER_API_KEY
          if (!key?.trim()) {
            response.statusCode = 404
            response.end(JSON.stringify({ error: 'No local OpenRouter key is configured.' }))
            return
          }
          response.end(JSON.stringify({ key }))
        } catch {
          response.statusCode = 500
          response.end(JSON.stringify({ error: 'The local key could not be loaded.' }))
        }
      })
    },
  }
}
