import { expect, test } from '@playwright/test'
import { z } from 'zod'
import { stationarityBatterySchema } from '../src/domain/stationarity'
import { dagCheckEvidenceSchema } from '../src/domain/dagValidation'
import { directLingamEvidenceSchema } from '../src/domain/discovery'
import { identifiedDiscreteQueryEvidenceSchema } from '../src/domain/intervention'
import { binaryEttEvidenceSchema, causalEffectsEvidenceSchema, frontdoorTwoStageEvidenceSchema } from '../src/domain/estimation'
import { seasonalAdjustedEvidenceSchema } from '../src/domain/seasonal'
import { parseSeriesStructureEvidence, seriesStructureEvidenceSchema } from '../src/domain/sensitivity'

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
