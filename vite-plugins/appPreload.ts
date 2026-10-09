import type { Plugin } from 'vite'

export const appPreload = (): Plugin => ({
  name: 'hirmos:app-preload',
  transformIndexHtml: {
    order: 'post',
    handler(_html, { bundle }) {
      if (bundle === undefined) return
      const chunks = Object.values(bundle).filter((output) => output.type === 'chunk')
      const app = chunks.find((chunk) => chunk.moduleIds.some((id) => id.endsWith('/src/App.tsx')))
      if (app === undefined) throw new Error('The App chunk is missing from the bundle.')
      const loaded = new Set(chunks.filter((chunk) => chunk.isEntry).flatMap((chunk) => [chunk.fileName, ...chunk.imports]))
      const files = [app.fileName, ...app.imports].filter((file) => !loaded.has(file)).map((file) => `/${file}`)
      return [
        {
          tag: 'script',
          injectTo: 'head-prepend',
          children: `if (location.pathname.startsWith('/app')) for (const href of ${JSON.stringify(files)}) document.head.append(Object.assign(document.createElement('link'), { rel: 'modulepreload', href }))`,
        },
      ]
    },
  },
})
