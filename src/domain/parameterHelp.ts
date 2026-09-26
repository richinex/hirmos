/**
 * Short descriptions for numerical controls. These define the parameter itself; method assumptions
 * and interpretation belong in the method catalogue and result panels.
 */
export const DISCOVERY_PARAMETER_HELP = {
  pcmci: {
    maximumLag: 'Largest time lag tested.',
    pcAlpha: 'Significance level used during condition selection.',
  },
  jpcmci: {
    maximumLag: 'Largest source lag tested within each aligned panel unit.',
    pcAlpha: 'Significance level used during joint condition selection.',
  },
  rpcmci: {
    regimes: 'Number of regimes to estimate.',
    maximumTransitions: 'Maximum membership changes permitted for each regime.',
    maximumLag: 'Largest time lag tested.',
    graphAlpha: 'Significance threshold used to include graph edges.',
    minimumLag: 'Smallest time lag tested; 1 excludes contemporaneous links.',
    pcAlpha: 'Significance level used during condition selection.',
    switchingThreshold: 'Minimum regime weight required to include a time point.',
    iterationsPerAnnealing: 'Maximum optimization iterations for each initialization.',
    annealingRuns: 'Number of random initializations attempted.',
    seed: 'Base seed used for reproducible initialization.',
  },
  cdnots: {
    maximumLag: 'Largest source lag included in conditional-independence tests.',
    alpha: 'Significance threshold used to remove links during skeleton discovery.',
    missing: 'Uses pairwise-complete test samples or records VAR-EM imputation before discovery.',
    context: 'Time basis added as observed context so changes in causal mechanisms can inform orientation.',
  },
  grace: {
    maximumLag: 'Largest source lag included in the CD-NOTS skeleton and neural refinement.',
    alpha: 'Significance threshold used to construct the initial CD-NOTS skeleton.',
    context: 'Time basis supplied to the initial skeleton to represent nonstationarity.',
    gateThreshold: 'Minimum fitted hard-concrete gate value retained as a relation.',
    epochs: 'Maximum number of neural optimization passes.',
    patience: 'Epochs without an improved loss permitted before training stops.',
    seed: 'Seed used for reproducible initialization and hard-concrete samples.',
  },
  dynotears: {
    maximumLag: 'Largest autoregressive lag included in the model.',
    contemporaneousPenalty: 'Sparsity penalty applied to contemporaneous weights.',
    laggedPenalty: 'Sparsity penalty applied to lagged weights.',
  },
  varLingam: {
    maximumLag: 'Largest lag considered when BIC selects the VAR order.',
    prune: 'Applies adaptive lasso to prune estimated connections.',
  },
  ocse: {
    maximumLag: 'Largest source lag considered.',
    informationEstimator: 'Method used to estimate conditional mutual information.',
    testAlpha: 'Significance threshold used during forward and backward selection.',
    permutationShuffles: 'Number of permutations used to form each null distribution.',
  },
  neural: {
    maximumLag: 'Number of previous time points supplied to each cMLP forecast.',
    context: 'Length of each overlapping history sequence used to train cLSTM.',
    hiddenWidth: 'Number of units in the hidden layer of each component network.',
    activation: 'Nonlinear activation between cMLP layers.',
    penalty: 'Structured sparsity penalty applied to cMLP input weights.',
    sparsity: 'Strength of the penalty that sets input groups to zero.',
    ridge: 'Ridge penalty applied to weights outside the sparse input groups.',
    learningRate: 'Step size used by the source ISTA training procedure.',
    iterations: 'Maximum number of ISTA updates.',
    checkEvery: 'Number of updates between convergence checks and recorded loss values.',
    lookback: 'Number of recorded loss values used by the early-stopping check.',
    seed: 'Seed used for reproducible network initialization.',
  },
} as const

export const ESTIMATION_PARAMETER_HELP = {
  frontdoor: {
    controlValue: 'Treatment value used for the control intervention.',
    treatmentValue: 'Treatment value used for the active intervention.',
    bootstrapResamples: 'Number of row-bootstrap samples used for the interval.',
    bootstrapSeed: 'Seed used to reproduce the bootstrap samples.',
  },
  instrumentalVariable: {
    bootstrapResamples: 'Number of row-bootstrap samples used for the interval.',
    bootstrapSeed: 'Seed used to reproduce the bootstrap samples.',
  },
  adjustedRegression: {
    interval: 'How the errors are treated: independent, serially correlated with a robust interval, or an ARMA process fitted with the coefficients.',
    autoregressiveOrder: 'p',
    movingAverageOrder: 'q',
    armaIterations: 'Maximum number of optimiser iterations.',
  },
  interruptedSeries: {
    outcomeType: 'This can be either a linear model of the series or a quasi-Poisson model with an offset.',
    exposure: 'The population or denominator the count is a rate of; its log enters as an offset.',
    interventionRow: 'The first row after the event. Rows before it are the pre-period.',
    lag: 'Rows after the intervention row before the change is assumed to start.',
    impactModel: {
      level: 'Assumes a lasting level change from the first affected row, with no slope change.',
      levelAndSlope: 'Assumes a level change and a slope change from the first affected row.',
      slope: 'Assumes a slope change from the first affected row, with no separate level-change term.',
      temporaryLevel: 'Assumes a temporary level change. The change ends before the until row.',
    },
    untilRow: 'The first row where the temporary change no longer applies.',
    seasonalTerms: 'Sine and cosine pairs at the sampling period; 0 for none.',
    noSeasonalPeriod: 'Yearly rows have no seasonal period.',
    neweyWest: 'Least squares with Newey–West standard errors.',
    armaErrors: 'Maximum likelihood with an ARMA error process.',
    neweyWestBandwidth: 'The number of autocovariances included.',
  },
  dml: {
    foldSeed: 'Seed used to reproduce the shuffled cross-fitting folds.',
  },
  tLearner: {
    learnerSeed: 'One seed for both outcome forests.',
    outcomeModel: 'Random forest is EconML’s TLearner with a forest per arm, fitted on all rows. Boosted, cross-fitted chooses one gradient-boosted classifier per arm by grid search on that arm’s rows, then splits the data set into 2 equally sized data sets; each set’s predictions come from the models trained on the other.',
    boostedSeed: 'scikit-learn’s random_state for the trees, and the seed of the split into two sets.',
  },
  propensity: {
    treatmentModel: 'Newton-Raphson fits the score the way statsmodels does, over a LAPACK solve. L-BFGS-B fits it the way scikit-learn does, and stops on its own criteria. Boosted trees search a grid of gradient-boosted classifiers and keep the one with the best held-out ROC AUC, which fits a score no linear form can.',
    maxIter: "scikit-learn's iteration limit. Its own default is 100; the chapter raises it to 1000.",
    scale: 'Inverse probability reweights the sample by the inverse probability of treatment. Stabilized weights multiply that by the marginal prevalence of the arm, which leaves this estimand unchanged and reduces the spread of the weights.',
    uncertainty: 'A point estimate alone, or a percentile interval from refitting the treatment model on bootstrap resamples of the rows.',
    crossFitted: 'One model fits the whole sample and scores it. Cross-fitted splits the rows in half, fits a model on each half, and scores every row with the model that did not see it, so a flexible model cannot score its own training rows.',
    splits: 'Folds the grid search scores each candidate on. Five is the usual choice.',
    learningRates: 'Values of learning_rate to search, separated by commas. scikit-learn: “Learning rate shrinks the contribution of each tree by learning_rate. There is a trade-off between learning_rate and n_estimators.”',
    maxDepths: 'Values of max_depth to search, separated by commas. scikit-learn: “Maximum depth of the individual regression estimators. The maximum depth limits the number of nodes in the tree.”',
    nEstimators: 'Values of n_estimators to search, separated by commas. scikit-learn: “The number of boosting stages to perform.”',
    treeSeed: 'Seed for the boosted trees and for the split that cross-fitting uses, so a run repeats exactly.',
    gpsScale: 'Inverse density weights each row by 1 over the conditional density of the treatment it received. Stabilized weights multiply that by the marginal density. With a continuous treatment, stabilizing is necessary rather than optional.',
  },
  ardl: {
    maximumLag: 'Largest lag considered by the AIC order search.',
    deterministicTerms: 'Deterministic terms included in the model.',
    pssCase: 'Bounds-test case determined by the deterministic specification.',
  },
  vecm: {
    maximumLags: 'Largest lag considered when selecting the VAR order.',
    deterministicTerms: 'Placement of constants and trends in the VECM.',
    traceSignificance: 'Significance level used by the Johansen trace test.',
    chowBreakRow: 'Optional number of rows before the parameter-stability split; the next row begins the second fit.',
  },
  syntheticControl: {
    interventionStart: 'First observation in the post-intervention period.',
    donorSeries: 'Untreated series available to construct the weighted comparison.',
    crossFitFolds: 'Number of folds used for cross-fitted effect inference.',
    inferenceAlpha: 'Tail probability used to construct the confidence interval.',
  },
  panelIntervention: {
    placeboReplications: 'Number of placebo treatment assignments used for inference.',
    placeboSeed: 'Seed used to reproduce the placebo assignments.',
  },
  nuts: {
    warmup: 'Number of adaptation draws discarded before posterior sampling.',
    draws: 'Number of posterior draws retained per chain.',
    seed: 'Seed used to reproduce sampler initialization and draws.',
  },
  discreteBn: {
    stateBudget: 'Maximum number of states retained or created for each variable. Observed low-cardinality states are preserved; higher-cardinality values are divided at quantiles.',
    equivalentSampleSize: 'Strength of the BDeu prior relative to the observed data.',
  },
  ingarch: {
    meanLink: 'Link function relating predictors to the conditional mean.',
    pastCountLags: 'Past observed-count lags included in the conditional mean.',
    pastMeanLags: 'Past conditional-mean lags included in the conditional mean.',
    forecastPeriods: 'Number of post-intervention periods forecast.',
    treatmentSchedule: 'Pattern applied to the treatment during the forecast.',
    controlValue: 'Treatment value used for the control forecast.',
    treatmentValue: 'Treatment value used for the intervention forecast.',
    decay: 'Fraction of the previous treatment value retained each period.',
  },
  causalEffects: {
    effectModel: 'Model used to estimate the graph-adjusted total effect.',
    neighbours: 'Number of neighbours used by the k-nearest-neighbours model.',
    adjustmentSet: 'Time-indexed variables conditioned on when estimating the effect.',
    adjustmentMembers: 'Variables and lags in the user-supplied adjustment set.',
    variable: 'Variable included in the adjustment set.',
    lag: 'Time lag of the selected adjustment variable.',
    treatmentLag: 'Lag of the treatment whose effect is estimated.',
    fromValue: 'Treatment value used for the reference prediction.',
    toValue: 'Treatment value used for the intervention prediction.',
    uncertainty: 'Whether uncertainty is estimated with a block bootstrap.',
    bootstrapSamples: 'Number of block-bootstrap samples used for the interval.',
    blockLength: 'Rule used to set the number of consecutive observations per block.',
    observationsPerBlock: 'Number of consecutive observations sampled in each block.',
    confidenceLevel: 'Coverage level of the bootstrap interval.',
    bootstrapSeed: 'Seed used to reproduce the block-bootstrap samples.',
  },
  causalImpact: {
    interventionStart: 'First observation in the post-intervention period.',
    evaluationWindow: 'The rows the effect is summarised over. This does not change the fit.',
    controlSeries: 'Unaffected series used to predict the outcome without intervention.',
  },
} as const

export const SENSITIVITY_PARAMETER_HELP = {
  linearRefutation: {
    simulations: 'Number of perturbation refits averaged for each refuter.',
    subsetFraction: 'Fraction of rows sampled in each data-subset refit.',
    seed: 'Base seed used to reproduce the perturbation samples.',
    ljungBoxLags: 'Largest residual lag included in the Ljung–Box test.',
  },
  dmlRefutation: {
    foldSeed: 'Seed used to reproduce cross-fitting and refutation samples.',
  },
  unobservedConfounding: {
    seed: 'Seed used to reproduce the simulated confounder.',
    treatmentFlipStrength: 'Fraction of binary treatment values flipped by the simulated confounder.',
    outcomeShiftStrength: 'Coefficient multiplying the simulated confounder in the outcome.',
  },
} as const
