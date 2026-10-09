import { middlewarePlugin } from './middleware'

/** Serve every client-side workbench route from the single document entry in development and preview. */
export const appRoute = () =>
  middlewarePlugin('hirmos:app-route', (request, _response, next) => {
    const url = new URL(request.url ?? '/', 'http://hirmos.local')
    if (url.pathname === '/app' || url.pathname.startsWith('/app/')) request.url = `/index.html${url.search}`
    next()
  })
