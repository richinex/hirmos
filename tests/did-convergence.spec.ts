import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'

const fixture=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/did_convergence.json','utf8'))

test('DR DiD worker is stable under covariate permutations and matches DoubleML',async({page})=>{
  test.setTimeout(120000)
  await page.goto('/app')
  const result=await page.evaluate(async cases=>{
    const load=(p:string):Promise<any>=>import(/* @vite-ignore */ p)
    const {runAdjustedDid,runDidSensitivity}=await load('/src/analysis/client.ts')
    const {adjustedDidEvidenceSchema}=await load('/src/domain/adjustedDid.ts')
    const {didSensitivityRequestSchema}=await load('/src/domain/didSensitivity.ts')
    const output=[]
    for(const c of cases){
      const n=c.y.length,p=c.x[0].length,rows=2*n
      const units=Array.from({length:rows},(_,i)=>String(Math.floor(i/2)).padStart(6,'0'))
      const times=Array.from({length:rows},(_,i)=>i%2)
      const base=Array.from({length:p},(_,i)=>i),swapped=[...base]
      ;[swapped[p-2],swapped[p-1]]=[swapped[p-1],swapped[p-2]]
      const fits=[]
      for(const order of [base,[...base].reverse(),swapped]){
        const values=new Float64Array([...c.y.flatMap((v:number)=>[0,v]),...c.d.flatMap((v:number)=>[0,v]),...order.flatMap(j=>c.x.flatMap((r:number[])=>[r[j],r[j]]))])
        const spec={kind:'doublyRobust',folds:c.folds,seed:c.seed,trimming:0.01,normalization:c.normalized?'in-sample':'population'}
        const fit=await runAdjustedDid(values.slice(),rows,p+2,units,times,spec)
        if(!fit.ok)throw Error(JSON.stringify(fit.error))
        const request={propensityFit:'standardized-logistic-v1',folds:c.folds,seed:c.seed,trimming:0.01,normalization:spec.normalization,scenarios:[[0.02,0.05]],rho:1,level:0.95,null:0,outcomeShares:[0,0.02],rieszShares:[0,0.05]}
        const sensitivity=await runDidSensitivity(values.slice(),rows,p+2,units,times,request)
        if(!sensitivity.ok)throw Error(JSON.stringify(sensitivity.error))
        const bad={...fit.value,inference:{...fit.value.inference,optimizerStatus:Array(c.folds).fill('iteration-limit')}}
        fits.push({estimate:fit.value.estimate,se:fit.value.standardError,inference:fit.value.inference,sensitivity:sensitivity.value,rejectsUnfinished:!adjustedDidEvidenceSchema.safeParse(bad).success,rejectsOldRequest:!didSensitivityRequestSchema.safeParse({...request,propensityFit:undefined}).success})
      }
      output.push(fits)
    }
    return output
  },fixture.cases)
  const close=(got:number,want:number)=>expect(Math.abs(got-want)).toBeLessThanOrEqual(1e-6+1e-8*Math.abs(want))
  for(let i=0;i<fixture.cases.length;i++)for(const fit of result[i]){
    const c=fixture.cases[i]
    close(fit.estimate,c.coef);close(fit.se,c.se);close(fit.estimate,result[i][0].estimate)
    expect(fit.inference.propensityFit).toBe('standardized-logistic-v1')
    expect(fit.inference.optimizerStatus.every((s:string)=>s==='projected-gradient'||s==='function-tolerance')).toBe(true)
    expect(fit.rejectsUnfinished).toBe(true);expect(fit.rejectsOldRequest).toBe(true)
    expect(fit.sensitivity.estimate).toBe(fit.estimate)
    for(let k=0;k<2;k++){close(fit.sensitivity.scenarios[0].effect[k],c.effect[k]);close(fit.sensitivity.scenarios[0].interval[k],c.interval[k])}
  }
})
