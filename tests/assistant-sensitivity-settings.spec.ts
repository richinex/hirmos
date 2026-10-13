import { expect, test } from '@playwright/test'

test('DiD sensitivity proposals cannot replace source estimates or fit settings', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { honestSettingsSchema } = await import(
      new URL('/src/domain/honestDidSettings.ts', location.href).href
    )
    const { defaultHonestConfiguration } = await import(
      new URL('/src/domain/honestDid.ts', location.href).href
    )
    const { didSensitivitySettingsSchema } = await import(
      new URL('/src/domain/didSensitivitySettings.ts', location.href).href
    )
    const honest = {
      sourceRun: 'saved-event-study',
      configuration: defaultHonestConfiguration,
      target: { kind: 'average' },
    }
    const did = {
      sourceRun: 'saved-did',
      scenarios: [[0.02, 0.05]],
      rho: 1,
      level: 0.95,
      null: 0,
      outcomeMaximum: 0.15,
      rieszMaximum: 0.2,
      points: 41,
    }
    return {
      honest: honestSettingsSchema.safeParse(honest).success,
      injectedEstimates: honestSettingsSchema.safeParse({
        ...honest,
        eventStudy: { coefficients: [1] },
      }).success,
      did: didSensitivitySettingsSchema.safeParse(did).success,
      changedFolds: didSensitivitySettingsSchema.safeParse({ ...did, folds: 10 }).success,
      invalidShare: didSensitivitySettingsSchema.safeParse({ ...did, scenarios: [[1, 0.05]] })
        .success,
      emptyWeights: honestSettingsSchema.safeParse({
        ...honest,
        target: { kind: 'specified', weights: [] },
      }).success,
    }
  })
  expect(result).toEqual({
    honest: true,
    injectedEstimates: false,
    did: true,
    changedFolds: false,
    invalidShare: false,
    emptyWeights: false,
  })
})

test('sensitivity settings preserve other probes and enforce numerical bounds', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { sensitivitySettingsSchema } = await import(
      new URL('/src/domain/sensitivitySettings.ts', location.href).href
    )
    const { initialSensitivityDraft, stepSensitivityDraft } = await import(
      new URL('/src/domain/sensitivityDraft.ts', location.href).href
    )
    const { probeEligibility } = await import(
      new URL('/src/domain/sensitivity.ts', location.href).href
    )
    const before = initialSensitivityDraft([], [])
    const configuration = {
      kind: 'unobserved-confounding',
      seed: 7,
      kappaT: { kind: 'range', from: 0, to: 1, steps: 20 },
      kappaY: { kind: 'inferred' },
    }
    const settings = { estimationRun: 'source-run', configuration }
    const parsed = sensitivitySettingsSchema.parse(settings)
    const after = stepSensitivityDraft(before, {
      type: 'settings-applied',
      run: parsed.estimationRun,
      configuration: parsed.configuration,
    })
    return {
      selected: after.estimationRun,
      probe: after.probe,
      saved: after.configurations['unobserved-confounding'],
      preserved:
        after.configurations['linear-refutation'] === before.configurations['linear-refutation'] &&
        after.configurations['dml-refutation'] === before.configurations['dml-refutation'],
      invalidStrength: sensitivitySettingsSchema.safeParse({
        ...settings,
        configuration: { ...configuration, kappaT: { kind: 'range', from: 0, to: 2, steps: 20 } },
      }).success,
      invalidSeed: sensitivitySettingsSchema.safeParse({
        ...settings,
        configuration: { kind: 'dml-refutation', seed: -1 },
      }).success,
      missing: probeEligibility('linear-refutation', null, null).kind,
      wrongMethod: probeEligibility('linear-refutation', { kind: 'double-ml-run' }, null).kind,
    }
  })
  expect(result.selected).toBe('source-run')
  expect(result.probe).toBe('unobserved-confounding')
  expect(result.saved.seed).toBe(7)
  expect(result.preserved).toBe(true)
  expect(result.invalidStrength).toBe(false)
  expect(result.invalidSeed).toBe(false)
  expect(result.missing).toBe('refused')
  expect(result.wrongMethod).toBe('refused')
})
