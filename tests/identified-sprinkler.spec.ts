import {test, expect} from '@playwright/test'
import {readFileSync} from 'node:fs'
import {choose} from './examples/support'

const oracle=JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/identified_sprinkler_bnlearn.json','utf8'))

test('sprinkler condition endpoints, budgets and explicit indices use the oracle-backed evaluator',async({page},info)=>{
  test.setTimeout(120_000)
  const example=oracle.cases[1]
  const rows=example.patterns.flatMap((r:number[],i:number)=>Array(example.counts[i]).fill(r.join(',')))
  const source={name:'sprinkler-oracle.csv',mimeType:'text/csv',buffer:Buffer.from([oracle.names.join(','),...rows].join('\n'))}
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill('Sprinkler conditional query')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles(source)
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
  await choose(page,'Variable to set','Sprinkler')
  await choose(page,'Variable to read','Wet_Grass')
  await choose(page,'Conditioning variable','Rain')
  await choose(page,'Condition state','Highest')
  await choose(page,'State budget','5')
  await expect(page.getByRole('combobox',{name:'Condition state',exact:true})).toContainText('Highest')
  await page.getByRole('combobox',{name:'Condition state',exact:true}).click()
  await expect(page.getByRole('option',{name:/State [23]/})).toHaveCount(0)
  await page.keyboard.press('Escape')
  const run=page.getByRole('button',{name:'Evaluate intervention',exact:true})
  const records=page.getByRole('list',{name:'Intervention queries'}).locator(':scope > li')
  await run.click()
  await expect(records).toHaveCount(1)
  await expect(records.first()).toContainText('Rain in bin 1')
  await choose(page,'State budget','2')
  await expect(page.getByRole('combobox',{name:'Condition state',exact:true})).toContainText('Highest')
  await run.click()
  await expect(records).toHaveCount(2)
  await choose(page,'Condition state','Lowest')
  await run.click()
  await expect(records).toHaveCount(3)
  await expect(records.first()).toContainText('Rain in bin 0')
  await choose(page,'Condition state','Specify state index')
  await page.getByRole('spinbutton',{name:'Condition state index'}).fill('2')
  await run.click()
  await expect(page.getByText(/condition state 2 is absent for Rain/)).toBeVisible()
  await expect(records).toHaveCount(3)
  await page.getByRole('spinbutton',{name:'Condition state index'}).fill('1')
  await run.click()
  await expect(records).toHaveCount(4)
  await records.first().scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('sprinkler-condition.png')})
  if(await page.getByRole('dialog').isVisible())await page.keyboard.press('Escape')
  const nav=page.getByRole('button',{name:'Expand section list',exact:true})
  if(await nav.isVisible())await nav.click()
  const download=page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path=await(await download).path();if(!path)throw Error('No export')
  const snapshot=JSON.parse(readFileSync(path,'utf8')).project
  expect(snapshot.interventionQueries).toHaveLength(4)
  for(const record of snapshot.interventionQueries){
    const evidence=record.route.result
    expect(evidence.result.expression).not.toContain('Cloudy')
    const rain=Number(evidence.query.state)
    for(const [s,key] of ['distributionLow','distributionHigh'].entries()){
      const actual=evidence.result[key].find((row:[string,number])=>row[0]==='1')[1]
      expect(Math.abs(actual-example.wet_probabilities[2*rain+s])).toBeLessThan(1e-12)
    }
  }
  expect(await page.evaluate(async snapshot=>{
    const {parseSnapshotValue}=await import(new URL('/src/domain/persistence.ts',location.href).href)
    return parseSnapshotValue(snapshot).ok
  },snapshot)).toBe(true)
})
