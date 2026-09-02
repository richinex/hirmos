import { expect, test } from '@playwright/test'

test('locks DML-IRM to the study target and refuses a mismatched result contract', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const baseStudy = {
      estimand: { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 },
    }
    const prepared = { kind: 'prepared-cross-section', columns: [] }
    const configuration = estimation.defaultConfiguration('dml-irm', prepared, baseStudy)
    const study = { ...baseStudy }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [] } } }
    const evidence = {
      kind: 'doubleMl', observations: 100, model: 'irm', att: true, treatBinary: true, seed: 7,
      estimate: 2, standardError: 0.25, interval: [1.51, 2.49], level: 0.95,
    }
    const matched = estimation.causalEstimateFrom(study, identification, { kind: 'double-ml-run', configuration, evidence })
    const mismatched = estimation.causalEstimateFrom(study, identification, {
      kind: 'double-ml-run',
      configuration: { ...configuration, att: false },
      evidence: { ...evidence, att: false },
    })
    return { configuration, matched, mismatched }
  })
  expect(result.configuration).toMatchObject({ kind: 'dml-irm', att: true })
  expect(result.matched?.estimand.kind).toBe('average-treatment-effect-on-treated')
  expect(result.mismatched).toBeNull()
})

test('binds a binary ETT estimate to the IDC* expressions recorded by identification', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const treatedExpression = 'P(Y_{X=1} = 1 | X = 1)'
    const untreatedExpression = 'P(Y_{X=0} = 1 | X = 1)'
    const study = { estimand: { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 } }
    const identification = {
      result: { kind: 'counterfactually-identified', treatedExpression, untreatedExpression },
    }
    const configuration = { kind: 'binary-ett-idc-star' }
    const evidence = {
      kind: 'binaryEtt', observations: 100, treatment: 'X', outcome: 'Y',
      treatedPotentialOutcomeMean: 0.70, untreatedPotentialOutcomeMean: 0.45,
      effectOnTreated: 0.25, treatedExpression, untreatedExpression,
    }
    const matched = estimation.causalEstimateFrom(study, identification, { kind: 'binary-ett-run', configuration, evidence })
    const mismatched = estimation.causalEstimateFrom(study, identification, {
      kind: 'binary-ett-run', configuration, evidence: { ...evidence, untreatedExpression: 'different expression' },
    })
    return { matched, mismatched }
  })
  expect(result.matched).toMatchObject({
    estimand: { kind: 'average-treatment-effect-on-treated' },
    effect: { kind: 'additive', value: 0.25 },
    interval: { kind: 'none' },
  })
  expect(result.mismatched).toBeNull()
})

test('records the time-indexed adjustment rows actually fitted by CausalEffects', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const study = {
      estimand: { kind: 'average-treatment-effect', scale: 'additive' },
      graph: {
        nodes: [
          { node: 'kms', column: 'kms', name: 'kms' },
          { node: 'deaths', column: 'deaths', name: 'DriversKilled' },
          { node: 'price', column: 'price', name: 'PetrolPrice' },
        ],
      },
    }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [] } } }
    const configuration = { kind: 'causal-effects-total', estimator: { kind: 'linear', adjustment: { kind: 'optimal' } }, treatmentLag: 2, interventions: [0, 1], uncertainty: { kind: 'none' } }
    const evidence = {
      kind: 'causalEffectsTotal', observations: 192, tauMax: 4, noCausalPath: false, identifiable: true,
      mediators: [], fit: { kind: 'adjustedLinear', selection: { kind: 'optimal' }, adjustmentSet: [[1, -1], [2, -1]] },
      interventions: [0, 1], predictions: [100, 95], totalEffect: -5, fittedObservations: 188, uncertainty: { kind: 'none' },
    }
    const graphVariables = study.graph.nodes
    const estimate = estimation.causalEstimateFrom(study, identification, { kind: 'causal-effects-run', configuration, evidence, graphVariables })
    const invalid = estimation.causalEstimateFrom(study, identification, {
      kind: 'causal-effects-run', configuration, evidence: { ...evidence, fit: { kind: 'adjustedLinear', selection: { kind: 'optimal' }, adjustmentSet: [[9, -1]] } }, graphVariables,
    })
    const mismatchedSelection = estimation.causalEstimateFrom(study, identification, {
      kind: 'causal-effects-run', configuration, evidence: { ...evidence, fit: { kind: 'adjustedLinear', selection: { kind: 'minimizedOptimal' }, adjustmentSet: [[1, -1]] } }, graphVariables,
    })
    return { estimate, labels: estimate === null ? [] : estimation.adjustmentLabels(estimate.adjustment), invalid, mismatchedSelection }
  })
  expect(result.estimate?.adjustment).toEqual({
    kind: 'time-indexed',
    variables: [
      { variable: { node: 'deaths', column: 'deaths', name: 'DriversKilled' }, lag: -1 },
      { variable: { node: 'price', column: 'price', name: 'PetrolPrice' }, lag: -1 },
    ],
  })
  expect(result.labels).toEqual(['DriversKilled (t−1)', 'PetrolPrice (t−1)'])
  expect(result.invalid).toBeNull()
  expect(result.mismatchedSelection).toBeNull()
})

test('unlocks estimation for every identified result and not for an identification failure', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const study = await import(new URL('/src/domain/study.ts', window.location.href).href)
    return {
      backdoor: study.identificationAllowsEstimation('identified'),
      levelTwo: study.identificationAllowsEstimation('graphically-identified'),
      levelThree: study.identificationAllowsEstimation('counterfactually-identified'),
      failure: study.identificationAllowsEstimation('backdoor-not-identified'),
    }
  })
  expect(result).toEqual({ backdoor: true, levelTwo: true, levelThree: true, failure: false })
})
