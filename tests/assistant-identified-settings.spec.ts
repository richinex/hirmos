import { expect, test } from '@playwright/test'

test('identified estimator settings preserve bounds and validate temporal graph references', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readEstimatorConfiguration, canConfigureEstimator } = await import(
      new URL('/src/assistant/estimatorConfiguration.ts', location.href).href
    )
    const { proposeSelection } = await import(
      new URL('/src/assistant/applicationTools.tsx', location.href).href
    )
    const bootstrap = { simulations: 399, sampleSizeFraction: 1, level: 0.95, seed: 7 }
    const frontdoor = { kind: 'frontdoor-two-stage', interventions: [0, 1], ...bootstrap }
    const iv = { kind: 'instrumental-variable', ...bootstrap }
    const gaussian = { kind: 'bayesian-gaussian', warmup: 100, samples: 200, seed: 3 }
    const temporal = {
      kind: 'causal-effects-total',
      estimator: { kind: 'linear', adjustment: { kind: 'optimal' } },
      treatmentLag: 1,
      interventions: [0, 1],
      uncertainty: { kind: 'none' },
    }
    const valid = [frontdoor, iv, gaussian, { ...gaussian, kind: 'negbin-nuts' }, temporal]
    const capability = {
      revision: 'test',
      inspection: {
        choices: [
          { id: temporal.kind, name: 'Temporal effect', eligibility: { kind: 'eligible' } },
        ],
        configuration: {
          current: temporal,
          columns: [],
          temporal: { candidates: [{ index: 2, name: 'X' }], graphLag: 1 },
        },
      },
      configure: () => {},
      evaluate: () => ({ kind: 'eligible' }),
    }
    const explicit = (nodes: number[][]) =>
      proposeSelection(capability, {
        estimator: temporal.kind,
        configuration: {
          ...temporal,
          estimator: { kind: 'linear', adjustment: { kind: 'explicit', nodes } },
        },
        reason: 'Review adjustment members',
      }).ok
    return {
      accepted: valid.map((c) => readEstimatorConfiguration(c, []).ok),
      editable: valid.map(canConfigureEstimator),
      rejected: [
        { ...frontdoor, interventions: [0, Infinity] },
        { ...iv, interventions: [0, 2] },
        { ...iv, simulations: 19 },
        { ...iv, sampleSizeFraction: 0.5 },
        { ...gaussian, samples: 5001 },
        { ...gaussian, priors: {} },
        { ...temporal, treatmentLag: 21 },
      ].map((c) => readEstimatorConfiguration(c, []).ok),
      explicit: [
        explicit([]),
        explicit([[2, -2]]),
        explicit([[0, -1]]),
        explicit([[2, -3]]),
        explicit([
          [2, 0],
          [2, 0],
        ]),
      ],
    }
  })
  expect(result).toEqual({
    accepted: Array(5).fill(true),
    editable: Array(5).fill(true),
    rejected: Array(7).fill(false),
    explicit: [true, true, false, false, false],
  })
})
