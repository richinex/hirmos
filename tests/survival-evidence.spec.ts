import { expect, test } from '@playwright/test'
import { describeSurvivalRefusal, parseFlexSurvEvidence } from '../src/domain/survival'

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
