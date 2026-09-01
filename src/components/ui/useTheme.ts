import { useEffect, useState, useSyncExternalStore } from 'react'

export type ThemeName = 'dark' | 'operational' | 'light' | 'sketchbook' | 'sketchbook-white'

/** What the user picked: a concrete theme, or 'system' (follow the OS, live). */
export type ThemeChoice = ThemeName | 'system'

const KEY = 'hirmos-theme'
export const THEMES: readonly ThemeName[] = ['dark', 'operational', 'light', 'sketchbook', 'sketchbook-white']
const CYCLE: readonly ThemeChoice[] = [...THEMES, 'system']

const isTheme = (value: unknown): value is ThemeName => THEMES.includes(value as ThemeName)

/** The OS preference resolves to the two default faces; the boot script in index.html applies the same rule. */
const systemTheme = (): ThemeName =>
  window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'sketchbook-white'

const subscribeOsTheme = (notify: () => void): (() => void) => {
  const query = window.matchMedia?.('(prefers-color-scheme: dark)')
  if (!query) return () => {}
  query.addEventListener('change', notify)
  return () => query.removeEventListener('change', notify)
}

/**
 * The theme lives on `<html data-theme>`, which index.css keys off. The stored choice may be 'system', in
 * which case a media-query listener re-resolves on change; `data-theme` always carries a concrete theme.
 */
export function useTheme(): {
  readonly theme: ThemeName
  readonly choice: ThemeChoice
  readonly next: ThemeChoice
  readonly setChoice: (choice: ThemeChoice) => void
  readonly cycle: () => void
} {
  const [choice, setChoice] = useState<ThemeChoice>(() => {
    try {
      const stored = localStorage.getItem(KEY)
      if (stored === 'system' || isTheme(stored)) return stored
    } catch { /* private mode */ }
    return 'system'
  })
  const osTheme = useSyncExternalStore(subscribeOsTheme, systemTheme, () => 'dark' as ThemeName)
  const theme: ThemeName = choice === 'system' ? osTheme : choice

  useEffect(() => {
    const root = document.documentElement
    // A theme change rewrites every custom property at once; anything with `transition-colors` would ease
    // over 150ms and arrive late. Transitions are suppressed for one frame while the palette commits.
    root.dataset.themeSwitching = ''
    root.dataset.theme = theme
    void root.offsetHeight
    const frame = requestAnimationFrame(() => { delete root.dataset.themeSwitching })
    try { localStorage.setItem(KEY, choice) } catch { /* ignore */ }
    return () => cancelAnimationFrame(frame)
  }, [theme, choice])

  const next = CYCLE[(CYCLE.indexOf(choice) + 1) % CYCLE.length]
  return {
    theme,
    choice,
    next,
    setChoice,
    cycle: () => setChoice((current) => CYCLE[(CYCLE.indexOf(current) + 1) % CYCLE.length]),
  }
}
