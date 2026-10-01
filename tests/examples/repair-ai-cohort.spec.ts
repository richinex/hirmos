import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { chapter, identify } from './support'

test.skip(!process.env.REPAIR_COHORT_EXAMPLE,'Explicit opt-in to export a repaired example candidate')
test('repair cohort target with new records and verify both advertised methods',async({page},info)=>{
  test.setTimeout(240_000)
  const old=JSON.parse(await readFile('public/examples/ai-adoption-cohort.hirmos.json','utf8')).project
  await page.goto('/app')
  await page.getByRole('button',{name:'Open AI adoption, March cohort',exact:true}).click()
  await identify(page,{graph:'March cohort adoption',treatment:'adopted',outcome:'bugs_per_kloc',target:/Treated rows/,mechanism:'Policy change',sentence:'Teams A, B and C switched the AI assistant on in month 13; teams F, G and H never did.'})
  await chapter(page,/Estimation/)
  await page.getByRole('combobox',{name:'Identified study'}).click()
  await page.getByRole('option',{name:/among treated rows/}).click()
  await page.getByRole('radio',{name:/^Interventions/}).click()
  await page.getByRole('radio',{name:/Panel difference-in-differences/}).click()
  let count=old.estimationRuns.length
  for(const method of ['Synthetic','Conventional']) {
    await page.getByRole('radio',{name:method,exact:true}).click()
    const run=page.getByRole('button',{name:/^Run panel difference-in-differences/}).first()
    await expect(run).toBeEnabled({timeout:60_000})
    await run.click()
    count++
    await expect(page.getByRole('heading',{name:`Runs (${count})`,exact:true})).toBeVisible({timeout:120_000})
    await page.screenshot({path:info.outputPath(`cohort-${method.toLowerCase()}.png`)})
  }
  await chapter(page,/Data studio/)
  const panel=page.locator('details',{has:page.getByText(/Source file\s+storage and export/)})
  if(!await panel.evaluate(e=>(e as HTMLDetailsElement).open)) await panel.locator('summary').click()
  await page.getByRole('checkbox',{name:/Include the source file/}).check()
  const download=page.waitForEvent('download')
  await page.locator('main').getByRole('button',{name:/Export project/}).click()
  const saved=await download
  const path=await saved.path()
  if(path===null) throw Error('Missing example export')
  const fresh=JSON.parse(await readFile(path,'utf8')).project
  for(let i=0;i<old.estimationRuns.length;i++) {
    expect(fresh.estimationRuns[i].estimate).toEqual(old.estimationRuns[i].estimate)
    expect(fresh.estimationRuns[i].configuration).toEqual(old.estimationRuns[i].configuration)
    expect(fresh.estimationRuns[i].study).toEqual(old.estimationRuns[i].study)
  }
  expect(fresh.estimationRuns.at(-1).configuration.primary).toBe('did')
  expect(fresh.estimationRuns.at(-1).estimate.estimand.kind).toBe('average-treatment-effect-on-treated')
  const synthetic=fresh.estimationRuns.at(-2)
  // The serializer omits the default synthetic primary in this legacy-compatible contract.
  expect(synthetic.configuration.primary ?? 'syntheticDid').toBe('syntheticDid')
  expect(synthetic.estimate.estimand.kind).toBe('average-treatment-effect-on-treated')
  expect(synthetic.estimate.effect.value).toBeCloseTo(old.estimationRuns[0].estimate.effect.value,9)
  await saved.saveAs(info.outputPath('ai-adoption-cohort.hirmos.json'))
})
