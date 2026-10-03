import { expect, test } from '@playwright/test'

const csv = 'sample,treatment,outcome,surrogate,baseline\n1,0,,0,1\n0,,1,0,2\n1,1,,1,3\n0,,3,1,4\n1,0,,2,2\n0,,5,2,1\n1,1,,3,4\n0,,7,3,3\n0,,9,4,5\n2,,,,\n'

test('two samples retain source alignment and permit absent responses outside their roles', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async csv => {
    const [data, workflow, samples, analysis] = await Promise.all([
      import(new URL('/src/data/client.ts', location.href).href),
      import(new URL('/src/domain/workflow.ts', location.href).href),
      import(new URL('/src/domain/surrogateSamples.ts', location.href).href),
      import(new URL('/src/analysis/client.ts', location.href).href),
    ])
    const file = new File([csv], 'two-samples.csv', { type: 'text/csv' })
    const p = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!p.ok) throw new Error(JSON.stringify(p.error))
    const id = (name: string) => p.value.columns.find((c: { name: string }) => c.name === name).id
    const selection = samples.surrogateSelectionSchema.parse({ sample: { column: id('sample'), experimental: 1, observational: 0 },
      treatment: id('treatment'), outcome: id('outcome'), surrogates: [id('surrogate')], adjustment: { kind: 'none' }, estimator: 'index', uncertainty: { kind: 'none' } })
    const materialized = await data.materializeNumericColumnsInWorker(file, p.value, samples.surrogateSelectionColumns(selection))
    if (!materialized.ok) throw new Error(JSON.stringify(materialized.error))
    const selected = samples.selectSurrogateSamples(materialized.value, selection, p.value)
    if (!selected.ok) throw new Error(JSON.stringify(selected.error))
    const validation = samples.selectSurrogateSamples(materialized.value, {...selection,checks:{validation:'observedOutcome',biasBounds:{kind:'none'}}},p.value)
    const missingPath=samples.selectSurrogateSamples(materialized.value,{...selection,horizons:{observed:{kind:'periods',periods:[{column:id('outcome'),label:'Observed outcome'}]},windows:{kind:'none'}}},p.value)
    const fitted = await analysis.runSurrogate(selected.value.request)
    const adjusted = { ...selection, adjustment: { kind: 'baseline', columns: [id('baseline')] } }
    const all = await data.materializeNumericColumnsInWorker(file, p.value, samples.surrogateSelectionColumns(adjusted))
    if (!all.ok) throw new Error(JSON.stringify(all.error))
    const baseline = samples.selectSurrogateSamples(all.value, adjusted, p.value)
    // A required predictor is missing: refuse, rather than silently deleting its row.
    const columnIndex = materialized.value.columns.findIndex((c: { id: string }) => c.id === id('surrogate'))
    const index = columnIndex * materialized.value.rowCount
    const validity = materialized.value.validity.slice()
    validity[index >> 3] &= ~(1 << (index & 7))
    const missing = samples.selectSurrogateSamples({ ...materialized.value, validity }, selection, p.value)
    const membershipValidity = materialized.value.validity.slice()
    membershipValidity[0] &= ~1
    const unknownSample = samples.selectSurrogateSamples({ ...materialized.value, validity: membershipValidity }, selection, p.value)
    const stale = samples.selectSurrogateSamples(materialized.value, selection, { ...p.value, rowCount: 11 })
    return { missingPath, validation, selected, fitted, baseline, missing, unknownSample, stale,
      sameSamples: samples.surrogateSelectionSchema.safeParse({ ...selection, sample: { ...selection.sample, observational: 1 } }).success,
      duplicate: samples.surrogateSelectionSchema.safeParse({ ...selection, surrogates: [id('treatment')] }).success }
  }, csv)
  expect(result.selected.value.rows).toEqual({ experimental: [0, 2, 4, 6], observational: [1, 3, 5, 7, 8], excluded: [9] })
  expect(result.selected.value.request.experimental.treatment).toEqual([0, 1, 0, 1])
  expect(result.selected.value.request.observational.outcome).toEqual([1, 3, 5, 7, 9])
  expect(result.validation.error).toMatchObject({kind:'missing-value',row:0,name:'outcome',sample:'experimental'})
  expect(result.missingPath.error).toMatchObject({kind:'missing-value',row:0,name:'outcome',sample:'experimental'})
  expect(result.fitted.ok, JSON.stringify(result.fitted)).toBe(true)
  expect(result.fitted.value.estimate).toBeCloseTo(2, 10)
  expect(result.baseline.value.request.adjustment).toEqual({ kind: 'baseline', experimental: [[1], [3], [2], [4]], observational: [[2], [4], [1], [3], [5]] })
  expect(result.missing.error).toMatchObject({ kind: 'missing-value', row: 0, name: 'surrogate', sample: 'experimental' })
  expect(result.unknownSample.error).toMatchObject({ kind: 'missing-value', row: 0, sample: 'membership' })
  expect(result.stale.error.kind).toBe('source-mismatch')
  expect(result.sameSamples).toBe(false)
  expect(result.duplicate).toBe(false)
})
