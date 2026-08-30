import { expect, test } from '@playwright/test'
import { z } from 'zod'
import { stationarityBatterySchema } from '../src/domain/stationarity'
import { dagCheckEvidenceSchema } from '../src/domain/dagValidation'

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

test('rejects impossible Holm evidence at the TypeScript boundary', () => {
  const parsed = dagCheckEvidenceSchema.safeParse({
    kind: 'dagCheck',
    observations: 100,
    significanceLevel: 0.05,
    correction: 'holm',
    implications: [{ x: 0, y: 1, given: [], pValue: 0.2, adjustedPValue: 0.1, observations: 100, decision: 'notRefuted' }],
    uniformity: { statistic: 0.2, pValue: 0.7, tests: 1 },
    falsification: { kind: 'skipped', reason: 'latent variables' },
  })
  expect(parsed.success).toBe(false)
})

test('runs the stationarity battery in the Rust analysis worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture spike runs once')
  const externalRequests = new Set<string>()
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') externalRequests.add(url.href)
  })
  await page.goto('/app')

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
  await page.goto('/app')
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

test('returns canonical and all minimal adjustment sets through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture contract runs once')
  await page.goto('/app')
  const result: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    // Dagitty's extended confounding triangle: A=0, B=1, D=2, E=3, Z=4.
    return analysis.identifyBackdoor({
      nodes: 5,
      edges: [[0, 3], [0, 4], [1, 2], [1, 4], [3, 2], [4, 2], [4, 3]],
      treatment: 3,
      outcome: 2,
      unobserved: [],
    })
  })
  expect(result).toEqual({
    ok: true,
    value: {
      kind: 'backdoorIdentification',
      nodes: 5,
      treatment: 3,
      outcome: 2,
      unobserved: [],
      result: {
        kind: 'identified',
        canonicalSet: [0, 1, 4],
        minimalSets: [[0, 4], [1, 4]],
        truncated: false,
      },
    },
  })
})

test('runs KCI, Holm, KS and permutation graph checks in the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 72
    const values = new Float64Array(rows * 3)
    for (let row = 0; row < rows; row += 1) {
      const x = Math.sin(row * 0.37) + 0.2 * Math.cos(row * 0.11)
      const middle = 0.8 * x + 0.3 * Math.sin(row * 1.17)
      values[row] = x
      values[rows + row] = middle
      values[2 * rows + row] = 0.7 * middle + 0.25 * Math.cos(row * 0.83)
    }
    const progress: unknown[] = []
    const design = {
      nodeColumns: [0, 1, 2],
      edges: [[0, 1], [1, 2]],
      implications: [{ x: 0, y: 2, given: [1] }],
      maximumObservations: 500,
      permutations: 20,
      significanceLevel: 0.05,
      runFalsification: true,
    } as const
    const skipped = await analysis.runDagCheck(Float64Array.from(values), rows, 3, { ...design, runFalsification: false })
    const result = await analysis.runDagCheck(values, rows, 3, design, (event: unknown) => progress.push(event))
    return { result, skipped, progress }
  })
  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: dagCheckEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    skipped: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: dagCheckEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.result.ok) return
  const evidence = parsed.data.result.value
  expect(parsed.data.skipped.ok).toBe(true)
  if (parsed.data.skipped.ok) expect(parsed.data.skipped.value.falsification.kind).toBe('skipped')
  expect(evidence.implications).toHaveLength(1)
  expect(evidence.implications[0].adjustedPValue).toBeGreaterThanOrEqual(evidence.implications[0].pValue)
  expect(evidence.uniformity.tests).toBe(1)
  expect(evidence.falsification.kind).toBe('completed')
  if (evidence.falsification.kind === 'completed') {
    expect(evidence.falsification.permutationLmcViolationFractions).toHaveLength(20)
    expect(evidence.falsification.permutationTpaViolationFractions).toHaveLength(20)
  }
  expect(parsed.data.progress.some((event) => event.stage === 'dag-implications')).toBe(true)
  expect(parsed.data.progress.some((event) => event.stage === 'dag-permutations' && event.completed === 20)).toBe(true)
})
