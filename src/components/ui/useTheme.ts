import { useEffect, useState, useSyncExternalStore } from 'react'

export type ThemeName = 'light' | 'dark'

/** What the user picked: a concrete theme, or 'system' (follow the OS, live). */
export type ThemeChoice = ThemeName | 'system'

const KEY = 'hirmos-theme'
export const THEMES: readonly ThemeName[] = ['light', 'dark']
const CYCLE: readonly ThemeChoice[] = [...THEMES, 'system']
export const THEME_LABELS: Record<ThemeChoice, string> = { light: 'Light', dark: 'Dark', system: 'System' }

const isTheme = (value: unknown): value is ThemeName => THEMES.includes(value as ThemeName)

/** The OS preference resolves to one of the two faces; the boot script in index.html applies the same rule. */
const systemTheme = (): ThemeName =>
  window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'

const subscribeOsTheme = (notify: () => void): (() => void) => {
  const query = window.matchMedia?.('(prefers-color-scheme: dark)')
  if (!query) return () => {}
  query.addEventListener('change', notify)
  return () => query.removeEventListener('change', notify)
}

/**
 * The theme lives on `<html data-theme>`, which themes.css keys off. Light is the default; the stored
 * choice may be 'system', in which case a media-query listener re-resolves on change. `data-theme`
 * always carries a concrete theme.
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
    return 'light'
  })
  const osTheme = useSyncExternalStore(subscribeOsTheme, systemTheme, () => 'light' as ThemeName)
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

  return {
    theme,
    choice,
    next: CYCLE[(CYCLE.indexOf(choice) + 1) % CYCLE.length],
    setChoice,
    cycle: () => setChoice(current => CYCLE[(CYCLE.indexOf(current) + 1) % CYCLE.length]),
  }
}
