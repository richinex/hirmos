const RELOADED_AT = 'hirmos:preload-reloaded-at'
const RETRY_AFTER_MS = 60_000

export type PreloadRecovery =
  { readonly kind: 'reload' } | { readonly kind: 'surface'; readonly reloadedAgoMs: number }

/** One reload per minute: a stale tab is fixed by the first, a still-failing one is not hidden by a loop. */
export const decidePreloadRecovery = (
  lastReloadAt: number | null,
  now: number,
): PreloadRecovery => {
  if (lastReloadAt === null) return { kind: 'reload' }
  const reloadedAgoMs = now - lastReloadAt
  return reloadedAgoMs > RETRY_AFTER_MS ? { kind: 'reload' } : { kind: 'surface', reloadedAgoMs }
}

const lastReloadAt = (): number | null => {
  try {
    const stored = sessionStorage.getItem(RELOADED_AT)
    if (stored === null) return null
    const at = Number(stored)
    return Number.isFinite(at) ? at : null
  } catch {
    return null
  }
}

const rememberReload = (now: number): void => {
  try {
    sessionStorage.setItem(RELOADED_AT, String(now))
  } catch {
    return
  }
}

/** A tab that loaded before a deploy asks for chunks that no longer exist; a reload fetches the current bundle. */
export function recoverFromStalePreload(): void {
  window.addEventListener('vite:preloadError', (event) => {
    const now = Date.now()
    const decision = decidePreloadRecovery(lastReloadAt(), now)
    switch (decision.kind) {
      case 'reload':
        event.preventDefault()
        rememberReload(now)
        window.location.reload()
        return
      case 'surface':
        return
    }
  })
}
