import { brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'

export type MethodId = Brand<string, 'MethodId'>
export type MethodCaveatId = Brand<string, 'MethodCaveatId'>

export type MethodFamily = 'diagnostic' | 'discovery' | 'identification' | 'estimation' | 'refuter'

export type MethodSource =
  | {
      readonly kind: 'reference-implementation'
      readonly repository: 'statsmodels' | 'tigramite'
      readonly revision: string
      readonly locator: string
    }
  | { readonly kind: 'paper'; readonly title: string; readonly locator: string }
  | { readonly kind: 'hirmos-constraint'; readonly locator: string }

export type CaveatCategory =
  | 'sampling-structure'
  | 'stationarity-and-dynamics'
  | 'identification'
  | 'functional-form'
  | 'noise-and-dependence'
  | 'missingness'
  | 'finite-sample'
  | 'computation'
  | 'interpretation'

export interface MethodCaveat {
  readonly id: MethodCaveatId
  readonly category: CaveatCategory
  readonly requirement: string
  readonly consequenceIfUnmet: string
  readonly sources: NonEmptyArray<MethodSource>
}

export interface MethodDefinition {
  readonly id: MethodId
  readonly name: string
  readonly family: MethodFamily
  readonly summary: string
  readonly caveats: NonEmptyArray<MethodCaveat>
}

export type CaveatEvaluation =
  | { readonly kind: 'satisfied'; readonly caveat: MethodCaveat; readonly evidence: string }
  | { readonly kind: 'unresolved'; readonly caveat: MethodCaveat; readonly missingEvidence: string }
  | { readonly kind: 'violated'; readonly caveat: MethodCaveat; readonly evidence: string }

export type MethodEligibility =
  | {
      readonly kind: 'eligible'
      readonly satisfied: readonly Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[]
    }
  | {
      readonly kind: 'caution'
      readonly satisfied: readonly Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[]
      readonly unresolved: NonEmptyArray<Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>>
    }
  | {
      readonly kind: 'refused'
      readonly violations: NonEmptyArray<Extract<CaveatEvaluation, { readonly kind: 'violated' }>>
    }

export type MethodLookupProblem = { readonly kind: 'unknown-method'; readonly id: MethodId }

const methodId = (value: string): MethodId => brand<string, 'MethodId'>(value)
const caveatId = (value: string): MethodCaveatId => brand<string, 'MethodCaveatId'>(value)

const STATSMODELS_REVISION = '9307ef1b3a975a3807009432cbb489c4ae6f5c60'
const TIGRAMITE_REVISION = 'ff3ff13e1481073b8c5833a6fde1c304627a208e'

const statsmodels = (locator: string): MethodSource => ({
  kind: 'reference-implementation',
  repository: 'statsmodels',
  revision: STATSMODELS_REVISION,
  locator,
})

const tigramite = (locator: string): MethodSource => ({
  kind: 'reference-implementation',
  repository: 'tigramite',
  revision: TIGRAMITE_REVISION,
  locator,
})

const hirmos = (locator: string): MethodSource => ({ kind: 'hirmos-constraint', locator })

export const ADF_METHOD_ID = methodId('adf')
export const KPSS_METHOD_ID = methodId('kpss')
export const ZIVOT_ANDREWS_METHOD_ID = methodId('zivot-andrews')
export const GRANGER_SSR_F_METHOD_ID = methodId('granger-ssr-f')
export const PCMCI_PLUS_PAR_CORR_METHOD_ID = methodId('pcmci-plus-parcorr')
export const LPCMCI_PAR_CORR_METHOD_ID = methodId('lpcmci-parcorr')
export const DYNOTEARS_METHOD_ID = methodId('dynotears')
export const OCSE_METHOD_ID = methodId('ocse')

const ADF: MethodDefinition = {
  id: ADF_METHOD_ID,
  name: 'Augmented Dickey–Fuller',
  family: 'diagnostic',
  summary: 'Tests a univariate series for a unit root under an explicit deterministic specification.',
  caveats: [
    {
      id: caveatId('adf-null'),
      category: 'interpretation',
      requirement: 'Interpret the null as a unit root; failure to reject is not proof that a unit root exists.',
      consequenceIfUnmet: 'Reversing the null turns inconclusive evidence into a false stationarity verdict.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:233-355')],
    },
    {
      id: caveatId('adf-deterministic'),
      category: 'stationarity-and-dynamics',
      requirement: 'Choose the constant/trend specification and lag-selection policy explicitly.',
      consequenceIfUnmet: 'A misspecified deterministic term or inadequate lag order can change the test statistic and inference.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:233-355')],
    },
    {
      id: caveatId('adf-dense'),
      category: 'missingness',
      requirement: 'Supply a finite dense univariate sequence on the accepted time grid.',
      consequenceIfUnmet: 'The Hirmos kernel refuses the run rather than compressing time or silently dropping observations.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#validate_stationarity_values')],
    },
  ],
}

const KPSS: MethodDefinition = {
  id: KPSS_METHOD_ID,
  name: 'KPSS',
  family: 'diagnostic',
  summary: 'Tests level or trend stationarity using a Newey–West long-run variance estimate.',
  caveats: [
    {
      id: caveatId('kpss-null'),
      category: 'interpretation',
      requirement: 'Interpret the null as level or trend stationarity according to the selected specification.',
      consequenceIfUnmet: 'Treating the null as a unit root reverses the conclusion.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3051-3175')],
    },
    {
      id: caveatId('kpss-pvalue-bounds'),
      category: 'finite-sample',
      requirement: 'Treat boundary p-values as table bounds rather than precise tail probabilities.',
      consequenceIfUnmet: 'A reported 0.01 or 0.10 can be overinterpreted even though statsmodels clips outside its interpolation table.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3051-3175')],
    },
    {
      id: caveatId('kpss-missing'),
      category: 'missingness',
      requirement: 'Resolve missing observations without collapsing the time grid.',
      consequenceIfUnmet: 'The reference does not handle missing values and row deletion can change temporal adjacency.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3051-3175')],
    },
  ],
}

const ZIVOT_ANDREWS: MethodDefinition = {
  id: ZIVOT_ANDREWS_METHOD_ID,
  name: 'Zivot–Andrews',
  family: 'diagnostic',
  summary: 'Tests a unit-root null while allowing one endogenous level or trend break.',
  caveats: [
    {
      id: caveatId('za-one-break'),
      category: 'stationarity-and-dynamics',
      requirement: 'The one-break model must be a plausible description of the series.',
      consequenceIfUnmet: 'Multiple breaks or changing regimes are not resolved by a one-break rejection.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3548-3945')],
    },
    {
      id: caveatId('za-null'),
      category: 'interpretation',
      requirement: 'Interpret the null as a unit root with a single structural break.',
      consequenceIfUnmet: 'A rejection supports stationarity around the selected break; it does not establish general stability.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3866-3945')],
    },
    {
      id: caveatId('za-baum-approximation'),
      category: 'computation',
      requirement: 'Record that statsmodels chooses autolag once on the base model rather than at every breakpoint.',
      consequenceIfUnmet: 'Results can differ from the original per-breakpoint Zivot–Andrews procedure.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3914-3928')],
    },
  ],
}

const GRANGER_SSR_F: MethodDefinition = {
  id: GRANGER_SSR_F_METHOD_ID,
  name: 'Granger SSR F test',
  family: 'diagnostic',
  summary: 'Tests whether past candidate-cause values add predictive information beyond the target’s own past.',
  caveats: [
    {
      id: caveatId('granger-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows must be an ordered, regularly sampled time series; independent observations are not a temporal sequence.',
      consequenceIfUnmet: 'Lagged predictors would be formed from arbitrary neighboring rows and the test would have no temporal meaning.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:2368-2445')],
    },
    {
      id: caveatId('granger-predictive-not-interventional'),
      category: 'interpretation',
      requirement: 'Interpret rejection as lagged predictive precedence, not intervention causality.',
      consequenceIfUnmet: 'Common causes, omitted dynamics, or measurement timing can be mislabeled as a causal effect.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:2368-2445')],
    },
    {
      id: caveatId('granger-lag-order'),
      category: 'stationarity-and-dynamics',
      requirement: 'Use a justified lag range on an accepted stable-series route.',
      consequenceIfUnmet: 'Too few lags leave serial structure in the residuals; too many consume power and can produce unstable fits.',
      sources: [statsmodels('docs/source/vector_ar.rst:80-95'), hirmos('crates/causal-core/src/tsdiag.rs#granger_ssr_ftest')],
    },
    {
      id: caveatId('granger-finite-dense'),
      category: 'finite-sample',
      requirement: 'Use finite paired observations with more than 3 × maxLag + constant rows.',
      consequenceIfUnmet: 'The unrestricted lag model lacks adequate residual degrees of freedom and is refused.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:2368-2475')],
    },
    {
      id: caveatId('granger-multiple-lags'),
      category: 'interpretation',
      requirement: 'Treat testing several lag orders as multiple related hypotheses.',
      consequenceIfUnmet: 'Selecting the smallest unadjusted p-value across lags inflates false-positive risk.',
      sources: [hirmos('DESIGN.md#sourced-assumptions-and-caveats-are-part-of-every-method')],
    },
  ],
}

const PCMCI_PLUS_PAR_CORR: MethodDefinition = {
  id: PCMCI_PLUS_PAR_CORR_METHOD_ID,
  name: 'PCMCI+ with ParCorr',
  family: 'discovery',
  summary: 'Discovers lagged and contemporaneous conditional-dependence structure in autocorrelated time series.',
  caveats: [
    {
      id: caveatId('pcmciplus-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows must be an ordered time series on the declared sampling grid.',
      consequenceIfUnmet: 'The constructed lagged variables connect unrelated rows and the discovered time graph is invalid.',
      sources: [tigramite('tigramite/data_processing.py:481-881'), tigramite('tigramite/pcmci.py:47-90')],
    },
    {
      id: caveatId('pcmciplus-causal-stationarity'),
      category: 'stationarity-and-dynamics',
      requirement: 'Assume the causal time-series graph is stationary over the analyzed window.',
      consequenceIfUnmet: 'A single repeated lag graph can average over incompatible regimes or changing edges.',
      sources: [tigramite('README.md:18-27'), tigramite('tigramite/pcmci.py:75-90')],
    },
    {
      id: caveatId('pcmciplus-no-hidden'),
      category: 'identification',
      requirement: 'Assume no hidden common causes among the modeled variables.',
      consequenceIfUnmet: 'Orientations from PCMCI+ can be confounded; LPCMCI is the latent-variable discovery route.',
      sources: [tigramite('README.md:20-27')],
    },
    {
      id: caveatId('parcorr-linear-gaussian'),
      category: 'functional-form',
      requirement: 'Use univariate continuous variables with linear dependencies and Gaussian noise for ParCorr.',
      consequenceIfUnmet: 'Conditional-independence p-values and edge decisions may be invalid even when the algorithm completes.',
      sources: [tigramite('README.md:30-40'), tigramite('tigramite/independence_tests/parcorr.py:15-40')],
    },
    {
      id: caveatId('pcmciplus-cpdag'),
      category: 'interpretation',
      requirement: 'Preserve the returned time-series CPDAG marks and unresolved contemporaneous orientations.',
      consequenceIfUnmet: 'Turning every reported adjacency into a directed causal arrow asserts information the method did not identify.',
      sources: [tigramite('README.md:20-27')],
    },
    {
      id: caveatId('pcmciplus-browser-dense'),
      category: 'missingness',
      requirement: 'The current browser command requires a finite dense matrix; Tigramite-mask execution is a pending separate command.',
      consequenceIfUnmet: 'The run is refused rather than silently compressing the time axis.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#pcmci_plus')],
    },
  ],
}

const LPCMCI_PAR_CORR: MethodDefinition = {
  id: LPCMCI_PAR_CORR_METHOD_ID,
  name: 'LPCMCI with ParCorr',
  family: 'discovery',
  summary: 'Discovers a lagged partial ancestral graph while allowing latent common causes.',
  caveats: [
    {
      id: caveatId('lpcmci-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows must be an ordered time series on the declared sampling grid.',
      consequenceIfUnmet: 'Lagged variables connect unrelated rows and the partial ancestral graph has no temporal meaning.',
      sources: [tigramite('tigramite/lpcmci.py:28-83'), tigramite('tigramite/data_processing.py:481-881')],
    },
    {
      id: caveatId('lpcmci-stationary-graph'),
      category: 'stationarity-and-dynamics',
      requirement: 'Assume the causal graph and relevant dependencies are stable over the analyzed window.',
      consequenceIfUnmet: 'One repeated graph can combine incompatible regimes and misleading endpoint marks.',
      sources: [tigramite('tigramite/lpcmci.py:28-83')],
    },
    {
      id: caveatId('lpcmci-markov-faithfulness'),
      category: 'identification',
      requirement: 'The time-series causal Markov and faithfulness assumptions must be scientifically plausible.',
      consequenceIfUnmet: 'Conditional independences need not identify the intended ancestral relations, even though latent confounding is allowed.',
      sources: [tigramite('tigramite/lpcmci.py:28-83')],
    },
    {
      id: caveatId('lpcmci-parcorr-form'),
      category: 'functional-form',
      requirement: 'ParCorr requires continuous variables with linear conditional relationships and suitable residual behavior.',
      consequenceIfUnmet: 'The conditional-independence p-values and therefore PAG marks can be invalid.',
      sources: [tigramite('tigramite/independence_tests/parcorr.py:15-40')],
    },
    {
      id: caveatId('lpcmci-pag-interpretation'),
      category: 'interpretation',
      requirement: 'Interpret circles, tails, and arrowheads as partial ancestral graph marks, not as a fully oriented DAG.',
      consequenceIfUnmet: 'Unresolved orientation or possible latent confounding is silently promoted to a definite causal arrow.',
      sources: [tigramite('tigramite/lpcmci.py:28-83'), hirmos('crates/causal-core/src/lpcmci.rs')],
    },
    {
      id: caveatId('lpcmci-browser-dense'),
      category: 'missingness',
      requirement: 'The current browser command requires a finite dense matrix.',
      consequenceIfUnmet: 'The run is refused rather than silently compressing time.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#lpcmci_evidence')],
    },
  ],
}

const DYNOTEARS: MethodDefinition = {
  id: DYNOTEARS_METHOD_ID,
  name: 'DYNOTEARS',
  family: 'discovery',
  summary: 'Fits a sparse linear dynamic structural equation model with an acyclic contemporaneous graph.',
  caveats: [
    {
      id: caveatId('dynotears-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows must be ordered observations on a regular time grid.',
      consequenceIfUnmet: 'The explicit lag design represents arbitrary row neighbors instead of temporal effects.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#dynotears_evidence')],
    },
    {
      id: caveatId('dynotears-linear-sem'),
      category: 'functional-form',
      requirement: 'A sparse linear dynamic structural equation model must be a useful approximation.',
      consequenceIfUnmet: 'Nonlinear or dense dependencies can be omitted, distorted, or assigned misleading weights.',
      sources: [{ kind: 'paper', title: 'DYNOTEARS: Structure Learning from Time-Series Data', locator: 'arXiv:2002.00498' }],
    },
    {
      id: caveatId('dynotears-stable-window'),
      category: 'stationarity-and-dynamics',
      requirement: 'Use a window where the dynamic coefficients and contemporaneous structure are stable.',
      consequenceIfUnmet: 'One coefficient matrix averages changing regimes and is not a faithful description of either regime.',
      sources: [{ kind: 'paper', title: 'DYNOTEARS: Structure Learning from Time-Series Data', locator: 'arXiv:2002.00498' }],
    },
    {
      id: caveatId('dynotears-penalty-sensitivity'),
      category: 'computation',
      requirement: 'Treat lambdaW, lambdaA, and any later display threshold as sensitivity choices.',
      consequenceIfUnmet: 'A tuning choice is mistaken for uniquely identified graph structure.',
      sources: [{ kind: 'paper', title: 'DYNOTEARS: Structure Learning from Time-Series Data', locator: 'arXiv:2002.00498' }, hirmos('crates/causal-core/src/dynotears.rs')],
    },
    {
      id: caveatId('dynotears-weight-interpretation'),
      category: 'interpretation',
      requirement: 'Interpret the returned matrices as fitted candidate structure, not intervention effects.',
      consequenceIfUnmet: 'Optimization weights are promoted to causal effects without a defended identification argument.',
      sources: [hirmos('crates/causal-core/src/dynotears.rs')],
    },
    {
      id: caveatId('dynotears-browser-boundary'),
      category: 'missingness',
      requirement: 'The current browser command accepts 2–12 finite dense variables and lags 1–6.',
      consequenceIfUnmet: 'The run is refused instead of imputing or silently reducing the matrix.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#dynotears_evidence')],
    },
  ],
}

const OCSE: MethodDefinition = {
  id: OCSE_METHOD_ID,
  name: 'Optimal Causation Entropy',
  family: 'discovery',
  summary: 'Selects lagged predictors by forward and backward conditional mutual-information tests.',
  caveats: [
    {
      id: caveatId('ocse-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows must be ordered observations on a regular time grid.',
      consequenceIfUnmet: 'Lagged candidates connect arbitrary neighbors and conditional information has no temporal interpretation.',
      sources: [hirmos('crates/causal-core/src/ocse.rs#discover_network')],
    },
    {
      id: caveatId('ocse-information-estimator'),
      category: 'functional-form',
      requirement: 'Choose an information estimator appropriate to the dependency shape and sample size.',
      consequenceIfUnmet: 'Gaussian CMI misses nonlinear dependence; k-nearest-neighbor CMI can be unstable in small or high-dimensional samples.',
      sources: [{ kind: 'paper', title: 'Optimal causation entropy principle for causal network reconstruction', locator: 'Phys. Rev. Lett. 112, 138701 (2014)' }, hirmos('crates/causal-core/src/ocse.rs#cmi')],
    },
    {
      id: caveatId('ocse-lag-and-stability'),
      category: 'stationarity-and-dynamics',
      requirement: 'Use a defensible maximum lag and a window with stable dependency structure.',
      consequenceIfUnmet: 'Relevant parents may be omitted or a changing process may be represented as one network.',
      sources: [{ kind: 'paper', title: 'Optimal causation entropy principle for causal network reconstruction', locator: 'Phys. Rev. Lett. 112, 138701 (2014)' }],
    },
    {
      id: caveatId('ocse-permutation-resolution'),
      category: 'finite-sample',
      requirement: 'Interpret p-values at the resolution permitted by the recorded shuffle count and seed.',
      consequenceIfUnmet: 'A coarse Monte Carlo p-value is treated as a precise tail probability.',
      sources: [hirmos('crates/causal-core/src/ocse.rs#shuffle_test')],
    },
    {
      id: caveatId('ocse-corrected-semantics'),
      category: 'computation',
      requirement: 'Record that Hirmos excludes duplicate target self-lags already present in the initial conditioning set.',
      consequenceIfUnmet: 'Results are incorrectly described as reproducing the reference package’s legacy singular-candidate behavior.',
      sources: [hirmos('crates/causal-core/src/ocse.rs#discover_network')],
    },
    {
      id: caveatId('ocse-not-interventional'),
      category: 'interpretation',
      requirement: 'Interpret selected edges as conditional-information evidence, not intervention causality.',
      consequenceIfUnmet: 'Predictive information pathways can be mistaken for identified causal effects.',
      sources: [{ kind: 'paper', title: 'Optimal causation entropy principle for causal network reconstruction', locator: 'Phys. Rev. Lett. 112, 138701 (2014)' }],
    },
  ],
}

export const METHOD_CATALOG: NonEmptyArray<MethodDefinition> = [
  ADF,
  KPSS,
  ZIVOT_ANDREWS,
  GRANGER_SSR_F,
  PCMCI_PLUS_PAR_CORR,
  LPCMCI_PAR_CORR,
  DYNOTEARS,
  OCSE,
]

export const STATIONARITY_METHODS: NonEmptyArray<MethodDefinition> = [ADF, KPSS, ZIVOT_ANDREWS]
export const TEMPORAL_DISCOVERY_METHODS: NonEmptyArray<MethodDefinition> = [
  GRANGER_SSR_F,
  PCMCI_PLUS_PAR_CORR,
  LPCMCI_PAR_CORR,
  DYNOTEARS,
  OCSE,
]

export function methodDefinition(id: MethodId): Result<MethodDefinition, MethodLookupProblem> {
  const definition = METHOD_CATALOG.find((candidate) => candidate.id === id)
  return definition ? ok(definition) : err({ kind: 'unknown-method', id })
}
