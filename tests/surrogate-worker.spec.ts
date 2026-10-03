import { expect, test } from '@playwright/test'
import { surrogateRequestSchema, surrogateEvidenceSchema, surrogateEvidenceMatchesRequest } from '../src/domain/surrogate'

const request = {
  experimental: { surrogates: [[0], [1], [2], [3]], treatment: [0, 1, 0, 1] },
  observational: { surrogates: [[0], [1], [2], [3], [4]], outcome: [1, 3, 5, 7, 9] },
  adjustment: { kind: 'none' }, estimator: 'index', uncertainty: { kind: 'none' },
} as const

test('surrogate sample contract rejects incompatible and incomplete samples', () => {
  expect(surrogateRequestSchema.safeParse(request).success).toBe(true)
  const invalid = [
    { ...request, experimental: { surrogates: [], treatment: [] } },
    { ...request, experimental: { ...request.experimental, treatment: [0, 0, 0, 0] } },
    { ...request, experimental: { ...request.experimental, treatment: [0, 2, 0, 1] } },
    { ...request, observational: { ...request.observational, outcome: [1] } },
    { ...request, observational: { ...request.observational, surrogates: [[0, 1], [1], [2], [3], [4]] } },
    { ...request, adjustment: { kind: 'baseline', experimental: [[1]] } },
    { ...request, adjustment: { kind: 'baseline', experimental: [], observational: [] } },
    { ...request, uncertainty: { kind: 'bootstrap', repetitions: 1, seed: 0 } },
    { ...request, uncertainty: { kind: 'bootstrap', repetitions: 10, seed: 2 ** 32 } },
    { ...request, unknown: true },
  ]
  for (const value of invalid) expect(surrogateRequestSchema.safeParse(value).success).toBe(false)
})

test('surrogate evidence must preserve estimator, dimensions and bootstrap settings', () => {
  const r = surrogateRequestSchema.parse(request)
  const e = surrogateEvidenceSchema.parse({ estimator: 'index', experimentalRows: 4, observationalRows: 5,
    surrogateColumns: 1, baselineColumns: 0, estimate: 2, uncertainty: { kind: 'none' } })
  expect(surrogateEvidenceMatchesRequest(e, r)).toBe(true)
  expect(surrogateEvidenceMatchesRequest({ ...e, estimator: 'score' }, r)).toBe(false)
  expect(surrogateEvidenceMatchesRequest({ ...e, experimentalRows: 5 }, r)).toBe(false)
  const b = surrogateRequestSchema.parse({ ...request, uncertainty: { kind: 'bootstrap', repetitions: 20, seed: 7 } })
  expect(surrogateEvidenceMatchesRequest(e, b)).toBe(false)
  const se = surrogateEvidenceSchema.parse({ ...e, uncertainty: { kind: 'bootstrapStandardError', standardError: 0.2, repetitions: 20, seed: 7 } })
  expect(surrogateEvidenceMatchesRequest(se, b)).toBe(true)
  expect(surrogateEvidenceMatchesRequest(se, { ...b, uncertainty: { kind: 'bootstrap', repetitions: 20, seed: 8 } })).toBe(false)
})

test('surrogate index executes through the browser client and WASM worker', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async model => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    return analysis.runSurrogate(model)
  }, request)
  expect(result.ok, JSON.stringify(result)).toBe(true)
  expect(result.value.estimate).toBeCloseTo(2, 10)
  expect(result.value.experimentalRows).toBe(4)
  expect(result.value.observationalRows).toBe(5)
  expect(result.value.uncertainty.kind).toBe('none')
})

test('all surrogate estimators match the pinned oracle through the browser worker', async ({ page }) => {
  const { readFileSync } = await import('node:fs')
  const fixture = JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/surrogate-index/fixtures.json', import.meta.url), 'utf8'))
  await page.goto('/app')
  for (const [name, c] of Object.entries(fixture.cases) as [string, { base: Record<string, any> }][]) {
    const b = c.base
    const baselineColumns = b.propensity_design?.[0]?.length - 1 || 0
    const predictors = (key: string) => b[key].map((row: number[]) => row.slice(1, row.length - baselineColumns))
    const baseline = (key: string) => b[key].map((row: number[]) => row.slice(1))
    for (const [estimator, expected] of [['index', b.effect], ['score', b.tight_score_effect], ['influenceFunction', b.paper_influence.effect]] as const) {
      const model = surrogateRequestSchema.parse({
        experimental: { surrogates: predictors('experimental'), treatment: b.treatment },
        observational: { surrogates: predictors('observational'), outcome: b.outcome },
        adjustment: baselineColumns === 0 ? { kind: 'none' } : { kind: 'baseline', experimental: baseline('propensity_design'), observational: baseline('observational_baseline') },
        estimator, uncertainty: { kind: 'none' },
      })
      const result = await page.evaluate(async input => {
        const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
        return analysis.runSurrogate(input)
      }, model)
      expect(result.ok, `${name}/${estimator}: ${JSON.stringify(result)}`).toBe(true)
      expect(Math.abs(result.value.estimate - expected), `${name}/${estimator}`).toBeLessThanOrEqual(1e-9 * (1 + Math.abs(expected)))
      expect(surrogateEvidenceMatchesRequest(surrogateEvidenceSchema.parse(result.value), model)).toBe(true)
    }
  }
})
