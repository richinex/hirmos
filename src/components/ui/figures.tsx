import type { ReactNode } from 'react'
import { label as labelCn, num } from './recipes'
import { formatCount, formatEstimate, formatInterval, type EffectScale, type Formatted, type IntervalType } from '@/lib/format/number'
import { cn } from '@/lib/utils'

/**
 * The number vocabulary: a metric tile, the hero interval figure, and the refusal tile that takes a
 * result's place when the machine declines. Every digit comes through `src/lib/format/number.ts`.
 */

function Parts({ value, unitClass = 'text-bone' }: { readonly value: Formatted; readonly unitClass?: string }) {
  return (
    <>
      <span className="sr-only">{value.srText}</span>
      <span aria-hidden className={num()}>
        {value.parts.map((part, index) => {
          switch (part.kind) {
            case 'unit': return <span key={index} className={cn('font-medium', unitClass)}>{part.text}</span>
            case 'qualifier': return <span key={index} className="text-bone">{part.text}</span>
            case 'token': return <span key={index} className="text-muted">{part.text}</span>
            default: return <span key={index}>{part.text}</span>
          }
        })}
      </span>
    </>
  )
}

export function MetricTile({ label, value, context, size = 'default', className }: {
  readonly label: string
  readonly value: Formatted
  readonly context?: ReactNode
  readonly size?: 'hero' | 'default' | 'compact'
  readonly className?: string
}) {
  const figure = size === 'hero' ? 'text-metric' : 'text-heading'
  return (
    <div className={cn('rounded-lg border border-hair bg-well px-4 py-3', className)}>
      <span className={labelCn('block text-muted')}>{label}</span>
      <p className={cn('mb-0 mt-1 font-semibold text-ink', figure)} title={value.exact}><Parts value={value} /></p>
      {context && <p className={num('mb-0 mt-1 text-body text-bone')}>{context}</p>}
    </div>
  )
}

/** Hairline from lower to upper, the estimate dot, and a tick at the null value; no axis labels. */
function IntervalBar({ estimate, lower, upper, nullValue = 0, accent }: {
  readonly estimate: number
  readonly lower: number
  readonly upper: number
  readonly nullValue?: number
  readonly accent: boolean
}) {
  const min = Math.min(lower, nullValue)
  const max = Math.max(upper, nullValue)
  const span = max - min || 1
  const pad = span * 0.08
  const x = (value: number) => ((value - (min - pad)) / (span + 2 * pad)) * 100
  return (
    <svg aria-hidden viewBox="0 0 100 12" preserveAspectRatio="none" className="mt-2 block h-3 w-full max-w-md">
      <line x1={x(nullValue)} x2={x(nullValue)} y1="1" y2="11" stroke="var(--color-muted)" strokeWidth="0.4" strokeDasharray="1 1" vectorEffect="non-scaling-stroke" />
      <line x1={x(lower)} x2={x(upper)} y1="6" y2="6" stroke="var(--color-bone)" strokeWidth="1" vectorEffect="non-scaling-stroke" />
      <line x1={x(lower)} x2={x(lower)} y1="3.5" y2="8.5" stroke="var(--color-bone)" strokeWidth="1" vectorEffect="non-scaling-stroke" />
      <line x1={x(upper)} x2={x(upper)} y1="3.5" y2="8.5" stroke="var(--color-bone)" strokeWidth="1" vectorEffect="non-scaling-stroke" />
      <line x1={x(estimate)} x2={x(estimate)} y1="6" y2="6" stroke={accent ? 'var(--color-signal)' : 'var(--color-ink)'} strokeWidth="5" strokeLinecap="round" vectorEffect="non-scaling-stroke" />
    </svg>
  )
}

/** The hero estimate: the estimand sentence, the figure, its named interval, and the scale line. */
export function IntervalFigure({ sentence, estimate, lower, upper, type, scale, standardError, observations, scaleLine, accent = false, testId }: {
  readonly sentence: string
  readonly estimate: number
  readonly lower: number
  readonly upper: number
  readonly type: IntervalType
  readonly scale: EffectScale
  readonly standardError?: number
  readonly observations: number
  readonly scaleLine: string
  /** Only the study's current accepted answer takes the signal colour. */
  readonly accent?: boolean
  readonly testId?: string
}) {
  const figure = formatInterval(estimate, lower, upper, type, scale)
  const point = figure.parts.filter((part) => part.kind !== 'qualifier')
  const se = standardError === undefined ? null : formatEstimate(standardError, scale, { precision: { kind: 'significant', digits: 2 } }).text
  return (
    <figure className="m-0" data-testid={testId}>
      <figcaption className="text-title text-ink">{sentence}</figcaption>
      <p className={cn('mb-0 mt-1 text-metric font-semibold', accent ? 'text-signal' : 'text-ink')} title={figure.exact}>
        <span className="sr-only">{figure.srText}</span>
        <span aria-hidden className={num()}>
          {point.map((part, index) => part.kind === 'unit'
            ? <span key={index} className="text-display-sub font-medium text-bone">{part.text}</span>
            : <span key={index}>{part.text}</span>)}
        </span>
      </p>
      <p aria-hidden className={num('mb-0 mt-1 text-body text-bone')}>
        [{figure.bounds.lower}, {figure.bounds.upper}] <span className="text-ink">{figure.typeLabel}</span>
        {se !== null && <> · SE {se}</>}
        {' · '}n = {formatCount(observations).text}
      </p>
      <IntervalBar estimate={estimate} lower={lower} upper={upper} accent={accent} />
      <p className={labelCn('mb-0 mt-2 text-muted')}>{scaleLine}</p>
    </figure>
  )
}

/** Takes the slot of a result when the machine declines: grey, the reason, the rule, and what to do. */
export function RefusalTile({ label, headline, reason, rule, actions, testId }: {
  readonly label: string
  readonly headline: string
  readonly reason: ReactNode
  readonly rule: string
  readonly actions?: ReactNode
  readonly testId?: string
}) {
  return (
    <div className="rounded-lg border border-hair bg-well px-4 py-3" data-testid={testId} data-context={rule} role="status">
      <span className={labelCn('block text-muted')}>{label}</span>
      <p className="mb-0 mt-1 text-heading font-semibold text-bone">{headline}</p>
      <div className="mt-1 text-body text-bone">{reason}</div>
      {actions && <div className="mt-3 flex flex-wrap gap-2">{actions}</div>}
    </div>
  )
}
