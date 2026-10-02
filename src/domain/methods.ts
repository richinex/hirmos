import { brand, err, mapNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { LevelIssueAction } from './stationarityAssessment'

export type MethodId = Brand<string, 'MethodId'>
export type MethodCaveatId = Brand<string, 'MethodCaveatId'>

export type MethodFamily = 'diagnostic' | 'discovery' | 'identification' | 'estimation' | 'refuter' | 'counterfactual'

export type MethodSource =
  | {
      readonly kind: 'reference-implementation'
      readonly repository: 'statsmodels' | 'tigramite' | 'lingam' | 'dowhy' | 'pyro' | 'ruptures' | 'pgmpy' | 'econml' | 'causationentropy' | 'causalnex' | 'synthdid' | 'linearmodels'
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

/**
 * Series that share one reason for review. The stage shows the summary line; the requirements panel
 * heads its list of the series with the reason, so a series is named in one place however many there are.
 */
export interface EvidenceGroup {
  readonly summary: string
  readonly reason: string
  readonly series: NonEmptyArray<{ readonly name: string; readonly detail: string | null }>
  readonly action: LevelIssueAction | null
}

export type CaveatEvaluation =
  | { readonly kind: 'satisfied'; readonly caveat: MethodCaveat; readonly evidence: string }
  | { readonly kind: 'unresolved'; readonly caveat: MethodCaveat; readonly missingEvidence: string; readonly groups?: NonEmptyArray<EvidenceGroup> }
  | { readonly kind: 'violated'; readonly caveat: MethodCaveat; readonly evidence: string }

/** An unresolved condition the stage states beside the run button, one line per group of series. */
export const stageGroups = (evaluation: CaveatEvaluation): readonly EvidenceGroup[] =>
  evaluation.kind === 'unresolved' ? evaluation.groups ?? [] : []

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
const LINEARMODELS_REVISION = 'v6.1'
const linearmodels = (locator: string): MethodSource => ({ kind: 'reference-implementation', repository: 'linearmodels', revision: LINEARMODELS_REVISION, locator })

const hirmos = (locator: string): MethodSource => ({ kind: 'hirmos-constraint', locator })

export const ADF_METHOD_ID = methodId('adf')
export const KPSS_METHOD_ID = methodId('kpss')
export const ZIVOT_ANDREWS_METHOD_ID = methodId('zivot-andrews')
export const GRANGER_SSR_F_METHOD_ID = methodId('granger-ssr-f')
export const COUNT_SERIES_INTERVENTION_SCAN_METHOD_ID = methodId('count-series-intervention-scan')
export const PCMCI_PLUS_PAR_CORR_METHOD_ID = methodId('pcmci-plus-parcorr')
export const JPCMCI_PLUS_PAR_CORR_METHOD_ID = methodId('jpcmciplus-parcorr-mult')
export const LPCMCI_PAR_CORR_METHOD_ID = methodId('lpcmci-parcorr')
export const RPCMCI_PAR_CORR_METHOD_ID = methodId('rpcmci-parcorr')
export const CDNOTS_PAR_CORR_METHOD_ID = methodId('cdnots-parcorr')
export const CDNOTS_PLUS_PAR_CORR_METHOD_ID = methodId('cdnots-plus-parcorr')
export const GRACE_METHOD_ID = methodId('grace')
export const DYNOTEARS_METHOD_ID = methodId('dynotears')
export const DIRECT_LINGAM_METHOD_ID = methodId('direct-lingam')
export const PC_STABLE_METHOD_ID = methodId('pc-stable')
export const FCI_METHOD_ID = methodId('fci')
export const VAR_LINGAM_METHOD_ID = methodId('var-lingam')
export const OCSE_METHOD_ID = methodId('ocse')
export const CMLP_METHOD_ID = methodId('neural-granger-cmlp')
export const CLSTM_METHOD_ID = methodId('neural-granger-clstm')
export const BACKDOOR_IDENTIFICATION_METHOD_ID = methodId('backdoor-identification')
export const GRAPHICAL_IDENTIFICATION_METHOD_ID = methodId('graphical-identification-id')
export const COUNTERFACTUAL_IDENTIFICATION_METHOD_ID = methodId('counterfactual-identification-id-star')
export const BACKDOOR_LINEAR_REGRESSION_METHOD_ID = methodId('backdoor-linear-regression')
export const FRONTDOOR_TWO_STAGE_METHOD_ID = methodId('frontdoor-two-stage')
export const INSTRUMENTAL_VARIABLE_METHOD_ID = methodId('instrumental-variable')
export const POISSON_GLM_METHOD_ID = methodId('poisson-glm')
export const NEGATIVE_BINOMIAL_METHOD_ID = methodId('negative-binomial-p')
export const NEGATIVE_BINOMIAL_INGARCH_METHOD_ID = methodId('negative-binomial-ingarch')
export const CAUSAL_EFFECTS_TOTAL_METHOD_ID = methodId('causal-effects-total')
export const CAUSAL_IMPACT_METHOD_ID = methodId('causal-impact')
export const DML_PLR_METHOD_ID = methodId('dml-plr')
export const DML_IRM_METHOD_ID = methodId('dml-irm')
export const T_LEARNER_METHOD_ID = methodId('t-learner')
export const CAUSAL_FOREST_METHOD_ID = methodId('causal-forest')
export const DML_REFUTATION_METHOD_ID = methodId('dml-refutation-batch')
export const ARDL_PSS_METHOD_ID = methodId('ardl-pss')
export const VECM_METHOD_ID = methodId('vecm')
export const INTERRUPTED_SERIES_METHOD_ID = methodId('interrupted-series')
export const SYNTHETIC_CONTROL_METHOD_ID = methodId('synthetic-control')
export const PANEL_INTERVENTION_METHOD_ID = methodId('panel-intervention')
export const SHARP_RD_METHOD_ID = methodId('sharp-rd')
export const RD_DESIGN_METHOD_ID = methodId('sharp-rd-design')
export const PROPENSITY_WEIGHTING_METHOD_ID = methodId('propensity-weighting')
export const PROPENSITY_MATCHING_METHOD_ID = methodId('propensity-matching')
export const DOUBLY_ROBUST_METHOD_ID = methodId('doubly-robust')
export const CONTINUOUS_GPS_METHOD_ID = methodId('continuous-gps')

const SHARP_RD: MethodDefinition = {
  id: SHARP_RD_METHOD_ID, name: 'Sharp regression discontinuity', family: 'estimation',
  summary: 'Estimate the treatment effect at an assignment cutoff with local-linear regressions, automatic bandwidth selection and robust bias-corrected inference.',
  caveats: [
    { id: caveatId('rd-design'), category: 'identification', requirement: 'Treatment switches from 0 below the cutoff to 1 at or above it; both potential-outcome means are continuous there.', consequenceIfUnmet: 'The outcome discontinuity need not identify a treatment effect.', sources: [{ kind: 'paper', title: 'Robust nonparametric confidence intervals for regression-discontinuity designs', locator: 'https://doi.org/10.3982/ECTA11757' }] },
    { id: caveatId('rd-sample'), category: 'sampling-structure', requirement: 'Observations are independently sampled with adequate running-variable support on both sides.', consequenceIfUnmet: 'Nearest-neighbor uncertainty does not account for serial or within-unit dependence.', sources: [{ kind: 'hirmos-constraint', locator: 'crates/causal-core/src/rd.rs; sharp local-linear, triangular kernel, NN(3), mserd' }] },
  ],
}
export const NEGBIN_NUTS_METHOD_ID = methodId('negbin-nuts')
const RD_DESIGN: MethodDefinition = { ...SHARP_RD, id: RD_DESIGN_METHOD_ID, name: 'Sharp RD design', family: 'identification', summary: 'Record a cutoff-local target and continuity assumptions. A graph alone does not verify these assumptions.' }
export const BAYESIAN_GAUSSIAN_METHOD_ID = methodId('bayesian-gaussian')
export const DISCRETE_BN_METHOD_ID = methodId('discrete-bn-query')
export const BINARY_ETT_METHOD_ID = methodId('binary-ett-idc-star')
export const LINEAR_SCM_METHOD_ID = methodId('linear-scm-counterfactual')
export const DYNAMIC_LINEAR_SCM_METHOD_ID = methodId('dynamic-linear-scm-counterfactual')
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
const RUIZ_DE_VILLA_CH5 = (locator: string): MethodSource => paper('Causal Inference for Data Science (Aleix Ruiz de Villa, Manning), chapter 5', locator)
const RUIZ_DE_VILLA_CH7 = (locator: string): MethodSource => paper('Causal Inference for Data Science (Aleix Ruiz de Villa, Manning), chapter 7', locator)
const RUIZ_DE_VILLA_CH8 = (locator: string): MethodSource => paper('Causal Inference for Data Science (Aleix Ruiz de Villa, Manning), chapter 8', locator)
const NESS_CH4 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 4', locator)
const NESS_CH11 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 11', locator)
const NESS_CH10 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 10', locator)
const NESS_CH13 = (locator: string): MethodSource => paper('Causal AI (Ness, Manning), chapter 13', locator)
const MOLAK_CH6 = (locator: string): MethodSource => paper('Causal Inference and Discovery in Python (Molak, Packt), chapter 6', locator)
const MOLAK_CH9 = (locator: string): MethodSource => paper('Causal Inference and Discovery in Python (Molak, Packt), chapter 9', locator)
const KUNZEL_2019 = paper('Metalearners for estimating heterogeneous treatment effects using machine learning (Künzel, Sekhon, Bickel and Yu, 2019)', 'PNAS 116(10); the T-learner')
const ECONML_TLEARNER = paper('EconML: a Python package for ML-based heterogeneous treatment effects estimation (Battocchi and others)', 'econml.metalearners.TLearner, version 0.17')
const CIR_CH10 = (locator: string): MethodSource => paper('Causal Inference in R (Packt, 2024), chapter 10', locator)
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
const FACURE_CH5 = (locator: string): MethodSource => paper('Causal Inference in Python (Matheus Facure, O\u2019Reilly, 2023), chapter 5', locator)
const ROSENBAUM_RUBIN_1983 = paper('The central role of the propensity score in observational studies for causal effects (Rosenbaum and Rubin, 1983)', 'Biometrika 70(1), 41\u201355')
const HIRANO_IMBENS_2004 = paper('The propensity score with continuous treatments (Hirano and Imbens, 2004)', 'Applied Bayesian Modeling and Causal Inference, 73\u201384')
const NEWEY_WEST_1987 = paper('A Simple, Positive Semi-definite, Heteroskedasticity and Autocorrelation Consistent Covariance Matrix (Newey and West, 1987)', 'Econometrica 55(3), 703–708')
const MACKINNON_WHITE_1985 = paper('Some heteroskedasticity-consistent covariance matrix estimators with improved finite sample properties (MacKinnon and White, 1985)', 'Journal of Econometrics 29(3), 305–325')
const CAMERON_MILLER_2015 = paper('A Practitioner’s Guide to Cluster-Robust Inference (Cameron and Miller, 2015)', 'Journal of Human Resources 50(2), 317–372')
const WOOLDRIDGE_2010 = paper('Econometric Analysis of Cross Section and Panel Data, second edition (Wooldridge, 2010)', 'MIT Press, §10.5 fixed effects methods')
const GRANGER_NEWBOLD_1974 = paper('Spurious regressions in econometrics (Granger and Newbold, 1974)', 'Journal of Econometrics 2(2), 111–120')
const DICKEY_FULLER_1979 = paper('Distribution of the Estimators for Autoregressive Time Series with a Unit Root (Dickey and Fuller, 1979)', 'Journal of the American Statistical Association 74(366), 427–431')
const KPSS_1992 = paper('Testing the Null Hypothesis of Stationarity against the Alternative of a Unit Root (Kwiatkowski, Phillips, Schmidt and Shin, 1992)', 'Journal of Econometrics 54(1–3), 159–178')
const ZIVOT_ANDREWS_1992 = paper('Further Evidence on the Great Crash, the Oil-Price Shock, and the Unit-Root Hypothesis (Zivot and Andrews, 1992)', 'Journal of Business & Economic Statistics 10(3), 251–270')
const GRANGER_1969 = paper('Investigating Causal Relations by Econometric Models and Cross-spectral Methods (Granger, 1969)', 'Econometrica 37(3), 424–438')
const PSS_2001 = paper('Bounds Testing Approaches to the Analysis of Level Relationships (Pesaran, Shin and Smith, 2001)', 'Journal of Applied Econometrics 16(3), 289–326; cases I–V')
const HERRERA_ITS = paper('Why interrupted time series might be your new favorite data tool (Herrera, Medium)', 'https://medium.com/@JuanPabloHerrera/why-interrupted-time-series-might-be-your-new-favorite-data-tool-399d260a9cd9')
const LOPEZ_BERNAL_2017 = paper('Interrupted time series regression for the evaluation of public health interventions: a tutorial (Lopez Bernal, Cummins and Gasparrini, 2017; corrigendum 2020)', 'International Journal of Epidemiology 46(1), 348–355')
const BHASKARAN_2013 = paper('Time series regression studies in environmental epidemiology (Bhaskaran, Gasparrini, Hajat, Smeeth and Armstrong, 2013)', 'International Journal of Epidemiology 42(4), 1187–1195')
const JOHANSEN_1988 = paper('Statistical Analysis of Cointegration Vectors (Johansen, 1988)', 'Journal of Economic Dynamics and Control 12(2–3), 231–254')
const CAMERON_TRIVEDI = paper('Regression Analysis of Count Data, 2nd ed. (Cameron and Trivedi, 2013)', '§§3.2–3.5; overdispersion in §3.4')
const HOFFMAN_GELMAN_2014 = paper('The No-U-Turn Sampler: Adaptively Setting Path Lengths in Hamiltonian Monte Carlo (Hoffman and Gelman, 2014)', 'Journal of Machine Learning Research 15(47), 1593–1623')
const BETANCOURT_2017 = paper('A Conceptual Introduction to Hamiltonian Monte Carlo (Betancourt, 2017)', 'arXiv:1701.02434; multinomial trajectory sampling')
const ABADIE_2010 = paper('Synthetic Control Methods for Comparative Case Studies (Abadie, Diamond and Hainmueller, 2010)', 'Journal of the American Statistical Association 105(490), 493–505')
const ABADIE_2021 = paper('Using Synthetic Controls: Feasibility, Data Requirements, and Methodological Aspects (Abadie, 2021)', 'Journal of Economic Literature 59(2), 391–425')
const CALLAWAY_SANTANNA_2021 = paper('Difference-in-Differences with multiple time periods (Callaway and Sant\'Anna, 2021)', 'Journal of Econometrics 225(2), 200–230; simultaneous adoption as the one-cohort case')
const ARKHANGELSKY_2021 = paper('Synthetic Difference-in-Differences (Arkhangelsky, Athey, Hirshberg, Imbens and Wager, 2021)', 'American Economic Review 111(12), 4088–4118')
const BRODERSEN_2015 = paper('Inferring causal impact using Bayesian structural time-series models (Brodersen and others, 2015)', 'Annals of Applied Statistics 9(1), 247–274')
const KUNSCH_1989 = paper('The Jackknife and the Bootstrap for General Stationary Observations (Künsch, 1989)', 'Annals of Statistics 17(3), 1217–1241; doi:10.1214/aos/1176347265')
const CHERNOZHUKOV_OVB = paper('Long Story Short: Omitted Variable Bias in Causal Machine Learning (Chernozhukov, Cinelli, Newey, Sharma and Syrgkanis, 2022)', 'NBER Working Paper 30302; sensitivity bounds')
const HYVARINEN_2010 = paper('Estimation of a Structural Vector Autoregression Model Using Non-Gaussianity (Hyvärinen, Zhang, Shimizu and Hoyer, 2010)', 'Journal of Machine Learning Research 11, 1709–1731')
const SHIMIZU_2011 = paper('DirectLiNGAM: A Direct Method for Learning a Linear Non-Gaussian Structural Equation Model (Shimizu and others, 2011)', 'Journal of Machine Learning Research 12, 1225–1248')
const SPIRTES_2000 = paper('Causation, Prediction, and Search, 2nd ed. (Spirtes, Glymour and Scheines, 2000)', 'MIT Press; PC and FCI algorithms')
const COLOMBO_MAATHUIS_2014 = paper('Order-independent constraint-based causal structure learning (Colombo and Maathuis, 2014)', 'Journal of Machine Learning Research 15, 3921–3962')
const SPIRTES_MEEK_RICHARDSON_1995 = paper('Causal inference in the presence of latent variables and selection bias (Spirtes, Meek and Richardson, 1995)', 'Proceedings of UAI 1995, 499–506; doi:10.7551/mitpress/2006.003.0009')
const ZHANG_2008 = paper('On the completeness of orientation rules for causal discovery in the presence of latent confounders and selection bias (Zhang, 2008)', 'Artificial Intelligence 172(16–17), 1873–1896')
const ZHANG_KCI_2011 = paper('Kernel-based conditional independence test and application in causal discovery (Zhang, Peters, Janzing and Schölkopf, 2011)', 'Proceedings of UAI 2011, 804–813')
const PAMFIL_2020 = paper('DYNOTEARS: Structure Learning from Time-Series Data (Pamfil and others, 2020)', 'Proceedings of AISTATS, PMLR 108, 1595–1605')
const SUN_2015 = paper('Causal Network Inference by Optimal Causation Entropy (Sun, Taylor and Bollt, 2015)', 'SIAM Journal on Applied Dynamical Systems 14(1), 65–83')
const RUNGE_2020 = paper('Discovering contemporaneous and lagged causal relations in autocorrelated nonlinear time series datasets (Runge, 2020)', 'Proceedings of UAI, PMLR 124; PCMCI+')
const GERHARDUS_RUNGE_2020 = paper('High-recall causal discovery for autocorrelated time series with latent confounders (Gerhardus and Runge, 2020)', 'Advances in Neural Information Processing Systems 33; LPCMCI')
const SAGGIORO_2020 = paper('Reconstructing regime-dependent causal relationships from observational time series (Saggioro, de Wiljes, Kretschmer and Runge, 2020)', 'Chaos 30(11), 113115; doi:10.1063/5.0020538')
const TANK_2021 = paper('Neural Granger Causality (Tank, Covert, Foti, Shojaie and Fox, 2021)', 'IEEE Transactions on Pattern Analysis and Machine Intelligence 44(8), 4267–4279; arXiv:1802.05842')
const CDNOTS_2025 = paper('Causal Discovery from Nonstationary Time Series (Sadeghi, Gopal and Fesanghary, 2025)', 'International Journal of Data Science and Analytics 19, 33–59; doi:10.1007/s41060-024-00679-7')
const GRACE_2026 = paper('GRACE: Gated Refinement for Accurate Causal Edge Discovery in High-Dimensional Time Series (Fesanghary and Havaldar, 2026)', 'arXiv:2606.23880')
const LJUNG_BOX_1978 = paper('On a Measure of Lack of Fit in Time Series Models (Ljung and Box, 1978)', 'Biometrika 65(2), 297–303')
const SHAPIRO_WILK_1965 = paper('An Analysis of Variance Test for Normality (Complete Samples) (Shapiro and Wilk, 1965)', 'Biometrika 52(3/4), 591–611')
const KILLICK_2012 = paper('Optimal Detection of Changepoints with a Linear Computational Cost (Killick, Fearnhead and Eckley, 2012)', 'Journal of the American Statistical Association 107(500), 1590–1598; PELT')
const CLEVELAND_1990 = paper('STL: A Seasonal-Trend Decomposition Procedure Based on Loess (Cleveland, Cleveland, McRae and Terpenning, 1990)', 'Journal of Official Statistics 6(1), 3–73')
const HYNDMAN_FPP = paper('Forecasting: Principles and Practice, 3rd ed. (Hyndman and Athanasopoulos, 2021)', '§4.3, strength of trend and seasonality')
const HYNDMAN_FPP_DHR = paper('Forecasting: Principles and Practice, 3rd ed. (Hyndman and Athanasopoulos, 2021)', '§10.5, dynamic harmonic regression: Fourier terms for the season with ARMA errors')
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
      requirement: 'The null is stationarity around a constant or trend, as selected. Rejection is evidence against that form of stationarity, not proof of a unit root.',
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
      requirement: 'The lag is chosen once on the base model, not at every candidate break.',
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
      requirement: 'Rejection provides evidence of incremental predictive information at the tested lag order, not an identified intervention effect.',
      consequenceIfUnmet: 'Predictive precedence is presented as a causal effect.',
      sources: [GRANGER_1969],
    },
  ],
}

const COUNT_SERIES_INTERVENTION_SCAN: MethodDefinition = {
  id: COUNT_SERIES_INTERVENTION_SCAN_METHOD_ID,
  name: 'Negative-binomial INGARCH diagnostic',
  family: 'diagnostic',
  summary: 'Fits a negative-binomial model to a count series and searches the selected date range for a temporary, fading or persistent change.',
  caveats: [
    {
      id: caveatId('count-scan-regular-count-series'),
      category: 'sampling-structure',
      requirement: 'Use non-negative whole-number counts recorded at regular intervals.',
      consequenceIfUnmet: 'The model is not suitable for these values or their timing.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', 'Journal of Statistical Software 82(5), §2')],
    },
    {
      id: caveatId('count-scan-stable-recursion'),
      category: 'stationarity-and-dynamics',
      requirement: 'Check that the fitted model is stable and accounts for how counts depend on earlier observations.',
      consequenceIfUnmet: 'An apparent change may reflect patterns the model missed.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', 'Journal of Statistical Software 82(5), §§2–3')],
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

const JPCMCI_PLUS_PAR_CORR: MethodDefinition = {
  id: JPCMCI_PLUS_PAR_CORR_METHOD_ID,
  name: 'J-PCMCI+ with ParCorrMult',
  family: 'discovery',
  summary: 'Learns one joint time-series CPDAG from aligned panel units while separating system variables from observed and generated time or unit context.',
  caveats: [
    {
      id: caveatId('jpcmciplus-balanced-panel'),
      category: 'sampling-structure',
      requirement: 'Every panel unit is observed on the same ordered period grid, with one row per unit and period.',
      consequenceIfUnmet: 'Joint lagged samples compare different periods or give some units more influence.',
      sources: [tigramite('tigramite/jpcmciplus.py'), tigramite('tigramite/data_processing.py')],
    },
    {
      id: caveatId('jpcmciplus-context-roles'),
      category: 'identification',
      requirement: 'Time-context variables are shared across units at each period, space-context variables are constant within each unit, and at least two variables remain system variables.',
      consequenceIfUnmet: 'The joint causal sufficiency assumptions are applied to variables with the wrong invariance structure.',
      sources: [tigramite('tigramite/jpcmciplus.py')],
    },
    {
      id: caveatId('jpcmciplus-stationary-system'),
      category: 'stationarity-and-dynamics',
      requirement: 'After the declared context variables account for observed heterogeneity, one lagged system graph applies across units and periods.',
      consequenceIfUnmet: 'The result merges changing system mechanisms into one graph.',
      sources: [tigramite('tigramite/jpcmciplus.py')],
    },
    {
      id: caveatId('jpcmciplus-parcorr-mult'),
      category: 'functional-form',
      requirement: 'ParCorrMult’s linear-Gaussian conditional-independence model is adequate for the scalar and vector-valued variables.',
      consequenceIfUnmet: 'Conditional-independence decisions and orientations can be wrong.',
      sources: [tigramite('tigramite/independence_tests/parcorr_mult.py')],
    },
    {
      id: caveatId('jpcmciplus-cpdag-reading'),
      category: 'interpretation',
      requirement: 'Read the output as a joint time-series CPDAG; auxiliary context nodes explain heterogeneity and unresolved endpoints do not identify a unique DAG.',
      consequenceIfUnmet: 'Context evidence or an unresolved endpoint is presented as a uniquely established causal direction.',
      sources: [tigramite('tigramite/jpcmciplus.py')],
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

const RPCMCI_PAR_CORR: MethodDefinition = {
  id: RPCMCI_PAR_CORR_METHOD_ID,
  name: 'RPCMCI with ParCorr',
  family: 'discovery',
  summary: 'Alternates regime assignment with PCMCI to estimate a separate lagged graph for each persistent regime.',
  caveats: [
    {
      id: caveatId('rpcmci-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Observations form one scalar, regularly ordered time series; the analysis mask is not used by RPCMCI.',
      consequenceIfUnmet: 'Lagged tests and the regime transition constraint refer to the wrong observation sequence.',
      sources: [SAGGIORO_2020],
    },
    {
      id: caveatId('rpcmci-persistent-discrete-regimes'),
      category: 'stationarity-and-dynamics',
      requirement: 'A finite, persistent background regime exists, and the causal relations are stationary within each regime.',
      consequenceIfUnmet: 'The fitted partition can combine distinct processes or split gradual change into artificial regimes.',
      sources: [SAGGIORO_2020],
    },
    {
      id: caveatId('rpcmci-regime-specification'),
      category: 'identification',
      requirement: 'The assumed number of regimes and maximum transitions are substantively plausible and assessed across alternative settings.',
      consequenceIfUnmet: 'The transition budget predetermines a partition that can change the regime-specific graphs.',
      sources: [SAGGIORO_2020],
    },
    {
      id: caveatId('rpcmci-parcorr-form'),
      category: 'functional-form',
      requirement: 'Within each regime, ParCorr and the linear prediction model adequately describe the conditional relations.',
      consequenceIfUnmet: 'Regime assignments and PCMCI p-values can both be misspecified.',
      sources: [SAGGIORO_2020],
    },
    {
      id: caveatId('rpcmci-local-optima'),
      category: 'finite-sample',
      requirement: 'Multiple seeded annealings are compared because the alternating optimization can reach different solutions.',
      consequenceIfUnmet: 'A single initialization can be reported as if it were the stable regime solution.',
      sources: [SAGGIORO_2020],
    },
    {
      id: caveatId('rpcmci-result-reading'),
      category: 'interpretation',
      requirement: 'Regime labels are exchangeable; interpret the membership paths and the corresponding graph together.',
      consequenceIfUnmet: 'A label permutation is mistaken for a substantive difference between runs.',
      sources: [SAGGIORO_2020],
    },
  ],
}

const CDNOTS_PAR_CORR: MethodDefinition = {
  id: CDNOTS_PAR_CORR_METHOD_ID,
  name: 'CD-NOTS with ParCorr',
  family: 'discovery',
  summary: 'Extends constraint-based discovery to lagged data and uses an explicit time-context variable to orient relations associated with changing mechanisms.',
  caveats: [
    {
      id: caveatId('cdnots-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows follow one regular temporal grid, and the selected maximum lag covers the relevant history.',
      consequenceIfUnmet: 'Embedded variables represent the wrong time intervals or omit relevant lagged relations.',
      sources: [CDNOTS_2025],
    },
    {
      id: caveatId('cdnots-markov-faithfulness'),
      category: 'identification',
      requirement: 'The causal Markov, faithfulness and causal-sufficiency assumptions used by the method are credible for the measured system.',
      consequenceIfUnmet: 'Conditional independences need not correspond to the reported causal adjacencies and orientations.',
      sources: [CDNOTS_2025],
    },
    {
      id: caveatId('cdnots-context-basis'),
      category: 'stationarity-and-dynamics',
      requirement: 'The selected time-context basis represents the mechanism changes relevant to the study window.',
      consequenceIfUnmet: 'The nonstationarity-orientation phase can omit changes or attribute an unsuitable pattern to them.',
      sources: [CDNOTS_2025],
    },
    {
      id: caveatId('cdnots-parcorr-form'),
      category: 'functional-form',
      requirement: 'Partial correlation is an adequate conditional-independence test for the selected variables.',
      consequenceIfUnmet: 'Nonlinear or non-Gaussian conditional relations can produce incorrect edge decisions.',
      sources: [CDNOTS_2025],
    },
    {
      id: caveatId('cdnots-missingness'),
      category: 'missingness',
      requirement: 'Pairwise-complete testing or VAR-EM imputation is appropriate for the process that produced the missing observations.',
      consequenceIfUnmet: 'The retained test samples or imputed trajectories can distort conditional-independence decisions.',
      sources: [CDNOTS_2025],
    },
    {
      id: caveatId('cdnots-graph-reading'),
      category: 'interpretation',
      requirement: 'Unoriented and conflicting endpoints remain unresolved; directed marks are discovery evidence rather than intervention-effect estimates.',
      consequenceIfUnmet: 'Graph uncertainty is presented as a uniquely identified causal effect.',
      sources: [CDNOTS_2025],
    },
  ],
}

const CDNOTS_PLUS_PAR_CORR: MethodDefinition = {
  ...CDNOTS_PAR_CORR,
  id: CDNOTS_PLUS_PAR_CORR_METHOD_ID,
  name: 'CD-NOTS+ with ParCorr',
  summary: 'Uses a PCMCI+-style two-stage skeleton before applying the CD-NOTS orientation rules for nonstationary time series.',
  caveats: mapNonEmpty(CDNOTS_PAR_CORR.caveats, (caveat) => ({
    ...caveat,
    id: caveatId(caveat.id.replace('cdnots-', 'cdnots-plus-')),
  })),
}

const GRACE: MethodDefinition = {
  id: GRACE_METHOD_ID,
  name: 'GRACE',
  family: 'discovery',
  summary: 'Refines a high-recall CD-NOTS skeleton with hard-concrete gates that retain candidate lagged inputs that improve nonlinear prediction.',
  caveats: [
    {
      id: caveatId('grace-ordered-time-series'),
      category: 'sampling-structure',
      requirement: 'Rows follow one regular temporal grid, and the maximum lag covers the candidate predictive history.',
      consequenceIfUnmet: 'The neural windows join incorrect time points or omit relevant history.',
      sources: [GRACE_2026],
    },
    {
      id: caveatId('grace-skeleton'),
      category: 'identification',
      requirement: 'The CD-NOTS skeleton has sufficient recall because the neural stage can remove candidates but cannot recover an edge excluded by the skeleton.',
      consequenceIfUnmet: 'A true relation missing from the first stage is structurally unavailable to the gated model.',
      sources: [GRACE_2026],
    },
    {
      id: caveatId('grace-dense-input'),
      category: 'missingness',
      requirement: 'The dense neural tensor is observed or produced by a defensible recorded imputation model.',
      consequenceIfUnmet: 'Imputed patterns can be selected as predictive relations.',
      sources: [GRACE_2026],
    },
    {
      id: caveatId('grace-model-form'),
      category: 'functional-form',
      requirement: 'The additive gated encoder and per-target nonlinear decoder adequately represent the predictive mechanisms.',
      consequenceIfUnmet: 'Gate values can suppress relations that require an unsupported interaction or representation.',
      sources: [GRACE_2026],
    },
    {
      id: caveatId('grace-optimization'),
      category: 'computation',
      requirement: 'The seed, training budget, regularization and gate threshold are assessed because neural optimization and edge selection are tuning-sensitive.',
      consequenceIfUnmet: 'Different initializations or thresholds can select different graphs.',
      sources: [GRACE_2026],
    },
    {
      id: caveatId('grace-result-reading'),
      category: 'interpretation',
      requirement: 'A retained gate is lagged predictive evidence within the fitted system, not an intervention-effect estimate.',
      consequenceIfUnmet: 'Predictive selection is reported as the effect of manipulating a variable.',
      sources: [GRACE_2026],
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
      requirement: 'The structural errors are mutually independent and non-Gaussian. Shapiro–Wilk assesses normality, not independence.',
      consequenceIfUnmet: 'Gaussian errors do not provide the non-Gaussian information used to identify the contemporaneous causal order.',
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
      requirement: 'At least 2 complete, varying columns and lags 1 to 6.',
      consequenceIfUnmet: 'The analysis cannot run with a constant column or missing time periods.',
      sources: [hirmos('crates/analysis-wasm/src/lib.rs#var_lingam_evidence')],
    },
  ],
}

const PC_STABLE: MethodDefinition = {
  id: PC_STABLE_METHOD_ID,
  name: 'PC-stable',
  family: 'discovery',
  summary: 'Uses conditional-independence tests to estimate a completed partially directed acyclic graph from independent observations.',
  caveats: [
    {
      id: caveatId('pc-stable-independent-observations'),
      category: 'sampling-structure',
      requirement: 'Rows are independent observations from one unchanged data-generating process.',
      consequenceIfUnmet: 'Serial dependence, repeated units, or mechanism changes can produce conditional dependences that the graph does not represent correctly.',
      sources: [SPIRTES_2000, COLOMBO_MAATHUIS_2014],
    },
    {
      id: caveatId('pc-stable-markov-faithful'),
      category: 'identification',
      requirement: 'The observational distribution is Markov and faithful to the causal graph.',
      consequenceIfUnmet: 'Conditional independences may not correspond to missing graph connections, so the recovered equivalence class can be wrong.',
      sources: [SPIRTES_MEEK_RICHARDSON_1995, SPIRTES_2000],
    },
    {
      id: caveatId('pc-stable-causal-sufficiency'),
      category: 'identification',
      requirement: 'There are no unmeasured common causes or selection variables among the selected variables.',
      consequenceIfUnmet: 'A CPDAG cannot represent the latent-confounding or selection structure responsible for the observed independences.',
      sources: [SPIRTES_2000],
    },
    {
      id: caveatId('pc-stable-ci-model'),
      category: 'functional-form',
      requirement: 'Fisher Z requires a linear Gaussian conditional-independence model; KCI replaces that model with kernel conditional-independence testing.',
      consequenceIfUnmet: 'Misspecified or low-power conditional-independence tests can add, remove, or orient the wrong connections.',
      sources: [SPIRTES_2000, ZHANG_KCI_2011],
    },
    {
      id: caveatId('pc-stable-background-knowledge'),
      category: 'interpretation',
      requirement: 'Required directions, forbidden directions, and tiers are correct and mutually consistent.',
      consequenceIfUnmet: 'The result is constrained to satisfy an incorrect causal claim.',
      sources: [SPIRTES_2000],
    },
    {
      id: caveatId('pc-stable-cpdag-interpretation'),
      category: 'interpretation',
      requirement: 'Undirected connections represent directions not determined within the Markov-equivalence class.',
      consequenceIfUnmet: 'An unresolved connection is treated as a directed causal claim.',
      sources: [SPIRTES_2000, COLOMBO_MAATHUIS_2014],
    },
    {
      id: caveatId('pc-stable-complete-data'),
      category: 'missingness',
      requirement: 'The selected numeric columns form a complete matrix.',
      consequenceIfUnmet: 'The analysis cannot run until these input requirements are met.',
      sources: [COLOMBO_MAATHUIS_2014],
    },
  ],
}

const FCI: MethodDefinition = {
  id: FCI_METHOD_ID,
  name: 'FCI',
  family: 'discovery',
  summary: 'Uses conditional-independence tests to estimate a partial ancestral graph that permits latent confounding and selection bias.',
  caveats: [
    {
      id: caveatId('fci-independent-observations'),
      category: 'sampling-structure',
      requirement: 'Rows are independent observations from one unchanged data-generating process.',
      consequenceIfUnmet: 'Temporal dependence, repeated units, or mechanism changes can be represented as graph structure.',
      sources: [SPIRTES_2000],
    },
    {
      id: caveatId('fci-markov-faithful-mag'),
      category: 'identification',
      requirement: 'The observed distribution is Markov and faithful to an underlying causal DAG whose latent projection is a maximal ancestral graph.',
      consequenceIfUnmet: 'The PAG need not contain the causal structure that generated the observations.',
      sources: [SPIRTES_MEEK_RICHARDSON_1995, SPIRTES_2000, ZHANG_2008],
    },
    {
      id: caveatId('fci-ci-model'),
      category: 'functional-form',
      requirement: 'Fisher Z requires a linear Gaussian conditional-independence model; KCI uses kernel conditional-independence testing.',
      consequenceIfUnmet: 'Conditional-independence errors propagate into the skeleton and endpoint orientations.',
      sources: [SPIRTES_MEEK_RICHARDSON_1995, SPIRTES_2000, ZHANG_KCI_2011],
    },
    {
      id: caveatId('fci-selection-semantics'),
      category: 'identification',
      requirement: 'Selection effects are represented by ancestral-graph endpoint semantics rather than interpreted as ordinary directed causes.',
      consequenceIfUnmet: 'Tail, arrowhead, and circle endpoints are assigned causal meanings they do not establish.',
      sources: [SPIRTES_MEEK_RICHARDSON_1995, SPIRTES_2000, ZHANG_2008],
    },
    {
      id: caveatId('fci-background-knowledge'),
      category: 'interpretation',
      requirement: 'Required directions, forbidden directions, patterns, and tiers are correct and mutually consistent.',
      consequenceIfUnmet: 'Orientation rules are forced to incorporate incorrect expert knowledge.',
      sources: [SPIRTES_MEEK_RICHARDSON_1995, SPIRTES_2000],
    },
    {
      id: caveatId('fci-pag-interpretation'),
      category: 'interpretation',
      requirement: 'The result is read as a PAG: circles are unresolved endpoints and bidirected connections can indicate latent confounding.',
      consequenceIfUnmet: 'The PAG is converted into a DAG and claims directions or causal sufficiency that FCI did not establish.',
      sources: [SPIRTES_2000, ZHANG_2008],
    },
    {
      id: caveatId('fci-complete-data'),
      category: 'missingness',
      requirement: 'The selected numeric columns form a complete matrix.',
      consequenceIfUnmet: 'The analysis cannot run until these input requirements are met.',
      sources: [SPIRTES_2000],
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
      requirement: 'At least 2 complete, varying numeric columns are selected.',
      consequenceIfUnmet: 'The analysis cannot run until these input requirements are met.',
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
      requirement: 'At least 2 complete columns and lags 1 to 6.',
      consequenceIfUnmet: 'The analysis cannot run with missing time periods.',
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
      requirement: 'Duplicate target self-lags already in the conditioning set are excluded.',
      consequenceIfUnmet: 'Duplicate self-lags are counted in the conditioning set.',
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

const CMLP: MethodDefinition = {
  id: CMLP_METHOD_ID,
  name: 'cMLP',
  family: 'discovery',
  summary: 'Fits one nonlinear autoregressive multilayer perceptron per series and selects Granger relations with structured sparsity.',
  caveats: [
    {
      id: caveatId('cmlp-ordered-stationary-series'),
      category: 'stationarity-and-dynamics',
      requirement: 'The selected variables form a stationary multivariate time series over the fitted window.',
      consequenceIfUnmet: 'Changing levels or dynamics can be selected as predictive relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('cmlp-lag-window'),
      category: 'sampling-structure',
      requirement: 'Rows are regularly ordered, and the maximum lag covers the predictive history under study.',
      consequenceIfUnmet: 'The network joins the wrong time points or cannot represent relations beyond the selected lag.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('cmlp-model-capacity'),
      category: 'functional-form',
      requirement: 'The component-wise MLP has adequate capacity for the nonlinear autoregressive relationships.',
      consequenceIfUnmet: 'Model misspecification can omit or distort predictive relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('cmlp-regularization'),
      category: 'computation',
      requirement: 'The sparsity penalty, architecture and optimization settings are assessed because the objective is non-convex.',
      consequenceIfUnmet: 'Different tuning choices or initializations can select different relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('cmlp-sample-size'),
      category: 'finite-sample',
      requirement: 'The number of observations is adequate for the selected variables, lag window and network size.',
      consequenceIfUnmet: 'The fitted sparsity pattern can be unstable or overfit.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('cmlp-granger-reading'),
      category: 'interpretation',
      requirement: 'An active input means its past helps predict the target within the fitted system; it is not an intervention-effect estimate.',
      consequenceIfUnmet: 'Predictive Granger relations are read as effects of manipulating a variable.',
      sources: [TANK_2021],
    },
  ],
}

const CLSTM: MethodDefinition = {
  id: CLSTM_METHOD_ID,
  name: 'cLSTM',
  family: 'discovery',
  summary: 'Fits one recurrent forecasting model per series and selects window-level Granger relations with a group-lasso penalty.',
  caveats: [
    {
      id: caveatId('clstm-ordered-stationary-series'),
      category: 'stationarity-and-dynamics',
      requirement: 'The selected variables form a stationary multivariate time series over the fitted window.',
      consequenceIfUnmet: 'Changing levels or dynamics can be selected as predictive relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('clstm-context-window'),
      category: 'sampling-structure',
      requirement: 'Rows are regularly ordered, and the context length is sufficient to train the recurrent history model.',
      consequenceIfUnmet: 'The network joins the wrong time points or trains on an inadequate history window.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('clstm-model-capacity'),
      category: 'functional-form',
      requirement: 'The component-wise LSTM has adequate capacity for the nonlinear recurrent relationships.',
      consequenceIfUnmet: 'Model misspecification can omit or distort predictive relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('clstm-regularization'),
      category: 'computation',
      requirement: 'The group-lasso strength, hidden width and optimization settings are assessed because the objective is non-convex.',
      consequenceIfUnmet: 'Different tuning choices or initializations can select different relations.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('clstm-sample-size'),
      category: 'finite-sample',
      requirement: 'The number of observations is adequate for the selected variables, context and recurrent network size.',
      consequenceIfUnmet: 'The fitted sparsity pattern can be unstable or overfit.',
      sources: [TANK_2021],
    },
    {
      id: caveatId('clstm-window-reading'),
      category: 'interpretation',
      requirement: 'cLSTM identifies whether a source history helps predict a target, but does not assign that relation to an individual lag.',
      consequenceIfUnmet: 'A window-level relation is displayed as if the model had selected a specific lag or an intervention effect.',
      sources: [TANK_2021],
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
      consequenceIfUnmet: 'Choosing the set whose estimate you prefer adds a decision about the analysis that the record does not show.',
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

const PROPENSITY_WEIGHTING: MethodDefinition = {
  id: PROPENSITY_WEIGHTING_METHOD_ID,
  name: 'Inverse propensity weighting',
  family: 'estimation',
  summary: 'Fit the propensity score on the adjustment set, then reweight the sample by the inverse probability of treatment. Weights may be stabilized by the marginal treatment prevalence.',
  summaryTex: {
    tex: String.raw`\mathrm{ATE} = \frac{1}{N}\sum_i\left(\frac{T_i Y_i}{e(X_i)} - \frac{(1-T_i) Y_i}{1 - e(X_i)}\right),\quad e(X) = P(T{=}1 \mid X)`,
    plain: 'ATE = (1/N) Σ [ T·Y / e(X) − (1−T)·Y / (1 − e(X)) ], where e(X) = P(T=1 | X)',
  },
  caveats: [
    {
      id: caveatId('ipw-unconfoundedness'),
      category: 'identification',
      requirement: 'The covariates are an identified adjustment set, so that given the same propensity score treatment is as good as random.',
      consequenceIfUnmet: 'The estimate is an association.',
      sources: [FACURE_CH5('conditional independence and the propensity score'), ROSENBAUM_RUBIN_1983],
    },
    {
      id: caveatId('ipw-positivity'),
      category: 'identification',
      requirement: 'Every unit has some chance of either arm given the covariates, so no fitted score sits at zero or one.',
      consequenceIfUnmet: 'A weight divides by a near-zero probability and the estimate is carried by a handful of rows.',
      sources: [FACURE_CH5('positivity and the bias-variance trade-off'), HERNAN_ROBINS],
    },
    {
      id: caveatId('ipw-extreme-weights'),
      category: 'finite-sample',
      requirement: 'Inspect the weight distribution and effective sample size within each arm. Extreme propensity scores can produce large relative weights even when positivity holds.',
      consequenceIfUnmet: 'A stable-looking point estimate rests on a few heavily weighted rows.',
      sources: [FACURE_CH5('small or large propensity scores and the variance of IPW')],
    },
    {
      id: caveatId('ipw-treatment-model'),
      category: 'functional-form',
      requirement: 'The selected treatment model estimates treatment probabilities from the adjustment variables using logistic regression or gradient-boosted trees. Declare categorical covariates before fitting.',
      consequenceIfUnmet: 'A misspecified score misweights the sample.',
      sources: [FACURE_CH5('estimating the propensity score; propensity score and ML'), statsmodels('statsmodels/discrete/discrete_model.py#Logit'), hirmos('crates/analysis-wasm/src/estimation/propensity_score.rs#boosted_scores')],
    },
  ],
}

const PROPENSITY_MATCHING: MethodDefinition = {
  id: PROPENSITY_MATCHING_METHOD_ID,
  name: 'Propensity-score matching',
  family: 'estimation',
  summary: 'Pair every row with its nearest neighbour on the fitted propensity score from the opposite arm, then average the paired differences over the whole sample.',
  summaryTex: {
    tex: String.raw`\mathrm{ATE} = \frac{1}{N}\sum_i (2T_i - 1)\bigl(Y_i - Y_{j_m(i)}\bigr)`,
    plain: 'ATE = (1/N) Σ (2T − 1)(Y − Y_jm), where jm is the nearest neighbour on the score from the other arm',
  },
  caveats: [
    {
      id: caveatId('matching-unconfoundedness'),
      category: 'identification',
      requirement: 'The covariates are an identified adjustment set, so that given the same propensity score treatment is as good as random.',
      consequenceIfUnmet: 'The estimate is an association.',
      sources: [FACURE_CH5('conditional independence and the propensity score'), ROSENBAUM_RUBIN_1983],
    },
    {
      id: caveatId('matching-positivity'),
      category: 'identification',
      requirement: 'Both treatment arms must have support at the covariate values being compared. Inspect overlap in their fitted propensity scores.',
      consequenceIfUnmet: 'Nearest neighbours may have very different propensity scores, so the matched outcomes may not provide comparable counterfactuals.',
      sources: [FACURE_CH5('positivity and the bias-variance trade-off'), HERNAN_ROBINS],
    },
    {
      id: caveatId('matching-single-neighbour'),
      category: 'finite-sample',
      requirement: 'Each row is paired with one nearest neighbour, which is a K-nearest-neighbours fit with K of one.',
      consequenceIfUnmet: 'A single neighbour carries the whole comparison for that row.',
      sources: [FACURE_CH5('matching as a K-nearest-neighbours estimator with K = 1')],
    },
    {
      id: caveatId('matching-average-not-treated'),
      category: 'interpretation',
      requirement: 'Pairs are averaged over every row, so the estimand is the average effect rather than the effect on the treated.',
      consequenceIfUnmet: 'An average effect is read as an effect on the treated.',
      sources: [FACURE_CH5('the matching estimator')],
    },
  ],
}

const DOUBLY_ROBUST: MethodDefinition = {
  id: DOUBLY_ROBUST_METHOD_ID,
  name: 'Doubly robust estimation',
  family: 'estimation',
  summary: 'Combine propensity weighting with outcome regression fitted separately in each arm. Under the identification and regularity assumptions, the estimator is consistent if either the propensity model or the outcome regressions are correctly specified.',
  summaryTex: {
    tex: String.raw`\mathrm{ATE} = \frac{1}{N}\sum_i\left(\frac{T_i\bigl(Y_i - \hat\mu_1(X_i)\bigr)}{e(X_i)} + \hat\mu_1(X_i)\right) - \frac{1}{N}\sum_i\left(\frac{(1-T_i)\bigl(Y_i - \hat\mu_0(X_i)\bigr)}{1 - e(X_i)} + \hat\mu_0(X_i)\right)`,
    plain: 'ATE = (1/N) Σ [ T(Y − μ1(X)) / e(X) + μ1(X) ] − (1/N) Σ [ (1−T)(Y − μ0(X)) / (1 − e(X)) + μ0(X) ]',
  },
  caveats: [
    {
      id: caveatId('aipw-unconfoundedness'),
      category: 'identification',
      requirement: 'The covariates are an identified adjustment set, so that given the same propensity score treatment is as good as random.',
      consequenceIfUnmet: 'The estimate is an association.',
      sources: [FACURE_CH5('conditional independence and the propensity score'), ROSENBAUM_RUBIN_1983],
    },
    {
      id: caveatId('aipw-positivity'),
      category: 'identification',
      requirement: 'Every unit has some chance of either arm given the covariates, so no fitted score sits at zero or one.',
      consequenceIfUnmet: 'A weight divides by a near-zero probability and the estimate is carried by a handful of rows.',
      sources: [FACURE_CH5('positivity and the bias-variance trade-off'), HERNAN_ROBINS],
    },
    {
      id: caveatId('aipw-one-model-right'),
      category: 'functional-form',
      requirement: 'Either the propensity model or both arm-specific outcome regressions must be correctly specified. Double robustness does not remove the need for unconfoundedness and positivity.',
      consequenceIfUnmet: 'If both model specifications are incorrect, the estimator may be biased.',
      sources: [FACURE_CH5('why it is called doubly robust'), ROBINS_1994, KENNEDY_DR],
    },
    {
      id: caveatId('aipw-arm-regressions'),
      category: 'functional-form',
      requirement: 'The outcome model is a linear regression fitted within each arm on the same design.',
      consequenceIfUnmet: 'The linear regressions may miss nonlinear outcome relationships; consistency then depends on a correctly specified propensity model.',
      sources: [FACURE_CH5('the doubly robust estimator in code')],
    },
  ],
}

const CONTINUOUS_GPS: MethodDefinition = {
  id: CONTINUOUS_GPS_METHOD_ID,
  name: 'Generalised propensity score',
  family: 'estimation',
  summary: 'For a continuous treatment, the generalised propensity score is a conditional treatment density rather than a binary treatment probability. This estimator fits a Gaussian treatment model and uses density-based weights in a linear outcome regression.',
  summaryTex: {
    tex: String.raw`w_i = \frac{1}{\hat f(T_i \mid X_i)},\qquad \hat\tau = \arg\min_{\alpha,\tau}\sum_i w_i\bigl(Y_i - \alpha - \tau T_i\bigr)^2`,
    plain: 'w = 1 / f(T | X); τ minimises Σ w (Y − α − τT)²',
  },
  caveats: [
    {
      id: caveatId('gps-unconfoundedness'),
      category: 'identification',
      requirement: 'Potential outcomes must be independent of treatment assignment conditional on the adjustment variables, with treatment support at the values being compared.',
      consequenceIfUnmet: 'The estimate is an association.',
      sources: [FACURE_CH5('generalized propensity score'), HIRANO_IMBENS_2004],
    },
    {
      id: caveatId('gps-normal-treatment'),
      category: 'functional-form',
      requirement: 'The treatment is taken as normally distributed around its fitted value with constant variance.',
      consequenceIfUnmet: 'A skewed or heteroskedastic treatment gives the wrong density and the wrong weights.',
      sources: [FACURE_CH5('assuming a conditional Gaussian for the treatment'), HIRANO_IMBENS_2004],
    },
    {
      id: caveatId('gps-stabilize'),
      category: 'finite-sample',
      requirement: 'Stabilized weights use the marginal treatment density divided by the conditional treatment density. Inspect weight concentration; stabilization does not guarantee adequate overlap.',
      consequenceIfUnmet: 'Large relative weights can make the estimate sensitive to a small number of observations.',
      sources: [FACURE_CH5('stabilizing the weights by the marginal density')],
    },
    {
      id: caveatId('gps-linear-response'),
      category: 'interpretation',
      requirement: 'The weighted outcome model assumes a linear dose-response relationship. Its slope is the estimated change in outcome per one-unit increase in treatment.',
      consequenceIfUnmet: 'A curved dose response is read as one slope.',
      sources: [FACURE_CH5('the weighted final model')],
    },
  ],
}

const BACKDOOR_LINEAR_REGRESSION: MethodDefinition = {
  id: BACKDOOR_LINEAR_REGRESSION_METHOD_ID,
  name: 'Adjusted linear regression',
  family: 'estimation',
  summary: 'Regress the outcome on the treatment and adjustment variables. Optional fixed effects account for additive unit differences, common period differences, or both. Choose uncertainty appropriate to the observations. A causal interpretation of the treatment coefficient depends on the identifying assumptions and the regression specification.',
  caveats: [
    {
      id: caveatId('linear-identified-adjustment'),
      category: 'identification',
      requirement: 'The covariates are an identified adjustment set; regression cannot close an open back-door path.',
      consequenceIfUnmet: 'The coefficient is an association.',
      sources: [NESS_CH11('§11.4.1 regression with back-door confounders'), RUIZ_DE_VILLA_CH7('§7.4.5 total effect theorem')],
    },
    {
      id: caveatId('linear-fixed-effects'),
      category: 'identification',
      requirement: 'Fixed effects account for additive unit differences, common period differences, or both. Treatment must retain variation after absorbing those effects. A causal interpretation also requires appropriate adjustment and exogeneity assumptions.',
      consequenceIfUnmet: 'Remaining confounding can bias the coefficient. If the fixed effects absorb the treatment, its coefficient cannot be estimated.',
      sources: [WOOLDRIDGE_2010, linearmodels('linearmodels/panel/model.py#PanelOLS'), linearmodels('linearmodels/panel/covariance.py#ClusteredCovariance')],
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
      requirement: 'The interval selection aligns with the error structure: classical for independent errors with constant variance, robust (HC1) for heteroskedastic errors, clustered for repeated measures within units, and for time-series data, the HAC (Newey–West) interval or an ARMA error process fitted using the coefficients.',
      consequenceIfUnmet: 'Heteroskedastic, clustered, or autocorrelated errors can render the classical interval unreliable.',
      sources: [MACKINNON_WHITE_1985, CAMERON_MILLER_2015, NEWEY_WEST_1987, statsmodels('statsmodels/stats/sandwich_covariance.py#cov_hc1'), statsmodels('statsmodels/stats/sandwich_covariance.py#cov_cluster'), statsmodels('statsmodels/stats/sandwich_covariance.py#cov_hac_simple'), HYNDMAN_FPP_DHR, statsmodels('statsmodels/tsa/statespace/sarimax.py#SARIMAX')],
    },
    {
      id: caveatId('linear-hac-bandwidth'),
      category: 'finite-sample',
      requirement: 'The HAC bandwidth is floor(4 (n/100)^(2/9)) lags, recorded with the run.',
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

const INSTRUMENTAL_VARIABLE: MethodDefinition = {
  id: INSTRUMENTAL_VARIABLE_METHOD_ID,
  name: 'Instrumental variable',
  family: 'estimation',
  summary: 'Targets the instrumental variable estimand under a linearity assumption: the Wald estimator for one binary instrument, the covariance ratio for one continuous instrument, and two-stage least squares otherwise.',
  caveats: [
    {
      id: caveatId('iv-identified-instrument'),
      category: 'identification',
      requirement: 'Each instrument meets the two level 2 definitional requirements for a valid instrument. As-if-random: any back-door paths between the instrument and the outcome can be blocked. Exclusion: the instrument is a cause of the outcome only indirectly through the treatment.',
      consequenceIfUnmet: 'With either path present there is no instrumental variable estimand.',
      sources: [NESS_CH11('§11.3.2 the instrumental variable estimand'), dowhy('dowhy/graph.py#get_instruments'), hirmos('crates/causal-core/src/iv.rs#identify_instrument_set')],
    },
    {
      id: caveatId('iv-linearity'),
      category: 'functional-form',
      requirement: 'The level 2 graphical assumptions are not sufficient for instrumental variable identification; additional parametric assumptions are needed. The estimator makes a linearity assumption and derives the ATE as a function of the coefficients of linear models of the outcome and the treatment given the instrument, without covariates.',
      consequenceIfUnmet: 'The ratio of the two coefficients is the ATE only under that linear assumption.',
      sources: [NESS_CH11('§11.3.2 parametric assumptions for instrumental variable estimation'), MOLAK_CH6('instrumental variables: fit Y ~ Z and X ~ Z and compute the ratio of their coefficients'), dowhy('dowhy/causal_estimators/instrumental_variable_estimator.py#estimate_effect')],
    },
    {
      id: caveatId('iv-effect-homogeneity'),
      category: 'functional-form',
      requirement: 'Each unit’s treatment is affected in the same way by common causes of the treatment and outcome, and each unit’s outcome is affected in the same way by those common causes.',
      consequenceIfUnmet: 'The Wald estimator reports one effect for every unit; variation of the effect across units is not represented.',
      sources: [dowhy('dowhy/causal_estimators/instrumental_variable_estimator.py#construct_symbolic_estimator'), NESS_CH11('§11.3.2 the instrumental variable estimand')],
    },
    {
      id: caveatId('iv-instrument-strength'),
      category: 'noise-and-dependence',
      requirement: 'The instrument is strong, meaning it has a strong causal effect on the treatment variable.',
      consequenceIfUnmet: 'Weak instruments can lead to high variance estimates of the ATE.',
      sources: [NESS_CH11('§11.4.5 good instrumental variables should be “strong”'), CIR_CH10('test for weak instruments: first-stage regression of the endogenous variable on the instruments, F statistic')],
    },
    {
      id: caveatId('iv-bootstrap-rows'),
      category: 'noise-and-dependence',
      requirement: 'The row bootstrap is appropriate for the sampling design; dependent rows require a dependence-aware resampling scheme.',
      consequenceIfUnmet: 'The confidence interval does not represent the estimator’s sampling variation.',
      sources: [NESS_CH11('§11.4.5 instrumental variable methods'), dowhy('dowhy/causal_estimator.py#_estimate_confidence_intervals_with_bootstrap'), hirmos('crates/causal-core/src/dowhy_bootstrap.rs')],
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
      requirement: 'The Poisson model assumes the outcome variance equals its mean conditional on the predictors. Assess dispersion after accounting for those predictors.',
      consequenceIfUnmet: 'Overdispersion can make model-based standard errors too small.',
      sources: [CAMERON_TRIVEDI, statsmodels('statsmodels/genmod/families/family.py#Poisson')],
    }
    : {
      id: caveatId('negbin-convergence'),
      category: 'computation',
      requirement: 'Check the reported convergence status. Reaching the dispersion boundary is not treated as convergence.',
      consequenceIfUnmet: 'Without convergence, the reported estimates and standard errors may be unreliable.',
      sources: [statsmodels('statsmodels/discrete/discrete_model.py#NegativeBinomialP.fit'), hirmos('crates/causal-core/src/bfgs.rs')],
    },
  {
    id: caveatId(`${prefix}-independence`),
    category: 'noise-and-dependence',
    requirement: prefix === 'poisson'
      ? 'Counts are independent across rows; no serial-correlation correction exists in this fit.'
      : 'Rows are independent; the dispersion term absorbs extra variance, not serial dependence.',
    consequenceIfUnmet: 'Serial dependence can invalidate intervals calculated under independence.',
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
      : 'exp(β) is the multiplicative change in the expected count per unit of treatment, as for Poisson; alpha describes spread, not effect.',
    consequenceIfUnmet: 'A ratio is read as an additive difference.',
    sources: [CAMERON_TRIVEDI, hirmos('crates/analysis-wasm/src/lib.rs#count_glm')],
  },
]

const POISSON_GLM: MethodDefinition = {
  id: POISSON_GLM_METHOD_ID,
  name: 'Poisson GLM',
  family: 'estimation',
  summary: 'Poisson family with log link by iteratively reweighted least squares; exp(β) is an expected-count ratio.',
  caveats: countCaveats('poisson', 'statsmodels/genmod/generalized_linear_model.py#GLM'),
}

const NEGATIVE_BINOMIAL: MethodDefinition = {
  id: NEGATIVE_BINOMIAL_METHOD_ID,
  name: 'Negative binomial (NB2)',
  family: 'estimation',
  summary: 'NB2 by BFGS: a Poisson mean with gamma-distributed rate, so alpha absorbs variance above the mean.',
  caveats: countCaveats('negbin', 'statsmodels/discrete/discrete_model.py#NegativeBinomialP'),
}

const NEGATIVE_BINOMIAL_INGARCH: MethodDefinition = {
  id: NEGATIVE_BINOMIAL_INGARCH_METHOD_ID,
  name: 'Negative-binomial INGARCH',
  family: 'estimation',
  summary: 'A count time-series model whose conditional mean depends on past counts, past conditional means, and recorded regressors. Identity and log links represent additive and multiplicative effects; the run compares forecast count paths under two future treatment schedules.',
  caveats: [
    {
      id: caveatId('ingarch-regular-count-series'),
      category: 'sampling-structure',
      requirement: 'The outcome is a regularly sampled sequence of non-negative integer counts.',
      consequenceIfUnmet: 'The count recursion and its lag spacing do not describe the observations.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', 'Journal of Statistical Software 82(5)')],
    },
    {
      id: caveatId('ingarch-conditional-mean'),
      category: 'functional-form',
      requirement: 'The selected identity or log conditional-mean form is adequately represented by the chosen past-count lags, past-mean lags, and regressors. Identity-link regressors must be non-negative.',
      consequenceIfUnmet: 'The fitted treatment trajectory can reflect omitted dynamics or a misspecified response surface.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', '§2 INGARCH and log-linear count time-series models')],
    },
    {
      id: caveatId('ingarch-identified-regressors'),
      category: 'identification',
      requirement: 'The treatment and supplied covariates are an identified contemporaneous adjustment set, and the proposed future regressor paths are well-defined interventions.',
      consequenceIfUnmet: 'The two forecast paths are model-based scenarios rather than an identified causal contrast.',
      sources: [RUIZ_DE_VILLA_CH7('§7.2 back-door criterion'), PEARL_2009('§3.2 interventions and modified models')],
    },
    {
      id: caveatId('ingarch-stability'),
      category: 'stationarity-and-dynamics',
      requirement: 'The fitted recursion is stable over the observed and forecast periods.',
      consequenceIfUnmet: 'Small changes in recent counts can produce implausible or explosive forecasts.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', '§2 stationarity and ergodicity')],
    },
    {
      id: caveatId('ingarch-no-interval'),
      category: 'finite-sample',
      requirement: 'Sampling uncertainty requires a dependence-aware bootstrap or another validated interval procedure.',
      consequenceIfUnmet: 'The fitted baseline and intervention trajectories are point predictions, not uncertainty bounds.',
      sources: [paper('tscount: An R Package for Analysis of Count Time Series Following Generalized Linear Models (Liboschik, Fokianos and Fried, 2017)', '§3 prediction')],
    },
  ],
}

const CAUSAL_EFFECTS_TOTAL: MethodDefinition = {
  id: CAUSAL_EFFECTS_TOTAL_METHOD_ID,
  name: 'CausalEffects total effect',
  family: 'estimation',
  summary: '',
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
      requirement: 'In the projected graph, the selected generated set, or a set supplied by the user, blocks every path between treatment and outcome that is not causal.',
      consequenceIfUnmet: 'The run refuses the adjustment set and does not fit an effect model.',
      sources: [RUNGE_2021, VAN_DER_ZANDER_2014],
    },
    {
      id: caveatId('causal-effects-functional-form'),
      category: 'functional-form',
      requirement: 'The first-stage estimator matches the dependence: linear for additive, k-nearest neighbours for local nonlinearity.',
      consequenceIfUnmet: 'A misspecified first stage biases the predicted difference.',
      sources: [tigramite('tigramite/causal_effects.py#fit_total_effect')],
    },
    {
      id: caveatId('causal-effects-bootstrap'),
      category: 'finite-sample',
      requirement: 'The block-bootstrap length represents the serial dependence in the fitted observations.',
      consequenceIfUnmet: 'The percentile interval can understate or overstate sampling variation.',
      sources: [KUNSCH_1989],
    },
  ],
}

const CAUSAL_IMPACT: MethodDefinition = {
  id: CAUSAL_IMPACT_METHOD_ID,
  name: 'Causal impact',
  family: 'estimation',
  summary: 'Fit a state-space model with optional static control regression before the intervention. Compare observed outcomes with its no-intervention prediction. Bayesian models can include trend and seasonal components.',
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
      consequenceIfUnmet: 'With weak control series, the counterfactual is driven primarily by the selected trend and seasonal components.',
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
  summary: 'Adds a randomly generated covariate and refits to assess the sensitivity of the estimate to an irrelevant variable.',
  caveats: [
    refuterCaveat('random-cause-reads-against-original', 'Compare the refitted estimates with the original, allowing for variation across simulations.', 'Small changes can occur by chance; substantial changes warrant further investigation.', [NESS_CH11('§11.5.2 random common cause refuter'), dowhy('dowhy/causal_refuters/random_common_cause.py')]),
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
  ],
}

const LJUNG_BOX: MethodDefinition = {
  id: LJUNG_BOX_METHOD_ID,
  name: 'Ljung–Box on residuals',
  family: 'diagnostic',
  summary: 'Cumulative autocorrelation statistic of the residuals at each lag with a chi-square p-value.',
  caveats: [
    { id: caveatId('ljung-box-null'), category: 'interpretation', requirement: 'A small p-value indicates residual autocorrelation through the tested lag. It does not identify which model terms are missing.', consequenceIfUnmet: 'Autocorrelation is read as a specific dynamic model.', sources: [LJUNG_BOX_1978, statsmodels('statsmodels/stats/diagnostic.py#acorr_ljungbox')] },
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
    id: caveatId(`dml-${model}-group-effects`),
    category: 'functional-form',
    requirement: 'For a conditional target the effect of the treatment is linear within each group of the effect modifier, and may differ between groups; the modifier is distinct from the treatment and joins the adjustment set as a nuisance input.',
    consequenceIfUnmet: 'A group effect then averages over effects that vary inside the group, and the contrast between groups is not the heterogeneity it appears to be.',
    sources: [RUIZ_DE_VILLA_CH8('§8.1.4 heterogeneous treatment effects, the conditional average treatment effect'), NESS_CH11('§11.4 conditional average treatment effect estimation'), BACH_DOUBLEML],
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

const CAUSAL_FOREST: MethodDefinition = {
  id: CAUSAL_FOREST_METHOD_ID, name: 'Causal forest', family: 'estimation',
  summary: 'Estimate conditional treatment effects with an honest forest. Report binary treatment contrasts or continuous treatment slopes according to the recorded study target.',
  caveats: [
    { id: caveatId('forest-adjustment'), category: 'identification', requirement: 'Identification requires a valid adjustment set and covariates measured before treatment.', consequenceIfUnmet: 'Flexible estimation does not remove unmeasured confounding or justify conditioning on variables affected by treatment.', sources: [{ kind: 'paper', title: 'Generalized random forests (Athey, Tibshirani and Wager, 2019)', locator: 'Section 6; https://doi.org/10.1214/18-AOS1709' }] },
    { id: caveatId('forest-sampling'), category: 'sampling-structure', requirement: 'This route uses independent observations.', consequenceIfUnmet: 'This product route does not yet pass cluster identifiers or model serial dependence. Repeated observations require a supported dependence specification.', sources: [{ kind: 'hirmos-constraint', locator: 'Causal forest browser route: independent observations; clustered core inference is not exposed here.' }] },
    { id: caveatId('forest-target'), category: 'functional-form', requirement: 'Binary targets require both treatment groups. Continuous targets require conditional treatment variation and assumptions supporting a causal slope.', consequenceIfUnmet: 'A continuous partial effect is not an arbitrary treatment contrast or an unrestricted dose-response curve.', sources: [{ kind: 'paper', title: 'Generalized random forests (Athey, Tibshirani and Wager, 2019)', locator: 'Section 6, exogenous random-coefficient model and local moment equation' }] },
    { id: caveatId('forest-overlap'), category: 'identification', requirement: 'Treatment must vary at the covariate values relevant to the target population.', consequenceIfUnmet: 'Effects in regions without treatment variation are not supported. Overlap weighting changes the target population; it does not recover effects everywhere.', sources: [{ kind: 'paper', title: 'Balancing covariates via propensity score weighting (Li, Morgan and Zaslavsky, 2018)', locator: 'JASA 113(521); overlap weights' }] },
    { id: caveatId('forest-settings'), category: 'computation', requirement: 'Forest settings must be valid and recorded with the result.', consequenceIfUnmet: 'Invalid settings prevent estimation. Failed tuning is reported rather than silently substituted.', sources: [{ kind: 'hirmos-constraint', locator: 'Causal forest configuration and worker contract' }] },
    { id: caveatId('forest-reading'), category: 'interpretation', requirement: 'Conditional predictions describe averages at covariate values, not observed individual effects. Row intervals are pointwise.', consequenceIfUnmet: 'A spread of predictions alone does not establish treatment-effect heterogeneity.', sources: [{ kind: 'paper', title: 'Everyday causal inference', locator: 'Section 12, heterogeneous treatment effects; https://www.everydaycausal.com/heterogeneous-effects.html' }, { kind: 'paper', title: 'GRF calibration test documentation', locator: 'https://grf-labs.github.io/grf/reference/test_calibration.html' }] },
  ],
}

const T_LEARNER: MethodDefinition = {
  id: T_LEARNER_METHOD_ID,
  name: 'T-learner',
  family: 'estimation',
  summary: 'One outcome model per treatment arm on the adjustment variables: a random forest, or a gradient-boosted classifier chosen by grid search and cross-fitted over two halves. Each row’s effect is the treated prediction minus the control prediction at that row.',
  caveats: [
    {
      id: caveatId('t-learner-identified-adjustment'),
      category: 'identification',
      requirement: 'The two outcome models see the identified adjustment set, which is also what each row’s effect is conditioned on; the learner removes no confounding of its own.',
      consequenceIfUnmet: 'Every row’s effect is an adjusted association under a wrong set, so the whole distribution of effects is off, not one number.',
      sources: [MOLAK_CH9('§ T-Learner: Together We Can Do More; DoWhy’s backdoor.econml.metalearners.TLearner'), KUNZEL_2019],
    },
    {
      id: caveatId('t-learner-binary-treatment'),
      category: 'functional-form',
      requirement: 'The treatment is 0 or 1 with rows in both arms, since one outcome model is fitted per arm.',
      consequenceIfUnmet: 'A continuous or one-armed treatment has no second arm to fit and the estimator refuses.',
      sources: [ECONML_TLEARNER, KUNZEL_2019],
    },
    {
      id: caveatId('t-learner-independent-rows'),
      category: 'sampling-structure',
      requirement: 'Rows are independent draws: forests bootstrap the rows, and the search folds and the two halves split them, as exchangeable.',
      consequenceIfUnmet: 'On a series or a panel the arm models treat dependent rows as separate evidence and the effects are read with more confidence than the data carry.',
      sources: [KUNZEL_2019, hirmos('docs/DESIGN.md#10 time dependence is not cosmetic')],
    },
    {
      id: caveatId('t-learner-overlap'),
      category: 'identification',
      requirement: 'Positivity holds at each row’s covariate values: both arms have rows nearby, so neither arm model extrapolates.',
      consequenceIfUnmet: 'Where one arm has no rows nearby, that arm’s prediction is an extrapolation and the row’s effect is unsupported.',
      sources: [RUIZ_DE_VILLA_CH7('§7.4.3 positivity'), HERNAN_ROBINS],
    },
    {
      id: caveatId('t-learner-row-effect-reading'),
      category: 'interpretation',
      requirement: 'A row’s estimate is a conditional average treatment effect at its covariate values, not an identified individual treatment effect.',
      consequenceIfUnmet: 'An average contrast may be mistaken for that individual’s treatment effect, which is not identified by these outcome regressions.',
      sources: [NESS_CH11('§11.4 conditional average treatment effect estimation; meta-learners such as the T-learner'), RUIZ_DE_VILLA_CH8('§8.1.4 heterogeneous treatment effects, the conditional average treatment effect')],
    },
    {
      id: caveatId('t-learner-learner-settings'),
      category: 'computation',
      requirement: 'The run records the learner settings. For boosted models, grid search selects hyperparameters on all rows within each treatment arm before the two-half split. Each half is predicted by models fitted on the other half; hyperparameter selection is not repeated within the training halves.',
      consequenceIfUnmet: 'Changing the model settings changes the estimator specification and every row’s effect.',
      sources: [ECONML_TLEARNER, RUIZ_DE_VILLA_CH5('§5.4.1: cross-fitting with hyperparameter selection within training halves; differs from selection before splitting'), hirmos('crates/causal-core/src/tlearner.rs'), hirmos('crates/analysis-wasm/src/estimation/adjusted_outcome.rs#cross_fitted_t_learner')],
    },
    {
      id: caveatId('t-learner-no-interval'),
      category: 'finite-sample',
      requirement: 'With random forests, optional bootstrap intervals refit both forests on resampled independent rows; row intervals are pointwise, not simultaneous, and the average-effect interval uses a conservative standard-error bound. The boosted, cross-fitted model reports no interval.',
      consequenceIfUnmet: 'The spread between rows is read as heterogeneity when part of it is sampling noise in the two arm models.',
      sources: [ECONML_TLEARNER, KUNZEL_2019],
    },
  ],
}

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
  summary: 'The outcome is modelled from its earlier values and from current and earlier predictor values. For eligible lag orders, an error-correction form tests for a long-run relationship.',
  caveats: [
    {
      id: caveatId('ardl-orders-assessed'),
      category: 'stationarity-and-dynamics',
      requirement: 'Use series that are stationary in levels or after first differencing. Assess their order of integration in Data studio before interpreting the bounds test.',
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
      requirement: 'Include the predictors needed to represent the proposed long-run relationship. Distinguish distributed-lag predictors from fixed regressors such as policy indicators.',
      consequenceIfUnmet: 'An omitted predictor can change the fitted long-run relationship.',
      sources: [statsmodels('statsmodels/tsa/ardl/model.py#UECM'), hirmos('crates/causal-core/src/ardl.rs')],
    },
    {
      id: caveatId('ardl-bounds-reading'),
      category: 'interpretation',
      requirement: 'A statistic above the upper bound supports a level relationship. Below the lower bound, the test does not support one; between the bounds, the result is inconclusive.',
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
  summary: 'Fits long-run equilibrium relationships and estimates how changes in the series respond to departures from them.',
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
      requirement: 'A single long-run relationship is interpreted only at rank one; multiple relationships are reported as coefficient matrices.',
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
      requirement: 'Units are observed over common periods. An outcome-history fit uses donor columns; a predictor-based fit uses a long panel with unit and period keys.',
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
      requirement: 'Donor units are unaffected by the intervention and its spillovers. A donor series must not be a treatment descendant in the recorded graph.',
      consequenceIfUnmet: 'A treated donor absorbs the effect.',
      sources: [ABADIE_2010, hirmos('src/domain/estimation.ts#affectedNodes')],
    },
    {
      id: caveatId('synthetic-convex-hull'),
      category: 'interpretation',
      requirement: 'Inspect pre-intervention fit and predictor balance. Non-negative donor weights summing to one restrict the synthetic control to combinations of the donor units.',
      consequenceIfUnmet: 'Poor pre-intervention fit weakens the case that the donor combination approximates the treated unit without intervention.',
      sources: [ABADIE_2010],
    },
    {
      id: caveatId('synthetic-no-interval'),
      category: 'finite-sample',
      requirement: 'The predictor-based fit reports no uncertainty interval. For an outcome-history fit, interpret cross-fitted confidence inference, donor-placebo ranking and fixed-weight prediction bands separately.',
      consequenceIfUnmet: 'A prediction band or placebo rank is incorrectly presented as a confidence interval for the average effect.',
      sources: [ABADIE_2010, ABADIE_2021],
    },
  ],
}

const PANEL_INTERVENTION: MethodDefinition = {
  id: PANEL_INTERVENTION_METHOD_ID,
  name: 'Panel difference-in-differences',
  family: 'estimation',
  summary: 'Estimate average treatment effects on treated units. Staggered adoption estimates group-time effects and event-time, cohort and calendar averages. Conventional, regression, cross-fitted doubly robust and synthetic DiD use the shared-adoption design.',
  caveats: [
    {
      id: caveatId('panel-balanced-layout'), category: 'sampling-structure',
      requirement: 'The prepared panel has one observation per unit and period on a complete grid. Shared-adoption methods require a common adoption period; staggered DiD permits different adoption cohorts. Treatment is absorbing: units remain treated after adoption.',
      consequenceIfUnmet: 'The implemented weighting system does not define the requested comparison.',
      sources: [ARKHANGELSKY_2021, CALLAWAY_SANTANNA_2021, synthdid('R/utils.R#panel.matrices')],
    },
    {
      id: caveatId('panel-parallel-trends'), category: 'identification',
      requirement: 'Absent treatment, treated and comparison outcomes would follow parallel trends under the selected specification: unadjusted, conditional on covariates, or after synthetic weighting.',
      consequenceIfUnmet: 'The post-period contrast combines the intervention with an untreated trend difference.',
      sources: [ARKHANGELSKY_2021, CALLAWAY_SANTANNA_2021],
    },
    {
      id: caveatId('panel-no-anticipation'), category: 'identification',
      requirement: 'Treatment does not affect outcomes before adoption, or before the specified anticipation window when staggered DiD allows anticipation.',
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
      requirement: 'Synthetic DiD needs pre-period variation to fit weights. The implemented regression and doubly robust specifications require one pre-treatment and one post-treatment period, with adequate covariate support. Doubly robust DiD also needs treatment overlap within its folds.',
      consequenceIfUnmet: 'The selected specification cannot be fitted even when the panel grid is valid and another DiD method is available.',
      sources: [ARKHANGELSKY_2021, ABADIE_2021, synthdid('R/solver.R; R/synthdid.R')],
    },
    {
      id: caveatId('panel-no-interval'), category: 'finite-sample',
      requirement: 'Use the uncertainty for the selected method: regression uses classical independent-error Student-t intervals; DR DiD uses cross-fitted scores with independent units; synthetic placebo inference needs more comparison units than treated units.',
      consequenceIfUnmet: 'The point estimate remains available, but this uncertainty calculation is not.',
      sources: [ARKHANGELSKY_2021],
    },
  ],
}

const NEGBIN_NUTS: MethodDefinition = {
  id: NEGBIN_NUTS_METHOD_ID,
  name: 'Bayesian negative binomial',
  family: 'estimation',
  summary: 'A negative binomial count model with one treatment and one confounder, sampled by the no-U-turn sampler (NUTS). The effect is the posterior expected-count ratio.',
  caveats: [
    {
      id: caveatId('nuts-model-shape'),
      category: 'identification',
      requirement: 'The model takes one confounder, so the identified adjustment set holds one variable.',
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
      requirement: 'There are no divergent transitions, acceptance is near the 0.8 target, and the warmup, draws and seed are recorded.',
      consequenceIfUnmet: 'Posterior summaries from a chain with divergent transitions may not reliably represent the target distribution.',
      sources: [HOFFMAN_GELMAN_2014, BETANCOURT_2017, pyro('pyro/infer/mcmc/nuts.py:57-93'), hirmos('crates/causal-core/src/nuts.rs#multinomial_nuts')],
    },
    {
      id: caveatId('nuts-independence'),
      category: 'noise-and-dependence',
      requirement: 'Rows are independent; the model has no serial-correlation term.',
      consequenceIfUnmet: 'Dependence between rows is not represented in the posterior uncertainty.',
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
      requirement: 'There are no divergent transitions, acceptance is near the 0.8 target in every chain, and the warmup, draws and seed are recorded.',
      consequenceIfUnmet: 'Posterior summaries from a chain with divergent transitions may not reliably represent the target distribution.',
      sources: [HOFFMAN_GELMAN_2014, BETANCOURT_2017, hirmos('crates/causal-core/src/nuts.rs#multinomial_nuts')],
    },
    {
      id: caveatId('bayes-gaussian-independence'),
      category: 'noise-and-dependence',
      requirement: 'Rows are independent; the model has no serial-correlation term.',
      consequenceIfUnmet: 'Dependence between rows is not represented in the posterior uncertainty.',
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
      requirement: 'Observed binary and ordinal states are preserved when they fit the state budget. Higher-cardinality variables are divided into quantile states, each represented in original units by its within-state mean.',
      consequenceIfUnmet: 'Quantile discretisation removes within-state variation and can change the estimated contrast.',
      sources: [NESS_CH4('§4.4.6 discretise continuous variables at quantiles'), pgmpy('pgmpy/estimators/base.py#BaseEstimator'), hirmos('crates/causal-core/src/discrete_bn.rs#discretize_for_discrete_bn')],
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

const DYNAMIC_LINEAR_SCM: MethodDefinition = {
  id: DYNAMIC_LINEAR_SCM_METHOD_ID,
  name: 'Dynamic linear SCM counterfactual',
  family: 'counterfactual',
  summary: 'Fits each series on its contemporaneous and lagged DAG parents, recovers the observed innovation sequence, and replays a one-time or persistent intervention through the temporal graph.',
  caveats: [
    {
      id: caveatId('dynamic-scm-time-series'),
      category: 'sampling-structure',
      requirement: 'The prepared data are one regularly sampled multivariate time series in chronological order.',
      consequenceIfUnmet: 'A lag no longer denotes a consistent elapsed interval, so the fitted dynamic equations and replay horizon have no stable temporal meaning.',
      sources: [RUNGE_2020],
    },
    {
      id: caveatId('dynamic-scm-structural-equations'),
      category: 'functional-form',
      requirement: 'Each variable is linear in the contemporaneous and lagged parents shown in the DAG, with additive innovations.',
      consequenceIfUnmet: 'The recovered innovations and propagated intervention path come from a misspecified structural model.',
      sources: [RUIZ_DE_VILLA_CH7('§7.1.3 structural causal models'), RUIZ_DE_VILLA_CH8('§8.1.1 linear structural equations')],
    },
    {
      id: caveatId('dynamic-scm-graph-complete'),
      category: 'identification',
      requirement: 'Every parent needed by the dynamic structural equations is measured and represented at the correct lag.',
      consequenceIfUnmet: 'An omitted contemporaneous or lagged cause can be absorbed into the innovation sequence and then incorrectly held fixed across worlds.',
      sources: [RUIZ_DE_VILLA_CH7('§7.1.3 omitting an arrow asserts its absence'), PEARL_2009('§7.1 abduction')],
    },
    {
      id: caveatId('dynamic-scm-history'),
      category: 'finite-sample',
      requirement: 'The intervention begins after at least the maximum recorded lag, leaving an observed factual history for the replay.',
      consequenceIfUnmet: 'The first counterfactual values depend on unobserved pre-sample history.',
      sources: [PEARL_2009('§7.1 abduction, action, prediction')],
    },
    {
      id: caveatId('dynamic-scm-modularity'),
      category: 'interpretation',
      requirement: 'The intervention replaces only the treatment equation; the remaining equations and the abducted innovation at each time point are invariant across the two worlds.',
      consequenceIfUnmet: 'The replay does not compare the same evolving system under two alternative interventions.',
      sources: [PEARL_2009('§7.1 abduction, action, prediction'), NESS_CH13('structural mechanisms and counterfactual worlds')],
    },
    {
      id: caveatId('dynamic-scm-intervention-schedule'),
      category: 'interpretation',
      requirement: 'A one-time intervention changes one treatment value; a persistent intervention replaces the treatment equation at every replayed time point.',
      consequenceIfUnmet: 'The reported horizon answers a different intervention from the one intended.',
      sources: [PEARL_2009('§3.2 interventions and modified models')],
    },
    {
      id: caveatId('dynamic-scm-no-interval'),
      category: 'finite-sample',
      requirement: 'A sampling interval requires dependence-aware resampling with a block length appropriate for the series.',
      consequenceIfUnmet: 'A point estimate does not describe sampling uncertainty; an unsuitable block length can misrepresent serial dependence.',
      sources: [KUNSCH_1989],
    },
  ],
}

const INTERRUPTED_SERIES: MethodDefinition = {
  id: INTERRUPTED_SERIES_METHOD_ID,
  name: 'Interrupted series',
  family: 'estimation',
  summary: 'Estimates changes after an intervention, compared with the modelled continuation of the series without it.',
  caveats: [
    {
      id: caveatId('its-impact-model-declared'),
      category: 'functional-form',
      requirement: 'The impact model was chosen before fitting, from what is known about the event.',
      consequenceIfUnmet: 'Choosing the shape to obtain a stronger result can overstate the evidence.',
      sources: [LOPEZ_BERNAL_2017, HERRERA_ITS],
    },
    {
      id: caveatId('its-no-concurrent-change'),
      category: 'identification',
      requirement: 'Consider other changes around the intervention that could affect the outcome.',
      consequenceIfUnmet: 'The model cannot separate the intervention from unaccounted concurrent changes.',
      sources: [LOPEZ_BERNAL_2017],
    },
    {
      id: caveatId('its-seasonality'),
      category: 'stationarity-and-dynamics',
      requirement: 'Seasonality is represented, by harmonic terms at the period or by seasonal adjustment in Data studio.',
      consequenceIfUnmet: 'Unaccounted seasonality can be mistaken for an intervention-related change.',
      sources: [LOPEZ_BERNAL_2017, BHASKARAN_2013],
    },
    {
      id: caveatId('its-residual-autocorrelation'),
      category: 'noise-and-dependence',
      requirement: 'Check residual autocorrelation. For ARMA fits, these checks use the one-step prediction errors.',
      consequenceIfUnmet: 'Newey–West adjusts standard errors for serial dependence. It does not remove that dependence or correct a misspecified trend.',
      sources: [LOPEZ_BERNAL_2017, statsmodels('statsmodels/regression/linear_model.py#RegressionResults.get_robustcov_results'), hirmos('crates/causal-core/src/interrupted_series.rs')],
    },
    {
      id: caveatId('its-arma-errors'),
      category: 'noise-and-dependence',
      requirement: 'Choose ARMA orders using subject knowledge, diagnostics or information criteria. Check convergence and the remaining autocorrelation.',
      consequenceIfUnmet: 'Non-convergence or a poorly specified error model can make estimates and intervals unreliable. Report any model selection.',
      sources: [HYNDMAN_FPP_DHR, statsmodels('statsmodels/tsa/statespace/sarimax.py#SARIMAX'), hirmos('crates/causal-core/src/arma_regression.rs#fit')],
    },
    {
      id: caveatId('its-rows-each-side'),
      category: 'finite-sample',
      requirement: 'Enough rows on each side of the event.',
      consequenceIfUnmet: 'The estimate has little power; interpret with caution.',
      sources: [LOPEZ_BERNAL_2017],
    },
    {
      id: caveatId('its-count-model'),
      category: 'functional-form',
      requirement: 'Counts use quasi-Poisson regression, with a log offset when exposure is supplied. Continuous outcomes use least squares or joint maximum likelihood with ARMA errors.',
      consequenceIfUnmet: 'A Poisson fit with the dispersion fixed at one understates the intervals of an overdispersed count.',
      sources: [LOPEZ_BERNAL_2017, statsmodels('statsmodels/genmod/generalized_linear_model.py#GLM.estimate_scale'), hirmos('crates/causal-core/src/glm.rs#poisson_glm_with')],
    },
  ],
}

export const METHOD_CATALOG: NonEmptyArray<MethodDefinition> = [
  SHARP_RD,
  RD_DESIGN,
  ADF,
  KPSS,
  ZIVOT_ANDREWS,
  GRANGER_SSR_F,
  COUNT_SERIES_INTERVENTION_SCAN,
  PCMCI_PLUS_PAR_CORR,
  JPCMCI_PLUS_PAR_CORR,
  LPCMCI_PAR_CORR,
  RPCMCI_PAR_CORR,
  CDNOTS_PAR_CORR,
  CDNOTS_PLUS_PAR_CORR,
  GRACE,
  DYNOTEARS,
  PC_STABLE,
  FCI,
  DIRECT_LINGAM,
  VAR_LINGAM,
  OCSE,
  CMLP,
  CLSTM,
  BACKDOOR_IDENTIFICATION,
  GRAPHICAL_IDENTIFICATION,
  COUNTERFACTUAL_IDENTIFICATION,
  BACKDOOR_LINEAR_REGRESSION,
  PROPENSITY_WEIGHTING,
  PROPENSITY_MATCHING,
  DOUBLY_ROBUST,
  CONTINUOUS_GPS,
  FRONTDOOR_TWO_STAGE,
  INSTRUMENTAL_VARIABLE,
  POISSON_GLM,
  NEGATIVE_BINOMIAL,
  NEGATIVE_BINOMIAL_INGARCH,
  CAUSAL_EFFECTS_TOTAL,
  CAUSAL_IMPACT,
  DML_PLR,
  DML_IRM,
  T_LEARNER,
  CAUSAL_FOREST,
  DML_REFUTATION,
  ARDL_PSS,
  VECM,
  INTERRUPTED_SERIES,
  SYNTHETIC_CONTROL,
  PANEL_INTERVENTION,
  NEGBIN_NUTS,
  BAYESIAN_GAUSSIAN,
  DISCRETE_BN,
  BINARY_ETT,
  LINEAR_SCM,
  DYNAMIC_LINEAR_SCM,
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

export const IDENTIFICATION_METHODS: NonEmptyArray<MethodDefinition> = [BACKDOOR_IDENTIFICATION, GRAPHICAL_IDENTIFICATION, COUNTERFACTUAL_IDENTIFICATION, RD_DESIGN]
export const COUNTERFACTUAL_METHODS: NonEmptyArray<MethodDefinition> = [LINEAR_SCM, DYNAMIC_LINEAR_SCM]

export const DML_SENSITIVITY_METHODS: NonEmptyArray<MethodDefinition> = [DML_REFUTATION]

export const ESTIMATION_METHODS: NonEmptyArray<MethodDefinition> = [SHARP_RD, BACKDOOR_LINEAR_REGRESSION, FRONTDOOR_TWO_STAGE, INSTRUMENTAL_VARIABLE, BAYESIAN_GAUSSIAN, POISSON_GLM, NEGATIVE_BINOMIAL, NEGATIVE_BINOMIAL_INGARCH, NEGBIN_NUTS, DML_PLR, DML_IRM, T_LEARNER, CAUSAL_FOREST, CAUSAL_EFFECTS_TOTAL, CAUSAL_IMPACT, SYNTHETIC_CONTROL, PANEL_INTERVENTION, ARDL_PSS, VECM, DISCRETE_BN, BINARY_ETT]

export const STATIONARITY_METHODS: NonEmptyArray<MethodDefinition> = [ADF, KPSS, ZIVOT_ANDREWS]
export const TIME_SERIES_METHODS = { ardl: ARDL_PSS, vecm: VECM, interrupted: INTERRUPTED_SERIES } as const
export const COUNT_SERIES_DIAGNOSTIC_METHODS: NonEmptyArray<MethodDefinition> = [COUNT_SERIES_INTERVENTION_SCAN]
export const CROSS_SECTIONAL_DISCOVERY_METHODS: NonEmptyArray<MethodDefinition> = [PC_STABLE, FCI, DIRECT_LINGAM]
export const TEMPORAL_DISCOVERY_METHODS: NonEmptyArray<MethodDefinition> = [
  GRANGER_SSR_F,
  PCMCI_PLUS_PAR_CORR,
  JPCMCI_PLUS_PAR_CORR,
  LPCMCI_PAR_CORR,
  RPCMCI_PAR_CORR,
  CDNOTS_PAR_CORR,
  CDNOTS_PLUS_PAR_CORR,
  GRACE,
  DYNOTEARS,
  VAR_LINGAM,
  OCSE,
  CMLP,
  CLSTM,
]

export function methodDefinition(id: MethodId): Result<MethodDefinition, MethodLookupProblem> {
  const definition = METHOD_CATALOG.find((candidate) => candidate.id === id)
  return definition ? ok(definition) : err({ kind: 'unknown-method', id })
}
