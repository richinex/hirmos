import { FigureParts, IntervalFigure } from '@/components/ui/figures'
import { label, num, td, th } from '@/components/ui/recipes'
import { additive, headlineValue, intervalTypeOf, summariseRowEffects, type CausalEstimate } from '@/domain/estimation'
import { formatCount, formatEstimate, formatPercent, formatStatistic, type EffectScale, type Formatted } from '@/lib/format/number'

/** The scale an estimate's figures are printed on. */
export const EXPECTED_COUNT_RATIO = { kind: 'ratio', label: 'ECR' } as const

export const scaleOf = (estimate: CausalEstimate): EffectScale => (estimate.effect.kind === 'expectedCountRatio' ? EXPECTED_COUNT_RATIO : additive)

/** The headline figure of any estimate, for ledgers and comparisons. */
export const headlineFigure = (estimate: CausalEstimate): Formatted => formatEstimate(headlineValue(estimate.effect), scaleOf(estimate))

/** Where the per-row effects sit: the quartiles and extremes, then the average as the closing row. */
function RowEffectTable({ effect }: { readonly effect: Extract<CausalEstimate['effect'], { kind: 'perRow' }> }) {
  const summary = summariseRowEffects(effect.effects)
  const figure = (value: number) => formatStatistic('raw', value).text
  const rows: readonly (readonly [string, number])[] = [
    ['Smallest row effect', summary.minimum],
    ['Lower quartile', summary.lowerQuartile],
    ['Median', summary.median],
    ['Upper quartile', summary.upperQuartile],
    ['Largest row effect', summary.maximum],
  ]
  return (
    <table className="mt-2 w-full border-collapse text-body">
      <thead>
        <tr>
          <th className={th()}>Across {formatCount(summary.rows).text} rows</th>
          <th className={th('text-right')}>Effect</th>
        </tr>
      </thead>
      <tbody>
        {rows.map(([name, value]) => (
          <tr key={name}>
            <td className={td('text-ink')}>{name}</td>
            <td className={td(num('text-right text-ink'))}>{figure(value)}</td>
          </tr>
        ))}
        <tr>
          <td className={td('text-muted')}>Rows with an effect above zero</td>
          <td className={td(num('text-right text-muted'))}>{formatPercent(summary.positiveShare, { precision: 0 }).text}</td>
        </tr>
        <tr>
          <td className={td('text-muted')}>Average over all rows</td>
          <td className={td(num('text-right text-ink'))}>{figure(effect.overall)}</td>
        </tr>
      </tbody>
    </table>
  )
}

/** One effect per group of the modifier, with the whole-population average as the closing row. */
function GroupEffectTable({ effect, interval, observations }: {
  readonly effect: Extract<CausalEstimate['effect'], { kind: 'byGroup' }>
  readonly interval: CausalEstimate['interval']
  readonly observations: number
}) {
  const bounds = (lower: number, upper: number) => `[${formatStatistic('raw', lower).text}, ${formatStatistic('raw', upper).text}]`
  return (
    <table className="mt-2 w-full border-collapse text-body">
      <thead>
        <tr>
          <th className={th()}>{effect.modifier}</th>
          <th className={th('text-right')}>Effect</th>
          <th className={th('text-right')}>{formatPercent(effect.groups[0].interval.level, { precision: 0 }).text} interval</th>
          <th className={th('text-right')}>n</th>
        </tr>
      </thead>
      <tbody>
        {effect.groups.map((group) => (
          <tr key={group.label}>
            <td className={td('text-ink')}>{group.label}{group.fewObservations ? <span className="text-warn"> · few rows</span> : null}</td>
            <td className={td(num('text-right text-ink'))}>{formatStatistic('raw', group.value).text}</td>
            <td className={td(num('text-right text-muted'))}>{bounds(group.interval.lower, group.interval.upper)}</td>
            <td className={td(num('text-right text-muted'))}>{formatCount(group.observations).text}</td>
          </tr>
        ))}
        <tr>
          <td className={td('text-muted')}>All rows</td>
          <td className={td(num('text-right text-ink'))}>{formatStatistic('raw', effect.overall).text}</td>
          <td className={td(num('text-right text-muted'))}>{interval.kind === 'none' ? 'no interval' : bounds(interval.lower, interval.upper)}</td>
          <td className={td(num('text-right text-muted'))}>{formatCount(observations).text}</td>
        </tr>
      </tbody>
    </table>
  )
}

/**
 * The hero of a result, in the one shape every chapter shows it: the estimand sentence, the figure,
 * what is known about its uncertainty, and the scale line. An estimate with an interval is the
 * interval figure; a path, or an estimate whose method reports no interval, says so in its place.
 */
export function EstimateHeadline({ estimate, sentence, scaleLine, sampleLine, stepLabel, accent, testId }: {
  readonly estimate: CausalEstimate
  readonly sentence: string
  readonly scaleLine: string
  readonly sampleLine: string
  readonly stepLabel: string
  /** Only the study's current accepted answer takes the signal colour. */
  readonly accent: boolean
  readonly testId: string
}) {
  if (estimate.effect.kind === 'byGroup') {
    return (
      <figure className="m-0" data-testid={testId}>
        <figcaption className="text-title text-ink">{sentence}</figcaption>
        <GroupEffectTable effect={estimate.effect} interval={estimate.interval} observations={estimate.sample.observations} />
        <p className={label('mb-0 mt-2 text-muted')}>{scaleLine}</p>
      </figure>
    )
  }
  if (estimate.effect.kind === 'perRow') {
    return (
      <figure className="m-0" data-testid={testId}>
        <figcaption className="text-title text-ink">{sentence}</figcaption>
        <RowEffectTable effect={estimate.effect} />
        {estimate.interval.kind === 'none' && <p className="mb-0 mt-1 text-body text-muted">{estimate.interval.reason}</p>}
        <p className={label('mb-0 mt-2 text-muted')}>{scaleLine}</p>
      </figure>
    )
  }
  if (estimate.interval.kind !== 'none' && estimate.effect.kind !== 'path') {
    return (
      <IntervalFigure
        sentence={sentence}
        estimate={estimate.effect.value}
        lower={estimate.interval.lower}
        upper={estimate.interval.upper}
        type={intervalTypeOf(estimate.interval)}
        scale={scaleOf(estimate)}
        standardError={estimate.standardError ?? undefined}
        sampleLine={sampleLine}
        scaleLine={scaleLine}
        accent={accent}
        testId={testId}
      />
    )
  }
  const figure = headlineFigure(estimate)
  const span = estimate.effect.kind === 'path'
    ? `cumulative over ${formatCount(estimate.effect.values.length).text} post-intervention ${stepLabel}s · average ${formatStatistic('raw', estimate.effect.aggregate.average).text} per ${stepLabel} · `
    : ''
  return (
    <figure className="m-0" data-testid={testId}>
      <figcaption className="text-title text-ink">{sentence}</figcaption>
      <p className={num(`mb-0 mt-1 text-metric font-semibold leading-none tracking-tight ${accent ? 'text-signal' : 'text-ink'}`)} title={figure.exact}><FigureParts value={figure} /></p>
      <p className={num('mb-0 mt-1 text-body text-bone')}>{span}no interval · {sampleLine}</p>
      {estimate.interval.kind === 'none' && <p className="mb-0 mt-1 text-body text-muted">{estimate.interval.reason}</p>}
      <p className={label('mb-0 mt-2 text-muted')}>{scaleLine}</p>
    </figure>
  )
}
