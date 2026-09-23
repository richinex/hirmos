import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('Bayesian impact worker preserves seeded evidence, intervals and saved method identity', async ({ page }) => {
  await page.goto('/app')
  const csv = readFileSync('tests/fixtures/chapter-impact.csv','utf8')
  const result = await page.evaluate(async (source) => {
    const analysis = await import(new URL('/src/analysis/client.ts',location.href).href)
    const domain = await import(new URL('/src/domain/estimation.ts',location.href).href)
    const persistence = await import(new URL('/src/domain/persistence.ts',location.href).href)
    const rows = source.trim().split('\n').slice(1).map((line:string) => line.split(',').map(Number))
    const values = new Float64Array([2,3,4].flatMap((column) => rows.map((row:number[]) => row[column])))
    const settings = { kind:'bayesian',draws:900,warmup:100,seed:1234,priorLevelSd:0.01 }
    const design = { outcome:0,controls:[1,2],nPre:15,postEnd:rows.length,draws:settings.draws,warmup:settings.warmup,seed:settings.seed,priorLevelSd:settings.priorLevelSd }
    const fitted = await analysis.runBayesianCausalImpact(values.slice(),rows.length,3,design)
    if (!fitted.ok) throw new Error(JSON.stringify(fitted.error))
    const again = await analysis.runBayesianCausalImpact(values.slice(),rows.length,3,design)
    const legacy = await analysis.runCausalImpact(values.slice(),rows.length,3,{outcome:0,controls:[1,2],nPre:15,postEnd:rows.length,maxIter:100})
    if (!legacy.ok) throw new Error(JSON.stringify(legacy.error))
    const configuration = {kind:'causal-impact',start:{kind:'row',row:16},controls:['x1','x2'],inference:settings}
    const run = {id:'bayesian-impact',kind:'causal-impact-run',configuration,evidence:fitted.value}
    const oldRun = {id:'ml-impact',kind:'causal-impact-run',configuration:{kind:'causal-impact',start:{kind:'row',row:16},controls:['x1','x2'],maxIter:100},evidence:legacy.value}
    const snapshot = {
      kind:'hirmos-project',version:1,savedAt:new Date().toISOString(),origin:{kind:'user'},
      project:{id:'impact-project',name:'Chapter impact',createdAt:new Date().toISOString()},
      source:null,profile:null,prepared:null,stationarity:null,discoveryRuns:[],dagDocuments:[],
      studyDraft:{},studies:[],identifications:[],estimationRuns:[run,oldRun],sensitivityRuns:[],counterfactualRuns:[],
    }
    const saved = persistence.parseSnapshotValue(snapshot)
    const mismatch = persistence.parseSnapshotValue({...snapshot,estimationRuns:[{...run,configuration:oldRun.configuration}]})
    const changedWindow = persistence.parseSnapshotValue({...snapshot,estimationRuns:[{...run,configuration:{...configuration,start:{kind:'row',row:17}}}]})
    const changedControls = persistence.parseSnapshotValue({...snapshot,estimationRuns:[{...run,configuration:{...configuration,controls:['x1']}}]})
    const badBand = domain.parseCausalImpactEvidence({...fitted.value,counterfactualLower:[]})
    const study = {estimand:{kind:'average-treatment-effect',scale:'additive'}}
    const identification = {result:{kind:'identified',adjustment:{kind:'canonical',variables:[]}}}
    const estimate = domain.causalEstimateFrom(study,identification,run)
    return {fitted,again,saved,mismatch,changedWindow,changedControls,badBand,estimate}
  },csv)
  expect(result.fitted.ok).toBe(true)
  expect(result.again).toEqual(result.fitted)
  const evidence = result.fitted.value
  expect(evidence.kind).toBe('bayesianCausalImpact')
  expect(evidence.controlInclusion).toEqual([{column:1,probability:1},{column:2,probability:1}])
  expect(evidence.counterfactual).toHaveLength(16)
  expect(evidence.average).toBeGreaterThan(190)
  expect(evidence.average).toBeLessThan(240)
  expect(result.saved.ok).toBe(true)
  expect(result.mismatch.ok).toBe(false)
  expect(result.changedWindow.ok).toBe(false)
  expect(result.changedControls.ok).toBe(false)
  expect(result.badBand.ok).toBe(false)
  expect(result.estimate.interval).toEqual({
    kind:'credible',level:0.95,summary:'ETI',
    lower:evidence.cumulativeSummary.absolute.lower,upper:evidence.cumulativeSummary.absolute.upper,
  })
})
