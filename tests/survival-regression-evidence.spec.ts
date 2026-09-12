import { expect, test } from '@playwright/test'
import { parseAalenEvidence, parseForestEvidence, forestSettingsSchema } from '../src/domain/survivalRegression'

const aalen = () => ({
  kind: 'aalen', observations: 20, events: 12, fittedEvents: 10, lastTime: 5,
  coefficients: [[0.2, 0.3, 0.1, 3, 0.003], [-0.1, -0.2, 0.1, -2, 0.05]],
  curves: [[[0, 0, 0, 0], [5, 0.2, -0.1, 0.5]], [[0, 0, 0, 0], [5, -0.2, -0.4, 0]]],
  chisq: 4, degreesOfFreedom: 1, pValue: 0.05,
})
const forest = () => ({
  kind: 'survivalForest', observations: 20, events: 12, trees: 7,
  importance: [{ kind: 'recorded', result: -0.1 }],
  concordance: { kind: 'recorded', result: 0.7 },
  predictionRow: 0, profile: [2], predictionTimes: [1, 5],
  cumulativeHazard: [0.1, 1], survival: [Math.exp(-0.1), Math.exp(-1)],
})

test('Aalen admits signed coefficients and pointwise bounds crossing zero', () => {
  expect(parseAalenEvidence(aalen()).ok).toBe(true)
})

test('Aalen rejects reversed intervals, unordered time, mismatched terms and event counts', () => {
  const invalid = [
    { ...aalen(), fittedEvents: 13 },
    { ...aalen(), degreesOfFreedom: 2 },
    { ...aalen(), lastTime: 6 },
    { ...aalen(), curves: [[[0, 0, 0, 0], [5, 0.2, 0.5, -0.1]], aalen().curves[1]] },
    { ...aalen(), curves: [[[5, 0, 0, 0], [5, 0.2, -0.1, 0.5]], aalen().curves[1]] },
  ]
  for (const value of invalid) expect(parseAalenEvidence(value).ok).toBe(false)
})

test('forest importance may be negative while survival remains a probability', () => {
  expect(parseForestEvidence(forest()).ok).toBe(true)
  expect(parseForestEvidence({ ...forest(), survival: [-0.1, 0.2] }).ok).toBe(false)
})

test('forest unavailable scores remain unavailable, not zero', () => {
  const result = parseForestEvidence({ ...forest(), concordance: { kind: 'unavailable', reason: 'No comparable pairs.' }, importance: [{ kind: 'unavailable', reason: 'No comparable pairs.' }] })
  expect(result.ok).toBe(true)
  if (result.ok) expect(result.value.concordance.kind).toBe('unavailable')
})

test('forest rejects wrong profile, invalid row and contradictory curves', () => {
  for (const value of [
    { ...forest(), profile: [1, 2] },
    { ...forest(), predictionRow: 20 },
    { ...forest(), survival: [0.9, 0.5] },
    { ...forest(), predictionTimes: [5, 1] },
    { ...forest(), cumulativeHazard: [1, 0.1], survival: [Math.exp(-1), Math.exp(-0.1)] },
    { ...forest(), survival: [0.5] },
  ]) expect(parseForestEvidence(value).ok).toBe(false)
})

test('forest settings require a reproducible seed and supported split rule', () => {
  const settings = { trees: 500, mtry: 4, seed: 1907, minNodeSize: 3, minBucket: 3, splitRule: 'extraTrees' }
  expect(forestSettingsSchema.safeParse(settings).success).toBe(true)
  for (const change of [{ seed: 0 }, { seed: 0x100000000 }, { mtry: 0 }, { trees: 0 }, { trees: 5001 }, { minBucket: 1.5 }, { splitRule: 'not-supported' }]) {
    expect(forestSettingsSchema.safeParse({ ...settings, ...change }).success).toBe(false)
  }
})
