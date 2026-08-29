import { useMemo } from 'react'
import { useDocTheme } from '@/components/ui/useDocTheme'

/** The tokens a chart option is allowed to use, read once per theme from `<html>`; builders never touch `document`. */
export interface ChartTheme {
  readonly name: string
  readonly ink: string
  readonly bone: string
  readonly muted: string
  readonly faint: string
  readonly hair: string
  readonly line: string
  readonly panel: string
  readonly well: string
  readonly signal: string
  readonly info: string
  readonly ok: string
  readonly warn: string
  readonly danger: string
  /** Okabe–Ito categorical ramp, colour-blind safe, for series that mean categories. */
  readonly categorical: readonly string[]
  readonly font: string
  readonly mono: string
  readonly labelSize: number
  readonly bodySize: number
}

export const CATEGORICAL_RAMP = ['#0072B2', '#E69F00', '#009E73', '#CC79A7', '#56B4E9', '#D55E00', '#F0E442', '#BBBBBB'] as const

/** The same hues darkened in OKLCH until each reaches 3:1 on a white or paper panel; neighbours alternate light and dark. */
export const LIGHT_CATEGORICAL_RAMP = ['#0072B2', '#CC8600', '#009E73', '#B0609A', '#3D9DD1', '#D55E00', '#A49600', '#949494'] as const

const LIGHT_THEMES = new Set(['light', 'sketchbook', 'sketchbook-white'])

const FALLBACK: ChartTheme = {
  name: 'dark',
  ink: '#e6e3dc',
  bone: '#b8b5ae',
  muted: '#8c8a85',
  faint: '#817f78',
  hair: '#1e1e22',
  line: '#17171a',
  panel: '#0c0c0e',
  well: '#101012',
  signal: '#e4501f',
  info: '#4fa3da',
  ok: '#34d399',
  warn: '#e0a63c',
  danger: '#e5484d',
  categorical: CATEGORICAL_RAMP,
  font: '"Geist Variable", ui-sans-serif, system-ui, sans-serif',
  mono: '"Geist Mono Variable", ui-monospace, Menlo, monospace',
  labelSize: 11,
  bodySize: 12,
}

const lengthPixels = (root: CSSStyleDeclaration, name: string, fallback: number): number => {
  const value = root.getPropertyValue(name).trim()
  const rootPixels = Number.parseFloat(root.fontSize) || 16
  const numeric = Number.parseFloat(value)
  if (!Number.isFinite(numeric)) return fallback
  return value.endsWith('rem') ? numeric * rootPixels : numeric
}

/** Read the live palette. Called by the hook below, once per theme switch, never inside an option builder. */
export function readChartTheme(): ChartTheme {
  if (typeof document === 'undefined') return FALLBACK
  const root = getComputedStyle(document.documentElement)
  const colour = (name: string, fallback: string): string => root.getPropertyValue(name).trim() || fallback
  const name = document.documentElement.dataset.theme ?? FALLBACK.name
  return {
    name,
    ink: colour('--color-ink', FALLBACK.ink),
    bone: colour('--color-bone', FALLBACK.bone),
    muted: colour('--color-muted', FALLBACK.muted),
    faint: colour('--color-faint', FALLBACK.faint),
    hair: colour('--color-hair', FALLBACK.hair),
    line: colour('--color-line', FALLBACK.line),
    panel: colour('--color-panel', FALLBACK.panel),
    well: colour('--color-well', FALLBACK.well),
    signal: colour('--color-signal', FALLBACK.signal),
    info: colour('--color-info', FALLBACK.info),
    ok: colour('--color-ok', FALLBACK.ok),
    warn: colour('--color-warn', FALLBACK.warn),
    danger: colour('--color-danger', FALLBACK.danger),
    categorical: LIGHT_THEMES.has(name) ? LIGHT_CATEGORICAL_RAMP : CATEGORICAL_RAMP,
    font: FALLBACK.font,
    mono: FALLBACK.mono,
    labelSize: lengthPixels(root, '--text-label', FALLBACK.labelSize),
    bodySize: lengthPixels(root, '--text-body', FALLBACK.bodySize),
  }
}

/** The chart theme as a value that changes identity on every `data-theme` switch, so memoised options rebuild. */
export function useChartTheme(): ChartTheme {
  const name = useDocTheme()
  return useMemo(() => readChartTheme(), [name])
}
