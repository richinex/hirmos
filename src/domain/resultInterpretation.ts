import type { CounterfactualRunArtifact } from './counterfactual'
import { namesInProse } from '@/lib/format/names'
import type { DiscoveryRunArtifact } from './discovery'
import { assertNever, isNonEmpty, type NonEmptyArray } from './dop'
import {
  adjustmentLabels,
  boundsReading,
  headlineValue,
  summariseRowEffects,
  tLearnerRowEffects,
  treatmentModelStoppedEarly,
  type EstimateInterval,
  type EstimationRunArtifact,
} from './estimation'
import type { StudySpecification } from './study'
import type { SensitivityRunArtifact } from './sensitivity'
import type { AdjustedDidSpecification } from './adjustedDid'
import {
  formatCount,
  formatEstimate,
  formatPercent,
  formatStatistic,
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

const countRatioScale = { kind: 'ratio', label: 'ECR' } as const

const number = (value: number): string => formatStatistic('raw', value).text
const absolute = (value: number): string => number(Math.abs(value))

const plainName = (name: string): string => name.replaceAll('_', ' ')

const change = (value: number, outcome: string): string => {
  const name = plainName(outcome)
  if (value > 0) return `an estimated increase of ${absolute(value)} in ${name}`
  if (value < 0) return `an estimated decrease of ${absolute(value)} in ${name}`
  return `no estimated change in ${name}`
}

const gap = (value: number): string => {
  if (value > 0) return `${absolute(value)} higher than`
  if (value < 0) return `${absolute(value)} lower than`
  return 'equal to'
}

const directionalDifference = (value: number): string => {
  if (value > 0) return `${absolute(value)} higher`
  if (value < 0) return `${absolute(value)} lower`
  return '0'
}

const outcomeDifference = (value: number, outcome: string): string => {
  const name = plainName(outcome)
  if (value > 0) return `${absolute(value)} more ${name}`
  if (value < 0) return `${absolute(value)} fewer ${name}`
  return `no difference in ${name}`
}

const relativeLevel = (value: number): string => {
  if (value > 0) return `${absolute(value)} higher`
  if (value < 0) return `${absolute(value)} lower`
  return 'unchanged'
}

const timeBeforeOutcome = (lag: number, stepLabel: string): string => {
  if (lag === 0) return `in the same ${stepLabel} as the outcome`
  if (lag === 1) return `one ${stepLabel} before the outcome`
  return `${lag} ${stepLabel}s before the outcome`
}

const timeIndexedAdjustment = (
  run: Extract<EstimationRunArtifact, { readonly kind: 'causal-effects-run' }>,
  stepLabel: string,
): string => {
  switch (run.estimate.adjustment.kind) {
    case 'none':
      return 'The recorded temporal graph did not require another measured variable for adjustment.'
    case 'time-indexed': {
      const variables = run.estimate.adjustment.variables.map(({ variable, lag }) => {
        const timing = timeBeforeOutcome(Math.abs(lag), stepLabel)
        return `${plainName(variable.name)} ${timing}`
      })
      return `The model accounts for ${variables.join(', ')}, which the recorded temporal graph selected to block non-causal paths.`
    }
    case 'structural-parent-model':
      return 'The model fits each variable from its recorded causes at their stated times and combines the directed treatment-to-outcome paths.'
    case 'contemporaneous':
      return `The model accounts for ${run.estimate.adjustment.variables.map(({ name }) => plainName(name)).join(', ')} in the outcome period.`
    default:
      return assertNever(run.estimate.adjustment)
  }
}

type IntervalRelation = 'above' | 'below' | 'includes'

const intervalRelation = (lower: number, upper: number, reference: number): IntervalRelation => {
  if (lower > reference) return 'above'
  if (upper < reference) return 'below'
  return 'includes'
}

const additiveConfidenceMeaning = (relation: IntervalRelation): string => {
  switch (relation) {
    case 'above': return 'Because the whole range is above zero, the estimated change remains an increase after allowing for sampling uncertainty.'
    case 'below': return 'Because the whole range is below zero, the estimated change remains a decrease after allowing for sampling uncertainty.'
    case 'includes': return 'Because the range includes zero, these data are compatible with no change at this confidence level.'
    default: return assertNever(relation)
  }
}

const directionalConfidenceRange = (lower: number, upper: number, unit: string): string => {
  const relation = intervalRelation(lower, upper, 0)
  switch (relation) {
    case 'above': return `is between ${absolute(lower)} and ${absolute(upper)} higher per ${unit}`
    case 'below': return `is between ${absolute(upper)} and ${absolute(lower)} lower per ${unit}`
    case 'includes': return `runs from ${absolute(lower)} lower to ${absolute(upper)} higher per ${unit}`
    default: return assertNever(relation)
  }
}

type IntervalReference =
  | { readonly kind: 'additive' }
  | { readonly kind: 'count-ratio' }

const intervalStatement = (
  interval: Exclude<EstimateInterval, { readonly kind: 'none' }>,
  reference: IntervalReference,
): InterpretationStatement => {
  const referenceValue = reference.kind === 'additive' ? 0 : 1
  const relation = intervalRelation(interval.lower, interval.upper, referenceValue)
  const level = `${Math.round(interval.level * 100)}%`
  const range = `${number(interval.lower)} to ${number(interval.upper)}`
  const confidenceReading = (() => {
    if (reference.kind === 'additive') return additiveConfidenceMeaning(relation)
    if (relation === 'above') return 'Because the whole range is above 1, every value in it indicates a higher expected count.'
    if (relation === 'below') return 'Because the whole range is below 1, every value in it indicates a lower expected count.'
    return 'Because the range includes 1, it includes no change in the expected count.'
  })()
  const posteriorReading = (() => {
    if (reference.kind === 'additive') {
      if (relation === 'above') return 'Under the fitted model and priors, every value in the reported range is an increase.'
      if (relation === 'below') return 'Under the fitted model and priors, every value in the reported range is a decrease.'
      return 'Under the fitted model and priors, the reported range includes no change.'
    }
    if (relation === 'above') return 'Under the fitted model and priors, every value in the reported range indicates a higher expected count.'
    if (relation === 'below') return 'Under the fitted model and priors, every value in the reported range indicates a lower expected count.'
    return 'Under the fitted model and priors, the reported range includes no change in the expected count.'
  })()
  if (interval.kind === 'confidence') {
    return { kind: 'uncertainty', text: `The ${level} confidence interval is ${range}. ${confidenceReading}` }
  }
  const intervalName = interval.summary === 'HDI' ? 'highest-density interval' : 'credible interval'
  return { kind: 'uncertainty', text: `The ${level} ${intervalName} is ${range}. ${posteriorReading}` }
}

const noInterval = (reason: string): InterpretationStatement => ({ kind: 'uncertainty', text: reason })

const armModel = (evidence: { readonly kind: 'tLearner' | 'crossFittedTLearner' }): string => {
  switch (evidence.kind) {
    case 'tLearner': return 'forest’s'
    case 'crossFittedTLearner': return 'boosted classifier’s'
    default: return assertNever(evidence.kind)
  }
}

const staggeredBaseline = (anticipation: number): string => {
  if (anticipation === 0) return 'the period immediately before adoption'
  return `the period immediately before the ${anticipation}-period anticipation window`
}

/** The two-period regression counts rows, not units, when computing standard errors; the interval says so. */
const adjustedDidUncertainty = (specification: AdjustedDidSpecification, interval: EstimateInterval): InterpretationStatement => {
  const base = interval.kind === 'none' ? noInterval(interval.reason) : intervalStatement(interval, { kind: 'additive' })
  switch (specification.kind) {
    case 'doublyRobust': return base
    case 'regression': return { kind: 'uncertainty', text: `${base.text} This interval assumes independent errors. Because each unit appears in both periods, errors may be correlated within units. These standard errors are not clustered by unit and do not account for that dependence. Clustering can change the interval in either direction.` }
    default: return assertNever(specification)
  }
}

const additiveIntervalForOutcome = (interval: EstimateInterval, outcome: string): InterpretationStatement => {
  switch (interval.kind) {
    case 'none': return noInterval(interval.reason)
    case 'credible': return intervalStatement(interval, { kind: 'additive' })
    case 'confidence': {
      const level = formatPercent(interval.level, { precision: 0 }).text
      const relation = intervalRelation(interval.lower, interval.upper, 0)
      switch (relation) {
        case 'above': return { kind: 'uncertainty', text: `The ${level} confidence interval places the increase in ${plainName(outcome)} between ${absolute(interval.lower)} and ${absolute(interval.upper)}. The entire range points to an increase after allowing for sampling uncertainty.` }
        case 'below': return { kind: 'uncertainty', text: `The ${level} confidence interval places the decrease in ${plainName(outcome)} between ${absolute(interval.upper)} and ${absolute(interval.lower)}. The entire range points to a decrease after allowing for sampling uncertainty.` }
        case 'includes': return { kind: 'uncertainty', text: `The ${level} confidence interval runs from a decrease of ${absolute(interval.lower)} to an increase of ${absolute(interval.upper)} in ${plainName(outcome)}. Because the range includes no difference, the data are also compatible with no change at this confidence level.` }
        default: return assertNever(relation)
      }
    }
    default: return assertNever(interval)
  }
}

/** The adjustment variables inside a sentence: named when few, counted when many. */
const adjustmentNames = (run: EstimationRunArtifact): string =>
  namesInProse(adjustmentLabels(run.estimate.adjustment).map(plainName), (count) => `the ${count} variables of the identified adjustment set`)

const adjustedFor = (run: EstimationRunArtifact): string => adjustmentLabels(run.estimate.adjustment).length === 0
  ? 'without additional measured adjustment variables'
  : `after adjustment for ${adjustmentNames(run)}`

/** The opening of a bottom line that says what the estimate accounted for. */
const accountingOpening = (run: EstimationRunArtifact): string => {
  const effects = run.kind === 'backdoor-linear-run' ? run.configuration.fixedEffects : { kind: 'none' as const }
  const unit = effects.kind === 'unit' || effects.kind === 'time' ? ` and a fixed effect for each ${plainName(effects.name)}` : effects.kind === 'unit-and-time' ? ` and a fixed effect for each ${plainName(effects.name)} and each ${plainName(effects.timeName)}` : ''
  return adjustmentLabels(run.estimate.adjustment).length === 0
    ? (unit === '' ? 'Without additional measured adjustment variables' : `After accounting for ${unit.slice(5)}`)
    : `After accounting for ${adjustmentNames(run)}${unit}`
}

/** Which adjustment variables the unit fixed effects absorbed, as a sentence, or nothing when none were. */
const absorbedSentence = (run: EstimationRunArtifact): string => {
  if (run.kind !== 'backdoor-linear-run' || run.configuration.fixedEffects.kind === 'none' || run.evidence.fixedEffects.kind === 'none' || run.evidence.fixedEffects.absorbed.length === 0) return ''
  const names = run.evidence.fixedEffects.absorbed.map((column) => plainName(run.columns[column]?.name ?? String(column)))
  return ` The fixed effects absorb ${namesInProse(names, (count) => `${count} of the adjustment variables`)}. Their coefficients cannot be estimated separately in this specification.`
}

const targetPopulation = (study: StudySpecification): string => study.estimand.kind === 'average-treatment-effect-on-treated'
  ? 'among rows that received treatment'
  : 'over the prepared study population'

const ratioMeaning = (ratio: number, outcome: string): string => {
  const percent = formatPercent(Math.abs(ratio - 1), { precision: 1 }).text
  const name = plainName(outcome)
  if (ratio > 1) return `The expected ${name} count is ${percent} higher`
  if (ratio < 1) return `The expected ${name} count is ${percent} lower`
  return `The expected ${name} count is unchanged`
}

/** The scale line shown under the figure. Method-specific standardisation belongs here, not in UI branches. */
export function resultScaleLine(run: EstimationRunArtifact, study: StudySpecification, stepLabel: string): string {
  const treatment = plainName(study.treatment.name)
  const outcome = plainName(study.outcome.name)
  switch (run.kind) {
    case 'sharp-rd-run': return `Local difference in ${outcome} at the assignment cutoff, not an average across the prepared population.`
    case 'backdoor-linear-run':
    case 'double-ml-run': return run.estimate.effect.kind === 'byGroup'
      ? `Difference in ${outcome} per 1-unit increase in ${treatment}, within each ${plainName(run.estimate.effect.modifier)} group.`
      : `Difference in ${outcome} per 1-unit increase in ${treatment}.`
    case 'causal-forest-run': return run.evidence.target.kind === 'binary-average' || run.evidence.target.kind === 'binary-conditional'
      ? `Contrast in ${outcome} between treatment 1 and treatment 0.`
      : `Conditional slope of ${outcome} with respect to ${treatment}.`
    case 't-learner-run': return run.estimate.effect.kind === 'perRow'
      ? `Difference in expected ${outcome} with ${treatment} set to 1 rather than 0, estimated for each row.`
      : `Mean over every row of the difference in expected ${outcome} with ${treatment} set to 1 rather than 0.`
    case 'frontdoor-two-stage-run': return `Difference in expected ${outcome} with ${treatment} set to ${run.evidence.treatmentValue} rather than ${run.evidence.controlValue}.`
    case 'instrumental-variable-run': return `Difference in expected ${outcome} with ${treatment} set to 1 rather than 0.`
    case 'count-glm-run': return `Ratio of expected ${outcome} counts for a 1-unit increase in ${treatment}.`
    case 'negative-binomial-ingarch-run': return `Difference in the forecast ${outcome} count per ${stepLabel}, with ${treatment} at ${run.evidence.treatmentValue} rather than ${run.evidence.controlValue}.`
    case 'negbin-nuts-run': return `Ratio of expected ${outcome} counts for a 1-unit increase in ${treatment}.`
    case 'bayesian-gaussian-run': return `Difference in expected ${outcome} with ${treatment} set to 1 rather than 0.`
    case 'ardl-run': return `Long-run difference in ${outcome} per 1-unit increase in ${treatment}.`
    case 'vecm-run': return `Long-run relationship between ${outcome} and ${treatment}.`
    case 'predictor-synthetic-control-run': return `Observed ${outcome} minus its synthetic-control outcome for the selected treated unit, summed over the selected post-intervention periods.`
    case 'synthetic-control-run':
    case 'causal-impact-run': return `Observed ${outcome} minus its estimated no-intervention outcome per ${stepLabel}.`
    case 'panel-intervention-run': return run.evidence.kind==='staggeredDid'?'Average of supported post-adoption event-time ATT estimates.':`Average difference over treated units and post-adoption periods.`
    case 'discrete-bn-run': return `Difference in expected ${outcome} in the high rather than low ${treatment} state.`
    case 'binary-ett-run': return `Expected ${outcome} under treatment minus no treatment among treated rows.`
    case 'propensity-weighting-run': return `Difference in ${outcome} between the arms, with the sample reweighted by the inverse probability of treatment.`
    case 'propensity-matching-run': return `Average difference in ${outcome} between each row and its nearest neighbour on the propensity score from the other arm.`
    case 'doubly-robust-run': return `Difference in ${outcome} between the arms, combining the propensity score with an outcome regression fitted in each arm.`
    case 'continuous-gps-run': return `Difference in ${outcome} per 1-unit increase in ${treatment}, each row weighted by the conditional density of the treatment it received.`
    case 'causal-effects-run': {
      const treatmentTime = run.configuration.treatmentLag === 0 ? 't' : `t−${run.configuration.treatmentLag}`
      return `Difference in expected ${outcome} at t after setting ${treatment} at ${treatmentTime} from ${run.evidence.interventions[0]} to ${run.evidence.interventions[1]}.`
    }
    default: return assertNever(run)
  }
}

/** The result headline may be narrower than the study's general estimand when a design has its own target. */
export function resultHeadline(run: EstimationRunArtifact, study: StudySpecification): string {
  if (run.kind === 'predictor-synthetic-control-run') return `Cumulative post-intervention gap in ${plainName(study.outcome.name)} for the treated unit`
  if (run.kind === 'panel-intervention-run') return `Average post-adoption difference in ${plainName(study.outcome.name)}`
  const treatment = plainName(study.treatment.name)
  const outcome = plainName(study.outcome.name)
  switch (study.estimand.kind) {
    case 'local-cutoff-effect': return `Local effect on ${outcome} at ${plainName(study.estimand.running.name)} = ${number(study.estimand.cutoff)}`
    case 'average-treatment-effect': return `Effect of changing ${treatment} on ${outcome}`
    case 'average-treatment-effect-on-treated': return `Effect of changing ${treatment} on ${outcome} among treated rows`
    case 'average-treatment-effect-on-controls': return `Average treatment effect on ${outcome} among control rows`
    case 'overlap-weighted-average-treatment-effect': return `Overlap-weighted average treatment effect on ${outcome}`
    case 'average-partial-effect': return `Average partial effect of ${treatment} on ${outcome}`
    case 'variance-weighted-average-partial-effect': return `Variance-weighted average partial effect of ${treatment} on ${outcome}`
    case 'conditional-partial-effect-per-row': return `Conditional partial effect of ${treatment} on ${outcome}`
    case 'conditional-average-treatment-effect': return `Effect of changing ${treatment} on ${outcome} within groups of ${plainName(study.estimand.modifier.name)}`
    case 'conditional-average-treatment-effect-per-row': return `Effect of changing ${treatment} on ${outcome} for each row`
    default: return assertNever(study.estimand)
  }
}

/** Describe the evidence shape without presenting repeated panel cells as independent observations. */
export function resultSampleLine(run: EstimationRunArtifact): string {
  if (run.kind === 'predictor-synthetic-control-run') return `One treated unit and ${run.evidence.donors.length} donor units; ${run.evidence.fitPeriods.length} fitting periods and ${run.estimate.effect.kind === 'path' ? run.estimate.effect.values.length : 0} selected post-intervention periods` 
  if (run.kind !== 'panel-intervention-run') return `n = ${formatCount(run.estimate.sample.observations).text}`
  if(run.evidence.kind==='staggeredDid') return `${run.evidence.units.length} retained units across ${run.evidence.times.length} periods; event-time support is reported separately`
  const treatedCells = run.evidence.treatedUnits * run.evidence.nPost
  return `${run.evidence.units.length} units × ${run.evidence.times.length} periods; the average covers ${treatedCells} treated-unit periods after adoption`
}

/**
 * Explain one estimator result on its actual scale. The sentences are deliberately method-specific:
 * a cointegrating vector is not described as an intervention effect, and a missing interval is never
 * converted into certainty.
 */
export function interpretEstimationResult(run: EstimationRunArtifact, study: StudySpecification, stepLabel: string): ResultInterpretation {
  const { estimate } = run
  switch (run.kind) {
    case 'sharp-rd-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `At the assignment cutoff, the estimated treatment effect on ${plainName(study.outcome.name)} is ${number(run.evidence.robust.value)}. The headline uses bias correction and its robust standard error.` },
      additiveIntervalForOutcome(estimate.interval, study.outcome.name),
      { kind: 'qualification', text: 'This is a local effect at the cutoff. A causal interpretation requires continuous potential-outcome means there, no precise manipulation of assignment, no other intervention at the same cutoff, and independent observations. The fitted jump alone does not establish those conditions.' },
    ] }
    case 'frontdoor-two-stage-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const mediator = plainName(run.columns[run.evidence.mediator]?.name ?? 'the mediator')
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `If ${plainName(study.treatment.name)} were set to ${number(run.evidence.treatmentValue)} rather than ${number(run.evidence.controlValue)}, expected ${plainName(study.outcome.name)} would be ${relativeLevel(effect)} on average. The estimate uses ${mediator} as the intermediate step through which ${plainName(study.treatment.name)} affects ${plainName(study.outcome.name)}.` },
        additiveIntervalForOutcome(estimate.interval, study.outcome.name),
        { kind: 'qualification', text: `This should be interpreted as the total effect of ${plainName(study.treatment.name)} only if ${mediator} carries all of its effect on ${plainName(study.outcome.name)}. Nothing unaccounted for may jointly influence ${plainName(study.treatment.name)} and ${mediator}, and—after accounting for ${plainName(study.treatment.name)}—nothing unaccounted for may jointly influence ${mediator} and ${plainName(study.outcome.name)}. The two straight-line relationships and the row resampling must also suit the data.` },
      ] }
    }
    case 'instrumental-variable-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const instruments = run.evidence.instruments.map((index) => run.columns[index]?.name ?? 'the instrument').join(', ')
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `With ${plainName(instruments)} as the source of treatment variation, a 1-unit increase in ${plainName(study.treatment.name)} changes expected ${plainName(study.outcome.name)} by ${number(effect)} in the fitted model.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, { kind: 'additive' }),
        { kind: 'qualification', text: `This can be interpreted as an effect only if ${plainName(instruments)} changes treatment, affects the outcome only through treatment, and is otherwise unrelated to causes of the outcome. The instrument must also provide enough treatment variation for a useful estimate, and the linear-effect model must fit.` },
      ] }
    }
    case 'backdoor-linear-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const opening = accountingOpening(run)
      const statements: NonEmptyArray<InterpretationStatement> = [
        { kind: 'magnitude', text: `${opening}, a 1-unit higher level of ${plainName(study.treatment.name)} is associated with ${change(effect, study.outcome.name)} on average ${targetPopulation(study)}.${absorbedSentence(run)}` },
        additiveIntervalForOutcome(estimate.interval, study.outcome.name),
        { kind: 'qualification', text: run.configuration.fixedEffects.kind !== 'none'
          ? 'Fixed effects account for additive differences in the selected groups. A causal interpretation requires the remaining treatment variation to be unrelated to unmeasured causes of the outcome, conditional on the specification. Fixed effects do not automatically remove time-varying confounding or bias from inappropriate adjustment. The linear model and a common treatment slope must also suit the question.'
          : 'This should be interpreted as a total effect only if the recorded adjustment variables account for the important common causes of treatment and outcome, while leaving out variables through which treatment works or variables that would create bias when controlled. Comparable treatment levels must exist among otherwise similar observations, and the straight-line model must suit the data.' },
      ]
      return { kind: 'result-interpretation', statements }
    }
    case 'propensity-weighting-run':
    case 'propensity-matching-run':
    case 'doubly-robust-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const opening = accountingOpening(run)
      const method = run.kind === 'propensity-weighting-run'
        ? 'The sample is reweighted by the inverse probability of treatment.'
        : run.kind === 'propensity-matching-run'
          ? 'Each row is paired with its nearest neighbour on the propensity score from the other arm, and the pairs are averaged over every row.'
          : 'The propensity score is combined with an outcome regression fitted in each arm, so only one of the two models has to be correct.'
      const stopped = run.kind === 'doubly-robust-run'
        ? (run.evidence.converged ? '' : ' The treatment model stopped before its own convergence rule was met, so read the estimate with that in mind.')
        : (treatmentModelStoppedEarly(run.evidence.treatmentModel) ? ' The treatment model stopped before its own convergence rule was met, so read the estimate with that in mind.' : '')
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${opening}, setting ${plainName(study.treatment.name)} to 1 rather than 0 is associated with ${change(effect, study.outcome.name)} on average ${targetPopulation(study)}. ${method}${stopped}` },
        additiveIntervalForOutcome(estimate.interval, study.outcome.name),
        { kind: 'qualification', text: 'This counts as a total effect only when the recorded adjustment variables include all important shared causes of treatment and outcome, and every row could have ended up in either group. If a fitted score is close to zero or one, that row has no match in the other group, so the estimate depends on a few rows with large weights.' },
      ] }
    }
    case 'continuous-gps-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const opening = accountingOpening(run)
      const stabilized = run.configuration.scale === 'stabilized'
        ? `Weights are stabilized by the marginal density of the treatment and sum to ${Math.round(run.evidence.weightSum)} across ${run.evidence.observations} rows.`
        : `Weights are the inverse density alone and sum to ${Math.round(run.evidence.weightSum)} across ${run.evidence.observations} rows. With a continuous treatment, stabilizing is necessary rather than optional.`
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${opening}, a 1-unit higher level of ${plainName(study.treatment.name)} is associated with ${change(effect, study.outcome.name)} on average ${targetPopulation(study)}. ${stabilized}` },
        additiveIntervalForOutcome(estimate.interval, study.outcome.name),
        { kind: 'qualification', text: 'The treatment is taken as normally distributed around its fitted value with constant variance, and the weighted model fits one slope. The analysis does not represent a skewed treatment or a curved dose response.' },
      ] }
    }
    case 'count-glm-run': {
      const ratio = estimate.effect.kind === 'expectedCountRatio' ? estimate.effect.value : Number.NaN
      const countAssumption = run.evidence.family === 'poisson'
        ? 'The Poisson model also requires the count variance to be close to its mean.'
        : 'The negative-binomial model allows extra count variation, but it does not account for serial dependence.'
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${ratioMeaning(ratio, study.outcome.name)} for each 1-unit increase in ${plainName(study.treatment.name)}, ${adjustedFor(run)}. The fitted expected-count ratio is ${formatEstimate(ratio, countRatioScale).text}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, { kind: 'count-ratio' }),
        { kind: 'qualification', text: `This can be interpreted as an effect only if the adjustment variables block the paths between treatment and outcome that are not causal, the outcome is a genuine count, the fitted relationship is adequate, and rows are independent. ${countAssumption} The ratio compares modelled expected counts, not observed totals.` },
      ] }
    }
    case 'negative-binomial-ingarch-run': {
      const effect = estimate.effect.kind === 'path' ? estimate.effect : null
      const schedule = (() => {
        switch (run.evidence.schedule.kind) {
          case 'point': return `were set to ${number(run.evidence.treatmentValue)} for the first forecast ${stepLabel} and then returned to the control path, instead of staying at ${number(run.evidence.controlValue)}`
          case 'persistent': return `stayed at ${number(run.evidence.treatmentValue)} for the next ${run.evidence.horizon} ${stepLabel}s instead of ${number(run.evidence.controlValue)}`
          case 'decaying': return `started at ${number(run.evidence.treatmentValue)} instead of ${number(run.evidence.controlValue)}, then the difference from the control path shrank by a factor of ${number(run.evidence.schedule.delta)} each ${stepLabel}`
          default: return assertNever(run.evidence.schedule)
        }
      })()
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `If ${plainName(study.treatment.name)} ${schedule}, the model predicts ${outcomeDifference(effect?.aggregate.average ?? Number.NaN, study.outcome.name)} per ${stepLabel} on average. Across the ${run.evidence.horizon} forecast ${stepLabel}s, the predicted differences add up to ${outcomeDifference(effect?.aggregate.cumulative ?? Number.NaN, study.outcome.name)}.` },
        noInterval('No confidence interval is available for these forecast differences.'),
        { kind: 'qualification', text: `The forecast assumes that the way recent ${plainName(study.outcome.name)} counts predict later counts remains stable. It should be interpreted as an intervention effect only if the treatment setting is the only systematic difference between the two forecast paths and the recorded adjustment accounts for common causes.` },
      ] }
    }
    case 'double-ml-run': {
      const contrast = run.evidence.model === 'irm' ? `Changing ${study.treatment.name} from 0 to 1` : `A 1-unit increase in ${study.treatment.name}`
      if (estimate.effect.kind === 'byGroup') {
        const { groups, modifier, overall } = estimate.effect
        const intervalLevel = formatPercent(groups[0].interval.level, { precision: 0 }).text
        return { kind: 'result-interpretation', statements: [
          { kind: 'magnitude', text: `${contrast} corresponds to ${groups.map((group) => `${change(group.value, study.outcome.name)} where ${plainName(modifier)} is ${group.label}`).join('; ')}. The average over the whole prepared population is ${number(overall)}.` },
          { kind: 'uncertainty', text: `Each group has its own ${intervalLevel} confidence interval. These ranges describe uncertainty around each group estimate; they do not directly test whether the groups differ.` },
          { kind: 'qualification', text: `These can be interpreted as group-specific effects only if the recorded adjustment variables account for the common causes and comparable treatment conditions exist within every group. Small groups give less certain estimates.` },
        ] }
      }
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${contrast} corresponds to ${change(effect, study.outcome.name)} ${targetPopulation(study)} after accounting for the recorded adjustment variables.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, { kind: 'additive' }),
        { kind: 'qualification', text: `This can be interpreted as ${run.evidence.att ? 'an effect among treated rows' : 'an effect over the prepared population'} only if the adjustment variables account for the common causes, comparable treatment conditions exist for similar rows, and rows are independent. Cross-fitting cannot correct for a missing common cause.` },
      ] }
    }
    case 'causal-forest-run': return { kind: 'result-interpretation', statements: [
      { kind: 'qualification', text: 'Conditional predictions describe average effects at the recorded covariate values, not observed individual treatment effects. Variation in fitted predictions alone does not establish treatment-effect heterogeneity.' },
      { kind: 'qualification', text: 'A causal interpretation requires the recorded identification assumptions and adequate treatment variation conditional on the covariates. Pointwise intervals do not provide simultaneous coverage of all predictions.' },
    ] }
    case 't-learner-run': {
      const effects = tLearnerRowEffects(run.evidence)
      const rows = effects === null ? null : summariseRowEffects(effects)
      const adjustment = adjustedFor(run)
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: rows === null
          ? `Setting ${study.treatment.name} from 0 to 1 changes expected ${study.outcome.name} by ${number(headlineValue(estimate.effect))} on average across rows, ${adjustment}.`
          : `Setting ${study.treatment.name} from 0 to 1 changes expected ${study.outcome.name} by ${number(run.evidence.average)} on average across the ${formatCount(rows.rows).text} rows, ${adjustment}. The row effects run from ${number(rows.minimum)} to ${number(rows.maximum)}; the middle half lies between ${number(rows.lowerQuartile)} and ${number(rows.upperQuartile)}, with a median of ${number(rows.median)}. Of the row effects, ${formatPercent(rows.positiveShare, { precision: 0 }).text} are above zero.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : { kind: 'uncertainty', text: `The ${formatPercent(estimate.interval.level, { precision: 0 }).text} confidence interval for the average effect is ${number(estimate.interval.lower)} to ${number(estimate.interval.upper)}. It uses a conservative uncertainty calculation based on refitting both forests on resampled rows. Individual row intervals appear in the table below.` },
        { kind: 'qualification', text: `Each row’s effect is the treated ${armModel(run.evidence)} prediction minus the control ${armModel(run.evidence)} prediction at that row’s values of the adjustment variables: the average contrast for rows like it, not that row’s own counterfactual. ${run.evidence.kind === 'crossFittedTLearner' ? 'Each row is predicted by the models fitted on the other half of the rows. ' : ''}The spread across rows shows variation in fitted predictions and can also contain fitting noise; it is not an uncertainty interval.` },
      ] }
    }
    case 'ardl-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      const reading = boundsReading(run.evidence)
      const magnitude = (() => {
        switch (reading) {
          case 'level-relation': return `The results support a stable long-run relationship between ${plainName(study.treatment.name)} and ${plainName(study.outcome.name)}. In that relationship, a 1-unit higher ${plainName(study.treatment.name)} is associated with ${change(effect, study.outcome.name)}.`
          case 'no-level-relation': return `The results do not support a stable long-run relationship between ${plainName(study.treatment.name)} and ${plainName(study.outcome.name)}. The fitted long-run coefficient is ${number(effect)}, but it should not be interpreted as an established long-run effect.`
          case 'inconclusive': return `The results do not settle whether ${plainName(study.treatment.name)} and ${plainName(study.outcome.name)} have a stable long-run relationship. The fitted long-run coefficient is ${number(effect)}, but the evidence is inconclusive.`
          default: return assertNever(reading)
        }
      })()
      const uncertainty = (() => {
        switch (estimate.interval.kind) {
          case 'none': return noInterval(estimate.interval.reason)
          case 'confidence': return {
            kind: 'uncertainty' as const,
            text: `The ${formatPercent(estimate.interval.level, { precision: 0 }).text} confidence interval for the fitted coefficient is ${number(estimate.interval.lower)} to ${number(estimate.interval.upper)}. This range assumes the selected lag and trend settings are fixed.`,
          }
          case 'credible': {
            const statement = intervalStatement(estimate.interval, { kind: 'additive' })
            return { ...statement, text: `${statement.text} This range assumes the selected lag and trend settings are fixed.` }
          }
          default: return assertNever(estimate.interval)
        }
      })()
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: magnitude },
        uncertainty,
        { kind: 'qualification', text: 'The method requires each series to be stable either as recorded or after taking one change, but not two. The selected lag pattern and trend must be appropriate, and the relationship must remain stable. It should be interpreted causally only if the study design separately establishes the direction and accounts for common causes.' },
      ] }
    }
    case 'vecm-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The series provide evidence of one long-run equilibrium relationship, even though each changes over time. In that relationship, ${plainName(study.treatment.name)} being 1 unit higher corresponds to ${plainName(study.outcome.name)} being ${relativeLevel(effect)}.` },
        noInterval('No confidence interval is available for this long-run coefficient.'),
        { kind: 'qualification', text: 'Each series must become stable after taking one change. The selected lag pattern, trend settings, and one-relationship structure must be appropriate, and the relationship must remain stable. This should be interpreted causally only if the study design separately establishes the direction and accounts for common causes.' },
      ] }
    }
    case 'predictor-synthetic-control-run': {
      const average = estimate.effect.kind === 'path' ? estimate.effect.aggregate.average : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Over the selected post-intervention periods, observed ${plainName(study.outcome.name)} averaged ${gap(average)} the synthetic-control outcome for the treated unit.` },
        noInterval('This predictor-based fit does not report an uncertainty interval.'),
        { kind: 'qualification', text: 'A causal interpretation requires the synthetic control to approximate the treated unit’s outcome without intervention. Inspect pre-intervention fit and predictor balance, and consider whether spillovers or other changes affect the comparison.' },
      ] }
    }
    case 'synthetic-control-run': {
      const effect = estimate.effect.kind === 'path' ? estimate.effect : null
      const average = effect?.aggregate.average ?? Number.NaN
      const cumulative = effect?.aggregate.cumulative ?? Number.NaN
      const crossFit = run.evidence.crossFit.kind === 'available'
        ? `The main estimate is ${directionalDifference(average)} per ${stepLabel}. It does not have a confidence interval. An uncertainty calculation gives an average estimate of ${directionalDifference(run.evidence.crossFit.att)} per ${stepLabel}, with a 95% confidence interval that ${directionalConfidenceRange(run.evidence.crossFit.confidenceInterval[0], run.evidence.crossFit.confidenceInterval[1], stepLabel)}. This range applies to the second average, not the main average or the cumulative difference.`
        : `The main estimate is ${directionalDifference(average)} per ${stepLabel}. It does not have a confidence interval. The additional uncertainty calculation was unavailable: ${run.evidence.crossFit.reason}`
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Across the ${run.evidence.nPost} ${stepLabel}s after the intervention, observed ${plainName(study.outcome.name)} averaged ${gap(average)} its estimated no-intervention outcome per ${stepLabel}. That is the average gap for one ${stepLabel}. Together, the ${stepLabel}-by-${stepLabel} gaps add up to ${directionalDifference(cumulative)} across the full period.` },
        { kind: 'uncertainty', text: crossFit },
        { kind: 'qualification', text: 'This can be interpreted as an intervention effect only if the weighted donors show what would have happened to the treated series without intervention, the donor relationship remains stable, and neither spillovers nor another change affects the comparison.' },
      ] }
    }
    case 'panel-intervention-run': {
      if(run.evidence.kind==='staggeredDid') return {kind:'result-interpretation',statements:[
        {kind:'magnitude',text:`The headline is the equal-weight average of supported post-adoption event-time effects. Each post-adoption effect compares the outcome in that period with ${staggeredBaseline(run.evidence.specification.anticipation)}, for the treated cohort against the comparison group. Group-time ATT, cohort averages and calendar averages are reported separately.`},
        estimate.interval.kind==='none'?noInterval(estimate.interval.reason):intervalStatement(estimate.interval,{kind:'additive'}),
        {kind:'qualification',text:'Interpretation requires parallel untreated trends for the selected comparison group, treatment overlap, no effects before the specified anticipation window, and no interference between units. Adjustment covariates must not be affected by treatment.'},
      ]}
      if (run.evidence.kind === 'panelAdjusted') return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `For the treated group, the estimated average effect on ${plainName(study.outcome.name)} is ${directionalDifference(run.evidence.estimate)} after adoption.` },
        adjustedDidUncertainty(run.evidence.specification, estimate.interval),
        { kind: 'qualification', text: run.evidence.specification.kind === 'regression' ? 'Interpretation requires parallel untreated trends, no anticipation or spillovers, and an appropriate regression specification.' : 'Interpretation requires conditional parallel trends, no anticipation or spillovers, treatment overlap and an adequate nuisance model. Cross-fitting does not establish these causal conditions. Covariates are measured before treatment.' },
      ] }
      const primary = run.evidence.kind === 'panelDid' ? run.evidence.did.estimate : run.evidence.syntheticDid.estimate
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `After adoption, ${plainName(study.outcome.name)} for the units that adopted averaged ${gap(primary)} the value estimated for them without adoption per ${stepLabel}.` },
        noInterval('No confidence interval is shown for this result.'),
        { kind: 'qualification', text: 'This can be interpreted as an adoption effect only if the comparison units show what would have happened to the treated units without adoption, no effect began early, and treatment of one unit did not affect another. The estimates do not establish those conditions.' },
      ] }
    }
    case 'negbin-nuts-run': {
      const ratio = estimate.effect.kind === 'expectedCountRatio' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${ratioMeaning(ratio, study.outcome.name)} for a 1-unit increase in ${plainName(study.treatment.name)}, after accounting for the recorded common cause. The posterior median expected-count ratio is ${formatEstimate(ratio, countRatioScale).text}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, { kind: 'count-ratio' }),
        { kind: 'qualification', text: 'This can be interpreted as an effect only if the recorded common cause is enough to account for the treatment–outcome relationship, the count model fits, rows are independent, and the sampler represented the posterior reliably. The interval describes posterior draws under the fitted model and priors; it does not test whether the causal assumptions are true.' },
      ] }
    }
    case 'bayesian-gaussian-run': {
      const effect = estimate.effect.kind === 'additive' ? estimate.effect.value : Number.NaN
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Setting ${plainName(study.treatment.name)} to 1 rather than 0 gives ${change(effect, study.outcome.name)} on average in the fitted model, ${adjustedFor(run)}.` },
        estimate.interval.kind === 'none' ? noInterval(estimate.interval.reason) : intervalStatement(estimate.interval, { kind: 'additive' }),
        { kind: 'uncertainty', text: `Of the retained posterior effect draws, ${formatPercent(run.evidence.probabilityPositive, { precision: 1 }).text} are above zero. This is a probability under the specified model and priors, not the probability that the causal assumptions are true.` },
        { kind: 'qualification', text: 'This can be interpreted as an effect only if the recorded adjustment variables account for the common causes, treatment is genuinely binary, the additive outcome model fits, rows are independent, and the priors suit the outcome scale.' },
      ] }
    }
    case 'discrete-bn-run': {
      const [low, high] = run.evidence.treatmentStates
      const [expectedLow, expectedHigh] = run.evidence.expectations
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `In the fitted network, setting ${plainName(study.treatment.name)} from its ${low} state to its ${high} state changes expected ${plainName(study.outcome.name)} from ${number(expectedLow)} to ${number(expectedHigh)}, a difference of ${number(run.evidence.effect)}.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No uncertainty interval is available.'),
        { kind: 'qualification', text: 'For a continuous treatment, the contrast is between quantile states rather than a 1-unit change on the original scale. Observed low-cardinality treatment states are preserved. The causal interpretation depends on the graph, adjustment and state preparation.' },
      ] }
    }
    case 'binary-ett-run': {
      const { evidence } = run
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Among rows that actually received treatment, expected ${plainName(study.outcome.name)} is ${number(evidence.treatedPotentialOutcomeMean)} under treatment and ${number(evidence.untreatedPotentialOutcomeMean)} under the estimated no-treatment alternative. Their difference is ${number(evidence.effectOnTreated)}.` },
        noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No uncertainty interval is available.'),
        { kind: 'qualification', text: 'This is the binary effect on the treated under the recorded graph and empirical joint distribution. It is not an ATE, and no continuous variable was discretised automatically.' },
      ] }
    }
    case 'causal-effects-run': {
      const [low, high] = run.evidence.interventions
      const effect = run.evidence.totalEffect ?? Number.NaN
      const model = (() => {
        switch (run.evidence.fit.kind) {
          case 'unfitted': return 'No model was fitted.'
          case 'invalidAdjustment': return 'No model was fitted because the supplied adjustment set did not pass the graph criterion.'
          case 'adjustedLinear': return 'The two settings are compared with a linear model for the outcome.'
          case 'adjustedKnn': return `The two settings are compared with a local model using ${run.evidence.fit.k} neighbouring observations.`
          case 'wrightParents': return `The fitted graph attributes ${number(run.evidence.fit.directEffect)} to direct paths and ${number(run.evidence.fit.indirectEffect)} to paths through other variables.`
          default: return assertNever(run.evidence.fit)
        }
      })()
      const timing = timeBeforeOutcome(run.configuration.treatmentLag, stepLabel)
      const adjustment = timeIndexedAdjustment(run, stepLabel)
      const uncertainty = estimate.interval.kind === 'none'
        ? noInterval(estimate.interval.reason)
        : intervalStatement(estimate.interval, { kind: 'additive' })
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `If ${plainName(study.treatment.name)} were set to ${number(high)} ${timing}, rather than ${number(low)}, estimated ${plainName(study.outcome.name)} would be ${gap(effect)} under the lower setting on average. ${adjustment}` },
        estimate.interval.kind === 'none'
          ? uncertainty
          : { ...uncertainty, text: `${uncertainty.text} It comes from repeatedly refitting blocks of adjacent periods. The calculation treats the recorded causes and their timing as fixed; it does not allow for uncertainty about whether the graph itself is right.` },
        { kind: 'qualification', text: `This has a causal interpretation only if the recorded temporal graph correctly represents the relevant causes and their timing, those relationships remain stable over the study period, and the fitted outcome relationship is suitable. ${model}` },
      ] }
    }
    case 'causal-impact-run': {
      const effect = estimate.effect.kind === 'path' ? estimate.effect : null
      const average = effect?.aggregate.average ?? Number.NaN
      const cumulative = effect?.aggregate.cumulative ?? Number.NaN
      const counterfactualAverage = run.evidence.counterfactual.reduce((sum, value) => sum + value, 0) / run.evidence.counterfactual.length
      const baseline = counterfactualAverage > 0
        ? ` The estimated no-intervention baseline averaged ${number(counterfactualAverage)} per ${stepLabel}, so the average difference is ${formatPercent(Math.abs(average / counterfactualAverage), { precision: 1 }).text} of that baseline.`
        : ''
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `After the intervention, observed ${plainName(study.outcome.name)} averaged ${gap(average)} its estimated no-intervention outcome per ${stepLabel}. Across ${run.evidence.nPost} ${stepLabel}s, the differences sum to ${number(cumulative)}.${baseline}` },
        run.evidence.kind !== 'causalImpact'
          ? { kind: 'qualification', text: `The 95% equal-tailed posterior interval for the average difference is ${number(run.evidence.averageSummary.absolute.lower)} to ${number(run.evidence.averageSummary.absolute.upper)}. For the cumulative difference it is ${number(run.evidence.cumulativeSummary.absolute.lower)} to ${number(run.evidence.cumulativeSummary.absolute.upper)}. These ranges are conditional on the model and priors; they do not account for an incorrect causal design.` }
          : noInterval(estimate.interval.kind === 'none' ? estimate.interval.reason : 'No interval for the aggregate effect is available.'),
        { kind: 'qualification', text: 'This can be interpreted as an intervention effect only if the selected control series show what would have happened to the outcome without intervention, their relationship with the outcome remains stable, the controls are not themselves affected, and no other outcome-specific change begins with the intervention.' },
      ] }
    }
    default: return assertNever(run)
  }
}

/** Discovery numbers describe fitted structural evidence, not identified intervention effects. */
export function interpretDiscoveryResult(run: DiscoveryRunArtifact): ResultInterpretation {
  switch (run.kind) {
    case 'pc-stable-run': {
      const connections = run.result.graph.flatMap((targets, source) => targets.filter((lags, target) => source < target && (lags[0]?.length ?? 0) > 0)).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The data retained ${connections} connections among the selected variables. Arrows show directions the method could settle; plain lines show directions that remain unresolved.` },
        { kind: 'qualification', text: 'The CPDAG relies on causal sufficiency, the Markov and faithfulness assumptions, the selected conditional-independence test, and any background constraints. It is structural evidence rather than an intervention-effect estimate.' },
      ] }
    }
    case 'fci-run': {
      const connections = run.result.graph.flatMap((targets, source) => targets.filter((lags, target) => source < target && (lags[0]?.length ?? 0) > 0)).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The data retained ${connections} connections among the selected variables. Arrowheads and tails show settled endpoint information; circles show what the method could not resolve.` },
        { kind: 'qualification', text: 'A PAG represents an equivalence class that can include latent confounding and selection. Circles are unresolved endpoints, and the graph must not be read as a fully directed causal DAG.' },
      ] }
    }
    case 'direct-lingam-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `The method ordered ${run.result.variables} variables and drew the linear connections that remained after pruning. A positive or negative weight shows the fitted direction and size of each relationship.` },
      { kind: 'qualification', text: 'The order and weights identify a causal structure only under the linear, acyclic, causally sufficient model with mutually independent non-Gaussian disturbances. They are not intervention-effect estimates.' },
    ] }
    case 'pcmci-plus-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: 'A displayed connection means the source and target remained related after the method accounted for selected other values. Its lag says how many time steps the source comes before the target.' },
      { kind: 'qualification', text: 'Same-period o–o endpoints remain unoriented. The resulting time-series CPDAG represents an equivalence class, not a completed causal DAG or an intervention-effect estimate.' },
    ] }
    case 'lpcmci-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: 'A displayed connection means the source and target remained related after the method accounted for selected other values. Its lag gives the time gap, while circles preserve directions the data could not settle.' },
      { kind: 'qualification', text: 'Circles preserve unresolved endpoints and bidirected marks permit latent confounding. The PAG is evidence to review, not a fully oriented causal DAG or an intervention-effect estimate.' },
    ] }
    case 'rpcmci-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `The method split ${run.result.observations} time points into ${run.result.numRegimes} recurring patterns and estimated a separate lag graph for each one. ${run.result.errorFreeAnnealings} of ${run.result.maxAnneal} searches completed without an optimisation error.` },
      { kind: 'qualification', text: 'The regime memberships and conditional-dependence graphs depend on the selected number of regimes, transition budget, linear partial-correlation test and assumption that each regime has a stationary causal structure. Regime labels have no ordering or substantive meaning by themselves.' },
    ] }
    case 'cdnots-run':
    case 'cdnots-plus-run': {
      const method = run.kind === 'cdnots-run' ? 'CD-NOTS' : 'CD-NOTS+'
      const context = run.result.contextVariables.length === 0
        ? 'No time-context node was included.'
        : `The graph includes ${run.result.contextVariables.join(' and ')} to represent changes over time.`
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${method} searched for same-period and delayed connections up to ${run.result.maxLag} time steps apart. ${context}` },
        { kind: 'qualification', text: `The endpoint marks represent the graph orientations supported by the ${method} rules and the selected partial-correlation test. Causal interpretation requires the method's Markov, faithfulness, causal-sufficiency and time-context assumptions; the marks are not intervention-effect estimates.` },
      ] }
    }
    case 'grace-run': {
      const selected = run.result.graph.flatMap((targets) => targets.flatMap((lags) => lags)).filter(Boolean).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${selected} candidate connections remained after the neural model refined the initial time-series graph. A larger gate value means the fitted model retained that connection more strongly.` },
        { kind: 'qualification', text: 'The initial skeleton and neural refinement depend on the selected lag window, conditional-independence test, nonstationarity context, regularisation and optimization. A retained gate is discovery evidence, not an intervention-effect estimate.' },
      ] }
    }
    case 'jpcmci-plus-run': {
      const links = run.result.graph.flatMap((targets) => targets.flatMap((lags) => lags)).filter((mark) => mark.length > 0).length
      const contexts = run.nodes.filter((node) => node.kind === 'generated').length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `The joint panel search found ${links} marked connections across ${run.result.datasets} aligned units and ${run.result.periods} periods. It retained ${contexts} generated context ${contexts === 1 ? 'variable' : 'variables'} for shared time or unit differences.` },
        { kind: 'qualification', text: 'J-PCMCI+ pools conditional-independence evidence under the declared system, time-context and unit-context roles. Its CPDAG is discovery evidence, not an intervention-effect estimate; unresolved endpoints and context relations must remain unresolved.' },
      ] }
    }
    case 'dynotears-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: 'Each displayed connection links an earlier or same-period source to a target. Its positive or negative weight gives the fitted direction and size of the linear relationship.' },
      { kind: 'qualification', text: 'DYNOTEARS supplies a sparse candidate structure under its model assumptions. The weights are not uncertainty intervals or identified intervention effects.' },
    ] }
    case 'var-lingam-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `The fitted graph uses up to ${run.result.selectedLag} previous time steps. Each weight shows the direction and size of a same-period or delayed linear relationship.` },
      { kind: 'qualification', text: 'The order and weights rely on linearity, non-Gaussian independent disturbances and the stated lag model. They remain structural evidence to review against domain knowledge.' },
    ] }
    case 'ocse-run': return { kind: 'result-interpretation', statements: [
      { kind: 'magnitude', text: `${run.result.edges.length} delayed connections remained after the method added useful predictors and removed redundant ones. The displayed score measures information left after accounting for the other selected histories.` },
      { kind: 'qualification', text: 'A selected relation is conditional-information evidence at the stated lag. It is not an estimate of the effect of intervening on the source.' },
    ] }
    case 'cmlp-run': {
      const selected = run.result.lagActive.flatMap((targets) => targets.flatMap((lags) => lags)).filter(Boolean).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${selected} delayed source-to-target connections remained after training. A larger score means that source and lag contributed more strongly to predicting the target in the fitted network.` },
        { kind: 'qualification', text: 'An active group means the source history helps predict the target within the fitted component-wise MLP. Selection depends on the lag window, network, structured penalty, initialization and optimization; it is not an intervention-effect estimate.' },
      ] }
    }
    case 'clstm-run': {
      const selected = run.result.summaryActive.flatMap((targets) => targets).filter(Boolean).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `${selected} source-to-target history connections remained after training. A larger score means the source history contributed more strongly to predicting the target in the fitted network.` },
        { kind: 'qualification', text: 'An active group means the source history helps predict the target within the fitted recurrent model. cLSTM does not identify an individual lag, and the relation is not an intervention-effect estimate.' },
      ] }
    }
    default: return assertNever(run)
  }
}

/** Structural counterfactuals require a stronger reading than an average-effect estimate. */
export function interpretCounterfactualResult(run: CounterfactualRunArtifact, study: StudySpecification, stepLabel: string): ResultInterpretation {
  switch (run.kind) {
    case 'linear-scm-run': {
      const { evidence } = run
      const positive = formatPercent(evidence.sharePositive, { precision: 1 }).text
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `For each of the ${evidence.observations} ${stepLabel}s, Hirmos replays the fitted row with ${plainName(study.treatment.name)} set to ${number(evidence.interventions[1])} and to ${number(evidence.interventions[0])}, while keeping that row’s other recovered conditions fixed. The average difference is ${number(evidence.averageEffect)} in ${plainName(study.outcome.name)}; ${positive} of the fitted row differences are positive.` },
        { kind: 'uncertainty', text: evidence.observationNoise === null
          ? 'Exact disturbance-term abduction reproduces each fitted row but does not provide a sampling or posterior uncertainty interval.'
          : `The observation-noise scale is ${number(evidence.observationNoise)}. It changes disturbance-term abduction but is not a reported interval for the individual effects.` },
        { kind: 'qualification', text: 'Each pair is a modelled replay of one observed row; only one of the two outcomes was observed. The comparison has a counterfactual meaning only if the recorded graph and linear equations are suitable, the relevant causes are measured, and changing treatment would leave the other causal mechanisms unchanged.' },
      ] }
    }
    case 'dynamic-linear-scm-run': {
      const { evidence } = run
      const schedule = evidence.timing.kind === 'point'
        ? `once in ${stepLabel} ${evidence.timing.time + 1}`
        : `from ${stepLabel} ${evidence.timing.start + 1} onward`
      const uncertainty = (() => {
        switch (evidence.uncertainty.kind) {
          case 'none': return 'No sampling interval was requested; the path and horizon summaries are point estimates conditional on the fitted equations.'
          case 'blockBootstrap': {
            const level = Math.round(evidence.uncertainty.confidenceLevel * 100)
            return `${level}% block-bootstrap confidence intervals are [${number(evidence.uncertainty.averageInterval[0])}, ${number(evidence.uncertainty.averageInterval[1])}] for the average horizon contrast and [${number(evidence.uncertainty.cumulativeInterval[0])}, ${number(evidence.uncertainty.cumulativeInterval[1])}] for the cumulative contrast. The path band is pointwise, not a simultaneous band for the entire path.`
          }
          default: return assertNever(evidence.uncertainty)
        }
      })()
      return { kind: 'result-interpretation', statements: [
        { kind: 'magnitude', text: `Hirmos replays ${evidence.effects.length} observed ${stepLabel}s with ${plainName(study.treatment.name)} set to ${number(evidence.interventions[1])} rather than ${number(evidence.interventions[0])} ${schedule}. With each period’s recovered unexplained shock held fixed, modelled ${plainName(study.outcome.name)} differs by ${number(evidence.averageEffect)} per ${stepLabel} on average, and the period-by-period differences sum to ${number(evidence.cumulativeEffect)}.` },
        { kind: 'uncertainty', text: `${uncertainty} Resampling represents coefficient-estimation uncertainty under the recorded graph and stationary block-bootstrap assumptions; it does not cover graph choice, preprocessing, an unsuitable model, or uncertainty about the intervention schedule.` },
        { kind: 'qualification', text: 'This is a retrospective replay of observed periods, not a forecast of new periods. It has a counterfactual meaning only if the recorded lagged graph and linear equations are suitable and changing treatment would leave every other equation and recovered period-specific shock unchanged.' },
      ] }
    }
    default: return assertNever(run)
  }
}

/** Sensitivity procedures answer different questions; none is a second identification analysis. */
export function interpretSensitivityResult(run: SensitivityRunArtifact): ResultInterpretation {
  switch (run.kind) {
    case 'linear-refutation-run': {
      const placeboDistance = Math.abs(run.evidence.placeboEffect)
      const subsetMovement = Math.abs(run.evidence.subsetEffect - run.evidence.estimate)
      const randomMovement = Math.abs(run.evidence.randomCommonCauseEffect - run.evidence.estimate)
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `After the treatment values were shuffled, the estimated effect was ${number(run.evidence.placeboEffect)}, which is ${number(placeboDistance)} from zero. A useful placebo result is close to zero.` },
        { kind: 'comparison', text: `Refitting on ${formatPercent(run.evidence.subsetFraction, { precision: 0 }).text} of the rows changed the estimate by ${number(subsetMovement)}. A result that depends heavily on which rows are retained would move much more.` },
        { kind: 'comparison', text: `Adding an independent noise variable changed the estimate by ${number(randomMovement)}. A variable generated at random should have little effect on the answer.` },
        { kind: 'qualification', text: 'These checks test the estimate against three specific disturbances. Passing them does not validate the graph or rule out an unmeasured common cause.' },
      ] }
    }
    case 'dml-refutation-run': {
      const alpha = 0.05
      const placeboDifferent = run.evidence.placebo.pValue < alpha
      const randomDifferent = run.evidence.randomCommonCause.pValue < alpha
      const randomMovement = run.evidence.randomCommonCause.refutedEffect - run.evidence.randomCommonCause.originalEffect
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `Shuffling the treatment produced an average placebo estimate of ${number(run.evidence.placebo.refutedEffect)} across eight lighter refits. Those estimates ${placeboDifferent ? 'differ from zero' : 'are not distinguishable from zero'} at the 5% level (p = ${number(run.evidence.placebo.pValue)}); a useful placebo result is close to zero.` },
        { kind: 'comparison', text: `Adding independent noise variables changed the lighter-fit estimate by ${number(randomMovement)} on average across six refits. The shifts ${randomDifferent ? 'differ from zero' : 'are not distinguishable from zero'} at the 5% level (p = ${number(run.evidence.randomCommonCause.pValue)}).` },
        { kind: 'magnitude', text: `Under the equal-strength confounding model, an unmeasured common cause would need to explain ${formatPercent(run.evidence.sensitivity.robustnessValue, { precision: 1 }).text} of the remaining variation in both treatment and outcome to move the estimate to zero. It would need ${formatPercent(run.evidence.sensitivity.robustnessValueCi, { precision: 1 }).text} to make the reported interval reach zero.` },
        { kind: 'qualification', text: 'The p-values describe the simulated refits. They do not test whether the causal graph is true. The confounding percentages apply to the equal-strength model shown here and do not cover every possible missing variable.' },
      ] }
    }
    case 'unobserved-confounding-run': {
      const effects = run.evidence.effects.flat()
      const flips = effects.filter((value) => Math.sign(value) !== Math.sign(run.evidence.originalEffect)).length
      return { kind: 'result-interpretation', statements: [
        { kind: 'comparison', text: `Across the ${run.evidence.kappaT.length} × ${run.evidence.kappaY.length} simulated scenarios, the refitted effect ranges from ${number(Math.min(...effects))} to ${number(Math.max(...effects))}. ${flips} of ${effects.length} scenarios reverse its sign.` },
        { kind: 'qualification', text: 'Each scenario gives an unmeasured variable a chosen influence on treatment assignment and outcome, then refits the estimate. The grid does not say how likely any scenario is, and it says nothing about stronger or differently shaped confounding outside the values tested.' },
      ] }
    }
    default: return assertNever(run)
  }
}
