import { expect, test } from '@playwright/test'

test('synthetic settings reuse schemas and reject absent column references', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { z } = await import(new URL('/node_modules/.vite/deps/zod.js', location.href).href)
    const { syntheticConfigurationSchema } = await import(new URL('/src/assistant/syntheticConfiguration.ts', location.href).href)
    const { readEstimatorConfiguration, canConfigureEstimator } = await import(new URL('/src/assistant/estimatorConfiguration.ts', location.href).href)
    const { defaultRidgeConfiguration } = await import(new URL('/src/domain/ridgeAugmented.ts', location.href).href)
    const { defaultPredictorSyntheticConfiguration } = await import(new URL('/src/domain/predictorSyntheticControl.ts', location.href).href)
    const columns = [{ id: 'donor', name: 'Donor' }, { id: 'predictor', name: 'Predictor' }]
    const history = { kind: 'synthetic-control', specification: 'outcome-history', start: { kind: 'row', row: 12 }, donors: ['donor'], crossFitFolds: 3, alpha: 0.05 }
    const ridge = { ...defaultRidgeConfiguration, treatedUnit: 'Texas', donorUnits: ['Ohio', 'Utah'], interventionPeriod: 10 }
    const predictors = { ...defaultPredictorSyntheticConfiguration(), treatedUnit: 'Texas', donorUnits: ['Ohio', 'Utah'], predictors: ['predictor'], predictorPeriods: [1, 2], fitPeriods: [1, 2], plotPeriods: [1, 2, 3], interventionPeriod: 3 }
    const good = [history, ridge, predictors]
    return {
      schema: !!z.toJSONSchema(syntheticConfigurationSchema, { io: 'input' }),
      accepted: good.map(c => readEstimatorConfiguration(c, columns).ok),
      editable: good.map(canConfigureEstimator),
      rejected: [
        { ...history, donors: ['missing'] },
        { ...history, donors: ['donor', 'donor'] },
        { ...history, crossFitFolds: 21 },
        { ...history, alpha: 1 },
        { ...history, start: { kind: 'row', row: 2 } },
        { ...predictors, predictors: ['missing'] },
        { ...predictors, special: [{ column: 'missing', periods: [1], summary: 'mean' }] },
        { ...ridge, regularization: { kind: 'made-up' } },
      ].map(c => readEstimatorConfiguration(c, columns).ok),
    }
  })
  expect(result).toEqual({ schema: true, accepted: [true, true, true], editable: [true, true, true], rejected: Array(8).fill(false) })
})
