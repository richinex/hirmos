import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
const fixture = JSON.parse(readFileSync('crates/causal-core/oracle/grf/fixtures/nuisance.json','utf8')).cases[0]

test('pooled forest tuning matches serial tuning and cancels every worker', async ({page}) => {
  test.setTimeout(600_000)
  await page.goto('/app')
  const result = await page.evaluate(async source => {
    const client = await import(new URL('/src/analysis/client.ts',location.href).href)
    const domain = await import(new URL('/src/domain/causalForest.ts',location.href).href)
    const wasm = await import(new URL('/src/generated/analysis-wasm/hirmos_analysis.js',location.href).href)
    await wasm.default()
    const values = new Float64Array([source.W,source.Y,...source.X].flat().map(Number))
    const config = {...domain.DEFAULT_CAUSAL_FOREST,trees:100,seed:source.seed,tuning:{kind:'all',trees:30,repetitions:12,draws:30}}
    const design = {treatment:0,outcome:1,adjustment:[2,3,4,5],target:{kind:'binary-average',population:'all'},configuration:config}
    const serial = JSON.parse(wasm.runAnalysis(JSON.stringify({kind:'causalForest',rows:source.Y.length,columns:6,...design}),values,new Uint8Array(),new Uint8Array(),()=>{})).evidence
    const workers: Worker[] = []
    const Original = window.Worker
    window.Worker = class extends Original { constructor(url: URL|string, options?: WorkerOptions) { super(url,options); workers.push(this) } }
    try {
      const pooled = await client.runCausalForest(values.slice(),source.Y.length,6,design)
      if (!pooled.ok) throw new Error(JSON.stringify(pooled.error))
      const count = workers.length
      const active: Worker[] = []
      const terminated = new Set<Worker>()
      window.Worker = class extends Original {
        constructor(url:URL|string,options?:WorkerOptions) {super(url,options);active.push(this)}
        terminate() {terminated.add(this);super.terminate()}
      }
      const pending = client.runCausalForest(values.slice(),source.Y.length,6,design)
      await new Promise(resolve=>setTimeout(resolve,50))
      client.cancelAnalysisRuns()
      const cancelled = await pending
      return {equal:JSON.stringify(serial)===JSON.stringify(pooled.value),workers:count,
        cancelled:!cancelled.ok && cancelled.error.kind==='analysis-cancelled',allTerminated:active.length>0&&active.every(w=>terminated.has(w))}
    } finally {window.Worker=Original}
  },fixture)
  expect(result.equal).toBe(true)
  expect(result.workers).toBeGreaterThan(1)
  expect(result.cancelled).toBe(true)
  expect(result.allTerminated).toBe(true)
})
