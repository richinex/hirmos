import { expect, test } from '@playwright/test'
import { grangerSsrEvidenceSchema } from '../src/domain/granger'
import { z } from 'zod'
import {
  cmlpEvidenceSchema,
  clstmEvidenceSchema,
  dynotearsEvidenceSchema,
  lpcmciEvidenceSchema,
  ocseEvidenceSchema,
  varLingamEvidenceSchema,
  pcmciPlusEvidenceSchema,
} from '../src/domain/discovery'
import { stationarityBatterySchema } from '../src/domain/stationarity'
import { panelInterventionEvidenceSchema, syntheticControlEvidenceSchema } from '../src/domain/estimation'

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
  await page.goto('/app')

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
  await page.goto('/app')
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

test('keeps calendar dates temporal in the bounded and paged previews', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Data preview boundary runs once')
  await page.goto('/app')

  const raw: unknown = await page.evaluate(async () => {
    const [dataModule, workflowModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
    ])
    const file = new File(['date,x\n1958-03-29,1\n1958-04-05,2\n'], 'dated.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const preview = await dataModule.previewWindowInWorker(file, profiled.value, {
      offset: 0,
      limit: 2,
      sort: null,
      filters: [],
      search: '',
    })
    if (!preview.ok) throw new Error(`Preview failed: ${preview.error.kind}`)
    return {
      physicalType: profiled.value.columns[0].duckdbType,
      bounded: profiled.value.preview[0][0],
      paged: preview.value.rows[0]?.cells[0],
    }
  })

  expect(raw).toEqual({
    physicalType: 'DATE',
    bounded: { kind: 'temporal', value: '1958-03-29T00:00:00.000Z' },
    paged: { kind: 'temporal', value: '1958-03-29T00:00:00.000Z' },
  })
})

test('materializes saved per-column time-series transformations on one aligned grid', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Prepared transformation boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const [dataModule, workflowModule, preparedModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/data/prepared.ts', window.location.href).href),
    ])
    const file = new File(['time,x,y,z\n1,10,100,1\n2,12,101,4\n3,15,103,9\n4,19,106,16\n'], 'transforms.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const column = (name: string) => {
      const found = profiled.value.columns.find((candidate: { readonly name: string }) => candidate.name === name)
      if (!found) throw new Error(`${name} was not profiled.`)
      return found.id
    }
    const time = column('time')
    const x = column('x')
    const y = column('y')
    const z = column('z')
    const source = { file, name: file.name, bytes: file.size, mediaType: file.type, lastModified: file.lastModified, format: 'csv' as const }
    const prepared = {
      kind: 'prepared-time-series' as const,
      id: 'prepared-transform-test',
      recipe: 'recipe-transform-test',
      sourceProfile: profiled.value.id,
      observations: 3,
      columns: [x, y, z],
      sampling: { kind: 'regular-series' as const, timeColumn: time, frequency: 'daily' as const },
      missingness: { kind: 'not-present' as const },
      resolution: { kind: 'none' as const },
      resampling: { kind: 'none' as const },
      seasonalAdjustment: { kind: 'none' as const },
      seriesTransforms: [
        { column: x, transform: { kind: 'difference' as const, order: 1 as const } },
        { column: y, transform: { kind: 'levels' as const } },
        { column: z, transform: { kind: 'linear-detrend' as const } },
      ],
    }
    const materialized = await preparedModule.materialisePrepared(source, profiled.value, prepared, [x, y, z])
    if (!materialized.ok) throw new Error(`Prepared materialization failed: ${materialized.error.kind}`)
    const yOnly = await preparedModule.materialisePrepared(source, profiled.value, prepared, [y])
    if (!yOnly.ok) throw new Error(`Subset materialization failed: ${yOnly.error.kind}`)
    return {
      values: Array.from(materialized.value.values),
      rows: materialized.value.rowCount,
      leadingRowsRemoved: materialized.value.leadingRowsRemoved,
      yOnly: Array.from(yOnly.value.values),
      yOnlyRows: yOnly.value.rowCount,
      yOnlyLeadingRowsRemoved: yOnly.value.leadingRowsRemoved,
    }
  })

  expect(raw).toEqual({
    values: [2, 3, 4, 101, 103, 106, -1, -1, 1],
    rows: 3,
    leadingRowsRemoved: 1,
    yOnly: [101, 103, 106],
    yOnlyRows: 3,
    yOnlyLeadingRowsRemoved: 1,
  })
})

test('upgrades saved version-1 transformation fields at the persistence boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Saved-project compatibility runs once')
  await page.goto('/app')
  const raw = await page.evaluate(async () => {
    const persistence = await import(new URL('/src/domain/persistence.ts', window.location.href).href)
    const parsed = persistence.parseSnapshotValue({
      kind: 'hirmos-project',
      version: 1,
      savedAt: '2026-08-31T00:00:00.000Z',
      project: { id: 'project', name: 'Legacy prepared project', createdAt: '2026-08-30T00:00:00.000Z' },
      source: null,
      profile: null,
      prepared: { id: 'prepared', kind: 'prepared-time-series', columns: ['x', 'y'] },
      stationarity: { id: 'stationarity', kind: 'stationarity-evidence', transform: { kind: 'difference', order: 1 } },
      grangerEvidence: [],
      discoveryRuns: [],
      dagDocuments: [],
      dagChecks: [],
      interventionQueries: [],
      studyDraft: {},
      studies: [],
      identifications: [],
      estimationRuns: [{ id: 'run', kind: 'backdoor-linear-run', estimate: { adjustmentSet: [{ node: 'node-z', column: 'z', name: 'Z' }] } }],
      sensitivityRuns: [],
      counterfactualRuns: [],
    })
    if (!parsed.ok) return parsed
    return {
      ok: true,
      transforms: parsed.value.prepared?.kind === 'prepared-time-series' ? parsed.value.prepared.seriesTransforms : null,
      resampling: parsed.value.prepared?.kind === 'prepared-time-series' ? parsed.value.prepared.resampling : null,
      diagnosticTransform: parsed.value.stationarity?.diagnosticTransform ?? null,
      legacyTransformRetained: parsed.value.stationarity !== null && 'transform' in parsed.value.stationarity,
      appliedAdjustment: parsed.value.estimationRuns[0]?.estimate.adjustment ?? null,
    }
  })

  expect(raw).toEqual({
    ok: true,
    transforms: [
      { column: 'x', transform: { kind: 'levels' } },
      { column: 'y', transform: { kind: 'levels' } },
    ],
    resampling: { kind: 'none' },
    diagnosticTransform: { kind: 'difference', order: 1 },
    legacyTransformRetained: false,
    appliedAdjustment: { kind: 'contemporaneous', variables: [{ node: 'node-z', column: 'z', name: 'Z' }] },
  })
})

test('the shipped example uses the current applied-adjustment record', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Example compatibility runs once')
  await page.goto('/app')
  const adjustments = await page.evaluate(async () => {
    const bundle = await import(new URL('/src/domain/bundle.ts', window.location.href).href)
    const response = await fetch('/examples/seatbelts.hirmos.json')
    const parsed = bundle.parseBundle(await response.text())
    if (!parsed.ok) throw new Error(`Example bundle failed: ${parsed.error.kind}`)
    return parsed.value.project.estimationRuns.map((run: { readonly estimate: { readonly adjustment: unknown } }) => run.estimate.adjustment)
  })
  expect(adjustments).toEqual([
    { kind: 'contemporaneous', variables: [{ node: '855efccf-0540-48c3-b7f0-4620b6744971:6:PetrolPrice', column: '6:PetrolPrice', name: 'PetrolPrice' }] },
    { kind: 'contemporaneous', variables: [{ node: '855efccf-0540-48c3-b7f0-4620b6744971:6:PetrolPrice', column: '6:PetrolPrice', name: 'PetrolPrice' }] },
    { kind: 'contemporaneous', variables: [{ node: '855efccf-0540-48c3-b7f0-4620b6744971:6:PetrolPrice', column: '6:PetrolPrice', name: 'PetrolPrice' }] },
  ])
})

test('opens the shipped example Estimation chapter without the compatibility boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Example rendering runs once')
  await page.goto('/app')
  const example = page.getByRole('listitem').filter({ hasText: 'Seat-belt law and road deaths' })
  await example.getByRole('button', { name: 'Open' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Estimation/ }).click()
  await expect(page.getByText('The Estimation chapter could not be displayed.')).toHaveCount(0)
  await expect(page.getByText('Runs · 3')).toBeVisible()
})

test('returns synthetic-control cross-fit, donor-placebo, and prediction-band evidence through Wasm', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Synthetic-control boundary runs once')
  await page.goto('/app')
  const raw = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 40
    const nPre = 25
    const donorA = Array.from({ length: rows }, (_, index) => 10 + 0.5 * index + 0.2 * Math.sin(index))
    const donorB = Array.from({ length: rows }, (_, index) => 30 - 0.2 * index + 0.15 * Math.cos(index / 2))
    const treated = donorA.map((value, index) => 0.6 * value + 0.4 * (donorB[index] ?? 0) + (index >= nPre ? 5 : 0))
    const values = Float64Array.from([...treated, ...donorA, ...donorB])
    return analysis.runSyntheticControl(values, rows, 3, { treated: 0, donors: [1, 2], nPre, crossFitFolds: 3, alpha: 0.1 })
  })
  const parsed = z.object({ ok: z.literal(true), value: syntheticControlEvidenceSchema }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.value.crossFit.kind).toBe('available')
  expect(parsed.data.value.donorPlacebo.kind).toBe('available')
  expect(parsed.data.value.conformalBand.kind).toBe('available')
  expect(parsed.data.value.gaussianBand.kind).toBe('available')
  expect(Math.abs(parsed.data.value.att - 5)).toBeLessThan(1e-6)
})

test('validates, materializes, and estimates a balanced long panel through both workers', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Panel boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const [dataModule, workflowModule, analysisModule, panelModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/analysis/client.ts', window.location.href).href),
      import(new URL('/src/domain/panel.ts', window.location.href).href),
    ])
    const rows = ['unit,time,outcome,treated']
    const periods = 9
    for (const unit of ['control-a', 'control-b', 'treated']) {
      for (let time = 0; time < periods; time += 1) {
        const a = 10 + time + 0.2 * Math.sin(time)
        const b = 20 + 0.45 * time + 0.15 * Math.cos(time / 2)
        const post = time >= 5
        const outcome = unit === 'control-a' ? a : unit === 'control-b' ? b : 0.4 * a + 0.6 * b + (post ? 4 : 0)
        rows.push(`${unit},${time},${outcome},${unit === 'treated' && post ? 1 : 0}`)
      }
    }
    const file = new File([rows.join('\n')], 'balanced-panel.csv', { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const column = (name: string) => {
      const found = profiled.value.columns.find((candidate: { readonly name: string }) => candidate.name === name)
      if (!found) throw new Error(`${name} was not profiled.`)
      return found.id
    }
    const unit = column('unit')
    const time = column('time')
    const outcome = column('outcome')
    const treatment = column('treated')
    const structure = await dataModule.inspectPanelInWorker(file, profiled.value, unit, time)
    if (!structure.ok) throw new Error(`Panel inspection failed: ${structure.error.kind}`)
    const matrix = await dataModule.materializePanelInWorker(file, profiled.value, { unit, time, outcome, treatment })
    if (!matrix.ok) throw new Error(`Panel materialization failed: ${matrix.error.kind}`)
    const layout = panelModule.assessPanelInterventionLayout(matrix.value)
    if (!layout.ok) throw new Error(`Panel preflight failed: ${layout.error.kind}`)
    const altered = (changes: readonly (readonly [number, number])[]) => {
      const values = matrix.value.values.slice()
      for (const [row, value] of changes) values[matrix.value.rowCount + row] = value
      return panelModule.assessPanelInterventionLayout({ ...matrix.value, values })
    }
    const nonBinary = altered([[18, 0.5]])
    const nonAbsorbing = altered([[26, 0]])
    const noControls = altered([[5, 1], [6, 1], [7, 1], [8, 1], [14, 1], [15, 1], [16, 1], [17, 1]])
    const progress: { stage: string; completed: number; total: number }[] = []
    const estimated = await analysisModule.runPanelIntervention(
      matrix.value.values,
      matrix.value.rowCount,
      matrix.value.units,
      matrix.value.times,
      { placeboReplications: 24, seed: 0 },
      (next: { stage: string; completed: number; total: number }) => progress.push(next),
    )
    return {
      structure,
      layout,
      preflightRefusals: [nonBinary, nonAbsorbing, noControls].map((result) => result.ok ? 'unexpected-ready' : result.error.kind),
      estimated,
      progress,
      detachedBytes: matrix.value.values.byteLength,
    }
  })

  const parsed = z.object({
    structure: z.object({
      ok: z.literal(true),
      value: z.object({
        kind: z.literal('panel-structure'), observations: z.literal(27), units: z.literal(3), periods: z.literal(9), duplicateKeys: z.literal(0), missingUnitKeys: z.literal(0), missingTimeKeys: z.literal(0), balanced: z.literal(true),
      }).passthrough(),
    }).strict(),
    layout: z.object({
      ok: z.literal(true),
      value: z.object({ kind: z.literal('panel-intervention-layout'), controls: z.array(z.string()).length(2), treated: z.array(z.string()).length(1), prePeriods: z.literal(5), postPeriods: z.literal(4), adoptionLabel: z.literal('5'), controlPreDifferenceSd: z.number().positive() }).passthrough(),
    }).strict(),
    preflightRefusals: z.tuple([z.literal('treatment-not-binary'), z.literal('non-simultaneous-adoption'), z.literal('no-control-unit')]),
    estimated: z.object({ ok: z.literal(true), value: panelInterventionEvidenceSchema }).strict(),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
    detachedBytes: z.literal(0),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(Math.abs(parsed.data.estimated.value.syntheticDid.estimate - 4)).toBeLessThan(0.1)
  expect(Math.abs(parsed.data.estimated.value.syntheticControl.estimate - 4)).toBeLessThan(0.1)
  expect(parsed.data.estimated.value.syntheticControlPlacebo.kind).toBe('available')
  expect(parsed.data.estimated.value.syntheticDidPlacebo.kind).toBe('available')
  expect(parsed.data.estimated.value.syntheticControlInTime.kind).toBe('available')
  expect(parsed.data.estimated.value.syntheticDidInTime.kind).toBe('available')
  expect(parsed.data.progress).toEqual([
    { stage: 'validated-panel', completed: 1, total: 9 },
    { stage: 'difference-in-differences', completed: 2, total: 9 },
    { stage: 'synthetic-control', completed: 3, total: 9 },
    { stage: 'synthetic-did', completed: 4, total: 9 },
    { stage: 'placebo-permutations', completed: 5, total: 9 },
    { stage: 'synthetic-control-placebos', completed: 6, total: 9 },
    { stage: 'synthetic-did-placebos', completed: 7, total: 9 },
    { stage: 'synthetic-control-in-time', completed: 8, total: 9 },
    { stage: 'synthetic-did-in-time', completed: 9, total: 9 },
  ])
})

test('keeps completed and refused missingness outcomes disjoint at the Wasm boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const values = new Float64Array([1, Number.NaN, Number.NaN, 4, 5, 6, Number.NaN, 2, 2, 2, 2, 2])
    const validity = new Uint8Array([1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1])
    const refused = await analysis.resolveMissingnessInWorker(values, 6, 2, validity, {
      kind: 'imputation',
      method: 'linearInterior',
      maxGap: 2,
      confirmation: null,
    })
    const completed = await analysis.resolveMissingnessInWorker(values, 6, 2, validity, {
      kind: 'imputation',
      method: 'structuralZero',
      maxGap: 1,
      confirmation: 'absence means zero',
    })
    return { refused, completed }
  })

  const parsed = z.object({
    refused: z.object({
      ok: z.literal(true),
      value: z.object({
        kind: z.literal('missingnessResolved'),
        outcome: z.object({
          kind: z.literal('refused'),
          reasons: z.array(z.object({ kind: z.literal('unresolvedCells'), count: z.literal(1) }).strict()).length(1),
          remainingMissing: z.literal(1),
        }).passthrough(),
      }).passthrough(),
    }).strict(),
    completed: z.object({
      ok: z.literal(true),
      value: z.object({
        kind: z.literal('missingnessResolved'),
        outcome: z.object({
          kind: z.literal('completed'),
          values: z.array(z.number()).length(12),
          imputedCells: z.array(z.tuple([z.number(), z.number()])).length(3),
        }).passthrough(),
      }).passthrough(),
    }).strict(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
})

test('refuses materialization when the file no longer matches its profile', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
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
  await page.goto('/app')
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
    const result = await analysisModule.runPcmciPlus(materialized.value.values, 120, 3, 2, 0.05, { kind: 'dense' })
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

test('runs PCMCI+ with Tigramite-compatible role-aware missing-sample construction', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')

  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 80
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      values[row] = Math.sin(row / 3)
      values[rows + row] = Math.cos(row / 5)
      values[2 * rows + row] = Math.sin(row / 7)
    }
    const validity = new Uint8Array(values.length).fill(1)
    const analysisMask = new Uint8Array(values.length)
    values[17] = Number.NaN
    validity[17] = 0
    analysisMask[rows + 31] = 1

    const result = await analysis.runPcmciPlus(values, rows, columns, 2, 0.05, {
      kind: 'role-aware',
      validity,
      analysisMask,
      cutOff: 'twoTauMax',
      propagateThroughMaxLag: true,
      maskType: 'xyz',
    })
    return {
      result,
      detached: {
        values: values.byteLength,
        validity: validity.byteLength,
        analysisMask: analysisMask.byteLength,
      },
    }
  })

  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: pcmciPlusEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.object({ kind: z.string(), detail: z.string() }).strict() }).strict(),
    ]),
    detached: z.object({ values: z.number(), validity: z.number(), analysisMask: z.number() }).strict(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.detached).toEqual({ values: 0, validity: 0, analysisMask: 0 })
  if (!parsed.data.result.ok) throw new Error(`${parsed.data.result.error.kind}: ${parsed.data.result.error.detail}`)
  expect(parsed.data.result.value.observations).toBe(80)
  expect(parsed.data.result.value.variables).toBe(3)
})

test('offers role-aware samples only to PCMCI+ and LPCMCI', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')

  const verdicts: unknown = await page.evaluate(async () => {
    const [discovery, methods] = await Promise.all([
      import(new URL('/src/domain/discovery.ts', window.location.href).href),
      import(new URL('/src/domain/methods.ts', window.location.href).href),
    ])
    const prepared = {
      kind: 'prepared-time-series',
      id: 'prepared-role-aware',
      recipe: 'recipe-role-aware',
      sourceProfile: 'profile-role-aware',
      observations: 80,
      columns: ['x', 'y'],
      resolution: { kind: 'lag-aware-exclusion', cells: 1 },
      seasonalAdjustment: { kind: 'none' },
      sampling: { kind: 'regular-series', timeColumn: 'date', frequency: 'daily' },
      missingness: {
        kind: 'lag-aware-exclusion',
        cells: 1,
        cutOff: '2xtau-max',
        propagateThroughMaxLag: false,
        analysisExclusions: { kind: 'ignore' },
      },
      resampling: { kind: 'none' },
      seriesTransforms: [
        { column: 'x', transform: { kind: 'levels' } },
        { column: 'y', transform: { kind: 'levels' } },
      ],
    }
    const eligibility = (id: string) => {
      const definition = methods.methodDefinition(id)
      if (!definition.ok) throw new Error(`Method ${id} is absent.`)
      return discovery.evaluateDiscoveryEligibility(definition.value, prepared, null).kind
    }
    return {
      pcmci: eligibility(methods.PCMCI_PLUS_PAR_CORR_METHOD_ID),
      lpcmci: eligibility(methods.LPCMCI_PAR_CORR_METHOD_ID),
      rpcmci: eligibility(methods.RPCMCI_PAR_CORR_METHOD_ID),
      dynotears: eligibility(methods.DYNOTEARS_METHOD_ID),
      pcmciReady: discovery.readyDiscoverySpecification({ kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 }, prepared).ok,
      lpcmciReady: discovery.readyDiscoverySpecification({ kind: 'lpcmci', tauMax: 2, pcAlpha: 0.05 }, prepared).ok,
      rpcmciReady: discovery.readyDiscoverySpecification({ kind: 'rpcmci', numRegimes: 2, maxTransitions: 4, tauMin: 1, tauMax: 2, pcAlpha: 0.2, alphaLevel: 0.01, switchThres: 0.05, numIterations: 20, maxAnneal: 10, seed: 43 }, prepared).ok,
    }
  })

  expect(verdicts).toEqual({
    pcmci: 'caution',
    lpcmci: 'caution',
    rpcmci: 'refused',
    dynotears: 'refused',
    pcmciReady: true,
    lpcmciReady: true,
    rpcmciReady: false,
  })
})

test('runs the Granger SSR F port with target then candidate-cause column order', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
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

test('runs LPCMCI, DYNOTEARS, and corrected oCSE through Wasm with progress callbacks', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 60
    const columns = 3
    const makeValues = () => {
      const values = new Float64Array(rows * columns)
      for (let row = 0; row < rows; row += 1) {
        const time = row
        values[row] = Math.sin(time / 3.7) + 0.07 * Math.cos(time / 1.9)
        values[rows + row] = (row === 0 ? 0 : 0.72 * values[row - 1]) + 0.05 * Math.sin(time / 2.1)
        values[2 * rows + row] = (row === 0 ? 0 : -0.55 * values[rows + row - 1]) + 0.04 * Math.cos(time / 2.9)
      }
      return values
    }
    const lpcmciProgress: unknown[] = []
    const dynotearsProgress: unknown[] = []
    const ocseProgress: unknown[] = []
    const varLingamProgress: unknown[] = []
    const lpcmci = await analysis.runLpcmci(makeValues(), rows, columns, 1, 0.05, { kind: 'dense' }, (progress: unknown) => lpcmciProgress.push(progress))
    const dynotears = await analysis.runDynotears(makeValues(), rows, columns, 1, 0.1, 0.1, (progress: unknown) => dynotearsProgress.push(progress))
    const ocse = await analysis.runOcse(makeValues(), rows, columns, 1, 0.05, 20, 'gaussian', 5, (progress: unknown) => ocseProgress.push(progress))
    const varLingam = await analysis.runVarLingam(makeValues(), rows, columns, 2, true, (progress: unknown) => varLingamProgress.push(progress))
    return { lpcmci, dynotears, ocse, varLingam, lpcmciProgress, dynotearsProgress, ocseProgress, varLingamProgress }
  })

  const progressSchema = z.array(z.object({
    stage: z.string().min(1),
    completed: z.number().int().nonnegative(),
    total: z.number().int().positive(),
  }).strict()).min(2)
  const outcome = <Value extends z.ZodType>(value: Value) => z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value }).strict(),
    z.object({ ok: z.literal(false), error: z.object({ kind: z.string(), detail: z.string() }).strict() }).strict(),
  ])
  const parsed = z.object({
    lpcmci: outcome(lpcmciEvidenceSchema),
    dynotears: outcome(dynotearsEvidenceSchema),
    ocse: outcome(ocseEvidenceSchema),
    varLingam: outcome(varLingamEvidenceSchema),
    lpcmciProgress: progressSchema,
    dynotearsProgress: progressSchema,
    ocseProgress: progressSchema,
    varLingamProgress: progressSchema,
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  if (!parsed.data.lpcmci.ok || !parsed.data.dynotears.ok || !parsed.data.ocse.ok || !parsed.data.varLingam.ok) {
    throw new Error('One of the new discovery methods was refused at the browser boundary.')
  }
  const expectedColumns = 3
  expect(parsed.data.lpcmci.value.graph).toHaveLength(expectedColumns)
  expect(parsed.data.dynotears.value.contemporaneousWeights).toHaveLength(expectedColumns)
  expect(parsed.data.dynotears.value.laggedWeights).toHaveLength(1)
  expect(parsed.data.ocse.value.seed).toBe(42)
  expect(parsed.data.varLingam.value.causalOrder).toHaveLength(expectedColumns)
  expect(parsed.data.varLingam.value.laggedWeights).toHaveLength(parsed.data.varLingam.value.selectedLag)
  expect(parsed.data.varLingamProgress.at(-1)).toMatchObject({ stage: 'complete', completed: 2, total: 2 })
  expect(parsed.data.lpcmciProgress.at(-1)).toMatchObject({ stage: 'complete', completed: 5, total: 5 })
  const finalDynotearsProgress = parsed.data.dynotearsProgress.at(-1)
  expect(finalDynotearsProgress?.stage).toBe('complete')
  expect(finalDynotearsProgress?.completed).toBe(finalDynotearsProgress?.total)
  expect(parsed.data.ocseProgress.at(-1)).toMatchObject({ stage: 'complete', completed: expectedColumns, total: expectedColumns })
})

test('runs cMLP and cLSTM through the Neural worker boundary without inventing cLSTM lags', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 32
    const columns = 2
    const makeValues = () => {
      const values = new Float64Array(rows * columns)
      for (let row = 0; row < rows; row += 1) {
        const time = row
        values[row] = 15_000 + 2_500 * Math.sin(time / 3.7)
        values[rows + row] = 0.1 + (row === 0 ? 0 : 0.02 * (values[row - 1] - 15_000) / 2_500) + 0.003 * Math.sin(time / 2.1)
      }
      return values
    }
    const cmlpProgress: unknown[] = []
    const clstmProgress: unknown[] = []
    const cmlp = await analysis.runCmlp(makeValues(), rows, columns, {
      lag: 2,
      hidden: [3],
      activation: 'relu',
      penalty: 'hierarchical',
      lambda: 0.005,
      ridgeLambda: 0.01,
      learningRate: 0.01,
      maxIter: 1,
      checkEvery: 1,
      lookback: 1,
      seed: 0,
    }, (progress: unknown) => cmlpProgress.push(progress))
    const clstm = await analysis.runClstm(makeValues(), rows, columns, {
      context: 3,
      hidden: 3,
      lambda: 0.005,
      ridgeLambda: 0.01,
      learningRate: 0.01,
      maxIter: 1,
      checkEvery: 1,
      lookback: 1,
      seed: 0,
    }, (progress: unknown) => clstmProgress.push(progress))
    return { cmlp, clstm, cmlpProgress, clstmProgress }
  })

  const progressSchema = z.array(z.object({
    stage: z.string().min(1),
    completed: z.number().int().nonnegative(),
    total: z.number().int().positive(),
  }).strict()).min(2)
  const outcome = <Value extends z.ZodType>(value: Value) => z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value }).strict(),
    z.object({ ok: z.literal(false), error: z.object({ kind: z.string(), detail: z.string() }).strict() }).strict(),
  ])
  const parsed = z.object({
    cmlp: outcome(cmlpEvidenceSchema),
    clstm: outcome(clstmEvidenceSchema),
    cmlpProgress: progressSchema,
    clstmProgress: progressSchema,
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  if (!parsed.data.cmlp.ok || !parsed.data.clstm.ok) {
    throw new Error('One of the Neural Granger methods was refused at the browser boundary.')
  }
  expect(parsed.data.cmlp.value.summaryScores).toHaveLength(2)
  expect(parsed.data.cmlp.value.standardization.scales[0]).toBeGreaterThan(1_000)
  expect(parsed.data.cmlp.value.standardization.scales[1]).toBeLessThan(0.1)
  expect(parsed.data.cmlp.value.lagScores[0]?.[0]).toHaveLength(2)
  expect(parsed.data.cmlp.value.lagOrder).toEqual([2, 1])
  expect(parsed.data.clstm.value.summaryScores).toHaveLength(2)
  expect(parsed.data.clstm.value.standardization.scales).toHaveLength(2)
  expect('lagScores' in parsed.data.clstm.value).toBe(false)
  expect('lagOrder' in parsed.data.clstm.value).toBe(false)
  expect(parsed.data.cmlpProgress.at(-1)).toMatchObject({ stage: 'complete', completed: 1, total: 1 })
  expect(parsed.data.clstmProgress.at(-1)).toMatchObject({ stage: 'complete', completed: 1, total: 1 })
})

test('cancels a Neural run by replacing the blocked worker and starts a fresh run afterwards', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 96
    const columns = 2
    const makeValues = () => {
      const values = new Float64Array(rows * columns)
      for (let row = 0; row < rows; row += 1) {
        values[row] = 10_000 + 2_000 * Math.sin(row / 4)
        values[rows + row] = 0.1 + 0.02 * Math.cos(row / 5)
      }
      return values
    }
    const longRun = analysis.runCmlp(makeValues(), rows, columns, {
      lag: 3,
      hidden: [100],
      activation: 'relu',
      penalty: 'hierarchical',
      lambda: 0.005,
      ridgeLambda: 0.01,
      learningRate: 0.01,
      maxIter: 50_000,
      checkEvery: 100,
      lookback: 5,
      seed: 0,
    })
    await new Promise((resolve) => window.setTimeout(resolve, 25))
    analysis.cancelAnalysisRuns()
    const cancelled = await longRun
    const retry = await analysis.runCmlp(makeValues(), rows, columns, {
      lag: 2,
      hidden: [3],
      activation: 'relu',
      penalty: 'hierarchical',
      lambda: 0.005,
      ridgeLambda: 0.01,
      learningRate: 0.01,
      maxIter: 1,
      checkEvery: 1,
      lookback: 1,
      seed: 0,
    })
    return { cancelled, retry }
  })

  const parsed = z.object({
    cancelled: z.object({
      ok: z.literal(false),
      error: z.object({ kind: z.literal('analysis-cancelled'), detail: z.string().min(1) }).strict(),
    }).strict(),
    retry: z.object({ ok: z.literal(true), value: cmlpEvidenceSchema }).strict(),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
})
