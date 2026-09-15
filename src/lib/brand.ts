/** Shared geometry for the home mark and the themed favicon. */
export const MARK_PATHS = ['M7 7h5v7h6v-4l8 6-8 6v-4h-6v7H7z', 'M21 7h4v5l-4-3zm0 16 4-3v5h-4z'] as const

export function updateFavicon(): void {
  const css = getComputedStyle(document.documentElement)
  const ground = css.getPropertyValue('--color-panel').trim()
  const ink = css.getPropertyValue('--color-signal').trim()
  const link = document.querySelector<HTMLLinkElement>('link[rel="icon"]')
  if (!link || !ground || !ink) return
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><style>.mark{animation:direction 4s ease-in-out infinite}@keyframes direction{0%,65%,100%{transform:translateX(0)}28%{transform:translateX(-2px)}45%{transform:translateX(1px)}}@media(prefers-reduced-motion:reduce){.mark{animation:none}}</style><rect width="32" height="32" rx="7" fill="${ground}"/><g class="mark" fill="${ink}">${MARK_PATHS.map(d => `<path d="${d}"/>`).join('')}</g></svg>`
  const replacement = link.cloneNode() as HTMLLinkElement
  replacement.href = `data:image/svg+xml,${encodeURIComponent(svg)}`
  link.replaceWith(replacement)
}
