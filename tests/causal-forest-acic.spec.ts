import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose, identifyEffect } from './examples/support'

// Generate with oracle/grf/acic-ui.R in the pinned hirmos-grf:2.6.1 container.
const workflow = process.env.HIRMOS_ACIC_WORKFLOW === '1'
test(`ACIC ${workflow ? 'selected-variable workflow' : 'all-covariate fit'} matches pinned R`, async ({ page }, info) => {
  test.skip(!process.env.HIRMOS_ACIC_FIXTURE, 'Requires the explicitly generated ACIC oracle.')
  test.setTimeout(workflow ? 1_800_000 : 600_000)
  page.setDefaultTimeout(30_000)
  const base = process.env.HIRMOS_ACIC_FIXTURE!
  const reference = JSON.parse(readFileSync(`${base}/${workflow ? 'acic-workflow' : 'acic-ui'}.json`, 'utf8'))
  const project = `ACIC school clusters ${info.project.name}`
  await page.emulateMedia({ reducedMotion: 'reduce' })
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(project)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles(`${base}/acic-ui.csv`)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  await page.getByRole('button', { name: 'Select all columns' }).click()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(project)
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(`dag { W -> Y ${reference.features.map((name: string) => `${name} -> W ${name} -> Y`).join(' ')} }`)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', project)
  await choose(page, 'Treatment', 'W')
  await choose(page, 'Outcome', 'Y')
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('Assume treatment is unconfounded given the ten baseline covariates. School is the sampling cluster, not an adjustment variable. Each school receives equal weight.')
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Causal forest/ }).click()
  await page.getByRole('spinbutton', { name: 'Trees', exact: true }).fill(String(reference.trees))
  await page.getByRole('spinbutton', { name: 'Forest seed', exact: true }).fill(String(reference.seed))
  if (workflow) {
    await page.getByRole('radio', { name: 'Automatic tuning', exact: true }).click()
    await page.locator('summary').filter({ hasText: 'Importance-selected refit' }).click()
    await page.getByRole('radio', { name: 'Select and refit', exact: true }).click()
    for (const [label, key] of [['Nuisance trees','nuisanceTrees'],['Initial forest trees','initialTrees'],['Outcome forest seed','outcomeSeed'],['Treatment forest seed','treatmentSeed'],['Initial forest seed','initialSeed']])
      await page.getByRole('spinbutton', {name:label!,exact:true}).fill(String(reference.refit[key!]))
  }
  await page.locator('summary').filter({ hasText: 'Sampling and aggregate inference' }).click()
  await page.getByRole('radio', { name: 'Equal cluster weights', exact: true }).click()
  await choose(page, 'Cluster identifier', 'schoolid')
  await page.locator('summary').filter({ hasText: 'Best linear projection' }).click()
  await page.getByRole('radio', { name: 'Estimate projection', exact: true }).click()
  for (const name of ['X1', 'X2']) await page.getByRole('group', { name: 'Projection covariates' }).getByRole('checkbox', { name, exact: true }).check()
  if (workflow) {
    await page.locator('summary').filter({ hasText: 'Cluster-score moderation' }).click()
    await page.getByRole('radio', {name:'Compare clusters',exact:true}).click()
    for (const name of ['X1','X2']) await page.getByRole('group',{name:'Between-cluster characteristics'}).getByRole('checkbox',{name,exact:true}).check()
    await choose(page,'Within-cluster characteristic','S3')
    await page.getByRole('spinbutton',{name:'Within-cluster threshold'}).fill('6')
  }
  await page.screenshot({ path: info.outputPath('school-cluster-controls.png'), animations: 'disabled' })
  let reloaded = false
  page.on('framenavigated', frame => { if (frame === page.mainFrame()) reloaded = true })
  await page.getByRole('button', { name: /^Run causal forest/ }).click()
  const saved = () => page.evaluate(async name => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await store.listProjects()).find((item: { name: string }) => item.name === name)
    const loaded = header === undefined ? null : await store.loadProject(header.id)
    return loaded?.ok ? loaded.value : null
  }, project)
  await expect.poll(async () => reloaded ? 'Page reloaded before saving the forest' : (await saved())?.estimationRuns.length ?? 0, { timeout: workflow ? 1_500_000 : 300_000 }).toBe(1)
  const snapshot = await saved()
  const run = snapshot.estimationRuns[0]
  await info.attach('saved-forest-run', {body:Buffer.from(JSON.stringify(run)),contentType:'application/json'})
  expect(run.evidence.features.map((feature: {name:string}) => feature.name)).toEqual(reference.features)
  expect(run.evidence.analysis.specification.sampling.kind).toBe('equal-clusters')
  expect(run.evidence.observations).toBe(reference.observations)
  if (workflow) {
    expect(run.evidence.refit.selected).toEqual(reference.selected)
    for (const key of ['outcomePredictions','treatmentPredictions','initialImportance']) {
      expect(run.evidence.refit[key]).toHaveLength(reference[key].length)
      for (let i=0;i<reference[key].length;i++) expect(run.evidence.refit[key][i]).toBeCloseTo(reference[key][i],7)
    }
    expect(run.evidence.analysis.moderation).toHaveLength(reference.moderation.length)
    for (let i=0;i<reference.moderation.length;i++) {
      expect(run.evidence.analysis.moderation[i].result.kind).toBe('estimated')
      for (const [key,value] of Object.entries(reference.moderation[i])) expect(run.evidence.analysis.moderation[i].result.result[key]).toBeCloseTo(value as number,7)
    }
    expect(run.evidence.calibration.kind).toBe('estimated')
    for (const [index, term] of ['mean','differential'].entries())
      for (const [j, key] of ['estimate','standardError','statistic','pValue'].entries())
        expect(run.evidence.calibration[term][key]).toBeCloseTo(reference.calibration[index][j],7)
  }
  expect(run.evidence.summary.estimate).toBeCloseTo(reference.ate[0], 7)
  expect(run.evidence.summary.standardError).toBeCloseTo(reference.ate[1], 7)
  expect(run.evidence.predictions).toHaveLength(reference.predictions.length)
  for (let i = 0; i < reference.predictions.length; i++) {
    expect(run.evidence.predictions[i].kind).toBe('estimated')
    expect(run.evidence.predictions[i].estimate).toBeCloseTo(reference.predictions[i], 7)
  }
  const projection = run.evidence.analysis.projection
  expect(projection.kind).toBe('estimated')
  for (let i = 0; i < reference.projection.length; i++) {
    expect(projection.result.estimates[i]).toBeCloseTo(reference.projection[i][0], 7)
    expect(projection.result.standardErrors[i]).toBeCloseTo(reference.projection[i][1], 7)
  }
  const results = page.getByTestId('causal-forest-results').first()
  await results.getByTestId('causal-forest-aggregate').scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('school-cluster-results.png'), animations: 'disabled' })
  await results.getByText('Effect projection', { exact: true }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('school-cluster-projection.png'), animations: 'disabled' })
  if (workflow) {
    const importance = results.getByRole('table',{name:'Initial forest importance'})
    await importance.scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath('named-initial-importance.png'),animations:'disabled'})
    const finalImportance = results.getByRole('table',{name:'Final forest importance'})
    for (const index of reference.selected) await expect(finalImportance.getByRole('cell',{name:reference.features[index],exact:true})).toBeVisible()
    await finalImportance.scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath('named-final-importance.png'),animations:'disabled'})
    await results.getByText('Cluster-score comparisons',{exact:true}).scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath('school-moderation.png'),animations:'disabled'})
  }
  await info.attach('oracle-comparison', { contentType: 'application/json', body: Buffer.from(JSON.stringify({
    reference: reference.reference,
    observations: reference.observations, schools: reference.schools,
    ate: run.evidence.summary.estimate, standardError: run.evidence.summary.standardError,
    ateDifference: Math.abs(run.evidence.summary.estimate - reference.ate[0]),
    standardErrorDifference: Math.abs(run.evidence.summary.standardError - reference.ate[1]),
    maxPredictionDifference: Math.max(...run.evidence.predictions.map((p: { estimate: number }, i: number) => Math.abs(p.estimate - reference.predictions[i]))),
    projection: projection.result,
  }, null, 2)) })
  await page.reload()
  expect((await saved()).estimationRuns[0]).toEqual(run)
})
