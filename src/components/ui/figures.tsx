import { useMemo, type ReactNode } from 'react'
import { Metadata } from './Metadata'
import { ParameterHelp } from './ParameterLabel'
import { fontFor, textWidth, useTextMetricsVersion, type TextTier } from '@/lib/textMetrics'
import { useElementWidth } from '@/lib/useElementWidth'
import { label as labelCn, num, well } from './recipes'
import {
  formatEstimate,
  formatInterval,
  noEffectValue,
  type EffectScale,
  type Formatted,
  type IntervalType,
} from '@/lib/format/number'
import { cn } from '@/lib/utils'

/**
 * The number vocabulary: a metric tile, the hero interval figure, and the refusal tile that takes a
 * result's place when the machine declines. Every digit comes through `src/lib/format/number.ts`.
 */

/** Renders a `Formatted` with its screen-reader form and per-part styling; every figure outside a chart should pass through here. */
export function FigureParts({
  value,
  unitClass = 'text-bone',
}: {
  readonly value: Formatted
  readonly unitClass?: string
}) {
  return (
    <>
      <span className="sr-only">{value.srText}</span>
      <span aria-hidden className={num()}>
        {value.parts.map((part, index) => {
          switch (part.kind) {
            case 'unit':
              return (
                <span key={index} className={cn('font-medium', unitClass)}>
                  {part.text}
                </span>
              )
            case 'qualifier':
              return (
                <span key={index} className="text-bone">
                  {part.text}
                </span>
              )
            case 'token':
              return (
                <span key={index} className="text-muted">
                  {part.text}
                </span>
              )
            default:
              return <span key={index}>{part.text}</span>
          }
        })}
      </span>
    </>
  )
}

export function MetricTile({
  label,
  value,
  context,
  help,
  size = 'default',
  frame = 'card',
  className,
}: {
  readonly label: string
  readonly value: Formatted
  readonly context?: ReactNode
  /** Background for the label, behind the info icon rather than on the tile. */
  readonly help?: string
  readonly size?: 'hero' | 'default' | 'compact'
  /** `cell` is for a value inside a group that supplies its surface. */
  readonly frame?: 'card' | 'cell'
  readonly className?: string
}) {
  const figure =
    size === 'hero' ? 'text-metric' : size === 'compact' ? 'text-title' : 'text-heading'
  const [values, width] = useElementWidth<HTMLDivElement>()
  const version = useTextMetricsVersion()
  // The figure size is for a value that sits on one line. One that would wrap there, words or several
  // figures joined, steps down a tier and wraps on one-unit lines; the width comes from pretext, not a reflow.
  const tier: TextTier = size === 'compact' ? 'title' : 'heading'
  const length = useMemo(() => {
    if (size === 'hero' || width === 0) return 'figure'
    void version
    return textWidth(value.text, fontFor(tier, 600)) <= width ? 'figure' : 'phrase'
  }, [size, width, value.text, tier, version])
  return (
    <div
      data-size={size}
      className={cn(
        '@container metric-tile',
        frame === 'card' ? 'metric-card' : 'bg-well',
        size === 'compact' ? 'px-3 py-2.5' : 'px-4 py-3',
        className,
      )}
    >
      {help === undefined ? (
        <span className={labelCn('metric-card-label block min-h-5 text-muted')}>{label}</span>
      ) : (
        <span className={labelCn('metric-card-label flex min-h-5 items-center gap-1.5 text-muted')}>
          {label}
          <ParameterHelp label={label} help={help} />
        </span>
      )}
      <div ref={values} className="metric-card-values">
        <p
          data-length={length}
          className={cn(
            'metric-card-number mb-0 mt-1 font-semibold leading-none tracking-tight text-ink',
            figure,
            '@max-[9rem]:text-title',
          )}
          title={value.exact || value.srText}
        >
          <FigureParts value={value} />
        </p>
        {context && (
          <p
            className={num(
              'metric-card-context mb-0 mt-1 text-body text-bone [overflow-wrap:anywhere]',
            )}
          >
            {context}
          </p>
        )}
      </div>
    </div>
  )
}

/** Related numeric summaries share one surface; lists retain their semantic wrapper. */
export function MetricGrid({
  children,
  as: Element = 'div',
  className,
  label,
  testId,
}: {
  readonly children: ReactNode
  readonly as?: 'div' | 'ul'
  readonly className?: string
  readonly label?: string
  readonly testId?: string
}) {
  return (
    <Element
      className={cn('numeric-surface metric-cards', className)}
      aria-label={label}
      data-testid={testId}
    >
      {children}
    </Element>
  )
}

/** The interval on its own axis with the no-effect value marked, so a reader sees whether it crosses it. */
function IntervalStrip({
  estimate,
  lower,
  upper,
  reference,
  accent,
}: {
  readonly estimate: number
  readonly lower: number
  readonly upper: number
  readonly reference: number
  readonly accent: boolean
}) {
  if (![estimate, lower, upper].every(Number.isFinite) || upper < lower) return null
  const low = Math.min(lower, reference)
  const high = Math.max(upper, reference)
  const pad = (high - low) * 0.06 || 1
  const at = (value: number) => `${((value - low + pad) / (high - low + 2 * pad)) * 100}%`

  return (
    <div aria-hidden className="relative mt-3 mb-5 h-5 w-full max-w-[26rem]">
      <span
        className="absolute inset-y-0 border-l border-dashed border-muted"
        style={{ left: at(reference) }}
      />
      <span
        className={labelCn('absolute top-full mt-0.5 -translate-x-1/2 text-faint')}
        style={{ left: at(reference) }}
      >
        {reference}
      </span>
      <span
        className={cn('absolute top-1/2 h-2 -translate-y-1/2', accent ? 'bg-signal-text' : 'bg-bone')}
        style={{ left: at(lower), width: `calc(${at(upper)} - ${at(lower)})` }}
      />
      <span
        className="absolute top-1/2 size-3.5 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-panel bg-ink"
        style={{ left: at(estimate) }}
      />
    </div>
  )
}

/** The hero estimate: the estimand sentence, the figure, its named interval, and the scale line. */
export function IntervalFigure({
  sentence,
  estimate,
  lower,
  upper,
  type,
  scale,
  standardError,
  sampleLine,
  scaleLine,
  accent = false,
  testId,
}: {
  readonly sentence: string
  readonly estimate: number
  readonly lower: number
  readonly upper: number
  readonly type: IntervalType
  readonly scale: EffectScale
  readonly standardError?: number
  readonly sampleLine: string
  readonly scaleLine: string
  /** Only the study's current accepted answer takes the signal colour. */
  readonly accent?: boolean
  readonly testId?: string
}) {
  const figure = formatInterval(estimate, lower, upper, type, scale)
  // The point uses the same precision as prose and history. Interval width only rounds the bounds.
  const pointFigure = formatEstimate(estimate, scale)
  const point = pointFigure.parts
  const se =
    standardError === undefined
      ? null
      : formatEstimate(standardError, scale, { precision: { kind: 'significant', digits: 2 } }).text
  return (
    <figure className="m-0" data-testid={testId}>
      <figcaption className="font-heading text-title leading-7 text-ink">{sentence}</figcaption>
      <p
        className={cn(
          'mb-0 mt-1 font-heading text-metric leading-none tracking-tight',
          accent ? 'text-signal-text' : 'text-ink',
        )}
        title={figure.exact}
      >
        <span className="sr-only">
          {pointFigure.srText}; {figure.srInterval}
        </span>
        <span aria-hidden className={num('font-heading')}>
          {point.map((part, index) =>
            part.kind === 'unit' ? (
              <span key={index} className="text-display-sub font-medium text-bone">
                {part.text}
              </span>
            ) : (
              <span key={index}>{part.text}</span>
            ),
          )}
        </span>
      </p>
      <IntervalStrip
        estimate={estimate}
        lower={lower}
        upper={upper}
        reference={noEffectValue(scale)}
        accent={accent}
      />
      <p aria-hidden className={num('mb-0 mt-1 text-body text-bone')}>
        <Metadata>
          <span>
            [{figure.bounds.lower}, {figure.bounds.upper}]{' '}
            <span className="text-ink">{figure.typeLabel}</span>
          </span>
          {se !== null && <span>SE {se}</span>}
          <span>{sampleLine}</span>
        </Metadata>
      </p>
      <p className={labelCn('mb-0 mt-2 text-muted')}>{scaleLine}</p>
    </figure>
  )
}

/** Takes the slot of a result when the machine declines: grey, the reason, the rule, and what to do. */
export function RefusalTile({
  label,
  headline,
  reason,
  rule,
  actions,
  testId,
}: {
  readonly label: string
  readonly headline: string
  readonly reason: ReactNode
  readonly rule: string
  readonly actions?: ReactNode
  readonly testId?: string
}) {
  return (
    <div className={well('px-4 py-3')} data-testid={testId} data-context={rule} role="status">
      <span className={labelCn('block text-muted')}>{label}</span>
      <p className="mb-0 mt-1 text-heading font-semibold text-bone">{headline}</p>
      <div className="mt-1 text-body text-bone">{reason}</div>
      {actions && <div className="mt-3 flex flex-wrap gap-2">{actions}</div>}
    </div>
  )
}
