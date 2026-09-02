import { expect, test } from '@playwright/test'
import { z } from 'zod'

test('requires sourced caveats for every registered method', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Static method catalog runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const methods = await import(new URL('/src/domain/methods.ts', window.location.href).href)
    return methods.METHOD_CATALOG.map((method: {
      id: string
      family: string
      caveats: readonly {
        id: string
        category: string
        requirement: string
        consequenceIfUnmet: string
        sources: readonly Record<string, unknown>[]
      }[]
    }) => ({
      id: method.id,
      family: method.family,
      caveats: method.caveats,
    }))
  })

  const source = z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('reference-implementation'), repository: z.string().min(1), revision: z.string().min(1), locator: z.string().min(1) }).strict(),
    z.object({ kind: z.literal('paper'), title: z.string().min(1), locator: z.string().min(1) }).strict(),
    z.object({ kind: z.literal('hirmos-constraint'), locator: z.string().min(1) }).strict(),
  ])
  const parsed = z.array(z.object({
    id: z.string().min(1),
    family: z.enum(['diagnostic', 'discovery', 'identification', 'estimation', 'refuter', 'counterfactual']),
    caveats: z.array(z.object({
      id: z.string().min(1),
      category: z.string().min(1),
      requirement: z.string().min(1),
      consequenceIfUnmet: z.string().min(1),
      sources: z.array(source).min(1),
    }).strict()).min(1),
  }).strict()).min(1).safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.map((method) => method.id)).toEqual([
    'adf',
    'kpss',
    'zivot-andrews',
    'granger-ssr-f',
    'count-series-intervention-scan',
    'pcmci-plus-parcorr',
    'lpcmci-parcorr',
    'dynotears',
    'direct-lingam',
    'var-lingam',
    'ocse',
    'backdoor-identification',
    'graphical-identification-id',
    'counterfactual-identification-id-star',
    'backdoor-linear-regression',
    'frontdoor-two-stage',
    'poisson-glm',
    'negative-binomial-p',
    'negative-binomial-ingarch',
    'causal-effects-total',
    'causal-impact',
    'dml-plr',
    'dml-irm',
    'dml-refutation-batch',
    'ardl-pss',
    'vecm',
    'synthetic-control',
    'panel-intervention',
    'negbin-nuts',
    'bayesian-gaussian',
    'discrete-bn-query',
    'binary-ett-idc-star',
    'linear-scm-counterfactual',
    'dynamic-linear-scm-counterfactual',
    'placebo-treatment-refuter',
    'data-subset-refuter',
    'random-common-cause-refuter',
    'unobserved-common-cause-sensitivity',
    'ljung-box',
    'shapiro-wilk',
    'pelt-change-points',
    'stl-decomposition',
  ])
  expect(parsed.data.filter((method) => method.family === 'refuter')).toHaveLength(5)
  for (const id of ['poisson-glm', 'negative-binomial-p', 'causal-effects-total', 'causal-impact']) {
    expect(parsed.data.find((method) => method.id === id)?.family).toBe('estimation')
  }
  const backdoor = parsed.data.find((method) => method.id === 'backdoor-identification')
  expect(backdoor?.family).toBe('identification')
  expect(backdoor?.caveats.some((caveat) => caveat.requirement.includes('never enter the adjustment set'))).toBe(true)
  const linear = parsed.data.find((method) => method.id === 'backdoor-linear-regression')
  expect(linear?.family).toBe('estimation')
  expect(linear?.caveats.some((caveat) => caveat.requirement.includes('HAC'))).toBe(true)
  const granger = parsed.data.find((method) => method.id === 'granger-ssr-f')
  expect(granger?.caveats.some((caveat) => caveat.requirement.includes('not an identified intervention effect'))).toBe(true)
  const pcmci = parsed.data.find((method) => method.id === 'pcmci-plus-parcorr')
  expect(pcmci?.caveats.some((caveat) => caveat.requirement.includes('no unmeasured common causes'))).toBe(true)
  const lpcmci = parsed.data.find((method) => method.id === 'lpcmci-parcorr')
  expect(lpcmci?.caveats.some((caveat) => caveat.requirement.includes('partial ancestral graph'))).toBe(true)
  const dynotears = parsed.data.find((method) => method.id === 'dynotears')
  expect(dynotears?.caveats.some((caveat) => caveat.requirement.includes('sparse linear dynamic'))).toBe(true)
  const varLingam = parsed.data.find((method) => method.id === 'var-lingam')
  expect(varLingam?.caveats.some((caveat) => caveat.requirement.includes('non-Gaussian errors'))).toBe(true)
  const directLingam = parsed.data.find((method) => method.id === 'direct-lingam')
  expect(directLingam?.caveats.some((caveat) => caveat.requirement.includes('independent and non-Gaussian'))).toBe(true)
  const ocse = parsed.data.find((method) => method.id === 'ocse')
  expect(ocse?.caveats.some((caveat) => caveat.requirement.includes('not identified intervention effects'))).toBe(true)

  const repositories = new Set(parsed.data.flatMap((method) => method.caveats.flatMap((caveat) => caveat.sources.flatMap((item) => item.kind === 'reference-implementation' ? [item.repository] : []))))
  for (const repository of ['statsmodels', 'tigramite', 'lingam', 'pyro', 'ruptures', 'pgmpy', 'econml', 'causationentropy', 'causalnex']) {
    expect(repositories.has(repository), `missing pinned implementation source: ${repository}`).toBe(true)
  }
  const nuts = parsed.data.find((method) => method.id === 'negbin-nuts')
  expect(nuts?.caveats.flatMap((caveat) => caveat.sources).some((item) => item.kind === 'paper' && item.title.includes('Betancourt'))).toBe(true)
  const countSources = parsed.data.filter((method) => method.id === 'poisson-glm' || method.id === 'negative-binomial-p').flatMap((method) => method.caveats.flatMap((caveat) => caveat.sources))
  expect(countSources.some((item) => item.kind === 'paper' && item.locator.includes('overdispersion in §3.4'))).toBe(true)
  const stl = parsed.data.find((method) => method.id === 'stl-decomposition')
  expect(stl?.caveats.flatMap((caveat) => caveat.sources).some((item) => item.kind === 'paper' && item.locator.includes('§4.3'))).toBe(true)
})
