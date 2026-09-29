import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('Pooled search retains oracle selection after cancellation and failure', async ({ page }) => {
  const oracle = JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/sklearn_grid_search_kernel.json', 'utf8'))
  await page.goto('/app')
  const result = await page.evaluate(async (oracle) => {
    const { runBoostedGridSearch } = await import(new URL('/src/analysis/boostedSearch.ts', location.href).href)
    const { cancelAnalysisRuns, runPropensityWeighting, runPropensityMatching } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const x = Array.from({ length: oracle.rows }, (_, row) => Array.from({ length: oracle.columns }, (_, col) => (row * (col + 3) + col * col) % 29))
    const treatment = x.map(row => Number(((row[0] * 2 + row[1] - row[3]) % 11 + 11) % 11 > 5))
    const outcome = x.map((row, i) => row[2] + 2 * treatment[i])
    const values = new Float64Array([...treatment, ...outcome, ...Array.from({ length: oracle.columns }, (_, col) => x.map(row => row[col])).flat()])
    const design = { treatment: 0, outcome: 1, adjustment: [2, 3, 4, 5, 6] }
    const grid = { learningRate: oracle.grid.learning_rate, maxDepth: oracle.grid.max_depth, nEstimators: oracle.grid.n_estimators,
      splits: 5, minSamplesLeaf: oracle.min_samples_leaf, minSamplesSplit: 2, seed: oracle.random_state }
    const search = (columns = 7) => runBoostedGridSearch(values, oracle.rows, columns, design, grid)
    const pending = search()
    cancelAnalysisRuns()
    const cancelled = await pending
    const failed = await search(1)
    const first = await search()
    const second = await search()
    if (!first.ok) return { cancelled, failed, first, second, ipw: null, matching: null }
    const best = first.value.best
    const model = { ...grid, learningRate: [best.learningRate], maxDepth: [best.maxDepth], nEstimators: [best.nEstimators], scoring: 'one-model', candidatesSearched: 12 }
    const ipw = await runPropensityWeighting(values.slice(), oracle.rows, 7, { ...design, scale: 'inverseProbability', fit: { kind: 'boosted', model } })
    const matching = await runPropensityMatching(values.slice(), oracle.rows, 7, { ...design, model: { kind: 'boosted', model } })
    return { cancelled, failed, first, second, ipw, matching }
  }, oracle)
  expect(result.cancelled).toMatchObject({ ok: false, error: { kind: 'analysis-cancelled' } })
  expect(result.failed.ok).toBe(false)
  expect(result.first.ok, JSON.stringify(result.first)).toBe(true)
  expect(result.second).toEqual(result.first)
  expect(result.first.value.best).toEqual({ learningRate: oracle.best_params.learning_rate, maxDepth: oracle.best_params.max_depth, nEstimators: oracle.best_params.n_estimators, meanScore: oracle.best_score })
  expect(result.ipw?.ok, JSON.stringify(result.ipw)).toBe(true)
  expect(result.matching?.ok, JSON.stringify(result.matching)).toBe(true)
  expect(result.ipw?.value.propensity).toEqual(result.matching?.value.propensity)
})

test('Cancel stops every pooled propensity fit and settles the search', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { runBoostedGridSearch } = await import(new URL('/src/analysis/boostedSearch.ts', location.href).href)
    const { cancelAnalysisRuns } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const original = window.Worker
    const workers: { terminated: boolean; posted: boolean }[] = []
    class HeldWorker {
      onmessage = null
      onerror = null
      terminated = false
      posted = false
      constructor() { workers.push(this) }
      postMessage() { this.posted = true }
      terminate() { this.terminated = true }
    }
    // Hold the workers at the execution boundary, without timing a large numerical fit.
    Object.defineProperty(window, 'Worker', { value: HeldWorker, configurable: true, writable: true })
    try {
      const pending = runBoostedGridSearch(new Float64Array(80), 20, 4,
        { treatment: 0, outcome: 1, adjustment: [2, 3] },
        { learningRate: [.05, .1], maxDepth: [1, 2], nEstimators: [10], splits: 2, minSamplesLeaf: 2, minSamplesSplit: 2, seed: 7 })
      await Promise.resolve()
      cancelAnalysisRuns()
      const outcome = await Promise.race([
        pending,
        new Promise<null>(resolve => setTimeout(() => resolve(null), 1000)),
      ])
      return { outcome, started: workers.filter(w => w.posted).length, terminated: workers.every(w => w.terminated) }
    } finally {
      Object.defineProperty(window, 'Worker', { value: original, configurable: true, writable: true })
    }
  })
  expect(result.started).toBeGreaterThan(0)
  expect(result.outcome).toMatchObject({ ok: false, error: { kind: 'analysis-cancelled' } })
  expect(result.terminated).toBe(true)
})
