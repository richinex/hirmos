import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {addArrow,chapter,choose,createDag,identify} from './examples/support'

const input=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/mpdta-input.json','utf8')) as Record<string,number>[]
const reference=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/mpdta.json','utf8'))
const csv=['unit,year,outcome,treated,lpop,state',...input.map(r=>[r.countyreal,r.year,r.lemp,r['first.treat']!==0&&r.year!>=r['first.treat']!?1:0,r.lpop,`state-${String(Math.floor(r.countyreal!/1000)).padStart(3,'0')}`].join(','))].join('\n')

test('cluster labels stay aligned through data and WASM workers and invalid groupings are refused',async({page})=>{
  test.setTimeout(120_000)
  await page.goto('/app')
  // Preserve R's numeric unit order when the kernel sorts string identities.
  const keyedCsv=['unit,year,outcome,treated,lpop,state',...input.map(r=>[
    `unit-${String(r.countyreal).padStart(8,'0')}`,r.year,r.lemp,
    r['first.treat']!==0&&r.year!>=r['first.treat']!?1:0,r.lpop,
    `state-${String(Math.floor(r.countyreal!/1000)).padStart(3,'0')}`,
  ].join(','))].join('\n')
  const result=await page.evaluate(async csv=>{
    const data=await import(new URL('/src/data/client.ts',location.href).href)
    const workflow=await import(new URL('/src/domain/workflow.ts',location.href).href)
    const analysis=await import(new URL('/src/analysis/client.ts',location.href).href)
    const d=await import(new URL('/src/domain/staggeredDid.ts',location.href).href)
    const panel=await import(new URL('/src/domain/panel.ts',location.href).href)
    const file=new File([csv],'clustered.csv',{type:'text/csv'})
    const profiled=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
    if(!profiled.ok)throw Error(JSON.stringify(profiled.error))
    const profile=profiled.value
    const column=(name:string)=>profile.columns.find((c:{name:string})=>c.name===name).id
    const selection={unit:column('unit'),time:column('year'),outcome:column('outcome'),treatment:column('treated'),covariates:[column('lpop')],clusterColumn:column('state')}
    const materialized=await data.materializePanelInWorker(file,profile,selection)
    if(!materialized.ok)throw Error(JSON.stringify(materialized.error))
    const matrix=materialized.value
    const spec={...d.defaultStaggeredSpecification,inference:{kind:'bootstrapSimultaneous',iterations:999,seed:731}}
    const clustering={kind:'column',column:column('state')}
    const prepared=d.staggeredInput(matrix,spec,clustering)
    if(!prepared.ok)throw Error(prepared.error)
    const run=await analysis.runStaggeredDid(prepared.value.values,prepared.value.model)
    if(!run.ok)throw Error(JSON.stringify(run.error))
    const labels=[...matrix.cluster.labels]
    const repeated=matrix.units.findIndex((u:string,i:number)=>i>0&&matrix.units.indexOf(u)<i)
    labels[repeated]='changed'
    const config={kind:'panel-intervention',primary:'staggered',covariates:selection.covariates,specification:spec}
    const noCluster={...matrix};delete noCluster.cluster
    const missingCsv=csv.replace(/state-\d+/,'')
    if(missingCsv===csv)throw Error('The missing-cluster fixture did not remove a label.')
    const missingFile=new File([missingCsv],'missing-cluster.csv',{type:'text/csv'})
    const missingProfile=await data.profileSourceInWorker(workflow.newImportRequestId(),missingFile)
    if(!missingProfile.ok)throw Error(JSON.stringify(missingProfile.error))
    const col=(name:string)=>missingProfile.value.columns.find((c:{name:string})=>c.name===name).id
    const missing=await data.materializePanelInWorker(missingFile,missingProfile.value,{unit:col('unit'),time:col('year'),outcome:col('outcome'),treatment:col('treated'),clusterColumn:col('state')})
    return {
      evidence:run.value,
      changed:d.staggeredInput({...matrix,cluster:{...matrix.cluster,labels}},spec,clustering).ok,
      absent:d.staggeredInput(noCluster,spec,clustering).ok,
      unrequested:d.staggeredInput(matrix,spec).ok,
      single:d.staggeredInput({...matrix,cluster:{...matrix.cluster,labels:labels.map(()=> 'one')}},spec,clustering).ok,
      analytical:d.staggeredInput(matrix,{...spec,inference:{kind:'analytical'}},clustering).ok,
      malformed:panel.parsePanelLongMatrix({...matrix,cluster:{...matrix.cluster,labels:[]}},profile).ok,
      missing:missing.ok,
      legacy:d.staggeredConfigurationSchema.safeParse(config).success,
      current:d.staggeredConfigurationSchema.safeParse({...config,clustering}).success,
      badConfig:d.staggeredConfigurationSchema.safeParse({...config,clustering,specification:{...spec,inference:{kind:'analytical'}}}).success,
    }
  },keyedCsv)
  const oracle=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/inference.json','utf8'))
  const expected=oracle.cases[1]
  expect(result.evidence.clusterCount).toBe(new Set(expected.clusters).size)
  for(let i=0;i<oracle.event.length;i++) {
    expect(result.evidence.events.intervals[i].standardError).toBeCloseTo(expected.se[i],9)
    expect(result.evidence.events.intervals[i].lower).toBeCloseTo(expected.simultaneous_lower[i],9)
    expect(result.evidence.events.intervals[i].upper).toBeCloseTo(expected.simultaneous_upper[i],9)
  }
  expect({...result,evidence:undefined}).toEqual({evidence:undefined,changed:false,absent:false,unrequested:false,single:false,analytical:false,malformed:false,missing:false,legacy:true,current:true,badConfig:false})
})

test('staggered DiD travels through the data and WASM workers and preserves source effects',async({page})=>{
  test.setTimeout(120_000)
  await page.goto('/app')
  const result=await page.evaluate(async(csv)=>{
    const data=await import(new URL('/src/data/client.ts',location.href).href)
    const workflow=await import(new URL('/src/domain/workflow.ts',location.href).href)
    const analysis=await import(new URL('/src/analysis/client.ts',location.href).href)
    const staggered=await import(new URL('/src/domain/staggeredDid.ts',location.href).href)
    const estimation=await import(new URL('/src/domain/estimation.ts',location.href).href)
    const file=new File([csv],'mpdta.csv',{type:'text/csv'})
    const profile=await data.profileSourceInWorker(workflow.newImportRequestId(),file)
    if(!profile.ok) throw Error(JSON.stringify(profile.error))
    const column=(name:string)=>profile.value.columns.find((c:{name:string})=>c.name===name).id
    const matrix=await data.materializePanelInWorker(file,profile.value,{unit:column('unit'),time:column('year'),outcome:column('outcome'),treatment:column('treated'),covariates:[column('lpop')]})
    if(!matrix.ok) throw Error(JSON.stringify(matrix.error))
    const run=async(spec:object)=>{
      const input=staggered.staggeredInput(matrix.value,spec)
      if(!input.ok) throw Error(input.error)
      const result=await analysis.runStaggeredDid(input.value.values,input.value.model)
      if(!result.ok) throw Error(JSON.stringify(result.error))
      return result.value
    }
    const specification={...staggered.defaultStaggeredSpecification,inference:{kind:'analytical'}}
    const evidence=await run(specification)
    const bootstrap=await run({...specification,inference:{kind:'bootstrapSimultaneous',iterations:999,seed:731}})
    const universal=await run({...specification,baseline:'universal'})
    const study={id:'study',estimand:{kind:'average-treatment-effect-on-treated',scale:'additive',treatedValue:1},outcome:{column:column('outcome')},treatment:{column:column('treated')}}
    const identification={result:{kind:'identified',adjustment:{kind:'canonical',variables:[]}}}
    const configuration={kind:'panel-intervention',primary:'staggered',covariates:[column('lpop')],specification}
    const artifact={kind:'panel-intervention-run',study:'study',configuration,evidence,columns:[study.outcome,study.treatment,{column:column('lpop')}],timeLabels:matrix.value.periods.map((p:{label:string})=>p.label),sourcePeriods:matrix.value.periods.map((p:{code:number;label:string})=>({code:p.code,label:p.label}))}
    const estimate=estimation.causalEstimateFrom(study,identification,artifact)
    const record={...artifact,estimate}
    const saved=staggered.staggeredRecordMatches(JSON.parse(JSON.stringify(record)),study)
    const missingLabels=staggered.staggeredRecordMatches({...record,sourcePeriods:undefined},study)
    const duplicateLabels=staggered.staggeredRecordMatches({...record,sourcePeriods:[...artifact.sourcePeriods,artifact.sourcePeriods[0]]},study)
    const corrupted=staggered.staggeredRecordMatches({...record,evidence:{...evidence,specification:{...specification,anticipation:1}}},study)
    const badValues=matrix.value.values.slice();badValues[matrix.value.rowCount]=0.5
    const invalid=staggered.staggeredInput({...matrix.value,values:badValues},specification)
    return {evidence,bootstrap,universal,saved,corrupted,invalid,estimate,missingLabels,duplicateLabels}
  },csv)
  const expected=reference.cases.nevertreated_varying_adjusted
  expect(result.evidence.events.keys).toEqual(expected.event)
  for(let i=0;i<expected.event.length;i++) {
    expect(result.evidence.events.intervals[i].estimate).toBeCloseTo(expected.event_att[i],9)
    expect(result.evidence.events.intervals[i].standardError).toBeCloseTo(expected.event_se[i],9)
    expect(result.bootstrap.events.intervals[i].estimate).toBeCloseTo(result.evidence.events.intervals[i].estimate,12)
  }
  expect(result.missingLabels).toBe(false)
  expect(result.duplicateLabels).toBe(false)
  expect(result.estimate.effect.value).toBeCloseTo(expected.overall_att,9)
  expect(result.evidence.overall.simple.estimate).toBeCloseTo(expected.aggregations.simple.overall_att,9)
  expect(result.evidence.overall.simple.standardError).toBeCloseTo(expected.aggregations.simple.overall_se,9)
  expect(result.bootstrap.events.coverage.kind).toBe('simultaneous')
  expect(result.evidence.events.coverage.kind).toBe('pointwise')
  expect(result.bootstrap.overall.dynamic.estimate).toBeCloseTo(result.evidence.overall.dynamic.estimate,12)
  expect(result.universal.cells.intervals.some((i:{kind:string})=>i.kind==='reference')).toBe(true)
  expect(result.saved).toBe(true)
  expect(result.corrupted).toBe(false)
  expect(result.invalid.ok).toBe(false)
})

test('staggered schemas refuse fabricated references and mismatched support',async({page})=>{
  await page.goto('/app')
  const result=await page.evaluate(async()=>{
    const d=await import(new URL('/src/domain/staggeredDid.ts',location.href).href)
    return {
      reference:d.staggeredIntervalSchema.safeParse({kind:'reference',estimate:0}).success,
      interval:d.staggeredIntervalSchema.safeParse({kind:'estimated',estimate:2,standardError:1,lower:3,upper:4}).success,
      window:d.staggeredSpecificationSchema.safeParse({...d.defaultStaggeredSpecification,firstEvent:4,lastEvent:2}).success,
      inference:d.staggeredInferenceSchema.safeParse({kind:'analytical',iterations:999}).success,
    }
  })
  expect(result).toEqual({reference:false,interval:false,window:false,inference:false})
})

test('staggered adoption completes through the UI and restores its plots',async({page},info)=>{
  test.setTimeout(240_000)
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill('Staggered DiD source verification')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'mpdta.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:/^Panel/}).click()
  await choose(page,'Unit column','unit')
  await choose(page,'Time column','year')
  for(const name of ['outcome','treated','lpop']) await page.getByRole('checkbox',{name,exact:true}).first().check()
  await page.getByRole('button',{name:/Create prepared/}).click()
  await expect(page.getByRole('heading',{name:'Build a DAG or run discovery',exact:true})).toBeVisible({timeout:60_000})
  await createDag(page,'Treatment and employment')
  await addArrow(page,'treated','outcome','Treatment may change county employment.')
  await identify(page,{graph:'Treatment and employment',treatment:'treated',outcome:'outcome',target:/Treated rows/,mechanism:'Policy change',sentence:'Compare adoption cohorts with counties not yet or never treated under parallel untreated trends.'})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:/^Interventions/}).click()
  await page.getByRole('radio',{name:/Panel difference-in-differences/}).click()
  await page.getByRole('radio',{name:'Staggered adoption',exact:true}).click()
  await expect(page.getByRole('radio',{name:'Simultaneous bootstrap',exact:true})).toBeChecked()
  const help=page.getByTestId('staggered-did-controls').getByRole('button',{name:'About Uncertainty',exact:true})
  if(info.project.name==='mobile-chromium') await help.click()
  else await help.hover()
  await expect(page.getByRole(info.project.name==='mobile-chromium'?'dialog':'tooltip')).not.toBeEmpty()
  await page.keyboard.press('Escape')
  await page.getByRole('group',{name:'Staggered DiD covariates',exact:true}).getByRole('checkbox',{name:'lpop',exact:true}).check()
  await choose(page,'Staggered cluster column','state')
  await page.getByRole('radio',{name:'Analytical',exact:true}).click()
  await expect(page.getByRole('button',{name:'Run panel DiD',exact:true}).first()).toBeDisabled()
  await expect(page.getByText('Select pointwise or simultaneous bootstrap to use the cluster column.',{exact:true})).toBeVisible()
  await page.getByRole('radio',{name:'Simultaneous bootstrap',exact:true}).click()
  await page.getByTestId('staggered-did-controls').scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('staggered-controls.png')})
  const run=page.getByRole('button',{name:'Run panel DiD',exact:true}).first()
  await expect(run).toBeEnabled()
  await run.click()
  const result=page.getByTestId('staggered-did-result').first()
  await expect(result).toBeVisible({timeout:60_000})
  const simple=result.getByRole('table',{name:'Simple ATT',exact:true})
  await expect(simple).toBeVisible()
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('staggered-event-study.png')})
  expect(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth+1)).toBe(true)
  for(const view of ['Cohorts','Calendar','Group-time']) {
    await result.getByRole('radio',{name:view,exact:true}).click()
    await page.screenshot({path:info.outputPath(`staggered-${view.toLowerCase()}.png`)})
  }
  await result.getByRole('radio',{name:'Event study',exact:true}).click()
  await page.getByRole('button',{name:'Change theme',exact:true}).click()
  await expect(page.locator('html')).not.toHaveAttribute('data-theme-transition','')
  if(await page.locator('html').getAttribute('data-theme')!=='dark') await page.getByRole('button',{name:'Change theme',exact:true}).click()
  await expect(page.locator('html')).toHaveAttribute('data-theme','dark')
  await expect(page.locator('html')).not.toHaveAttribute('data-theme-transition','')
  await page.screenshot({path:info.outputPath('staggered-dark.png')})
  await page.getByRole('button',{name:'Open Event-study effects in a floating window',exact:true}).first().click()
  await expect(page.getByRole('dialog')).toBeVisible()
  await page.screenshot({path:info.outputPath('staggered-expanded.png')})
  await page.keyboard.press('Escape')
  const menu=page.getByRole('button',{name:'Expand section list',exact:true})
  if(await menu.isVisible()) await menu.click()
  const download=page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path=await(await download).path()
  if(!path) throw Error('No exported project')
  const snapshot=JSON.parse(readFileSync(path,'utf8')).project
  expect(snapshot.estimationRuns).toHaveLength(1)
  const simpleInterval=snapshot.estimationRuns[0].evidence.overall.simple
  expect(simpleInterval.kind).toBe('estimated')
  const simpleCells=await page.evaluate(async interval=>{
    const {formatStatistic}=await import(new URL('/src/lib/format/number.ts',location.href).href)
    return [interval.estimate,interval.standardError,interval.lower,interval.upper].map(value=>formatStatistic('raw',value).text)
  },simpleInterval)
  await expect(simple.getByRole('cell')).toHaveText(simpleCells)
  expect(snapshot.estimationRuns[0].configuration.clustering.kind).toBe('column')
  expect(snapshot.estimationRuns[0].evidence.clusterCount).toBeLessThan(500)
  expect(snapshot.estimationRuns[0].evidence.overall.dynamic.estimate).toBeCloseTo(reference.cases.nevertreated_varying_adjusted.overall_att,9)
  const restored=await page.evaluate(async snapshot=>{
    const persistence=await import(new URL('/src/domain/persistence.ts',location.href).href)
    const valid=persistence.parseSnapshotValue(snapshot)
    const bad=structuredClone(snapshot);bad.estimationRuns[0].evidence.version=2
    const cluster=structuredClone(snapshot);cluster.estimationRuns[0].configuration.clustering.column='unknown-column'
    const unit=structuredClone(snapshot);unit.estimationRuns[0].configuration.clustering={kind:'unit'}
    return {valid:valid.ok,invalid:persistence.parseSnapshotValue(bad).ok,unknownCluster:persistence.parseSnapshotValue(cluster).ok,changedClustering:persistence.parseSnapshotValue(unit).ok}
  },snapshot)
  expect(restored).toEqual({valid:true,invalid:false,unknownCluster:false,changedClustering:false})
  await page.reload()
  await page.getByRole('button',{name:'Open Staggered DiD source verification',exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles({name:'mpdta.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
  await expect(page.getByRole('navigation',{name:'Workspace sections'}).getByRole('button',{name:/Estimation/})).not.toHaveAttribute('aria-disabled','true',{timeout:60_000})
  await chapter(page,/Estimation/)
  await expect(page.getByTestId('staggered-did-result').first()).toBeVisible()
  await expect(page.getByTestId('staggered-did-result').first().getByRole('table',{name:'Simple ATT',exact:true})).toBeVisible()
  await expect(page.getByTestId('staggered-did-result').first().getByRole('table',{name:'Simple ATT',exact:true}).getByRole('cell')).toHaveText(simpleCells)
  await page.getByRole('radio',{name:'Staggered adoption',exact:true}).click()
  await page.getByRole('spinbutton',{name:'Staggered bootstrap replications',exact:true}).fill('500000')
  await page.getByRole('button',{name:'Run panel DiD',exact:true}).first().click()
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toBeVisible()
  await chapter(page,/Study design/)
  await chapter(page,/Estimation/)
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toBeVisible()
  await page.getByRole('button',{name:'Cancel run',exact:true}).click()
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toHaveCount(0)
  await expect(page.getByTestId('staggered-did-result').first()).toBeVisible()
})
