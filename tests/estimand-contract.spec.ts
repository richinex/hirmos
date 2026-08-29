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
