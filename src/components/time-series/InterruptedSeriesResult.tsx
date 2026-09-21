import { useState } from 'react'
import { Metadata } from '@/components/ui/Metadata'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { interruptedCorrelationOption, interruptedFitOption, interruptedResidualOption } from '@/charts/data/interruptedSeries'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { label, resultSurface, resultTitle, table, td, th } from '@/components/ui/recipes'
import { describeImpact, levelTerm, rateRatioOf, slopeTerm, trendTerm, type InterruptedTermEvidence } from '@/domain/interruptedSeries'
import { timeSeriesRunLabel, type TimeSeriesRun } from '@/domain/timeSeries'
import type { NonEmptyArray } from '@/domain/dop'
import type { InterpretationStatement } from '@/domain/resultInterpretation'
import { formatTime } from '@/lib/format/date'
import { formatEstimate, formatInterval, formatP, formatStatistic, formatWords } from '@/lib/format/number'
import { EChart } from '@/charts/EChart'
import { TimeSeriesEquation } from './TimeSeriesEquation'

type Run = Extract<TimeSeriesRun, { kind: 'interrupted-series' }>
const number = (value: number) => formatStatistic('raw', value).text
const CI95 = { kind: 'confidence', level: 0.95 } as const
const THREE = { precision: { kind: 'decimals', places: 3 } } as const
/** The paper rounds rate ratios and their limits to three decimals: 0.885 [0.839, 0.933]. */
const ratio = (term: InterruptedTermEvidence) => { const r = rateRatioOf(term); return formatInterval(r.ratio, r.interval[0], r.interval[1], CI95, { kind: 'ratio', label: 'RR' }, THREE).text }
const additive = (term: InterruptedTermEvidence, unit = '') => formatInterval(term.coefficient, term.interval[0], term.interval[1], CI95, { kind: 'additive', unit }).text
const three = (value: number) => formatEstimate(value, { kind: 'additive', unit: '' }, THREE).text
const significant = (value: number) => formatEstimate(value, { kind: 'additive', unit: '' }, { precision: { kind: 'significant', digits: 3 } }).text

const termName = (name: string): string => {
  switch (name) {
    case 'const': return 'Constant'
    case 'time': return 'Trend per row'
    case 'step': return 'Level change'
    case 'slope_change': return 'Slope change per row'
    default: return name.startsWith('sin') ? `Sine ${name.slice(3)}` : name.startsWith('cos') ? `Cosine ${name.slice(3)}` : name
  }
}

/** The effect as a sentence: a difference in the series' units, or a rate ratio for a count. */
function reading(term: InterruptedTermEvidence, count: boolean, what: string, unit: string): string {
  const crosses = term.interval[0] <= 0 && term.interval[1] >= 0
  const estimate = count
    ? `The ${what} is a rate ratio of ${ratio(term)}, ${formatP(term.pValue).text}`
    : `The ${what} is ${additive(term)} ${unit}, ${formatP(term.pValue).text}`
  return `${estimate}. ${crosses ? `The interval includes ${count ? 'one' : 'zero'}, so the data do not distinguish this change from none.` : `The interval excludes ${count ? 'one' : 'zero'}.`}`
}

export function InterruptedSeriesResult({ run }: { readonly run: Run }) {
  const { specification: s, evidence: e } = run
  const count = e.model.kind === 'count'
  const level = levelTerm(e)
  const slope = slopeTerm(e)
  const trend = trendTerm(e)
  const theme = useChartTheme()
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const lastBox = e.ljungBox.at(-1)
  const period = e.seasonal.kind === 'harmonic' ? e.seasonal.period : null
  const uncertainty = e.model.kind === 'count' ? `Intervals come from the quasi-Poisson fit with dispersion ${three(e.model.dispersion)}.` : `Intervals use Newey–West standard errors with bandwidth ${e.model.hacMaxLags}.`
  const statements: NonEmptyArray<InterpretationStatement> = [
    { kind: 'magnitude', text: level === null ? reading(slope!, count, 'slope change', 'per row') : reading(level, count, s.impact.kind === 'temporaryLevel' ? 'temporary level change' : 'level change', 'in the units of the series') },
    ...(level !== null && slope !== null ? [{ kind: 'magnitude' as const, text: reading(slope, count, 'slope change', 'per row') }] : []),
    { kind: 'uncertainty', text: `${uncertainty} The impact model was declared as a ${describeImpact(s.impact)}${s.lag > 0 ? ` with a lag of ${s.lag} rows` : ''}; a single series cannot separate the event from anything else that changed at the same row.` },
    ...(lastBox === undefined ? [] : [{ kind: 'uncertainty' as const, text: lastBox.pValue < 0.05 ? `The Ljung–Box test at ${e.ljungBox.length} lags rejects white-noise residuals (${formatP(lastBox.pValue).text}); the intervals may still be too narrow.` : `The Ljung–Box test at ${e.ljungBox.length} lags does not reject white-noise residuals (${formatP(lastBox.pValue).text}).` }]),
  ]
  // The paper's trend: exp(coef × 12) per year for a monthly count; here per cycle of the period when there is one.
  const cycle = period === null ? null : Number.isInteger(period) ? String(period) : number(period)
  const trendValue = period === null ? trend.coefficient : count ? Math.exp(trend.coefficient * period) : trend.coefficient * period
  const tiles = [
    ...(level === null ? [] : [[count ? 'Level change, rate ratio' : 'Level change', count ? ratio(level) : additive(level)] as const]),
    ...(slope === null ? [] : [[count ? 'Slope change per row, rate ratio' : 'Slope change per row', count ? ratio(slope) : additive(slope)] as const]),
    [cycle === null ? (count ? 'Trend per row, rate ratio' : 'Trend per row') : count ? `Trend per ${cycle} rows, rate ratio` : `Trend per ${cycle} rows`, cycle === null && count ? three(Math.exp(trendValue)) : three(trendValue)] as const,
    ...(e.model.kind === 'count' ? [['Dispersion', three(e.model.dispersion)] as const] : []),
  ]
  const exposureName = s.model.kind === 'count' ? s.model.exposure?.name ?? null : null
  const scale = count ? (exposureName === null ? 'counts' : `counts at the mean ${exposureName}`) : 'the units of the series'
  const observed = e.path.map((row) => row.observed)
  const fitted = e.path.map((row) => row.fitted)
  const counterfactual = e.path.map((row) => row.counterfactual)
  const residuals = e.path.map((row) => row.residual)
  return <section className={resultSurface('text-body text-muted')} aria-label="Time-series result">
    <h3 className={`${resultTitle} m-0`}>{timeSeriesRunLabel(run)}</h3>
    <ResultInterpretation interpretation={{ kind: 'result-interpretation', statements }} />
    <MetricGrid label="Interrupted series summary">{tiles.map(([title, value]) => <MetricTile key={title} label={title} value={formatWords(value)} />)}</MetricGrid>
    {run.plotTime !== undefined && <div className="grid min-w-0 gap-6">
      <div className="min-w-0">
        <h4 className={label('m-0 text-faint')}>Series, fit, and the counterfactual</h4>
        <ExpandableChart option={interruptedFitOption({ title: 'Series, fit, and the counterfactual', axis: run.plotTime, observed, fitted, counterfactual, fittedName: 'Fitted', interventionRow: e.interventionRow, outcomeName: run.outcome.name }, theme)} label="Interrupted series fit" className="mt-1 h-72" window={window} onWindow={setWindow} />
        <p className="m-0 mt-1 text-label text-muted">Values in {scale}; the shaded band is the period after the event. The dashed line continues the fit with the event's terms at zero. The gap between the lines is the fitted effect, not a forecast.</p>
      </div>
      {e.seasonal.kind === 'harmonic' && <div className="min-w-0">
        <h4 className={label('m-0 text-faint')}>Deseasonalised trend</h4>
        <ExpandableChart option={interruptedFitOption({ title: 'Deseasonalised trend', axis: run.plotTime, observed, fitted: e.seasonal.deseasonalised.map((row) => row.fitted), counterfactual: e.seasonal.deseasonalised.map((row) => row.counterfactual), fittedName: 'Fitted, seasonal terms fixed', interventionRow: e.interventionRow, outcomeName: run.outcome.name }, theme)} label="Deseasonalised trend" className="mt-1 h-72" window={window} onWindow={setWindow} />
        <p className="m-0 mt-1 text-label text-muted">Every row predicted at one phase of the seasonal cycle, half a period in, so the level and slope changes show without the seasonal curve.</p>
      </div>}
      <div className="min-w-0">
        <h4 className={label('m-0 text-faint')}>Residuals over time</h4>
        <EChart option={interruptedResidualOption({ axis: run.plotTime, residuals, interventionRow: e.interventionRow, kind: count ? 'deviance' : 'ordinary' }, theme)} label="Residuals over time" className="mt-1 h-48" />
      </div>
      {e.residualAcf.length > 1 && <div className="grid min-w-0 gap-4 @lg/panel:grid-cols-2">
        <div className="min-w-0"><h4 className={label('m-0 text-faint')}>Autocorrelation of residuals</h4><EChart option={interruptedCorrelationOption({ title: 'Autocorrelation', correlations: e.residualAcf }, theme)} label="Residual autocorrelation" className="mt-1 h-48" /></div>
        <div className="min-w-0"><h4 className={label('m-0 text-faint')}>Partial autocorrelation of residuals</h4><EChart option={interruptedCorrelationOption({ title: 'Partial autocorrelation', correlations: e.residualPacf }, theme)} label="Residual partial autocorrelation" className="mt-1 h-48" /></div>
      </div>}
      <p className="m-0 text-label text-muted">Bars outside the dashed band mean serial correlation remains at that lag; consider more seasonal terms{count ? '' : ' or a wider Newey–West bandwidth'}.</p>
    </div>}
    <div className="overflow-x-auto"><table className={table} aria-label="Fitted terms"><thead><tr><th className={th()}>Term</th><th className={th()}>Coefficient</th><th className={th()}>Standard error</th>{count ? <th className={th()}>Rate ratio (95% interval)</th> : <th className={th()}>95% interval</th>}<th className={th()}>p</th></tr></thead><tbody>
      {e.terms.map((term) => { const r = rateRatioOf(term); return <tr key={term.name}><td className={td()}>{termName(term.name)}</td><td className={td()}>{significant(term.coefficient)}</td><td className={td()}>{significant(term.standardError)}</td>{count ? <td className={td()}>{three(r.ratio)} ({three(r.interval[0])} to {three(r.interval[1])})</td> : <td className={td()}>{number(term.interval[0])} to {number(term.interval[1])}</td>}<td className={td()}>{formatP(term.pValue, { withLabel: false }).text}</td></tr> })}
    </tbody></table></div>
    {e.ljungBox.length > 0 && <details><summary className="cursor-pointer text-body text-ink">Ljung–Box tests</summary><div className="mt-3 overflow-x-auto"><table className={table} aria-label="Ljung–Box tests"><thead><tr><th className={th()}>Lags</th><th className={th()}>Q</th><th className={th()}>p</th></tr></thead><tbody>{e.ljungBox.map((box, i) => <tr key={i}><td className={td()}>{i + 1}</td><td className={td()}>{number(box.statistic)}</td><td className={td()}>{formatP(box.pValue, { withLabel: false }).text}</td></tr>)}</tbody></table></div></details>}
    <TimeSeriesEquation run={run} />
    <p className="m-0 text-label text-faint"><Metadata><span>{e.observations} observations</span><span>{e.interventionRow} before the event</span><span>{formatTime(run.createdAt)}</span></Metadata></p>
  </section>
}
