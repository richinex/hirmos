import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose, identifyEffect } from './examples/support'

const fixture = JSON.parse(readFileSync('crates/causal-core/oracle/grf/fixtures/nuisance.json', 'utf8'))

for (const kind of ['binary', 'continuous', 'conditional', 'encoded'] as const) test(`${kind} forest runs through Study and restores its saved evidence`, async ({ page }, info) => {
  test.setTimeout(180_000)
  page.setDefaultTimeout(15_000)
  await page.emulateMedia({ reducedMotion: 'reduce' })
  const source = fixture.cases[kind === 'continuous' ? 1 : 0]
  const names = ['W', 'Y', 'x1', 'x2', 'x3', 'x4']
  const columns = [source.W, source.Y, ...source.X]
  if (kind === 'encoded') columns[4] = source.X[2].map((value: number) => value > 0 ? 2 : 1)
  const csv = [names.join(','), ...source.Y.map((_: unknown, row: number) => columns.map(column => column[row]).join(','))].join('\n')
  const project = `Forest ${kind} ${info.project.name}`
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(project)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'forest.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of names) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(project)
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(`dag { W -> Y ${names.slice(2).map(name => `${name} -> W ${name} -> Y`).join(' ')} }`)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', project)
  await choose(page, 'Treatment', 'W')
  await choose(page, 'Outcome', 'Y')
  const targets = page.getByTestId('study-target-options')
  const targetSize = await targets.evaluate(element => ({ height: element.clientHeight, content: element.scrollHeight }))
  expect(targetSize.content).toBeGreaterThan(targetSize.height)
  expect(targetSize.height).toBeLessThanOrEqual(384)
  if (kind === 'binary') {
    const first = targets.getByRole('radio', { name: /^All prepared rows/ })
    await first.focus()
    await page.keyboard.press('End')
    await expect(targets.getByRole('radio', { name: /^Conditional partial effect/ })).toBeChecked()
    expect(await targets.evaluate(element => element.scrollTop)).toBeGreaterThan(0)
    await page.keyboard.press('Home')
    await expect(first).toBeChecked()
    await targets.scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath('target-options.png'), animations: 'disabled' })
  }
  if (kind === 'continuous') await page.getByRole('radio', { name: /^Average partial effect/ }).check()
  if (kind === 'conditional') {
    await page.getByRole('radio', { name: /Each row, given its covariates/ }).check()
    await page.getByRole('group', { name: 'Effect modifiers' }).getByRole('checkbox', { name: /^x1/ }).check()
  }
  await page.getByRole('radio', { name: /Observed choice/ }).check()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill('The simulated treatment is adjusted for x1 to x4. The continuous design uses the random-coefficient treatment model.')
  await page.screenshot({ path: info.outputPath('study.png') })
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Causal forest/ }).click()
  await page.getByRole('spinbutton', { name: 'Trees', exact: true }).fill('100')
  await page.getByRole('spinbutton', { name: 'Forest seed', exact: true }).fill(String(source.seed))
  if (kind === 'encoded') {
    await page.getByRole('group', { name: 'Categorical covariates' }).getByRole('checkbox', { name: 'x3', exact: true }).check()
    await page.locator('summary').filter({ hasText: 'Importance-selected refit' }).click()
    await page.getByRole('radio', { name: 'Select and refit', exact: true }).click()
    await page.getByRole('spinbutton', { name: 'Nuisance trees', exact: true }).fill('100')
    await page.getByRole('spinbutton', { name: 'Initial forest trees', exact: true }).fill('100')
  }
  await page.screenshot({ path: info.outputPath('controls.png') })
  await page.getByRole('button', { name: /^Run causal forest/ }).click()
  const canvas = page.getByTestId('causal-forest-results').first()
  await expect(canvas).toBeVisible({ timeout: 30_000 })
  const saved = () => page.evaluate(async name => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await store.listProjects()).find((item: { name: string }) => item.name === name)
    const loaded = header === undefined ? null : await store.loadProject(header.id)
    return loaded?.ok ? loaded.value : null
  }, project)
  await expect.poll(async () => (await saved())?.estimationRuns.length ?? 0).toBe(1)
  const snapshot = await saved()
  const run = snapshot.estimationRuns[0]
  expect(run.kind).toBe('causal-forest-run')
  expect(run.evidence.features).toHaveLength(run.evidence.variableImportance.length)
  if (kind === 'encoded') {
    expect(run.evidence.features.filter((f: {kind:string}) => f.kind === 'indicator').map((f: {name:string;level:number}) => `${f.name} = ${f.level}`)).toEqual(['x3 = 1', 'x3 = 2'])
    const initial = canvas.getByRole('table', { name: 'Initial forest importance' })
    for (const name of ['x1','x2','x3 = 1','x3 = 2','x4']) await expect(initial.getByRole('cell', {name,exact:true})).toBeVisible()
    const final = canvas.getByRole('table', { name: 'Final forest importance' })
    for (const index of run.evidence.refit.selected) {
      const feature = run.evidence.features[index]
      await expect(final.getByRole('cell', {name:feature.kind==='numeric'?feature.name:`${feature.name} = ${feature.level}`,exact:true})).toBeVisible()
    }
    await initial.scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath('encoded-importance.png'),animations:'disabled'})
  }
  if (kind === 'conditional') {
    expect(run.evidence.summary.kind).toBe('not-requested')
    expect(run.estimate.effect.kind).toBe('perRow')
    expect(run.estimate.effect.effects).toHaveLength(source.Y.length)
    await expect(canvas.getByTestId('causal-forest-aggregate')).toHaveCount(0)
  } else if (kind !== 'encoded') {
    expect(run.evidence.summary.estimate).toBeCloseTo(Number(source.ate[0]), 7)
    expect(run.evidence.summary.standardError).toBeCloseTo(Number(source.ate[1]), 7)
  }
  const validation = await page.evaluate(async snapshot => {
    const { parseSnapshotValue } = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const changedTarget = structuredClone(snapshot)
    changedTarget.estimationRuns[0].evidence.target = { kind: 'binary-average', population: 'treated' }
    const changedSeed = structuredClone(snapshot)
    changedSeed.estimationRuns[0].configuration.seed += 1
    return [parseSnapshotValue(snapshot).ok, parseSnapshotValue(changedTarget).ok, parseSnapshotValue(changedSeed).ok]
  }, snapshot)
  expect(validation).toEqual([true, false, false])
  if (kind === 'binary') {
    await page.getByRole('spinbutton', { name: 'Trees', exact: true }).fill('100000')
    await page.getByRole('button', { name: /^Run causal forest/ }).click()
    await page.getByRole('button', { name: 'Cancel run', exact: true }).click()
    await expect(page.getByText('The analysis was cancelled.', { exact: true })).toBeVisible()
    expect((await saved()).estimationRuns).toHaveLength(1)
    await page.getByRole('spinbutton', { name: 'Trees', exact: true }).fill('100')
  }
  for (const theme of ['light', 'dark']) {
    await page.evaluate(theme => { document.documentElement.dataset.theme = theme }, theme)
    await canvas.getByTestId('causal-forest-predictions').scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath(`results-${theme}.png`) })
    await canvas.getByRole('button', { name: 'Open Conditional-effect estimates and pointwise intervals in a floating window' }).click()
    await expect(page.getByRole('button', { name: 'Close the floating window' })).toBeVisible()
    await expect(page.getByRole('dialog', { name: 'Conditional-effect estimates and pointwise intervals' })).toHaveCSS('opacity', '1')
    await page.screenshot({ path: info.outputPath(`expanded-${theme}.png`), animations: 'disabled' })
    await page.getByRole('button', { name: 'Close the floating window' }).click()
  }
  await page.reload()
  const restored = await saved()
  expect(restored.estimationRuns[0]).toEqual(run)
})
