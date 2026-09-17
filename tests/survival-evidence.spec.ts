import { readFileSync } from 'node:fs'
import { expect, test } from '@playwright/test'
import { parseAnalysisWorkerCommand } from '../src/workers/analysisProtocol'
import {
  describeSurvivalRefusal,
  parseComparisonSurvivalEvidence,
  parseCoxRegressionEvidence,
  parseFlexSurvEvidence,
  parseNonparametricSurvivalEvidence,
} from '../src/domain/survival'

// A family whose hazard rises without bound as time approaches zero reports no value there; the
// evidence admits that one point and nothing else.

const weibull = (hazard: readonly (number | null)[]) => ({
  kind: 'flexSurv', observations: 4000, events: 964, family: 'weibull',
  naturalBaseline: [0.168, 35732], coefficients: [-0.731], parameterIntervals: [[0.158, 0.179], [17008, 75068], [-1.48, 0.022]],
  logLikelihood: -1235, aic: 2477, bic: 2496, profile: [0.5],
  predictionTimes: [0, 3.5, 7], survival: [1, 0.82, 0.78], hazard, median: 2805, mean: null,
})

test('a hazard undefined at time zero is admitted', () => {
  const parsed = parseFlexSurvEvidence(weibull([null, 0.03, 0.02]))
  expect(parsed.ok).toBe(true)
})

test('a hazard undefined after time zero is refused', () => {
  const parsed = parseFlexSurvEvidence(weibull([0.5, null, 0.02]))
  expect(parsed.ok).toBe(false)
  if (!parsed.ok) expect(parsed.error.detail).toContain('only at time zero')
})

test('a kernel refusal over a zero duration names the row and the way out', () => {
  expect(describeSurvivalRefusal('survival initialization refused: InitialValues(NonPositiveTransformedTime { row: 5 })'))
    .toBe('Row 6 of the data has a duration of zero or less. A parametric distribution needs every duration above zero; shift the times or use Compare groups, which accepts them.')
  expect(describeSurvivalRefusal('survival fit failed: Singular')).toBe('survival fit failed: Singular')
})

const nonparametric = () => ({
  kind: 'nonparametricSurvival',
  observations: 10,
  events: 3,
  predictionTimes: [0, 1, 2],
  survival: [1, 0.8, 0.6],
  survivalLower: [1, 0.7, 0.5],
  survivalUpper: [1, 0.9, 0.7],
  cumulativeDensity: [0, 0.2, 0.4],
  cumulativeHazard: [0, 0.2, 0.45],
  cumulativeHazardLower: [0, 0.1, 0.3],
  cumulativeHazardUpper: [0, 0.3, 0.6],
  hazardIncrement: [0, 0.2, 0.25],
})

test('nonparametric evidence admits internally consistent curves', () => {
  expect(parseNonparametricSurvivalEvidence(nonparametric()).ok).toBe(true)
})

test('nonparametric evidence refuses intervals and increments that contradict the curves', () => {
  const invalidInterval = nonparametric()
  invalidInterval.survivalLower[1] = 0.85
  expect(parseNonparametricSurvivalEvidence(invalidInterval).ok).toBe(false)

  const invalidIncrement = nonparametric()
  invalidIncrement.hazardIncrement[2] = 0.5
  expect(parseNonparametricSurvivalEvidence(invalidIncrement).ok).toBe(false)
})

const coxRegression = () => ({
  kind: 'coxRegression',
  observations: 40,
  events: 12,
  totalWeight: 40,
  observation: { kind: 'rightCensored', delayedEntry: false },
  standardErrors: 'modelBased',
  coefficients: [{
    coefficient: 0.2,
    hazardRatio: Math.exp(0.2),
    standardError: 0.1,
    coefficientInterval: [0.004, 0.396],
    hazardRatioInterval: [Math.exp(0.004), Math.exp(0.396)],
    z: 2,
    pValue: 0.0455,
  }],
  covariance: [0.01],
  logLikelihood: -42,
  nullLogLikelihood: -44,
  likelihoodRatio: 4,
  likelihoodRatioPValue: 0.0455,
  partialAic: 86,
  iterations: 4,
  covariateMeans: [0.5],
  covariateStandardDeviations: [0.2],
  baseline: {
    kind: 'shared',
    estimates: [{ time: 1, hazard: 0.1, cumulativeHazard: 0.1, survival: Math.exp(-0.1) }],
  },
  concordance: { kind: 'recorded', result: 0.7 },
  proportionalHazardsTests: {
    kind: 'recorded',
    transforms: [{ transform: 'eventRank', tests: [{ statistic: 0.4, pValue: 0.53 }] }],
  },
})

test('Cox evidence admits a consistent fitted model', () => {
  expect(parseCoxRegressionEvidence(coxRegression()).ok).toBe(true)
})

test('clustered Breslow retains its sandwich covariance and convergence status', () => {
  const value = {
    ...coxRegression(), standardErrors: 'clustered',
    fitting: { kind: 'clusteredBreslow', clusters: 10, convergence: 'iterationLimit', robustCovariance: [0.01], scoreTest: 4, robustScoreTest: 3, waldTest: 4 },
    proportionalHazardsTests: { kind: 'unavailable', reason: 'Not available for this fit.' },
  }
  const parsed = parseCoxRegressionEvidence(value)
  expect(parsed.ok).toBe(true)
  if (parsed.ok) expect(parsed.value.fitting).toMatchObject({ kind: 'clusteredBreslow', convergence: 'iterationLimit' })
  expect(parseCoxRegressionEvidence({ ...value, fitting: { ...value.fitting, robustCovariance: [0.04] } }).ok).toBe(false)
  expect(parseCoxRegressionEvidence({ ...value, observation: { kind: 'startStop', subjects: 20 } }).ok).toBe(false)
  expect(parseCoxRegressionEvidence({ ...value, fitting: { ...value.fitting, clusters: 41 } }).ok).toBe(false)
})

test('Cox evidence refuses a hazard ratio that contradicts its coefficient', () => {
  const evidence = coxRegression()
  evidence.coefficients[0]!.hazardRatio = 0.5
  expect(parseCoxRegressionEvidence(evidence).ok).toBe(false)
})

test('clustered Breslow rejects unsupported combinations at the worker boundary', () => {
  const design = {
    observation: { kind: 'rightCensored', duration: 0, event: 1, entry: { kind: 'notUsed' }, standardErrors: { kind: 'clusteredBreslow', column: 2 }, frailty: { kind: 'none' } },
    weights: { kind: 'equal' }, strata: { kind: 'unstratified' }, penalty: { kind: 'unpenalized' }, covariates: [3], confidenceLevel: 0.95,
  }
  const command = { kind: 'cox-regression', request: 'caea2001-5302-4037-997e-fcfcdb27ed56', rows: 2, columns: 5, values: new Float64Array(10), design }
  expect(parseAnalysisWorkerCommand(command).ok).toBe(true)
  for (const changed of [
    { ...design, weights: { kind: 'column', column: 4 } },
    { ...design, strata: { kind: 'column', column: 4 } },
    { ...design, penalty: { kind: 'elasticNet', penalizer: 0.1, l1Ratio: 0 } },
    { ...design, observation: { ...design.observation, entry: { kind: 'column', column: 4 } } },
    { ...design, observation: { ...design.observation, frailty: { kind: 'gamma', column: 4, ties: 'breslow' } } },
  ]) expect(parseAnalysisWorkerCommand({ ...command, design: changed }).ok).toBe(false)
})

test('Cox evidence refuses right-censored diagnostics on a start-stop fit', () => {
  const evidence = { ...coxRegression(), observation: { kind: 'startStop', subjects: 20 } }
  expect(parseCoxRegressionEvidence(evidence).ok).toBe(false)
})

// The veteran trial compared through day 500: the untransformed fixed-time interval runs past 1,
// as the reference reports it, and the evidence keeps it.
test('comparison evidence admits an untransformed fixed-time interval that passes 1', () => {
  const evidence = JSON.parse(readFileSync('tests/fixtures/veteran-comparison-500.json', 'utf8'))
  const parsed = parseComparisonSurvivalEvidence(evidence)
  expect(parsed.ok).toBe(true)
  expect(evidence.fixedTimeConversion.result.scaleTests[0].groupZeroInterval[1]).toBeGreaterThan(1)
})
