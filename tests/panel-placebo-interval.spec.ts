import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('synthetic DiD uses the placebo SE, not the in-time placebo, for its interval', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { causalEstimateFrom } = await import(new URL('/src/domain/estimation.ts', location.href).href)
    const study = { estimand: { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 } }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [] } } }
    const run = {
      kind: 'panel-intervention-run',
      configuration: { kind: 'panel-intervention', primary: 'syntheticDid', placeboReplications: 100, seed: 0 },
      evidence: {
        kind: 'panelIntervention', observations: 120,
        syntheticDid: { estimate: -0.35, lambda: [0.5, 0.5], omega: [0.5, 0.5] },
        syntheticDidPlacebo: { kind: 'available', standardError: 0.215 },
        syntheticDidInTime: { kind: 'available', estimate: 99 },
      },
    }
    return {
      available: causalEstimateFrom(study, identification, run),
      unavailable: causalEstimateFrom(study, identification, {
        ...run,
        evidence: { ...run.evidence, syntheticDidPlacebo: { kind: 'unavailable', reason: 'Too few controls' } },
      }),
    }
  })
  expect(result.available.interval.kind).toBe('confidence')
  expect(result.available.interval.level).toBe(0.95)
  expect(result.available.interval.lower).toBeCloseTo(-0.7714, 12)
  expect(result.available.interval.upper).toBeCloseTo(0.0714, 12)
  expect(result.available.standardError).toBe(0.215)
  expect(result.unavailable.interval.kind).toBe('none')
  expect(result.unavailable.standardError).toBeNull()
})

test('a refused simultaneous band offers an explicit retry preserving bootstrap settings', async ({ page }, info) => {
  await page.addInitScript(() => {
    const NativeWorker = window.Worker
    const attempts: unknown[] = []
    Object.assign(window, { didAttempts: attempts })
    window.Worker = class extends NativeWorker {
      postMessage(message: any, options?: any) {
        if (message.kind === 'staggered-did') {
          attempts.push(message.model.specification.inference)
          const detail = message.model.specification.inference.kind === 'bootstrapSimultaneous'
            ? 'The estimated simultaneous critical value is below the pointwise critical value. No interval was substituted.'
            : 'Pointwise request received by test worker.'
          queueMicrotask(() => this.dispatchEvent(new MessageEvent('message', {
            data: { kind: 'analysis-failed', request: message.request, problem: { kind: 'kernel-refused', detail } },
          })))
          return
        }
        super.postMessage(message, options)
      }
    }
  })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Staggered adoption', exact: true }).check()
  await page.getByRole('button', { name: 'Run panel DiD', exact: true }).click()
  const retry = page.getByRole('button', { name: 'Run with pointwise bootstrap', exact: true })
  await expect(retry).toBeVisible()
  expect(await page.evaluate(() => (window as any).didAttempts)).toEqual([
    { kind: 'bootstrapSimultaneous', iterations: 999, seed: 731 },
  ])
  await retry.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('explicit-retry.png') })
  await retry.click()
  await expect(page.getByText('Pointwise request received by test worker.', { exact: true })).toBeVisible()
  await expect(retry).toHaveCount(0)
  await expect(page.getByRole('radio', { name: 'Pointwise bootstrap', exact: true })).toBeChecked()
  expect(await page.evaluate(() => (window as any).didAttempts)).toEqual([
    { kind: 'bootstrapSimultaneous', iterations: 999, seed: 731 },
    { kind: 'bootstrapPointwise', iterations: 999, seed: 731 },
  ])
})
