import { brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'

export type MethodId = Brand<string, 'MethodId'>
export type MethodCaveatId = Brand<string, 'MethodCaveatId'>

export type MethodFamily = 'diagnostic' | 'discovery' | 'identification' | 'estimation' | 'refuter' | 'counterfactual'

export type MethodSource =
  | {
      readonly kind: 'reference-implementation'
      readonly repository: 'statsmodels' | 'tigramite' | 'lingam' | 'dowhy' | 'pyro' | 'ruptures' | 'pgmpy' | 'econml' | 'causationentropy' | 'causalnex' | 'synthdid'
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
  /** The method's estimand as TeX with its plain-text reading, typeset where the summary is shown. */
  readonly summaryTex?: { readonly tex: string; readonly plain: string }
  readonly caveats: NonEmptyArray<MethodCaveat>
}

export type CaveatEvaluation =
  | { readonly kind: 'satisfied'; readonly caveat: MethodCaveat; readonly evidence: string }
  | { readonly kind: 'unresolved'; readonly caveat: MethodCaveat; readonly missingEvidence: string }
  | { readonly kind: 'violated'; readonly caveat: MethodCaveat; readonly evidence: string }

/** The one unassessed condition the stage states beside the run button: integrated series in levels. The inspector row then carries only its status, so the sentence appears once. */
export const isStageNote = (evaluation: CaveatEvaluation): boolean =>
  evaluation.kind === 'unresolved' && evaluation.caveat.category === 'stationarity-and-dynamics' && evaluation.missingEvidence.includes('I(1)')

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
      readonly satisfied: readonly Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[]
      readonly unresolved: readonly Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>[]
      readonly violations: NonEmptyArray<Extract<CaveatEvaluation, { readonly kind: 'violated' }>>
    }

export type MethodLookupProblem = { readonly kind: 'unknown-method'; readonly id: MethodId }

const methodId = (value: string): MethodId => brand<string, 'MethodId'>(value)
const caveatId = (value: string): MethodCaveatId => brand<string, 'MethodCaveatId'>(value)

const STATSMODELS_REVISION = '9307ef1b3a975a3807009432cbb489c4ae6f5c60'
const TIGRAMITE_REVISION = 'ff3ff13e1481073b8c5833a6fde1c304627a208e'
const LINGAM_REVISION = '690e2aabb834459ec5238b6f947e7dcef36bd788'
const DOWHY_REVISION = '0.14'
const PYRO_REVISION = '1af296c78b8e150b3b7da4ba2f04c45617ba753b'
const RUPTURES_REVISION = 'ee1c8ff8a548d54c641b2bb471562165931f31c7'
const PGMPY_REVISION = 'a8e16403385435e5b43515b485fdd756947a0048'
const ECONML_REVISION = 'f0fc2e7d39d20e1a9d9201233fb7945e88cab0cc'
const CAUSATION_ENTROPY_REVISION = 'ae9868f36d2b315afd68a596fa59d1fd2ef39a70'
const CAUSALNEX_REVISION = '0.12.1'
const SYNTHDID_REVISION = '70c1ce3eac58e28c30b67435ca377bb48baa9b8a'

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

const lingam = (locator: string): MethodSource => ({
  kind: 'reference-implementation',
  repository: 'lingam',
  revision: LINGAM_REVISION,
  locator,
})

const dowhy = (locator: string): MethodSource => ({
  kind: 'reference-implementation',
  repository: 'dowhy',
  revision: DOWHY_REVISION,
  locator,
})

const pyro = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'pyro', revision: PYRO_REVISION, locator })
const ruptures = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'ruptures', revision: RUPTURES_REVISION, locator })
const pgmpy = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'pgmpy', revision: PGMPY_REVISION, locator })
const econml = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'econml', revision: ECONML_REVISION, locator })
const causationEntropy = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'causationentropy', revision: CAUSATION_ENTROPY_REVISION, locator })
const causalnex = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'causalnex', revision: CAUSALNEX_REVISION, locator })
const synthdid = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'synthdid', revision: SYNTHDID_REVISION, locator })

const hirmos = (locator: string): MethodSource => ({ kind: 'hirmos-constraint', locator })

export const ADF_METHOD_ID = methodId('adf')
export const KPSS_METHOD_ID = methodId('kpss')
export const ZIVOT_ANDREWS_METHOD_ID = methodId('zivot-andrews')
export const GRANGER_SSR_F_METHOD_ID = methodId('granger-ssr-f')
export const PCMCI_PLUS_PAR_CORR_METHOD_ID = methodId('pcmci-plus-parcorr')
export const LPCMCI_PAR_CORR_METHOD_ID = methodId('lpcmci-parcorr')
export const DYNOTEARS_METHOD_ID = methodId('dynotears')
export const DIRECT_LINGAM_METHOD_ID = methodId('direct-lingam')
export const VAR_LINGAM_METHOD_ID = methodId('var-lingam')
export const OCSE_METHOD_ID = methodId('ocse')
export const BACKDOOR_IDENTIFICATION_METHOD_ID = methodId('backdoor-identification')
export const GRAPHICAL_IDENTIFICATION_METHOD_ID = methodId('graphical-identification-id')
export const COUNTERFACTUAL_IDENTIFICATION_METHOD_ID = methodId('counterfactual-identification-id-star')
export const BACKDOOR_LINEAR_REGRESSION_METHOD_ID = methodId('backdoor-linear-regression')
export const FRONTDOOR_TWO_STAGE_METHOD_ID = methodId('frontdoor-two-stage')
export const POISSON_GLM_METHOD_ID = methodId('poisson-glm')
export const NEGATIVE_BINOMIAL_METHOD_ID = methodId('negative-binomial-p')
export const CAUSAL_EFFECTS_TOTAL_METHOD_ID = methodId('causal-effects-total')
export const CAUSAL_IMPACT_METHOD_ID = methodId('causal-impact')
export const DML_PLR_METHOD_ID = methodId('dml-plr')
export const DML_IRM_METHOD_ID = methodId('dml-irm')
export const DML_REFUTATION_METHOD_ID = methodId('dml-refutation-batch')
export const ARDL_PSS_METHOD_ID = methodId('ardl-pss')
export const VECM_METHOD_ID = methodId('vecm')
export const SYNTHETIC_CONTROL_METHOD_ID = methodId('synthetic-control')
export const PANEL_INTERVENTION_METHOD_ID = methodId('panel-intervention')
export const NEGBIN_NUTS_METHOD_ID = methodId('negbin-nuts')
export const BAYESIAN_GAUSSIAN_METHOD_ID = methodId('bayesian-gaussian')
export const DISCRETE_BN_METHOD_ID = methodId('discrete-bn-query')
export const BINARY_ETT_METHOD_ID = methodId('binary-ett-idc-star')
export const LINEAR_SCM_METHOD_ID = methodId('linear-scm-counterfactual')
export const PLACEBO_REFUTER_METHOD_ID = methodId('placebo-treatment-refuter')
export const DATA_SUBSET_REFUTER_METHOD_ID = methodId('data-subset-refuter')
export const RANDOM_COMMON_CAUSE_REFUTER_METHOD_ID = methodId('random-common-cause-refuter')
export const UNOBSERVED_COMMON_CAUSE_METHOD_ID = methodId('unobserved-common-cause-sensitivity')
export const LJUNG_BOX_METHOD_ID = methodId('ljung-box')
export const SHAPIRO_WILK_METHOD_ID = methodId('shapiro-wilk')
export const PELT_METHOD_ID = methodId('pelt-change-points')
export const STL_METHOD_ID = methodId('stl-decomposition')

/** A literature source verified against the publication or the pinned upstream implementation. */
const paper = (title: string, locator: string): MethodSource => ({ kind: 'paper', title, locator })

// Sources the ontology notes establish.
const RUIZ_DE_VILLA_CH7 = (locator: string): MethodSource => paper('Causal Inference for Data Science (Ruiz de Villa, Manning), chapter 7', locator)
const RUIZ_DE_VILLA_CH8 = (locator: string): MethodSource => paper('Causal Inference for Data Science (Ruiz de Villa, Manning), chapter 8', locator)
const NESS_CH4 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 4', locator)
const NESS_CH11 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 11', locator)
const NESS_CH10 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 10', locator)
const SHPITSER_PEARL_ID = paper('Identification of joint interventional distributions in recursive semi-Markovian causal models (Shpitser and Pearl, 2006)', 'AAAI 2006, ID algorithm')
const SHPITSER_PEARL_COUNTERFACTUAL = paper('Complete identification methods for the causal hierarchy (Shpitser and Pearl, 2008)', 'JMLR 9:1941–1979; ID* and IDC*')
const CINELLI_FORNEY_PEARL = paper('A crash course in good and bad controls (Cinelli, Forney and Pearl, 2022)', 'good, neutral and bad controls')
const HENCKEL_2019 = paper('Graphical criteria for efficient total effect estimation via adjustment in causal linear models (Henckel, Perković and Maathuis, 2019)', 'efficient adjustment sets')
const ROTNITZKY_SMUCLER_2019 = paper('Efficient adjustment sets for population average treatment effect estimation in nonparametric causal graphical models (Rotnitzky and Smucler, 2019)', 'efficient adjustment sets')
const CHERNOZHUKOV_DML = paper('Double/debiased machine learning for treatment and causal parameters (Chernozhukov and others, 2018)', 'arXiv:1608.00060; partially linear model, cross-fitting, root-n normality')
const BACH_DOUBLEML = paper('DoubleML: an object-oriented implementation of double machine learning in R (Bach, Chernozhukov, Kurz and Spindler)', 'arXiv:2103.09603; theorem 1')
const ROBINS_1994 = paper('Estimation of regression coefficients when some regressors are not always observed (Robins, Rotnitzky and Zhao, 1994)', 'augmented inverse probability weighting')
const KENNEDY_DR = paper('Semiparametric doubly robust targeted double machine learning: a review (Kennedy)', 'efficiency of AIPW')
const HINES_2022 = paper('Demystifying statistical learning based on efficient influence functions (Hines, Dukes, Diaz-Ordaz and Vansteelandt)', '§4.1 plug-in bias')
const RUNGE_2021 = paper('Necessary and sufficient graphical conditions for optimal adjustment sets in causal graphical models with hidden variables (Runge, NeurIPS 2021)', 'optimal adjustment set; identifiability')
const MAEDA_SHIMIZU_2021 = paper('Causal additive models with unobserved variables (Maeda and Shimizu, UAI 2021)', 'proceedings.mlr.press/v161/maeda21a')

// Primary literature checked against the publication record and, where available, the vendored package documentation.
const PEARL_2009 = (locator: string): MethodSource => paper('Causality: Models, Reasoning, and Inference, 2nd ed. (Pearl, 2009)', locator)
const HERNAN_ROBINS = paper('Causal Inference: What If (Hernán and Robins, 2020)', '§3.3, positivity')
const NEWEY_WEST_1987 = paper('A Simple, Positive Semi-definite, Heteroskedasticity and Autocorrelation Consistent Covariance Matrix (Newey and West, 1987)', 'Econometrica 55(3), 703–708')
const GRANGER_NEWBOLD_1974 = paper('Spurious regressions in econometrics (Granger and Newbold, 1974)', 'Journal of Econometrics 2(2), 111–120')
const DICKEY_FULLER_1979 = paper('Distribution of the Estimators for Autoregressive Time Series with a Unit Root (Dickey and Fuller, 1979)', 'Journal of the American Statistical Association 74(366), 427–431')
const KPSS_1992 = paper('Testing the Null Hypothesis of Stationarity against the Alternative of a Unit Root (Kwiatkowski, Phillips, Schmidt and Shin, 1992)', 'Journal of Econometrics 54(1–3), 159–178')
const ZIVOT_ANDREWS_1992 = paper('Further Evidence on the Great Crash, the Oil-Price Shock, and the Unit-Root Hypothesis (Zivot and Andrews, 1992)', 'Journal of Business & Economic Statistics 10(3), 251–270')
const GRANGER_1969 = paper('Investigating Causal Relations by Econometric Models and Cross-spectral Methods (Granger, 1969)', 'Econometrica 37(3), 424–438')
const PSS_2001 = paper('Bounds Testing Approaches to the Analysis of Level Relationships (Pesaran, Shin and Smith, 2001)', 'Journal of Applied Econometrics 16(3), 289–326; cases I–V')
const JOHANSEN_1988 = paper('Statistical Analysis of Cointegration Vectors (Johansen, 1988)', 'Journal of Economic Dynamics and Control 12(2–3), 231–254')
const CAMERON_TRIVEDI = paper('Regression Analysis of Count Data, 2nd ed. (Cameron and Trivedi, 2013)', '§§3.2–3.5; overdispersion in §3.4')
const HOFFMAN_GELMAN_2014 = paper('The No-U-Turn Sampler: Adaptively Setting Path Lengths in Hamiltonian Monte Carlo (Hoffman and Gelman, 2014)', 'Journal of Machine Learning Research 15(47), 1593–1623')
const BETANCOURT_2017 = paper('A Conceptual Introduction to Hamiltonian Monte Carlo (Betancourt, 2017)', 'arXiv:1701.02434; multinomial trajectory sampling')
const ABADIE_2010 = paper('Synthetic Control Methods for Comparative Case Studies (Abadie, Diamond and Hainmueller, 2010)', 'Journal of the American Statistical Association 105(490), 493–505')
const ABADIE_2021 = paper('Using Synthetic Controls: Feasibility, Data Requirements, and Methodological Aspects (Abadie, 2021)', 'Journal of Economic Literature 59(2), 391–425')
const CALLAWAY_SANTANNA_2021 = paper('Difference-in-Differences with multiple time periods (Callaway and Sant\'Anna, 2021)', 'Journal of Econometrics 225(2), 200–230; simultaneous adoption as the one-cohort case')
const ARKHANGELSKY_2021 = paper('Synthetic Difference-in-Differences (Arkhangelsky, Athey, Hirshberg, Imbens and Wager, 2021)', 'American Economic Review 111(12), 4088–4118')
const BRODERSEN_2015 = paper('Inferring causal impact using Bayesian structural time-series models (Brodersen and others, 2015)', 'Annals of Applied Statistics 9(1), 247–274')
const CHERNOZHUKOV_OVB = paper('Long Story Short: Omitted Variable Bias in Causal Machine Learning (Chernozhukov, Cinelli, Newey, Sharma and Syrgkanis, 2022)', 'NBER Working Paper 30302; sensitivity bounds')
const HYVARINEN_2010 = paper('Estimation of a Structural Vector Autoregression Model Using Non-Gaussianity (Hyvärinen, Zhang, Shimizu and Hoyer, 2010)', 'Journal of Machine Learning Research 11, 1709–1731')
const SHIMIZU_2011 = paper('DirectLiNGAM: A Direct Method for Learning a Linear Non-Gaussian Structural Equation Model (Shimizu and others, 2011)', 'Journal of Machine Learning Research 12, 1225–1248')
const PAMFIL_2020 = paper('DYNOTEARS: Structure Learning from Time-Series Data (Pamfil and others, 2020)', 'Proceedings of AISTATS, PMLR 108, 1595–1605')
const SUN_2015 = paper('Causal Network Inference by Optimal Causation Entropy (Sun, Taylor and Bollt, 2015)', 'SIAM Journal on Applied Dynamical Systems 14(1), 65–83')
const RUNGE_2020 = paper('Discovering contemporaneous and lagged causal relations in autocorrelated nonlinear time series datasets (Runge, 2020)', 'Proceedings of UAI, PMLR 124; PCMCI+')
const GERHARDUS_RUNGE_2020 = paper('High-recall causal discovery for autocorrelated time series with latent confounders (Gerhardus and Runge, 2020)', 'Advances in Neural Information Processing Systems 33; LPCMCI')
const LJUNG_BOX_1978 = paper('On a Measure of Lack of Fit in Time Series Models (Ljung and Box, 1978)', 'Biometrika 65(2), 297–303')
const SHAPIRO_WILK_1965 = paper('An Analysis of Variance Test for Normality (Complete Samples) (Shapiro and Wilk, 1965)', 'Biometrika 52(3/4), 591–611')
const KILLICK_2012 = paper('Optimal Detection of Changepoints with a Linear Computational Cost (Killick, Fearnhead and Eckley, 2012)', 'Journal of the American Statistical Association 107(500), 1590–1598; PELT')
const CLEVELAND_1990 = paper('STL: A Seasonal-Trend Decomposition Procedure Based on Loess (Cleveland, Cleveland, McRae and Terpenning, 1990)', 'Journal of Official Statistics 6(1), 3–73')
const HYNDMAN_FPP = paper('Forecasting: Principles and Practice, 3rd ed. (Hyndman and Athanasopoulos, 2021)', '§4.3, strength of trend and seasonality')
const ABADIE_CATTANEO_2018 = paper('Econometric Methods for Program Evaluation (Abadie and Cattaneo, 2018)', 'Annual Review of Economics 10, 465–503; doi:10.1146/annurev-economics-080217-053402; ATE and ATT target populations')
const VAN_DER_ZANDER_2014 = paper('Constructing Separators and Adjustment Sets in Ancestral Graphs (van der Zander, Liśkiewicz and Textor, 2014)', 'UAI 2014, 907–916; proper back-door graph and minimal adjustment-set construction')
const TAKATA_2010 = paper('Space-optimal, backtracking algorithms to list the minimal vertex separators of a graph (Takata, 2010)', 'Discrete Applied Mathematics 158, 1660–1667; doi:10.1016/j.dam.2010.05.013')

const ADF: MethodDefinition = {
  id: ADF_METHOD_ID,
  name: 'Augmented Dickey–Fuller',
  family: 'diagnostic',
  summary: 'Tests a series for a unit root under a stated constant or trend term.',
  caveats: [
    {
      id: caveatId('adf-null'),
      category: 'interpretation',
      requirement: 'Interpret the null as a unit root; failure to reject is not proof that a unit root exists.',
      consequenceIfUnmet: 'Inconclusive evidence is read as a stationarity verdict.',
      sources: [DICKEY_FULLER_1979, statsmodels('statsmodels/tsa/stattools/_stattools.py:233-355')],
    },
    {
      id: caveatId('adf-deterministic'),
      category: 'stationarity-and-dynamics',
      requirement: 'State the constant or trend term and the lag policy before reading the statistic.',
      consequenceIfUnmet: 'Another deterministic term or lag order gives another statistic.',
      sources: [DICKEY_FULLER_1979, statsmodels('statsmodels/tsa/stattools/_stattools.py:233-355')],
    },
  ],
}

const KPSS: MethodDefinition = {
  id: KPSS_METHOD_ID,
  name: 'KPSS',
  family: 'diagnostic',
  summary: 'Tests level or trend stationarity as the null, complementing the ADF unit-root test.',
  caveats: [
    {
      id: caveatId('kpss-null'),
      category: 'interpretation',
      requirement: 'The null is stationarity (level or trend as chosen); rejection points to a unit root.',
      consequenceIfUnmet: 'The conclusion is reversed.',
      sources: [KPSS_1992, statsmodels('statsmodels/tsa/stattools/_stattools.py:3051-3175')],
    },
    {
      id: caveatId('kpss-pvalue-bounds'),
      category: 'finite-sample',
      requirement: 'p-values of 0.01 and 0.10 are table limits, not exact tail probabilities.',
      consequenceIfUnmet: 'A tabulated boundary value is interpreted as an exact tail probability.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3051-3175')],
    },
  ],
}

const ZIVOT_ANDREWS: MethodDefinition = {
  id: ZIVOT_ANDREWS_METHOD_ID,
  name: 'Zivot–Andrews',
  family: 'diagnostic',
  summary: 'Tests a unit root while allowing one break in level, trend or both, chosen from the data.',
  caveats: [
    {
      id: caveatId('za-one-break'),
      category: 'stationarity-and-dynamics',
      requirement: 'One break describes the series; several breaks or regimes need another route.',
      consequenceIfUnmet: 'A one-break rejection is read as general stability.',
      sources: [ZIVOT_ANDREWS_1992, statsmodels('statsmodels/tsa/stattools/_stattools.py:3548-3945')],
    },
    {
      id: caveatId('za-null'),
      category: 'interpretation',
      requirement: 'The null is a unit root with one break; rejection supports stationarity around that break.',
      consequenceIfUnmet: 'The break date is read as an established event.',
      sources: [ZIVOT_ANDREWS_1992, statsmodels('statsmodels/tsa/stattools/_stattools.py:3866-3945')],
    },
    {
      id: caveatId('za-baum-approximation'),
      category: 'computation',
      requirement: 'The lag is chosen once on the base model, not at every candidate break, as statsmodels does.',
      consequenceIfUnmet: 'Results differ from the per-break procedure in the paper.',
      sources: [statsmodels('statsmodels/tsa/stattools/_stattools.py:3914-3928')],
    },
  ],
}

const GRANGER_SSR_F: MethodDefinition = {
  id: GRANGER_SSR_F_METHOD_ID,
  name: 'Granger SSR F test',
  family: 'diagnostic',
  summary: 'Asks whether past values of one series help predict another beyond its own past.',
  caveats: [
    {
      id: caveatId('granger-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows are a regular time series; the lags are built from row order.',
      consequenceIfUnmet: 'Lags of independent rows carry no meaning.',
      sources: [GRANGER_1969, statsmodels('statsmodels/tsa/stattools/_stattools.py:2368-2445')],
    },
    {
      id: caveatId('granger-lag-order'),
      category: 'stationarity-and-dynamics',
      requirement: 'Use a justified lag range on series with an accepted stationarity route.',
      consequenceIfUnmet: 'Too few lags leave residual serial dependence; too many lags reduce statistical power.',
      sources: [statsmodels('docs/source/vector_ar.rst:80-95'), hirmos('crates/causal-core/src/tsdiag.rs#granger_ssr_ftest')],
    },
    {
      id: caveatId('granger-predictive-reading'),
      category: 'interpretation',
      requirement: 'A rejection establishes incremental temporal prediction at the tested lag order, not an identified intervention effect.',
      consequenceIfUnmet: 'Predictive precedence is presented as a causal effect.',
      sources: [GRANGER_1969],
    },
  ],
}

const PCMCI_PLUS_PAR_CORR: MethodDefinition = {
  id: PCMCI_PLUS_PAR_CORR_METHOD_ID,
  name: 'PCMCI+ with ParCorr',
  family: 'discovery',
  summary: 'Finds lagged and same-period conditional dependencies in autocorrelated series. It returns a completed partially directed acyclic graph (CPDAG) over time.',
  caveats: [
    {
      id: caveatId('pcmciplus-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'The lagged copies of each variable follow the declared sampling grid.',
      consequenceIfUnmet: 'Links join rows that are not neighbours in time.',
      sources: [tigramite('tigramite/data_processing.py:481-881'), tigramite('tigramite/pcmci.py:47-90')],
    },
    {
      id: caveatId('pcmciplus-causal-stationarity'),
      category: 'stationarity-and-dynamics',
      requirement: 'One causal graph holds across the window (causal stationarity).',
      consequenceIfUnmet: 'One graph averages regimes with different links.',
      sources: [RUNGE_2020, tigramite('README.md:18-27')],
    },
    {
      id: caveatId('pcmciplus-no-hidden'),
      category: 'identification',
      requirement: 'Assume no unmeasured common causes among the modelled variables; LPCMCI allows them.',
      consequenceIfUnmet: 'Orientations can be confounded.',
      sources: [RUNGE_2020, tigramite('README.md:20-27')],
    },
    {
      id: caveatId('parcorr-linear-gaussian'),
      category: 'functional-form',
      requirement: 'ParCorr assumes linear dependence with Gaussian noise between continuous variables.',
      consequenceIfUnmet: 'Independence p-values, and so the links, can be wrong.',
      sources: [tigramite('tigramite/independence_tests/parcorr.py:15-40')],
    },
    {
      id: caveatId('pcmciplus-cpdag-reading'),
      category: 'interpretation',
      requirement: 'The output is a time-series CPDAG: directed marks are shared by the represented Markov-equivalent DAGs, while unoriented contemporaneous endpoints remain unresolved.',
      consequenceIfUnmet: 'An unresolved endpoint is presented as a uniquely identified causal direction.',
      sources: [RUNGE_2020],
    },
  ],
}

const LPCMCI_PAR_CORR: MethodDefinition = {
  id: LPCMCI_PAR_CORR_METHOD_ID,
  name: 'LPCMCI with ParCorr',
  family: 'discovery',
  summary: 'Discovers a lagged partial ancestral graph that admits latent common causes.',
  caveats: [
    {
      id: caveatId('lpcmci-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Lagged copies follow the declared grid, as for PCMCI+.',
      consequenceIfUnmet: 'Endpoint marks describe relations between incorrect time points.',
      sources: [tigramite('tigramite/lpcmci.py:28-83')],
    },
    {
      id: caveatId('lpcmci-stationary-graph'),
      category: 'stationarity-and-dynamics',
      requirement: 'The graph is stable across the window.',
      consequenceIfUnmet: 'Marks from different regimes are merged.',
      sources: [GERHARDUS_RUNGE_2020, tigramite('tigramite/lpcmci.py:28-83')],
    },
    {
      id: caveatId('lpcmci-markov-faithfulness'),
      category: 'identification',
      requirement: 'The causal Markov and faithfulness assumptions hold; faithfulness cannot be tested from the data.',
      consequenceIfUnmet: 'Independences need not identify the ancestral relations.',
      sources: [NESS_CH4('§4.6 discovery relies on faithfulness'), GERHARDUS_RUNGE_2020],
    },
    {
      id: caveatId('lpcmci-parcorr-form'),
      category: 'functional-form',
      requirement: 'ParCorr’s linear-Gaussian assumption applies to every test.',
      consequenceIfUnmet: 'PAG marks rest on invalid p-values.',
      sources: [tigramite('tigramite/independence_tests/parcorr.py:15-40')],
    },
    {
      id: caveatId('lpcmci-pag-reading'),
      category: 'interpretation',
      requirement: 'The output is a time-series partial ancestral graph (PAG): circles remain undetermined and bidirected endpoints permit latent confounding.',
      consequenceIfUnmet: 'Endpoint uncertainty is converted into a fully directed DAG.',
      sources: [GERHARDUS_RUNGE_2020],
    },
  ],
}

const VAR_LINGAM: MethodDefinition = {
  id: VAR_LINGAM_METHOD_ID,
  name: 'VAR-LiNGAM',
  family: 'discovery',
  summary: 'Fits a vector autoregression, then orders the same-period structure of its residuals by non-Gaussianity.',
  caveats: [
    {
      id: caveatId('var-lingam-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'The VAR lags are built from row order on a regular grid.',
      consequenceIfUnmet: 'Lag matrices describe neighbours that are not consecutive periods.',
      sources: [lingam('lingam/var_lingam.py'), hirmos('crates/analysis-wasm/src/lib.rs#var_lingam_evidence')],
    },
    {
      id: caveatId('var-lingam-non-gaussian-errors'),
      category: 'functional-form',
      requirement: 'Independent non-Gaussian errors drive the residuals; Shapiro–Wilk checks this.',
      consequenceIfUnmet: 'With Gaussian errors the same-period order is not identified and the returned order is a tie-break.',
      sources: [HYVARINEN_2010, lingam('lingam/var_lingam.py')],
    },
    {
      id: caveatId('var-lingam-acyclic-contemporaneous'),
      category: 'functional-form',
      requirement: 'Same-period effects are acyclic with no hidden common cause.',
      consequenceIfUnmet: 'Feedback or confounding is reported as a direction.',
      sources: [HYVARINEN_2010, lingam('lingam/direct_lingam.py')],
    },
    {
      id: caveatId('var-lingam-stable-var'),
      category: 'stationarity-and-dynamics',
      requirement: 'VAR coefficients are stable across the window; BIC picks the lag up to the maximum.',
      consequenceIfUnmet: 'Residual non-Gaussianity can come from a break rather than structure.',
      sources: [lingam('lingam/var_lingam.py'), hirmos('crates/causal-core/src/var_lingam.rs')],
    },
    {
      id: caveatId('var-lingam-pruning-sensitivity'),
      category: 'computation',
      requirement: 'Adaptive-lasso pruning is a sparsity choice; the unpruned fit keeps every coefficient.',
      consequenceIfUnmet: 'Regularisation is mistaken for identified structure.',
      sources: [lingam('lingam/utils/__init__.py#predict_adaptive_lasso'), hirmos('crates/causal-core/src/lars.rs')],
    },
    {
      id: caveatId('var-lingam-weight-interpretation'),
      category: 'interpretation',
      requirement: 'The adjacency matrices represent candidate structure rather than intervention effects; temporal precedence is encoded in the lag matrices.',
      consequenceIfUnmet: 'Coefficients are read as causal effects without identification.',
      sources: [lingam('lingam/var_lingam.py#get_total_causal_effects'), hirmos('crates/causal-core/src/var_lingam.rs')],
    },
    {
      id: caveatId('var-lingam-browser-boundary'),
      category: 'missingness',
      requirement: 'Between 2 and 12 complete, varying columns and lags 1 to 6.',
      consequenceIfUnmet: 'A constant column or a gap refuses the run.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#var_lingam_evidence')],
    },
  ],
}

const DIRECT_LINGAM: MethodDefinition = {
  id: DIRECT_LINGAM_METHOD_ID,
  name: 'DirectLiNGAM',
  family: 'discovery',
  summary: 'Estimates a causal order and weighted directed graph from independent observations under a linear non-Gaussian acyclic model.',
  caveats: [
    {
      id: caveatId('direct-lingam-independent-observations'),
      category: 'sampling-structure',
      requirement: 'Rows are independent observations from one unchanged data-generating process.',
      consequenceIfUnmet: 'Serial dependence, repeated units, or a changing process can be mistaken for causal structure.',
      sources: [SHIMIZU_2011],
    },
    {
      id: caveatId('direct-lingam-linear-acyclic-sem'),
      category: 'functional-form',
      requirement: 'The variables follow a linear acyclic structural equation model.',
      consequenceIfUnmet: 'Nonlinearity or feedback can produce an order and weights that do not represent the underlying structure.',
      sources: [SHIMIZU_2011],
    },
    {
      id: caveatId('direct-lingam-independent-non-gaussian-errors'),
      category: 'noise-and-dependence',
      requirement: 'The structural disturbances are mutually independent and non-Gaussian.',
      consequenceIfUnmet: 'The causal order is not identified by the LiNGAM argument.',
      sources: [SHIMIZU_2011],
    },
    {
      id: caveatId('direct-lingam-no-hidden-common-causes'),
      category: 'identification',
      requirement: 'There is no unmeasured common cause among the selected variables.',
      consequenceIfUnmet: 'Latent confounding can be reported as a directed edge.',
      sources: [SHIMIZU_2011],
    },
    {
      id: caveatId('direct-lingam-pruning'),
      category: 'computation',
      requirement: 'The reported adjacency uses adaptive-lasso pruning after the causal order is selected.',
      consequenceIfUnmet: 'A zero weight is treated as absence of a relation without accounting for the pruning rule.',
      sources: [SHIMIZU_2011, hirmos('crates/causal-core/src/var_lingam.rs#direct_lingam')],
    },
    {
      id: caveatId('direct-lingam-browser-boundary'),
      category: 'missingness',
      requirement: 'Between 2 and 12 complete, varying numeric columns are selected.',
      consequenceIfUnmet: 'The browser refuses the run before the numerical kernel is called.',
      sources: [hirmos('crates/analysis-wasm/src/discovery.rs#direct_lingam_evidence')],
    },
    {
      id: caveatId('direct-lingam-weight-interpretation'),
      category: 'interpretation',
      requirement: 'The order and weights are candidate structural evidence under the model assumptions, not identified intervention effects.',
      consequenceIfUnmet: 'A fitted structural coefficient is presented as the effect of an intervention.',
      sources: [SHIMIZU_2011],
    },
  ],
}

const DYNOTEARS: MethodDefinition = {
  id: DYNOTEARS_METHOD_ID,
  name: 'DYNOTEARS',
  family: 'discovery',
  summary: 'Fits a sparse linear dynamic structural equation model under an acyclicity penalty.',
  caveats: [
    {
      id: caveatId('dynotears-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'The explicit lag design follows row order on a regular grid.',
      consequenceIfUnmet: 'Lag weights join the wrong periods.',
      sources: [PAMFIL_2020, causalnex('causalnex.structure.dynotears.from_numpy_dynamic'), hirmos('crates/analysis-wasm/src/lib.rs#dynotears_evidence')],
    },
    {
      id: caveatId('dynotears-linear-sem'),
      category: 'functional-form',
      requirement: 'A sparse linear dynamic structural equation model describes the dependencies.',
      consequenceIfUnmet: 'Nonlinear or dense dependence is dropped or misweighted.',
      sources: [PAMFIL_2020],
    },
    {
      id: caveatId('dynotears-stable-window'),
      category: 'stationarity-and-dynamics',
      requirement: 'Coefficients are stable across the window.',
      consequenceIfUnmet: 'One matrix averages regimes.',
      sources: [PAMFIL_2020],
    },
    {
      id: caveatId('dynotears-penalty-sensitivity'),
      category: 'computation',
      requirement: 'lambdaW, lambdaA and any display threshold are tuning choices.',
      consequenceIfUnmet: 'Tuning is mistaken for identified structure.',
      sources: [PAMFIL_2020, causalnex('causalnex.structure.dynotears.from_numpy_dynamic'), hirmos('crates/causal-core/src/dynotears.rs')],
    },
    {
      id: caveatId('dynotears-weight-interpretation'),
      category: 'interpretation',
      requirement: 'The returned matrices are candidate structure, not intervention effects.',
      consequenceIfUnmet: 'Optimisation weights are read as causal effects.',
      sources: [hirmos('crates/causal-core/src/dynotears.rs')],
    },
    {
      id: caveatId('dynotears-browser-boundary'),
      category: 'missingness',
      requirement: 'Between 2 and 12 complete columns and lags 1 to 6.',
      consequenceIfUnmet: 'A gap refuses the run.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#dynotears_evidence')],
    },
  ],
}

const OCSE: MethodDefinition = {
  id: OCSE_METHOD_ID,
  name: 'Optimal causation entropy',
  family: 'discovery',
  summary: 'Selects lagged parents by forward and backward conditional mutual information tests.',
  caveats: [
    {
      id: caveatId('ocse-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Lagged candidates follow row order on a regular grid.',
      consequenceIfUnmet: 'Conditional information between non-consecutive rows has no temporal reading.',
      sources: [SUN_2015, causationEntropy('causationentropy/core/discovery.py#discover_network'), hirmos('crates/causal-core/src/ocse.rs#discover_network')],
    },
    {
      id: caveatId('ocse-information-estimator'),
      category: 'functional-form',
      requirement: 'The information estimator fits the dependence and the sample: Gaussian for linear, k-nearest neighbours for nonlinear.',
      consequenceIfUnmet: 'Gaussian CMI misses nonlinearity; kNN CMI is unstable in small samples.',
      sources: [SUN_2015, causationEntropy('causationentropy/core/information/conditional_mutual_information.py'), hirmos('crates/causal-core/src/ocse.rs#cmi')],
    },
    {
      id: caveatId('ocse-lag-and-stability'),
      category: 'stationarity-and-dynamics',
      requirement: 'A defensible maximum lag and a stable window.',
      consequenceIfUnmet: 'Parents beyond the lag are missed; a changing process is drawn as one network.',
      sources: [SUN_2015],
    },
    {
      id: caveatId('ocse-permutation-resolution'),
      category: 'finite-sample',
      requirement: 'p-values are Monte Carlo at the recorded shuffle count and seed.',
      consequenceIfUnmet: 'A coarse p-value is read as exact.',
      sources: [hirmos('crates/causal-core/src/ocse.rs#shuffle_test')],
    },
    {
      id: caveatId('ocse-corrected-semantics'),
      category: 'computation',
      requirement: 'Duplicate target self-lags already in the conditioning set are excluded, unlike the reference package.',
      consequenceIfUnmet: 'Results are described as the reference’s legacy behaviour.',
      sources: [hirmos('crates/causal-core/src/ocse.rs#discover_network')],
    },
    {
      id: caveatId('ocse-not-interventional'),
      category: 'interpretation',
      requirement: 'Selected edges are conditional-information evidence about the observed process, not identified intervention effects.',
      consequenceIfUnmet: 'Predictive pathways are read as identified effects.',
      sources: [SUN_2015],
    },
  ],
}

const BACKDOOR_IDENTIFICATION: MethodDefinition = {
  id: BACKDOOR_IDENTIFICATION_METHOD_ID,
  name: 'Back-door adjustment',
  family: 'identification',
  summary: 'Constructs the canonical measured set and enumerates inclusion-minimal sets that satisfy the total-effect adjustment criterion.',
  caveats: [
    {
      id: caveatId('backdoor-graph-is-complete'),
      category: 'identification',
      requirement: 'The DAG holds every common cause of treatment and outcome; a cause not measured is still drawn as an unmeasured node.',
      consequenceIfUnmet: 'An omitted common cause leaves a back-door path open.',
      sources: [RUIZ_DE_VILLA_CH7('§7.2 back-door criterion and adjustment theorem'), NESS_CH11('§11.4.1 any blocking set works'), VAN_DER_ZANDER_2014],
    },
    {
      id: caveatId('backdoor-unmeasured-nodes-excluded'),
      category: 'identification',
      requirement: 'Unmeasured nodes block or open paths but never enter the adjustment set.',
      consequenceIfUnmet: 'A set that needs an unmeasured variable is correctly refused as a measured back-door adjustment set.',
      sources: [VAN_DER_ZANDER_2014],
    },
    {
      id: caveatId('backdoor-minimal-set-choice'),
      category: 'interpretation',
      requirement: 'When several minimal sets exist, choose among them using measurement quality, observed support and the planned model before estimation.',
      consequenceIfUnmet: 'Choosing the set whose estimate is preferred introduces an unrecorded analysis-selection decision.',
      sources: [VAN_DER_ZANDER_2014, TAKATA_2010],
    },
    {
      id: caveatId('backdoor-lags-collapsed'),
      category: 'stationarity-and-dynamics',
      requirement: 'Lagged arrows are collapsed to variable level; adjustment uses same-period values.',
      consequenceIfUnmet: 'Effects that travel through lags are attributed to one period.',
      sources: [hirmos('src/domain/study.ts#studyGraphOf')],
    },
    {
      id: caveatId('backdoor-not-the-only-strategy'),
      category: 'interpretation',
      requirement: 'The adjustment-set result and the general ID result are reported separately. A general identifying expression need not be estimable by an adjustment estimator.',
      consequenceIfUnmet: 'An identified functional is passed to an estimator that targets a different functional.',
      sources: [NESS_CH10('query, model, identification, estimation workflow'), NESS_CH11('§11.3 back-door and front-door estimands')],
    },
  ],
}

const GRAPHICAL_IDENTIFICATION: MethodDefinition = {
  id: GRAPHICAL_IDENTIFICATION_METHOD_ID,
  name: 'Graphical identification (ID)',
  family: 'identification',
  summary: 'Derives an observational expression for an interventional distribution from an acyclic directed mixed graph, or returns the hedge that prevents identification.',
  caveats: [
    {
      id: caveatId('id-causal-model'),
      category: 'identification',
      requirement: 'The directed and latent-confounding edges encode the relevant causal model, and the observed distribution is compatible with that model.',
      consequenceIfUnmet: 'The returned expression identifies the query in the recorded graph, not necessarily in the data-generating process.',
      sources: [SHPITSER_PEARL_ID, NESS_CH10('queries, models, and identification')],
    },
    {
      id: caveatId('id-level-two-query'),
      category: 'interpretation',
      requirement: 'This implementation identifies unconditional level-2 queries P(Y | do(X)) from observational P(V).',
      consequenceIfUnmet: 'Conditional interventions or level-3 counterfactual queries are treated as if ID, rather than IDC or ID*/IDC*, had established them.',
      sources: [SHPITSER_PEARL_ID, NESS_CH10('ID, IDC, ID*, and IDC*')],
    },
    {
      id: caveatId('id-estimator-separate'),
      category: 'interpretation',
      requirement: 'Identification supplies a functional; estimation requires an implementation that evaluates that same functional.',
      consequenceIfUnmet: 'A back-door estimator is used for a front-door or more general ID expression.',
      sources: [NESS_CH10('query → model → identify → estimate')],
    },
  ],
}

const COUNTERFACTUAL_IDENTIFICATION: MethodDefinition = {
  id: COUNTERFACTUAL_IDENTIFICATION_METHOD_ID,
  name: 'Counterfactual identification (ID*/IDC*)',
  family: 'identification',
  summary: 'Identifies the conditional potential-outcome distributions required for the binary effect on the treated, or records that the query is not identifiable from the observational distribution.',
  caveats: [
    {
      id: caveatId('id-star-causal-model'),
      category: 'identification',
      requirement: 'The directed and latent-confounding edges encode the causal model relevant to the counterfactual query.',
      consequenceIfUnmet: 'The expressions are identified in the recorded graph, not necessarily in the data-generating process.',
      sources: [SHPITSER_PEARL_COUNTERFACTUAL, NESS_CH10('counterfactual graphs and ID*/IDC*')],
    },
    {
      id: caveatId('id-star-query-scope'),
      category: 'interpretation',
      requirement: 'The current product query is binary ETT: E[Y(1) − Y(0) | X=1].',
      consequenceIfUnmet: 'Identification of this ETT is presented as identification of a different counterfactual target such as PN, PS, or PNS.',
      sources: [SHPITSER_PEARL_COUNTERFACTUAL, NESS_CH10('level-3 counterfactual queries')],
    },
    {
      id: caveatId('id-star-estimator-separate'),
      category: 'interpretation',
      requirement: 'The identified expressions and their numerical evaluation remain separate records.',
      consequenceIfUnmet: 'A symbolic identification result is reported as an estimate before the observed distribution has been evaluated.',
      sources: [NESS_CH10('query → model → identify → estimate')],
    },
  ],
}

const BACKDOOR_LINEAR_REGRESSION: MethodDefinition = {
  id: BACKDOOR_LINEAR_REGRESSION_METHOD_ID,
  name: 'Adjusted linear regression',
  family: 'estimation',
  summary: 'Least squares of the outcome on the treatment and the adjustment set; the treatment coefficient is the effect, with a classical or Newey–West interval.',
  caveats: [
    {
      id: caveatId('linear-identified-adjustment'),
      category: 'identification',
      requirement: 'The covariates are an identified adjustment set; regression cannot close an open back-door path.',
      consequenceIfUnmet: 'The coefficient is an association.',
      sources: [NESS_CH11('§11.4.1 regression with back-door confounders'), RUIZ_DE_VILLA_CH7('§7.4.5 total effect theorem')],
    },
    {
      id: caveatId('linear-functional-form'),
      category: 'functional-form',
      requirement: 'The outcome is linear and additive in treatment and covariates with one effect for every unit.',
      consequenceIfUnmet: 'Effect modification is averaged away or missed.',
      sources: [RUIZ_DE_VILLA_CH8('§8.1.4 heterogeneous effects as interactions'), statsmodels('statsmodels/regression/linear_model.py#OLS')],
    },
    {
      id: caveatId('linear-serial-dependence'),
      category: 'noise-and-dependence',
      requirement: 'On time-series rows use the HAC (Newey–West) interval; the classical interval assumes independent errors.',
      consequenceIfUnmet: 'Autocorrelated errors make the classical interval too narrow.',
      sources: [NEWEY_WEST_1987, statsmodels('statsmodels/stats/sandwich_covariance.py#cov_hac_simple')],
    },
    {
      id: caveatId('linear-hac-bandwidth'),
      category: 'finite-sample',
      requirement: 'The HAC bandwidth is statsmodels’ default, floor(4 (n/100)^(2/9)) lags, recorded with the run.',
      consequenceIfUnmet: 'A bandwidth chosen after the result changes the interval without a record.',
      sources: [statsmodels('statsmodels/stats/sandwich_covariance.py#S_hac_simple')],
    },
    {
      id: caveatId('linear-overlap'),
      category: 'identification',
      requirement: 'Treated and untreated values occur across the range of the covariates (positivity).',
      consequenceIfUnmet: 'The effect is extrapolated from the functional form.',
      sources: [RUIZ_DE_VILLA_CH7('§7.4.3 positivity'), HERNAN_ROBINS],
    },
    {
      id: caveatId('linear-not-time-graph'),
      category: 'stationarity-and-dynamics',
      requirement: 'The coefficient is a same-period effect; lagged and cumulative effects need a time-graph estimator.',
      consequenceIfUnmet: 'A same-period coefficient is presented as the dynamic effect.',
      sources: [hirmos('docs/DESIGN.md#10 total effect from stationary time graph')],
    },
    {
      id: caveatId('linear-level-stationarity'),
      category: 'stationarity-and-dynamics',
      requirement: 'On time-series rows the prepared treatment and outcome are stationary, or the study uses a suitable cointegration route for I(1) variables retained in levels.',
      consequenceIfUnmet: 'A regression between integrated series is spurious.',
      sources: [GRANGER_NEWBOLD_1974, hirmos('src/domain/stationarityAssessment.ts#assessStationarity')],
    },
  ],
}

const FRONTDOOR_TWO_STAGE: MethodDefinition = {
  id: FRONTDOOR_TWO_STAGE_METHOD_ID,
  name: 'Linear front-door regression',
  family: 'estimation',
  summary: 'Fits the treatment-to-mediator and mediator-to-outcome regressions and multiplies their intervention contrasts.',
  caveats: [
    {
      id: caveatId('frontdoor-identified-mediator'),
      category: 'identification',
      requirement: 'An observed mediator satisfies all three front-door conditions for the treatment and outcome in the recorded graph.',
      consequenceIfUnmet: 'The product of the two regression coefficients does not identify the requested intervention effect.',
      sources: [NESS_CH10('front-door identification'), NESS_CH11('§11.3 front-door estimand'), PEARL_2009('§3.3.2 front-door criterion')],
    },
    {
      id: caveatId('frontdoor-linear-stages'),
      category: 'functional-form',
      requirement: 'Both stage regressions are linear and additive over the requested intervention contrast.',
      consequenceIfUnmet: 'Multiplying two constant slopes does not evaluate the nonparametric front-door functional.',
      sources: [NESS_CH11('§11.4 two-stage regression for the front-door estimand')],
    },
    {
      id: caveatId('frontdoor-single-mediator'),
      category: 'functional-form',
      requirement: 'The identified front-door set contains one mediator.',
      consequenceIfUnmet: 'The two-stage implementation cannot represent the joint mediator intervention.',
      sources: [NESS_CH11('§11.4 front-door estimation with one mediator')],
    },
    {
      id: caveatId('frontdoor-bootstrap-rows'),
      category: 'noise-and-dependence',
      requirement: 'The row bootstrap is appropriate for the sampling design; dependent rows require a dependence-aware resampling scheme.',
      consequenceIfUnmet: 'The confidence interval does not represent the estimator’s sampling variation.',
      sources: [NESS_CH11('§11.4 confidence interval for the front-door estimate')],
    },
  ],
}

const countCaveats = (prefix: 'poisson' | 'negbin', model: string): NonEmptyArray<MethodCaveat> => [
  {
    id: caveatId(`${prefix}-identified-adjustment`),
    category: 'identification',
    requirement: 'The covariates in the linear predictor are an identified adjustment set.',
    consequenceIfUnmet: 'The rate ratio is an association.',
    sources: [RUIZ_DE_VILLA_CH7('§7.2 back-door criterion'), hirmos('src/domain/study.ts#identificationFrom')],
  },
  {
    id: caveatId(`${prefix}-count-outcome`),
    category: 'functional-form',
    requirement: prefix === 'poisson'
      ? 'The outcome is a non-negative integer count; with the log link the treatment multiplies the expected count.'
      : 'The outcome is a non-negative integer count; the NB2 variance is the mean plus alpha times the mean squared.',
    consequenceIfUnmet: 'The likelihood has no meaning for a non-count outcome.',
    sources: [CAMERON_TRIVEDI, statsmodels(model)],
  },
  prefix === 'poisson'
    ? {
      id: caveatId('poisson-dispersion'),
      category: 'noise-and-dependence',
      requirement: 'Variance equals the mean; compare with the negative binomial alpha before trusting the standard errors.',
      consequenceIfUnmet: 'Overdispersed counts give standard errors that are too small.',
      sources: [CAMERON_TRIVEDI, statsmodels('statsmodels/genmod/families/family.py#Poisson')],
    }
    : {
      id: caveatId('negbin-convergence'),
      category: 'computation',
      requirement: 'BFGS reaches the maximum; a stop at the alpha boundary is reported as not converged.',
      consequenceIfUnmet: 'Estimates from an unconverged fit are not maximum likelihood.',
      sources: [statsmodels('statsmodels/discrete/discrete_model.py#NegativeBinomialP.fit'), hirmos('crates/causal-core/src/bfgs.rs')],
    },
  {
    id: caveatId(`${prefix}-independence`),
    category: 'noise-and-dependence',
    requirement: prefix === 'poisson'
      ? 'Counts are independent across rows; no serial-correlation correction exists in this fit.'
      : 'Rows are independent; the dispersion term absorbs extra variance, not serial dependence.',
    consequenceIfUnmet: 'Autocorrelated counts make the interval too narrow.',
    sources: [statsmodels(model)],
  },
  {
    id: caveatId(`${prefix}-level-stationarity`),
    category: 'stationarity-and-dynamics',
    requirement: 'On time-series rows the treatment and covariates are stationary in levels.',
    consequenceIfUnmet: 'A shared trend is read as a causal multiplier.',
    sources: [GRANGER_NEWBOLD_1974, hirmos('src/domain/stationarityAssessment.ts#assessStationarity')],
  },
  {
    id: caveatId(`${prefix}-interpretation`),
    category: 'interpretation',
    requirement: prefix === 'poisson'
      ? 'exp(β) is the multiplicative change in the expected count per unit of treatment, covariates fixed.'
      : 'exp(β) is a rate ratio, as for Poisson; alpha describes spread, not effect.',
    consequenceIfUnmet: 'A ratio is read as an additive difference.',
    sources: [CAMERON_TRIVEDI, hirmos('crates/analysis-wasm/src/lib.rs#count_glm')],
  },
]

const POISSON_GLM: MethodDefinition = {
  id: POISSON_GLM_METHOD_ID,
  name: 'Poisson GLM',
  family: 'estimation',
  summary: 'Poisson family with log link by iteratively reweighted least squares; exp(β) is an incidence rate ratio.',
  caveats: countCaveats('poisson', 'statsmodels/genmod/generalized_linear_model.py#GLM'),
}

const NEGATIVE_BINOMIAL: MethodDefinition = {
  id: NEGATIVE_BINOMIAL_METHOD_ID,
  name: 'Negative binomial (NB2)',
  family: 'estimation',
  summary: 'NB2 by BFGS: a Poisson mean with gamma-distributed rate, so alpha absorbs variance above the mean.',
  caveats: countCaveats('negbin', 'statsmodels/discrete/discrete_model.py#NegativeBinomialP'),
}

const CAUSAL_EFFECTS_TOTAL: MethodDefinition = {
  id: CAUSAL_EFFECTS_TOTAL_METHOD_ID,
  name: 'CausalEffects total effect',
  family: 'estimation',
  summary: 'Tigramite’s CausalEffects on a stationary time-series DAG: latent projection, Runge’s optimal adjustment set, and a total effect predicted at two intervention values.',
  caveats: [
    {
      id: caveatId('causal-effects-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows are a regular time series so lagged nodes exist.',
      consequenceIfUnmet: 'Independent rows have no lag structure to project.',
      sources: [tigramite('tigramite/causal_effects.py#CausalEffects(graph_type="stationary_dag")')],
    },
    {
      id: caveatId('causal-effects-stationary-dag'),
      category: 'identification',
      requirement: 'The lagged DAG is the whole stationary structure; unmeasured nodes are declared hidden.',
      consequenceIfUnmet: 'A missing arrow or undeclared hidden node changes the adjustment set.',
      sources: [RUNGE_2021, tigramite('tigramite/causal_effects.py#_get_latent_projection_graph')],
    },
    {
      id: caveatId('causal-effects-stationarity'),
      category: 'stationarity-and-dynamics',
      requirement: 'Every modelled series has an accepted stationarity route.',
      consequenceIfUnmet: 'A trending series breaks the stationary-graph assumption.',
      sources: [tigramite('tigramite/causal_effects.py'), hirmos('src/domain/stationarityAssessment.ts')],
    },
    {
      id: caveatId('causal-effects-identifiable'),
      category: 'identification',
      requirement: 'An optimal adjustment set exists in the projected graph; it may be larger than a minimal set because it minimises variance.',
      consequenceIfUnmet: 'The run reports not identifiable and no estimate.',
      sources: [RUNGE_2021],
    },
    {
      id: caveatId('causal-effects-functional-form'),
      category: 'functional-form',
      requirement: 'The first-stage estimator matches the dependence: linear for additive, k-nearest neighbours for local nonlinearity.',
      consequenceIfUnmet: 'A misspecified first stage biases the predicted difference.',
      sources: [tigramite('tigramite/causal_effects.py#fit_total_effect')],
    },
    {
      id: caveatId('causal-effects-no-interval'),
      category: 'finite-sample',
      requirement: 'The result is a point estimate; the bootstrap is not ported.',
      consequenceIfUnmet: 'A point is read as if it carried an interval.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#causal_effects_total')],
    },
  ],
}

const CAUSAL_IMPACT: MethodDefinition = {
  id: CAUSAL_IMPACT_METHOD_ID,
  name: 'Causal impact',
  family: 'estimation',
  summary: 'A local level model with a static regression on control series, fitted on the pre-period and forecast forward; the effect is the gap to the forecast.',
  caveats: [
    {
      id: caveatId('impact-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows are a regular time series split into a pre and a post window.',
      consequenceIfUnmet: 'The pre-intervention fit and post-intervention contrast are not defined.',
      sources: [statsmodels('statsmodels/tsa/statespace/structural.py#UnobservedComponents(level="llevel")')],
    },
    {
      id: caveatId('impact-intervention-time'),
      category: 'identification',
      requirement: 'The intervention starts at one known row and nothing else changes then.',
      consequenceIfUnmet: 'A pre-trend or another event is attributed to the intervention.',
      sources: [BRODERSEN_2015],
    },
    {
      id: caveatId('impact-pre-period'),
      category: 'finite-sample',
      requirement: 'The pre-period is long and stable enough to fit the level and the regression.',
      consequenceIfUnmet: 'The counterfactual band is too narrow or the fit fails.',
      sources: [hirmos('crates/causal-core/src/causal_impact.rs#fit')],
    },
    {
      id: caveatId('impact-controls'),
      category: 'functional-form',
      requirement: 'Control series predict the outcome through a static linear regression.',
      consequenceIfUnmet: 'With weak control series, the counterfactual is driven primarily by the local-level component.',
      sources: [statsmodels('statsmodels/tsa/statespace/structural.py#exog')],
    },
    {
      id: caveatId('impact-controls-unaffected'),
      category: 'identification',
      requirement: 'Controls are untouched by the intervention and are not DAG descendants of the treatment.',
      consequenceIfUnmet: 'An affected control absorbs part of the effect.',
      sources: [BRODERSEN_2015, hirmos('src/domain/estimation.ts#affectedNodes')],
    },
    {
      id: caveatId('impact-interpretation'),
      category: 'interpretation',
      requirement: 'Pointwise, cumulative and average effects cover the post window only.',
      consequenceIfUnmet: 'The cumulative effect is extrapolated past the data.',
      sources: [hirmos('crates/causal-core/src/causal_impact.rs#causal_impact')],
    },
  ],
}

const refuterCaveat = (id: string, requirement: string, consequenceIfUnmet: string, sources: NonEmptyArray<MethodSource>): MethodCaveat => ({
  id: caveatId(id),
  category: 'interpretation',
  requirement,
  consequenceIfUnmet,
  sources,
})

const PLACEBO_REFUTER: MethodDefinition = {
  id: PLACEBO_REFUTER_METHOD_ID,
  name: 'Placebo treatment',
  family: 'refuter',
  summary: 'Permutes the treatment and refits; the resulting estimate is compared with zero.',
  caveats: [
    refuterCaveat('placebo-reads-against-zero', 'Read the placebo estimate against zero.', 'A placebo far from zero means the estimator picks up structure that is not the treatment.', [NESS_CH11('§11.5.3 placebo treatment refuter'), dowhy('dowhy/causal_refuters/placebo_treatment_refuter.py#placebo_type="permute"')]),
    { id: caveatId('placebo-seeded'), category: 'computation', requirement: 'Permutations follow the recorded seed and count through the ported MT19937 stream.', consequenceIfUnmet: 'The refutation cannot be repeated.', sources: [hirmos('crates/causal-core/src/backdoor.rs#refute_placebo')] },
  ],
}

const DATA_SUBSET_REFUTER: MethodDefinition = {
  id: DATA_SUBSET_REFUTER_METHOD_ID,
  name: 'Data subset',
  family: 'refuter',
  summary: 'Refits on random row subsets; a stable effect stays close to the original.',
  caveats: [
    refuterCaveat('subset-reads-against-original', 'Compare each subset estimate with the original estimate.', 'A large shift indicates that a small subset of observations has high influence on the estimate.', [NESS_CH11('§11.5.1 data subset refuter'), dowhy('dowhy/causal_refuters/data_subset_refuter.py')]),
    { id: caveatId('subset-time-order'), category: 'sampling-structure', requirement: 'Subsets ignore time order; for a time series, this diagnostic assesses observation influence rather than serial dependence.', consequenceIfUnmet: 'The subset diagnostic is interpreted as a test of serial dependence.', sources: [hirmos('crates/causal-core/src/backdoor.rs#refute_data_subset')] },
  ],
}

const RANDOM_COMMON_CAUSE_REFUTER: MethodDefinition = {
  id: RANDOM_COMMON_CAUSE_REFUTER_METHOD_ID,
  name: 'Random common cause',
  family: 'refuter',
  summary: 'Adds an independent random covariate and refits; the estimate should not move.',
  caveats: [
    refuterCaveat('random-cause-reads-against-original', 'Read the estimate with the random covariate against the original.', 'Movement means the estimator is sensitive to irrelevant conditioning.', [NESS_CH11('§11.5.2 random common cause refuter'), dowhy('dowhy/causal_refuters/random_common_cause.py')]),
    { id: caveatId('random-cause-not-a-confounder-test'), category: 'identification', requirement: 'The added covariate is independent by construction; passing says nothing about a real unmeasured confounder.', consequenceIfUnmet: 'The probe is read as evidence against confounding.', sources: [NESS_CH11('§11.5.2 random common cause refuter')] },
  ],
}

const UNOBSERVED_COMMON_CAUSE: MethodDefinition = {
  id: UNOBSERVED_COMMON_CAUSE_METHOD_ID,
  name: 'Simulated unmeasured confounder',
  family: 'refuter',
  summary: 'Simulates an unmeasured confounder over a grid of treatment and outcome strengths, then refits each cell. The result reports how far the estimate moves, not a p-value.',
  caveats: [
    { id: caveatId('unobserved-binary-flip'), category: 'functional-form', requirement: 'The treatment is binary and the simulated outcome shift is linear; the grid represents specified sensitivity scenarios.', consequenceIfUnmet: 'The sensitivity parameters are interpreted as estimates of an actual unmeasured confounder.', sources: [NESS_CH11('§11.5.5 unobserved common cause refuter'), dowhy('dowhy/causal_refuters/add_unobserved_common_cause.py#simulation_method="direct-simulation"')] },
    { id: caveatId('unobserved-kappa-range'), category: 'interpretation', requirement: 'Default strengths are calibrated from observed common causes; stronger unmeasured confounding is outside the evaluated grid.', consequenceIfUnmet: 'The reported grid omits confounding scenarios that may be relevant to the study.', sources: [dowhy('dowhy/causal_refuters/add_unobserved_common_cause.py#_infer_default_kappa_t')] },
    { id: caveatId('unobserved-stream'), category: 'computation', requirement: 'Cells share one seeded stream and mutate the frame in turn, as the reference does.', consequenceIfUnmet: 'One cell rerun alone gives another number.', sources: [hirmos('crates/causal-core/src/unobserved.rs#unobserved_common_cause_grid')] },
  ],
}

const LJUNG_BOX: MethodDefinition = {
  id: LJUNG_BOX_METHOD_ID,
  name: 'Ljung–Box on residuals',
  family: 'diagnostic',
  summary: 'Cumulative autocorrelation statistic of the residuals at each lag with a chi-square p-value.',
  caveats: [
    { id: caveatId('ljung-box-null'), category: 'interpretation', requirement: 'A small p-value rejects white-noise residuals up to that lag; it does not say which lags are missing.', consequenceIfUnmet: 'Autocorrelation is read as a specific dynamic model.', sources: [LJUNG_BOX_1978, statsmodels('statsmodels/stats/diagnostic.py#acorr_ljungbox')] },
  ],
}

const SHAPIRO_WILK: MethodDefinition = {
  id: SHAPIRO_WILK_METHOD_ID,
  name: 'Shapiro–Wilk on residuals',
  family: 'diagnostic',
  summary: 'W and its p-value on the residuals for 3 to 5,000 rows; the null is normality.',
  caveats: [
    { id: caveatId('shapiro-sample-size'), category: 'finite-sample', requirement: 'At large sample sizes, small departures from normality can produce small p-values; interpret W together with the p-value.', consequenceIfUnmet: 'A minor distributional departure may be treated as substantively important.', sources: [SHAPIRO_WILK_1965, hirmos('crates/causal-core/src/tsdiag.rs#shapiro')] },
  ],
}

const PELT: MethodDefinition = {
  id: PELT_METHOD_ID,
  name: 'Pruned exact linear time change points',
  family: 'diagnostic',
  summary: 'Exact penalised segmentation of the series mean with the L2 cost.',
  caveats: [
    { id: caveatId('pelt-mean-shift'), category: 'interpretation', requirement: 'The L2 cost finds mean shifts; changes in spread or season need another cost.', consequenceIfUnmet: 'A change in spread is missed or a seasonal swing is read as a break.', sources: [KILLICK_2012, ruptures('src/ruptures/detection/pelt.py'), hirmos('crates/causal-core/src/pelt.rs#CostL2')] },
  ],
}

const STL: MethodDefinition = {
  id: STL_METHOD_ID,
  name: 'Seasonal-trend decomposition using loess',
  family: 'diagnostic',
  summary: 'Seasonal-trend decomposition by loess at the declared period, with trend and seasonal strength.',
  caveats: [
    { id: caveatId('stl-period'), category: 'sampling-structure', requirement: 'STL assumes the period matches the declared sampling frequency, with at least two complete cycles available.', consequenceIfUnmet: 'An incorrect period induces a spurious seasonal component; fewer than two cycles are insufficient for the decomposition.', sources: [CLEVELAND_1990, statsmodels('statsmodels/tsa/seasonal.py#STL')] },
    { id: caveatId('stl-strength-reading'), category: 'interpretation', requirement: 'Trend and seasonal strength are variance-ratio summaries of the fitted decomposition, not tests that a component exists.', consequenceIfUnmet: 'A descriptive strength score is presented as a hypothesis-test result.', sources: [HYNDMAN_FPP] },
  ],
}

const dmlCaveats = (model: 'plr' | 'irm'): NonEmptyArray<MethodCaveat> => [
  {
    id: caveatId(`dml-${model}-identified-adjustment`),
    category: 'identification',
    requirement: 'The nuisance learners see the identified adjustment set; cross-fitting removes regularisation bias, not confounding.',
    consequenceIfUnmet: 'The result is an adjusted association, even if the reported confidence interval is numerically well formed.',
    sources: [RUIZ_DE_VILLA_CH8('§8.1.2 partially linear model'), RUIZ_DE_VILLA_CH8('§8.3 no unobserved confounders')],
  },
  {
    id: caveatId(`dml-${model}-independent-rows`),
    category: 'sampling-structure',
    requirement: 'Rows are independent: folds are shuffled at random and dependent rows leak across them.',
    consequenceIfUnmet: 'Cross-fitting on a series overstates precision.',
    sources: [CHERNOZHUKOV_DML, hirmos('docs/DESIGN.md#10 time dependence is not cosmetic')],
  },
  model === 'irm'
    ? {
      id: caveatId('dml-irm-binary-treatment'),
      category: 'functional-form',
      requirement: 'The treatment is 0 or 1 with rows on both sides; propensities are trimmed at 0.01.',
      consequenceIfUnmet: 'The interactive score is undefined for a continuous treatment and unstable without overlap.',
      sources: [ROBINS_1994, KENNEDY_DR],
    }
    : {
      id: caveatId('dml-plr-partial-linearity'),
      category: 'functional-form',
      requirement: 'The outcome is linear in the treatment after partialling out flexible functions of the covariates.',
      consequenceIfUnmet: 'Under treatment-effect heterogeneity, the score targets a weighted average that may differ from the stated estimand.',
      sources: [CHERNOZHUKOV_DML, RUIZ_DE_VILLA_CH8('§8.1.2 partially linear model')],
    },
  {
    id: caveatId(`dml-${model}-overlap`),
    category: 'identification',
      requirement: 'Positivity requires a non-zero treatment probability at relevant covariate values; overlap asks whether both treatment groups are adequately represented in the observed sample.',
      consequenceIfUnmet: 'The effect is unsupported for part of the target population and the nuisance models extrapolate beyond observed treatment support.',
    sources: [RUIZ_DE_VILLA_CH7('§7.4.3 positivity'), HERNAN_ROBINS],
  },
  {
    id: caveatId(`dml-${model}-learner-settings`),
    category: 'computation',
    requirement: 'Five shuffled folds, 200 random-forest trees, minimum leaf 5, learner seed 7, and the recorded fold seed.',
    consequenceIfUnmet: 'Changing learner or fold settings changes the estimator specification and may change the estimate.',
    sources: [RUIZ_DE_VILLA_CH8('§8.1.3 cross-fitting and tuning'), hirmos('crates/analysis-wasm/src/lib.rs#double_ml')],
  },
  {
    id: caveatId(`dml-${model}-interval`),
    category: 'finite-sample',
    requirement: 'The sandwich interval requires the nuisance estimators to converge sufficiently quickly for the asymptotic approximation.',
    consequenceIfUnmet: 'Finite-sample confidence-interval coverage may be below the nominal level.',
    sources: [CHERNOZHUKOV_DML, BACH_DOUBLEML, HINES_2022],
  },
]

const DML_PLR: MethodDefinition = {
  id: DML_PLR_METHOD_ID,
  name: 'DML partially linear',
  family: 'estimation',
  summary: 'Random forests predict outcome and treatment from the covariates across folds; the effect is the regression of one residual on the other.',
  caveats: dmlCaveats('plr'),
}

const DML_IRM: MethodDefinition = {
  id: DML_IRM_METHOD_ID,
  name: 'DML interactive',
  family: 'estimation',
  summary: 'The doubly robust (AIPW) score for a binary treatment: outcome models per arm and a propensity model, reporting the study’s recorded ATE or ATT target.',
  caveats: [
    ...dmlCaveats('irm'),
    {
      id: caveatId('dml-irm-target-population'),
      category: 'interpretation',
      requirement: 'ATE averages over the study population; ATT averages the same potential-outcome contrast among units that received treatment. The study target determines which score is run.',
      consequenceIfUnmet: 'An effect for treated units is reported as an effect for the full population, or conversely.',
      sources: [ABADIE_CATTANEO_2018, HERNAN_ROBINS],
    },
  ] as NonEmptyArray<MethodCaveat>,
}

const DML_REFUTATION: MethodDefinition = {
  id: DML_REFUTATION_METHOD_ID,
  name: 'DML refutation batch',
  family: 'refuter',
  summary: 'One seeded stream, one order: main fit, placebo refits, random-common-cause refits, then confounding-strength bounds.',
  caveats: [
    {
      id: caveatId('dml-refutation-order'),
      category: 'computation',
      requirement: 'The batch runs whole in the recorded order; its main fit equals the estimation run at the same seed.',
      consequenceIfUnmet: 'Another order draws other folds and other numbers.',
      sources: [hirmos('crates/causal-core/src/refute_dml.rs'), hirmos('docs/DESIGN.md#12 DmlRefutationBatch')],
    },
    {
      id: caveatId('dml-refutation-light-refits'),
      category: 'finite-sample',
      requirement: 'Refits use 80 trees and two folds on at most 1,000 sampled rows; they check direction and scale.',
      consequenceIfUnmet: 'A refit’s shift is read with the main fit’s precision.',
      sources: [NESS_CH11('§11.5 refutation as simulated assumption violations'), hirmos('crates/causal-core/src/refute_dml.rs#fit_effect')],
    },
    {
      id: caveatId('dml-refutation-sensitivity-scenarios'),
      category: 'interpretation',
      requirement: 'Bounds assume equal confounding shares of 2%, 5% and 10% in outcome and treatment; the robustness value is the share that moves the effect to zero.',
      consequenceIfUnmet: 'A bound is read as a probability.',
      sources: [CHERNOZHUKOV_OVB, econml('doc/spec/validation.rst#sensitivity-analysis')],
    },
  ],
}

const ARDL_PSS: MethodDefinition = {
  id: ARDL_PSS_METHOD_ID,
  name: 'ARDL long run',
  family: 'estimation',
  summary: 'An error-correction form with AIC-chosen lags; the cointegrating vector gives the long-run effect and the bounds test says whether a level relation exists.',
  caveats: [
    {
      id: caveatId('ardl-orders-assessed'),
      category: 'stationarity-and-dynamics',
      requirement: 'Treatment and outcome are each I(0) or I(1); an I(2) or unresolved series is refused.',
      consequenceIfUnmet: 'The bounds critical values do not cover an I(2) regressor.',
      sources: [PSS_2001, hirmos('src/domain/stationarityAssessment.ts#assessStationarity')],
    },
    {
      id: caveatId('ardl-time-series'),
      category: 'sampling-structure',
      requirement: 'A regular series long enough for the maximum lag.',
      consequenceIfUnmet: 'The candidate lag models do not have sufficient residual degrees of freedom.',
      sources: [statsmodels('statsmodels/tsa/ardl/model.py#ardl_select_order')],
    },
    {
      id: caveatId('ardl-single-regressor'),
      category: 'identification',
      requirement: 'The port fits one regressor, so the identified adjustment set must be empty.',
      consequenceIfUnmet: 'A confounder left out biases the long-run coefficient.',
      sources: [statsmodels('statsmodels/tsa/ardl/model.py#UECM'), hirmos('crates/causal-core/src/ardl.rs')],
    },
    {
      id: caveatId('ardl-bounds-reading'),
      category: 'interpretation',
      requirement: 'Above the I(1) bound a level relation exists; below the I(0) bound none does; between them the test is inconclusive.',
      consequenceIfUnmet: 'A long-run coefficient is reported without a level relation.',
      sources: [PSS_2001, statsmodels('statsmodels/tsa/ardl/model.py#UECM.bounds_test')],
    },
    {
      id: caveatId('ardl-deterministic-case'),
      category: 'functional-form',
      requirement: 'The deterministic terms and PSS case are recorded: constant with case 2 or 3, trend with case 4 or 5.',
      consequenceIfUnmet: 'Critical values from another case give the wrong decision.',
      sources: [PSS_2001],
    },
  ],
}

const VECM: MethodDefinition = {
  id: VECM_METHOD_ID,
  name: 'VECM',
  family: 'estimation',
  summary: 'Johansen rank selection and maximum-likelihood error-correction fit; with one relation the outcome-normalised vector gives the long-run effect.',
  caveats: [
    {
      id: caveatId('vecm-all-i1'),
      category: 'stationarity-and-dynamics',
      requirement: 'Every modelled series is I(1).',
      consequenceIfUnmet: 'The rank test describes the wrong system.',
      sources: [JOHANSEN_1988, hirmos('src/domain/stationarityAssessment.ts#assessStationarity')],
    },
    {
      id: caveatId('vecm-rank'),
      category: 'interpretation',
      requirement: 'The trace test finds at least one relation; rank zero means no long-run relation.',
      consequenceIfUnmet: 'A level relation is read into series that only share a trend.',
      sources: [JOHANSEN_1988, statsmodels('statsmodels/tsa/vector_ar/vecm.py#select_coint_rank')],
    },
    {
      id: caveatId('vecm-single-relation'),
      category: 'interpretation',
      requirement: 'A long-run effect is read only at rank one; more relations are reported as matrices.',
      consequenceIfUnmet: 'A column from a multi-relation cointegrating matrix is incorrectly interpreted as a unique long-run effect.',
      sources: [statsmodels('statsmodels/tsa/vector_ar/vecm.py#VECM.fit')],
    },
    {
      id: caveatId('vecm-sample'),
      category: 'finite-sample',
      requirement: 'Enough rows for the variables and lags; deterministic terms and lag are recorded.',
      consequenceIfUnmet: 'Asymptotic critical values over-reject in a short sample.',
      sources: [statsmodels('statsmodels/tsa/vector_ar/vecm.py#select_order')],
    },
    {
      id: caveatId('vecm-no-interval'),
      category: 'finite-sample',
      requirement: 'The long-run vector comes without a standard error; alpha p-values speak to adjustment speed.',
      consequenceIfUnmet: 'A point long-run effect is presented as precise.',
      sources: [hirmos('crates/causal-core/src/vecm.rs')],
    },
  ],
}

const SYNTHETIC_CONTROL: MethodDefinition = {
  id: SYNTHETIC_CONTROL_METHOD_ID,
  name: 'Synthetic control',
  family: 'estimation',
  summary: 'Non-negative donor weights summing to one, fitted to the treated series before the intervention; the effect is the post-intervention gap.',
  caveats: [
    {
      id: caveatId('synthetic-panel-layout'),
      category: 'sampling-structure',
      requirement: 'Rows are periods, one column is the treated unit, donor columns are untreated units over the same periods.',
      consequenceIfUnmet: 'Weights fitted on unrelated series match noise.',
      sources: [ABADIE_2010],
    },
    {
      id: caveatId('synthetic-pre-period'),
      category: 'finite-sample',
      requirement: 'A known intervention row with enough pre-periods to fit the weights; the pre-period loss is reported.',
      consequenceIfUnmet: 'With a short pre-period, donor weights may fit idiosyncratic variation and generalise poorly after intervention.',
      sources: [ABADIE_2010, ABADIE_2021, hirmos('crates/causal-core/src/synthetic_control.rs#fit_synthetic_control')],
    },
    {
      id: caveatId('synthetic-donors-untreated'),
      category: 'identification',
      requirement: 'Donors are untouched by the intervention and are not DAG descendants of the treatment.',
      consequenceIfUnmet: 'A treated donor absorbs the effect.',
      sources: [ABADIE_2010, hirmos('src/domain/estimation.ts#affectedNodes')],
    },
    {
      id: caveatId('synthetic-convex-hull'),
      category: 'interpretation',
      requirement: 'The treated series lies inside the donors’ convex hull before the intervention; a large loss says it does not.',
      consequenceIfUnmet: 'The counterfactual extrapolates beyond any weighting of donors.',
      sources: [ABADIE_2010],
    },
    {
      id: caveatId('synthetic-no-interval'),
      category: 'finite-sample',
      requirement: 'No placebo distribution is ported, so no interval.',
      consequenceIfUnmet: 'A post gap is read as significant.',
      sources: [hirmos('crates/causal-core/src/synthetic_control.rs')],
    },
  ],
}

const PANEL_INTERVENTION: MethodDefinition = {
  id: PANEL_INTERVENTION_METHOD_ID,
  name: 'Panel DID / synthetic DID',
  family: 'estimation',
  summary: 'Estimates a simultaneous-adoption panel intervention with conventional DID, synthetic control, and synthetic difference-in-differences on the same validated panel.',
  caveats: [
    {
      id: caveatId('panel-balanced-layout'), category: 'sampling-structure',
      requirement: 'One observation per unit and period on a complete balanced panel; treated units adopt at the same period and remain treated.',
      consequenceIfUnmet: 'The implemented weighting system does not define the requested comparison.',
      sources: [ARKHANGELSKY_2021, CALLAWAY_SANTANNA_2021, synthdid('R/utils.R#panel.matrices')],
    },
    {
      id: caveatId('panel-parallel-trends'), category: 'identification',
      requirement: 'Absent treatment, treated and control outcomes would have followed parallel trends after accounting for the fitted unit and time weights.',
      consequenceIfUnmet: 'The post-period contrast combines the intervention with an untreated trend difference.',
      sources: [ARKHANGELSKY_2021, CALLAWAY_SANTANNA_2021],
    },
    {
      id: caveatId('panel-no-anticipation'), category: 'identification',
      requirement: 'Treatment does not affect outcomes before the recorded adoption period.',
      consequenceIfUnmet: 'Pre-treatment periods used to fit the counterfactual are already treated.',
      sources: [ARKHANGELSKY_2021],
    },
    {
      id: caveatId('panel-no-spillovers'), category: 'identification',
      requirement: 'Treatment of one unit does not change outcomes for control units.',
      consequenceIfUnmet: 'The controls no longer represent untreated potential outcomes.',
      sources: [ARKHANGELSKY_2021],
    },
    {
      id: caveatId('panel-pre-fit'), category: 'finite-sample',
      requirement: 'The control pre-period contains enough variation for the synthetic weights; report the unit and time weights and compare all three estimators.',
      consequenceIfUnmet: 'Synthetic weights may be unstable or undefined even when conventional DID is computable.',
      sources: [ARKHANGELSKY_2021, ABADIE_2021, synthdid('R/solver.R; R/synthdid.R')],
    },
    {
      id: caveatId('panel-no-interval'), category: 'finite-sample',
      requirement: 'No placebo, jackknife, or bootstrap variance estimator is included in this port, so the result has no sampling interval.',
      consequenceIfUnmet: 'A point estimate is interpreted as statistically precise.',
      sources: [synthdid('R/vcov.R'), hirmos('crates/causal-core/src/panel.rs')],
    },
  ],
}

const NEGBIN_NUTS: MethodDefinition = {
  id: NEGBIN_NUTS_METHOD_ID,
  name: 'Bayesian negative binomial',
  family: 'estimation',
  summary: 'A negative binomial count model with one treatment and one confounder, sampled by the no-U-turn sampler (NUTS). The effect is the posterior incidence rate ratio.',
  caveats: [
    {
      id: caveatId('nuts-model-shape'),
      category: 'identification',
      requirement: 'The ported model takes one confounder, so the identified adjustment set holds one variable.',
      consequenceIfUnmet: 'A second confounder is left out.',
      sources: [hirmos('crates/causal-core/src/negbin_nuts.rs#PbcNegBinModel'), hirmos('docs/DESIGN.md#10 Gamma-Poisson via NUTS')],
    },
    {
      id: caveatId('nuts-count-outcome'),
      category: 'functional-form',
      requirement: 'A non-negative integer outcome with log link; treatment and confounder are standardised first.',
      consequenceIfUnmet: 'The likelihood has no meaning.',
      sources: [CAMERON_TRIVEDI, hirmos('crates/causal-core/src/negbin_nuts.rs#log_prob_grad')],
    },
    {
      id: caveatId('nuts-convergence'),
      category: 'computation',
      requirement: 'No divergent transitions and acceptance near the 0.8 target; warmup, draws and seed are recorded.',
      consequenceIfUnmet: 'Posterior summaries from a chain with divergent transitions may not reliably represent the target distribution.',
      sources: [HOFFMAN_GELMAN_2014, BETANCOURT_2017, pyro('pyro/infer/mcmc/nuts.py:57-93'), hirmos('crates/causal-core/src/nuts.rs#multinomial_nuts')],
    },
    {
      id: caveatId('nuts-independence'),
      category: 'noise-and-dependence',
      requirement: 'Rows are independent; the model has no serial-correlation term.',
      consequenceIfUnmet: 'The posterior interval is too narrow.',
      sources: [hirmos('crates/causal-core/src/negbin_nuts.rs')],
    },
    {
      id: caveatId('nuts-interpretation'),
      category: 'interpretation',
      requirement: 'The rate ratio is the posterior median of exp(β / sd(treatment)) with 2.5% and 97.5% quantiles.',
      consequenceIfUnmet: 'A credible interval is read as a confidence interval.',
      sources: [NESS_CH11('§11.6 Bayesian estimation and credible intervals'), hirmos('crates/causal-core/src/negbin_nuts.rs#irr_summary')],
    },
  ],
}

const BAYESIAN_GAUSSIAN: MethodDefinition = {
  id: BAYESIAN_GAUSSIAN_METHOD_ID,
  name: 'Bayesian Gaussian regression',
  family: 'estimation',
  summary: 'A Gaussian regression of the outcome on the treatment and the identified adjustment set with Normal(0, 1) slope priors, sampled by the no-U-turn sampler (NUTS) over three chains. The effect is the posterior mean of the intervention contrast, with a 94% highest-density interval.',
  summaryTex: {
    tex: String.raw`\mathrm{ATE} = \mathbb{E}\bigl[\,Y \mid \mathrm{do}(T{=}1)\bigr] - \mathbb{E}\bigl[\,Y \mid \mathrm{do}(T{=}0)\bigr]`,
    plain: 'ATE = E[Y | do(T=1)] − E[Y | do(T=0)]',
  },
  caveats: [
    {
      id: caveatId('bayes-gaussian-identified-adjustment'),
      category: 'identification',
      requirement: 'The covariates are an identified back-door adjustment set for the treatment–outcome pair.',
      consequenceIfUnmet: 'The posterior summarises an association, not the intervention effect.',
      sources: [PEARL_2009('§3.3.1 back-door adjustment'), hirmos('crates/causal-core/src/bayesian_gaussian.rs#effect_draws')],
    },
    {
      id: caveatId('bayes-gaussian-binary-treatment'),
      category: 'functional-form',
      requirement: 'A 0/1 treatment; the outcome is linear and additive in treatment and covariates with one effect for every unit.',
      consequenceIfUnmet: 'Effect modification is averaged away or missed, and do(0) versus do(1) has no meaning.',
      sources: [hirmos('crates/causal-core/src/bayesian_gaussian.rs#BayesianGaussianScm'), paper('Structural Causal Models with PathMC (Orduz)', 'juanitorduz.github.io/intro_pathmc; two-equation Lalonde SCM and do(mean) semantics')],
    },
    {
      id: caveatId('bayes-gaussian-prior-scale'),
      category: 'functional-form',
      requirement: 'The Normal(0, 1) slope priors and half-normal(10) residual prior suit the outcome scale; non-binary adjustment columns are standardised first.',
      consequenceIfUnmet: 'On an outcome scale where plausible effects are large the prior shrinks the estimate toward zero.',
      sources: [paper('Structural Causal Models with PathMC (Orduz)', 'juanitorduz.github.io/intro_pathmc; priors and standardisation'), hirmos('crates/causal-core/src/bayesian_gaussian.rs#try_adjustment_log_prob_grad')],
    },
    {
      id: caveatId('bayes-gaussian-convergence'),
      category: 'computation',
      requirement: 'No divergent transitions and acceptance near the 0.8 target across all chains; warmup, draws and seed are recorded.',
      consequenceIfUnmet: 'Posterior summaries from a chain with divergent transitions may not reliably represent the target distribution.',
      sources: [HOFFMAN_GELMAN_2014, BETANCOURT_2017, hirmos('crates/causal-core/src/nuts.rs#multinomial_nuts')],
    },
    {
      id: caveatId('bayes-gaussian-independence'),
      category: 'noise-and-dependence',
      requirement: 'Rows are independent; the model has no serial-correlation term.',
      consequenceIfUnmet: 'The posterior interval is too narrow.',
      sources: [hirmos('crates/causal-core/src/bayesian_gaussian.rs')],
    },
    {
      id: caveatId('bayes-gaussian-interpretation'),
      category: 'interpretation',
      requirement: 'The interval is a 94% highest-density credible interval under the stated priors.',
      consequenceIfUnmet: 'A credible interval is read as a confidence interval.',
      sources: [NESS_CH11('§11.6 Bayesian estimation and credible intervals'), hirmos('crates/causal-core/src/bayesian_gaussian.rs#highest_density_interval')],
    },
  ],
}

const DISCRETE_BN: MethodDefinition = {
  id: DISCRETE_BN_METHOD_ID,
  name: 'Discrete Bayesian network do-query',
  family: 'estimation',
  summary: 'An ideal intervention cuts arrows into the treatment. With every parent measured, the fitted discrete Bayesian network computes do(low) and do(high) by adjustment for those parents.',
  caveats: [
    {
      id: caveatId('bn-discretisation'),
      category: 'functional-form',
      requirement: 'Quantile binning is a recorded meaning change; each state stands for the mean of its values.',
      consequenceIfUnmet: 'Within-bin variation is lost without a record.',
      sources: [NESS_CH4('§4.4.6 discretise continuous variables at quantiles'), pgmpy('pgmpy/inference/CausalInference.py'), hirmos('crates/causal-core/src/discrete_bn.rs#discretize_805')],
    },
    {
      id: caveatId('bn-observed-graph'),
      category: 'identification',
      requirement: 'Every node is measured; the query adjusts for the treatment’s parents.',
      consequenceIfUnmet: 'An unmeasured parent cannot be adjusted for.',
      sources: [PEARL_2009('§3.2 intervention and adjustment'), pgmpy('pgmpy/inference/CausalInference.py:35-51'), NESS_CH11('§11.6.1 graph surgery cuts the incoming edges'), hirmos('crates/causal-core/src/discrete_bn.rs#do_query')],
    },
    {
      id: caveatId('bn-sample-per-cell'),
      category: 'finite-sample',
      requirement: 'Enough rows per parent configuration; the Dirichlet pseudo-counts smooth sparse cells and are recorded.',
      consequenceIfUnmet: 'Posterior conditional probabilities are dominated by the Dirichlet prior in sparsely observed configurations.',
      sources: [paper('Causal AI (Ness, Manning), chapter 3', '§3.1.9 Dirichlet prior as smoothing'), hirmos('crates/causal-core/src/discrete_bn.rs#estimate_cpd')],
    },
    {
      id: caveatId('bn-independent-rows'),
      category: 'sampling-structure',
      requirement: 'Rows represent independent observational units; this network does not model repeated units or serial dependence.',
      consequenceIfUnmet: 'Repeated or serially dependent rows are treated as independent evidence, overstating the information in the conditional tables.',
      sources: [hirmos('crates/causal-core/src/discrete_bn.rs#DiscreteBayesianNetwork')],
    },
    {
      id: caveatId('bn-treatment-states'),
      category: 'interpretation',
      requirement: 'The effect is the expected outcome under the highest treatment bin minus the lowest.',
      consequenceIfUnmet: 'A bin contrast is compared with a per-unit coefficient.',
      sources: [hirmos('crates/causal-core/src/discrete_bn.rs#do_query')],
    },
  ],
}

const BINARY_ETT: MethodDefinition = {
  id: BINARY_ETT_METHOD_ID,
  name: 'Binary ETT by IDC*',
  family: 'estimation',
  summary: 'Evaluates the identified expressions for E[Y(1) | X=1] and E[Y(0) | X=1] against the observed binary joint distribution and reports their difference.',
  summaryTex: {
    tex: String.raw`\mathrm{ETT}=\mathbb{E}[Y(1)-Y(0)\mid X=1]`,
    plain: 'ETT = E[Y(1) − Y(0) | X=1]',
  },
  caveats: [
    {
      id: caveatId('ett-identified-expression'),
      category: 'identification',
      requirement: 'IDC* identifies both conditional potential-outcome distributions for the recorded graph.',
      consequenceIfUnmet: 'The observed distribution does not determine the requested ETT under the graph.',
      sources: [SHPITSER_PEARL_COUNTERFACTUAL, NESS_CH10('ID* and IDC*')],
    },
    {
      id: caveatId('ett-binary-table'),
      category: 'functional-form',
      requirement: 'Treatment, outcome, and every other observed graph variable are recorded as 0 or 1; no automatic discretisation is applied.',
      consequenceIfUnmet: 'The empirical binary distribution does not represent the supplied variables.',
      sources: [SHPITSER_PEARL_COUNTERFACTUAL, hirmos('crates/causal-core/src/counterfactual_evaluator.rs#estimate_binary_ett')],
    },
    {
      id: caveatId('ett-positive-conditioning-mass'),
      category: 'finite-sample',
      requirement: 'Treated observations and every conditional configuration used by the identified expressions have positive observed mass.',
      consequenceIfUnmet: 'The plug-in conditional probability has a zero denominator and the run is refused.',
      sources: [PEARL_2009('§3.2 positivity for adjustment'), NESS_CH11('positivity and support')],
    },
    {
      id: caveatId('ett-independent-rows'),
      category: 'sampling-structure',
      requirement: 'Rows are independent observational units.',
      consequenceIfUnmet: 'Repeated or serially dependent rows are counted as independent observations in the empirical distribution.',
      sources: [NESS_CH11('observational estimation assumptions')],
    },
    {
      id: caveatId('ett-no-interval'),
      category: 'interpretation',
      requirement: 'The result is a plug-in point estimate; this implementation does not report a sampling interval.',
      consequenceIfUnmet: 'The point estimate is presented with unsupported precision.',
      sources: [hirmos('crates/causal-core/src/counterfactual_evaluator.rs#estimate_binary_ett')],
    },
  ],
}

const LINEAR_SCM: MethodDefinition = {
  id: LINEAR_SCM_METHOD_ID,
  name: 'Linear SCM counterfactual',
  family: 'counterfactual',
  summary: 'Fits each node by least squares on its parents, infers an observation-specific disturbance term, and predicts the outcome under two treatment values.',
  caveats: [
    {
      id: caveatId('scm-structural-equations'),
      category: 'functional-form',
      requirement: 'Each equation is linear in its parents with additive noise; the fitted coefficients and residual scales are the model.',
      consequenceIfUnmet: 'Abduction inverts the wrong model.',
      sources: [RUIZ_DE_VILLA_CH7('§7.1.3 structural causal models'), RUIZ_DE_VILLA_CH8('§8.1.1 linear structural equations')],
    },
    {
      id: caveatId('scm-graph-complete'),
      category: 'identification',
      requirement: 'Every node is measured and the DAG is the whole structure.',
      consequenceIfUnmet: 'The estimated disturbance term may absorb the effect of an omitted common cause.',
      sources: [RUIZ_DE_VILLA_CH7('§7.1.3 omitting an arrow asserts its absence'), PEARL_2009('§7.1 abduction')],
    },
    {
      id: caveatId('scm-abduction'),
      category: 'interpretation',
      requirement: 'The row-specific disturbance terms are inferred by exact inversion of the fitted equations, or by a Gaussian posterior mean under the stated observation-noise model.',
      consequenceIfUnmet: 'Observation-specific disturbance estimates depend on an undocumented observation-noise model.',
      sources: [PEARL_2009('§7.1 abduction, action, prediction'), hirmos('crates/causal-core/src/counterfactual.rs#abduct_exact')],
    },
    {
      id: caveatId('scm-modularity'),
      category: 'interpretation',
      requirement: 'Intervention replaces the treatment equation while every other structural mechanism remains invariant; the same inferred disturbance term is carried into both treatment worlds for each row.',
      consequenceIfUnmet: 'The two predictions do not describe the same unit under alternative interventions.',
      sources: [PEARL_2009('§7.1 abduction, action, prediction'), paper('Causal AI (Ness, Manning), chapter 13', '§13.1.6 counterfactual notation and structural mechanisms')],
    },
    {
      id: caveatId('scm-contemporaneous'),
      category: 'stationarity-and-dynamics',
      requirement: 'Equations are same-period; lagged arrows are collapsed.',
      consequenceIfUnmet: 'A dynamic effect is read as a same-row effect.',
      sources: [hirmos('src/domain/study.ts#studyGraphOf')],
    },
    {
      id: caveatId('scm-reading'),
      category: 'interpretation',
      requirement: 'Row-level differences are model-implied individual effects under the fitted SCM; neither counterfactual outcome is directly observed.',
      consequenceIfUnmet: 'A model-implied counterfactual is presented as a measurement.',
      sources: [PEARL_2009('§7.1 prediction step'), paper('Causal AI (Ness, Manning), chapter 13', '§13.1.6 counterfactual notation')],
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
  DIRECT_LINGAM,
  VAR_LINGAM,
  OCSE,
  BACKDOOR_IDENTIFICATION,
  GRAPHICAL_IDENTIFICATION,
  COUNTERFACTUAL_IDENTIFICATION,
  BACKDOOR_LINEAR_REGRESSION,
  FRONTDOOR_TWO_STAGE,
  POISSON_GLM,
  NEGATIVE_BINOMIAL,
  CAUSAL_EFFECTS_TOTAL,
  CAUSAL_IMPACT,
  DML_PLR,
  DML_IRM,
  DML_REFUTATION,
  ARDL_PSS,
  VECM,
  SYNTHETIC_CONTROL,
  PANEL_INTERVENTION,
  NEGBIN_NUTS,
  BAYESIAN_GAUSSIAN,
  DISCRETE_BN,
  BINARY_ETT,
  LINEAR_SCM,
  PLACEBO_REFUTER,
  DATA_SUBSET_REFUTER,
  RANDOM_COMMON_CAUSE_REFUTER,
  UNOBSERVED_COMMON_CAUSE,
  LJUNG_BOX,
  SHAPIRO_WILK,
  PELT,
  STL,
]

export const REFUTER_METHODS: NonEmptyArray<MethodDefinition> = [PLACEBO_REFUTER, DATA_SUBSET_REFUTER, RANDOM_COMMON_CAUSE_REFUTER, UNOBSERVED_COMMON_CAUSE]
export const SENSITIVITY_DIAGNOSTIC_METHODS: NonEmptyArray<MethodDefinition> = [LJUNG_BOX, SHAPIRO_WILK]
export const SERIES_STRUCTURE_METHODS: NonEmptyArray<MethodDefinition> = [PELT, STL]

export const IDENTIFICATION_METHODS: NonEmptyArray<MethodDefinition> = [BACKDOOR_IDENTIFICATION, GRAPHICAL_IDENTIFICATION, COUNTERFACTUAL_IDENTIFICATION]
export const COUNTERFACTUAL_METHODS: NonEmptyArray<MethodDefinition> = [LINEAR_SCM]

export const DML_SENSITIVITY_METHODS: NonEmptyArray<MethodDefinition> = [DML_REFUTATION]

export const ESTIMATION_METHODS: NonEmptyArray<MethodDefinition> = [BACKDOOR_LINEAR_REGRESSION, FRONTDOOR_TWO_STAGE, BAYESIAN_GAUSSIAN, POISSON_GLM, NEGATIVE_BINOMIAL, NEGBIN_NUTS, DML_PLR, DML_IRM, CAUSAL_EFFECTS_TOTAL, CAUSAL_IMPACT, SYNTHETIC_CONTROL, PANEL_INTERVENTION, ARDL_PSS, VECM, DISCRETE_BN, BINARY_ETT]

export const STATIONARITY_METHODS: NonEmptyArray<MethodDefinition> = [ADF, KPSS, ZIVOT_ANDREWS]
export const CROSS_SECTIONAL_DISCOVERY_METHODS: NonEmptyArray<MethodDefinition> = [DIRECT_LINGAM]
export const TEMPORAL_DISCOVERY_METHODS: NonEmptyArray<MethodDefinition> = [
  GRANGER_SSR_F,
  PCMCI_PLUS_PAR_CORR,
  LPCMCI_PAR_CORR,
  DYNOTEARS,
  VAR_LINGAM,
  OCSE,
]

export function methodDefinition(id: MethodId): Result<MethodDefinition, MethodLookupProblem> {
  const definition = METHOD_CATALOG.find((candidate) => candidate.id === id)
  return definition ? ok(definition) : err({ kind: 'unknown-method', id })
}
