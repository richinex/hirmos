import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { addArrow, chapter, choose, createDag, identifyEffect } from './examples/support'

test('DiD covariate identities use values, not names or correlations', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const { didCovariateRestrictions } = await load('/src/domain/adjustedDid.ts')
    const { describeEstimand } = await load('/src/domain/study.ts')
    const units = ['a','a','b','b','c','c','d','d']
    const d = [0,0,0,0,0,1,0,1]
    const group = [0,0,0,0,1,1,1,1]
    const matrix = { rowCount: 8, units, periodCodes: [0,1,0,1,0,1,0,1], covariates: ['alias','recoded','baseline','nsw'], values: new Float64Array([
      ...units.map((_,i) => i), ...d, ...d, ...group.map(v => 5-3*v), 0,2,0,3,1,4,1,5, 0,0,1,1,0,0,1,1,
    ]) }
    const layout = { treated: ['c','d'], periods: [{code:0,label:'1975'},{code:1,label:'1978'}] }
    const shuffled = [7,3,0,6,1,4,2,5]
    const reordered = { ...matrix, units: shuffled.map(i => units[i]), periodCodes: shuffled.map(i => matrix.periodCodes[i]), values: new Float64Array(Array.from({length:6}, (_,c) => shuffled.map(i => matrix.values[8*c+i])).flat()) }
    const dr = {kind:'doublyRobust'}
    return {
      dr: didCovariateRestrictions(matrix,layout,dr),
      reordered: didCovariateRestrictions(reordered,layout,dr),
      regression: didCovariateRestrictions(matrix,layout,{kind:'regression'}),
      description: describeEstimand({ estimand:{kind:'average-treatment-effect'}, treatment:{name:'treatment'},outcome:{name:'outcome'},population:{observations:32578} }),
    }
  })
  expect(result.dr).toEqual([{column:'alias',role:'treatment-indicator'},{column:'recoded',role:'group-indicator'},{column:'baseline',role:'baseline-group-indicator'}])
  expect(result.reordered).toEqual(result.dr)
  expect(result.regression).toEqual(result.dr.slice(0,2))
  expect(result.description).toContain('32,578 prepared rows')
})

test('ATT preview, two-period default, aliases and DR result through the UI', async ({ page }, info) => {
  test.setTimeout(240_000)
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  const lines = readFileSync('tests/fixtures/chapter-did-covariates.csv','utf8').trim().split('\n')
  const csv = ['id,time_points,D,Y,treatment,A,nsw,sample', ...lines.slice(1).map(line => `${line},${line.split(',')[2]},${1-Number(line.split(',')[2])}`)].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox',{name:'Project name'}).fill('DiD presentation regression')
  await page.getByRole('button',{name:'Create project',exact:true}).click()
  await page.locator('input[type=file]').setInputFiles({name:'did-aliases.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
  await page.getByRole('button',{name:'Inspect data',exact:true}).click()
  await page.getByRole('radio',{name:/^Panel/}).click()
  await choose(page,'Unit column','id')
  await choose(page,'Time column','time_points')
  for (const name of ['Y','treatment','A','nsw','sample']) await page.getByRole('checkbox',{name,exact:true}).check()
  await page.getByRole('button',{name:/Create prepared/}).click()
  await createDag(page,'DiD graph')
  await addArrow(page,'treatment','Y','Treatment can change the outcome after baseline.')
  await chapter(page,/Study design/)
  await choose(page,'Causal graph','DiD graph')
  await choose(page,'Treatment','treatment')
  await choose(page,'Outcome','Y')
  await page.getByRole('radio',{name:/Treated rows/}).click()
  const preview = page.locator('dl[aria-label="Estimand and population"]')
  await expect(preview).toContainText('Average treatment effect on the treated')
  await expect(preview).not.toContainText('averaged over all')
  await page.getByRole('radio',{name:/Policy change/}).click()
  await expect(preview).toContainText('Average treatment effect on the treated')
  await page.getByRole('textbox',{name:'Assignment sentence'}).fill('Compare treated and control units before and after adoption.')
  await identifyEffect(page)
  await chapter(page,/Estimation/)
  await page.getByRole('radio',{name:/^Interventions/}).click()
  await page.getByRole('radio',{name:/Panel difference-in-differences/}).click()
  await expect(page.getByRole('radio',{name:'Conventional',exact:true})).toBeChecked()
  await expect(page.getByRole('button',{name:'Run panel DiD',exact:true})).toBeEnabled()
  await page.getByRole('radio',{name:'Two-period DR',exact:true}).click()
  const choices = page.getByRole('group',{name:'DiD covariates',exact:true})
  await expect(choices.getByRole('checkbox',{name:'nsw',exact:true})).toBeDisabled({timeout:60_000})
  await expect(choices.getByRole('checkbox',{name:'sample',exact:true})).toBeDisabled()
  await expect(choices).toContainText('Identifies the treated and control groups exactly')
  await page.getByRole('button',{name:'Select all DiD covariates',exact:true}).click()
  await expect(choices.getByRole('checkbox',{name:'A',exact:true})).toBeChecked()
  await expect(choices.getByRole('checkbox',{name:'nsw',exact:true})).not.toBeChecked()
  await choices.scrollIntoViewIfNeeded()
  await page.screenshot({path:info.outputPath('did-covariates.png')})
  await page.getByRole('spinbutton',{name:'DiD folds',exact:true}).fill('5')
  await page.getByRole('button',{name:'Run panel DiD',exact:true}).click()
  await expect(page.getByText('Current estimate',{exact:true}).first()).toBeVisible({timeout:60_000})
  await expect(page.getByText('Baseline-covariate adjustment',{exact:true}).first()).toBeVisible()
  await expect(page.getByText('Unit and time weights',{exact:true})).toHaveCount(0)
  const open = page.getByRole('button',{name:'Expand section list',exact:true})
  if (await open.isVisible()) await open.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button',{name:'Export project',exact:true}).click()
  const path = await (await download).path()
  if (path === null) throw Error('No project export')
  const snapshot = JSON.parse(readFileSync(path,'utf8')).project
  const run = snapshot.estimationRuns[0]
  expect(run.estimate.estimand.kind).toBe('average-treatment-effect-on-treated')
  expect(run.evidence.estimate).toBeCloseTo(314.7960332262636,6)
  expect(run.configuration.covariates).toHaveLength(1)

  // Historical presentation fixture only. New fits reject unfinished optimizers.
  await page.evaluate(async ({run,study}) => {
    const load = (path:string):Promise<any> => import(/* @vite-ignore */ path)
    const [{React,createRoot},{ResultCard}] = await Promise.all([load('/tests/support/reactRuntime.ts'),load('/src/components/estimation/EstimationPanel.tsx')])
    const fixture = structuredClone(run)
    delete fixture.evidence.inference.propensityFit
    fixture.evidence.estimate = fixture.estimate.effect.value = 8.1
    fixture.evidence.standardError = fixture.estimate.standardError = 200
    fixture.evidence.interval = [-383.9,400.1]
    fixture.estimate.interval.lower = -383.9
    fixture.estimate.interval.upper = 400.1
    fixture.evidence.inference.optimizerStatus[0] = 'iteration-limit'
    const host = document.createElement('section')
    host.setAttribute('aria-label','Result presentation fixture')
    host.style.cssText = 'position:fixed;inset:0;overflow:auto;z-index:9999;padding:24px;background:var(--color-panel)'
    document.body.append(host)
    createRoot(host).render(React.createElement(ResultCard,{run:fixture,study,current:true,stepLabel:'panel row'}))
  },{run,study:snapshot.studies.find((study:{id:string})=>study.id===run.study)})
  const result = page.getByRole('region',{name:'Result presentation fixture'})
  await expect(result.getByTestId('effect-estimate')).toContainText('8.10')
  await expect(result).toContainText('8.10 higher after adoption')
  const warning = result.getByText('At least one propensity fit did not converge.',{exact:false})
  await expect(warning).toHaveCount(1)
  const warningBox = await warning.boundingBox()
  const headlineBox = await result.getByTestId('effect-estimate').boundingBox()
  expect(warningBox!.y).toBeLessThan(headlineBox!.y)
  await page.screenshot({path:info.outputPath('did-result-light.png')})
  await page.evaluate(()=>document.documentElement.setAttribute('data-theme','dark'))
  await page.screenshot({path:info.outputPath('did-result-dark.png')})
  expect(errors).toEqual([])
})
