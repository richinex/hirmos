import {test,expect,type Page} from '@playwright/test'
import {readFileSync} from 'node:fs'
const fixture=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/did_sensitivity.json','utf8'))
const cases=fixture.cases.slice(0,2)
const close=(got:number,want:number)=>expect(Math.abs(got-want)).toBeLessThanOrEqual(1e-9*Math.max(1,Math.abs(want)))

test('compiled worker matches the pinned DiD sensitivity oracle, preserving source ATT',async({page})=>{
  test.setTimeout(120000)
  await page.goto('/app')
  const outputs=await page.evaluate(async cases=>{
    const load=(p:string):Promise<any>=>import(/* @vite-ignore */ p)
    const analysis=await load('/src/analysis/client.ts')
    const output=[]
    for(const c of cases){
      const n=c.y.length,rows=n*2
      const units=Array.from({length:rows},(_,i)=>String(Math.floor(i/2)).padStart(5,'0'))
      const times=Array.from({length:rows},(_,i)=>i%2)
      const values=new Float64Array([
        ...c.y.flatMap((v:number)=>[0,v]),...c.d.flatMap((v:number)=>[0,v]),
        ...c.x.flatMap((r:number[])=>[r[0],r[0]]),...c.x.flatMap((r:number[])=>[r[1],r[1]]),
      ])
      const specification={kind:'doublyRobust',folds:2,seed:7,trimming:0.01,normalization:c.normalized?'in-sample':'population'}
      const source=await analysis.runAdjustedDid(values.slice(),rows,4,units,times,specification)
      if(!source.ok)throw Error(JSON.stringify(source.error))
      const results=[]
      for(const reference of c.results){
        const s=reference.scenario
        const request={propensityFit:'standardized-logistic-v1',folds:2,seed:7,trimming:0.01,normalization:specification.normalization,
          scenarios:[[s.cf_y,s.cf_d]],rho:s.rho,level:s.level,null:s.null_hypothesis,
          outcomeShares:[0,0.02,0.1],rieszShares:[0,0.02,0.1]}
        const fitted=await analysis.runDidSensitivity(values.slice(),rows,4,units,times,request)
        if(!fitted.ok)throw Error(JSON.stringify(fitted.error))
        results.push(fitted.value)
      }
      output.push({source:source.value,results})
    }
    return output
  },cases)
  for(let i=0;i<cases.length;i++){
    const c=cases[i],o=outputs[i]
    for(let j=0;j<c.results.length;j++){
      const e=o.results[j],r=c.results[j]
      expect(e.estimate).toBe(o.source.estimate)
      close(e.estimate,c.coef);close(e.standardError,c.se)
      close(e.sigma2,c.elements.sigma2[0]);close(e.nu2,c.elements.nu2[0])
      for(let k=0;k<2;k++){close(e.scenarios[0].effect[k],r.effect[k]);close(e.scenarios[0].interval[k],r.interval[k])}
      close(e.robustnessValue,r.rv);close(e.robustnessValueCi,r.rva)
      expect(e.grid).toHaveLength(3);expect(e.grid[0]).toHaveLength(3)
      expect(e.grid[0][0]).toEqual(e.baseline)
    }
  }
})

async function mount(page:Page,restore=false,historical=false){
  await page.evaluate(async({c,restore,historical})=>{
    const load=(p:string):Promise<any>=>import(/* @vite-ignore */ p)
    const [{React,createRoot},{JobsProvider,DidSensitivityPanel},data,analysis,workflow,domain]=await Promise.all([
      load('/tests/support/reactRuntime.ts'),load('/tests/support/didSensitivityRuntime.ts'),
      load('/src/data/client.ts'),load('/src/analysis/client.ts'),load('/src/domain/workflow.ts'),load('/src/domain/didSensitivity.ts'),
    ])
    const records=c.y.flatMap((v:number,i:number)=>[0,1].map(t=>[String(i).padStart(5,'0'),t,t===0?0:v,t===0?0:c.d[i],...c.x[i]].join(',')))
    const file=new File([['unit,period,outcome,treatment,x1,x2',...records].join('\n')],'paired-sensitivity.csv',{type:'text/csv'})
    const profiled=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
    if(!profiled.ok)throw Error(JSON.stringify(profiled.error))
    const profile=profiled.value,col=(name:string)=>profile.columns.find((v:any)=>v.name===name).id
    const matrix=await data.materializePanelInWorker(file,profile,{unit:col('unit'),time:col('period'),outcome:col('outcome'),treatment:col('treatment'),covariates:[col('x1'),col('x2')]})
    if(!matrix.ok)throw Error(JSON.stringify(matrix.error))
    const m=matrix.value,specification={kind:'doublyRobust',folds:2,seed:7,trimming:0.01,normalization:'in-sample'}
    const fit=await analysis.runAdjustedDid(m.values,m.rowCount,4,m.units,m.periodCodes,specification)
    if(!fit.ok)throw Error(JSON.stringify(fit.error))
    if(historical)delete fit.value.inference.propensityFit
    const prepared={id:'11111111-1111-4111-8111-111111111111',kind:'prepared-panel',sampling:{kind:'regular-panel',unitColumn:col('unit'),timeColumn:col('period')}}
    const sourceRun={kind:'panel-intervention-run',id:'22222222-2222-4222-8222-222222222222',createdAt:'2026-10-04T12:00:00Z',preparedDataset:prepared.id,
      configuration:{kind:'panel-intervention',primary:'adjusted',covariates:[col('x1'),col('x2')],specification},evidence:fit.value,
      columns:['outcome','treatment','x1','x2'].map(name=>({column:col(name),name,node:name})),
    }
    const host=document.createElement('div');host.style.cssText='position:fixed;inset:0;z-index:9999;background:var(--color-bone);overflow:auto;display:flex;flex-direction:column'
    document.body.append(host)
    function Example(){
      const [runs,setRuns]=React.useState(()=>restore?[domain.didSensitivityRunSchema.parse(JSON.parse(localStorage.getItem('did-sensitivity-test-run')!))]:[])
      return React.createElement(JobsProvider,{prepared:prepared.id},React.createElement(DidSensitivityPanel,{
        source:{file},profile,prepared,estimates:[sourceRun],runs,selector:React.createElement('p',null,'DiD omitted variables'),
        onRun:(run:any)=>{
          if(!domain.didSensitivityRunMatches(run,[sourceRun]))throw Error('Source mismatch')
          localStorage.setItem('did-sensitivity-test-run',JSON.stringify(run))
          localStorage.setItem('did-sensitivity-test-validation',JSON.stringify({
            original:domain.didSensitivityRunMatches(run,[sourceRun]),
            alteredSeed:domain.didSensitivityRunMatches({...run,evidence:{...run.evidence,request:{...run.evidence.request,seed:8}}},[sourceRun]),
            alteredEstimate:domain.didSensitivityRunMatches({...run,evidence:{...run.evidence,estimate:run.evidence.estimate+1}},[sourceRun]),
            invalidGrid:domain.didSensitivityRunSchema.safeParse({...run,evidence:{...run.evidence,grid:[]}}).success,
          }))
          setRuns((old:any[])=>[...old,run])
        },onDelete:(id:string)=>setRuns((old:any[])=>old.filter(r=>r.id!==id)),
      }))
    }
    createRoot(host).render(React.createElement(Example))
  },{c:cases[0],restore,historical})
}

test('historical DiD fits require an explicit refit, without a numerical fallback',async({page})=>{
  await page.goto('/app')
  await mount(page,false,true)
  await expect(page.getByText('Refit this DiD analysis before computing sensitivity.',{exact:false})).toBeVisible()
  await expect(page.getByRole('button',{name:'Compute DiD sensitivity'})).toBeDisabled()
})

test('DiD sensitivity controls, bounds, contour and saved run work in light and dark themes',async({page},info)=>{
  test.setTimeout(120000)
  const errors:string[]=[]
  page.on('pageerror',e=>errors.push(e.message))
  await page.goto('/app')
  await mount(page)
  await expect(page.getByRole('button',{name:'Compute DiD sensitivity'})).toBeEnabled()
  await page.getByLabel('Outcome shares (%)',{exact:true}).fill('2, 5')
  await expect(page.getByRole('button',{name:'Compute DiD sensitivity'})).toBeDisabled()
  await page.getByLabel('Riesz shares (%)',{exact:true}).fill('1, 10')
  await page.getByRole('button',{name:'Compute DiD sensitivity'}).click()
  const result=page.getByRole('article',{name:'DiD omitted-variable sensitivity result'}).first()
  await expect(result).toBeVisible({timeout:60000})
  await expect(result).toContainText('90% central interval')
  await expect(result).toContainText('2%')
  await expect(result).toContainText('10%')
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('did-bounds-light.png')})
  await page.getByRole('radio',{name:'Contour',exact:true}).first().click()
  await page.getByRole('radio',{name:'Confidence bound',exact:true}).first().click()
  await expect(result.getByRole('img',{name:'DiD confounding-share contours',exact:true})).toBeVisible()
  await page.screenshot({path:info.outputPath('did-contour-light.png')})
  await page.evaluate(()=>{document.documentElement.dataset.theme='dark'})
  await page.screenshot({path:info.outputPath('did-contour-dark.png')})
  const validation=await page.evaluate(()=>JSON.parse(localStorage.getItem('did-sensitivity-test-validation')!))
  expect(validation).toEqual({original:true,alteredSeed:false,alteredEstimate:false,invalidGrid:false})
  await page.reload()
  await mount(page,true)
  await expect(page.getByRole('article',{name:'DiD omitted-variable sensitivity result'}).first()).toBeVisible()
  await expect(page.getByLabel('Outcome shares (%)',{exact:true})).toHaveValue('2, 5')
  await expect(page.getByLabel('Riesz shares (%)',{exact:true})).toHaveValue('1, 10')
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth+1)).toBe(true)
  expect(errors).toEqual([])
})
