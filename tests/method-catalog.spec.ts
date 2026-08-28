import { expect, test } from '@playwright/test'
import { z } from 'zod'

test('requires sourced caveats for every registered method', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Static method catalog runs once')
  await page.goto('/')
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
        sources: readonly { kind: string }[]
      }[]
    }) => ({
      id: method.id,
      family: method.family,
      caveats: method.caveats,
    }))
  })

  const parsed = z.array(z.object({
    id: z.string().min(1),
    family: z.enum(['diagnostic', 'discovery', 'identification', 'estimation', 'refuter']),
    caveats: z.array(z.object({
      id: z.string().min(1),
      category: z.string().min(1),
      requirement: z.string().min(1),
      consequenceIfUnmet: z.string().min(1),
      sources: z.array(z.object({ kind: z.string().min(1) }).passthrough()).min(1),
    }).strict()).min(1),
  }).strict()).min(1).safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.map((method) => method.id)).toEqual([
    'adf',
    'kpss',
    'zivot-andrews',
    'granger-ssr-f',
    'pcmci-plus-parcorr',
  ])
  const granger = parsed.data.find((method) => method.id === 'granger-ssr-f')
  expect(granger?.caveats.some((caveat) => caveat.requirement.includes('not intervention causality'))).toBe(true)
  const pcmci = parsed.data.find((method) => method.id === 'pcmci-plus-parcorr')
  expect(pcmci?.caveats.some((caveat) => caveat.requirement.includes('no hidden common causes'))).toBe(true)
})
