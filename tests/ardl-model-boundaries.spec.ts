import { expect, test } from '@playwright/test'
import { ardlModelRequestSchema, ardlModelResponseSchema, ardlModelEvidenceSchema } from '../src/domain/ardlModel'

const request = {
  outcome: 0, predictors: [1, 2], fixed: [3], terms: 'constant',
  orders: { kind: 'fixed', outcomeLag: 2, predictorLags: [1, 2] },
  holdBack: 2, multiplierHorizon: 12, future: { kind: 'none' },
}

test('ARDL requests keep column roles and lag specifications consistent', () => {
  expect(ardlModelRequestSchema.safeParse(request).success).toBe(true)
  for (const change of [
    { predictors: [0, 2] }, { fixed: [2] }, { predictors: [1, 1] },
    { holdBack: 1 }, { orders: { kind: 'fixed', outcomeLag: 2, predictorLags: [1] } },
    { orders: { kind: 'search', maximumLag: 2, maximumOrders: [1] } },
  ]) expect(ardlModelRequestSchema.safeParse({ ...request, ...change }).success).toBe(false)
})

test('R searches require complete orders and a common sample', () => {
  const horizontal = {kind:'rHorizontal',maximum:[6,6,6],fixed:[null,1,null],starting:[5,5,5]}
  const base = {...request,holdBack:8,orders:horizontal}
  expect(ardlModelRequestSchema.safeParse(base).success).toBe(true)
  for (const orders of [
    {...horizontal,maximum:[6,6]}, {...horizontal,fixed:[null,1]}, {...horizontal,starting:[5,5]},
    {...horizontal,starting:[0,5,5]}, {...horizontal,starting:[7,5,5]}, {...horizontal,fixed:[0,1,null]},
    {...horizontal,fixed:[null,7,null]},
  ]) expect(ardlModelRequestSchema.safeParse({...base,orders}).success).toBe(false)
  expect(ardlModelRequestSchema.safeParse({...base,holdBack:null}).success).toBe(false)
  const grid = {kind:'rGrid',minimumLag:1,maximumLag:6,maximumOrders:[6,6],fixedOrders:[1,null]}
  expect(ardlModelRequestSchema.safeParse({...base,orders:grid}).success).toBe(true)
  for (const orders of [{...grid,minimumLag:7},{...grid,fixedOrders:[1]},{...grid,fixedOrders:[7,null]}]) expect(ardlModelRequestSchema.safeParse({...base,orders}).success).toBe(false)
  expect(ardlModelRequestSchema.safeParse({...base,orders:{kind:'rFixed',outcomeLag:1,predictorLags:[0,0]}}).success).toBe(true)
  expect(ardlModelRequestSchema.safeParse({...base,orders:{kind:'rFixed',outcomeLag:0,predictorLags:[0,0]}}).success).toBe(false)
})

test('a forecast scenario requires complete aligned future columns', () => {
  const future = { kind: 'scenario', confidence: 0.95, predictors: [[1, 2], [3, 4]], fixed: [[0, 1]] }
  expect(ardlModelRequestSchema.safeParse({ ...request, future }).success).toBe(true)
  for (const change of [
    { fixed: [] }, { predictors: [[1, 2]] }, { predictors: [[1], [3, 4]] },
    { predictors: [[NaN, 2], [3, 4]] }, { confidence: 1 },
  ]) expect(ardlModelRequestSchema.safeParse({ ...request, future: { ...future, ...change } }).success).toBe(false)
  expect(ardlModelRequestSchema.safeParse({ ...request, future: { kind: 'none', predictors: [[1]] } }).success).toBe(false)
})

// Small display-boundary fixture; these values make no statistical parity claim.
const evidence = {
  observations: 4, startRow: 1, fittedRows: 3, outcomeLag: 1, predictorLags: [0],
  coefficients: [{ kind: 'constant' }, { kind: 'outcome', lag: 1 }, { kind: 'predictor', column: 0, lag: 0 }],
  params: [1, 0.2, 0.3], covariance: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
  observed: [1, 2, 3, 4], fitted: [2, 3, 4],
  longRun: { kind: 'unavailable', reason: 'zeroOrder' }, multipliers: { kind: 'unavailable', reason: 'numericalFailure' },
  forecast: { kind: 'recorded', confidence: 0.95, mean: [5], variance: [1], interval: [[3, 7]] },
}

test('ARDL transport rejects absent evidence and incorrect result kinds', () => {
  expect(ardlModelResponseSchema.safeParse({ kind: 'ardlModel', evidence }).success).toBe(true)
  for (const value of [null, {}, { kind: 'ardlModel' }, { kind: 'vecm', evidence }, evidence]) {
    expect(ardlModelResponseSchema.safeParse(value).success).toBe(false)
  }
})

test('forecast intervals and fitted sample dimensions must agree with their results', () => {
  expect(ardlModelEvidenceSchema.safeParse(evidence).success).toBe(true)
  for (const change of [
    { fittedRows: 2 }, { covariance: [[1]] }, { observed: [1, 2] },
    { forecast: { ...evidence.forecast, interval: [[6, 7]] } },
    { forecast: { ...evidence.forecast, variance: [-1] } },
    { forecast: { ...evidence.forecast, mean: [5, 6] } },
  ]) expect(ardlModelEvidenceSchema.safeParse({ ...evidence, ...change }).success).toBe(false)
})
