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
      estimate: 2, standardError: 0.25, interval: [1.51, 2.49], level: 0.95, groups: { kind: 'none' },
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

test('binds a conditional effect to the group effects the run reports, and refuses every other shape', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const methods = await import(new URL('/src/domain/methods.ts', window.location.href).href)
    const modifier = { node: 'season', column: 'season', name: 'season' }
    const study = {
      estimand: { kind: 'conditional-average-treatment-effect', scale: 'additive', modifier, grouping: { kind: 'quantiles', bins: 3 } },
      treatment: { node: 'price', column: 'price', name: 'price' },
      outcome: { node: 'sales', column: 'sales', name: 'sales' },
      graph: { nodes: [], laggedArrows: 0 },
    }
    const prepared = { kind: 'prepared-cross-section', observations: 300, columns: ['price', 'sales', 'season'] }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [modifier] } } }
    const configuration = estimation.defaultConfiguration('dml-plr', prepared, study)
    const group = (lower: number | null, upper: number | null, effect: number) => ({ lower, upper, observations: 100, effect, standardError: 0.3, interval: [effect - 0.6, effect + 0.6], fewObservations: false })
    const grouped = {
      kind: 'doubleMl', observations: 300, model: 'plr', att: false, treatBinary: false, seed: 7,
      estimate: 1.5, standardError: 0.2, interval: [1.1, 1.9], level: 0.95,
      groups: { kind: 'grouped', modifier: 2, grouping: { kind: 'quantiles', bins: 3 }, level: 0.95, groups: [group(null, 0.3, 0.8), group(0.3, 0.7, 1.4), group(0.7, null, 2.3)] },
    }
    const matched = estimation.causalEstimateFrom(study, identification, { kind: 'double-ml-run', configuration, evidence: grouped })
    const ungrouped = estimation.causalEstimateFrom(study, identification, { kind: 'double-ml-run', configuration, evidence: { ...grouped, groups: { kind: 'none' } } })
    const otherCut = estimation.causalEstimateFrom(study, identification, { kind: 'double-ml-run', configuration, evidence: { ...grouped, groups: { ...grouped.groups, grouping: { kind: 'levels' } } } })
    const linear = methods.methodDefinition(estimation.methodIdOf('backdoor-linear-regression'))
    const dml = methods.methodDefinition(estimation.methodIdOf('dml-plr'))
    if (!linear.ok || !dml.ok) throw new Error('method definitions missing')
    const context = { identification: identification.result, prepared, stationarity: null, document: null, study, panelPreflight: { kind: 'not-applicable' }, treatmentIsBinary: false, outcomeIsCount: false }
    return {
      matched,
      ungrouped,
      otherCut,
      linearRefused: estimation.evaluateEstimatorEligibility(linear.value, { ...context, configuration: estimation.defaultConfiguration('backdoor-linear-regression', prepared, study) }),
      dmlAllowed: estimation.evaluateEstimatorEligibility(dml.value, { ...context, configuration }),
    }
  })
  expect(result.matched?.effect).toMatchObject({ kind: 'byGroup', modifier: 'season', overall: 1.5 })
  expect(result.matched?.effect.groups.map((group: { label: string }) => group.label)).toEqual(['≤ 0.300', '0.300 to 0.700', '> 0.700'])
  expect(result.ungrouped).toBeNull()
  expect(result.otherCut).toBeNull()
  expect(result.linearRefused.kind).toBe('refused')
  expect(result.linearRefused.violations.map((entry: { caveat: { id: string } }) => entry.caveat.id)).toContain('estimand-target-compatibility')
  expect(result.dmlAllowed.kind).not.toBe('refused')
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

test('binds a per-row effect to the T-learner alone, and refuses it for an average target', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const methods = await import(new URL('/src/domain/methods.ts', window.location.href).href)
    const covariate = { node: 'w', column: 'w', name: 'w' }
    const perRow = {
      estimand: { kind: 'conditional-average-treatment-effect-per-row', scale: 'additive', modifiers: [] },
      treatment: { node: 'price', column: 'price', name: 'price' },
      outcome: { node: 'sales', column: 'sales', name: 'sales' },
      graph: { nodes: [], laggedArrows: 0 },
    }
    const average = { ...perRow, estimand: { kind: 'average-treatment-effect', scale: 'additive' } }
    const prepared = { kind: 'prepared-cross-section', observations: 4, columns: ['price', 'sales', 'w'] }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [covariate] } } }
    const configuration = estimation.defaultConfiguration('t-learner', prepared, perRow)
    const evidence = { kind: 'tLearner', observations: 4, controlRows: 2, treatedRows: 2, seed: 7, trees: 200, minLeaf: 5, effects: [1, 2, 3, 4], average: 2.5 }
    const bound = estimation.causalEstimateFrom(perRow, identification, { kind: 't-learner-run', configuration, evidence })
    const refused = estimation.causalEstimateFrom(average, identification, { kind: 't-learner-run', configuration, evidence })
    const tLearner = methods.methodDefinition(estimation.methodIdOf('t-learner'))
    const dml = methods.methodDefinition(estimation.methodIdOf('dml-plr'))
    if (!tLearner.ok || !dml.ok) throw new Error('method definitions missing')
    const context = { identification: identification.result, prepared, stationarity: null, document: null, study: perRow, panelPreflight: { kind: 'not-applicable' }, treatmentIsBinary: true, outcomeIsCount: false }
    return {
      bound,
      refused,
      summary: estimation.summariseRowEffects([1, 2, 3, 4]),
      defaultEstimator: estimation.defaultEstimatorFor(identification.result, prepared, perRow),
      dmlRefused: estimation.evaluateEstimatorEligibility(dml.value, { ...context, configuration: estimation.defaultConfiguration('dml-plr', prepared, perRow) }),
      tLearnerAllowed: estimation.evaluateEstimatorEligibility(tLearner.value, { ...context, configuration }),
    }
  })
  expect(result.bound?.effect).toMatchObject({ kind: 'perRow', overall: 2.5, effects: [1, 2, 3, 4] })
  expect(result.bound?.interval.kind).toBe('none')
  expect(result.refused).toBeNull()
  expect(result.summary).toMatchObject({ rows: 4, minimum: 1, median: 2.5, maximum: 4, positiveShare: 1 })
  expect(result.defaultEstimator).toBe('t-learner')
  expect(result.dmlRefused.kind).toBe('refused')
  expect(result.tLearnerAllowed.kind).not.toBe('refused')
})
