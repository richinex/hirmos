import { z } from 'zod'
import type { NumericColumnSelection } from './dataset'
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

export interface RightCensoredSurvivalConfiguration<
  Family extends ParametricSurvivalFamily = ParametricSurvivalFamily,
> {
  readonly kind: 'right-censored-parametric'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
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
  readonly covariates: readonly NumericColumnSelection[]
  readonly family: Family
  readonly predictionTimes: NonEmptyArray<number>
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

export interface MultiStateSurvivalConfiguration {
  readonly kind: 'multi-state-proportional-hazards'
  readonly start: NumericColumnSelection
  readonly stop: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly from: NumericColumnSelection
  readonly to: NumericColumnSelection
  readonly family: ProportionalHazardsFamily
  readonly predictionTimes: NonEmptyArray<number>
}

export type SurvivalConfiguration =
  | RightCensoredSurvivalConfiguration
  | StartStopSurvivalConfiguration
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
  hazard: z.array(finiteNumber.nonnegative()).min(1),
  median: nonNegativeTime,
  mean: z.union([nonNegativeTime, z.null()]),
}).strict()

type ParsedFlexSurvEvidence = z.infer<typeof flexSurvEvidenceSchema>

export type FlexSurvEvidence<
  Family extends ParametricSurvivalFamily = ParametricSurvivalFamily,
> = Omit<ParsedFlexSurvEvidence, 'family'> & { readonly family: Family }

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

export const comparisonSurvivalEvidenceSchema = z.object({
  kind: z.literal('comparisonSurvival'),
  observations: z.number().int().positive(),
  truncationTime: finiteNumber.positive(),
  groupZeroCurve: z.array(survivalCurvePointSchema).min(1),
  groupOneCurve: z.array(survivalCurvePointSchema).min(1),
  diagnostics: comparisonSurvivalDiagnosticsSchema,
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
  predictionTimes: z.array(nonNegativeTime).min(1),
  probabilities: z.array(z.array(finiteNumber)).min(1),
}).strict()

export type MultiStateSurvivalEvidence = z.infer<typeof multiStateSurvivalEvidenceSchema>

export type SurvivalEvidence =
  | FlexSurvEvidence
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
  /** Columns in the order used to materialise the numeric matrix. */
  readonly columns: NonEmptyArray<NumericColumnSelection>
}

export interface NewSurvivalRunIdentity {
  readonly id: SurvivalRunId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly columns: NonEmptyArray<NumericColumnSelection>
}

export type SurvivalRunProblem = { readonly kind: 'family-mismatch' }

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
      readonly kind: 'two-group-survival-run'
      readonly configuration: TwoGroupSurvivalConfiguration
      readonly evidence: ComparisonSurvivalEvidence
    }
  | SurvivalRunIdentity & {
      readonly kind: 'multi-state-survival-run'
      readonly configuration: MultiStateSurvivalConfiguration
      readonly evidence: MultiStateSurvivalEvidence
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
