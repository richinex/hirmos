import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('adjusted DiD worker preserves keyed alignment and pinned regression inference', async ({ page }) => {
  await page.goto('/app')
  const csv = readFileSync('tests/fixtures/chapter-did-covariates.csv','utf8')
  const result = await page.evaluate(async csv => {
    const data = await import(new URL('/src/data/client.ts',location.href).href)
    const workflow = await import(new URL('/src/domain/workflow.ts',location.href).href)
    const analysis = await import(new URL('/src/analysis/client.ts',location.href).href)
    const run = async (text: string, specification: object) => {
      const file = new File([text],'paired.csv',{ type:'text/csv' })
      const profile = await data.profileSourceInWorker(workflow.newImportRequestId(),file)
      if (!profile.ok) throw Error(JSON.stringify(profile.error))
      const column = (name: string) => profile.value.columns.find((c: { name: string }) => c.name === name).id
      const matrix = await data.materializePanelInWorker(file,profile.value,{ unit:column('id'),time:column('time_points'),outcome:column('Y'),treatment:column('treatment'),covariates:[column('A')] })
      if (!matrix.ok) throw Error(JSON.stringify(matrix.error))
      const m = matrix.value
      const fit = await analysis.runAdjustedDid(m.values,m.rowCount,3,m.units,m.periodCodes,specification)
      if (!fit.ok) throw Error(JSON.stringify(fit.error))
      return fit.value
    }
    const [header,...rows] = csv.trim().split('\n')
    const regression = await run(csv,{ kind:'regression' })
    const reversed = await run([header,...rows.reverse()].join('\n'),{ kind:'regression' })
    const dr = await run(csv,{ kind:'doublyRobust',folds:5,seed:1234,trimming:0.01,normalization:'in-sample' })
    return { regression,reversed,dr }
  },csv)
  expect(result.regression.estimate).toBeCloseTo(290.53750263095316,8)
  expect(result.regression.standardError).toBeCloseTo(59.77346090819873,8)
  expect(result.regression.interval[0]).toBeCloseTo(169.190925742257,8)
  expect(result.regression.interval[1]).toBeCloseTo(411.8840795196493,8)
  expect(result.regression.inference.degreesOfFreedom).toBe(35)
  expect(result.reversed).toEqual(result.regression)
  expect(result.dr.specification.kind).toBe('doublyRobust')
  expect(result.dr.inference.optimizerStatus).toHaveLength(5)
  expect(result.dr.estimate).toBeCloseTo(314.7960332262636,5)
})

test('sharp RD worker matches rdrobust and refuses non-sharp assignment', async ({ page }) => {
  await page.goto('/app')
  const csv = readFileSync('tests/fixtures/chapter-rd.csv','utf8')
  const result = await page.evaluate(async csv => {
    const analysis = await import(new URL('/src/analysis/client.ts',location.href).href)
    const rows = csv.trim().split('\n').slice(1).map(row => row.split(',').map(Number))
    const values = () => new Float64Array([0,2,1].flatMap(j => rows.map(row => row[j]!)))
    const valid = await analysis.runSharpRd(values(),rows.length,0)
    const bad = values(); bad[2*rows.length] = 1
    const invalid = await analysis.runSharpRd(bad,rows.length,0)
    return { valid,invalid }
  },csv)
  expect(result.valid.ok,JSON.stringify(result.valid)).toBe(true)
  expect(result.valid.value.conventional.value).toBeCloseTo(952.3046619369882,7)
  expect(result.valid.value.robust.value).toBeCloseTo(1040.9315095351085,7)
  expect(result.valid.value.robust.standardError).toBeCloseTo(400.6142202316428,7)
  expect(result.invalid.ok).toBe(false)
})
