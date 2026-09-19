import { expect, test } from '@playwright/test'
import { z } from 'zod'
import { stationarityBatterySchema } from '../src/domain/stationarity'
import { dagCheckEvidenceSchema } from '../src/domain/dagValidation'
import { cdnotsEvidenceSchema, cdnotsPlusEvidenceSchema, directLingamEvidenceSchema, fciEvidenceSchema, graceEvidenceSchema, pcStableEvidenceSchema } from '../src/domain/discovery'
import { identifiedDiscreteQueryEvidenceSchema } from '../src/domain/intervention'
import { binaryEttEvidenceSchema, causalEffectsEvidenceSchema, frontdoorTwoStageEvidenceSchema, instrumentalVariableEvidenceSchema } from '../src/domain/estimation'
import { seasonalAdjustedEvidenceSchema } from '../src/domain/seasonal'
import { parseSeriesStructureEvidence, seriesStructureEvidenceSchema } from '../src/domain/sensitivity'
import { multicollinearityEvidenceSchema } from '../src/domain/multicollinearity'
import { describeAnalysisWorkerProblem, parseAnalysisRefusal } from '../src/workers/analysisProtocol'

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

test('parses and explains a discrete-state refusal without flattening its cause', () => {
  const parsed = parseAnalysisRefusal({
    kind: 'discreteStateRefused',
    query: 'identifiedExpression',
    node: 2,
    name: 'recovery',
    problem: { kind: 'singleObservedState', value: 4, observations: 614 },
  })

  expect(parsed.ok).toBe(true)
  if (!parsed.ok || parsed.value === null) return
  expect(parsed.value.problem.kind).toBe('singleObservedState')
  expect(describeAnalysisWorkerProblem(parsed.value)).toBe(
    'recovery takes only one value (4) across 614 prepared rows. This query requires at least two observed states.',
  )
})

test('rejects an effect attached to an unidentifiable intervention result', () => {
  const parsed = identifiedDiscreteQueryEvidenceSchema.safeParse({
    kind: 'identifiedDiscreteQuery', observations: 80, bins: 2, stateCounts: [2, 2], treatmentStates: ['0', '1'],
    query: { kind: 'unconditional' },
    result: { kind: 'unidentifiable', hedgeGraph: [0, 1], hedgeSubgraph: [1], effect: 0.4 },
  })
  expect(parsed.success).toBe(false)
})

test('requires complete STL components and aligned lag-correlation evidence', () => {
  const seasonal = seasonalAdjustedEvidenceSchema.safeParse({
    kind: 'seasonalAdjusted', rows: 3, columns: 1, period: 2, values: [1, 2, 3],
    adjusted: [{ column: 0, seasonalStrengthBefore: 0.8, seasonalStrengthAfter: 0.1 }],
  })
  expect(seasonal.success).toBe(false)

  const structure = parseSeriesStructureEvidence({
    kind: 'seriesStructure', observations: 20, period: 4,
    series: [{ column: 0, trendStrength: 0.5, seasonalStrength: 0.5, correlationMaxLag: 3, acf: [1, 0.2], acfLimits: [0, 0.4], pacf: [1, 0.2], pacfLimits: [0, 0.4], changePoints: [], peltPenalty: 2 }],
  })
  expect(structure.ok).toBe(false)
})

test('requires multicollinearity selections to partition the matrix', () => {
  const parsed = multicollinearityEvidenceSchema.safeParse({
    kind: 'multicollinearity',
    observations: 20,
    variables: 2,
    correlationThreshold: 0.9,
    vifThreshold: 10,
    correlation: [[1, 0.95], [0.95, 1]],
    correlationKeep: [0],
    correlationDrop: [],
    correlationClusters: [[0, 1]],
    vifKeep: [0],
    vifDrop: [1],
    vifHistory: [{ column: 1, vif: 14 }],
  })
  expect(parsed.success).toBe(false)
})

test('runs correlation clustering and VIF through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const x = [1, 2, 4, 7, 11, 16, 22, 29]
    const nearX = [1.1, 2.1, 3.9, 7.2, 10.8, 16.1, 21.9, 29.2]
    const z = [2, -1, 3, 0.5, -2, 4, 1, -3]
    return analysis.runMulticollinearity(Float64Array.from([...x, ...nearX, ...z]), 8, 3, { correlation: 0.99, vif: 10 })
  })
  const parsed = z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value: multicollinearityEvidenceSchema }).strict(),
    z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
  ]).parse(raw)
  expect(parsed.ok).toBe(true)
  if (parsed.ok) {
    expect(parsed.value.correlationClusters[0]).toEqual([0, 1])
    expect(parsed.value.vifHistory.length).toBeGreaterThan(0)
  }
})

test('returns STL components and notebook-compatible ACF/PACF through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const values = Float64Array.from({ length: 72 }, (_, row) => 0.04 * row + 2 * Math.sin(row * Math.PI / 3) + 0.1 * Math.cos(row * 0.7))
    const seasonal = await analysis.seasonalAdjustInWorker(Float64Array.from(values), 72, 1, { period: 6, robust: true, adjust: [0] })
    const structure = await analysis.runSeriesStructure(values, 72, 1, { period: 6, robust: true, correlationMaxLag: 12, peltMinSize: 4, peltJump: 1, peltPenalty: 10 })
    return { seasonal, structure }
  })
  const parsed = z.object({
    seasonal: z.discriminatedUnion('ok', [z.object({ ok: z.literal(true), value: seasonalAdjustedEvidenceSchema }).strict(), z.object({ ok: z.literal(false), error: z.unknown() }).strict()]),
    structure: z.discriminatedUnion('ok', [z.object({ ok: z.literal(true), value: seriesStructureEvidenceSchema }).strict(), z.object({ ok: z.literal(false), error: z.unknown() }).strict()]),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.seasonal.ok || !parsed.data.structure.ok) return
  const component = parsed.data.seasonal.value.adjusted[0]
  expect(component.observed).toHaveLength(72)
  expect(component.trend).toHaveLength(72)
  expect(component.seasonal).toHaveLength(72)
  expect(component.remainder).toHaveLength(72)
  expect(component.observed[20]).toBeCloseTo(component.trend[20] + component.seasonal[20] + component.remainder[20], 12)
  const correlation = parsed.data.structure.value.series[0]
  expect(correlation.acf).toHaveLength(13)
  expect(correlation.pacf).toHaveLength(13)
  expect(correlation.acfLimits).toHaveLength(13)
  expect(correlation.pacfLimits).toHaveLength(13)
})

test('runs CD-NOTS, CD-NOTS+ and GRACE through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Causal-TS boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 96
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      values[row] = Math.sin(row / 7) + row * 0.003
      values[rows + row] = (row === 0 ? 0 : values[row - 1]) + 0.1 * Math.cos(row / 5)
      values[2 * rows + row] = 0.6 * values[rows + row] + Math.sin(row / 11)
    }
    const withMissingCell = () => {
      const validity = new Uint8Array(rows * columns).fill(1)
      validity[10] = 0
      return validity
    }
    return {
      cdnots: await analysis.runCdnots(Float64Array.from(values), withMissingCell(), rows, columns, {
        maxLag: 1, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear',
      }),
      plus: await analysis.runCdnotsPlus(Float64Array.from(values), withMissingCell(), rows, columns, {
        maxLag: 1, alpha: 0.05, missing: 'varEm', context: 'linear',
      }),
      grace: await analysis.runGrace(Float64Array.from(values), withMissingCell(), rows, columns, {
        maxLag: 1, alpha: 0.05, context: 'linear', gateThreshold: 0.5, epochs: 3, patience: 3, seed: 7,
      }),
    }
  })
  const outcome = <Value extends z.ZodType>(value: Value) => z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value }).strict(),
    z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
  ])
  const parsed = z.object({
    cdnots: outcome(cdnotsEvidenceSchema),
    plus: outcome(cdnotsPlusEvidenceSchema),
    grace: outcome(graceEvidenceSchema),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.cdnots.ok, JSON.stringify(parsed.data.cdnots)).toBe(true)
  expect(parsed.data.plus.ok, JSON.stringify(parsed.data.plus)).toBe(true)
  expect(parsed.data.grace.ok, JSON.stringify(parsed.data.grace)).toBe(true)
  if (parsed.data.cdnots.ok) expect(parsed.data.cdnots.value.contextVariables).toEqual(['C_lin'])
  if (parsed.data.plus.ok) {
    expect(parsed.data.plus.value.contextVariables).toEqual(['C_lin'])
    expect(parsed.data.plus.value.missing).toBe('varEm')
  }
  if (parsed.data.grace.ok) {
    expect(parsed.data.grace.value.imputedCells).toBe(1)
    expect(parsed.data.grace.value.loss).toHaveLength(parsed.data.grace.value.epochs)
    expect(parsed.data.grace.value.gateValues).toHaveLength(3)
  }
})

test('runs PC-stable and FCI with background knowledge through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Cross-sectional constraint boundary runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 160
    const columns = 4
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      const x = Math.sin(row * 0.71) + 0.2 * Math.cos(row * 1.13)
      const y = Math.cos(row * 0.43) - 0.15 * Math.sin(row * 1.37)
      values[row] = x
      values[rows + row] = y
      values[2 * rows + row] = x + y + 0.05 * Math.sin(row * 2.03)
      values[3 * rows + row] = 0.7 * values[2 * rows + row] + 0.05 * Math.cos(row * 1.91)
    }
    const background = {
      forbidden: [[3, 2]],
      required: [[2, 3]],
      forbiddenPatterns: [],
      requiredPatterns: [],
      tiers: [0, 0, 1, 2],
      forbiddenWithinTiers: [],
    }
    const common = {
      names: ['X', 'Y', 'M', 'O'], alpha: 0.05, maxDepth: null, ciTest: 'fisherZ', background,
    } as const
    return {
      pc: await analysis.runPcStable(Float64Array.from(values), rows, columns, common),
      fci: await analysis.runFci(Float64Array.from(values), rows, columns, { ...common, maxPathLength: null }),
    }
  })
  const outcome = <Value extends z.ZodType>(value: Value) => z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value }).strict(),
    z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
  ])
  const parsed = z.object({
    pc: outcome(pcStableEvidenceSchema),
    fci: outcome(fciEvidenceSchema),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.pc.ok, JSON.stringify(parsed.data.pc)).toBe(true)
  expect(parsed.data.fci.ok, JSON.stringify(parsed.data.fci)).toBe(true)
  if (parsed.data.pc.ok) {
    expect(parsed.data.pc.value.graph).toHaveLength(4)
    expect(parsed.data.pc.value.ciTests.length).toBeGreaterThan(0)
  }
  if (parsed.data.fci.ok) {
    expect(parsed.data.fci.value.graph).toHaveLength(4)
    expect(parsed.data.fci.value.ciTests.length).toBeGreaterThan(0)
  }
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
    // Pure sinusoids make the lagged auxiliary regression rank deficient.
    let seed = 555
    const values = Float64Array.from({ length: 120 }, (_, index) => {
      seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0
      return 0.015 * index + Math.sin(index / 5) + 0.2 * Math.cos(index / 2.7)
        + 0.1 * (seed / 4294967296 - 0.5)
    })
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
      names: ['A', 'B', 'D', 'E', 'Z'],
      edges: [[0, 3], [0, 4], [1, 2], [1, 4], [3, 2], [4, 2], [4, 3]],
      treatment: 3,
      outcome: 2,
      unobserved: [],
      estimand: 'ate',
    })
  })
  expect(result).toEqual({
    ok: true,
    value: {
      kind: 'backdoorIdentification',
      frontdoor: { kind: 'notIdentified' },
      instruments: { kind: 'notIdentified' },
      nodes: 5,
      treatment: 3,
      outcome: 2,
      unobserved: [],
      graphicalIdentification: {
        kind: 'identified',
        expression: 'Sum[A, B, Z](P(D | A, B, E, Z) * P(Z | A, B) * Sum[A, D, E, Z](P(A, B, D, E, Z)) * Sum[B, D, E, Z](P(A, B, D, E, Z)))',
        latex: String.raw`\sum_{A, B, Z} P(D \mid A, B, E, Z) \; P(Z \mid A, B) \; \sum_{A, D, E, Z} P(A, B, D, E, Z) \; \sum_{B, D, E, Z} P(A, B, D, E, Z)`,
        projection: {
          directedEdges: [[0, 3], [0, 4], [1, 2], [1, 4], [3, 2], [4, 2], [4, 3]],
          bidirectedEdges: [],
        },
      },
      counterfactualIdentification: { kind: 'notApplicable' },
      result: {
        kind: 'identified',
        canonicalSet: [0, 1, 4],
        minimalSets: [[0, 4], [1, 4]],
        truncated: false,
      },
    },
  })
})

test('identifies front-door when no measured back-door set exists', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture contract runs once')
  await page.goto('/app')
  const result: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    // U -> X, U -> Y, X -> M -> Y. U is unmeasured.
    return analysis.identifyBackdoor({
      nodes: 4,
      names: ['X', 'M', 'Y', 'U'],
      edges: [[3, 0], [3, 2], [0, 1], [1, 2]],
      treatment: 0,
      outcome: 2,
      unobserved: [3],
      estimand: 'ate',
    })
  })
  expect(result).toEqual({
    ok: true,
    value: {
      kind: 'backdoorIdentification',
      frontdoor: { kind: 'identified', mediators: [1] },
      instruments: { kind: 'notIdentified' },
      nodes: 4,
      treatment: 0,
      outcome: 2,
      unobserved: [3],
      result: { kind: 'notIdentified' },
      graphicalIdentification: {
        kind: 'identified',
        expression: 'Sum[M](P(M | X) * Sum[X](P(X) * P(Y | M, X)))',
        latex: String.raw`\sum_{M} P(M \mid X) \; \sum_{X} P(X) \; P(Y \mid M, X)`,
        projection: {
          directedEdges: [[0, 1], [1, 2]],
          bidirectedEdges: [[0, 2]],
        },
      },
      counterfactualIdentification: { kind: 'notApplicable' },
    },
  })
})

test('identifies and evaluates binary ETT through IDC*', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const identification = await analysis.identifyBackdoor({
      nodes: 3,
      names: ['Z', 'X', 'Y'],
      edges: [[0, 1], [0, 2], [1, 2]],
      treatment: 1,
      outcome: 2,
      unobserved: [],
      estimand: 'att',
    })
    const cells = [
      [0, 0, 0, 30], [0, 0, 1, 10], [0, 1, 0, 5], [0, 1, 1, 5],
      [1, 0, 0, 5], [1, 0, 1, 5], [1, 1, 0, 10], [1, 1, 1, 30],
    ] as const
    const zValues: number[] = []
    const xValues: number[] = []
    const yValues: number[] = []
    for (const [z, x, y, count] of cells) for (let index = 0; index < count; index += 1) { zValues.push(z); xValues.push(x); yValues.push(y) }
    const values = Float64Array.from([...zValues, ...xValues, ...yValues])
    const estimate = await analysis.runBinaryEtt(values, zValues.length, 3, {
      observedNodes: [0, 1, 2],
      names: ['Z', 'X', 'Y'],
      edges: [[0, 1], [0, 2], [1, 2]],
      treatment: 1,
      outcome: 2,
      unobserved: [],
    })
    return { identification, estimate, detachedBytes: values.byteLength }
  })
  const parsed = z.object({
    identification: z.object({ ok: z.literal(true), value: z.object({
      counterfactualIdentification: z.object({ kind: z.literal('identified'), treatedExpression: z.string().min(1), untreatedExpression: z.string().min(1) }).strict(),
    }).passthrough() }).strict(),
    estimate: z.object({ ok: z.literal(true), value: binaryEttEvidenceSchema }).strict(),
    detachedBytes: z.literal(0),
  }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.estimate.value.treatedPotentialOutcomeMean).toBeCloseTo(0.70, 12)
  expect(parsed.data.estimate.value.untreatedPotentialOutcomeMean).toBeCloseTo(0.45, 12)
  expect(parsed.data.estimate.value.effectOnTreated).toBeCloseTo(0.25, 12)
  expect(parsed.data.estimate.value.treatedExpression).toBe(parsed.data.identification.value.counterfactualIdentification.treatedExpression)
  expect(parsed.data.estimate.value.untreatedExpression).toBe(parsed.data.identification.value.counterfactualIdentification.untreatedExpression)
  expect(parsed.data.estimate.value.treatedExpression).not.toBe(parsed.data.estimate.value.untreatedExpression)
  expect(parsed.data.estimate.value.treatedExpression).toContain('Y @ +X: +Y')
  expect(parsed.data.estimate.value.untreatedExpression).toContain('Y @ -X: +Y')
})

test('runs the front-door estimator through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 96
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      // Six repeats of a 2^4 factorial keep u and all 3 disturbances exactly orthogonal.
      const cell = row % 16
      const sign = (bit: number) => ((cell >> bit) & 1) === 0 ? -1 : 1
      const u = sign(0)
      const treatment = u + sign(1)
      const mediator = 2 * treatment + sign(2)
      const outcome = 3 * mediator + 4 * u + sign(3)
      values[row] = treatment
      values[rows + row] = mediator
      values[2 * rows + row] = outcome
    }
    const progress: unknown[] = []
    const result = await analysis.runFrontdoorTwoStage(values, rows, columns, {
      treatment: 0,
      mediator: 1,
      outcome: 2,
      firstStageAdjustment: [],
      secondStageAdjustment: [0],
      controlValue: 0,
      treatmentValue: 1,
      uncertainty: { kind: 'bootstrap', simulations: 20, sampleSizeFraction: 1, confidenceLevel: 0.95, seed: 0 },
    }, (event: unknown) => progress.push(event))
    return { result, progress, detachedBytes: values.byteLength }
  })
  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: frontdoorTwoStageEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.result.ok) return
  expect(parsed.data.detachedBytes).toBe(0)
  expect(parsed.data.result.value.firstStageEffect).toBeCloseTo(2, 1)
  expect(parsed.data.result.value.secondStageEffect).toBeCloseTo(3, 1)
  expect(parsed.data.result.value.estimate).toBeCloseTo(6, 0)
  expect(parsed.data.result.value.uncertainty.kind).toBe('bootstrap')
  expect(parsed.data.progress[0]).toEqual({ stage: 'frontdoor-bootstrap', completed: 0, total: 20 })
  expect(parsed.data.progress.at(-1)).toEqual({ stage: 'frontdoor-bootstrap', completed: 20, total: 20 })
})

test('identifies an instrument when a latent common cause blocks every other strategy', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Architecture contract runs once')
  await page.goto('/app')
  const result: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    // Z -> X -> Y with U -> X and U -> Y; U is unmeasured, so Z is the only route.
    return analysis.identifyBackdoor({
      nodes: 4,
      names: ['Z', 'X', 'Y', 'U'],
      edges: [[0, 1], [1, 2], [3, 1], [3, 2]],
      treatment: 1,
      outcome: 2,
      unobserved: [3],
      estimand: 'ate',
    })
  })
  const parsed = z.object({ ok: z.literal(true), value: z.object({ instruments: z.unknown(), frontdoor: z.unknown(), result: z.unknown(), graphicalIdentification: z.object({ kind: z.string() }).passthrough() }).passthrough() }).safeParse(result)
  expect(parsed.success).toBe(true)
  if (!parsed.success) return
  expect(parsed.data.value.instruments).toEqual({ kind: 'identified', instruments: [0] })
  expect(parsed.data.value.frontdoor).toEqual({ kind: 'notIdentified' })
  expect(parsed.data.value.result).toEqual({ kind: 'notIdentified' })
  expect(parsed.data.value.graphicalIdentification.kind).toBe('unidentifiable')
})

test('runs the instrumental-variable estimator through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 96
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      // Six repeats of a 2^4 factorial: a binary instrument z, a latent u, and two disturbances, all orthogonal.
      const cell = row % 16
      const sign = (bit: number) => ((cell >> bit) & 1) === 0 ? -1 : 1
      const z = ((cell >> 0) & 1)
      const u = sign(1)
      const treatment = 2 * z + u + 0.5 * sign(2)
      const outcome = 3 * treatment + 4 * u + sign(3)
      values[row] = treatment
      values[rows + row] = outcome
      values[2 * rows + row] = z
    }
    const progress: unknown[] = []
    const result = await analysis.runInstrumentalVariable(values, rows, columns, {
      treatment: 0,
      outcome: 1,
      instruments: [2],
      uncertainty: { kind: 'bootstrap', simulations: 20, sampleSizeFraction: 1, confidenceLevel: 0.95, seed: 0 },
    }, (event: unknown) => progress.push(event))
    return { result, progress, detachedBytes: values.byteLength }
  })
  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: instrumentalVariableEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.result.ok) return
  expect(parsed.data.detachedBytes).toBe(0)
  expect(parsed.data.result.value.route).toBe('waldRatio')
  // The Wald ratio removes the latent confounding exactly: (6 outcome units) / (2 treatment units).
  expect(parsed.data.result.value.estimate).toBeCloseTo(3, 10)
  expect(parsed.data.result.value.uncertainty.kind).toBe('bootstrap')
  expect(parsed.data.result.value.standardError).not.toBeNull()
  expect(parsed.data.progress[0]).toEqual({ stage: 'instrumental-variable-bootstrap', completed: 0, total: 20 })
  expect(parsed.data.progress.at(-1)).toEqual({ stage: 'instrumental-variable-bootstrap', completed: 20, total: 20 })
})

test('runs ID, IDC and hedge outcomes through the discrete intervention boundary', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const completeBinary = (variables: number): { rows: number; values: Float64Array } => {
      const repeats = 8
      const rows = (2 ** variables) * repeats
      const values = new Float64Array(rows * variables)
      for (let variable = 0; variable < variables; variable += 1) {
        for (let pattern = 0; pattern < 2 ** variables; pattern += 1) {
          for (let repeat = 0; repeat < repeats; repeat += 1) values[variable * rows + pattern * repeats + repeat] = (pattern >> variable) & 1
        }
      }
      return { rows, values }
    }
    const frontdoorData = completeBinary(3)
    const frontdoor = await analysis.runIdentifiedDiscreteQuery(frontdoorData.values, frontdoorData.rows, 3, {
      observedNodes: [0, 1, 2], names: ['U', 'X', 'M', 'Y'], edges: [[0, 1], [0, 3], [1, 2], [2, 3]],
      treatment: 1, outcome: 3, unobserved: [0], bins: 2, condition: null,
    })
    const conditionalData = completeBinary(3)
    const conditional = await analysis.runIdentifiedDiscreteQuery(conditionalData.values, conditionalData.rows, 3, {
      observedNodes: [0, 1, 2], names: ['Z', 'X', 'Y'], edges: [[0, 1], [0, 2], [1, 2]],
      treatment: 1, outcome: 2, unobserved: [], bins: 2, condition: { variable: 0, state: 1 },
    })
    const hedgeData = completeBinary(2)
    const hedge = await analysis.runIdentifiedDiscreteQuery(hedgeData.values, hedgeData.rows, 2, {
      observedNodes: [0, 1], names: ['U', 'X', 'Y'], edges: [[0, 1], [0, 2], [1, 2]],
      treatment: 1, outcome: 2, unobserved: [0], bins: 2, condition: null,
    })
    return { frontdoor, conditional, hedge }
  })
  const outcome = z.discriminatedUnion('ok', [
    z.object({ ok: z.literal(true), value: identifiedDiscreteQueryEvidenceSchema }).strict(),
    z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
  ])
  const parsed = z.object({ frontdoor: outcome, conditional: outcome, hedge: outcome }).strict().safeParse(raw)
  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.frontdoor.ok || !parsed.data.conditional.ok || !parsed.data.hedge.ok) return
  expect(parsed.data.frontdoor.value.result.kind).toBe('identified')
  if (parsed.data.frontdoor.value.result.kind === 'identified') {
    expect(parsed.data.frontdoor.value.result.algorithm).toBe('ID')
    expect(parsed.data.frontdoor.value.result.normalizationLow).toBeCloseTo(1, 12)
    expect(parsed.data.frontdoor.value.result.normalizationHigh).toBeCloseTo(1, 12)
  }
  expect(parsed.data.conditional.value.query.kind).toBe('conditional')
  expect(parsed.data.conditional.value.result.kind).toBe('identified')
  if (parsed.data.conditional.value.result.kind === 'identified') expect(parsed.data.conditional.value.result.algorithm).toBe('IDC')
  expect(parsed.data.hedge.value.result.kind).toBe('unidentifiable')
  if (parsed.data.hedge.value.result.kind === 'unidentifiable') expect(parsed.data.hedge.value.result.hedgeGraph.length).toBeGreaterThan(0)
})

test('runs the seeded CausalEffects block bootstrap through the Rust worker', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 120
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 1; row < rows; row += 1) {
      const z = 0.4 * values[2 * rows + row - 1] + 0.3 * Math.sin(row * 1.17)
      const x = 0.5 * z + 0.2 * Math.cos(row * 0.73)
      const y = 0.7 * values[row - 1] + 0.35 * z + 0.2 * Math.sin(row * 0.41)
      values[row] = x
      values[rows + row] = y
      values[2 * rows + row] = z
    }
    const graph = Array.from({ length: columns }, () => Array.from({ length: columns }, () => ['', '']))
    graph[0][1][1] = '-->'
    graph[2][0][1] = '-->'
    graph[2][1][1] = '-->'
    graph[0][0][1] = '-->'
    graph[1][1][1] = '-->'
    graph[2][2][1] = '-->'
    const progress: unknown[] = []
    const result = await analysis.runCausalEffectsTotal(values, rows, columns, {
      statLag: 1,
      graph,
      x: [[0, -1]],
      y: [[1, 0]],
      hidden: [],
      estimator: { kind: 'linear', adjustment: { kind: 'optimal' } },
      interventions: [0, 1],
      uncertainty: { kind: 'bootstrap', samples: 20, blockLength: { kind: 'fixed', length: 4 }, confidenceLevel: 0.9, seed: 4 },
    }, (event: unknown) => progress.push(event))
    return { result, progress, detachedBytes: values.byteLength }
  })
  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: causalEffectsEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.result.ok) return
  expect(parsed.data.detachedBytes).toBe(0)
  expect(parsed.data.result.value.identifiable).toBe(true)
  expect(parsed.data.result.value.fit.kind).toBe('adjustedLinear')
  if (parsed.data.result.value.fit.kind === 'adjustedLinear') {
    expect(parsed.data.result.value.fit.selection).toEqual({ kind: 'optimal' })
  }
  expect(parsed.data.result.value.uncertainty.kind).toBe('bootstrap')
  if (parsed.data.result.value.uncertainty.kind !== 'bootstrap') return
  expect(parsed.data.result.value.uncertainty.effectDraws).toHaveLength(20)
  expect(parsed.data.result.value.uncertainty.resolvedBlockLength).toBe(4)
  expect(parsed.data.progress[0]).toEqual({ stage: 'causal-effects-bootstrap', completed: 0, total: 20 })
  expect(parsed.data.progress.at(-1)).toEqual({ stage: 'causal-effects-bootstrap', completed: 20, total: 20 })
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

test('runs cross-sectional DirectLiNGAM and reports its numerical progress', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Numerical boundary contract runs once')
  await page.goto('/app')
  const raw: unknown = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rows = 96
    const columns = 3
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row += 1) {
      const x = ((row * 37 % 101) - 50) / 25
      const yNoise = ((row * 61 % 103) - 51) / 30
      const zNoise = ((row * 73 % 107) - 53) / 35
      values[row] = x
      values[rows + row] = 0.8 * x + yNoise
      values[2 * rows + row] = -0.5 * values[rows + row] + zNoise
    }
    const progress: unknown[] = []
    const result = await analysis.runDirectLingam(values, rows, columns, (event: unknown) => progress.push(event))
    return { result, progress, detachedBytes: values.byteLength }
  })
  const parsed = z.object({
    result: z.discriminatedUnion('ok', [
      z.object({ ok: z.literal(true), value: directLingamEvidenceSchema }).strict(),
      z.object({ ok: z.literal(false), error: z.unknown() }).strict(),
    ]),
    progress: z.array(z.object({ stage: z.string(), completed: z.number(), total: z.number() }).strict()),
    detachedBytes: z.number().int().nonnegative(),
  }).strict().safeParse(raw)

  expect(parsed.success).toBe(true)
  if (!parsed.success || !parsed.data.result.ok) return
  expect(parsed.data.detachedBytes).toBe(0)
  expect([...parsed.data.result.value.causalOrder].sort((left, right) => left - right)).toEqual([0, 1, 2])
  expect(parsed.data.result.value.weights).toHaveLength(3)
  expect(parsed.data.result.value.weights.every((row) => row.length === 3)).toBe(true)
  expect(parsed.data.progress.some((event) => event.stage === 'causal-order')).toBe(true)
  expect(parsed.data.progress.some((event) => event.stage === 'adaptive-lasso')).toBe(true)
  expect(parsed.data.progress.at(-1)).toEqual({ stage: 'complete', completed: 4, total: 4 })
})
