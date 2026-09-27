import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {addArrow,chapter,choose,createDag,identify} from './examples/support'

const input=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/mpdta-input.json','utf8')) as Record<string,number>[]
const reference=JSON.parse(readFileSync('crates/causal-core/fixtures/staggered-did/mpdta.json','utf8'))
const csv=['unit,year,outcome,treated,lpop',...input.map(r=>[r.countyreal,r.year,r.lemp,r['first.treat']!==0&&r.year!>=r['first.treat']!?1:0,r.lpop].join(','))].join('\n')

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
    const artifact={kind:'panel-intervention-run',study:'study',configuration,evidence,columns:[study.outcome,study.treatment,{column:column('lpop')}],timeLabels:matrix.value.periods.map((p:{label:string})=>p.label)}
    const estimate=estimation.causalEstimateFrom(study,identification,artifact)
    const record={...artifact,estimate}
    const saved=staggered.staggeredRecordMatches(JSON.parse(JSON.stringify(record)),study)
    const corrupted=staggered.staggeredRecordMatches({...record,evidence:{...evidence,specification:{...specification,anticipation:1}}},study)
    const badValues=matrix.value.values.slice();badValues[matrix.value.rowCount]=0.5
    const invalid=staggered.staggeredInput({...matrix.value,values:badValues},specification)
    return {evidence,bootstrap,universal,saved,corrupted,invalid,estimate}
  },csv)
  const expected=reference.cases.nevertreated_varying_adjusted
  expect(result.evidence.events.keys).toEqual(expected.event)
  for(let i=0;i<expected.event.length;i++) {
    expect(result.evidence.events.intervals[i].estimate).toBeCloseTo(expected.event_att[i],9)
    expect(result.evidence.events.intervals[i].standardError).toBeCloseTo(expected.event_se[i],9)
    expect(result.bootstrap.events.intervals[i].estimate).toBeCloseTo(result.evidence.events.intervals[i].estimate,12)
  }
  expect(result.estimate.effect.value).toBeCloseTo(expected.overall_att,9)
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
  await expect(page.getByText('This choice changes standard errors and intervals, not ATT estimates.',{exact:false}).first()).toBeVisible()
  await page.keyboard.press('Escape')
  await page.getByRole('group',{name:'Staggered DiD covariates',exact:true}).getByRole('checkbox',{name:'lpop',exact:true}).check()
  await page.getByTestId('staggered-did-controls').scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('staggered-controls.png')})
  const run=page.getByRole('button',{name:/^Run panel difference-in-differences/i}).first()
  await expect(run).toBeEnabled()
  await run.click()
  const result=page.getByTestId('staggered-did-result').first()
  await expect(result).toBeVisible({timeout:60_000})
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('staggered-event-study.png')})
  expect(await page.locator('body').evaluate(e=>e.scrollWidth<=innerWidth+1)).toBe(true)
  for(const view of ['Cohorts','Calendar','Group-time']) {
    await result.getByRole('radio',{name:view,exact:true}).click()
    await page.screenshot({path:info.outputPath(`staggered-${view.toLowerCase()}.png`)})
  }
  await result.getByRole('radio',{name:'Event study',exact:true}).click()
  await page.getByRole('button',{name:'Change theme',exact:true}).click()
  if(await page.locator('html').getAttribute('data-theme')!=='dark') await page.getByRole('button',{name:'Change theme',exact:true}).click()
  await page.screenshot({path:info.outputPath('staggered-dark.png')})
  await page.getByRole('button',{name:'Open Event-study effects in a floating window',exact:true}).first().click()
  await expect(page.getByRole('dialog')).toBeVisible()
  await page.screenshot({path:info.outputPath('staggered-expanded.png')})
  await page.keyboard.press('Escape')
  const menu=page.getByRole('button',{name:'Expand chapter list',exact:true})
  if(await menu.isVisible()) await menu.click()
  const download=page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path=await(await download).path()
  if(!path) throw Error('No exported project')
  const snapshot=JSON.parse(readFileSync(path,'utf8')).project
  expect(snapshot.estimationRuns).toHaveLength(1)
  expect(snapshot.estimationRuns[0].evidence.overall.dynamic.estimate).toBeCloseTo(reference.cases.nevertreated_varying_adjusted.overall_att,9)
  const restored=await page.evaluate(async snapshot=>{
    const persistence=await import(new URL('/src/domain/persistence.ts',location.href).href)
    const valid=persistence.parseSnapshotValue(snapshot)
    const bad=structuredClone(snapshot);bad.estimationRuns[0].evidence.version=2
    return {valid:valid.ok,invalid:persistence.parseSnapshotValue(bad).ok}
  },snapshot)
  expect(restored).toEqual({valid:true,invalid:false})
  await page.reload()
  await page.getByRole('button',{name:'Open Staggered DiD source verification',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'mpdta.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
  await expect(page.getByRole('navigation',{name:'Workspace chapters'}).getByRole('button',{name:/Estimation/})).not.toHaveAttribute('aria-disabled','true',{timeout:60_000})
  await chapter(page,/Estimation/)
  await expect(page.getByTestId('staggered-did-result').first()).toBeVisible()
  await page.getByRole('radio',{name:'Staggered adoption',exact:true}).click()
  await page.getByRole('spinbutton',{name:'Staggered bootstrap replications',exact:true}).fill('500000')
  await page.getByRole('button',{name:/^Run panel difference-in-differences/i}).first().click()
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toBeVisible()
  await chapter(page,/Study design/)
  await chapter(page,/Estimation/)
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toBeVisible()
  await page.getByRole('button',{name:'Cancel run',exact:true}).click()
  await expect(page.getByRole('button',{name:'Cancel run',exact:true})).toHaveCount(0)
  await expect(page.getByTestId('staggered-did-result').first()).toBeVisible()
})
