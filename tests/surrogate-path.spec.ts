import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {z} from 'zod'
import {surrogatePathRequestSchema,surrogatePathEvidenceSchema,surrogatePathMatches} from '../src/domain/surrogatePath'

test('path contracts reject incomplete observations and ambiguous window order',()=>{
  const observed={kind:'observedOutcomes',uncertainty:{kind:'none'},outcomes:[[1,2],[2,4],[2,5],[4,6]],treatment:[0,1,0,1],labels:['First','Second']}
  expect(surrogatePathRequestSchema.safeParse(observed).success).toBe(true)
  for(const extra of [{labels:[]},{labels:['First','First']},{labels:['First','']},{outcomes:[[1],[2,4],[2,5],[4,6]]},{treatment:[0,0,0,0]},{treatment:[0,1]},{uncertainty:{kind:'bootstrap'}}])
    expect(surrogatePathRequestSchema.safeParse({...observed,...extra}).success).toBe(false)
  const model={experimental:{surrogates:[[1,2],[2,4],[2,5],[4,6]],treatment:[0,1,0,1]},observational:{surrogates:[[1,2],[2,4],[2,5],[4,6]],outcome:[2,3,5,6]},adjustment:{kind:'none'},estimator:'index',uncertainty:{kind:'none'}}
  const windows=[{label:'First',surrogateColumns:1},{label:'Second',surrogateColumns:2}]
  expect(surrogatePathRequestSchema.safeParse({kind:'surrogateWindows',model,windows}).success).toBe(true)
  for(const invalid of [[],windows.slice(0,1),[...windows].reverse(),[windows[0],windows[0]],[{label:'A',surrogateColumns:0},windows[1]]])
    expect(surrogatePathRequestSchema.safeParse({kind:'surrogateWindows',model,windows:invalid}).success).toBe(false)
  expect(surrogatePathEvidenceSchema.safeParse({kind:'surrogateWindows',windows:[]}).success).toBe(false)
})

const fixtureSchema=z.object({experimental:z.array(z.array(z.number())),observational:z.array(z.array(z.number())),treatment:z.array(z.number()),outcome:z.array(z.number()),
  path:z.object({quarter_contrasts:z.array(z.number()),cumulative_contrasts:z.array(z.number()),treated_means:z.array(z.number()),control_means:z.array(z.number()),participant_benchmark:z.array(z.number())}),
  cases:z.array(z.object({quarters:z.number(),effect:z.number()}))})

test('public simulation horizon paths agree with R through the browser worker',async({page})=>{
  test.setTimeout(120_000)
  const f=fixtureSchema.parse(JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/surrogate-index/public-fixtures.json',import.meta.url),'utf8')))
  await page.goto('/app')
  const requests=[
    surrogatePathRequestSchema.parse({kind:'observedOutcomes',uncertainty:{kind:'none'},outcomes:f.experimental,treatment:f.treatment,labels:f.cases.map(c=>`Quarter ${c.quarters}`)}),
    surrogatePathRequestSchema.parse({kind:'surrogateWindows',model:{experimental:{surrogates:f.experimental,treatment:f.treatment},observational:{surrogates:f.observational,outcome:f.outcome},adjustment:{kind:'none'},estimator:'index',uncertainty:{kind:'none'}},windows:f.cases.map(c=>({label:`Quarter ${c.quarters}`,surrogateColumns:c.quarters}))}),
  ]
  const close=(actual:number,expected:number)=>expect(Math.abs(actual-expected)).toBeLessThanOrEqual(1e-9*(1+Math.abs(expected)))
  for(const request of requests){
    const result=await page.evaluate(async input=>{
      const client=await import(new URL('/src/analysis/client.ts',location.href).href)
      return client.runSurrogatePath(input)
    },request)
    expect(result.ok,JSON.stringify(result)).toBe(true)
    const e=surrogatePathEvidenceSchema.parse(result.value)
    expect(surrogatePathMatches(e,request)).toBe(true)
    if(e.kind==='observedOutcomes'){
      e.periods.forEach((p,i)=>{close(p.contrast,f.path.quarter_contrasts[i]!);close(p.cumulativeMeanContrast,f.path.cumulative_contrasts[i]!);close(p.treatedMean,f.path.treated_means[i]!);close(p.controlMean,f.path.control_means[i]!)})
      close(e.participantBenchmark.estimate,f.path.participant_benchmark[0]!)
      close(e.participantBenchmark.standardError,f.path.participant_benchmark[1]!)
      if(request.kind==='observedOutcomes') expect(surrogatePathMatches(e,{...request,labels:request.labels.map((s,i)=>i===0?'Wrong period':s)})).toBe(false)
    } else {
      e.windows.forEach((w,i)=>close(w.evidence.estimate,f.cases[i]!.effect))
      if(request.kind==='surrogateWindows') expect(surrogatePathMatches(e,{...request,model:{...request.model,estimator:'score'}})).toBe(false)
    }
  }
})
