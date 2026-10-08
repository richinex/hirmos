import {test,expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {choose} from './examples/support'
const oracle=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/network_query_bnlearn.json','utf8'))
const maintenance=JSON.parse(readFileSync('docs/medium/data/maintenance-reference/maintenance-reference.json','utf8'))

test('maintenance preserves reference categories and matches every oracle query',async({page})=>{
  const csv=readFileSync('docs/medium/data/maintenance-reference/maintenance-discrete.csv','utf8')
  await page.goto('/app')
  const errors=await page.evaluate(async({f,csv})=>{
    const {runNetworkQuery}=await import(new URL('/src/analysis/client.ts',location.href).href)
    const lines=csv.trim().split('\n').slice(1).map((l:string)=>l.split(',').map(Number))
    const values=f.names.flatMap((_:string,c:number)=>lines.map((r:number[])=>r[c]))
    const errors:number[]=[]
    for(const q of f.queries){
      const actual=await runNetworkQuery(new Float64Array(values),{rows:lines.length,columns:f.names.length,names:f.names,edges:f.edges,outcomes:q.outcomes,observations:q.observations,interventions:q.interventions,bins:3,equivalentSampleSize:f.equivalentSampleSize})
      if(!actual.ok)throw Error(JSON.stringify(actual.error))
      actual.value.states.forEach((states:[string,number][],i:number)=>{
        if(states.length!==f.encoding[f.names[i]].length||states.some(([label,value],j)=>label!==String(j)||value!==j))throw Error('Category encoding was changed')
      })
      actual.value.distribution.forEach(([states,p]:[string[],number],i:number)=>{
        if(JSON.stringify(states)!==JSON.stringify(q.distribution[i][0]))throw Error('Joint state order changed')
        errors.push(Math.abs(p-q.distribution[i][1]))
      })
    }
    return errors
  },{f:maintenance,csv})
  expect(errors).toHaveLength(26)
  expect(Math.max(...errors)).toBeLessThan(1e-12)
})

test('general network worker agrees with bnlearn for all query shapes and priors',async({page})=>{
  await page.goto('/app')
  const differences=await page.evaluate(async f=>{
    const {runNetworkQuery}=await import(new URL('/src/analysis/client.ts',location.href).href)
    const errors:number[]=[]
    for(const c of f.cases){
      const rows=c.counts.reduce((a:number,b:number)=>a+b,0)
      const values=f.names.flatMap((_:string,col:number)=>f.patterns.flatMap((p:number[],i:number)=>Array(c.counts[i]).fill(p[col])))
      for(const q of c.queries){
        const result=await runNetworkQuery(new Float64Array(values),{rows,columns:4,names:f.names,edges:f.edges,outcomes:q.outcomes,interventions:q.interventions,observations:q.observations,bins:3,equivalentSampleSize:c.equivalentSampleSize})
        if(!result.ok)throw Error(JSON.stringify(result.error))
        result.value.distribution.forEach(([s,p]:[string[],number],i:number)=>{if(JSON.stringify(s)!==JSON.stringify(q.distribution[i][0]))throw Error('State ordering changed');errors.push(Math.abs(p-q.distribution[i][1]))})
      }
    }
    return errors
  },oracle)
  expect(differences.length).toBe(42)
  expect(Math.max(...differences)).toBeLessThan(1e-12)
})

test('sprinkler article queries are expressible, visible and saved',async({page},info)=>{
  test.setTimeout(120_000)
  const c=oracle.cases[0]
  const rows=oracle.patterns.flatMap((p:number[],i:number)=>Array(c.counts[i]).fill(p.join(',')))
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill('Article probability queries')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'article-sprinkler.csv',mimeType:'text/csv',buffer:Buffer.from([oracle.names.join(','),...rows].join('\n'))})
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:/Independent observations/}).check()
  await page.getByRole('button',{name:'Select all columns',exact:true}).click()
  await page.getByRole('button',{name:/Create prepared/}).click()
  await page.getByRole('button',{name:/Build a DAG/}).click()
  await page.getByRole('button',{name:/Substantive knowledge/}).click()
  await page.getByLabel('DAG name').fill('Sprinkler')
  await page.getByRole('button',{name:/Create DAG/}).click()
  await page.getByRole('button',{name:'From text',exact:true}).click()
  await page.getByRole('textbox',{name:'Graph text'}).fill('dag { Cloudy -> Sprinkler; Cloudy -> Rain; Sprinkler -> Wet_Grass; Rain -> Wet_Grass; }')
  await page.getByRole('button',{name:'Convert to DAG',exact:true}).click()
  const inspector=page.getByRole('button',{name:'Inspector',exact:true})
  if(await inspector.isVisible())await inspector.click()
  await page.getByRole('radio',{name:'Intervene',exact:true}).check()
  await expect(page.getByRole('spinbutton',{name:'Equivalent sample size',exact:true})).toHaveValue('1000')
  await page.getByRole('radio',{name:'Distribution',exact:true}).check()
  const records=page.getByRole('list',{name:'Probability queries'}).locator(':scope > li')
  for(const [index,q] of c.queries.entries()){
    for(const [i,name] of oracle.names.entries()){
      const action=q.interventions.find((a:{variable:number})=>a.variable===i)
      const evidence=q.observations.find((a:{variable:number})=>a.variable===i)
      await choose(page,`${name} query role`,q.outcomes.includes(i)?'Read outcome':action?'Intervene':evidence?'Observe':'Marginalise')
      if(action||evidence)await page.getByRole('spinbutton',{name:`${name} state index`,exact:true}).fill(String((action??evidence).state))
    }
    await page.getByRole('button',{name:'Evaluate probability',exact:true}).click()
    await expect(records).toHaveCount(index+1)
    for(const [,p] of q.distribution)await expect(records.first()).toContainText(p.toFixed(6))
  }
  await records.first().scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('network-query.png')})
  await choose(page,'Parameter estimation','BDeu')
  await expect(page.getByRole('spinbutton',{name:'Query equivalent sample size',exact:true})).toHaveValue('1000')
  await page.getByRole('spinbutton',{name:'Sprinkler state index',exact:true}).fill('2')
  await page.getByRole('button',{name:'Evaluate probability',exact:true}).click()
  await expect(page.getByText('State 2 is absent for Sprinkler.',{exact:false})).toBeVisible()
  await expect(records).toHaveCount(6)
  await expect.poll(()=>page.evaluate(async()=>{
    const store=await import(new URL('/src/data/projectStore.ts',location.href).href)
    const header=(await store.listProjects())[0];const saved=await store.loadProject(header.id)
    if(!saved.ok)return 0
    const {parseSnapshotValue}=await import(new URL('/src/domain/persistence.ts',location.href).href)
    if(!parseSnapshotValue(saved.value).ok)throw Error('Saved query failed decoding')
    return saved.value.interventionQueries.length
  })).toBe(6)
  await page.goto('/app/projects')
  await page.getByRole('button',{name:'Open Article probability queries',exact:true}).click()
  await page.locator('input[type=file][accept*=".csv"]').setInputFiles({name:'article-sprinkler.csv',mimeType:'text/csv',buffer:Buffer.from([oracle.names.join(','),...rows].join('\n'))})
  const sections=page.getByRole('button',{name:'Expand section list',exact:true})
  if(await sections.isVisible())await sections.click()
  await page.getByRole('navigation',{name:'Workspace sections'}).getByRole('button',{name:/DAG workspace/}).click()
  if(info.project.name==='mobile-chromium')await inspector.click()
  await page.getByRole('radio',{name:'Intervene',exact:true}).check()
  await page.getByRole('radio',{name:'Distribution',exact:true}).check()
  await expect(records).toHaveCount(6)
  await expect(records.last()).toContainText('0.927000')
})
