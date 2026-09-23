import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {chapter,choose} from './examples/support'

const fixtures=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/panel_glm.json','utf8')).cases
const names=['nb2_distributed_lags','binomial_team_week','nb2_cohort_events','nb2_cohort_finite_window','nb2_cohort_summary','nb2_its_standard_tolerance']
function source(name:string) {
  const c=fixtures.find((c:{name:string})=>c.name===name)
  const interrupted=name==='nb2_its_standard_tolerance',binomial=name==='binomial_team_week'
  const keys:number[][]=interrupted?c.y.map((_:number,i:number)=>[0,i]):binomial?Array.from({length:216},(_,i)=>[Math.floor(i/18),i%18]):c.design.keys
  const raw:number[][]=interrupted?c.x.map((r:number[])=>[r[4]]):binomial?c.x.map((r:number[])=>[r[1],r[2]]):c.design.values
  let retained=0
  const fixtureRows:number[]=[]
  const rows=keys.map(([unit,period],i)=>{
    const keep=name==='nb2_distributed_lags'?period!>=4:name.startsWith('nb2_cohort')?unit!<8:true
    fixtureRows.push(keep?retained:-1)
    const y=keep?c.y[retained]:0,exposure=keep?c.exposure[retained++]:1
    const onset=c.design?.adoption?.[String(unit)]??null
    return [unit,period,y,exposure,...raw[i]!,onset!==null&&period!>=onset?1:0]
  })
  const headers=['unit','period','outcome','denominator',...raw[0]!.map((_,i)=>`x${i}`),'adopted']
  const columns=raw[0]!.length+2,values=Array.from({length:columns},(_,j)=>rows.map(r=>r[j+2]!)).flat()
  let design:object
  if(interrupted)design={kind:'interrupted',intervention:20,horizon:7,bandwidth:3,covariates:[{column:2,lag:0,name:'holiday'}]}
  else if(binomial)design={kind:'lags',keys,terms:[{column:2,lag:0,name:'x1'},{column:3,lag:0,name:'x2'}],sum:[0,1]}
  else if(name==='nb2_distributed_lags')design={kind:'lags',keys,terms:c.design.columns.map(([column,lag,label]:[number,number,string])=>({column:column+2,lag,name:label})),sum:[0,1,2,3]}
  else {
    const common={keys,adoption:Object.entries(c.design.adoption).map(([u,t])=>[Number(u),t]),cohort:c.design.cohort,covariates:[]}
    design=name==='nb2_cohort_summary'?{kind:'summary',...common}:{kind:'events',...common,window:name==='nb2_cohort_finite_window'?{kind:'finite',first:c.design.first,last:c.design.last}:{kind:'all'}}
  }
  return {c,values,fixtureRows,csv:[headers.join(','),...rows.map(r=>r.join(','))].join('\n'),headers,request:{rows:rows.length,columns,outcome:0,family:binomial?{kind:'binomial',trials:1}:{kind:'negativeBinomial',exposure:1},design,confidence:0.95,iterations:1000,tolerance:c.tolerance}}
}

test('count models preserve pinned oracle fits through the WASM worker and reject corrupted evidence',async({page})=>{
  test.setTimeout(240_000)
  await page.goto('/app')
  for(const name of names){
    const s=source(name)
    const result=await page.evaluate(async({values,request})=>{
      const client=await import(new URL('/src/analysis/client.ts',location.href).href)
      const schemas=await import(new URL('/src/domain/countRegression.ts',location.href).href)
      const parsed=schemas.countRegressionRequestSchema.parse(request)
      const response=await client.runCountRegression(new Float64Array(values),parsed)
      if(!response.ok)throw Error(JSON.stringify(response.error))
      const e=response.value
      return {e,roundtrip:schemas.countRegressionEvidenceSchema.safeParse(JSON.parse(JSON.stringify(e))).success,badRows:schemas.countRegressionEvidenceSchema.safeParse({...e,retained:[0,...e.retained]}).success,badEvents:schemas.countRegressionEvidenceSchema.safeParse({...e,eventPeriods:[[999,0]]}).success}
    },{values:s.values,request:s.request})
    expect(result.roundtrip,name).toBe(true)
    expect(result.badRows,name).toBe(false)
    expect(result.badEvents,name).toBe(false)
    expect(result.e.observations,name).toBe(s.c.y.length)
    const relative=(a:number,b:number)=>Math.abs(a-b)/(1+Math.abs(b))
    for(let i=0;i<s.c.fitted.length;i++)expect(relative(result.e.fitted[i],s.c.fitted[i]),`${name} fitted ${i}`).toBeLessThan(1e-4)
    if(name!=='nb2_its_standard_tolerance')for(let i=0;i<result.e.terms.length;i++){
      expect(relative(result.e.terms[i].estimate,s.c.params[i]),`${name} coefficient ${i}`).toBeLessThan(2e-5)
      expect(relative(result.e.terms[i].standardError,Math.sqrt(s.c.covariances.cluster[i][i])),`${name} SE ${i}`).toBeLessThan(2e-5)
    }
  }
})

for(const name of names)test(`${name} completes using the UI and survives project reopening`,async({page},info)=>{
  test.setTimeout(240_000)
  const s=source(name),interrupted=name==='nb2_its_standard_tolerance'
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill(`Count regression ${name}`)
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'count-regression.csv',mimeType:'text/csv',buffer:Buffer.from(s.csv)})
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:interrupted?/Regular time series/:/^Panel/}).click()
  if(!interrupted)await choose(page,'Unit column','unit')
  await choose(page,'Time column','period')
  for(const column of s.headers.filter(c=>!['unit','period'].includes(c)))await page.getByRole('checkbox',{name:column,exact:true}).first().check()
  await page.getByRole('button',{name:/Create prepared/}).click()
  await expect(page.getByRole('heading',{name:'Build a DAG or run discovery',exact:true})).toBeVisible({timeout:60_000})
  await chapter(page,/Time-series analysis/)
  if(interrupted)await page.getByRole('radio',{name:'Count regression',exact:true}).click()
  if(name==='binomial_team_week')await page.getByRole('radio',{name:'Grouped binomial',exact:true}).click()
  if(name==='nb2_cohort_events'||name==='nb2_cohort_finite_window')await page.getByRole('radio',{name:'Cohort event study',exact:true}).click()
  if(name==='nb2_cohort_summary')await page.getByRole('radio',{name:'Cohort summary',exact:true}).click()
  await choose(page,'Outcome','outcome')
  await choose(page,name==='binomial_team_week'?'Trials':'Exposure','denominator')
  if(interrupted){
    await page.getByRole('textbox',{name:'Intervention row (from 1)',exact:true}).fill('21')
    await page.getByRole('textbox',{name:'Contrast periods after intervention',exact:true}).fill('7')
    await page.getByRole('checkbox',{name:'x0',exact:true}).check()
  }else if(name.startsWith('nb2_cohort')){
    await choose(page,'Onset indicator','adopted')
    await page.getByRole('textbox',{name:'Cohort onset period (from 1)',exact:true}).fill('8')
    if(name==='nb2_cohort_finite_window'){
      await page.getByRole('radio',{name:'Specified window',exact:true}).click()
      await page.getByRole('textbox',{name:'First event period',exact:true}).fill(String(s.c.design.first))
      await page.getByRole('textbox',{name:'Last event period',exact:true}).fill(String(s.c.design.last))
    }
  }else{
    await choose(page,'Predictor','x0')
    if(name==='binomial_team_week')await page.getByRole('textbox',{name:'Predictor lags',exact:true}).fill('0')
    await page.getByRole('checkbox',{name:'x1',exact:true}).check()
  }
  await page.getByRole('textbox',{name:'Optimizer tolerance',exact:true}).fill(String(s.c.tolerance))
  await page.getByRole('button',{name:'Fit regression',exact:true}).click()
  const result=page.getByRole('region',{name:'Count regression result',exact:true})
  await expect(result).toBeVisible({timeout:120_000})
  await expect(result.getByRole('heading',{name:'Coefficients',exact:true})).toBeVisible()
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath(`${name}-result.png`)})
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1)).toBe(true)
  const navigation=page.getByRole('button',{name:'Expand chapter list',exact:true})
  if(await navigation.isVisible())await navigation.click()
  const download=page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path=await(await download).path()
  if(!path)throw Error('No exported project')
  const snapshot=JSON.parse(readFileSync(path,'utf8')).project
  const saved=snapshot.timeSeriesRuns.at(-1)
  expect(saved.kind).toBe('count-regression')
  for(let i=0;i<s.c.fitted.length;i++){
    const expected=s.c.fitted[s.fixtureRows[saved.evidence.retained[i]]!]
    expect(Math.abs(saved.evidence.fitted[i]-expected)/(1+Math.abs(expected)),`${name} UI fitted ${i}`).toBeLessThan(1e-4)
  }
  const contract=await page.evaluate(async snapshot=>{
    const persistence=await import(new URL('/src/domain/persistence.ts',location.href).href)
    const valid=persistence.parseSnapshotValue(snapshot).ok
    const invalid=structuredClone(snapshot)
    invalid.timeSeriesRuns.at(-1).specification.confidence=0.8
    return {valid,corrupted:persistence.parseSnapshotValue(invalid).ok}
  },snapshot)
  expect(contract).toEqual({valid:true,corrupted:false})
  await expect.poll(()=>page.evaluate(async id=>{
    const store=await import(new URL('/src/data/projectStore.ts',location.href).href)
    const loaded=await store.loadProject(id)
    return loaded.ok?loaded.value.timeSeriesRuns.length:0
  },snapshot.project.id)).toBe(1)
  await page.reload()
  await page.getByRole('button',{name:`Open Count regression ${name}`,exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'count-regression.csv',mimeType:'text/csv',buffer:Buffer.from(s.csv)})
  await chapter(page,/Time-series analysis/)
  if(interrupted)await page.getByRole('radio',{name:'Count regression',exact:true}).click()
  await expect(page.getByRole('region',{name:'Count regression result',exact:true})).toBeVisible({timeout:30_000})
})
