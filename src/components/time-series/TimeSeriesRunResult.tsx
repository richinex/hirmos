import { boundsReading } from '@/domain/estimation'
import { assertNever } from '@/domain/dop'
import { timeSeriesRunLabel, type TimeSeriesRun } from '@/domain/timeSeries'
import { formatStatistic, formatWords } from '@/lib/format/number'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { formatTime } from '@/lib/format/date'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { TimeSeriesEquation } from './TimeSeriesEquation'
import { LongRunCharts } from './LongRunCharts'
import { ArdlModelResult } from './ArdlModelResult'
import { VecmForecastChart } from './VecmForecastChart'
import { resultSurface, resultTitle, table, td, th } from '@/components/ui/recipes'

const number = (value: number) => formatStatistic('raw', value).text

export function TimeSeriesRunResult({ run }: { readonly run: TimeSeriesRun }) {
  if (run.kind === 'ardl-model') return <ArdlModelResult run={run} />
  const content = (() => {
    switch (run.kind) {
      case 'ardl': {
        const e = run.evidence
        const reading = boundsReading(e)
        return <>
          <ResultInterpretation interpretation={{ kind: 'result-interpretation', statements: [
            { kind: 'magnitude', text: reading === 'level-relation' ? `The bounds test supports a long-run relationship between ${run.outcome.name} and ${run.predictor.name}.` : reading === 'no-level-relation' ? 'The bounds test does not support a long-run relationship at the 5% level.' : 'The bounds test is inconclusive at the 5% level.' },
            { kind: 'magnitude', text: e.longRunEffect === 0 ? `The fitted long-run coefficient for ${run.predictor.name} is zero.` : `In the fitted long-run relationship, a 1-unit higher ${run.predictor.name} corresponds to ${number(Math.abs(e.longRunEffect))} ${e.longRunEffect < 0 ? 'lower' : 'higher'} ${run.outcome.name}.` },
            { kind: 'uncertainty', text: `The ${Math.round(e.level * 100)}% confidence interval for that coefficient is ${number(e.interval[0])} to ${number(e.interval[1])}. Interpreting it as a long-run relationship also requires support from the bounds test.` },
          ] }} />
          <MetricGrid label="ARDL summary">{([
            ['Outcome lags', String(e.arLag)], ['Predictor lags', `0–${e.dlLag}`], ['Bounds statistic', number(e.boundsStatistic)], ['5% bounds', e.boundsCritical[1]?.map(number).join(' to ') ?? '—'],
          ] as const).map(([title, value]) => <MetricTile key={title} label={title} value={formatWords(value)} />)}</MetricGrid>
        </>
      }
      case 'vecm': {
        const e = run.evidence
        return <>
          <ResultInterpretation interpretation={{ kind: 'result-interpretation', statements: [
            { kind: 'magnitude', text: e.rank === 0 ? 'The rank test does not support a long-run equilibrium relationship among these series.' : e.rank === run.variables.length ? 'The rank test selects full rank. This does not support treating the series as a cointegrated system of nonstationary variables.' : `The series provide evidence of ${e.rank === 1 ? 'one long-run equilibrium relationship' : `${e.rank} long-run equilibrium relationships`}.` },
            { kind: 'uncertainty', text: `The rank test uses a ${run.specification.significance}% confidence level. No confidence interval is calculated for the equilibrium coefficients.` },
          ] }} />
          <p className="m-0 text-body text-muted">{e.rank === 0 ? `The rank test used ${e.kArDiff} lagged ${e.kArDiff === 1 ? 'change' : 'changes'} per series. No coefficient model was fitted after the rank-zero result.` : `The fitted model uses ${e.kArDiff} lagged ${e.kArDiff === 1 ? 'change' : 'changes'} for each series.`}</p>
          {e.rank > 0 && <div className="figure-strip overflow-x-auto"><table className={table} aria-label="Equilibrium coefficients"><thead><tr><th className={th()}>Series</th>{Array.from({ length: e.rank }, (_, i) => <th key={i} className={th()}>Relationship {i + 1}</th>)}</tr></thead><tbody>{run.variables.map((variable, i) => <tr key={variable.id}><td className={td()}>{variable.name}</td>{e.beta[i]?.map((value, j) => <td key={j} className={td()}>{number(value)}</td>)}</tr>)}</tbody></table><p className="text-label text-faint">Each column gives the coefficients in an equilibrium combination of the series. These are not estimates of what would happen under an intervention.</p></div>}
        </>
      }
      default: return assertNever(run)
    }
  })()
  return <section className={resultSurface('text-body text-muted')} aria-label="Time-series result"><h3 className={`${resultTitle} m-0`}>{timeSeriesRunLabel(run)}</h3>{content}<LongRunCharts key={run.id} run={run} />{run.kind==='vecm'&&<VecmForecastChart run={run} />}<TimeSeriesEquation run={run} /><p className="m-0 text-label text-faint">{run.evidence.observations} observations · {formatTime(run.createdAt)}</p></section>
}
