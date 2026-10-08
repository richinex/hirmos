import { expect, test } from '@playwright/test'
import { chapter, choose, prepare } from './examples/support'

for(const membership of ['numeric','categories'] as const)test('surrogate '+membership+' workflow saves and restores a two-sample estimate', async ({ page }, info) => {
  test.setTimeout(90000)
  const failures: string[] = []
  page.on('pageerror', error => failures.push(error.message))
  const csv = 'sample,treatment,outcome,surrogate\n1,0,,0\n0,,1,0\n1,1,,1\n0,,3,1\n1,0,,2\n0,,5,2\n1,1,,3\n0,,7,3\n0,,9,4\n'
  const sourceCsv=membership==='numeric'?csv:csv.split('\n').map((row,i)=>i===0?row:row.startsWith('1,')?row.replace('1,','Riverside,'):row.replace('0,',i%3===0?'Alameda,':i%3===1?"O’Brien,":'Los Angeles,')).join('\n')
  const name = 'Surrogate UI acceptance'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name', exact: true }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'surrogate-samples.csv', mimeType: 'text/csv', buffer: Buffer.from(sourceCsv) })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 60000 })
  await prepare(page, { structure: 'cross-section', columns: ['surrogate'] })
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Two-sample surrogates', exact: true }).check()
  await choose(page, 'Sample membership', 'sample')
  if(membership==='categories'){
    const experiment=page.getByRole('group',{name:'Experimental sample categories',exact:true})
    const observation=page.getByRole('group',{name:'Observational sample categories',exact:true})
    await experiment.getByRole('checkbox',{name:'Riverside',exact:true}).check()
    await expect(observation.getByRole('checkbox',{name:'Riverside',exact:true})).toBeDisabled()
    await expect(page.getByRole('textbox',{name:'Search sample categories'})).toHaveCount(0)
    for(const site of ['Alameda','O’Brien','Los Angeles'])await observation.getByRole('checkbox',{name:site,exact:true}).check()
    for(const site of ['Alameda','O’Brien','Los Angeles'])await expect(observation.getByRole('checkbox',{name:site,exact:true})).toBeChecked()
    await expect(experiment.getByRole('checkbox',{name:'Alameda',exact:true})).toBeDisabled()
  }
  await choose(page, 'Treatment in experimental sample', 'treatment')
  await choose(page, 'Long-term outcome in observational sample', 'outcome')
  await page.getByRole('checkbox', { name: 'surrogate', exact: true }).first().check()
  await page.getByRole('textbox', { name: 'Identifying-assumption rationale' }).fill('Synthetic acceptance example: treatment is assigned independently and the same outcome rule Y = 1 + 2S holds in both samples. Surrogacy and comparability hold by construction; both treatment arms and overlapping surrogate values are observed.')
  await page.getByRole('combobox', {name:'Sample membership',exact:true}).scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('surrogate-controls.png')})
  await page.getByRole('button', { name: 'Estimate long-term effect', exact: true }).click()
  const estimate=page.getByTestId('surrogate-estimate').first()
  await expect(estimate).toContainText('2.00', { timeout: 30000 })
  await estimate.scrollIntoViewIfNeeded()
  for (const theme of ['light', 'dark']) {
    await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
    await page.screenshot({ path: info.outputPath(`surrogate-${theme}.png`) })
  }
  await chapter(page, /^Results/)
  await expect(page.getByTestId('surrogate-estimate').first()).toContainText('2.00')
  await expect.poll(() => page.evaluate(async name => {
    const store = await import(new URL('/src/data/projectStore.ts',location.href).href)
    const header = (await store.listProjects()).find((p: {name:string}) => p.name === name)
    if (!header) return false
    const saved = await store.loadProject(header.id)
    return saved.ok && saved.value.surrogateRuns.length === 1 && header.estimationRuns === 1
  }, name)).toBe(true)
  await page.goto('/app/projects')
  await page.getByRole('button', {name: `Open ${name}`, exact: true}).click()
  await expect(page.getByRole('heading', {name:'Choose the data file again',exact:true})).toBeVisible()
  await page.locator('input[type=file]').setInputFiles({name:'surrogate-samples.csv',mimeType:'text/csv',buffer:Buffer.from(sourceCsv)})
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Two-sample surrogates', exact: true }).check()
  await expect(page.getByTestId('surrogate-estimate').first()).toContainText('2.00')
  if(membership==='numeric')await page.getByRole('textbox',{name:'Experimental sample value',exact:true}).fill('99')
  else await page.getByRole('group',{name:'Experimental sample categories',exact:true}).getByRole('checkbox',{name:'Riverside',exact:true}).uncheck()
  await page.getByText('Specification and recorded assumptions',{exact:true}).first().click()
  await page.getByRole('button', { name: 'Restore this specification', exact: true }).first().click()
  if(membership==='numeric')await expect(page.getByRole('textbox',{name:'Experimental sample value',exact:true})).toHaveValue('1')
  else await expect(page.getByRole('group',{name:'Experimental sample categories',exact:true}).getByRole('checkbox',{name:'Riverside',exact:true})).toBeChecked()
  if(info.project.name==='mobile-chromium')await page.getByRole('button',{name:/^Surrogate runs/}).click()
  await page.getByRole('button', { name: 'Delete this surrogate run', exact: true }).click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Delete run', exact: true }).click()
  if(info.project.name==='mobile-chromium')await page.keyboard.press('Escape')
  await expect(page.getByTestId('surrogate-estimate')).toHaveCount(0)
  expect(failures).toEqual([])
})
