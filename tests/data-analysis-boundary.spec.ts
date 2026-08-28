import { expect, test } from '@playwright/test'
import { z } from 'zod'
import { grangerSsrEvidenceSchema, pcmciPlusEvidenceSchema } from '../src/domain/discovery'
import { stationarityBatterySchema } from '../src/domain/stationarity'

const analysisOutcomeSchema = z.discriminatedUnion('ok', [
  z.object({ ok: z.literal(true), value: stationarityBatterySchema }).strict(),
  z.object({
    ok: z.literal(false),
    error: z.object({ kind: z.string(), detail: z.string() }).strict(),
  }).strict(),
])

test('moves a selected Seatbelts column from DuckDB to Rust through transferable buffers', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  const externalRequests = new Set<string>()
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') externalRequests.add(url.href)
  })
  await page.goto('/')

  const raw: unknown = await page.evaluate(async () => {
    const [dataModule, workflowModule, analysisModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/analysis/client.ts', window.location.href).href),
    ])
    const response = await fetch('/tests/fixtures/Seatbelts.csv')
    if (!response.ok) throw new Error(`Fixture request failed with ${response.status}.`)
    const file = new File([await response.blob()], 'Seatbelts.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const column = profiled.value.columns.find((candidate: { readonly name: string }) => candidate.name === 'DriversKilled')
    if (!column) throw new Error('DriversKilled was not profiled.')
    const materialized = await dataModule.materializeNumericColumnsInWorker(file, profiled.value, [column.id])
    if (!materialized.ok) throw new Error(`Materialization failed: ${materialized.error.kind}`)
    const first = materialized.value.values[0]
    const second = materialized.value.values[1]
    const valuesBytesBeforeRust = materialized.value.values.byteLength
    const validityBytes = materialized.value.validity.byteLength
    const analysis = await analysisModule.runStationarityBattery(materialized.value.values)
    return {
      analysis,
      first,
      second,
      rows: materialized.value.rowCount,
      missing: materialized.value.missingCells,
      valuesBytesBeforeRust,
      valuesBytesAfterRust: materialized.value.values.byteLength,
      validityBytes,
    }
  })

  const parsed = z.object({
    analysis: analysisOutcomeSchema,
    first: z.number(),
    second: z.number(),
    rows: z.number().int(),
    missing: z.number().int(),
    valuesBytesBeforeRust: z.number().int(),
    valuesBytesAfterRust: z.number().int(),
    validityBytes: z.number().int(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.first).toBe(107)
  expect(parsed.data.second).toBe(97)
  expect(parsed.data.rows).toBe(192)
  expect(parsed.data.missing).toBe(0)
  expect(parsed.data.valuesBytesBeforeRust).toBe(192 * Float64Array.BYTES_PER_ELEMENT)
  expect(parsed.data.valuesBytesAfterRust).toBe(0)
  expect(parsed.data.validityBytes).toBe(24)
  expect(parsed.data.analysis.ok).toBe(true)
  if (parsed.data.analysis.ok) expect(parsed.data.analysis.value.observations).toBe(192)
  expect([...externalRequests]).toEqual([])
})

test('keeps null and zero distinct in a materialized numeric buffer', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/')
  const raw: unknown = await page.evaluate(async () => {
    const dataModule = await import(new URL('/src/data/client.ts', window.location.href).href)
    const workflowModule = await import(new URL('/src/domain/workflow.ts', window.location.href).href)
    const file = new File(['x,y\n1,\n0,2\n'], 'null-and-zero.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const column = profiled.value.columns.find((candidate: { readonly name: string }) => candidate.name === 'y')
    if (!column) throw new Error('y was not profiled.')
    const materialized = await dataModule.materializeNumericColumnsInWorker(file, profiled.value, [column.id])
    if (!materialized.ok) throw new Error(`Materialization failed: ${materialized.error.kind}`)
    return {
      firstIsNaN: Number.isNaN(materialized.value.values[0]),
      second: materialized.value.values[1],
      validityByte: materialized.value.validity[0],
      missing: materialized.value.missingCells,
    }
  })

  expect(raw).toEqual({ firstIsNaN: true, second: 2, validityByte: 2, missing: 1 })
})

test('refuses materialization when the file no longer matches its profile', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/')
  const raw: unknown = await page.evaluate(async () => {
    const dataModule = await import(new URL('/src/data/client.ts', window.location.href).href)
    const workflowModule = await import(new URL('/src/domain/workflow.ts', window.location.href).href)
    const original = new File(['x\n1\n2\n'], 'source.csv', { type: 'text/csv' })
    const changed = new File(['x\n1\n3\n'], 'source.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), original)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    return dataModule.materializeNumericColumnsInWorker(changed, profiled.value, [profiled.value.columns[0].id])
  })

  const parsed = z.object({
    ok: z.literal(false),
    error: z.object({
      kind: z.literal('source-changed'),
      expected: z.string().regex(/^[a-f0-9]{64}$/),
      actual: z.string().regex(/^[a-f0-9]{64}$/),
    }).strict(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (parsed.success) expect(parsed.data.error.actual).not.toBe(parsed.data.error.expected)
})

test('runs PCMCI+ as a distinct heavy discovery method over three materialized columns', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  const externalRequests = new Set<string>()
  page.on('request', (request) => {
    const url = new URL(request.url())
    if (url.protocol.startsWith('http') && url.hostname !== '127.0.0.1') externalRequests.add(url.href)
  })
  await page.goto('/')
  const raw: unknown = await page.evaluate(async () => {
    const [dataModule, workflowModule, analysisModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/analysis/client.ts', window.location.href).href),
    ])
    const rows: string[] = ['x,y,z']
    let priorX = 0
    let priorY = 0
    for (let row = 0; row < 120; row += 1) {
      const time = row
      const x = Math.sin(time / 4) + 0.1 * Math.cos(time / 1.7)
      const y = 0.8 * priorX + 0.05 * Math.sin(time / 2.3)
      const z = -0.6 * priorY + 0.04 * Math.cos(time / 3.1)
      rows.push(`${x},${y},${z}`)
      priorX = x
      priorY = y
    }
    const file = new File([rows.join('\n')], 'pcmci-fixture.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const [x, y, z] = profiled.value.columns
    if (!x || !y || !z) throw new Error('The three discovery columns were not profiled.')
    const materialized = await dataModule.materializeNumericColumnsInWorker(
      file,
      profiled.value,
      [x.id, y.id, z.id],
    )
    if (!materialized.ok) throw new Error(`Materialization failed: ${materialized.error.kind}`)
    const result = await analysisModule.runPcmciPlus(materialized.value.values, 120, 3, 2, 0.05)
    return { result, detachedBytes: materialized.value.values.byteLength }
  })

  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: pcmciPlusEvidenceSchema }).strict(),
      z.object({
        ok: z.literal(false),
        error: z.object({ kind: z.string(), detail: z.string() }).strict(),
      }).strict(),
    ]),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.detachedBytes).toBe(0)
  if (!parsed.data.result.ok) {
    throw new Error(`${parsed.data.result.error.kind}: ${parsed.data.result.error.detail}`)
  }
  expect(parsed.data.result.value.observations).toBe(120)
  expect(parsed.data.result.value.variables).toBe(3)
  expect(parsed.data.result.value.tauMax).toBe(2)
  expect(parsed.data.result.value.graph).toHaveLength(3)
  expect(parsed.data.result.value.graph.flat(2).filter((mark) => mark.length > 0).length).toBeGreaterThan(0)
  expect([...externalRequests]).toEqual([])
})

test('runs the Granger SSR F port with target then candidate-cause column order', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/')
  const raw: unknown = await page.evaluate(async () => {
    const analysisModule = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 100
    const values = new Float64Array(rows * 2)
    for (let row = 0; row < rows; row += 1) {
      const time = row
      const cause = Math.sin(time / 4) + 0.1 * Math.cos(time / 1.7)
      values[rows + row] = cause
      values[row] = row === 0 ? 0 : 0.75 * values[rows + row - 1] + 0.04 * Math.sin(time / 2.3)
    }
    const result = await analysisModule.runGrangerSsrF(values, rows, 4)
    return { result, detachedBytes: values.byteLength }
  })

  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: grangerSsrEvidenceSchema }).strict(),
      z.object({
        ok: z.literal(false),
        error: z.object({ kind: z.string(), detail: z.string() }).strict(),
      }).strict(),
    ]),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.detachedBytes).toBe(0)
  if (!parsed.data.result.ok) {
    throw new Error(`${parsed.data.result.error.kind}: ${parsed.data.result.error.detail}`)
  }
  expect(parsed.data.result.value.observations).toBe(100)
  expect(parsed.data.result.value.maxLag).toBe(4)
  expect(parsed.data.result.value.tests.map((result) => result.lag)).toEqual([1, 2, 3, 4])
  expect(parsed.data.result.value.tests[0].pValue).toBeLessThan(1e-10)
})
