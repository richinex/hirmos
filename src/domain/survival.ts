import { z } from 'zod'
import type { ColumnSelection, NumericColumnSelection } from './dataset'
import {
  assertNever,
  brand,
  err,
  ok,
  type Brand,
  type NonEmptyArray,
  type Result,
} from './dop'
import type { PreparedDatasetVersionId } from './preprocessing'

export type SurvivalRunId = Brand<string, 'SurvivalRunId'>
export type ConversionRate = Brand<number, 'ConversionRate'>
export type ConversionDifference = Brand<number, 'ConversionDifference'>

export const newSurvivalRunId = (): SurvivalRunId =>
  brand<string, 'SurvivalRunId'>(crypto.randomUUID())

export type ProportionalHazardsFamily = 'exponential' | 'weibullPh' | 'gompertz'

export type ParametricSurvivalFamily =
  | ProportionalHazardsFamily
  | 'weibull'
  | 'logNormal'
  | 'gamma'
  | 'logLogistic'
  | 'generalizedGamma'
  | 'generalizedF'

export const parametricSurvivalFamilySchema = z.enum([
  'exponential',
  'weibull',
  'weibullPh',
  'logNormal',
  'gamma',
  'gompertz',
  'logLogistic',
  'generalizedGamma',
  'generalizedF',
])

export const proportionalHazardsFamilySchema = z.enum([
  'exponential',
  'weibullPh',
  'gompertz',
])

export type SurvivalRowFrequency =
  | { readonly kind: 'one-observation-per-row' }
  | { readonly kind: 'frequency-column'; readonly column: NumericColumnSelection }

export interface RightCensoredSurvivalConfiguration<
  Family extends ParametricSurvivalFamily = ParametricSurvivalFamily,
> {
  readonly kind: 'right-censored-parametric'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly rowFrequency: SurvivalRowFrequency
  readonly covariates: readonly NumericColumnSelection[]
  readonly family: Family
  readonly predictionTimes: NonEmptyArray<number>
}

export interface StartStopSurvivalConfiguration<
  Family extends ProportionalHazardsFamily = ProportionalHazardsFamily,
> {
  readonly kind: 'start-stop-proportional-hazards'
  readonly start: NumericColumnSelection
  readonly stop: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly rowFrequency: SurvivalRowFrequency
  readonly covariates: readonly NumericColumnSelection[]
  readonly family: Family
  readonly predictionTimes: NonEmptyArray<number>
}

export type CoxStandardErrors =
  | { readonly kind: 'model-based' }
  | { readonly kind: 'robust' }
  | { readonly kind: 'clustered'; readonly cluster: NumericColumnSelection }

export type CoxPenalty =
  | { readonly kind: 'none' }
  | {
      readonly kind: 'uniform'
      readonly strength: number
      readonly l1Ratio: number
    }

export type CoxTies = 'efron' | 'breslow'

/** A shared gamma frailty on a group column, as survival's `frailty(group, distribution = "gamma")`. */
export type CoxFrailty =
  | { readonly kind: 'none' }
  | { readonly kind: 'gamma'; readonly group: NumericColumnSelection; readonly ties: CoxTies }

export type CoxObservationConfiguration =
  | {
      readonly kind: 'right-censored'
      readonly duration: NumericColumnSelection
      readonly event: NumericColumnSelection
      readonly entry:
        | { readonly kind: 'not-used' }
        | { readonly kind: 'column'; readonly column: NumericColumnSelection }
      readonly standardErrors: CoxStandardErrors
      readonly frailty: CoxFrailty
    }
  | {
      readonly kind: 'start-stop'
      readonly subject: NumericColumnSelection
      readonly start: NumericColumnSelection
      readonly stop: NumericColumnSelection
      readonly event: NumericColumnSelection
      readonly standardErrors: { readonly kind: 'model-based' }
    }

export interface CoxRegressionConfiguration {
  readonly kind: 'cox-regression'
  readonly observation: CoxObservationConfiguration
  readonly weights:
    | { readonly kind: 'equal' }
    | { readonly kind: 'column'; readonly column: NumericColumnSelection }
  readonly strata:
    | { readonly kind: 'unstratified' }
    | { readonly kind: 'column'; readonly column: NumericColumnSelection }
  readonly covariates: NonEmptyArray<NumericColumnSelection>
  readonly penalty: CoxPenalty
  readonly confidenceLevel: number
}

export type PenalizedAftFamily = 'weibull' | 'logLogistic'

/**
 * lifelines' `WeibullAFTFitter` or `LogLogisticAFTFitter` with an L2 penalty: the covariates enter the
 * location parameter with an intercept, and the ancillary parameter is an intercept alone.
 */
export interface PenalizedAftConfiguration {
  readonly kind: 'penalized-aft'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly covariates: NonEmptyArray<NumericColumnSelection>
  readonly family: PenalizedAftFamily
  readonly penalizer: number
  readonly confidenceLevel: number
  readonly predictionTimes: NonEmptyArray<number>
}

export interface NonparametricSurvivalConfiguration {
  readonly kind: 'right-censored-nonparametric'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly rowFrequency: SurvivalRowFrequency
  readonly predictionTimes: NonEmptyArray<number>
  readonly ties: 'discrete' | 'smoothed'
}

export interface TwoGroupSurvivalConfiguration {
  readonly kind: 'two-group-comparison'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly group: NumericColumnSelection
  readonly truncationTime: number
  readonly permutations: number
  readonly seed: number
}

export type MultiStateInputConfiguration =
  | {
      readonly kind: 'prepared-transition-rows'
      readonly start: NumericColumnSelection
      readonly stop: NumericColumnSelection
      readonly event: NumericColumnSelection
      readonly from: NumericColumnSelection
      readonly to: NumericColumnSelection
    }
  | {
      readonly kind: 'longitudinal-state-observations'
      readonly subject: ColumnSelection
      readonly time: NumericColumnSelection
      readonly state: NumericColumnSelection
      readonly stateCount: number
      readonly transitions: NonEmptyArray<readonly [number, number]>
    }
  | {
      readonly kind: 'wide-state-events'
      readonly states: NonEmptyArray<
        | { readonly kind: 'not-applicable' }
        | { readonly kind: 'recorded'; readonly time: NumericColumnSelection; readonly status: NumericColumnSelection }
      >
      readonly transitions: NonEmptyArray<readonly [number, number]>
      readonly entry:
        | { readonly kind: 'shared'; readonly state: number; readonly time: number }
        | { readonly kind: 'columns'; readonly state: NumericColumnSelection; readonly time: NumericColumnSelection }
    }

export interface MultiStateSurvivalConfiguration {
  readonly kind: 'multi-state-proportional-hazards'
  readonly input: MultiStateInputConfiguration
  readonly family: ProportionalHazardsFamily
  readonly predictionTimes: NonEmptyArray<number>
}

export type SurvivalConfiguration =
  | RightCensoredSurvivalConfiguration
  | PenalizedAftConfiguration
  | NonparametricSurvivalConfiguration
  | StartStopSurvivalConfiguration
  | CoxRegressionConfiguration
  | TwoGroupSurvivalConfiguration
  | MultiStateSurvivalConfiguration

const finiteNumber = z.number().finite()
const nonNegativeTime = finiteNumber.nonnegative()
const probability = finiteNumber.min(0).max(1)
const intervalSchema = z.tuple([finiteNumber, finiteNumber])
const optionalIntervalSchema = z.union([intervalSchema, z.null()])

export const flexSurvEvidenceSchema = z.object({
  kind: z.literal('flexSurv'),
  observations: z.number().int().positive(),
  events: z.number().int().nonnegative(),
  family: parametricSurvivalFamilySchema,
  naturalBaseline: z.array(finiteNumber).min(1),
  coefficients: z.array(finiteNumber),
  parameterIntervals: z.array(optionalIntervalSchema),
  logLikelihood: finiteNumber,
  aic: finiteNumber,
  bic: finiteNumber,
  profile: z.array(finiteNumber),
  predictionTimes: z.array(nonNegativeTime).min(1),
  survival: z.array(probability).min(1),
  // A family whose hazard rises without bound as time approaches zero reports no value there.
  hazard: z.array(z.union([finiteNumber.nonnegative(), z.null()])).min(1),
  median: nonNegativeTime,
  mean: z.union([nonNegativeTime, z.null()]),
}).strict()

type ParsedFlexSurvEvidence = z.infer<typeof flexSurvEvidenceSchema>

export type FlexSurvEvidence<
  Family extends ParametricSurvivalFamily = ParametricSurvivalFamily,
> = Omit<ParsedFlexSurvEvidence, 'family'> & { readonly family: Family }

export const nonparametricSurvivalEvidenceSchema = z.object({
  kind: z.literal('nonparametricSurvival'),
  observations: z.number().int().positive(),
  events: z.number().int().nonnegative(),
  predictionTimes: z.array(nonNegativeTime).min(1),
  survival: z.array(probability).min(1),
  survivalLower: z.array(probability).min(1),
  survivalUpper: z.array(probability).min(1),
  cumulativeDensity: z.array(probability).min(1),
  cumulativeHazard: z.array(finiteNumber.nonnegative()).min(1),
  cumulativeHazardLower: z.array(finiteNumber.nonnegative()).min(1),
  cumulativeHazardUpper: z.array(finiteNumber.nonnegative()).min(1),
  hazardIncrement: z.array(finiteNumber.nonnegative()).min(1),
}).strict()

export type NonparametricSurvivalEvidence = z.infer<typeof nonparametricSurvivalEvidenceSchema>

const coxCoefficientEstimateSchema = z.object({
  coefficient: finiteNumber,
  hazardRatio: finiteNumber.positive(),
  standardError: finiteNumber.positive(),
  coefficientInterval: intervalSchema,
  hazardRatioInterval: z.tuple([finiteNumber.positive(), finiteNumber.positive()]),
  z: finiteNumber,
  pValue: probability,
}).strict()

const coxBaselinePointSchema = z.object({
  time: nonNegativeTime,
  hazard: finiteNumber.nonnegative(),
  cumulativeHazard: finiteNumber.nonnegative(),
  survival: probability,
}).strict()

type CoxBaselinePoint = z.infer<typeof coxBaselinePointSchema>

const coxBaselineCurveSchema = z.array(coxBaselinePointSchema).min(1)

const coxBaselineSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('shared'),
    estimates: coxBaselineCurveSchema,
  }).strict(),
  z.object({
    kind: z.literal('stratified'),
    curves: z.array(z.object({
      stratum: z.number().int().nonnegative(),
      estimates: coxBaselineCurveSchema,
    }).strict()).min(1),
  }).strict(),
])

const coxConcordanceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('recorded'), result: probability }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
])

const coxProportionalHazardsTestSchema = z.object({
  statistic: finiteNumber.nonnegative(),
  pValue: probability,
}).strict()

const coxProportionalHazardsTestsSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('recorded'),
    transforms: z.array(z.object({
      transform: z.enum(['eventRank', 'kaplanMeier', 'identity', 'logTime']),
      tests: z.array(coxProportionalHazardsTestSchema).min(1),
    }).strict()).min(1),
  }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
])

const coxFrailtyEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('gamma'),
    groups: z.number().int().min(2),
    ties: z.enum(['efron', 'breslow']),
    theta: finiteNumber.nonnegative(),
    termTest: z.object({ statistic: finiteNumber.nonnegative(), df: finiteNumber, pValue: probability }).strict(),
    degreesOfFreedom: finiteNumber.positive(),
    outerIterations: z.number().int().positive(),
    innerIterations: z.number().int().positive(),
    history: z.array(z.tuple([finiteNumber.nonnegative(), finiteNumber, finiteNumber])).min(1),
    standardErrors2: z.array(finiteNumber.positive()).min(1),
  }).strict(),
])

export type CoxFrailtyEvidence = z.infer<typeof coxFrailtyEvidenceSchema>

export const coxRegressionEvidenceSchema = z.object({
  kind: z.literal('coxRegression'),
  observation: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('rightCensored'), delayedEntry: z.boolean() }).strict(),
    z.object({ kind: z.literal('startStop'), subjects: z.number().int().positive() }).strict(),
  ]),
  observations: z.number().int().positive(),
  events: z.number().int().positive(),
  totalWeight: finiteNumber.positive(),
  standardErrors: z.enum(['modelBased', 'robust', 'clustered']),
  coefficients: z.array(coxCoefficientEstimateSchema).min(1),
  covariance: z.array(finiteNumber).min(1),
  logLikelihood: finiteNumber,
  nullLogLikelihood: finiteNumber,
  likelihoodRatio: finiteNumber.nonnegative(),
  likelihoodRatioPValue: probability,
  partialAic: finiteNumber,
  iterations: z.number().int().positive(),
  covariateMeans: z.array(finiteNumber).min(1),
  covariateStandardDeviations: z.array(finiteNumber.positive()).min(1),
  concordance: coxConcordanceSchema,
  proportionalHazardsTests: coxProportionalHazardsTestsSchema,
  baseline: coxBaselineSchema,
  frailty: coxFrailtyEvidenceSchema.default({ kind: 'none' }),
}).strict()

export type CoxRegressionEvidence = z.infer<typeof coxRegressionEvidenceSchema>

const aftCoefficientEstimateSchema = z.object({
  coefficient: finiteNumber,
  timeRatio: finiteNumber.positive(),
  standardError: finiteNumber.positive(),
  coefficientInterval: intervalSchema,
  timeRatioInterval: z.tuple([finiteNumber.positive(), finiteNumber.positive()]),
  z: finiteNumber,
  pValue: probability,
}).strict()

export const penalizedAftEvidenceSchema = z.object({
  kind: z.literal('penalizedAft'),
  observations: z.number().int().positive(),
  events: z.number().int().positive(),
  family: z.enum(['weibull', 'logLogistic']),
  penalizer: finiteNumber.nonnegative(),
  coefficients: z.array(aftCoefficientEstimateSchema).min(1),
  intercept: aftCoefficientEstimateSchema,
  ancillary: aftCoefficientEstimateSchema,
  covariance: z.array(finiteNumber).min(1),
  logLikelihood: finiteNumber,
  aic: finiteNumber,
  bic: finiteNumber,
  iterations: z.number().int().positive(),
  concordance: coxConcordanceSchema,
  covariateMeans: z.array(finiteNumber).min(1),
  predictionTimes: z.array(nonNegativeTime).min(1),
  survival: z.array(probability).min(1),
  median: nonNegativeTime,
}).strict()

export type PenalizedAftEvidence = z.infer<typeof penalizedAftEvidenceSchema>

const survivalCurvePointSchema = z.tuple([nonNegativeTime, probability])
const nonNegativeCurvePointSchema = z.tuple([nonNegativeTime, finiteNumber.nonnegative()])
const atRiskPointSchema = z.tuple([nonNegativeTime, z.number().int().nonnegative()])

const groupSurvivalDiagnosticsSchema = z.object({
  cumulativeHazard: z.array(nonNegativeCurvePointSchema).min(1),
  smoothedHazard: z.array(nonNegativeCurvePointSchema).min(1),
  atRisk: z.array(atRiskPointSchema).min(1),
  censorTimes: z.array(nonNegativeTime),
  restrictedMean: finiteNumber.nonnegative(),
}).strict()

const comparisonSurvivalDiagnosticsSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRecorded') }).strict(),
  z.object({
    kind: z.literal('recorded'),
    groupZero: groupSurvivalDiagnosticsSchema,
    groupOne: groupSurvivalDiagnosticsSchema,
    crossingTimes: z.array(nonNegativeTime),
  }).strict(),
])

const unavailableSummarySchema = z.object({
  kind: z.literal('unavailable'),
  reason: z.string().min(1),
}).strict()

const notRecordedSummarySchema = z.object({ kind: z.literal('notRecorded') }).strict()
const conversionRateSchema = probability.transform((value) => brand<number, 'ConversionRate'>(value))
const conversionDifferenceSchema = finiteNumber.min(-1).max(1).transform((value) => brand<number, 'ConversionDifference'>(value))

const observedConversionResultSchema = z.object({
  groupZeroRate: conversionRateSchema,
  groupOneRate: conversionRateSchema,
  difference: conversionDifferenceSchema,
  standardError: finiteNumber.positive(),
  statistic: finiteNumber.nonnegative(),
  pValue: probability,
}).strict()

const observedConversionSummarySchema = z.discriminatedUnion('kind', [
  notRecordedSummarySchema,
  unavailableSummarySchema,
  z.object({ kind: z.literal('recorded'), result: observedConversionResultSchema }).strict(),
])

const fixedPointScaleSchema = z.enum([
  'naive',
  'log',
  'complementaryLogLog',
  'arcsineSquareRoot',
  'logit',
])

// Only the transformed scales keep their bounds inside the unit interval; the untransformed
// interval is the estimate plus or minus 1.96 standard errors, and the reference reports it as it
// falls, so a bound below 0 or above 1 is a result to show, not a malformed one.
const fixedPointTestSchema = z.object({
  scale: fixedPointScaleSchema,
  groupZeroInterval: z.tuple([finiteNumber, finiteNumber]),
  groupOneInterval: z.tuple([finiteNumber, finiteNumber]),
  statistic: finiteNumber.nonnegative(),
  pValue: probability,
}).strict()

const fixedTimeConversionResultSchema = observedConversionResultSchema.extend({
  time: nonNegativeTime,
  interval: z.tuple([conversionDifferenceSchema, conversionDifferenceSchema]),
  scaleTests: z.array(fixedPointTestSchema).length(5),
}).strict()

const fixedTimeConversionSummarySchema = z.discriminatedUnion('kind', [
  notRecordedSummarySchema,
  unavailableSummarySchema,
  z.object({ kind: z.literal('recorded'), result: fixedTimeConversionResultSchema }).strict(),
])

const gRhoResultSchema = z.object({
  rho: finiteNumber.nonnegative(),
  observed: z.tuple([finiteNumber.nonnegative(), finiteNumber.nonnegative()]),
  expected: z.tuple([finiteNumber.nonnegative(), finiteNumber.nonnegative()]),
  variance: z.tuple([
    z.tuple([finiteNumber, finiteNumber]),
    z.tuple([finiteNumber, finiteNumber]),
  ]),
  statistic: finiteNumber.nonnegative(),
  pValue: probability,
}).strict()

const gRhoSummarySchema = z.discriminatedUnion('kind', [
  notRecordedSummarySchema,
  unavailableSummarySchema,
  z.object({ kind: z.literal('recorded'), result: gRhoResultSchema }).strict(),
])

export const comparisonSurvivalEvidenceSchema = z.object({
  kind: z.literal('comparisonSurvival'),
  observations: z.number().int().positive(),
  truncationTime: finiteNumber.positive(),
  groupZeroCurve: z.array(survivalCurvePointSchema).min(1),
  groupOneCurve: z.array(survivalCurvePointSchema).min(1),
  diagnostics: comparisonSurvivalDiagnosticsSchema,
  observedConversion: observedConversionSummarySchema,
  fixedTimeConversion: fixedTimeConversionSummarySchema,
  petoPeto: gRhoSummarySchema,
  proportionalHazardsPValue: probability,
  logRankPValue: probability,
  gehanWilcoxonPValue: probability,
  taroneWarePValue: probability,
  weightedKaplanMeierPValue: probability,
  absoluteDifferencePValue: probability,
  twoStagePValue: probability,
  squaredDifferencePValue: probability,
  restrictedMeanDifference: finiteNumber,
  restrictedMeanInterval: intervalSchema,
}).strict()

export type ComparisonSurvivalEvidence = z.infer<typeof comparisonSurvivalEvidenceSchema>

const stateCode = finiteNumber.refine(Number.isInteger, 'State codes must be integers.')

export const multiStateSurvivalEvidenceSchema = z.object({
  kind: z.literal('multiStateSurvival'),
  observations: z.number().int().positive(),
  states: z.array(stateCode).min(2),
  transitions: z.array(z.tuple([
    z.number().int().nonnegative(),
    z.number().int().nonnegative(),
  ])).min(1),
  family: proportionalHazardsFamilySchema,
  preparation: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('preparedRows') }).strict(),
    z.object({ kind: z.literal('longitudinalStates'), sourceRows: z.number().int().positive(), transitionRows: z.number().int().positive(), notices: z.array(z.string()) }).strict(),
    z.object({ kind: z.literal('wideEvents'), sourceRows: z.number().int().positive(), transitionRows: z.number().int().positive(), notices: z.array(z.string()) }).strict(),
  ]),
  predictionTimes: z.array(nonNegativeTime).min(1),
  probabilities: z.array(z.array(finiteNumber)).min(1),
}).strict()

export type MultiStateSurvivalEvidence = z.infer<typeof multiStateSurvivalEvidenceSchema>

export type SurvivalEvidence =
  | FlexSurvEvidence
  | PenalizedAftEvidence
  | NonparametricSurvivalEvidence
  | CoxRegressionEvidence
  | ComparisonSurvivalEvidence
  | MultiStateSurvivalEvidence

export type SurvivalEvidenceProblem = {
  readonly kind: 'invalid-survival-evidence'
  readonly detail: string
}

const invalidEvidence = (detail: string): Result<never, SurvivalEvidenceProblem> =>
  err({ kind: 'invalid-survival-evidence', detail })

function baselineParameterCount(family: ParametricSurvivalFamily): number {
  switch (family) {
    case 'exponential': return 1
    case 'weibull':
    case 'weibullPh':
    case 'logNormal':
    case 'gamma':
    case 'gompertz':
    case 'logLogistic': return 2
    case 'generalizedGamma': return 3
    case 'generalizedF': return 4
    default: return assertNever(family)
  }
}

function baselineParametersAreValid(
  family: ParametricSurvivalFamily,
  values: readonly number[],
): boolean {
  switch (family) {
    case 'exponential': return values[0]! > 0
    case 'weibull':
    case 'weibullPh':
    case 'gamma':
    case 'logLogistic': return values.every((value) => value > 0)
    case 'logNormal': return values[1]! > 0
    case 'gompertz': return values[1]! > 0
    case 'generalizedGamma': return values[1]! > 0
    case 'generalizedF': return values[1]! > 0 && values[3]! > 0
    default: return assertNever(family)
  }
}

function intervalsAreOrdered(
  intervals: readonly (readonly [number, number] | null)[],
): boolean {
  return intervals.every((interval) => interval === null || interval[0] <= interval[1])
}

function curveIsOrdered(curve: readonly (readonly [number, number])[]): boolean {
  return curve.every((point, index) => {
    if (index === 0) return true
    const previous = curve[index - 1]!
    return previous[0] <= point[0] && previous[1] >= point[1]
  })
}

function valuesMoveForward(
  points: readonly (readonly [number, number])[],
  direction: 'any' | 'nondecreasing' | 'nonincreasing',
): boolean {
  return points.every((point, index) => {
    if (index === 0) return true
    const previous = points[index - 1]!
    if (previous[0] > point[0]) return false
    switch (direction) {
      case 'any': return true
      case 'nondecreasing': return previous[1] <= point[1]
      case 'nonincreasing': return previous[1] >= point[1]
      default: return assertNever(direction)
    }
  })
}

function comparisonDiagnosticsProblem(
  diagnostics: ComparisonSurvivalEvidence['diagnostics'],
  restrictedMeanDifference: number,
): string | null {
  switch (diagnostics.kind) {
    case 'notRecorded': return null
    case 'recorded': {
      const groups = [diagnostics.groupZero, diagnostics.groupOne]
      const expectedDifference = diagnostics.groupOne.restrictedMean - diagnostics.groupZero.restrictedMean
      const scale = Math.max(1, Math.abs(expectedDifference), Math.abs(restrictedMeanDifference))
      const checks: readonly (readonly [invalid: boolean, detail: string])[] = [
        [groups.some((group) => !valuesMoveForward(group.cumulativeHazard, 'nondecreasing')), 'Each cumulative-hazard curve must move forward in time without decreasing.'],
        [groups.some((group) => !valuesMoveForward(group.smoothedHazard, 'any')), 'Each smoothed-hazard curve must move forward in time.'],
        [groups.some((group) => !valuesMoveForward(group.atRisk, 'nonincreasing')), 'Each number-at-risk series must move forward in time without increasing.'],
        [diagnostics.crossingTimes.some((time, index) => index > 0 && diagnostics.crossingTimes[index - 1]! > time), 'Survival-curve crossing times must be ordered.'],
        [Math.abs(expectedDifference - restrictedMeanDifference) > scale * 1e-8, 'The group restricted means do not reproduce the reported difference.'],
      ]
      return checks.find(([invalid]) => invalid)?.[1] ?? null
    }
    default: return assertNever(diagnostics)
  }
}

function conversionDifferenceMatches(groupZero: number, groupOne: number, difference: number): boolean {
  const expected = groupOne - groupZero
  const scale = Math.max(1, Math.abs(expected), Math.abs(difference))
  return Math.abs(expected - difference) <= scale * 1e-8
}

function approximatelyEqual(left: number, right: number): boolean {
  const scale = Math.max(1, Math.abs(left), Math.abs(right))
  return Math.abs(left - right) <= scale * 1e-8
}

function coxCoefficientProblem(estimate: CoxRegressionEvidence['coefficients'][number]): string | null {
  const [coefficientLower, coefficientUpper] = estimate.coefficientInterval
  const [hazardLower, hazardUpper] = estimate.hazardRatioInterval
  const checks: readonly (readonly [invalid: boolean, detail: string])[] = [
    [coefficientLower > coefficientUpper, 'A Cox coefficient interval has its bounds reversed.'],
    [hazardLower > hazardUpper, 'A Cox hazard-ratio interval has its bounds reversed.'],
    [!approximatelyEqual(estimate.hazardRatio, Math.exp(estimate.coefficient)), 'A Cox hazard ratio does not equal the exponential of its coefficient.'],
    [!approximatelyEqual(hazardLower, Math.exp(coefficientLower)) || !approximatelyEqual(hazardUpper, Math.exp(coefficientUpper)), 'A Cox hazard-ratio interval does not match its coefficient interval.'],
    [!approximatelyEqual(estimate.z, estimate.coefficient / estimate.standardError), 'A Cox z statistic does not match its coefficient and standard error.'],
  ]
  return checks.find(([invalid]) => invalid)?.[1] ?? null
}

function coxBaselineCurveProblem(curve: readonly CoxBaselinePoint[]): string | null {
  const inconsistentSurvival = curve.some((point) => (
    !approximatelyEqual(point.survival, Math.exp(-point.cumulativeHazard))
  ))
  if (inconsistentSurvival) return 'A Cox baseline survival value does not match its cumulative hazard.'
  return curve.reduce<string | null>((problem, point, index) => {
    if (problem !== null || index === 0) return problem
    const previous = curve[index - 1]!
    if (previous.time > point.time) return 'A Cox baseline curve is not ordered by time.'
    if (previous.cumulativeHazard > point.cumulativeHazard) return 'A Cox baseline cumulative hazard decreases over time.'
    if (previous.survival < point.survival) return 'A Cox baseline survival curve increases over time.'
    return null
  }, null)
}

function comparisonSummaryProblem(evidence: ComparisonSurvivalEvidence): string | null {
  switch (evidence.observedConversion.kind) {
    case 'notRecorded':
    case 'unavailable': break
    case 'recorded': {
      const result = evidence.observedConversion.result
      if (!conversionDifferenceMatches(result.groupZeroRate, result.groupOneRate, result.difference)) {
        return 'The observed conversion rates do not reproduce their reported difference.'
      }
      break
    }
    default: return assertNever(evidence.observedConversion)
  }

  switch (evidence.fixedTimeConversion.kind) {
    case 'notRecorded':
    case 'unavailable': break
    case 'recorded': {
      const result = evidence.fixedTimeConversion.result
      if (result.time !== evidence.truncationTime) {
        return 'The fixed-time conversion comparison does not use the recorded comparison time.'
      }
      if (!conversionDifferenceMatches(result.groupZeroRate, result.groupOneRate, result.difference)) {
        return 'The fixed-time conversion rates do not reproduce their reported difference.'
      }
      if (result.interval[0] > result.interval[1]) {
        return 'The fixed-time conversion interval has its bounds reversed.'
      }
      if (new Set(result.scaleTests.map((test) => test.scale)).size !== result.scaleTests.length) {
        return 'The fixed-time comparison repeats a transformation scale.'
      }
      const intervalReversed = result.scaleTests.some((test) => (
        test.groupZeroInterval[0] > test.groupZeroInterval[1]
        || test.groupOneInterval[0] > test.groupOneInterval[1]
      ))
      if (intervalReversed) return 'A fixed-time conversion interval has its bounds reversed.'
      break
    }
    default: return assertNever(evidence.fixedTimeConversion)
  }

  switch (evidence.petoPeto.kind) {
    case 'notRecorded':
    case 'unavailable': return null
    case 'recorded': return evidence.petoPeto.result.rho === 1
      ? null
      : 'The Peto–Peto result must use rho = 1.'
    default: return assertNever(evidence.petoPeto)
  }
}

export function parseFlexSurvEvidence(
  value: unknown,
): Result<FlexSurvEvidence, SurvivalEvidenceProblem> {
  const parsed = flexSurvEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return invalidEvidence(z.prettifyError(parsed.error))
  }
  const evidence = parsed.data
  const expectedBaseline = baselineParameterCount(evidence.family)
  if (evidence.events > evidence.observations) {
    return invalidEvidence('The event count exceeds the observation count.')
  }
  if (evidence.naturalBaseline.length !== expectedBaseline) {
    return invalidEvidence(
      `${evidence.family} evidence must carry ${expectedBaseline} baseline parameter${expectedBaseline === 1 ? '' : 's'}.`,
    )
  }
  if (!baselineParametersAreValid(evidence.family, evidence.naturalBaseline)) {
    return invalidEvidence('The fitted baseline parameters are outside the family parameter space.')
  }
  if (evidence.coefficients.length !== evidence.profile.length) {
    return invalidEvidence('The prediction profile must contain one value for each fitted covariate coefficient.')
  }
  if (evidence.parameterIntervals.length !== expectedBaseline + evidence.coefficients.length) {
    return invalidEvidence('The parameter intervals do not match the fitted baseline and covariate parameters.')
  }
  if (!intervalsAreOrdered(evidence.parameterIntervals)) {
    return invalidEvidence('A parametric survival interval has its bounds reversed.')
  }
  if (
    evidence.predictionTimes.length !== evidence.survival.length
    || evidence.predictionTimes.length !== evidence.hazard.length
  ) {
    return invalidEvidence('Prediction times, survival probabilities and hazards must have the same length.')
  }
  if (evidence.hazard.some((value, index) => value === null && evidence.predictionTimes[index] !== 0)) {
    return invalidEvidence('The hazard may be undefined only at time zero.')
  }
  return ok(evidence)
}

export function parseNonparametricSurvivalEvidence(
  value: unknown,
): Result<NonparametricSurvivalEvidence, SurvivalEvidenceProblem> {
  const parsed = nonparametricSurvivalEvidenceSchema.safeParse(value)
  if (!parsed.success) return invalidEvidence(z.prettifyError(parsed.error))
  const evidence = parsed.data
  const lengths = [
    evidence.survival,
    evidence.survivalLower,
    evidence.survivalUpper,
    evidence.cumulativeDensity,
    evidence.cumulativeHazard,
    evidence.cumulativeHazardLower,
    evidence.cumulativeHazardUpper,
    evidence.hazardIncrement,
  ].map((values) => values.length)
  if (lengths.some((length) => length !== evidence.predictionTimes.length)) {
    return invalidEvidence('Every nonparametric curve must contain one value for each prediction time.')
  }
  if (evidence.events > evidence.observations) return invalidEvidence('The event count exceeds the observation count.')
  if (evidence.predictionTimes.some((time, index) => index > 0 && evidence.predictionTimes[index - 1]! > time)) {
    return invalidEvidence('Nonparametric prediction times must be ordered.')
  }
  if (evidence.survival.some((value, index) => index > 0 && evidence.survival[index - 1]! < value)) {
    return invalidEvidence('The Kaplan–Meier curve must not increase over time.')
  }
  if (evidence.survival.some((value, index) => evidence.survivalLower[index]! > value || value > evidence.survivalUpper[index]!)) {
    return invalidEvidence('Each Kaplan–Meier estimate must lie inside its confidence interval.')
  }
  if (evidence.cumulativeDensity.some((value, index) => Math.abs(value - (1 - evidence.survival[index]!)) > 1e-10)) {
    return invalidEvidence('The cumulative density must equal one minus the Kaplan–Meier estimate.')
  }
  if (evidence.cumulativeHazard.some((value, index) => index > 0 && evidence.cumulativeHazard[index - 1]! > value)) {
    return invalidEvidence('The Nelson–Aalen cumulative hazard must not decrease over time.')
  }
  if (evidence.cumulativeHazard.some((value, index) => evidence.cumulativeHazardLower[index]! > value || value > evidence.cumulativeHazardUpper[index]!)) {
    return invalidEvidence('Each Nelson–Aalen estimate must lie inside its confidence interval.')
  }
  const incrementsReproduceCurve = evidence.cumulativeHazard.every((value, index) => {
    const previous = index === 0 ? 0 : evidence.cumulativeHazard[index - 1]!
    const scale = Math.max(1, Math.abs(value))
    return Math.abs(value - previous - evidence.hazardIncrement[index]!) <= scale * 1e-10
  })
  if (!incrementsReproduceCurve) return invalidEvidence('The Nelson–Aalen increments do not reproduce the cumulative-hazard curve.')
  return ok(evidence)
}

export function parseCoxRegressionEvidence(
  value: unknown,
): Result<CoxRegressionEvidence, SurvivalEvidenceProblem> {
  const parsed = coxRegressionEvidenceSchema.safeParse(value)
  if (!parsed.success) return invalidEvidence(z.prettifyError(parsed.error))
  const evidence = parsed.data
  if (evidence.events > evidence.observations) {
    return invalidEvidence('The event count exceeds the observation count.')
  }
  const coefficientProblem = evidence.coefficients
    .map(coxCoefficientProblem)
    .find((problem) => problem !== null)
  if (coefficientProblem !== undefined && coefficientProblem !== null) {
    return invalidEvidence(coefficientProblem)
  }
  const baselineCurves = (() => {
    switch (evidence.baseline.kind) {
      case 'shared': return [evidence.baseline.estimates]
      case 'stratified': return evidence.baseline.curves.map(({ estimates }) => estimates)
      default: return assertNever(evidence.baseline)
    }
  })()
  const baselineProblem = baselineCurves
    .map(coxBaselineCurveProblem)
    .find((problem) => problem !== null)
  if (baselineProblem !== undefined && baselineProblem !== null) {
    return invalidEvidence(baselineProblem)
  }
  if (
    evidence.baseline.kind === 'stratified'
    && new Set(evidence.baseline.curves.map(({ stratum }) => stratum)).size !== evidence.baseline.curves.length
  ) {
    return invalidEvidence('A Cox baseline result repeats a stratum.')
  }
  if (evidence.covariance.length !== evidence.coefficients.length ** 2) {
    return invalidEvidence('The Cox covariance matrix does not match the coefficient count.')
  }
  const coefficientCount = evidence.coefficients.length
  const asymmetricCovariance = evidence.covariance.some((value, index) => {
    const row = Math.floor(index / coefficientCount)
    const column = index % coefficientCount
    return !approximatelyEqual(value, evidence.covariance[column * coefficientCount + row]!)
  })
  if (asymmetricCovariance) return invalidEvidence('The Cox covariance matrix is not symmetric.')
  if (evidence.standardErrors === 'modelBased') {
    const inconsistentStandardError = evidence.coefficients.some((estimate, column) => (
      !approximatelyEqual(
        estimate.standardError ** 2,
        evidence.covariance[column * coefficientCount + column]!,
      )
    ))
    if (inconsistentStandardError) {
      return invalidEvidence('A model-based Cox standard error does not match the covariance matrix.')
    }
  }
  if (
    evidence.covariateMeans.length !== evidence.coefficients.length
    || evidence.covariateStandardDeviations.length !== evidence.coefficients.length
  ) {
    return invalidEvidence('The Cox covariate summaries do not match the coefficient count.')
  }
  if (!approximatelyEqual(evidence.likelihoodRatio, 2 * (evidence.logLikelihood - evidence.nullLogLikelihood))) {
    return invalidEvidence('The Cox likelihood-ratio statistic does not match the fitted and null log likelihoods.')
  }
  if (!approximatelyEqual(evidence.partialAic, 2 * coefficientCount - 2 * evidence.logLikelihood)) {
    return invalidEvidence('The Cox partial AIC does not match the fitted log likelihood and coefficient count.')
  }
  if (evidence.proportionalHazardsTests.kind === 'recorded') {
    const transforms = evidence.proportionalHazardsTests.transforms
    if (new Set(transforms.map(({ transform }) => transform)).size !== transforms.length) {
      return invalidEvidence('The proportional-hazards result repeats a time transform.')
    }
    if (transforms.some(({ tests }) => tests.length !== evidence.coefficients.length)) {
      return invalidEvidence('Each proportional-hazards transform must contain one result for each coefficient.')
    }
  }
  if (evidence.observation.kind === 'startStop' && evidence.concordance.kind === 'recorded') {
    return invalidEvidence('A start-stop Cox result cannot record a concordance statistic.')
  }
  if (
    evidence.observation.kind === 'startStop'
    && evidence.proportionalHazardsTests.kind === 'recorded'
  ) {
    return invalidEvidence('A start-stop Cox result cannot record proportional-hazards tests.')
  }
  if (evidence.frailty.kind === 'gamma') {
    if (evidence.observation.kind !== 'rightCensored' || evidence.observation.delayedEntry) {
      return invalidEvidence('A shared frailty result needs right-censored rows without delayed entry.')
    }
    if (evidence.standardErrors !== 'modelBased') {
      return invalidEvidence('A shared frailty result reports model-based standard errors only.')
    }
    if (evidence.frailty.standardErrors2.length !== coefficientCount) {
      return invalidEvidence('The shared frailty second standard errors do not match the coefficient count.')
    }
    if (evidence.frailty.history.length !== evidence.frailty.outerIterations) {
      return invalidEvidence('The shared frailty history does not match the outer iteration count.')
    }
    if (evidence.proportionalHazardsTests.kind === 'recorded') {
      return invalidEvidence('A shared frailty result cannot record proportional-hazards tests.')
    }
  }
  return ok(evidence)
}

export function parsePenalizedAftEvidence(
  value: unknown,
): Result<PenalizedAftEvidence, SurvivalEvidenceProblem> {
  const parsed = penalizedAftEvidenceSchema.safeParse(value)
  if (!parsed.success) return invalidEvidence(z.prettifyError(parsed.error))
  const evidence = parsed.data
  if (evidence.events > evidence.observations) {
    return invalidEvidence('The event count exceeds the observation count.')
  }
  const parameters = evidence.coefficients.length + 2
  if (evidence.covariance.length !== parameters ** 2) {
    return invalidEvidence('The AFT covariance matrix does not match the parameter count.')
  }
  if (evidence.covariateMeans.length !== evidence.coefficients.length) {
    return invalidEvidence('The AFT covariate means do not match the coefficient count.')
  }
  if (evidence.survival.length !== evidence.predictionTimes.length) {
    return invalidEvidence('The AFT survival curve does not match its prediction times.')
  }
  const inconsistent = [...evidence.coefficients, evidence.intercept, evidence.ancillary]
    .some((estimate) => !approximatelyEqual(estimate.timeRatio, Math.exp(estimate.coefficient)))
  if (inconsistent) return invalidEvidence('An AFT time ratio does not match its coefficient.')
  return ok(evidence)
}

export function parseComparisonSurvivalEvidence(
  value: unknown,
): Result<ComparisonSurvivalEvidence, SurvivalEvidenceProblem> {
  const parsed = comparisonSurvivalEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return invalidEvidence(z.prettifyError(parsed.error))
  }
  const evidence = parsed.data
  if (!curveIsOrdered(evidence.groupZeroCurve) || !curveIsOrdered(evidence.groupOneCurve)) {
    return invalidEvidence('Each Kaplan–Meier curve must move forward in time without increasing.')
  }
  if (evidence.restrictedMeanInterval[0] > evidence.restrictedMeanInterval[1]) {
    return invalidEvidence('The restricted-mean interval has its bounds reversed.')
  }
  const diagnosticsProblem = comparisonDiagnosticsProblem(evidence.diagnostics, evidence.restrictedMeanDifference)
  if (diagnosticsProblem !== null) return invalidEvidence(diagnosticsProblem)
  const summaryProblem = comparisonSummaryProblem(evidence)
  if (summaryProblem !== null) return invalidEvidence(summaryProblem)
  return ok(evidence)
}

export function parseMultiStateSurvivalEvidence(
  value: unknown,
): Result<MultiStateSurvivalEvidence, SurvivalEvidenceProblem> {
  const parsed = multiStateSurvivalEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return invalidEvidence(z.prettifyError(parsed.error))
  }
  const evidence = parsed.data
  const stateCount = evidence.states.length
  const expectedMatrixLength = stateCount * stateCount
  if (new Set(evidence.states).size !== stateCount) {
    return invalidEvidence('Multi-state evidence must not repeat state codes.')
  }
  if (evidence.transitions.some(([from, to]) => from >= stateCount || to >= stateCount || from === to)) {
    return invalidEvidence('A multi-state transition has an invalid state index.')
  }
  if (evidence.predictionTimes.some((time, index) => index > 0 && evidence.predictionTimes[index - 1]! > time)) {
    return invalidEvidence('Multi-state prediction times must be ordered.')
  }
  if (evidence.probabilities.length !== evidence.predictionTimes.length) {
    return invalidEvidence('Multi-state predictions must contain one probability matrix for each time.')
  }
  for (const matrix of evidence.probabilities) {
    if (matrix.length !== expectedMatrixLength) {
      return invalidEvidence('A multi-state probability matrix does not match the number of states.')
    }
    if (matrix.some((value) => value < -1e-8 || value > 1 + 1e-8)) {
      return invalidEvidence('A multi-state probability lies outside the numerical range from 0 to 1.')
    }
    for (let row = 0; row < stateCount; row += 1) {
      const offset = row * stateCount
      const total = matrix.slice(offset, offset + stateCount).reduce((sum, value) => sum + value, 0)
      if (Math.abs(total - 1) > 1e-6) {
        return invalidEvidence('A row in a multi-state probability matrix does not sum to 1.')
      }
    }
  }
  return ok(evidence)
}

interface SurvivalRunIdentity {
  readonly id: SurvivalRunId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  /** Source columns used to construct the analysis input, in their recorded role order. */
  readonly columns: NonEmptyArray<ColumnSelection>
}

export interface NewSurvivalRunIdentity {
  readonly id: SurvivalRunId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly columns: NonEmptyArray<ColumnSelection>
}

export type SurvivalRunProblem =
  | { readonly kind: 'family-mismatch' }
  | { readonly kind: 'observation-mismatch' }
  | { readonly kind: 'entry-mismatch' }
  | { readonly kind: 'standard-errors-mismatch' }
  | { readonly kind: 'strata-mismatch' }
  | {
      readonly kind: 'coefficient-count-mismatch'
      readonly covariates: number
      readonly coefficients: number
    }

type RightCensoredSurvivalRun = {
  readonly [Family in ParametricSurvivalFamily]: SurvivalRunIdentity & {
    readonly kind: 'right-censored-survival-run'
    readonly configuration: RightCensoredSurvivalConfiguration<Family>
    readonly evidence: FlexSurvEvidence<Family>
  }
}[ParametricSurvivalFamily]

type StartStopSurvivalRun = {
  readonly [Family in ProportionalHazardsFamily]: SurvivalRunIdentity & {
    readonly kind: 'start-stop-survival-run'
    readonly configuration: StartStopSurvivalConfiguration<Family>
    readonly evidence: FlexSurvEvidence<Family>
  }
}[ProportionalHazardsFamily]

export type SurvivalRunArtifact =
  | RightCensoredSurvivalRun
  | StartStopSurvivalRun
  | SurvivalRunIdentity & {
      readonly kind: 'nonparametric-survival-run'
      readonly configuration: NonparametricSurvivalConfiguration
      readonly evidence: NonparametricSurvivalEvidence
    }
  | SurvivalRunIdentity & {
      readonly kind: 'two-group-survival-run'
      readonly configuration: TwoGroupSurvivalConfiguration
      readonly evidence: ComparisonSurvivalEvidence
    }
  | SurvivalRunIdentity & {
      readonly kind: 'multi-state-survival-run'
      readonly configuration: MultiStateSurvivalConfiguration
      readonly evidence: MultiStateSurvivalEvidence
    }
  | SurvivalRunIdentity & {
      readonly kind: 'cox-regression-run'
      readonly configuration: CoxRegressionConfiguration
      readonly evidence: CoxRegressionEvidence
    }
  | SurvivalRunIdentity & {
      readonly kind: 'penalized-aft-run'
      readonly configuration: PenalizedAftConfiguration
      readonly evidence: PenalizedAftEvidence
    }

/** Bind a fitted parametric result to the exact family and row roles that produced it. */
export function rightCensoredSurvivalRun(
  identity: NewSurvivalRunIdentity,
  configuration: RightCensoredSurvivalConfiguration,
  evidence: FlexSurvEvidence,
): Result<SurvivalRunArtifact, SurvivalRunProblem> {
  if (configuration.family !== evidence.family) return err({ kind: 'family-mismatch' })
  return ok({ ...identity, kind: 'right-censored-survival-run', configuration, evidence } as SurvivalRunArtifact)
}

/** Bind a start-stop fit only after its proportional-hazards family is confirmed. */
export function startStopSurvivalRun(
  identity: NewSurvivalRunIdentity,
  configuration: StartStopSurvivalConfiguration,
  evidence: FlexSurvEvidence,
): Result<SurvivalRunArtifact, SurvivalRunProblem> {
  if (configuration.family !== evidence.family) return err({ kind: 'family-mismatch' })
  return ok({ ...identity, kind: 'start-stop-survival-run', configuration, evidence } as SurvivalRunArtifact)
}

export function multiStateSurvivalRun(
  identity: NewSurvivalRunIdentity,
  configuration: MultiStateSurvivalConfiguration,
  evidence: MultiStateSurvivalEvidence,
): Result<SurvivalRunArtifact, SurvivalRunProblem> {
  if (configuration.family !== evidence.family) return err({ kind: 'family-mismatch' })
  return ok({ ...identity, kind: 'multi-state-survival-run', configuration, evidence })
}

/** Bind a penalised AFT fit to the family and covariates that produced it. */
export function penalizedAftRun(
  identity: NewSurvivalRunIdentity,
  configuration: PenalizedAftConfiguration,
  evidence: PenalizedAftEvidence,
): Result<SurvivalRunArtifact, SurvivalRunProblem> {
  if (configuration.family !== evidence.family) return err({ kind: 'family-mismatch' })
  if (configuration.covariates.length !== evidence.coefficients.length) {
    return err({ kind: 'coefficient-count-mismatch', covariates: configuration.covariates.length, coefficients: evidence.coefficients.length })
  }
  return ok({ ...identity, kind: 'penalized-aft-run', configuration, evidence })
}

export function coxRegressionRun(
  identity: NewSurvivalRunIdentity,
  configuration: CoxRegressionConfiguration,
  evidence: CoxRegressionEvidence,
): Result<SurvivalRunArtifact, SurvivalRunProblem> {
  const observationMatches = (
    configuration.observation.kind === 'right-censored'
    && evidence.observation.kind === 'rightCensored'
  ) || (
    configuration.observation.kind === 'start-stop'
    && evidence.observation.kind === 'startStop'
  )
  if (!observationMatches) {
    return err({ kind: 'observation-mismatch' })
  }
  if (
    configuration.observation.kind === 'right-censored'
    && evidence.observation.kind === 'rightCensored'
    && (configuration.observation.entry.kind === 'column') !== evidence.observation.delayedEntry
  ) {
    return err({ kind: 'entry-mismatch' })
  }
  const configuredStandardErrors = configuration.observation.standardErrors.kind === 'model-based'
    ? 'modelBased'
    : configuration.observation.standardErrors.kind
  if (configuredStandardErrors !== evidence.standardErrors) {
    return err({ kind: 'standard-errors-mismatch' })
  }
  const baselineMatches = (
    configuration.strata.kind === 'unstratified'
    && evidence.baseline.kind === 'shared'
  ) || (
    configuration.strata.kind === 'column'
    && evidence.baseline.kind === 'stratified'
  )
  if (!baselineMatches) return err({ kind: 'strata-mismatch' })
  if (configuration.covariates.length !== evidence.coefficients.length) {
    return err({
      kind: 'coefficient-count-mismatch',
      covariates: configuration.covariates.length,
      coefficients: evidence.coefficients.length,
    })
  }
  return ok({ ...identity, kind: 'cox-regression-run', configuration, evidence })
}

/**
 * The survival kernels report a refusal as the Rust value that carried it. The reader gets the sentence
 * behind the known ones; anything else passes through unchanged.
 */
export const describeSurvivalRefusal = (detail: string): string => {
  const nonPositive = /NonPositiveTransformedTime \{ row: (\d+) \}/.exec(detail)
  if (nonPositive !== null) {
    return `Row ${Number(nonPositive[1]) + 1} of the data has a duration of zero or less. A parametric distribution needs every duration above zero; shift the times or use Compare groups, which accepts them.`
  }
  if (detail.includes('NoPositiveTimes')) return 'No duration in the data is above zero, so no parametric distribution can be fitted.'
  if (detail.includes('DegenerateTimes')) return 'Every duration in the data is the same, so no parametric distribution can be fitted.'
  if (detail.includes('FixedTimeOutsideEventRange')) return 'The comparison time falls outside the observed event times. Choose a time after the first event and before the last.'
  if (detail.includes('ComparisonTimeTooEarly')) return 'The comparison time falls before the first observed event. Choose a later time.'
  if (detail.includes('ComparisonTimeTooLate')) return 'The comparison time falls after the last observed event. Choose an earlier time.'
  return detail
}
