import type { CounterfactualRunArtifact } from './counterfactual'
import type { DiscoveryRunArtifact } from './discovery'
import { assertNever, type NonEmptyArray } from './dop'
import {
  boundsReading,
  intervalTypeOf,
  type EstimateInterval,
  type EstimationRunArtifact,
} from './estimation'
import type { StudySpecification } from './study'
import type { SensitivityRunArtifact } from './sensitivity'
import {
  formatEstimate,
  formatInterval,
  formatPercent,
  formatStatistic,
  type EffectScale,
} from '../lib/format/number'

/**
 * A result explanation is derived from recorded numerical facts. It is not a second result and it
 * never changes eligibility, identification or the estimate. Tagged statements let every renderer
 * preserve the distinction between numerical meaning, uncertainty and an interpretive limit.
 */
export type InterpretationStatement =
  | { readonly kind: 'magnitude'; readonly text: string }
  | { readonly kind: 'uncertainty'; readonly text: string }
  | { readonly kind: 'comparison'; readonly text: string }
  | { readonly kind: 'qualification'; readonly text: string }

export interface ResultInterpretation {
  readonly kind: 'result-interpretation'
  readonly statements: NonEmptyArray<InterpretationStatement>
}

const additiveScale = { kind: 'additive', unit: '' } as const
const irrScale = { kind: 'ratio', label: 'IRR' } as const

const number = (value: number): string => formatStatistic('raw', value).text
const absolute = (value: number): string => number(Math.abs(value))

const change = (value: number, outcome: string): string => {
  if (value > 0) return `an estimated increase of ${absolute(value)} ${outcome} units`
  if (value < 0) return `an estimated decrease of ${absolute(value)} ${outcome} units`
  return `no estimated change in ${outcome}`
}

const gap = (value: number, outcome: string): string => {
  if (value > 0) return `${absolute(value)} ${outcome} units above`
  if (value < 0) return `${absolute(value)} ${outcome} units below`
  return `equal to`
}

const intervalRelation = (lower: number, upper: number, reference: number): 'above' | 'below' | 'includes' => {
  if (lower > reference) return 'above'
  if (upper < reference) return 'below'
  return 'includes'
}

const intervalStatement = (
  interval: Exclude<EstimateInterval, { readonly kind: 'none' }>,
  reference: 0 | 1,
  referenceName: 'no additive effect' | 'no multiplicative change',
): InterpretationStatement => {
  const relation = intervalRelation(interval.lower, interval.upper, reference)
  const level = `${Math.round(interval.level * 100)}%`
  if (interval.kind === 'confidence') {
    if (relation === 'includes') {
      return { kind: 'uncertainty', text: `The ${level} confidence interval includes ${reference}, the null value for ${referenceName}.` }
    }
    return { kind: 'uncertainty', text: `The ${level} confidence interval lies entirely ${relation} ${reference}, the null value for ${referenceName}.` }
  }
  const intervalName = interval.summary === 'HDI' ? 'highest-density interval' : 'credible interval'
  if (relation === 'includes') {
    return { kind: 'uncertainty', text: `The ${level} ${intervalName} includes ${reference}. Under the fitted model and priors, the reported posterior region therefore spans the null value for ${referenceName}.` }
  }
  return { kind: 'uncertainty', text: `The ${level} ${intervalName} lies entirely ${relation} ${reference}. Under the fitted model and priors, the reported posterior region stays on one side of the null value for ${referenceName}.` }
}

const noInterval = (reason: string): InterpretationStatement => ({ kind: 'uncertainty', text: reason })

const adjustedFor = (run: EstimationRunArtifact): string => run.estimate.adjustmentSet.length === 0
  ? 'without measured adjustment variables'
  : `after adjustment for ${run.estimate.adjustmentSet.map((variable) => variable.name).join(', ')}`

const targetPopulation = (study: StudySpecification): string => study.estimand.kind === 'average-treatment-effect-on-treated'
  ? 'among treated rows'
  : 'over the prepared study population'

const ratioMeaning = (ratio: number, outcome: string): string => {
  const percent = formatPercent(Math.abs(ratio - 1), { precision: 1 }).text
  if (ratio > 1) return `The expected ${outcome} count is ${percent} higher`
  if (ratio < 1) return `The expected ${outcome} count is ${percent} lower`
  return `The expected ${outcome} count is unchanged`
}

const intervalDisplay = (run: EstimationRunArtifact, scale: EffectScale): string => {
  const { interval, effect } = run.estimate
  if (interval.kind === 'none' || effect.kind === 'path') return 'no interval'
  return formatInterval(effect.value, interval.lower, interval.upper, intervalTypeOf(interval), scale).text
}

/** The scale line shown under the figure. Method-specific standardisation belongs here, not in UI branches. */
export function resultScaleLine(run: EstimationRunArtifact, study: StudySpecification, stepLabel: string): string {
  switch (run.kind) {
    case 'backdoor-linear-run':
    case 'double-ml-run': return `additive · ${study.outcome.name} units per 1-unit increase in ${study.treatment.name}`
    case 'frontdoor-two-stage-run': return `additive · expected ${study.outcome.name} for ${study.treatment.name} set to ${run.evidence.treatmentValue} rather than ${run.evidence.controlValue}`
    case 'count-glm-run': return `incidence rate ratio · expected ${study.outcome.name} count per 1-unit increase in ${study.treatment.name}`
    case 'negbin-nuts-run': return `incidence rate ratio · expected ${study.outcome.name} count per 1 standard deviation increase in ${study.treatment.name}`
    case 'bayesian-gaussian-run': return `additive · expected ${study.outcome.name} for ${study.treatment.name} set to 1 rather than 0`
    case 'ardl-run': return `additive · long-run ${study.outcome.name} units per 1-unit increase in ${study.treatment.name}`
    case 'vecm-run': return `outcome-normalised long-run relation · ${study.outcome.name} relative to ${study.treatment.name}`
    case 'synthetic-control-run':
    case 'causal-impact-run': return `additive · observed ${study.outcome.name} minus its counterfactual per ${stepLabel}`
    case 'panel-intervention-run': return `additive · average post-adoption ${study.outcome.name} effect among treated units`
    case 'discrete-bn-run': return `additive · expected ${study.outcome.name} in the high rather than low ${study.treatment.name} bin`
    case 'binary-ett-run': return `additive · expected ${study.outcome.name}(1) minus ${study.outcome.name}(0) among rows with ${study.treatment.name} = 1`
    case 'causal-effects-run': return `additive · total effect of setting ${study.treatment.name} from ${run.evidence.interventions[0]} to ${run.evidence.interventions[1]}`
    default: return assertNever(run)
  }
}

/**
 * Explain one estimator result on its actual scale. The sentences are deliberately method-specific:
 * a cointegrating vector is not described as an intervention effect, and a missing interval is never
 * converted into certainty.
 */
export function interpretEstimationResult(run: EstimationRunArtifact, study: StudySpecification, stepLabel: string): ResultInterpretation {
  const { estimate } = run
  switch (run.kind) {
    case 'frontdoor-two-stage-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const mediator = run.columns[run.evidence.mediator]?.name ?? 'the mediator'
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Under the fitted two-stage model, setting ${study.treatment.name} from ${number(run.evidence.controlValue)} to ${number(run.evidence.treatmentValue)} changes expected ${study.outcome.name} by ${number(effect)} through ${mediator}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 0, 'no additive effect'),
        { kind: 'qualification', text: `The estimate is the product of the fitted ${study.treatment.name} → ${mediator} and ${mediator} → ${study.outcome.name} coefficients. Its causal interpretation requires the recorded front-door conditions and adequate additive linear models for both stages.` },
      ] }
    }
    case 'backdoor-linear-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const statements: NonEmptyArray<InterpretationStatement> = [
        { kind: 'magnitude', text: `Under the fitted linear adjustment model, a 1-unit increase in ${study.treatment.name} corresponds to ${change(effect, study.outcome.name)} ${targetPopulation(study)}, ${adjustedFor(run)}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 0, 'no additive effect'),
        { kind: 'qualification', text: `This coefficient has a causal interpretation only under the recorded back-door, study-design and linear-model assumptions. The ${run.configuration.covariance === 'hac' ? 'HAC' : 'classical'} interval determines how sampling uncertainty is calculated.` },
      ]
      return { kind: 'result-interpretation', statements }
    }
    case 'count-glm-run': {
      const ratio = estimate.effect.kind === 'incidenceRateRatio' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${ratioMeaning(ratio, study.outcome.name)} for each 1-unit increase in ${study.treatment.name}, ${adjustedFor(run)}. The incidence rate ratio is ${formatEstimate(ratio, irrScale).text}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 1, 'no multiplicative change'),
        { kind: 'qualification', text: `This is a conditional mean-rate comparison from the fitted ${run.evidence.family === 'poisson' ? 'Poisson' : 'negative-binomial'} model. It is not a ratio of observed totals.` },
      ] }
    }
    case 'double-ml-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const contrast = run.evidence.model === 'irm' ? `Changing ${study.treatment.name} from 0 to 1` : `A 1-unit increase in ${study.treatment.name}`
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${contrast} corresponds to ${change(effect, study.outcome.name)} ${targetPopulation(study)} after cross-fitted adjustment.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 0, 'no additive effect'),
        { kind: 'qualification', text: `The result targets ${run.evidence.att ? 'the average effect among treated rows' : 'the average effect over the prepared population'}. Cross-fitting reduces nuisance-model bias but does not replace the recorded identification and overlap assumptions.` },
      ] }
    }
    case 'ardl-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const reading = boundsReading(run.evidence)
      const bounds = reading === 'level-relation'
        ? 'The bounds statistic is above the 5% upper bound, which supports a level relationship under the selected lag and deterministic specification.'
        : reading === 'no-level-relation'
          ? 'The bounds statistic is below the 5% lower bound, so this specification does not support a level relationship.'
          : 'The bounds statistic lies between the 5% bounds, so the level-relationship test is inconclusive.'
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The estimated long-run coefficient associates a 1-unit higher level of ${study.treatment.name} with ${change(effect, study.outcome.name)} in the fitted equilibrium relation.` },
        { kind: 'comparison', text: bounds },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 0, 'no additive effect'),
        { kind: 'qualification', text: 'A bounds-test level relationship and a long-run coefficient do not by themselves establish causal direction.' },
      ] }
    }
    case 'vecm-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The outcome-normalised cointegrating vector implies ${change(effect, study.outcome.name)} for a 1-unit long-run change in ${study.treatment.name}.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : `Reported interval: ${intervalDisplay(run, additiveScale)}.`),
        { kind: 'qualification', text: `The rank-${run.evidence.rank} cointegrating relation describes long-run co-movement. It does not by itself identify an intervention effect or its direction.` },
      ] }
    }
    case 'synthetic-control-run': {
      const effect = estimate.effect.kind === 'path' ? estimate.effect : null
      const average = effect?.aggregate.average ?? Number.NaN
      const cumulative = effect?.aggregate.cumulative ?? Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `After the intervention, observed ${study.outcome.name} averaged ${gap(average, study.outcome.name)} the synthetic-control counterfactual per ${stepLabel}. The cumulative gap is ${number(cumulative)} ${study.outcome.name} units.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No aggregate uncertainty interval is available.'),
        { kind: 'qualification', text: `The comparison is credible only to the extent that the weighted donors reproduced the treated series before intervention and remained unaffected afterwards. The pre-period loss is a fit measure, not a causal test.` },
      ] }
    }
    case 'panel-intervention-run': {
      const primary = run.evidence.syntheticDid.estimate
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Synthetic difference-in-differences estimates that treated units averaged ${gap(primary, study.outcome.name)} their weighted counterfactual after adoption.` },
        { kind: 'comparison', text: `The same panel gives DID ${number(run.evidence.did.estimate)} and synthetic control ${number(run.evidence.syntheticControl.estimate)}; these are comparison estimates, while synthetic difference-in-differences is the predeclared primary result.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No resampling interval is available.'),
        { kind: 'qualification', text: 'A causal reading depends on the recorded adoption pattern, comparison-unit validity and untreated potential-outcome assumptions. Agreement between the 3 estimators is not an identification test.' },
      ] }
    }
    case 'negbin-nuts-run': {
      const ratio = estimate.effect.kind === 'incidenceRateRatio' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${ratioMeaning(ratio, study.outcome.name)} for a 1 standard deviation increase in ${study.treatment.name}, conditional on the model's single standardised confounder. The posterior median incidence rate ratio is ${formatEstimate(ratio, irrScale).text}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 1, 'no multiplicative change'),
        { kind: 'qualification', text: 'The interval summarises posterior draws under the fitted Gamma–Poisson model and priors. It is not a frequentist confidence interval.' },
      ] }
    }
    case 'bayesian-gaussian-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Setting ${study.treatment.name} to 1 rather than 0 gives ${change(effect, study.outcome.name)} on average under the fitted Gaussian model, ${adjustedFor(run)}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, 0, 'no additive effect'),
        { kind: 'comparison', text: `${formatPercent(run.evidence.probabilityPositive, { precision: 1 }).text} of retained posterior effect draws are above zero. This is a posterior probability under the specified model and priors, not the probability that the causal assumptions are true.` },
      ] }
    }
    case 'discrete-bn-run': {
      const [low, high] = run.evidence.treatmentStates
      const [expectedLow, expectedHigh] = run.evidence.expectations
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Under the fitted interventional network, setting ${study.treatment.name} from ${low} to ${high} changes expected ${study.outcome.name} from ${number(expectedLow)} to ${number(expectedHigh)}, a difference of ${number(run.evidence.effect)}.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No uncertainty interval is available.'),
        { kind: 'qualification', text: 'The contrast is between quantile-bin states, not a 1-unit change on the original continuous scale. Its causal interpretation depends on the graph, adjustment and discretisation choices.' },
      ] }
    }
    case 'binary-ett-run': {
      const { evidence } = run
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Among rows with ${study.treatment.name} = 1, the identified model gives mean ${study.outcome.name}(1) of ${number(evidence.treatedPotentialOutcomeMean)} and mean ${study.outcome.name}(0) of ${number(evidence.untreatedPotentialOutcomeMean)}. Their difference is ${number(evidence.effectOnTreated)}.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No uncertainty interval is available.'),
        { kind: 'qualification', text: 'This is the binary effect on the treated under the recorded graph and empirical joint distribution. It is not an ATE, and no continuous variable was discretised automatically.' },
      ] }
    }
    case 'causal-effects-run': {
      const [low, high] = run.evidence.interventions
      const effect = run.evidence.totalEffect ?? Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Setting ${study.treatment.name} from ${number(low)} to ${number(high)} changes predicted ${study.outcome.name} by ${number(effect)} on average under the fitted total-effect model.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No uncertainty interval is available.'),
        { kind: 'qualification', text: `This is a total effect through the directed paths retained in the time-series graph. The ${run.evidence.estimator.kind === 'linear' ? 'linear' : `${run.evidence.estimator.k}-neighbour`} outcome model supplies the numerical contrast.` },
      ] }
    }
    case 'causal-impact-run': {
      const effect = estimate.effect.kind === 'path' ? estimate.effect : null
      const average = effect?.aggregate.average ?? Number.NaN
      const cumulative = effect?.aggregate.cumulative ?? Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `During the post-intervention period, observed ${study.outcome.name} averaged ${gap(average, study.outcome.name)} its state-space counterfactual per ${stepLabel}. The cumulative difference is ${number(cumulative)} ${study.outcome.name} units.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No interval for the aggregate effect is available.'),
        { kind: 'qualification', text: 'The pointwise band describes counterfactual forecast uncertainty. It is not an interval for the average or cumulative effect, and the causal reading depends on stable, unaffected controls.' },
      ] }
    }
    default: return assertNever(run)
  }
}

/** Discovery numbers describe fitted structural evidence, not identified intervention effects. */
export function interpretDiscoveryResult(run: DiscoveryRunArtifact): ResultInterpretation {
  switch (run.kind) {
    case 'direct-lingam-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `The reported order places the ${run.result.variables} variables in the sequence inferred from non-Gaussianity. Each nonzero weight is the fitted linear structural coefficient from its source to its target after adaptive-lasso pruning.` },
      { kind: 'qualification', text: 'The order and weights identify a causal structure only under the linear, acyclic, causally sufficient model with mutually independent non-Gaussian disturbances. They are not intervention-effect estimates.' },
    ] }
    case 'pcmci-plus-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `A marked cell records a conditional-dependence relation selected at alpha ${run.result.pcAlpha}. Its lag says how many time steps the source precedes the target; ParCorr gives the signed conditional association.` },
      { kind: 'qualification', text: 'Same-period o–o endpoints remain unoriented. The resulting time-series CPDAG represents an equivalence class, not a completed causal DAG or an intervention-effect estimate.' },
    ] }
    case 'lpcmci-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `A marked cell records lagged or same-period graphical evidence selected at alpha ${run.result.pcAlpha}. Endpoint marks carry the orientation information; ParCorr gives the signed conditional association used by the test.` },
      { kind: 'qualification', text: 'Circles preserve unresolved endpoints and bidirected marks permit latent confounding. The PAG is evidence to review, not a fully oriented causal DAG or an intervention-effect estimate.' },
    ] }
    case 'dynotears-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: 'Each nonzero weight is a fitted linear structural coefficient from source(t−lag) to target(t). Its sign gives the fitted direction of association; its magnitude depends on the variables’ scales and the selected penalties.' },
      { kind: 'qualification', text: 'DYNOTEARS supplies a sparse candidate structure under its model assumptions. The weights are not uncertainty intervals or identified intervention effects.' },
    ] }
    case 'var-lingam-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `The selected lag is ${run.result.selectedLag}. Each displayed coefficient belongs to the fitted contemporaneous or lagged structural vector autoregression; the reported order is the instantaneous order inferred from non-Gaussian residuals.` },
      { kind: 'qualification', text: 'The order and weights rely on linearity, non-Gaussian independent disturbances and the stated lag model. They remain structural evidence to review against domain knowledge.' },
    ] }
    case 'ocse-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `${run.result.edges.length} lagged relations survived forward and backward conditional-information selection. CMI measures remaining conditional information; the permutation p-value compares it with shuffled data.` },
      { kind: 'qualification', text: 'A selected relation is conditional-information evidence at the stated lag. It is not an estimate of the effect of intervening on the source.' },
    ] }
    default: return assertNever(run)
  }
}

/** Structural counterfactuals require a stronger reading than an average-effect estimate. */
export function interpretCounterfactualResult(run: CounterfactualRunArtifact, study: StudySpecification, stepLabel: string): ResultInterpretation {
  const { evidence } = run
  const average = evidence.averageEffect
  const positive = formatPercent(evidence.sharePositive, { precision: 1 }).text
  return { kind: 'result-interpretation', statements: [
    { kind: 'magnitude', text: `Across ${evidence.observations} ${stepLabel}s, setting ${study.treatment.name} from ${number(evidence.interventions[0])} to ${number(evidence.interventions[1])} gives ${change(average, study.outcome.name)} on average. ${positive} of the model-implied row effects are positive.` },
    { kind: 'qualification', text: 'These are model-implied alternatives, not 2 outcomes observed for the same row. The calculation assumes the recorded DAG, linear additive equations, measured fitted nodes and invariant non-treatment mechanisms.' },
    { kind: 'uncertainty', text: evidence.observationNoise === null
      ? 'Exact disturbance-term abduction reproduces each fitted row but does not provide a sampling or posterior uncertainty interval.'
      : `The observation-noise scale is ${number(evidence.observationNoise)}. It changes disturbance-term abduction but is not a reported interval for the individual effects.` },
  ] }
}

/** Sensitivity procedures answer different questions; none is a second identification analysis. */
export function interpretSensitivityResult(run: SensitivityRunArtifact): ResultInterpretation {
  switch (run.kind) {
    case 'linear-refutation-run': {
      const placeboDistance = Math.abs(run.evidence.placeboEffect)
      const subsetMovement = Math.abs(run.evidence.subsetEffect - run.evidence.estimate)
      const randomMovement = Math.abs(run.evidence.randomCommonCauseEffect - run.evidence.estimate)
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `The permuted-treatment estimate is ${number(run.evidence.placeboEffect)} (${number(placeboDistance)} from zero). Keeping ${formatPercent(run.evidence.subsetFraction, { precision: 0 }).text} of rows moves the estimate by ${number(subsetMovement)}, and adding an independent random covariate moves it by ${number(randomMovement)}.` },
        { kind: 'qualification', text: 'These are perturbation distances, not hypothesis-test p-values. Small movement under a chosen perturbation supports numerical stability to that perturbation only; it does not establish identification or rule out unmeasured confounding.' },
      ] }
    }
    case 'dml-refutation-run': {
      const alpha = 0.05
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `The placebo mean-shift test ${run.evidence.placebo.pValue < alpha ? 'rejects' : 'does not reject'} a zero simulated mean (p = ${number(run.evidence.placebo.pValue)}). The random-common-cause test ${run.evidence.randomCommonCause.pValue < alpha ? 'rejects' : 'does not reject'} it (p = ${number(run.evidence.randomCommonCause.pValue)}).` },
        { kind: 'magnitude', text: `The robustness value is ${number(run.evidence.sensitivity.robustnessValue)}: the equal confounding share required by this sensitivity model to move the point estimate to zero. The interval robustness value is ${number(run.evidence.sensitivity.robustnessValueCi)}.` },
        { kind: 'qualification', text: 'The p-values refer to the simulated refuter distributions, not to the truth of the causal graph. Robustness values are calibrated to the stated confounding model and do not cover arbitrary violations.' },
      ] }
    }
    case 'unobserved-confounding-run': {
      const effects = run.evidence.effects.flat()
      const flips = effects.filter((value) => Math.sign(value) !== Math.sign(run.evidence.originalEffect)).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `Across the specified ${run.evidence.kappaT.length} × ${run.evidence.kappaY.length} confounder grid, the refitted effect ranges from ${number(Math.min(...effects))} to ${number(Math.max(...effects))}; ${flips} of ${effects.length} scenarios change its sign.` },
        { kind: 'qualification', text: 'This is a deterministic scenario analysis over the chosen treatment-flip and outcome-shift strengths. It assigns no probability to those scenarios and does not prove that unmeasured confounding is absent outside the grid.' },
      ] }
    }
    default: return assertNever(run)
  }
}
