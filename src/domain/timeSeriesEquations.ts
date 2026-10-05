import { assertNever } from './dop'
import type { CountSeriesModelArtifact } from './countSeries'
import type { TimeSeriesRun } from './timeSeries'
import type { ContinuousErrorEvidence } from './interruptedSeries'

interface Expression {
  readonly tex: string
  readonly plain: string
}
type Fitted =
  | {
      readonly kind: 'available'
      readonly title: string
      readonly expressions: readonly Expression[]
      readonly explanation: string
    }
  | { readonly kind: 'unavailable'; readonly explanation: string }
export interface TimeSeriesEquations {
  readonly general: readonly Expression[]
  readonly definitions: readonly string[]
  readonly fitted: Fitted
  readonly reference: string
}
const expression = (tex: string, plain: string): Expression => ({ tex, plain })
const number = (value: number): string => Number(value.toPrecision(7)).toString()
const texNumber = (value: number): string => number(value).replace(/e([+-]?\d+)/, '\\times 10^{$1}')

/** Equations use the saved model settings, never the current setup controls. */
const errorsDefinition = (errors: ContinuousErrorEvidence): string => {
  switch (errors.kind) {
    case 'neweyWest':
      return `Standard errors are Newey–West with bandwidth ${errors.maxLags}.`
    case 'arma':
      return `The error u follows an ARMA(${errors.p}, ${errors.q}) process fitted jointly with the terms by maximum likelihood. Standard errors use the inverse or pseudoinverse of the outer-product-of-gradients information matrix. Innovation variance ${number(errors.sigma2)}, log likelihood ${number(errors.logLikelihood)}, AIC ${number(errors.aic)}.`
    default:
      return assertNever(errors)
  }
}

const armaTex = (errors: Extract<ContinuousErrorEvidence, { kind: 'arma' }>): string => {
  const ar = errors.ar.map((term, i) => `\\phi_{${i + 1}} u_{t-${i + 1}}`).join('+')
  const ma = errors.ma.map((term, i) => `\\theta_{${i + 1}} \\varepsilon_{t-${i + 1}}`).join('+')
  return `u_t=${ar === '' ? '' : `${ar}+`}\\varepsilon_t${ma === '' ? '' : `+${ma}`}`
}

const armaPlain = (errors: Extract<ContinuousErrorEvidence, { kind: 'arma' }>): string =>
  `error at t = ${errors.ar
    .map((term, i) => `${term.name} × error at t−${i + 1}`)
    .concat(
      'innovation at t',
      errors.ma.map((term, i) => `${term.name} × innovation at t−${i + 1}`),
    )
    .join(' + ')}`

export function timeSeriesEquations(
  run: TimeSeriesRun | CountSeriesModelArtifact,
): TimeSeriesEquations {
  switch (run.kind) {
    case 'ardl-model': {
      const e = run.evidence
      const fitted = e.coefficients.map((term, i) => {
        const coefficient = `(${texNumber(e.params[i]!)})`
        switch (term.kind) {
          case 'constant':
            return coefficient
          case 'trend':
            return `${coefficient}t`
          case 'outcome':
            return `${coefficient}y_{t-${term.lag}}`
          case 'predictor':
            return `${coefficient}x_{${term.column + 1},t-${term.lag}}`
          case 'fixed':
            return `${coefficient}z_{${term.column + 1},t}`
          default:
            return assertNever(term)
        }
      })
      return {
        general: [
          expression(
            'y_t=c+\\tau t+\\sum_{i=1}^{p}a_i y_{t-i}+\\sum_{k=1}^{K}\\sum_{j=0}^{q_k}b_{kj}x_{k,t-j}+\\sum_{m=1}^{M}d_m z_{m,t}+\\varepsilon_t',
            'Outcome = deterministic terms + weighted earlier outcomes + weighted current and earlier predictors + fixed regressors + error',
          ),
        ],
        definitions: [
          `y is ${run.outcome.name}. ${run.predictors.map((c, i) => `x${i + 1} is ${c.name}`).join('; ')}.`,
          `The trend term is included only when selected. Fixed regressors enter at the current time without distributed lags.${run.fixed.length ? ` ${run.fixed.map((c, i) => `z${i + 1} is ${c.name}`).join('; ')}.` : ''}`,
          't identifies a prepared time step. Forecasts require supplied future predictor and fixed-regressor values.',
        ],
        fitted: {
          kind: 'available',
          title: 'Fitted outcome model',
          expressions: [
            expression(
              `\\widehat y_t=${fitted.join('+')}`,
              `Fitted outcome coefficients: ${e.params.map(number).join(', ')}, in the order shown in the coefficient table.`,
            ),
          ],
          explanation:
            'This is the fitted levels equation. A long-run interpretation additionally requires a supported level relationship.',
        },
        reference:
          'Pesaran, Shin and Smith (2001), §2; statsmodels ARDL; Natsiopoulos and Tzeremes (2022), ARDL.',
      }
    }
    case 'count-series-model': {
      const e = run.result
      const log = e.link === 'log'
      const lhs = log ? '\\log\\lambda_t' : '\\lambda_t'
      const observation = (lag: string) => (log ? `\\log(1+Y_{t-${lag}})` : `Y_{t-${lag}}`)
      const mean = (lag: string) => (log ? `\\log\\lambda_{t-${lag}}` : `\\lambda_{t-${lag}}`)
      const valid = e.parameters.length === 1 + e.pastObservationLags.length + e.pastMeanLags.length
      const terms = valid
        ? [
            texNumber(e.parameters[0]!),
            ...e.pastObservationLags.map(
              (lag, i) => `(${texNumber(e.parameters[i + 1]!)})${observation(String(lag))}`,
            ),
            ...e.pastMeanLags.map(
              (lag, i) =>
                `(${texNumber(e.parameters[1 + e.pastObservationLags.length + i]!)})${mean(String(lag))}`,
            ),
          ]
        : []
      return {
        general: [
          expression(
            `${lhs}=\\beta_0+\\sum_{i\\in P}\\beta_i${observation('i')}+\\sum_{j\\in Q}\\alpha_j${mean('j')}`,
            log
              ? 'log mean(t) = intercept + weighted past log(1 + counts) + weighted past log means'
              : 'mean(t) = intercept + weighted past counts + weighted past means',
          ),
          expression(
            '\\operatorname{Var}(Y_t\\mid\\mathcal F_{t-1})=\\lambda_t+\\lambda_t^2/\\phi',
            'Conditional variance = mean + mean squared / negative-binomial size',
          ),
        ],
        definitions: [
          `Y is ${run.outcome.name}. t identifies a prepared time step; λ is the expected count given the earlier observations.`,
          `P contains count lags ${e.pastObservationLags.join(', ')}; Q contains mean lags ${e.pastMeanLags.join(', ')}. A lag of 1 means one prepared time step earlier.`,
          'φ is the negative-binomial size. This equation describes the baseline fit used by the change-date scan, not an estimated intervention effect.',
        ],
        fitted: valid
          ? {
              kind: 'available',
              title: 'Fitted mean model',
              expressions: [
                expression(
                  `${lhs}=${terms.join('+')}`,
                  `Fitted ${log ? 'log mean' : 'mean'}: coefficients ${e.parameters.map(number).join(', ')} in intercept, count-lag and mean-lag order`,
                ),
                expression(
                  `\\widehat\\phi=${texNumber(e.size)}`,
                  `Negative-binomial size = ${number(e.size)}`,
                ),
              ],
              explanation:
                'The fitted means are calculated recursively, using the selected earlier counts and earlier fitted means.',
            }
          : {
              kind: 'unavailable',
              explanation:
                'The saved coefficients do not match the selected lag terms, so a fitted equation cannot be shown.',
            },
        reference: 'Liboschik, Fokianos and Fried (2017), tscount, §2, equations (1)–(5).',
      }
    }
    case 'ardl': {
      const e = run.evidence
      return {
        general: [
          expression(
            `y_t=c${e.trend === 'ct' ? '+\\tau t' : ''}+\\sum_{i=1}^{${e.arLag}}a_i y_{t-i}+\\sum_{j=0}^{${e.dlLag}}b_j x_{t-j}+\\varepsilon_t`,
            `Outcome at t = constant${e.trend === 'ct' ? ' + time trend' : ''} + weighted earlier outcomes + weighted current and earlier predictor values + error`,
          ),
          expression(
            '\\theta=\\frac{\\sum_j b_j}{1-\\sum_i a_i}',
            'Long-run predictor coefficient = sum of predictor coefficients / (1 − sum of outcome coefficients)',
          ),
        ],
        definitions: [
          `y is ${run.outcome.name}; x is ${run.predictor.name}. t identifies a prepared time step. ε is the unexplained part of the outcome.`,
          'The first sum represents earlier outcomes; the second includes the current predictor and its earlier values. θ describes the fitted long-run relationship, when such a relationship is supported.',
          `The bounds test uses case ${e.case}. Restrictions on the constant or trend belong to that test; they do not remove these terms from the fitted levels model.`,
        ],
        fitted: {
          kind: 'available',
          title: 'Fitted long-run coefficient',
          expressions: [
            expression(
              `\\widehat\\theta=${texNumber(e.longRunEffect)}`,
              `Estimated long-run coefficient = ${number(e.longRunEffect)}`,
            ),
          ],
          explanation:
            'The saved result contains the long-run coefficient, but not the intercept and lag coefficients needed to display the complete fitted equation.',
        },
        reference:
          'Pesaran, Shin and Smith (2001), §2; statsmodels ARDL and UECM model definitions and cointegrating normalization.',
      }
    }
    case 'vecm': {
      const e = run.evidence
      const inside =
        e.deterministic === 'ci' ? '+\\eta' : e.deterministic === 'coli' ? '+\\eta(t-1)' : ''
      const outside = e.deterministic === 'co' || e.deterministic === 'coli' ? '+c' : ''
      const correction = e.rank === 0 ? '' : `\\alpha(\\beta^{\\mathsf T}y_{t-1}${inside})+`
      return {
        general: [
          expression(
            `\\Delta y_t=${correction}\\sum_{i=1}^{${e.kArDiff}}\\Gamma_i\\Delta y_{t-i}${outside}+u_t`,
            `Changes in the series = ${e.rank === 0 ? '' : 'adjustment to equilibrium departures + '}weighted earlier changes${outside ? ' + constant' : ''} + errors`,
          ),
        ],
        definitions: [
          `y contains all ${run.variables.length} selected series in the order shown in the result. Δy means the change from the preceding time step. Each coefficient matrix describes effects across the series, not just within one series.`,
          `The model uses ${e.kArDiff} earlier changes per series. ${e.rank === 0 ? 'The selected rank is zero, so no equilibrium-adjustment term is included.' : `β has ${e.rank} columns, one per selected relationship. α describes how each series adjusts to departures from those relationships.`}`,
          e.deterministic === 'ci'
            ? 'η is a constant inside each equilibrium relationship.'
            : e.deterministic === 'coli'
              ? 'η multiplies time inside the equilibrium relationships; c is a constant outside them.'
              : e.deterministic === 'co'
                ? 'c is a constant outside the equilibrium relationships.'
                : 'No constant or time trend is included.',
        ],
        fitted: {
          kind: 'unavailable',
          explanation:
            e.rank === 0
              ? 'No equilibrium coefficients were estimated at the selected rank of zero.'
              : e.deterministic === 'n'
                ? 'The equilibrium coefficients are listed in the result table. The equation above shows the structure of the system rather than expanding all of its fitted coefficient matrices.'
                : 'The equilibrium coefficients are listed in the result table. The complete fitted system is not displayed because this saved result does not retain every deterministic-term coefficient.',
        },
        reference:
          'Johansen (1991), §2, equations (2.1)–(2.2); statsmodels VECM uses the equivalent equilibrium term at t−1.',
      }
    }
    case 'count-regression':
      return {
        general: [],
        definitions: ['The design includes the recorded predictor terms and fixed effects.'],
        fitted: {
          kind: 'unavailable',
          explanation: 'Coefficients and joint contrasts are reported in the result table.',
        },
        reference: 'statsmodels 0.14.6 NegativeBinomialP, GLM Binomial and sandwich covariance.',
      }
    case 'panel-regression':
      return {
        general: [],
        definitions: ['The design uses the recorded reference periods and interactions.'],
        fitted: {
          kind: 'unavailable',
          explanation: 'Coefficients and their uncertainty are reported in the result table.',
        },
        reference: 'lfe 3.1.1 and estimatr 2.0.0.',
      }
    case 'bacon':
      return {
        general: [],
        definitions: [
          'The two-way fixed-effects coefficient is reconstructed from its weighted comparisons.',
        ],
        fitted: {
          kind: 'unavailable',
          explanation: 'Comparison estimates and weights are reported in the result table.',
        },
        reference: 'Goodman-Bacon (2021); bacondecomp 0.1.1.',
      }
    case 'interrupted-series': {
      const { specification: s, evidence: e } = run
      const count = s.model.kind === 'count'
      const step = s.impact.kind === 'slope' ? '' : `+\\beta_2 X_t`
      const slope =
        s.impact.kind === 'levelAndSlope' || s.impact.kind === 'slope'
          ? `+\\beta_3 (t-T_0) X_t`
          : ''
      const seasonal =
        s.seasonal.kind === 'none'
          ? ''
          : `+\\sum_{k=1}^{${s.seasonal.pairs}}\\left[\\gamma_k\\sin\\tfrac{2\\pi k t}{${number(s.seasonal.period)}}+\\delta_k\\cos\\tfrac{2\\pi k t}{${number(s.seasonal.period)}}\\right]`
      const left = count ? `\\log E[Y_t]=\\log N_t+` : 'Y_t='
      const plainLeft = count
        ? 'Log of the expected count = log of the exposure + '
        : 'Series at t = '
      const plainStep = s.impact.kind === 'slope' ? '' : ' + level change × after-event indicator'
      const plainSlope =
        slope === '' ? '' : ' + slope change × rows since the event × after-event indicator'
      const plainSeasonal = seasonal === '' ? '' : ' + harmonic seasonal terms'
      const fitted = e.terms
        .map((term) => `${term.name}=${texNumber(term.coefficient)}`)
        .join(',\;')
      return {
        general: [
          expression(
            `${left}\\beta_0+\\beta_1 t${step}${slope}${seasonal}${count ? '' : '+u_t'}`,
            `${plainLeft}constant + trend × t${plainStep}${plainSlope}${plainSeasonal}${count ? '' : ' + error'}`,
          ),
          ...(e.model.kind === 'continuous' && e.model.errors.kind === 'arma'
            ? [expression(armaTex(e.model.errors), armaPlain(e.model.errors))]
            : []),
        ],
        definitions: [
          `Y is ${run.outcome.name}; t is the row number from 1. X is 1 from row ${s.interventionRow}${s.lag > 0 ? ` plus a lag of ${s.lag}` : ''}${s.impact.kind === 'temporaryLevel' ? ` up to, but not including, row ${s.impact.until}` : ''}. X is 0 otherwise. T₀ is the row before the change. The slope term is 1 on the first changed row.`,
          s.model.kind === 'count' || e.model.kind !== 'continuous'
            ? `N is ${s.model.kind === 'count' ? (s.model.exposure?.name ?? 'one') : 'one'}, the exposure, entering as an offset with coefficient one. The dispersion is estimated from Pearson residuals (quasi-Poisson), not fixed at one.`
            : errorsDefinition(e.model.errors),
          s.seasonal.kind === 'none'
            ? 'No seasonal terms are included.'
            : `The harmonic terms are ${s.seasonal.pairs} sine and cosine pairs at period ${number(s.seasonal.period)}, used to represent the seasonal cycle.`,
        ],
        fitted: {
          kind: 'available',
          title: 'Fitted coefficients',
          expressions: [
            expression(
              fitted,
              e.terms.map((term) => `${term.name} = ${number(term.coefficient)}`).join('; '),
            ),
          ],
          explanation: count
            ? 'Exponentiating the level coefficient gives its rate ratio. Exponentiating the slope-change coefficient compares post-change and pre-change per-row rate multipliers.'
            : 'Level coefficients use the units of the series. Trend and slope-change coefficients use those units per row.',
        },
        reference:
          'Lopez Bernal, Cummins and Gasparrini (2017), International Journal of Epidemiology 46(1), 348–355, equation (1) and the additional material; the slope change is parameterised as in the 2020 corrigendum.',
      }
    }
    default:
      return assertNever(run)
  }
}
