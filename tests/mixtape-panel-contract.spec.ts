import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'

const cases=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/bacon-adjusted.json','utf8')).cases
test('adjusted Bacon preserves the within component and every between comparison through WASM',async({page})=>{
  await page.goto('/app')
  for(const [name,raw] of Object.entries(cases)){
    const c=raw as {data:{id:number;time:number;y:number;treated:number;x1:number;x2:number}[];twfe:number;omega:number;within:number;comparisons:{estimate:number;weight:number}[]}
    const request={rows:c.data.length,columns:4,units:c.data.map(r=>String(r.id)),times:c.data.map(r=>r.time),outcome:0,treatment:1,specification:{kind:'adjusted',controls:[2,3]}}
    const result=await page.evaluate(async({request,values})=>{
      const client=await import(new URL('/src/analysis/client.ts',location.href).href)
      const reply=await client.runBacon(new Float64Array(values),request)
      if(!reply.ok)throw Error(JSON.stringify(reply.error))
      return reply.value
    },{request,values:c.data.flatMap(r=>[r.y,r.treated,r.x1,r.x2])})
    const near=(a:number,b:number)=>expect(Math.abs(a-b),name).toBeLessThanOrEqual(1e-8*Math.max(1,Math.abs(b)))
    near(result.twfe,c.twfe);near(result.reconstructed,c.twfe);near(result.decomposition.withinEstimate,c.within);near(result.decomposition.withinWeight,c.omega)
    expect(result.decomposition.between).toHaveLength(c.comparisons.length)
    const expected=[...c.comparisons].sort((a,b)=>a.estimate-b.estimate)
    const actual=[...result.decomposition.between].sort((a,b)=>a.estimate-b.estimate)
    for(let i=0;i<expected.length;i++){near(actual[i].estimate,expected[i].estimate);near(actual[i].weight,expected[i].weight)}
  }
})

test('panel requests reject unknown options, overlapping roles and invalid event windows',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const schemas=await import(new URL('/src/domain/panelRegression.ts',location.href).href)
    const r={rows:4,columns:3,names:['y','x','cluster'],outcome:0,cluster:2,weights:null,confidence:0.95,specification:{kind:'interactions',predictors:[{column:1,name:'x',coding:{kind:'numeric'}}],terms:[[0]]}}
    const event={...r,specification:{kind:'eventStudy',keys:[[0,0],[0,1],[1,0],[1,1]],adoption:[[0,1],[1,null]],covariates:[1],window:{first:-2,last:2,reference:-2,tails:'bin'}}}
    return {
      valid:schemas.panelRegressionRequestSchema.safeParse(r).success,
      overlap:schemas.panelRegressionRequestSchema.safeParse({...r,weights:2}).success,
      unknown:schemas.panelRegressionRequestSchema.safeParse({...r,fallback:true}).success,
      duplicate:schemas.panelRegressionRequestSchema.safeParse({...r,specification:{...r.specification,terms:[[0,0]]}}).success,
      binnedReference:schemas.panelRegressionRequestSchema.safeParse(event).success,
      ignoredControls:schemas.baconRequestSchema.safeParse({rows:4,columns:2,units:['a','a','b','b'],times:[0,1,0,1],outcome:0,treatment:1,specification:{kind:'unadjusted',controls:[2]}}).success,
    }
  })
  expect(result).toEqual({valid:true,overlap:false,unknown:false,duplicate:false,binnedReference:false,ignoredControls:false})
})
