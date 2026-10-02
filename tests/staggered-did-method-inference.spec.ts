import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'

const input=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/mpdta-input.json','utf8')) as Record<string,number>[]
const reference=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/methods-inference.json','utf8'))

test('every staggered adjustment method preserves seeded pointwise and simultaneous R bands in WASM',async({page})=>{
  test.setTimeout(120_000)
  await page.goto('/app')
  const results=await page.evaluate(async input=>{
    const analysis=await import(new URL('/src/analysis/client.ts',location.href).href)
    const domain=await import(new URL('/src/domain/staggeredDid.ts',location.href).href)
    const values=new Float64Array([...input.map(r=>r.lemp),...input.map(r=>r.lpop)])
    const request={
      rows:input.length,columns:2,
      units:input.map(r=>`unit-${String(r.countyreal).padStart(8,'0')}`),
      times:input.map(r=>r.year),adoption:input.map(r=>r['first.treat']===0?null:r['first.treat']),weights:null,
    }
    const results=[]
    for(const adjustment of ['doublyRobust','outcomeRegression','inverseProbability']){
      for(const clustered of [false,true]){
        for(const kind of ['bootstrapPointwise','bootstrapSimultaneous']){
          const specification={...domain.defaultStaggeredSpecification,adjustment:{kind:adjustment},inference:{kind,iterations:999,seed:731}}
          const model=domain.staggeredRequestSchema.parse({...request,clusters:clustered?input.map(r=>`state-${String(Math.floor(r.countyreal/1000)).padStart(3,'0')}`):null,specification})
          const run=await analysis.runStaggeredDid(values.slice(),model)
          if(!run.ok)throw Error(JSON.stringify(run.error))
          results.push({adjustment,clustered,kind,evidence:run.value})
        }
      }
    }
    return results
  },input)
  for(const result of results){
    const oracle={doublyRobust:'dr',outcomeRegression:'reg',inverseProbability:'ipw'}[result.adjustment as 'doublyRobust'|'outcomeRegression'|'inverseProbability']
    const expected=reference[oracle].cases[result.clustered?1:0]
    const label=result.kind==='bootstrapSimultaneous'?'simultaneous':'pointwise'
    expect(result.evidence.events.keys).toEqual(reference[oracle].event)
    expect(result.evidence.clusterCount).toBe(new Set(expected.clusters).size)
    expect(result.evidence.events.coverage.kind).toBe(label)
    if(label==='simultaneous')expect(result.evidence.events.coverage.critical).toBeCloseTo(expected.critical,9)
    for(let i=0;i<reference[oracle].event.length;i++){
      const interval=result.evidence.events.intervals[i]
      expect(interval.kind).toBe('estimated')
      expect(interval.standardError).toBeCloseTo(expected.se[i],9)
      expect(interval.lower).toBeCloseTo(expected[`${label}_lower`][i],9)
      expect(interval.upper).toBeCloseTo(expected[`${label}_upper`][i],9)
    }
  }
})
