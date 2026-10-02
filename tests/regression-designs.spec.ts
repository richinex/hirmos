import {test,expect} from '@playwright/test'

test('regression specification catalogue is complete, disjoint and rejects unsupported pairs',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const d=await import(new URL('/src/domain/regressionDesigns.ts',location.href).href)
    const definitions=d.regressionDesigns
    const valid=[]
    const invalid=[]
    for(const analysis of d.regressionAnalyses)for(const model of [null,'linear','count']){
      const selected=d.selectRegressionDesign(analysis.value,model)
      if(selected.ok)valid.push({analysis:analysis.value,model,kind:selected.value.kind,roundtrip:d.regressionDesignInfo(selected.value).kind})
      else invalid.push({analysis:analysis.value,model,reason:selected.error})
    }
    return {valid,invalid,registered:definitions.map((item:{kind:string})=>item.kind),
      malformed:d.regressionDesignSchema.safeParse({kind:'count-cohort-summary',outcomeModel:'linear'}).success}
  })
  expect(result.valid).toHaveLength(5)
  expect(new Set(result.registered).size).toBe(5)
  expect(result.valid.map(v=>v.kind).sort()).toEqual([...result.registered].sort())
  expect(result.valid.every(v=>v.kind===v.roundtrip)).toBe(true)
  expect(result.invalid.find(v=>v.analysis==='cohort-summary'&&v.model==='linear')?.reason).toContain('not implemented')
  expect(result.malformed).toBe(false)
})

test('all specification transitions are deterministic, idempotent and preserve independent drafts',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const d=await import(new URL('/src/domain/regressionDesigns.ts',location.href).href)
    const panel=await import(new URL('/src/domain/panelRegression.ts',location.href).href)
    const count=await import(new URL('/src/domain/countRegression.ts',location.href).href)
    const initial={selection:{kind:'linear-event-study'},linear:{...panel.initialPanelRegression(),outcome:'y'},cohort:{...count.initialCountRegression(true),outcome:'y',model:{kind:'events',onset:'treated',cohort:'1987',window:{kind:'all'}}}}
    const snapshot=JSON.stringify(initial)
    const failures=[]
    let checks=0
    for(const source of d.regressionDesigns){
      const state=d.transitionRegressionDesign(initial,{kind:source.kind})
      const before=JSON.stringify(state)
      for(const target of d.regressionDesigns){
        const selected={kind:target.kind}
        const once=d.transitionRegressionDesign(state,selected)
        const repeat=d.transitionRegressionDesign(once,selected)
        if(JSON.stringify(once)!==JSON.stringify(repeat))failures.push('idempotence '+source.kind+' '+target.kind)
        if(JSON.stringify(once)!==JSON.stringify(d.transitionRegressionDesign(state,selected)))failures.push('determinism')
        if(JSON.stringify(state)!==before)failures.push('mutation')
        if(once.selection.kind!==target.kind)failures.push('selection')
        const info=d.regressionDesignInfo(once.selection)
        if(info.model==='count'&&!['events','summary'].includes(once.cohort.model.kind))failures.push('count route')
        if(info.model==='linear'&&once.linear.model.kind!=='eventStudy')failures.push('linear route')
        checks++
      }
    }
    return {checks,failures,unmodified:snapshot===JSON.stringify(initial),readiness:[
      d.assessCountOutcome([0,1,2]),d.assessCountOutcome([-1,2]),d.assessCountOutcome([0.5,2]),
      d.assessCountOutcome([NaN]),d.assessCountOutcome([Infinity]),d.assessCountOutcome([]),
    ].map(value=>value.kind)}
  })
  expect(result).toEqual({checks:25,failures:[],unmodified:true,readiness:['ready','refused','refused','refused','refused','refused']})
})

test('cohort count results move together without moving distributed lags or interrupted series',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const {isRegressionDesignRun}=await import(new URL('/src/domain/timeSeries.ts',location.href).href)
    return ['events','summary','lags','interrupted'].map(kind=>isRegressionDesignRun({kind:'count-regression',specification:{design:{kind}}}))
  })
  expect(result).toEqual([true,true,false,false])
})
