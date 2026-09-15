import { expect, test } from '@playwright/test'
import { readFileSync, writeFileSync } from 'node:fs'

test('401k intervention effects reproduce the reference through the Run button', async ({ page }, info) => {
  test.setTimeout(1_200_000)
  const model = JSON.parse(readFileSync('docs/2026-09-15-gcm-401k/data/model.json', 'utf8')) as { names: string[]; edges: number[][]; rows: number[][] }
  const oracle = JSON.parse(readFileSync('docs/2026-09-15-gcm-401k/reference-results.json', 'utf8'))
  await page.routeWebSocket(/.*/, socket => socket.close())
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('401k GCM reference')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: '401k-model.csv', mimeType: 'text/csv', buffer: Buffer.from([model.names.join(','), ...model.rows.map(row => row.join(','))].join('\n')) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of model.names) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  const navigation = page.getByRole('navigation', { name: 'Workspace chapters' })
  await navigation.getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('401k eligibility and financial assets')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  for (const [cause, effect] of model.edges) {
    for (const [label, name] of [['Proposed cause', model.names[cause]], ['Proposed effect', model.names[effect]]]) {
      await page.getByRole('combobox', { name: label }).click()
      await page.getByRole('option', { name, exact: true }).click()
    }
    await page.getByRole('textbox', { name: /Rationale/ }).first().fill('Relationship specified in the preserved DoWhy v0.14 401(k) example.')
    await page.getByRole('button', { name: 'Add the arrow' }).click()
  }
  await page.getByRole('button', { name: 'Use for root-cause analysis', exact: true }).click()
  await page.getByRole('radio', { name: 'Intervention effects', exact: true }).check()
  await page.getByRole('combobox', { name: 'Group effects by', exact: true }).click()
  await page.getByRole('option', { name: 'inc', exact: true }).click()
  await page.getByRole('radio', { name: 'Lower bound', exact: true }).check()
  await page.getByLabel('Minimum group value', { exact: true }).fill('0')
  await page.getByRole('region', { name: 'Intervention effects setup' }).getByRole('button', { name: 'Run analysis', exact: true }).click()
  const result = page.getByRole('region', { name: 'Intervention effect results' })
  await expect(result).toBeVisible({ timeout: 180000 })
  const saved = async () => page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const headers = await store.listProjects()
    const project = await store.loadProject(headers[0].id)
    return project.ok ? project.value.rootCause.effects.at(-1) : null
  })
  await expect.poll(async () => Boolean(await saved())).toBe(true)
  const run = await saved()
  writeFileSync(info.outputPath('gcm-ui-run.json'), JSON.stringify(run, null, 2))
  await info.attach('ui-run.json', { body: JSON.stringify(run, null, 2), contentType: 'application/json' })
  expect(run.model.names).toEqual(model.names)
  const labels = ['ate', ...oracle.groups]
  let maximumReplicateDifference = 0
  for (let r = 0; r < 100; r++) for (let c = 0; c < labels.length; c++) {
    const difference = Math.abs(run.evidence.replicates[r][c] - oracle.replicates[r][labels[c]])
    maximumReplicateDifference = Math.max(maximumReplicateDifference, difference)
    expect(difference, 'replicate ' + r + ', ' + labels[c]).toBeLessThanOrEqual(1e-8)
  }
  for (let c = 0; c < labels.length; c++) {
    expect(Math.abs(run.evidence.estimates[c] - oracle.summary[labels[c]])).toBeLessThanOrEqual(0.25)
    for (let k = 0; k < 2; k++) expect(Math.abs(run.evidence.bounds[c][k] - oracle.intervals[labels[c]][k])).toBeLessThanOrEqual(1e-8)
  }
  expect(run.evidence.optimizer.status).toBe('precisionLoss')
  expect(run.evidence.grouping.excluded).toBe(2)
  await info.attach('comparison.json', { body: JSON.stringify({ maximumReplicateDifference }), contentType: 'application/json' })
  await result.scrollIntoViewIfNeeded()
  await result.screenshot({ path: info.outputPath('401k-result-desktop.png') })
  await page.setViewportSize({ width: 390, height: 844 })
  await result.scrollIntoViewIfNeeded()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('401k-result-mobile.png'), fullPage: true })
  expect(errors).toEqual([])

  await page.setViewportSize({ width: 1440, height: 1000 })
  const choose = async (label: string, name: string) => {
    await page.getByRole('combobox', { name: label, exact: true }).click()
    await page.getByRole('option', { name, exact: true }).click()
  }
  await navigation.getByRole('button', { name: /Study design/ }).click()
  await choose('Causal graph', '401k eligibility and financial assets')
  await choose('Treatment', 'e401')
  await choose('Outcome', 'net_tfa')
  await page.getByRole('radio', { name: /Each row, given its covariates/ }).check()
  for (const name of model.names.slice(2)) await page.getByRole('group', { name: 'Effect modifiers' }).getByRole('checkbox', { name: new RegExp('^' + name + '(?:\\s|$)') }).check()
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('Compare eligibility using the same sixteen adjustment variables as the GCM reference.')
  await page.getByRole('button', { name: 'Identify the effect' }).click()
  await navigation.getByRole('button', { name: /Estimation/ }).click()
  await page.getByRole('radio', { name: 'Bootstrap intervals', exact: true }).check()
  await page.getByLabel('Learner seed', { exact: true }).fill('7')
  await page.getByLabel('Bootstrap samples', { exact: true }).fill('100')
  await page.getByRole('button', { name: /^Run T-learner/ }).click()
  await expect(page.getByText('Current estimate', { exact: true })).toBeVisible({ timeout: 900000 })
  const savedLearner = async () => page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const project = await store.loadProject((await store.listProjects())[0].id)
    return project.ok ? project.value.estimationRuns.at(-1) : null
  })
  await expect.poll(async () => Boolean(await savedLearner())).toBe(true)
  const learner = await savedLearner()
  writeFileSync(info.outputPath('tlearner-ui-run.json'), JSON.stringify(learner, null, 2))
  const reference = JSON.parse(readFileSync('docs/2026-09-15-gcm-401k/tlearner-results.json', 'utf8'))
  const evidence = learner.evidence
  expect(evidence.effects).toHaveLength(model.rows.length)
  for (let row = 0; row < model.rows.length; row++) {
    expect(Math.abs(evidence.effects[row] - reference.effects[row]), `effect row ${row}`).toBeLessThanOrEqual(1e-8)
    expect(Math.abs(evidence.uncertainty.standardErrors[row] - reference.standard_errors[row]), `SE row ${row}`).toBeLessThanOrEqual(1e-8)
    for (let k = 0; k < 2; k++) expect(Math.abs(evidence.uncertainty.intervals[row][k] - reference.intervals[row][k]), `interval row ${row}`).toBeLessThanOrEqual(1e-8)
  }
  for (let k = 0; k < 2; k++) expect(Math.abs(evidence.uncertainty.average.interval[k] - reference.summaries[0].interval[k])).toBeLessThanOrEqual(1e-8)
  await page.screenshot({ path: info.outputPath('tlearner-401k-desktop.png'), fullPage: true })
  expect(errors).toEqual([])
})
