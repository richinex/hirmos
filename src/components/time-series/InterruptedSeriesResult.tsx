import { useMemo, useState } from 'react'
import { ArmaUncertaintyAlert } from '@/components/estimation/ArmaUncertaintyAlert'
import { Metadata } from '@/components/ui/Metadata'
import { ExpandableChart } from '@/charts/ExpandableChart'
import {
  interruptedCorrelationOption,
  interruptedFitOption,
  interruptedResidualOption,
} from '@/charts/data/interruptedSeries'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { label, resultSurface, resultTitle } from '@/components/ui/recipes'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import {
  describeImpact,
  levelTerm,
  rateRatioOf,
  slopeTerm,
  trendTerm,
  type InterruptedTermEvidence,
} from '@/domain/interruptedSeries'
import { timeSeriesRunLabel, type TimeSeriesRun } from '@/domain/timeSeries'
import type { NonEmptyArray } from '@/domain/dop'
import type { InterpretationStatement } from '@/domain/resultInterpretation'
import { formatTime } from '@/lib/format/date'
import {
  formatEstimate,
  formatInterval,
  formatP,
  formatStatistic,
  formatWords,
  type Formatted,
} from '@/lib/format/number'
import { EChart } from '@/charts/EChart'
import { TimeSeriesEquation } from './TimeSeriesEquation'

type Run = Extract<TimeSeriesRun, { kind: 'interrupted-series' }>
interface TermRow {
  readonly key: string
  readonly term: string
  readonly coefficient: number
  readonly standardError: number
  readonly ratio: ReturnType<typeof rateRatioOf>
  readonly interval: readonly [number, number]
  readonly pValue: number
}
interface ErrorTermRow {
  readonly key: string
  readonly term: string
  readonly coefficient: number
  readonly standardError: number
  readonly interval: readonly [number, number]
  readonly pValue: number
}
interface LjungBoxRow {
  readonly lags: number
  readonly statistic: number
  readonly pValue: number
}
const number = (value: number) => formatStatistic('raw', value).text
const CI95 = { kind: 'confidence', level: 0.95 } as const
const THREE = { precision: { kind: 'decimals', places: 3 } } as const
/** The paper rounds rate ratios and their limits to three decimals: 0.885 [0.839, 0.933]. */
const ratio = (term: InterruptedTermEvidence) => {
  const r = rateRatioOf(term)
  return formatInterval(
    r.ratio,
    r.interval[0],
    r.interval[1],
    CI95,
    { kind: 'ratio', label: 'RR' },
    THREE,
  ).text
}
const additive = (term: InterruptedTermEvidence, unit = '') =>
  formatInterval(term.coefficient, term.interval[0], term.interval[1], CI95, {
    kind: 'additive',
    unit,
  }).text
const three = (value: number) => formatEstimate(value, { kind: 'additive', unit: '' }, THREE).text

/** A tile's figure is the estimate alone, at the interval's precision; the interval is its context line. */
const effectTile = (
  term: InterruptedTermEvidence,
  count: boolean,
): { readonly value: Formatted; readonly context: string } => {
  if (count) {
    const r = rateRatioOf(term)
    const interval = formatInterval(
      r.ratio,
      r.interval[0],
      r.interval[1],
      CI95,
      { kind: 'ratio', label: 'RR' },
      THREE,
    )
    return {
      value: formatEstimate(r.ratio, { kind: 'ratio', label: 'RR' }, THREE),
      context: `${interval.bounds.lower} to ${interval.bounds.upper}, ${interval.typeLabel}`,
    }
  }
  const interval = formatInterval(term.coefficient, term.interval[0], term.interval[1], CI95, {
    kind: 'additive',
    unit: '',
  })
  const halfWidth = Math.abs(term.interval[1] - term.interval[0]) / 3.92
  return {
    value: formatEstimate(
      term.coefficient,
      { kind: 'additive', unit: '' },
      halfWidth > 0 ? { precision: { kind: 'matchSe', se: halfWidth } } : {},
    ),
    context: `${interval.bounds.lower} to ${interval.bounds.upper}, ${interval.typeLabel}`,
  }
}
const significant = (value: number) =>
  formatEstimate(
    value,
    { kind: 'additive', unit: '' },
    { precision: { kind: 'significant', digits: 3 } },
  ).text

const termName = (name: string): string => {
  switch (name) {
    case 'const':
      return 'Constant'
    case 'time':
      return 'Pre-intervention trend per row'
    case 'step':
      return 'Level change'
    case 'slope_change':
      return 'Slope change per row'
    default:
      return name.startsWith('sin')
        ? `Sine ${name.slice(3)}`
        : name.startsWith('cos')
          ? `Cosine ${name.slice(3)}`
          : name
  }
}

/** The effect as a sentence: a difference in the series' units, or a rate ratio for a count. */
function reading(
  term: InterruptedTermEvidence,
  count: boolean,
  what: string,
  unit: string,
): string {
  const crosses = term.interval[0] <= 0 && term.interval[1] >= 0
  const estimate = count
    ? term.name === 'slope_change'
      ? `The ratio of post-change to pre-change per-row rate multipliers is ${ratio(term)}, ${formatP(term.pValue).text}`
      : `The ${what} is a rate ratio of ${ratio(term)}, ${formatP(term.pValue).text}`
    : `The ${what} is ${additive(term)} ${unit}, ${formatP(term.pValue).text}`
  return `${estimate}. ${crosses ? `The interval includes ${count ? 'one' : 'zero'}, so this ${what} is uncertain under the fitted model.` : `The interval excludes ${count ? 'one' : 'zero'}.`}`
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
  const errors = e.model.kind === 'continuous' ? e.model.errors : null
  const uncertainty =
    e.model.kind === 'count'
      ? `Intervals come from the quasi-Poisson fit with dispersion ${three(e.model.dispersion)}.`
      : e.model.errors.kind === 'neweyWest'
        ? `Intervals use Newey–West standard errors with bandwidth ${e.model.errors.maxLags}.`
        : `The terms were fitted jointly with ARMA(${e.model.errors.p}, ${e.model.errors.q}) errors by maximum likelihood; intervals use outer-product-of-gradients standard errors.${e.model.errors.converged ? '' : ` The optimiser stopped after ${e.model.errors.iterations} iterations without converging; review the specification and optimiser settings before reading the numbers.`}`
  const statements: NonEmptyArray<InterpretationStatement> = [
    {
      kind: 'magnitude',
      text:
        level === null
          ? reading(slope!, count, 'slope change', 'per row')
          : reading(
              level,
              count,
              s.impact.kind === 'temporaryLevel' ? 'temporary level change' : 'level change',
              'in the units of the series',
            ),
    },
    ...(level !== null && slope !== null
      ? [{ kind: 'magnitude' as const, text: reading(slope, count, 'slope change', 'per row') }]
      : []),
    {
      kind: 'uncertainty',
      text: `${uncertainty} The impact model was declared as a ${describeImpact(s.impact)}${s.lag > 0 ? ` with a lag of ${s.lag} rows` : ''}; a single series cannot separate the event from anything else that changed at the same row.`,
    },
    ...(level !== null && slope !== null
      ? [
          {
            kind: 'qualification' as const,
            text: count
              ? 'At the first affected row, add the level and slope coefficients, then exponentiate to obtain the fitted rate ratio.'
              : 'At the first affected row, the fitted difference is the level coefficient plus the slope coefficient.',
          },
        ]
      : []),
    ...(lastBox === undefined
      ? []
      : [
          {
            kind: 'uncertainty' as const,
            text:
              lastBox.pValue < 0.05
                ? `The Ljung–Box test finds residual autocorrelation through lag ${e.ljungBox.length} (${formatP(lastBox.pValue).text}). Review the model and its uncertainty assumptions.`
                : `The Ljung–Box test finds no evidence of residual autocorrelation through lag ${e.ljungBox.length} (${formatP(lastBox.pValue).text}). This does not establish model adequacy.`,
          },
        ]),
  ]
  // The paper's trend: exp(coef × 12) per year for a monthly count; here per cycle of the period when there is one.
  const cycle = period === null ? null : Number.isInteger(period) ? String(period) : number(period)
  const trendValue =
    period === null
      ? trend.coefficient
      : count
        ? Math.exp(trend.coefficient * period)
        : trend.coefficient * period
  const plain = (label: string, text: string) => ({
    label,
    value: formatWords(text),
    context: null,
  })
  const tiles: readonly {
    readonly label: string
    readonly value: Formatted
    readonly context: string | null
  }[] = [
    ...(level === null
      ? []
      : [
          {
            label: count ? 'Level change, rate ratio' : 'Level change',
            ...effectTile(level, count),
          },
        ]),
    ...(slope === null
      ? []
      : [
          {
            label: count ? 'Change in per-row rate multiplier' : 'Slope change per row',
            ...effectTile(slope, count),
          },
        ]),
    plain(
      cycle === null
        ? count
          ? 'Pre-intervention trend per row, rate ratio'
          : 'Pre-intervention trend per row'
        : count
          ? `Pre-intervention trend per ${cycle} rows, rate ratio`
          : `Pre-intervention trend per ${cycle} rows`,
      cycle === null && count ? three(Math.exp(trendValue)) : three(trendValue),
    ),
    ...(e.model.kind === 'count' ? [plain('Dispersion', three(e.model.dispersion))] : []),
    ...(errors !== null && errors.kind === 'arma'
      ? [plain('Innovation variance', three(errors.sigma2)), plain('AIC', three(errors.aic))]
      : []),
  ]
  const residualKind = count
    ? 'deviance'
    : errors !== null && errors.kind === 'arma'
      ? 'standardised'
      : 'ordinary'
  const exposureName = s.model.kind === 'count' ? (s.model.exposure?.name ?? null) : null
  const scale = count
    ? exposureName === null
      ? 'counts'
      : `counts at the mean ${exposureName}`
    : 'the units of the series'
  const observed = e.path.map((row) => row.observed)
  const fitted = e.path.map((row) => row.fitted)
  const counterfactual = e.path.map((row) => row.counterfactual)
  const residuals = e.path.map((row) => row.residual)
  // The charts share one window, so a zoom re-renders this component; the options must not be rebuilt with it.
  const plotTime = run.plotTime
  const fitOption = useMemo(
    () =>
      plotTime === undefined
        ? null
        : interruptedFitOption(
            {
              title: 'Series, fit, and the counterfactual',
              axis: plotTime,
              observed,
              fitted,
              counterfactual,
              fittedName: 'Fitted',
              interventionRow: e.interventionRow,
              outcomeName: run.outcome.name,
            },
            theme,
          ),
    [plotTime, observed, fitted, counterfactual, e.interventionRow, run.outcome.name, theme],
  )
  const deseasonalisedOption = useMemo(
    () =>
      plotTime === undefined || e.seasonal.kind !== 'harmonic'
        ? null
        : interruptedFitOption(
            {
              title: 'Deseasonalised trend',
              axis: plotTime,
              observed,
              fitted: e.seasonal.deseasonalised.map((row) => row.fitted),
              counterfactual: e.seasonal.deseasonalised.map((row) => row.counterfactual),
              fittedName: 'Fitted, seasonal terms fixed',
              interventionRow: e.interventionRow,
              outcomeName: run.outcome.name,
            },
            theme,
          ),
    [plotTime, observed, e.seasonal, e.interventionRow, run.outcome.name, theme],
  )
  const residualOption = useMemo(
    () =>
      plotTime === undefined
        ? null
        : interruptedResidualOption(
            { axis: plotTime, residuals, interventionRow: e.interventionRow, kind: residualKind },
            theme,
          ),
    [plotTime, residuals, e.interventionRow, residualKind, theme],
  )
  const acfOption = useMemo(
    () =>
      interruptedCorrelationOption(
        { title: 'Autocorrelation', correlations: e.residualAcf },
        theme,
      ),
    [e.residualAcf, theme],
  )
  const pacfOption = useMemo(
    () =>
      interruptedCorrelationOption(
        { title: 'Partial autocorrelation', correlations: e.residualPacf },
        theme,
      ),
    [e.residualPacf, theme],
  )
  return (
    <section className={resultSurface('text-body text-muted')} aria-label="Time-series result">
      <h3 className={`${resultTitle} m-0`}>{timeSeriesRunLabel(run)}</h3>
      <ResultInterpretation interpretation={{ kind: 'result-interpretation', statements }} />
      {errors !== null && errors.kind === 'arma' && <ArmaUncertaintyAlert evidence={errors} />}
      <MetricGrid label="Interrupted series summary">
        {tiles.map((tile) => (
          <MetricTile
            key={tile.label}
            label={tile.label}
            value={tile.value}
            context={tile.context}
          />
        ))}
      </MetricGrid>
      {fitOption !== null && residualOption !== null && (
        <div className="grid min-w-0 gap-6">
          <div className="min-w-0">
            <h4 className={label('m-0 text-faint')}>Series, fit, and the counterfactual</h4>
            <ExpandableChart
              option={fitOption}
              label="Interrupted series fit"
              className="mt-1 h-72"
              window={window}
              onWindow={setWindow}
            />
            <p className="m-0 mt-1 text-label text-muted">
              Values in {scale}; the shaded band is the period after the event. The dashed line
              continues the fit with the event's terms at zero. The gap is the model-estimated
              difference, not a forecast.
            </p>
          </div>
          {deseasonalisedOption !== null && (
            <div className="min-w-0">
              <h4 className={label('m-0 text-faint')}>Deseasonalised trend</h4>
              <ExpandableChart
                option={deseasonalisedOption}
                label="Deseasonalised trend"
                className="mt-1 h-72"
                window={window}
                onWindow={setWindow}
              />
              <p className="m-0 mt-1 text-label text-muted">
                Every row predicted at one phase of the seasonal cycle, half a period in, so the
                level and slope changes show without the seasonal curve.
              </p>
            </div>
          )}
          <div className="min-w-0">
            <h4 className={label('m-0 text-faint')}>Residuals over time</h4>
            <EChart option={residualOption} label="Residuals over time" className="mt-1 h-48" />
            {residualKind === 'standardised' && (
              <p className="m-0 mt-1 text-label text-muted">
                Observed values minus their one-step-ahead predictions, divided by the forecast
                error standard deviation.
              </p>
            )}
          </div>
          {e.residualAcf.length > 1 && (
            <div className="grid min-w-0 gap-4 @lg/panel:grid-cols-2">
              <div className="min-w-0">
                <h4 className={label('m-0 text-faint')}>Autocorrelation of residuals</h4>
                <EChart option={acfOption} label="Residual autocorrelation" className="mt-1 h-48" />
              </div>
              <div className="min-w-0">
                <h4 className={label('m-0 text-faint')}>Partial autocorrelation of residuals</h4>
                <EChart
                  option={pacfOption}
                  label="Residual partial autocorrelation"
                  className="mt-1 h-48"
                />
              </div>
            </div>
          )}
          <p className="m-0 text-label text-muted">
            Bars outside the dashed bands suggest residual correlation. The bands apply to each lag
            separately. Review the pattern alongside the Ljung–Box test.
          </p>
        </div>
      )}
      <WindowEvidence
        context={run.windowContext}
        seasonal={
          period === null ? undefined : { rows: e.observations, period, before: e.interventionRow }
        }
      />
      <EvidenceTable<TermRow>
        frame="none"
        title="Fitted terms"
        rows={e.terms.map((term) => ({
          key: term.name,
          term: termName(term.name),
          coefficient: term.coefficient,
          standardError: term.standardError,
          ratio: rateRatioOf(term),
          interval: term.interval,
          pValue: term.pValue,
        }))}
        rowKey={(row) => row.key}
        noun="term"
        empty="The fit reported no terms."
        exportName="interrupted-series-terms"
        columns={[
          { id: 'term', header: 'Term', value: (row) => row.term },
          figureColumn<TermRow>(
            'coefficient',
            'Coefficient',
            (row) => row.coefficient,
            significant,
          ),
          figureColumn<TermRow>(
            'standard-error',
            'Standard error',
            (row) => row.standardError,
            significant,
          ),
          count
            ? {
                id: 'rate-ratio',
                header: 'Rate ratio (95% interval)',
                align: 'right',
                value: (row) => row.ratio.ratio,
                format: (_, row) =>
                  `${three(row.ratio.ratio)} (${three(row.ratio.interval[0])} to ${three(row.ratio.interval[1])})`,
              }
            : {
                id: 'interval',
                header: '95% interval',
                align: 'right',
                value: (row) => row.interval[0],
                format: (_, row) => `${number(row.interval[0])} to ${number(row.interval[1])}`,
              },
          figureColumn<TermRow>(
            'p',
            'p',
            (row) => row.pValue,
            (value) => formatP(value, { withLabel: false }).text,
          ),
        ]}
      />
      {errors !== null && errors.kind === 'arma' && (
        <EvidenceTable<ErrorTermRow>
          frame="none"
          title="Error process"
          rows={[...errors.ar, ...errors.ma].map((term) => ({
            key: term.name,
            term: term.name,
            coefficient: term.coefficient,
            standardError: term.standardError,
            interval: term.interval,
            pValue: term.pValue,
          }))}
          rowKey={(row) => row.key}
          noun="term"
          empty="The error process has no terms."
          exportName="interrupted-series-error-process"
          columns={[
            { id: 'term', header: 'Error term', value: (row) => row.term },
            figureColumn<ErrorTermRow>(
              'coefficient',
              'Coefficient',
              (row) => row.coefficient,
              significant,
            ),
            figureColumn<ErrorTermRow>(
              'standard-error',
              'Standard error',
              (row) => row.standardError,
              significant,
            ),
            {
              id: 'interval',
              header: '95% interval',
              align: 'right',
              value: (row) => row.interval[0],
              format: (_, row) => `${number(row.interval[0])} to ${number(row.interval[1])}`,
            },
            figureColumn<ErrorTermRow>(
              'p',
              'p',
              (row) => row.pValue,
              (value) => formatP(value, { withLabel: false }).text,
            ),
          ]}
        />
      )}
      {errors !== null && errors.kind === 'arma' && (
        <p className="m-0 text-label text-muted">
          <Metadata>
            <span>innovation variance {significant(errors.sigma2)}</span>
            <span>log likelihood {number(errors.logLikelihood)}</span>
            <span>AIC {number(errors.aic)}</span>
            <span>BIC {number(errors.bic)}</span>
            <span>
              {errors.converged
                ? `converged in ${errors.iterations} iterations`
                : `stopped at ${errors.iterations} iterations`}
            </span>
          </Metadata>
        </p>
      )}
      {e.ljungBox.length > 0 && (
        <details>
          <summary className="cursor-pointer text-body text-ink">Ljung–Box tests</summary>
          <div className="mt-3">
            <EvidenceTable<LjungBoxRow>
              frame="none"
              title="Ljung–Box tests"
              rows={e.ljungBox.map((box, i) => ({
                lags: i + 1,
                statistic: box.statistic,
                pValue: box.pValue,
              }))}
              rowKey={(row) => String(row.lags)}
              noun="lag"
              empty="No Ljung–Box test was run."
              exportName="interrupted-series-ljung-box"
              columns={[
                figureColumn<LjungBoxRow>(
                  'lags',
                  'Lags',
                  (row) => row.lags,
                  (value) => String(value),
                ),
                figureColumn<LjungBoxRow>('q', 'Q', (row) => row.statistic, number),
                figureColumn<LjungBoxRow>(
                  'p',
                  'p',
                  (row) => row.pValue,
                  (value) => formatP(value, { withLabel: false }).text,
                ),
              ]}
            />
          </div>
        </details>
      )}
      <TimeSeriesEquation run={run} />
      <p className="m-0 text-label text-faint">
        <Metadata>
          <span>{e.observations} observations</span>
          <span>{e.interventionRow} before the event</span>
          <span>{formatTime(run.createdAt)}</span>
        </Metadata>
      </p>
    </section>
  )
}
import { WindowEvidence } from './WindowEvidence'
