import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('sensitivity inspector distinguishes baseline covariates from DAG adjustment', async ({ page }, info) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ React, createRoot }, { SensitivityAdjustment }] = await Promise.all([
      load('/tests/support/reactRuntime.ts'), load('/src/components/sensitivity/SensitivityAdjustment.tsx'),
    ])
    const h = React.createElement
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;padding:24px;background:var(--color-panel)'
    document.body.append(host)
    const covariates = ['age', 'educ', 'black', 'married', 'nodegree', 'hisp', 're74']
    const run = { kind: 'panel-intervention-run', evidence: { kind: 'panelAdjusted' }, columns: ['re', 'nsw', ...covariates].map(name => ({ name })), estimate: { adjustment: { kind: 'none' } } }
    createRoot(host).render(h('section', { 'aria-label': 'Adjustment regression' },
      h('h2', null, 'DR DiD'), h('dl', { 'data-testid': 'adjusted' }, h(SensitivityAdjustment, { run })),
      h('h2', null, 'Unadjusted comparison'), h('dl', { 'data-testid': 'unadjusted' }, h(SensitivityAdjustment, { run: { ...run, columns: run.columns.slice(0, 2) } })),
      h('h2', null, 'DAG adjustment'), h('dl', { 'data-testid': 'dag' }, h(SensitivityAdjustment, { run: { kind: 'backdoor-linear-run', estimate: { adjustment: { kind: 'contemporaneous', variables: [{ name: 'age' }] } } } }))))
  })
  await expect(page.getByTestId('adjusted')).toHaveText('Baseline covariatesage, educ, black, married, nodegree, hisp, re74')
  await expect(page.getByTestId('unadjusted')).toHaveText('Baseline covariatesNone')
  await expect(page.getByTestId('dag')).toHaveText('Adjustment setage')
  await page.screenshot({ path: info.outputPath('sensitivity-adjustment.png') })
})

test('WASM probes use full sample and paired folds and preserve saved-run meaning', async ({ page }) => {
  test.setTimeout(180_000)
  await page.goto('/app')
  const oracle = JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/refute_dml.json', 'utf8'))
  const result = await page.evaluate(async data => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [analysis, sensitivity] = await Promise.all([load('/src/analysis/client.ts'), load('/src/domain/sensitivity.ts')])
    const rows = data.y.length
    const fit = await analysis.runDmlRefutationBatch(new Float64Array([...data.y, ...data.d, ...data.w1, ...data.w2]), rows, 4,
      { outcome: 0, treatment: 1, adjustment: [2, 3], model: 'irm', att: false, seed: 7 })
    if (!fit.ok) throw Error(JSON.stringify(fit.error))
    const { probeDesign, ...legacy } = fit.value
    return { evidence: fit.value, legacy: sensitivity.parseDmlRefutationEvidence(legacy), invalid: sensitivity.parseDmlRefutationEvidence({ ...fit.value, probeDesign: 'unknown' }) }
  }, oracle.data)
  expect(result.evidence.probeDesign).toBe('full-sample-paired-folds')
  expect(result.evidence.observations).toBe(1200)
  expect(result.evidence.mainEstimate).toBeCloseTo(oracle.aipw.effect, 3)
  expect(Math.abs(result.evidence.randomCommonCause.originalEffect - oracle.aipw.randomCommonCause.baseline)).toBeLessThan(0.003)
  expect(Math.abs(result.evidence.randomCommonCause.refutedEffect - oracle.aipw.randomCommonCause.refutedEffect)).toBeLessThan(0.003)
  expect(Math.abs(result.evidence.placebo.refutedEffect - oracle.aipw.placebo.refutedEffect)).toBeLessThan(0.003)
  expect(result.legacy.ok).toBe(true)
  expect(result.legacy.value.probeDesign).toBeUndefined()
  expect(result.invalid.ok).toBe(false)
})
