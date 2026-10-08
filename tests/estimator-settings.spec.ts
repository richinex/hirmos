import { test, expect } from '@playwright/test'

test('estimator settings read as labelled words in a fixed order', async ({ page }) => {
  await page.goto('/app')
  const r = await page.evaluate(async () => {
    const { configurationSettings, DEFAULT_BOOSTED_GRID } = await import(
      new URL('/src/domain/estimation.ts', location.href).href
    )
    const { DEFAULT_CAUSAL_FOREST } = await import(
      new URL('/src/domain/causalForest.ts', location.href).href
    )
    const { defaultRidgeConfiguration } = await import(
      new URL('/src/domain/ridgeAugmented.ts', location.href).href
    )
    const { defaultStaggeredSpecification } = await import(
      new URL('/src/domain/staggeredDid.ts', location.href).href
    )
    const pairs = (configuration: unknown) =>
      configurationSettings(configuration).map((s: { label: string; value: string }) => ({
        label: s.label,
        value: s.value,
      }))
    const linear = {
      kind: 'backdoor-linear-regression',
      errors: { kind: 'classical' },
      fixedEffects: { kind: 'none' },
      level: 0.95,
    }
    const representative = [
      linear,
      { ...linear, errors: { kind: 'arma', p: 1, q: 1, maxIter: 200 } },
      {
        kind: 'propensity-weighting',
        model: 'boosted',
        maxIter: 1000,
        boosted: DEFAULT_BOOSTED_GRID,
        scale: 'inverseProbability',
        uncertainty: { kind: 'bootstrap', rounds: 200, seed: 3, level: 0.9 },
      },
      DEFAULT_CAUSAL_FOREST,
      defaultRidgeConfiguration,
      {
        kind: 'panel-intervention',
        primary: 'staggered',
        covariates: ['2:income'],
        specification: defaultStaggeredSpecification,
      },
      {
        kind: 'causal-effects-total',
        estimator: { kind: 'knn', k: 15, adjustment: { kind: 'optimal' } },
        treatmentLag: 1,
        interventions: [0, 1],
        uncertainty: {
          kind: 'bootstrap',
          samples: 100,
          blockLength: { kind: 'cubeRoot' },
          confidenceLevel: 0.9,
          seed: 4,
        },
      },
      {
        kind: 'causal-impact',
        start: { kind: 'from-treatment' },
        window: { kind: 'through-last-row' },
        controls: ['3:price', '4:volume'],
        inference: {
          kind: 'structural',
          model: {
            version: 'gaussian-components-v1',
            trend: 'linear',
            seasonality: { kind: 'seasonal', seasons: 7, duration: 1 },
          },
          draws: 1000,
          warmup: 500,
          seed: 1,
        },
      },
      { kind: 't-learner', model: { kind: 'forest', seed: 7, uncertainty: { kind: 'none' } } },
      { kind: 'sharp-rd' },
    ]
    return {
      linear: pairs(linear),
      values: representative.flatMap((c) => pairs(c).map((s: { value: string }) => s.value)),
      labels: representative.flatMap((c) => pairs(c).map((s: { label: string }) => s.label)),
      staggeredCovariates: pairs(representative[5]).find(
        (s: { label: string }) => s.label === 'Adjustment covariates',
      ),
      sharp: pairs({ kind: 'sharp-rd' }),
    }
  })
  expect(r.linear).toEqual([
    { label: 'Standard errors', value: 'Classical' },
    { label: 'Fixed effects', value: 'None' },
    { label: 'Confidence level', value: '95%' },
  ])
  expect(r.values.length).toBeGreaterThan(40)
  for (const value of r.values) {
    expect(value).not.toMatch(/[{}"·]/)
    expect(value).not.toMatch(/[a-z][A-Z]/)
    expect(value).not.toBe('')
  }
  for (const label of r.labels) {
    expect(label).toMatch(/^[A-Z]/)
    expect(label).not.toMatch(/[a-z][A-Z]/)
  }
  expect(r.staggeredCovariates).toEqual({ label: 'Adjustment covariates', value: 'income' })
  expect(r.sharp).toEqual([])
})
