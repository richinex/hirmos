import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { choose, prepare } from './examples/support'

const root = new URL('../../octopus/rust-causal-transpile/oracle/fixtures/', import.meta.url)
const reference = JSON.parse(readFileSync(new URL('ardl_paper.json', root), 'utf8'))
const searches = JSON.parse(readFileSync(new URL('ardl_paper_search.json', root), 'utf8'))
const names = ['w', 'Prod', 'UR', 'Wedge', 'Union', 'D7475', 'D7579']
function close(actual: number, expected: number, label: string) {
  if (!Number.isFinite(actual) || Math.abs(actual - expected) > 1e-8 * Math.max(1, Math.abs(expected))) {
    throw new Error(`${label}: ${actual} differs from ${expected}`)
  }
}

test('all 38 R ECM fixtures pass through WASM with both deterministic bounds cases', async ({ page }) => {
  test.setTimeout(180_000)
  page.on('pageerror', error => console.error(error.message))
  await page.goto('/app')
  for (const input of reference.cases) for (const bounds of input.bounds) {
    const result = await page.evaluate(async ({ input, bounds }) => {
      const client = await import(new URL('/src/analysis/client.ts', location.href).href)
      const domain = await import(new URL('/src/domain/ardlModel.ts', location.href).href)
      const columns = [input.y, ...input.x, ...input.fixed]
      const request = domain.ardlModelRequestSchema.parse({ outcome: 0, predictors: [1,2,3,4], fixed: [5,6],
        terms: ({2:'restricted-constant',3:'constant',4:'restricted-trend',5:'trend'})[bounds.case as 2|3|4|5],
        orders: { kind:'rFixed',outcomeLag:input.order[0],predictorLags:input.order.slice(1) }, holdBack:input.hold_back,
        multiplierHorizon:0,future:{kind:'none'} })
      return client.runArdlModel(new Float64Array(columns.flat()),input.y.length,columns.length,request)
    }, { input, bounds })
    expect(result.ok, `${input.name}: ${JSON.stringify(result)}`).toBe(true)
    if (!result.ok) throw Error(JSON.stringify(result))
    const r = result.value.rAnalysis
    expect(r.kind).toBe('recorded')
    close(r.aicPss,input.aic_pss,input.name)
    close(r.sbcPss,input.sbc_pss,input.name)
    close(r.boundsF,bounds.f,input.name)
    if (typeof bounds.t==='number') close(r.boundsT.value,bounds.t,input.name)
    else expect(r.boundsT.kind).toBe('notApplicable')
    expect(result.value.longRun.kind).toBe('uncalibrated')
    expect(r.params.length).toBe(input.params.length)
    r.params.forEach((v:number,i:number)=>close(v,input.params[i],`${input.name} coefficient ${i}`))
    r.covariance.forEach((row:number[],i:number)=>row.forEach((v,j)=>close(v,input.covariance[i][j],`${input.name} covariance ${i},${j}`)))
    expect(r.residuals.length).toBe(input.residuals.length)
    r.residuals.forEach((v:number,i:number)=>close(v,input.residuals[i],`${input.name} residual ${i}`))
    expect(r.serialCorrelation.length).toBe(5)
    r.serialCorrelation.forEach((test:{statistic:number,pValue:number},i:number)=>{
      close(test.statistic,input.bg[i].stat,`${input.name} BG ${i+1}`)
      close(test.pValue,input.bg[i].p,`${input.name} BG p ${i+1}`)
    })
  }
})

for (const mode of ['rHorizontal','rGrid','rFixed'] as const) {
  test(`${mode} runs from the UI, preserves the result and matches R`, async ({page},info)=>{
    test.setTimeout(180_000)
    const input = mode==='rFixed' ? reference.cases.find((c:{name:string})=>c.name==='wide-c-4-1-0-5-0') : reference.cases[0]
    const columns=[input.y,...input.x,...input.fixed]
    const start=mode==='rFixed'?1971:1970
    const csv=['date,'+names.join(','),...input.y.map((_:number,i:number)=>`${start+Math.floor(i/4)}-${String((i%4)*3+1).padStart(2,'0')}-01,${columns.map(c=>c[i]).join(',')}`)].join('\n')
    const projectName=`R ARDL ${mode}`
    await page.goto('/app')
    await page.getByRole('textbox',{name:'Project name'}).fill(projectName)
    await page.getByRole('button',{name:'Create project'}).click()
    await page.locator('input[type=file]').setInputFiles({name:'uk-earnings.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
    await page.getByRole('button',{name:'Inspect data'}).click()
    await expect(page.getByText('Choose the observation structure')).toBeVisible({timeout:60_000})
    await prepare(page,{structure:'time series',time:'date',frequency:'Quarterly',columns:names})
    if(info.project.name==='mobile-chromium')await page.getByRole('button',{name:'Expand chapter list'}).click()
    await page.getByRole('navigation',{name:'Workspace chapters'}).getByRole('button',{name:/Time-series analysis/}).click()
    await page.getByRole('radio',{name:'ARDL',exact:true}).click()
    await choose(page,'Outcome series','w')
    const label={rHorizontal:'Horizontal search',rGrid:'Constrained grid',rFixed:'Specify lags (R ARDL)'}[mode]
    await page.getByRole('radio',{name:label,exact:true}).click()
    await page.getByRole('spinbutton',{name:mode==='rFixed'?'Outcome lag':'Maximum outcome lag',exact:true}).fill(String(mode==='rFixed'?input.order[0]:6))
    for(const [i,name] of names.slice(1,5).entries()) {
      await page.getByRole('checkbox',{name,exact:true}).check()
      await page.getByRole('spinbutton',{name:`Lag for ${name}`,exact:true}).fill(String(mode==='rFixed'?input.order[i+1]:6))
    }
    for(const name of names.slice(5)) {
      await page.getByRole('checkbox',{name,exact:true}).check()
      await choose(page,`Role for ${name}`,'Fixed regressor')
    }
    if(mode!=='rFixed')await page.getByRole('spinbutton',{name:'Fixed lag for Prod',exact:true}).fill('1')
    if(mode==='rHorizontal')for(const name of names.slice(0,5))await page.getByRole('spinbutton',{name:`Starting lag for ${name}`,exact:true}).fill('5')
    await page.getByRole('spinbutton',{name:'Initial observations to exclude'}).fill(String(input.hold_back))
    await page.getByRole('button',{name:'Fit ARDL',exact:true}).click()
    const result=page.getByRole('region',{name:'Error-correction analysis'})
    await expect(result).toBeVisible({timeout:60_000})
    await result.getByText('Model diagnostics',{exact:true}).click()
    await expect(result.getByRole('table',{name:'Serial-correlation tests'}).getByRole('row')).toHaveCount(6)
    const saved=()=>page.evaluate(async name=>{
      const store=await import(new URL('/src/data/projectStore.ts',location.href).href)
      const project=(await store.listProjects()).find((p:{name:string})=>p.name===name)
      if(!project)return null
      const loaded=await store.loadProject(project.id)
      return loaded.ok?(loaded.value.timeSeriesRuns.at(-1)??null):null
    },projectName)
    await expect.poll(saved).not.toBeNull()
    const run=(await saved())!
    expect(run.specification.orders.kind).toBe(mode)
    const r=run.evidence.rAnalysis
    close(r.aicPss,input.aic_pss,'selected model PSS AIC')
    if(mode!=='rFixed') {
      const expected=mode==='rHorizontal'?searches.stepwise:searches.grid
      expect(r.ranking.rows.length).toBe(expected.length)
      r.ranking.rows.forEach((row:{order:number[],aicPss:number},i:number)=>{
        expect(row.order).toEqual(names.slice(0,5).map(name=>expected[i][name]))
        close(row.aicPss,expected[i].AIC_pss,`rank ${i+1}`)
      })
      if(mode==='rGrid')expect(r.ranking.evaluated).toBe(2058)
      await expect(result.getByRole('table',{name:'Lag-search ranking'}).getByRole('row')).toHaveCount(expected.length+1)
    } else {
      expect(run.evidence.predictorLags).toEqual([1,0,5,0])
      r.params.forEach((v:number,i:number)=>close(v,input.params[i],`zero-lag UI coefficient ${i}`))
    }
    expect(await page.locator('body').evaluate(el=>el.scrollWidth<=innerWidth+1)).toBe(true)
    await result.scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath(`${mode}.png`)})
    await page.reload()
    await page.getByRole('button',{name:`Open ${projectName}`,exact:true}).click()
    await expect(page.getByRole('heading',{name:'Choose the data file again',exact:true})).toBeVisible()
    await page.locator('input[type=file]').setInputFiles({name:'uk-earnings.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
    await expect(page.locator('#data-profile-title')).toBeVisible({timeout:30_000})
    if(info.project.name==='mobile-chromium')await page.getByRole('button',{name:'Expand chapter list'}).click()
    await page.getByRole('navigation',{name:'Workspace chapters'}).getByRole('button',{name:/Time-series analysis/}).click()
    await page.getByRole('radio',{name:'ARDL',exact:true}).click()
    await expect(page.getByRole('region',{name:'Error-correction analysis'})).toBeVisible({timeout:30_000})
    const reloaded=await saved()
    expect(reloaded.evidence.rAnalysis).toEqual(r)
  })
}
