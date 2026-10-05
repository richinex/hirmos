import { assertNever } from './dop'
import type { ParametricSurvivalFamily, SurvivalRunArtifact } from './survival'

export interface SurvivalFormula {
  readonly tex: string
  readonly plain: string
}

export interface SurvivalEquations {
  readonly general: readonly SurvivalFormula[]
  readonly fitted: readonly SurvivalFormula[]
  readonly definitions: readonly EquationExplanation[]
}

export type EquationExplanation =
  | string
  | {
      readonly kind: 'equation'
      readonly title: string
      readonly formula: SurvivalFormula
      readonly description: string
    }

const formula = (tex: string, plain: string): SurvivalFormula => ({ tex, plain })
// Retain small nonzero coefficients; fixed decimal formatting would turn them into zero.
const number = (value: number): string => Number(value.toPrecision(7)).toString()
const texNumber = (value: number): string => number(value).replace(/e([+-]?\d+)/, '\\times 10^{$1}')
const plainSymbol = (symbol: string) =>
  symbol
    .replace(/\\widehat/g, 'estimated ')
    .replace(/\\mathrm\{([^}]+)\}/g, '$1')
    .replace(/\\/g, '')
    .replace(/[{}]/g, '')
const numeric = (symbol: string, value: number) =>
  formula(`${symbol}=${texNumber(value)}`, `${plainSymbol(symbol)} = ${number(value)}`)
const predictor = (
  values: readonly number[],
  means?: readonly number[],
  varying = false,
): SurvivalFormula => {
  const argument = varying ? '(t)' : ''
  const term = (i: number, tex: boolean) => {
    const x = tex ? `x_{${i + 1}}${argument}` : `x${i + 1}${argument}`
    return means === undefined ? x : `(${x} - ${tex ? texNumber(means[i]!) : number(means[i]!)})`
  }
  return formula(
    `\\widehat\\eta${argument}=${values.map((v, i) => `(${texNumber(v)})${term(i, true)}`).join('+') || '0'}`,
    `eta${argument} = ${values.map((v, i) => `(${number(v)}) × ${term(i, false)}`).join(' + ') || '0'}`,
  )
}

interface DistributionEquation {
  readonly parameters: readonly string[]
  readonly location: number
  readonly link: 'log' | 'identity'
  readonly survival: SurvivalFormula
  readonly definitions: readonly string[]
}

// Parameter order and location links: flexsurv 2.3.2 R/distributions.R, flexsurv.dists.
export function survivalDistributionEquation(
  family: ParametricSurvivalFamily,
): DistributionEquation {
  switch (family) {
    case 'exponential':
      return {
        parameters: ['r'],
        location: 0,
        link: 'log',
        survival: formula('S(t\\mid x)=\\exp[-r(x)t]', 'S(t | x) = exp(−r(x)t)'),
        definitions: ['r(x) is the event rate.'],
      }
    case 'weibull':
      return {
        parameters: ['k', 's'],
        location: 1,
        link: 'log',
        survival: formula('S(t\\mid x)=\\exp[-(t/s(x))^k]', 'S(t | x) = exp(−(t/s(x))^k)'),
        definitions: ['k is the shape and s(x) is the time scale of the Weibull AFT model.'],
      }
    case 'weibullPh':
      return {
        parameters: ['k', 'm'],
        location: 1,
        link: 'log',
        survival: formula('S(t\\mid x)=\\exp[-m(x)t^k]', 'S(t | x) = exp(−m(x)t^k)'),
        definitions: [
          'k is the shape. m(x) multiplies the cumulative hazard; it is the Weibull PH scale parameter, not a time scale.',
        ],
      }
    case 'logNormal':
      return {
        parameters: ['\\mu', '\\sigma'],
        location: 0,
        link: 'identity',
        survival: formula(
          'S(t\\mid x)=1-\\Phi((\\log t-\\mu(x))/\\sigma)',
          'S(t | x) = 1 − Φ((log(t) − mu(x)) / sigma)',
        ),
        definitions: [
          'mu(x) and sigma are the mean and standard deviation of log time. Φ is the standard normal cumulative distribution function.',
        ],
      }
    case 'gamma':
      return {
        parameters: ['k', 'r'],
        location: 1,
        link: 'log',
        survival: formula(
          'S(t\\mid x)=\\Gamma(k,r(x)t)/\\Gamma(k)',
          'S(t | x) = upper incomplete gamma(k, r(x)t) / gamma(k)',
        ),
        definitions: [
          'k is the shape and r(x) is the rate. The numerator is the upper incomplete gamma function.',
        ],
      }
    case 'gompertz':
      return {
        parameters: ['a', 'r'],
        location: 1,
        link: 'log',
        survival: formula(
          'S(t\\mid x)=\\exp[-r(x)\\int_0^t e^{au}\\,du]',
          'S(t | x) = exp(−r(x) integral from 0 to t of exp(a u) du)',
        ),
        definitions: [
          'a is the shape and r(x) is the rate at time zero. The integral is (exp(a t) − 1)/a when a is nonzero, and t when a is zero.',
        ],
      }
    case 'logLogistic':
      return {
        parameters: ['k', 's'],
        location: 1,
        link: 'log',
        survival: formula('S(t\\mid x)=[1+(t/s(x))^k]^{-1}', 'S(t | x) = 1 / (1 + (t/s(x))^k)'),
        definitions: ['k is the shape and s(x) is the median event time.'],
      }
    case 'generalizedGamma':
      return {
        parameters: ['\\mu', '\\sigma', 'Q'],
        location: 0,
        link: 'identity',
        survival: formula(
          'S(t\\mid x)=1-F_{\\mathrm{GG}}(t;\\mu(x),\\sigma,Q)',
          'S(t | x) = 1 − F_GG(t; mu(x), sigma, Q)',
        ),
        definitions: [
          'F_GG is the generalized gamma cumulative distribution function in the Prentice parameterisation. mu is its location, sigma its positive scale, and Q its shape parameter.',
        ],
      }
    case 'generalizedF':
      return {
        parameters: ['\\mu', '\\sigma', 'Q', 'P'],
        location: 0,
        link: 'identity',
        survival: formula(
          'S(t\\mid x)=1-F_{\\mathrm{GF}}(t;\\mu(x),\\sigma,Q,P)',
          'S(t | x) = 1 − F_GF(t; mu(x), sigma, Q, P)',
        ),
        definitions: [
          'F_GF is the generalized F cumulative distribution function in flexsurv’s stable parameterisation. mu is its location, sigma its positive scale, and Q and P its shape parameters, with P positive.',
        ],
      }
    default:
      return assertNever(family)
  }
}

function parametric(
  run: Extract<
    SurvivalRunArtifact,
    { kind: 'right-censored-survival-run' | 'start-stop-survival-run' }
  >,
): SurvivalEquations {
  const { evidence } = run
  const model = survivalDistributionEquation(evidence.family)
  const symbol = model.parameters[model.location]!
  const baseline = evidence.naturalBaseline[model.location]!
  const linked = model.link === 'log' ? `\\log ${symbol}(x)` : `${symbol}(x)`
  const fitted = [
    predictor(evidence.coefficients),
    formula(
      `${linked}=${texNumber(model.link === 'log' ? Math.log(baseline) : baseline)}+\\widehat\\eta`,
      `${model.link === 'log' ? 'log ' : ''}${plainSymbol(symbol)}(x) = ${number(model.link === 'log' ? Math.log(baseline) : baseline)} + eta`,
    ),
  ]
  model.parameters.forEach((parameter, index) => {
    if (index !== model.location) fitted.push(numeric(parameter, evidence.naturalBaseline[index]!))
  })
  const definitions: EquationExplanation[] = [
    ...model.definitions,
    'The parameters without x are constant across covariate values. Substitute the fitted parameter expressions into the survival equation.',
  ]
  if (run.kind === 'start-stop-survival-run')
    definitions.push({
      kind: 'equation',
      title: 'Contribution of one interval',
      formula: formula(
        'L_i=h(t_i\\mid x_i)^{\\delta_i}\\,\\frac{S(t_i\\mid x_i)}{S(a_i\\mid x_i)}',
        'L_i = h(stop | x_i)^event × S(stop | x_i) / S(start | x_i)',
      ),
      description:
        'aᵢ is the interval start and tᵢ is its stop. δᵢ is 1 when the event occurs at the stop, and 0 otherwise. Covariates are constant within the interval. Dividing by survival at the start accounts for having survived to that time. A changing-covariate survival curve requires the full covariate history; the displayed curve uses the recorded fixed profile.',
    })
  return {
    general: [
      model.survival,
      formula(
        `${linked}=b_0+\\sum_{j=1}^p b_jx_j`,
        `${model.link === 'log' ? 'log ' : ''}${symbol}(x) = b0 + sum of b_j x_j`,
      ),
    ],
    fitted,
    definitions,
  }
}

/** Derive equations exclusively from the configuration and evidence saved with a run. */
export function survivalEquations(run: SurvivalRunArtifact): SurvivalEquations {
  switch (run.kind) {
    case 'right-censored-survival-run':
    case 'start-stop-survival-run':
      return parametric(run)
    case 'cox-regression-run': {
      const { configuration, evidence } = run
      const varying = configuration.observation.kind === 'start-stop'
      const time = varying ? '(t)' : ''
      const baseline = configuration.strata.kind === 'column' ? 'h_{0,s}(t)' : 'h_0(t)'
      const frailty = evidence.frailty.kind === 'gamma'
      const general = [
        formula(
          `h(t\\mid x${time})=${frailty ? 'z_g' : ''}${baseline}\\exp[\\eta${time}]`,
          `h(t | x${time}) = ${frailty ? 'z_g × ' : ''}${configuration.strata.kind === 'column' ? 'h_0,s(t)' : 'h_0(t)'} × exp(eta${time})`,
        ),
        formula(
          `\\eta${time}=\\sum_{j=1}^p\\beta_j(x_j${time}-\\bar x_j)`,
          `eta${time} = sum of beta_j × (x_j${time} − mean_j)`,
        ),
      ]
      const fitted = [
        predictor(
          evidence.coefficients.map((c) => c.coefficient),
          evidence.covariateMeans,
          varying,
        ),
      ]
      if (evidence.fitting.kind === 'clusteredBreslow') {
        general[1] = formula(
          '\\eta=\\sum_{j=1}^p\\beta_j(x_j-c_j)',
          'eta = sum of beta_j × (x_j − centring value_j)',
        )
      }
      const definitions = [
        'h is the event rate among observations still event-free at time t. The baseline rate is defined at the recorded centring values shown in the fitted expression. For a 1-unit covariate increase, exp(beta) is the hazard ratio, holding the other covariates fixed.',
      ]
      if (configuration.strata.kind === 'column')
        definitions.push(
          `s identifies a stratum of ${configuration.strata.column.name}; each stratum has its own baseline rate.`,
        )
      if (varying)
        definitions.push(
          'x(t) uses the covariate values recorded for the interval containing time t.',
        )
      if (evidence.frailty.kind === 'gamma') {
        general.push(
          formula(
            'E[z_g]=1,\\quad\\operatorname{Var}(z_g)=\\theta',
            'Mean group frailty = 1; variance = theta',
          ),
        )
        fitted.push(numeric('\\widehat\\theta', evidence.frailty.theta))
        definitions.push(
          'z_g is the shared gamma frailty for a group. Group-specific frailty estimates are not retained in this result, so the fitted expression leaves z_g unspecified.',
        )
      }
      return { general, fitted, definitions }
    }
    case 'penalized-aft-run': {
      const { evidence } = run
      const model = survivalDistributionEquation(evidence.family)
      return {
        general: [
          model.survival,
          formula('\\log s(x)=b_0+\\sum_{j=1}^p b_jx_j', 'log s(x) = b0 + sum of b_j x_j'),
        ],
        fitted: [
          predictor(evidence.coefficients.map((c) => c.coefficient)),
          formula(
            `\\log s(x)=${texNumber(evidence.intercept.coefficient)}+\\widehat\\eta`,
            `log s(x) = ${number(evidence.intercept.coefficient)} + eta`,
          ),
          numeric('k', Math.exp(evidence.ancillary.coefficient)),
        ],
        definitions: [
          ...model.definitions,
          'The coefficients are the fitted penalised estimates. The shape is constant across observations. exp(b_j) is the time ratio for a 1-unit increase in x_j, holding the other covariates fixed.',
        ],
      }
    }
    case 'aalen-run': {
      const time = run.evidence.lastTime
      const values = run.evidence.curves.map((curve) => curve.at(-1)![1])
      return {
        general: [
          formula(
            'H(t\\mid x)=A_0(t)+\\sum_{j=1}^p A_j(t)x_j',
            'H(t | x) = A0(t) + sum of A_j(t) x_j',
          ),
        ],
        fitted: [
          formula(
            `\\widehat H(${texNumber(time)}\\mid x)=${texNumber(values[0]!)}+${values
              .slice(1)
              .map((v, i) => `(${texNumber(v)})x_{${i + 1}}`)
              .join('+')}`,
            `H(${number(time)} | x) = ${number(values[0]!)} + ${values
              .slice(1)
              .map((v, i) => `(${number(v)}) × x${i + 1}`)
              .join(' + ')}`,
          ),
        ],
        definitions: [
          'H is cumulative hazard. A0 is the cumulative intercept and each A_j is a cumulative coefficient curve. The fitted expression is evaluated only at the last fitted time; the coefficients vary over follow-up. The weighted summary-table coefficients are not substituted into this equation.',
        ],
      }
    }
    case 'survival-forest-run':
      return {
        general: [
          formula(
            '\\widehat H(t\\mid x)=B^{-1}\\sum_{b=1}^B\\widehat H_b(t\\mid x)',
            'H(t | x) = average of the B trees’ cumulative hazards',
          ),
          formula(
            '\\widehat S(t\\mid x)=\\exp[-\\widehat H(t\\mid x)]',
            'S(t | x) = exp(−H(t | x))',
          ),
        ],
        fitted: [numeric('B', run.evidence.trees)],
        definitions: [
          `Each tree uses the cumulative hazard in the terminal node reached by the covariate values. The displayed prediction uses prepared row ${run.evidence.predictionRow + 1}. The forest has no regression coefficient equation; permutation importance values are not coefficients.`,
        ],
      }
    case 'nonparametric-survival-run':
      return {
        general: [
          formula(
            '\\widehat S(t)=\\prod_{t_i\\le t}(1-d_i/n_i)',
            'Kaplan–Meier S(t) = product over event times of (1 − events / number at risk)',
          ),
          run.configuration.ties === 'discrete'
            ? formula(
                '\\widehat H(t)=\\sum_{t_i\\le t}d_i/n_i',
                'Nelson–Aalen H(t) = sum over event times of events / number at risk',
              )
            : formula(
                '\\widehat H(t)=\\sum_{t_i\\le t}\\sum_{k=0}^{d_i-1}(n_i-k)^{-1}',
                'Smoothed Nelson–Aalen H(t) = sum over event times of sum from k = 0 to d_i − 1 of 1/(n_i − k)',
              ),
        ],
        fitted: [],
        definitions: [
          'd_i counts events at time t_i; n_i counts observations at risk immediately before that time. Counts include the recorded row frequencies when supplied. The table and curves contain the fitted values. Kaplan–Meier survival and Nelson–Aalen cumulative hazard are separate estimates; one is not obtained by exponentiating the other.',
        ],
      }
    case 'two-group-survival-run': {
      const general = [
        formula(
          '\\widehat S_g(t)=\\prod_{t_i\\le t}(1-d_{gi}/n_{gi})',
          'Group g Kaplan–Meier S_g(t) = product of (1 − group events / group number at risk)',
        ),
        formula(
          '\\Delta_{\\mathrm{RMST}}(\\tau)=\\int_0^\\tau[\\widehat S_1(t)-\\widehat S_0(t)]\\,dt',
          'Restricted mean difference = integral through tau of (S1 − S0)',
        ),
      ]
      const fitted = [
        numeric('\\tau', run.evidence.truncationTime),
        numeric('\\widehat\\Delta_{\\mathrm{RMST}}', run.evidence.restrictedMeanDifference),
      ]
      if (run.evidence.fixedTimeConversion.kind === 'recorded') {
        general.push(
          formula(
            '\\Delta_F(t)=[1-\\widehat S_1(t)]-[1-\\widehat S_0(t)]',
            'Conversion difference at t = (1 − S1(t)) − (1 − S0(t))',
          ),
        )
        fitted.push(
          numeric('t', run.evidence.fixedTimeConversion.result.time),
          numeric('\\widehat\\Delta_F', run.evidence.fixedTimeConversion.result.difference),
        )
      }
      return {
        general,
        fitted,
        definitions: [
          `g is the recorded group code in ${run.configuration.group.name} (0 or 1). d counts events and n counts observations at risk. Restricted mean event-free time is the area under the survival curve. Conversion is the probability that the event has occurred. Both differences use group 1 minus group 0; conversion differences are on a probability scale.`,
        ],
      }
    }
    case 'multi-state-survival-run':
      return {
        general: [
          formula(
            '\\frac{dP(t)}{dt}=P(t)Q(t),\\quad P(0)=I',
            'dP(t)/dt = P(t) Q(t), with P(0) = identity',
          ),
          formula(
            'q_{rr}(t)=-\\sum_{s\\ne r}q_{rs}(t)',
            'Each diagonal entry of Q is minus the sum of the other entries in its row',
          ),
        ],
        fitted: [],
        definitions: [
          `P_rs(t) is the probability of occupying state s at time t given state r at time zero. Q contains the fitted transition hazards; forbidden moves have hazard zero. This run uses the ${run.evidence.family} family for permitted transitions. Transition parameters are not stored in this result, so a numerical transition-hazard equation cannot be reconstructed. The saved matrices contain the fitted probabilities.`,
        ],
      }
    default:
      return assertNever(run)
  }
}

export function survivalEquationVariables(run: SurvivalRunArtifact): readonly string[] {
  switch (run.kind) {
    case 'right-censored-survival-run':
    case 'start-stop-survival-run':
    case 'cox-regression-run':
    case 'penalized-aft-run':
    case 'aalen-run':
    case 'survival-forest-run':
      return run.configuration.covariates.map((c) => c.name)
    case 'nonparametric-survival-run':
    case 'two-group-survival-run':
    case 'multi-state-survival-run':
      return []
    default:
      return assertNever(run)
  }
}
