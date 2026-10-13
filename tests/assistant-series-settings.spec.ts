import { expect, test } from '@playwright/test'

test('series settings respect deterministic cases, lag lists and break support', async ({
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
    const ardl = { kind: 'ardl-pss', maxLag: 4, trend: 'c', case: 3 }
    const vecm = {
      kind: 'vecm',
      maxLags: 4,
      deterministic: 'ci',
      significance: 95,
      breakIndex: null,
    }
    const ingarch = {
      kind: 'negative-binomial-ingarch',
      link: 'identity',
      pastObservationLags: [1, 12],
      pastMeanLags: [1],
      horizon: 24,
      controlValue: 0,
      treatmentValue: 1,
      schedule: { kind: 'persistent' },
    }
    const good = [
      ardl,
      { ...ardl, trend: 'ct', case: 5 },
      vecm,
      ingarch,
      { ...ingarch, link: 'log', controlValue: -1 },
    ]
    const capability = {
      revision: 'test',
      inspection: {
        choices: [{ id: 'vecm', name: 'VECM', eligibility: { kind: 'eligible' } }],
        configuration: { current: vecm, columns: [], observations: 20 },
      },
      configure: () => {},
      evaluate: () => ({ kind: 'eligible' }),
    }
    return {
      accepted: good.map((c) => readEstimatorConfiguration(c, []).ok && canConfigureEstimator(c)),
      rejected: [
        { ...ardl, case: 4 },
        { ...ardl, trend: 'ct', case: 2 },
        { ...ardl, maxLag: 25 },
        { ...vecm, significance: 1 },
        { ...ingarch, pastObservationLags: [] },
        { ...ingarch, pastMeanLags: [2, 1] },
        { ...ingarch, pastMeanLags: [1, 1] },
        { ...ingarch, controlValue: -1 },
        { ...ingarch, horizon: 241 },
        { ...ingarch, schedule: { kind: 'decaying', delta: 1.1 } },
      ].map((c) => readEstimatorConfiguration(c, []).ok),
      breaks: [null, 4, 16, 17, 3].map(
        (breakIndex) =>
          proposeSelection(capability, {
            estimator: 'vecm',
            configuration: { ...vecm, breakIndex },
            reason: 'Review the break specification',
          }).ok,
      ),
    }
  })
  expect(result).toEqual({
    accepted: Array(5).fill(true),
    rejected: Array(10).fill(false),
    breaks: [true, true, true, false, false],
  })
})
