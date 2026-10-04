import { assertNever } from '@/domain/dop'

/**
 * Every figure the UI shows passes through here. Rounding happens in this module and nowhere else, and
 * each helper returns the rendered text beside a clipboard form and a screen-reader form.
 */

export type NumberPart =
  | { readonly kind: 'sign'; readonly text: '−' | '+' | '' }
  | { readonly kind: 'digits'; readonly text: string }
  | { readonly kind: 'unit'; readonly text: string }
  | { readonly kind: 'qualifier'; readonly text: string }
  | { readonly kind: 'token'; readonly text: string; readonly srText: string }

export interface Formatted {
  /** What is rendered: U+2212 minus, grouped digits, attached or thin-spaced unit. */
  readonly text: string
  readonly parts: readonly NumberPart[]
  /** Clipboard form: ASCII minus, no grouping, full precision. */
  readonly exact: string
  /** Screen-reader form with expanded units. */
  readonly srText: string
}

export type Precision =
  | { readonly kind: 'significant'; readonly digits: 2 | 3 }
  | { readonly kind: 'decimals'; readonly places: 0 | 1 | 2 | 3 | 4 }
  /** Ratio scales: 2 significant figures when the leading digit is 4 or more, otherwise 3. */
  | { readonly kind: 'ruleOfFour' }
  /** Decimals chosen so the standard error shows one or two significant figures. */
  | { readonly kind: 'matchSe'; readonly se: number }

export type EffectScale =
  | { readonly kind: 'additive'; readonly unit: string }
  | { readonly kind: 'probabilityDifference' }
  | { readonly kind: 'ratio'; readonly label: 'RR' | 'OR' | 'ECR' | 'IRR' | 'HR' }
  | { readonly kind: 'logRatio'; readonly label: 'log-odds' | 'log-RR' }
  | { readonly kind: 'elasticity' }

export type IntervalType =
  | { readonly kind: 'confidence'; readonly level: number }
  | { readonly kind: 'credible'; readonly level: number; readonly summary: 'ETI' | 'HDI' }

export type AbsenceReason = 'notRun' | 'notApplicable' | 'notIdentified' | 'refused' | 'unavailable'

const THIN_SPACE = ' '
const MINUS = '−'

/** Display and screen-reader text share one locale: the prose is English, so the figures are too. */
const defaultLocale = (): string => 'en-GB'

const grouped = (value: number, locale: string, options: Intl.NumberFormatOptions): string =>
  new Intl.NumberFormat(locale, { useGrouping: true, ...options }).format(Math.abs(value))

const signPart = (value: number, showPlus = false): NumberPart => ({
  kind: 'sign',
  text: value < 0 ? MINUS : showPlus && value > 0 ? '+' : '',
})

const srSign = (value: number, showPlus = false): string => value < 0 ? 'minus ' : showPlus && value > 0 ? 'plus ' : ''

const assemble = (parts: readonly NumberPart[], exact: string, srText: string): Formatted => ({
  text: parts.map((part) => part.text).join(''),
  parts,
  exact,
  srText,
})

const decimalsFor = (value: number, precision: Precision): number => {
  switch (precision.kind) {
    case 'decimals': return precision.places
    case 'significant': return significantDecimals(value, precision.digits)
    case 'ruleOfFour': {
      const magnitude = Math.abs(value)
      const leading = magnitude === 0 || !Number.isFinite(magnitude) ? 1 : Math.floor(magnitude / 10 ** Math.floor(Math.log10(magnitude)))
      return significantDecimals(value, leading >= 4 ? 2 : 3)
    }
    case 'matchSe': {
      const se = Math.abs(precision.se)
      if (!Number.isFinite(se) || se === 0) return significantDecimals(value, 3)
      const magnitude = Math.floor(Math.log10(se))
      return Math.max(0, Math.min(4, 1 - magnitude))
    }
    default: return assertNever(precision)
  }
}

const significantDecimals = (value: number, digits: number): number => {
  if (value === 0 || !Number.isFinite(value)) return 0
  const magnitude = Math.floor(Math.log10(Math.abs(value)))
  const decimals = Math.max(0, Math.min(6, digits - 1 - magnitude))
  // Rounding can carry into the next power of ten (0.009998 to 0.0100); that figure needs one decimal fewer.
  return decimals > 0 && Number(Math.abs(value).toFixed(decimals)) >= 10 ** (magnitude + 1) ? decimals - 1 : decimals
}

/** A value that rounds to zero at the shown precision carries no sign: never "−0.00". */
const settled = (value: number, decimals: number): number => (Math.abs(value) < 0.5 * 10 ** -decimals ? 0 : value)

const digits = (value: number, decimals: number, locale: string): string =>
  grouped(value, locale, { minimumFractionDigits: decimals, maximumFractionDigits: decimals })

const exactText = (value: number): string => String(value)

const scaleUnit = (scale: EffectScale): { readonly text: string; readonly attached: boolean; readonly sr: string } => {
  switch (scale.kind) {
    case 'additive': return { text: scale.unit, attached: scale.unit === '%', sr: scale.unit }
    case 'probabilityDifference': return { text: 'pp', attached: false, sr: 'percentage points' }
    case 'ratio': return { text: '', attached: true, sr: '' }
    case 'logRatio': return { text: scale.label, attached: false, sr: scale.label.replace('-', ' ') }
    case 'elasticity': return { text: '', attached: true, sr: '' }
    default: return assertNever(scale)
  }
}

const scalePrefix = (scale: EffectScale): string => {
  switch (scale.kind) {
    case 'ratio': return { RR: 'risk ratio ', OR: 'odds ratio ', ECR: 'expected-count ratio ', IRR: 'incidence rate ratio ', HR: 'hazard ratio ' }[scale.label]
    case 'elasticity': return 'elasticity '
    case 'additive':
    case 'probabilityDifference':
    case 'logRatio': return ''
    default: return assertNever(scale)
  }
}

const defaultPrecision = (scale: EffectScale): Precision => scale.kind === 'ratio' ? { kind: 'ruleOfFour' } : { kind: 'significant', digits: 3 }

/** Probability differences render in percentage points; every other scale renders the raw value. */
const displayValue = (value: number, scale: EffectScale): number => scale.kind === 'probabilityDifference' ? value * 100 : value

const unitParts = (unit: { readonly text: string; readonly attached: boolean }): readonly NumberPart[] =>
  unit.text.length === 0 ? [] : [{ kind: 'unit', text: `${unit.attached ? '' : THIN_SPACE}${unit.text}` }]

/** Point estimate with scale-aware precision. Never emits '+'. */
export function formatEstimate(
  value: number,
  scale: EffectScale,
  opts: { readonly precision?: Precision; readonly locale?: string } = {},
): Formatted {
  if (!Number.isFinite(value)) return formatAbsent('unavailable')
  const locale = opts.locale ?? defaultLocale()
  const shown = displayValue(value, scale)
  const decimals = decimalsFor(shown, opts.precision ?? defaultPrecision(scale))
  const unit = scaleUnit(scale)
  const signed = settled(shown, decimals)
  const parts: NumberPart[] = [signPart(signed), { kind: 'digits', text: digits(shown, decimals, locale) }, ...unitParts(unit)]
  const srUnit = unit.sr.length > 0 ? ` ${unit.sr}` : ''
  return assemble(parts, exactText(value), `${scalePrefix(scale)}${srSign(signed)}${digits(shown, decimals, 'en-GB')}${srUnit}`.trim())
}

/** Estimate and typed interval rounded together to one decimal count; the interval type is never dropped. */
export function formatInterval(
  estimate: number,
  lower: number,
  upper: number,
  type: IntervalType,
  scale: EffectScale,
  opts: { readonly precision?: Precision; readonly locale?: string } = {},
): Formatted & { readonly bounds: { readonly lower: string; readonly upper: string }; readonly typeLabel: string } {
  const locale = opts.locale ?? defaultLocale()
  const shown = displayValue(estimate, scale)
  // Without an explicit precision the decimals follow the interval's width, so the bounds show one or two
  // significant figures of uncertainty rather than the point estimate's own digits.
  const halfWidth = displayValue(Math.abs(upper - lower), scale) / 3.92
  const precision: Precision = opts.precision ?? (Number.isFinite(halfWidth) && halfWidth > 0 ? { kind: 'matchSe', se: halfWidth } : defaultPrecision(scale))
  const decimals = decimalsFor(shown, precision)
  const unit = scaleUnit(scale)
  const bound = (value: number): string => {
    const shownBound = settled(displayValue(value, scale), decimals)
    return `${shownBound < 0 ? MINUS : ''}${digits(shownBound, decimals, locale)}`
  }
  const signed = settled(shown, decimals)
  const level = `${Math.round(type.level * 100)}%`
  const typeLabel = type.kind === 'confidence' ? `${level} CI` : `${level} ${type.summary === 'HDI' ? 'HDI' : 'CrI'}`
  const bounds = { lower: bound(lower), upper: bound(upper) }
  const parts: NumberPart[] = [
    signPart(signed),
    { kind: 'digits', text: digits(shown, decimals, locale) },
    ...unitParts(unit),
    { kind: 'qualifier', text: ` [${bounds.lower}, ${bounds.upper}] ${typeLabel}` },
  ]
  const srType = type.kind === 'confidence' ? `${level} confidence interval` : `${level} ${type.summary === 'HDI' ? 'highest density interval' : 'credible interval'}`
  const srBound = (value: number) => `${srSign(settled(displayValue(value, scale), decimals))}${digits(displayValue(value, scale), decimals, 'en-GB')}`
  const srUnit = unit.sr.length > 0 ? ` ${unit.sr}` : ''
  return {
    ...assemble(parts, `${exactText(estimate)} [${exactText(lower)}, ${exactText(upper)}]`, `${scalePrefix(scale)}${srSign(signed)}${digits(shown, decimals, 'en-GB')}${srUnit}, ${srType} ${srBound(lower)} to ${srBound(upper)}`.trim()),
    bounds,
    typeLabel,
  }
}

/** p-values: 3 decimals at or above 0.001, "< 0.001" below. Never "0.000". */
export function formatP(p: number, opts: { readonly locale?: string; readonly withLabel?: boolean } = {}): Formatted {
  if (!Number.isFinite(p)) return formatAbsent('notRun')
  const locale = opts.locale ?? defaultLocale()
  const withLabel = opts.withLabel ?? true
  const relation = p < 0.001 ? '<' : p >= 0.9995 ? '>' : '='
  const shown = p < 0.001 ? 0.001 : p >= 0.9995 ? 0.999 : p
  const text = digits(shown, 3, locale)
  const parts: NumberPart[] = withLabel
    ? [{ kind: 'qualifier', text: `p ${relation} ` }, { kind: 'digits', text }]
    : relation === '=' ? [{ kind: 'digits', text }] : [{ kind: 'qualifier', text: `${relation} ` }, { kind: 'digits', text }]
  const srRelation = relation === '<' ? 'less than' : relation === '>' ? 'greater than' : 'equals'
  return assemble(parts, exactText(p), `p ${srRelation} ${text}`)
}

/** Percentages from a proportion in [0, 1]; a denominator forces the fraction beside the figure. */
export function formatPercent(
  proportion: number,
  opts: { readonly numerator?: number; readonly denominator?: number; readonly precision?: 'auto' | 0 | 1 | 2; readonly locale?: string } = {},
): Formatted {
  if (!Number.isFinite(proportion)) return formatAbsent('unavailable')
  const locale = opts.locale ?? defaultLocale()
  const percent = proportion * 100
  const precision = opts.precision ?? 'auto'
  // Decide the decimals from the value as it will round, so 9.99 shows as 10% and 99.96 as 100%.
  const rounded = Math.round(percent * 10) / 10
  const decimals = precision === 'auto' ? (rounded > 0 && (rounded < 10 || rounded > 90) && rounded < 100 ? 1 : 0) : precision
  const floor = decimals === 0 ? 1 : 10 ** -decimals
  const tooSmall = percent > 0 && percent < floor / 2
  const parts: NumberPart[] = tooSmall
    ? [{ kind: 'qualifier', text: '<' }, { kind: 'digits', text: digits(floor, decimals, locale) }, { kind: 'unit', text: '%' }]
    : [{ kind: 'digits', text: digits(percent, decimals, locale) }, { kind: 'unit', text: '%' }]
  const denominator = opts.denominator
  const numerator = opts.numerator ?? (denominator === undefined ? undefined : Math.round(proportion * denominator))
  if (denominator !== undefined && numerator !== undefined) {
    parts.push({ kind: 'qualifier', text: ` (${grouped(numerator, locale, {})} of ${grouped(denominator, locale, {})})` })
  }
  const srBase = tooSmall ? `less than ${digits(floor, decimals, 'en-GB')} percent` : `${digits(percent, decimals, 'en-GB')} percent`
  const srFraction = denominator !== undefined && numerator !== undefined ? `, ${numerator} of ${denominator}` : ''
  return assemble(parts, exactText(proportion), `${srBase}${srFraction}`)
}

/** Percentage-point difference between two proportions, signed. */
export function formatPercentagePoints(before: number, after: number, opts: { readonly locale?: string } = {}): Formatted {
  const locale = opts.locale ?? defaultLocale()
  const delta = (after - before) * 100
  const decimals = Math.abs(delta) < 10 ? 1 : 0
  const signed = settled(delta, decimals)
  const parts: NumberPart[] = [signPart(signed, true), { kind: 'digits', text: digits(delta, decimals, locale) }, { kind: 'unit', text: `${THIN_SPACE}pp` }]
  return assemble(parts, exactText(after - before), `${srSign(signed, true)}${digits(delta, decimals, 'en-GB')} percentage points`)
}

/** Integer counts, grouped; compact form for constrained cells with the exact value beside it. */
export function formatCount(n: number, opts: { readonly compact?: boolean; readonly noun?: string; readonly locale?: string } = {}): Formatted {
  if (!Number.isFinite(n)) return formatAbsent('unavailable')
  const locale = opts.locale ?? defaultLocale()
  const text = opts.compact
    ? new Intl.NumberFormat(locale, { notation: 'compact', maximumFractionDigits: 1 }).format(n)
    : grouped(n, locale, { maximumFractionDigits: 0 })
  const signed = settled(n, 0)
  const parts: NumberPart[] = [signPart(signed), { kind: 'digits', text }, ...(opts.noun ? [{ kind: 'unit', text: ` ${opts.noun}` } as const] : [])]
  return assemble(parts, String(Math.round(n)), `${srSign(signed)}${grouped(n, 'en-GB', { maximumFractionDigits: 0 })}${opts.noun ? ` ${opts.noun}` : ''}`)
}

/** Stored digits as written, with a true minus sign: no grouping, so a year or an id reads as stored, and a wide integer kept as text keeps every digit. */
export const storedDigits = (text: string): string => text.startsWith('-') ? `${MINUS}${text.slice(1)}` : text

/** A stored number as the preview writes it: a whole number by storedDigits, a fraction as a raw statistic. */
export function formatStored(value: number): Formatted {
  if (!Number.isInteger(value)) return formatStatistic('raw', value)
  return assemble([{ kind: 'digits', text: storedDigits(String(value)) }], String(value), `${srSign(value)}${Math.abs(value)}`)
}

/** A number the reader set, as they would write it: 0.01 not 0.0100, free of binary noise. */
const asSet = (value: number): string => storedDigits(String(Number(value.toPrecision(12))))

/** A proportion the reader set, in percent as they would write it: 0.02 is 2 and 0.025 is 2.5. */
export const percentAsSet = (proportion: number): number => Number((proportion * 100).toPrecision(12))

/** A setting such as a seed, a trimming threshold or a correlation, written as set. */
export function formatSetting(value: number): Formatted {
  if (!Number.isFinite(value)) return formatAbsent('unavailable')
  const text = asSet(value)
  return assemble([{ kind: 'digits', text }], String(value), text.replace('\u2212', 'minus '))
}

/** A setting such as a confounding share or a confidence level: written as set, never padded to fixed decimals (2%, 2.5%, 95%). */
export function formatSetPercent(proportion: number): Formatted {
  if (!Number.isFinite(proportion)) return formatAbsent('unavailable')
  const text = `${asSet(proportion * 100)}%`
  return assemble([{ kind: 'digits', text }], String(proportion), text.replace('\u2212', 'minus '))
}

/** A chart axis tick: grouped as ECharts groups it, with a true minus, and free of binary noise (0.3, not 0.30000000000000004). */
export function formatAxisTick(value: number): string {
  if (!Number.isFinite(value)) return String(value)
  const clean = Number(value.toPrecision(12))
  return `${clean < 0 ? MINUS : ''}${grouped(clean, defaultLocale(), { maximumFractionDigits: 12 })}`
}

/** Durations from milliseconds; the unit follows the magnitude, one unit per figure. */
export function formatDuration(ms: number, opts: { readonly locale?: string } = {}): Formatted {
  if (!Number.isFinite(ms) || ms < 0) return formatAbsent('unavailable')
  const locale = opts.locale ?? defaultLocale()
  // Round once at the unit shown, then split, so a remainder never carries into "60 s" or "60 min".
  const wholeMs = Math.round(ms)
  const tenthSeconds = Math.round(ms / 100)
  const seconds = Math.round(ms / 1000)
  const minutes = Math.round(ms / 60_000)
  const text = wholeMs < 1000
    ? `${grouped(wholeMs, locale, { maximumFractionDigits: 0 })}${THIN_SPACE}ms`
    : tenthSeconds < 600
      ? `${grouped(tenthSeconds / 10, locale, { maximumFractionDigits: 1 })}${THIN_SPACE}s`
      : seconds < 3600
        ? `${Math.floor(seconds / 60)}${THIN_SPACE}min ${seconds % 60}${THIN_SPACE}s`
        : `${Math.floor(minutes / 60)}${THIN_SPACE}h ${minutes % 60}${THIN_SPACE}min`
  return assemble([{ kind: 'digits', text }], String(ms), text.replaceAll(THIN_SPACE, ' ').replace('ms', 'milliseconds').replace(/\bs\b/, 'seconds').replace('min', 'minutes').replace(/\bh\b/, 'hours'))
}

export type StatisticKind = 'adf' | 'kpss' | 'acf' | 'xcorr' | 'rhat' | 'ess' | 'gamma' | 'evalue' | 'stability' | 'score' | 'mean' | 'sd' | 'raw'

const STATISTIC_DECIMALS: Record<StatisticKind, number | 'significant'> = {
  adf: 1,
  kpss: 2,
  acf: 2,
  xcorr: 2,
  rhat: 2,
  ess: 0,
  gamma: 1,
  evalue: 1,
  stability: 2,
  score: 3,
  mean: 'significant',
  sd: 'significant',
  raw: 'significant',
}

/** Test statistics, diagnostics and summary statistics at their conventional precision. */
export function formatStatistic(kind: StatisticKind, value: number, opts: { readonly locale?: string } = {}): Formatted {
  if (!Number.isFinite(value)) return formatAbsent('unavailable')
  const locale = opts.locale ?? defaultLocale()
  const rule = STATISTIC_DECIMALS[kind]
  const decimals = rule === 'significant' ? (Number.isInteger(value) && Math.abs(value) >= 100 ? 0 : significantDecimals(value, 3)) : rule
  const signed = settled(value, decimals)
  const parts: NumberPart[] = [signPart(signed), { kind: 'digits', text: digits(value, decimals, locale) }]
  return assemble(parts, exactText(value), `${srSign(signed)}${digits(value, decimals, 'en-GB')}`)
}

/** Refusal and absence tokens. Always a token with screen-reader text; never an empty string. */
/** A words-only figure, for a tile whose value is a phrase rather than a number. */
export function formatWords(value: string): Formatted {
  return assemble([{ kind: 'digits', text: value }], value, value)
}

export function formatAbsent(reason: AbsenceReason, detail?: string): Formatted {
  const token = ((): { readonly text: string; readonly srText: string } => {
    switch (reason) {
      case 'notRun': return { text: 'not run', srText: 'not run' }
      case 'notApplicable': return { text: 'n/a', srText: 'not applicable' }
      case 'notIdentified': return { text: 'not identified', srText: 'not identified' }
      case 'refused': return { text: 'refused', srText: 'refused' }
      case 'unavailable': return { text: '—', srText: 'unavailable' }
      default: return assertNever(reason)
    }
  })()
  const srText = detail ? `${token.srText}: ${detail}` : token.srText
  return assemble([{ kind: 'token', text: token.text, srText }], '', srText)
}

/** A file size in the unit that keeps it to a few digits. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
