import { expect, test } from '@playwright/test'
import { continuousErrorsSchema, describeErrors, interruptedModelSchema, linearErrorModelSchema, parseInterruptedSeriesEvidence, sameErrors } from '../src/domain/interruptedSeries'
import { backdoorLinearEvidenceSchema, describeCovariance, linearReading } from '../src/domain/estimation'

const term = (name: string) => ({ name, coefficient: 0.1, standardError: 0.05, pValue: 0.04, interval: [0.0, 0.2] })
const arma = { kind: 'arma', p: 1, q: 1, ar: [term('ar.L1')], ma: [term('ma.L1')], sigma2: 1.5, logLikelihood: -10, aic: 30, bic: 32, iterations: 12, converged: true }

test('an error model carries only its own settings and needs at least one ARMA term', () => {
  expect(continuousErrorsSchema.safeParse({ kind: 'neweyWest', maxLags: null }).success).toBe(true)
  expect(continuousErrorsSchema.safeParse({ kind: 'arma', p: 2, q: 0, maxIter: 50 }).success).toBe(true)
  for (const bad of [
    { kind: 'arma', p: 0, q: 0, maxIter: 50 },
    { kind: 'arma', p: 13, q: 0, maxIter: 50 },
    { kind: 'arma', p: 1, q: 0, maxIter: 0 },
    { kind: 'arma', p: 1, q: 0, maxIter: 50, maxLags: 3 },
    { kind: 'neweyWest', maxLags: 3, p: 1 },
  ]) expect(continuousErrorsSchema.safeParse(bad).success).toBe(false)
  expect(linearErrorModelSchema.safeParse({ kind: 'neweyWest' }).success).toBe(true)
  expect(linearErrorModelSchema.safeParse({ kind: 'neweyWest', maxLags: 3 }).success).toBe(false)
  expect(interruptedModelSchema.safeParse({ kind: 'continuous', errors: { kind: 'arma', p: 1, q: 0, maxIter: 50 } }).success).toBe(true)
  expect(interruptedModelSchema.safeParse({ kind: 'continuous', hacMaxLags: null }).success).toBe(false)
})

test('a fitted error process must match the order it declares, and the request it answers', () => {
  const base = {
    kind: 'interruptedSeries', observations: 4, outcome: 0, interventionRow: 2, lag: 0, impact: { kind: 'level' }, seasonal: { kind: 'none' },
    terms: [term('const'), term('time'), term('step')],
    path: Array.from({ length: 4 }, () => ({ observed: 1, fitted: 1, counterfactual: 1, residual: 0 })),
    ljungBox: [], residualAcf: [], residualPacf: [], converged: true,
  }
  expect(parseInterruptedSeriesEvidence({ ...base, model: { kind: 'continuous', errors: arma } }).ok).toBe(true)
  expect(parseInterruptedSeriesEvidence({ ...base, model: { kind: 'continuous', errors: { ...arma, ma: [] } } }).ok).toBe(false)
  expect(parseInterruptedSeriesEvidence({ ...base, model: { kind: 'continuous', errors: { kind: 'neweyWest', maxLags: 2 } } }).ok).toBe(true)
  expect(sameErrors({ kind: 'arma', p: 1, q: 1, maxIter: 50 }, arma as never)).toBe(true)
  expect(sameErrors({ kind: 'arma', p: 2, q: 1, maxIter: 50 }, arma as never)).toBe(false)
  expect(sameErrors({ kind: 'neweyWest', maxLags: null }, { kind: 'neweyWest', maxLags: 3 })).toBe(true)
  expect(sameErrors({ kind: 'neweyWest', maxLags: 2 }, { kind: 'neweyWest', maxLags: 3 })).toBe(false)
  expect(describeErrors({ kind: 'arma', p: 1, q: 1, maxIter: 50 })).toBe('ARMA(1, 1) errors')
  expect(describeErrors({ kind: 'neweyWest', maxLags: null })).toBe('Newey–West errors')
})

test('the adjusted regression reads the coefficient the configured error treatment produced', () => {
  const { kind: _kind, ...fields } = arma
  const evidence = backdoorLinearEvidenceSchema.parse({
    kind: 'backdoorLinear', observations: 20, parameters: 3, treatment: 0, outcome: 1, adjustment: [2], level: 0.95,
    estimate: 1, standardError: 0.5, interval: [0, 2], degreesOfFreedom: 17, residualSd: 1, rSquared: 0.5,
    hacMaxLags: 2, hacStandardError: 0.6, hacInterval: [-0.2, 2.2], hacPValue: 0.1, durbinWatson: 1.9,
    errorModel: { kind: 'arma', estimate: 1.2, standardError: 0.4, interval: [0.4, 2.0], pValue: 0.01, errors: fields },
  })
  const level = 0.95 as const
  expect(linearReading({ configuration: { kind: 'backdoor-linear-regression', errors: { kind: 'classical' }, level }, evidence })).toEqual({ estimate: 1, standardError: 0.5, interval: [0, 2] })
  expect(linearReading({ configuration: { kind: 'backdoor-linear-regression', errors: { kind: 'hac' }, level }, evidence })).toEqual({ estimate: 1, standardError: 0.6, interval: [-0.2, 2.2] })
  expect(linearReading({ configuration: { kind: 'backdoor-linear-regression', errors: { kind: 'arma', p: 1, q: 1, maxIter: 50 }, level }, evidence })).toEqual({ estimate: 1.2, standardError: 0.4, interval: [0.4, 2.0] })
  const without = { ...evidence, errorModel: { kind: 'neweyWest' as const } }
  expect(linearReading({ configuration: { kind: 'backdoor-linear-regression', errors: { kind: 'arma', p: 1, q: 1, maxIter: 50 }, level }, evidence: without })).toBeNull()
  expect(backdoorLinearEvidenceSchema.safeParse({ ...without, errorModel: { kind: 'arma', errors: fields } }).success).toBe(false)
  expect(describeCovariance({ kind: 'arma', p: 2, q: 1, maxIter: 50 })).toBe('ARMA(2, 1) errors')
})
