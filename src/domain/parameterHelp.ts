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
    context:
      'Time basis added as observed context so changes in causal mechanisms can inform orientation.',
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
    interval:
      'How the errors are treated: independent, unequal in variance, correlated within clusters, serially correlated with a robust interval, or an ARMA process fitted with the coefficients.',
    clusterColumn:
      'Select the column whose values identify clusters, such as the unit in repeated observations. Keep the cluster column out of the design matrix.',
    fixedEffects:
      'Account for additive differences between units, differences shared within periods, or both. Treatment must retain variation after removing the selected effects.',
    unitColumn:
      'Select the column whose values identify the unit observed repeatedly, such as a person or a firm. Keep the unit column out of the design matrix.',
    timeColumn:
      'Select the column identifying each period, such as the month. Period effects account for additive differences shared within periods. Keep this column outside the regression design.',
    autoregressiveOrder: 'p',
    movingAverageOrder: 'q',
    armaIterations: 'Maximum number of optimiser iterations.',
  },
  interruptedSeries: {
    outcomeType:
      'This can be either a linear model of the series or a quasi-Poisson model with an offset.',
    exposure: 'The population or denominator the count is a rate of; its log enters as an offset.',
    interventionRow: 'The first row after the event. Rows before it are the pre-period.',
    lag: 'Rows after the intervention row before the change is assumed to start.',
    impactModel: {
      level: 'Assumes a lasting level change from the first affected row, with no slope change.',
      levelAndSlope: 'Assumes a level change and a slope change from the first affected row.',
      slope:
        'Assumes a slope change from the first affected row, with no separate level-change term.',
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
    outcomeModel:
      'Random forest fits one outcome model on all rows within each treatment arm. For boosted models, grid search selects hyperparameters within each arm before a two-half split. Each half is predicted by models fitted on the other half. Hyperparameter selection uses the full sample, not just the training half.',
    boostedSeed: 'Random seed for the boosted trees and the split into two halves.',
  },
  propensity: {
    treatmentModel:
      'Newton-Raphson and L-BFGS-B fit logistic regression. Boosted trees can model nonlinear relationships and interactions; grid search selects hyperparameters using mean cross-validation ROC AUC. Predictive performance alone does not establish adequate overlap or probability calibration.',
    maxIter:
      'Maximum number of L-BFGS-B iterations. Reaching the limit does not establish convergence.',
    scale:
      'Inverse probability weights use the probability of the treatment received. Stabilized weights multiply these by the marginal proportion in the corresponding arm. The estimator adjusts the denominators to preserve the same effect estimate; stabilization does not change relative weights within an arm.',
    uncertainty:
      'A point estimate alone, or a percentile interval from refitting the treatment model on bootstrap resamples of the rows.',
    crossFitted:
      'One model fits and scores the full sample. Cross-fitted fits each half and predicts the other half. Both modes select hyperparameters on the full sample first. Only cross-fitted predictions exclude the prediction rows from the final fit; neither mode excludes them from hyperparameter selection.',
    splits: 'Number of cross-validation folds used to calculate each candidate’s mean ROC AUC.',
    learningRates:
      'How much each tree contributes to the prediction. Smaller learning rates reduce each tree’s contribution and may require more trees. Enter values to compare, separated by commas.',
    maxDepths:
      'Maximum number of splits along a path through each tree. Greater depth allows more complex relationships. Enter depths to compare, separated by commas.',
    nEstimators:
      'Number of trees fitted in sequence. Each tree builds on the preceding predictions. Enter tree counts to compare, separated by commas.',
    treeSeed:
      'Seed for the boosted trees and for the split that cross-fitting uses, so a run repeats exactly.',
    gpsScale:
      'Inverse-density weighting uses one divided by the conditional treatment density. Stabilized weighting uses the marginal treatment density divided by the conditional treatment density. Stabilization changes the relative weights assigned to observations and can change the fitted slope. Neither weighting method guarantees adequate treatment overlap.',
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
    chowBreakRow:
      'Optional number of rows before the parameter-stability split; the next row begins the second fit.',
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
    stateBudget:
      'Maximum number of states retained or created for each variable. Observed low-cardinality states are preserved; higher-cardinality values are divided at quantiles.',
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
    treatmentFlipStrength:
      'Fraction of binary treatment values flipped by the simulated confounder.',
    outcomeShiftStrength: 'Coefficient multiplying the simulated confounder in the outcome.',
  },
} as const
