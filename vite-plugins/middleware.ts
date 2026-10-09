import type { Connect, Plugin } from 'vite'

/** A plugin that answers requests with one handler on both the dev and preview servers. */
export const middlewarePlugin = (name: string, handle: Connect.NextHandleFunction): Plugin => ({
  name,
  configureServer(server) {
    server.middlewares.use(handle)
  },
  configurePreviewServer(server) {
    server.middlewares.use(handle)
  },
})

export const requestPath = (request: Connect.IncomingMessage): string =>
  new URL(request.url ?? '/', 'http://hirmos.local').pathname
