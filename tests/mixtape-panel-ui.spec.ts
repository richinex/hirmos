import {test,expect,type Page} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {chapter,choose} from './examples/support'

const lfe=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/mixtape_lfe.json','utf8')).castle_events
const ddd=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/mixtape_estimatr.json','utf8')).cases.abortion_ddd
const bacon=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/bacon.json','utf8')).cases.castle
const csv=(headers:string[],rows:unknown[][])=>[headers.map(h=>JSON.stringify(h)).join(','),...rows.map(r=>r.join(','))].join('\n')
const tolerance=(a:number,b:number)=>expect(Math.abs(a-b)).toBeLessThanOrEqual(1e-8*Math.max(1,Math.abs(b)))
async function exportProject(page:Page){
  const navigation=page.getByRole('button',{name:'Expand section list',exact:true})
  if(await navigation.isVisible())await navigation.click()
  const promise=page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path=await(await promise).path();if(path===null)throw Error('No export')
  return JSON.parse(readFileSync(path,'utf8')).project
}
test.use({video:'on'})
for(const mode of ['event-study','ddd','bacon'] as const)test(`Mixtape ${mode}: source example through controls, inference, plots and restoration`,async({page},info)=>{
  test.setTimeout(300_000)
  const errors:string[]=[];page.on('pageerror',e=>errors.push(e.message))
  let headers:string[],rows:unknown[][]
  if(mode==='event-study'){
    headers=['unit','period','outcome','weights','cluster','adopted',...lfe.covariates]
    const adoption=new Map<number,number|null>(lfe.adoption.map((a:{unit:number;time:number|null})=>[a.unit,a.time]))
    rows=lfe.y.map((y:number,i:number)=>{const g=adoption.get(lfe.units[i]);return [lfe.units[i],lfe.times[i],y,lfe.weights[i],lfe.cluster_ids[i],g!==null&&g!==undefined&&lfe.times[i]>=g?1:0,...lfe.covariates.map((n:string)=>lfe.x[i][lfe.terms.indexOf(n)])]})
  }else if(mode==='ddd'){
    headers=['outcome','weights','cluster',...ddd.input.map((p:{name:string})=>p.name)]
    rows=ddd.y.map((y:number,i:number)=>[y,ddd.weights[i],ddd.clusters[i],...ddd.input.map((p:{values:number[]})=>p.values[i])])
  }else{
    headers=['unit','period','outcome','adopted'];rows=bacon.data.map((r:{id:number;time:number;y:number;treated:number})=>[r.id,r.time,r.y,r.treated])
  }
  const source={name:`mixtape-${mode}.csv`,mimeType:'text/csv',buffer:Buffer.from(csv(headers,rows))}
  await info.attach('source-data',{body:source.buffer,contentType:'text/csv'})
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill(`Mixtape ${mode}`)
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(source)
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:mode==='ddd'?/Independent observations/:/^Panel/}).click()
  if(mode!=='ddd'){await choose(page,'Unit column','unit');await choose(page,'Time column','period')}
  await page.getByRole('button',{name:'Select all columns',exact:true}).click()
  await page.getByRole('button',{name:/Create prepared/}).click()
  await expect(page.getByRole('heading',{name:'Next steps',exact:true})).toBeVisible({timeout:90_000})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Regression designs',exact:true}).click()
  await page.getByRole('radio',{name:mode==='event-study'?'Event study':mode==='ddd'?'Interactions / DDD':'Bacon decomposition',exact:true}).click()
  await choose(page,'Outcome','outcome')
  if(mode!=='bacon'){
    await choose(page,'Cluster column','cluster')
    await choose(page,'Observation weights','weights')
  }
  if(mode==='event-study'){
    await choose(page,'Treatment indicator','adopted')
    for(const name of lfe.covariates)await page.getByRole('group',{name:'Covariates',exact:true}).getByRole('checkbox',{name,exact:true}).check()
    for(const [label,value] of [['First event period',lfe.window.first],['Last event period',lfe.window.last],['Reference event period',lfe.window.reference]] as const)await page.getByRole('textbox',{name:label,exact:true}).fill(String(value))
  }else if(mode==='ddd'){
    for(const p of ddd.input){
      await page.getByRole('group',{name:'Predictors',exact:true}).getByRole('checkbox',{name:p.name,exact:true}).check()
      if(p.categorical){await choose(page,`${p.name} coding`,'Categorical');await page.getByRole('textbox',{name:`${p.name} reference`,exact:true}).fill(p.reference)}
    }
    for(const t of ddd.term_inputs.filter((t:number[])=>t.length>1)){
      await choose(page,'Interaction factor 1',ddd.input[t[0]].name)
      await choose(page,'Interaction factor 2',ddd.input[t[1]].name)
      await choose(page,'Interaction factor 3',t.length===3?ddd.input[t[2]].name:'None (pair)')
      await page.getByRole('button',{name:'Add interaction',exact:true}).click()
    }
  }else await choose(page,'Treatment indicator','adopted')
  await page.getByRole('button',{name:mode==='bacon'?'Decompose coefficient':mode==='event-study'?'Fit event study':'Fit interaction regression',exact:true}).scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('configured.png')})
  await page.getByRole('button',{name:mode==='bacon'?'Decompose coefficient':mode==='event-study'?'Fit event study':'Fit interaction regression',exact:true}).click()
  const result=page.getByRole('region',{name:mode==='bacon'?'Bacon decomposition result':'Panel regression result',exact:true})
  await expect(result).toBeVisible({timeout:180_000})
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('result.png')})
  if(mode!=='ddd'){
    const chart=page.getByRole('img',{name:mode==='bacon'?'Bacon comparison weights':'Regression event study',exact:true})
    await chart.scrollIntoViewIfNeeded()
    await expect(chart).not.toHaveAttribute('aria-busy','true')
    await page.screenshot({path:info.outputPath('plot.png')})
  }
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1)).toBe(true)
  // Moving the tools must not introduce a study/DAG requirement or leave a second entry in Time series.
  await page.getByRole('radio',{name:'Effect estimation',exact:true}).click()
  await expect(page.getByText('and identify a study first.')).toBeVisible()
  await page.getByRole('radio',{name:'Regression designs',exact:true}).click()
  await expect(result).toBeVisible()
  if(mode!=='ddd'){
    await chapter(page,/Time-series analysis/)
    await expect(page.getByRole('radio',{name:'Bacon decomposition',exact:true})).toHaveCount(0)
    await expect(page.getByRole('radio',{name:'Event study',exact:true})).toHaveCount(0)
    await chapter(page,/Estimation/)
    await page.getByRole('radio',{name:'Regression designs',exact:true}).click()
    await expect(result).toBeVisible()
  }
  await chapter(page,/Results/)
  await expect(result).toBeVisible()
  await expect(page.getByText('Review regression event-time coefficients', {exact:false})).toBeVisible()
  await expect(page.getByRole('radio',{name:'Time series',exact:true})).toHaveCount(0)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Regression designs',exact:true}).click()
  await result.scrollIntoViewIfNeeded()
  await page.getByRole('button',{name:'Change theme',exact:true}).click()
  await expect(page.locator('html')).toHaveAttribute('data-theme','dark')
  await expect(page.locator('html')).not.toHaveAttribute('data-theme-transition','')
  await page.screenshot({path:info.outputPath('result-dark.png')})
  await page.getByRole('button',{name:'Change theme',exact:true}).click()
  await expect(page.locator('html')).toHaveAttribute('data-theme','light')
  const snapshot=await exportProject(page),run=snapshot.timeSeriesRuns.at(-1)
  const contract=await page.evaluate(async snapshot=>{
    const persistence=await import(new URL('/src/domain/persistence.ts',location.href).href)
    const valid=persistence.parseSnapshotValue(snapshot).ok
    const changed=structuredClone(snapshot),saved=changed.timeSeriesRuns.at(-1)
    saved.controls.outcome='a-column-that-was-not-fitted'
    const wrongControls=persistence.parseSnapshotValue(changed).ok
    const mismatch=structuredClone(snapshot),record=mismatch.timeSeriesRuns.at(-1)
    if(record.kind==='panel-regression')record.controls.confidence='0.8'
    else record.controls.model.covariates=[record.outcome.id]
    return {valid,wrongControls,wrongSpecification:persistence.parseSnapshotValue(mismatch).ok}
  },snapshot)
  expect(contract).toEqual({valid:true,wrongControls:false,wrongSpecification:false})
  if(mode==='bacon'){
    tolerance(run.evidence.twfe,bacon.twfe);expect(run.evidence.decomposition.components).toHaveLength(25)
  }else if(mode==='ddd'){
    expect(run.evidence.terms).toHaveLength(ddd.params.length)
    for(let i=0;i<ddd.params.length;i++){
      tolerance(run.evidence.terms[i].estimate,ddd.params[i]);tolerance(run.evidence.terms[i].standardError,ddd.se[i]);tolerance(run.evidence.terms[i].degreesOfFreedom,ddd.df[i])
      for(let j=0;j<ddd.params.length;j++)tolerance(run.evidence.covariance[i][j],ddd.covariance[i][j])
    }
  }else{
    for(const term of run.evidence.terms){
      const event=run.evidence.events.find((e:{index:number})=>e.index===term.index)
      const label=event===undefined?term.name:event.period<0?`lead${-event.period}`:`lag${event.period}`
      const j=lfe.terms.indexOf(label);expect(j).toBeGreaterThanOrEqual(0)
      tolerance(term.estimate,lfe.params[j]);tolerance(term.standardError,lfe.se[j]);tolerance(term.lower,lfe.interval[j][0]);tolerance(term.upper,lfe.interval[j][1])
    }
  }
  await info.attach('project-bundle',{body:Buffer.from(JSON.stringify(snapshot,null,2)),contentType:'application/json'})
  await expect.poll(()=>page.evaluate(async id=>{const store=await import(new URL('/src/data/projectStore.ts',location.href).href);const saved=await store.loadProject(id);return saved.ok?saved.value.timeSeriesRuns.length:0},snapshot.project.id)).toBe(1)
  await page.goto('/app/projects')
  await page.getByRole('button',{name:`Open Mixtape ${mode}`,exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(source)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Regression designs',exact:true}).click()
  await expect(result).toBeVisible({timeout:30_000})
  await page.getByRole('button',{name:'Restore this specification',exact:true}).click()
  await expect(page.getByRole('radio',{name:mode==='event-study'?'Event study':mode==='ddd'?'Interactions / DDD':'Bacon decomposition',exact:true})).toBeChecked()
  const restored=await exportProject(page);expect(restored.timeSeriesRuns.at(-1)).toEqual(run)
  expect(errors).toEqual([])
})
