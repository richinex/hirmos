import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {z} from 'zod'
import {chapter,choose,prepare} from './examples/support'
import {surrogateRunSchema} from '../src/domain/surrogateRun'
import {surrogateSelectionSchema} from '../src/domain/surrogateSamples'

test('horizon selections and records reject changed group membership and missing results',()=>{
  const horizons={observed:{kind:'none'},windows:{kind:'groups',groups:[{label:'Early',columns:['s1']},{label:'Late',columns:['s2']}]}}
  const selection={sample:{column:'sample',experimental:1,observational:0},treatment:'w',outcome:'y',surrogates:['s1','s2'],adjustment:{kind:'none'},estimator:'index',uncertainty:{kind:'none'},horizons}
  const evidence={estimator:'index',experimentalRows:4,observationalRows:4,surrogateColumns:2,baselineColumns:0,estimate:2,uncertainty:{kind:'none'}}
  const results=[{kind:'surrogateWindows',windows:[{label:'Early',evidence:{...evidence,surrogateColumns:1,estimate:1}},{label:'Late',evidence}]}]
  const record={kind:'surrogate-run',id:'313803fd-9883-4d57-9b14-8d28c0fce2d5',createdAt:'2026-10-03T12:00:00.000Z',sourceFingerprint:'a'.repeat(64),sourceRows:8,
    selection,evidence,paths:{specification:horizons,results},rows:{experimental:[0,1,2,3],observational:[4,5,6,7],excluded:[]},rationale:'Synthetic horizon test.'}
  expect(surrogateRunSchema.safeParse(record).success).toBe(true)
  const changed={...horizons,windows:{kind:'groups',groups:[{label:'Early',columns:['s2']},{label:'Late',columns:['s1']}]}}
  expect(surrogateRunSchema.safeParse({...record,selection:{...selection,horizons:changed}}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...record,paths:{specification:horizons,results:[]}}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...record,paths:{specification:horizons,results:[...results,...results]}}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...record,evidence:{...evidence,estimate:4}}).success).toBe(false)
  for(const groups of [[],[{label:'a',columns:['s1']}],[{label:'a',columns:['s1','s1']}],[{label:'a',columns:['s1','s3']}],[{label:'a',columns:['s1']},{label:'a',columns:['s2']}]])
    expect(surrogateSelectionSchema.safeParse({...selection,horizons:{...horizons,windows:{kind:'groups',groups}}}).success).toBe(false)
  for(const periods of [[],[{label:'a',column:'w'}],[{label:'a',column:'s1'},{label:'a',column:'s2'}],[{label:'a',column:'s1'},{label:'b',column:'s1'}]])
    expect(surrogateSelectionSchema.safeParse({...selection,horizons:{observed:{kind:'periods',periods},windows:{kind:'none'}}}).success).toBe(false)
})

const base=z.object({experimental:z.array(z.array(z.number())),observational:z.array(z.array(z.number())),treatment:z.array(z.number()),outcome:z.array(z.number()),experimental_outcome:z.array(z.number()),effect:z.number(),naive_effects:z.array(z.number())})
const fixture=z.object({cases:z.record(z.string(),z.object({base}))}).parse(JSON.parse(readFileSync(new URL('../../octopus/rust-causal-transpile/oracle/surrogate-index/fixtures.json',import.meta.url),'utf8')))

test('horizon controls, plots and saved specifications work through the UI',async({page},info)=>{
  test.setTimeout(120000)
  const b=fixture.cases.employ_randomized_3!.base
  const csv=['sample,treatment,outcome,s1,s2,s3',...b.experimental.map((r,i)=>[1,b.treatment[i],b.experimental_outcome[i],...r.slice(1)].join(',')),...b.observational.map((r,i)=>[0,'',b.outcome[i],...r.slice(1)].join(','))].join('\n')
  const file={name:'surrogate-horizons.csv',mimeType:'text/csv',buffer:Buffer.from(csv)}
  const errors:string[]=[];page.on('pageerror',e=>errors.push(e.message))
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name',exact:true}).fill('Surrogate horizon acceptance')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(file)
  await page.getByRole('button',{name:/Inspect data/}).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({timeout:60000})
  await prepare(page,{structure:'cross-section',columns:['s1']})
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await choose(page,'Sample membership','sample')
  await choose(page,'Treatment in experimental sample','treatment')
  await choose(page,'Long-term outcome in observational sample','outcome')
  for(const name of ['s1','s2','s3'])await page.getByRole('group',{name:'Short-term surrogates',exact:true}).getByRole('checkbox',{name,exact:true}).check()
  await page.getByRole('textbox',{name:'Identifying-assumption rationale'}).fill('Synthetic oracle exercise. The long-term target and sample rows remain fixed across surrogate horizons; the recorded observed paths are unadjusted comparisons, not a separate identification strategy.')
  await page.getByRole('radio',{name:'Bootstrap standard error',exact:true}).check()
  await page.getByRole('textbox',{name:'Bootstrap repetitions',exact:true}).fill('12')
  await page.getByRole('textbox',{name:'Bootstrap seed',exact:true}).fill('19')
  await page.getByRole('radio',{name:'Observed outcomes by period',exact:true}).check()
  await expect(page.getByText('Choose at least one observed outcome period.',{exact:true})).toBeVisible()
  await expect(page.getByText(/Invalid input: expected/)).toHaveCount(0)
  for(let i=1;i<=3;i++){
    await page.getByRole('group',{name:'Outcome periods',exact:true}).getByRole('checkbox',{name:`s${i}`,exact:true}).check()
    await page.getByRole('textbox',{name:`Period ${i} label`,exact:true}).fill(`Period ${i}`)
  }
  await page.getByRole('button',{name:'Move period 2 earlier',exact:true}).click()
  await expect(page.getByRole('textbox',{name:'Period 1 label',exact:true})).toHaveValue('Period 2')
  await page.getByRole('button',{name:'Move period 1 later',exact:true}).click()
  await page.getByRole('radio',{name:'Compare surrogate windows',exact:true}).check()
  await page.getByRole('checkbox',{name:'Window ends at s1',exact:true}).check()
  await page.getByRole('textbox',{name:'Window label for s1',exact:true}).fill('')
  await expect(page.getByRole('button',{name:'Estimate long-term effect',exact:true})).toBeDisabled()
  await page.getByRole('textbox',{name:'Window label for s1',exact:true}).fill('Early window')
  await page.getByRole('textbox',{name:'Window label for s3',exact:true}).fill('Full window')
  await expect(page.getByRole('checkbox',{name:'Window ends at s3',exact:true})).toBeDisabled()
  await page.getByRole('radio',{name:'Compare surrogate windows',exact:true}).scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('horizon-controls.png')})
  await page.getByRole('button',{name:'Estimate long-term effect',exact:true}).click()
  const observed=page.getByRole('region',{name:'Observed experimental outcome paths',exact:true}),windows=page.getByRole('region',{name:'Surrogate window estimates',exact:true})
  await expect(windows).toBeVisible({timeout:30000})
  await expect.poll(()=>page.evaluate(async()=>{
    const store=await import(new URL('/src/data/projectStore.ts',location.href).href)
    const header=(await store.listProjects()).find((p:{name:string})=>p.name==='Surrogate horizon acceptance')
    if(!header)return false
    const result=await store.loadProject(header.id)
    return result.ok && result.value.surrogateRuns.length===1
  })).toBe(true)
  const saved=await page.evaluate(async()=>{
    const store=await import(new URL('/src/data/projectStore.ts',location.href).href)
    const header=(await store.listProjects()).find((p:{name:string})=>p.name==='Surrogate horizon acceptance')
    const result=await store.loadProject(header.id)
    if(!result.ok)throw Error('Save failed')
    return result.value.surrogateRuns[0]
  })
  const run=surrogateRunSchema.parse(saved)
  const path=run.paths.results.find(p=>p.kind==='surrogateWindows')!
  expect(path.kind).toBe('surrogateWindows')
  if(path.kind==='surrogateWindows'){
    expect(path.windows[0]!.evidence.estimate).toBeCloseTo(fixture.cases.employ_randomized_1!.base.effect,9)
    expect(path.windows[1]!.evidence.estimate).toBeCloseTo(b.effect,9)
  }
  const observedPath=run.paths.results.find(p=>p.kind==='observedOutcomes')!
  if(observedPath.kind==='observedOutcomes'){
    observedPath.periods.forEach((p,i)=>expect(p.cumulativeMeanContrast).toBeCloseTo(b.naive_effects[i]!,9))
    expect(observedPath.cumulativeUncertainty.kind).toBe('bootstrapStandardErrors')
    if(observedPath.cumulativeUncertainty.kind==='bootstrapStandardErrors'){
      expect(observedPath.cumulativeUncertainty.standardErrors).toHaveLength(3)
      expect(observedPath.cumulativeUncertainty.repetitions).toBe(12)
      expect(observedPath.cumulativeUncertainty.seed).toBe(19)
    }
  }
  await expect(observed).toContainText('Cumulative mean bootstrap SE')
  for(const theme of ['light','dark']){
    await page.evaluate(t=>{document.documentElement.dataset.theme=t},theme)
    for(const id of ['surrogate-observed-path','surrogate-window-estimates']){
      await page.getByTestId(id).filter({visible:true}).first().scrollIntoViewIfNeeded()
      await page.screenshot({path:info.outputPath(`${id}-${theme}.png`),animations:'disabled'})
    }
  }
  await chapter(page,/Results/)
  await expect(windows).toBeVisible()
  await page.goto('/app/projects')
  await page.getByRole('button',{name:'Open Surrogate horizon acceptance',exact:true}).click()
  await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles(file)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:'Two-sample surrogates',exact:true}).check()
  await expect(windows).toBeVisible()
  await expect(page.getByRole('textbox',{name:'Window label for s3',exact:true})).toHaveValue('Full window')
  await page.getByRole('textbox',{name:'Period 1 label',exact:true}).fill('Changed')
  await page.getByText('Specification and recorded assumptions',{exact:true}).first().click()
  await page.getByRole('button',{name:'Restore this specification',exact:true}).first().click()
  await expect(page.getByRole('textbox',{name:'Period 1 label',exact:true})).toHaveValue('Period 1')
  await page.getByRole('button',{name:'Mark all window ends',exact:true}).click()
  await expect(page.getByRole('textbox',{name:'Window label for s2',exact:true})).toHaveValue('s2')
  await expect(page.getByRole('textbox',{name:'Window label for s1',exact:true})).toHaveValue('Early window')
  await page.getByRole('button',{name:'Clear window ends',exact:true}).click()
  await expect(page.getByRole('textbox',{name:'Window label for s1',exact:true})).toHaveCount(0)
  await expect(page.getByRole('button',{name:'Estimate long-term effect',exact:true})).toBeEnabled()
  expect(errors).toEqual([])
})
