import { expect, test } from '@playwright/test'
import { z } from 'zod'
import { stationarityBatterySchema } from '../src/domain/stationarity'

const browserOutcomeSchema = z.object({
  result: z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value: stationarityBatterySchema }).strict(),
    z.object({
      ok: z.literal(false),
      error: z.object({ kind: z.string(), detail: z.string() }).strict(),
    }).strict(),
  ]),
  detachedBytes: z.number().int().nonnegative(),
}).strict()

test('runs the stationarity battery in the Rust analysis worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture spike runs once')
  const externalRequests = new Set<string>()
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') externalRequests.add(url.href)
  })
  await page.goto('/')

  const raw: unknown = await page.evaluate(async () => {
    const moduleUrl = new URL('/src/analysis/client.ts', window.location.href).href
    const analysisModule: unknown = await import(moduleUrl)
    if (
      typeof analysisModule !== 'object'
      || analysisModule === null
      || !('runStationarityBattery' in analysisModule)
      || typeof analysisModule.runStationarityBattery !== 'function'
    ) {
      throw new Error('Analysis client did not expose runStationarityBattery.')
    }
    const values = Float64Array.from({ length: 120 }, (_, index) =>
      0.015 * index + Math.sin(index / 5) + 0.2 * Math.cos(index / 2.7),
    )
    const result: unknown = await analysisModule.runStationarityBattery(values)
    return { result, detachedBytes: values.byteLength }
  })

  const parsed = browserOutcomeSchema.safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.result.ok).toBe(true)
  if (!parsed.data.result.ok) return
  expect(parsed.data.detachedBytes).toBe(0)
  expect(parsed.data.result.value.observations).toBe(120)
  expect(parsed.data.result.value.adf.constant.observations).toBeGreaterThan(0)
  expect(parsed.data.result.value.zivotAndrews.levelAndTrend.breakIndex).toBeGreaterThan(0)
  expect([...externalRequests]).toEqual([])
})

test('surfaces a constant-series refusal from the Rust boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture spike runs once')
  await page.goto('/')
  const raw: unknown = await page.evaluate(async () => {
    const moduleUrl = new URL('/src/analysis/client.ts', window.location.href).href
    const analysisModule: unknown = await import(moduleUrl)
    if (
      typeof analysisModule !== 'object'
      || analysisModule === null
      || !('runStationarityBattery' in analysisModule)
      || typeof analysisModule.runStationarityBattery !== 'function'
    ) {
      throw new Error('Analysis client did not expose runStationarityBattery.')
    }
    return analysisModule.runStationarityBattery(new Float64Array(24).fill(1))
  })

  const parsed = z.object({
    ok: z.literal(false),
    error: z.object({
      kind: z.literal('kernel-refused'),
      detail: z.string(),
    }).strict(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (parsed.success) expect(parsed.data.error.detail).toContain('constant series')
})
