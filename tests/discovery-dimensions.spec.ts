import { expect, test } from '@playwright/test'
import * as discovery from '../src/domain/discovery'
import { parseAnalysisWorkerCommand } from '../src/workers/analysisProtocol'

test('discovery evidence permits dimensions beyond former browser caps and rejects invalid counts', () => {
  const dimensions = [
    discovery.pcmciPlusEvidenceSchema.shape.variables,
    discovery.lpcmciEvidenceSchema.shape.variables,
    discovery.jpcmciPlusEvidenceSchema.shape.observedVariables,
    discovery.jpcmciPlusEvidenceSchema.shape.variables,
    discovery.rpcmciEvidenceSchema.shape.variables,
    discovery.cdnotsEvidenceSchema.shape.observedVariables,
    discovery.cdnotsPlusEvidenceSchema.shape.observedVariables,
    discovery.graceEvidenceSchema.shape.variables,
    discovery.pcStableEvidenceSchema.shape.variables,
    discovery.fciEvidenceSchema.shape.variables,
    discovery.directLingamEvidenceSchema.shape.variables,
    discovery.varLingamEvidenceSchema.shape.variables,
    discovery.dynotearsEvidenceSchema.shape.variables,
    discovery.ocseEvidenceSchema.shape.variables,
    discovery.cmlpEvidenceSchema.shape.variables,
    discovery.clstmEvidenceSchema.shape.variables,
  ]
  for (const schema of dimensions) {
    for (const count of [13, 33, 54, 128]) expect(schema.safeParse(count).success).toBe(true)
    for (const count of [0, 1, 2.5, Infinity, NaN]) expect(schema.safeParse(count).success).toBe(false)
  }
})

test('large discovery commands retain matrix and parameter validation', () => {
  const rows = 180, columns = 54
  const base = { request: '00000000-0000-4000-8000-000000000001', rows, columns, values: new Float64Array(rows * columns) }
  const background = { forbidden: [], required: [], forbiddenPatterns: [], requiredPatterns: [], tiers: Array(columns).fill(null), forbiddenWithinTiers: [] }
  const configurations = [
    { kind: 'cdnots', validity: new Uint8Array(rows * columns).fill(1), maxLag: 1, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear' },
    { kind: 'cdnots-plus', validity: new Uint8Array(rows * columns).fill(1), maxLag: 1, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear' },
    { kind: 'direct-lingam' },
    { kind: 'dynotears', maxLag: 1, lambdaW: 0.1, lambdaA: 0.1 },
    { kind: 'var-lingam', lags: 1, prune: true },
    { kind: 'pcmci-plus', tauMax: 1, pcAlpha: 0.05, samples: { kind: 'dense' } },
    { kind: 'lpcmci', tauMax: 1, pcAlpha: 0.05, samples: { kind: 'dense' } },
    { kind: 'rpcmci', numRegimes: 2, maxTransitions: 2, switchThres: 0.05, numIterations: 2, maxAnneal: 1, tauMin: 1, tauMax: 1, pcAlpha: 0.05, alphaLevel: 0.05, seed: 7 },
    { kind: 'grace', validity: new Uint8Array(rows * columns).fill(1), maxLag: 1, alpha: 0.05, context: 'linear', gateThreshold: 0.5, epochs: 2, patience: 2, seed: 7 },
    { kind: 'ocse', maxLag: 1, alpha: 0.05, nShuffles: 20, method: 'gaussian', k: 3 },
    { kind: 'cmlp', lag: 1, hidden: [4], activation: 'relu', penalty: 'groupLasso', lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 2, checkEvery: 1, lookback: 1, seed: 7 },
    { kind: 'clstm', context: 2, hidden: 4, lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 2, checkEvery: 1, lookback: 1, seed: 7 },
    { kind: 'pc-stable', names: Array.from({ length: columns }, (_, i) => `x${i}`), alpha: 0.05, maxDepth: null, ciTest: 'kci', background },
    { kind: 'fci', names: Array.from({ length: columns }, (_, i) => `x${i}`), alpha: 0.05, maxDepth: null, maxPathLength: null, ciTest: 'fisherZ', background },
  ]
  for (const configuration of configurations) {
    const command = { ...base, ...configuration }
    expect(parseAnalysisWorkerCommand(command).ok, configuration.kind).toBe(true)
    expect(parseAnalysisWorkerCommand({ ...command, values: new Float64Array(3) }).ok).toBe(false)
    expect(parseAnalysisWorkerCommand({ ...command, columns: 1 }).ok).toBe(false)
  }
  const panel = { kind: 'jpcmci-plus', request: base.request, values: base.values, rows, datasets: 2, periods: 90, observedColumns: columns, classes: Array(columns).fill('system'), timeDummy: true, spaceDummy: true, tauMax: 1, pcAlpha: 0.05 }
  expect(parseAnalysisWorkerCommand(panel).ok).toBe(true)
  expect(parseAnalysisWorkerCommand({ ...panel, classes: ['system'] }).ok).toBe(false)
})

test('54-variable CD-NOTS results cross the rebuilt WASM and worker boundaries', async ({ page }) => {
  test.setTimeout(180_000)
  await page.goto('/app')
  const outcomes = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
    const rows = 180, columns = 54
    let state = 347
    const values = Float64Array.from({ length: rows * columns }, () => {
      state = (Math.imul(state, 1664525) + 1013904223) >>> 0
      return state / 4294967296 - 0.5
    })
    const configuration = { maxLag: 1, alpha: 0.01, missing: 'pairwiseComplete', context: 'none' }
    const results = []
    for (const run of [analysis.runCdnots, analysis.runCdnotsPlus]) {
      const start = performance.now()
      const result = await run(values.slice(), new Uint8Array(values.length).fill(1), rows, columns, configuration)
      results.push({ result, elapsed: performance.now() - start })
    }
    return results
  })
  for (const outcome of outcomes) {
    expect(outcome.result.ok, JSON.stringify(outcome.result)).toBe(true)
    if (outcome.result.ok) {
      expect(outcome.result.value.observedVariables).toBe(54)
      expect(outcome.result.value.graph).toHaveLength(54)
    }
  }
})

for (const width of [320, 360, 393, 412]) {
  test(`mobile import routes share one row at ${width}px`, async ({ page }, info) => {
    await page.setViewportSize({ width, height: 850 })
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill(`Mobile import ${width}`)
    await page.getByRole('button', { name: 'Create project', exact: true }).click()
    const group = page.getByRole('radiogroup', { name: 'Data input method' })
    const boxes = await group.locator('[data-segment-option]').evaluateAll(nodes => nodes.map(node => {
      const r = node.getBoundingClientRect()
      return { top: r.top, left: r.left, right: r.right, width: r.width, scroll: node.scrollWidth, client: node.clientWidth }
    }))
    expect(boxes).toHaveLength(3)
    expect(Math.max(...boxes.map(b => b.top)) - Math.min(...boxes.map(b => b.top))).toBeLessThan(1)
    for (const box of boxes) { expect(box.left).toBeGreaterThanOrEqual(0); expect(box.right).toBeLessThanOrEqual(width); expect(box.scroll).toBeLessThanOrEqual(box.client + 1) }
    await page.screenshot({ path: info.outputPath(`import-${width}.png`), animations: 'disabled' })
    for (const name of ['Prepare with SQL', 'Build a pipeline', 'Upload a file']) {
      await group.getByRole('radio', { name, exact: true }).check()
      await expect(group.getByRole('radio', { name, exact: true })).toBeChecked()
    }
  })
}

test('larger discovery remains cancellable and a fresh worker can run afterwards', async ({ page }) => {
  test.setTimeout(60_000)
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
    const rows = 180, columns = 54
    const values = Float64Array.from({ length: rows * columns }, (_, i) => Math.sin(i * 0.917) + Math.cos(i * 0.317))
    const config = { maxLag: 1, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear' }
    let cancelledAfterProgress = false
    const cancelled = await analysis.runCdnotsPlus(values, new Uint8Array(values.length).fill(1), rows, columns, config, () => {
      if (!cancelledAfterProgress) { cancelledAfterProgress = true; analysis.cancelAnalysisRuns() }
    })
    const small = Float64Array.from({ length: rows * 2 }, (_, i) => Math.sin(i * 0.7) + Math.cos(i * 0.3))
    const recovered = await analysis.runCdnots(small, new Uint8Array(small.length).fill(1), rows, 2, config)
    return { cancelled, recovered, cancelledAfterProgress }
  })
  expect(results.cancelledAfterProgress).toBe(true)
  expect(results.cancelled.ok).toBe(false)
  if (!results.cancelled.ok) expect(results.cancelled.error.kind).toBe('analysis-cancelled')
  expect(results.recovered.ok, JSON.stringify(results.recovered)).toBe(true)
})
