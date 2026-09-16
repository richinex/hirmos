import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('automatic full-data supply-chain change matches Docker through Run analysis', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Mobile result layout is checked within the same run.')
  test.setTimeout(1_200_000)
  page.setDefaultTimeout(20_000)
  const oracle = JSON.parse(readFileSync('docs/2026-09-16-gcm-supply-chain/ui-reference.json', 'utf8'))
  const lines = readFileSync('docs/2026-09-16-gcm-supply-chain/supply_chain_week_over_week.csv', 'utf8').trim().split(/\r?\n/)
  const header = lines[0]!.split(',')
  const week = header.indexOf('week')
  const indices = oracle.names.map((name: string) => header.indexOf(name))
  const csv = (label: string) => Buffer.from([oracle.names.join(','), ...lines.slice(1).map(line => line.split(',')).filter(row => row[week] === label).map(row => indices.map((index: number) => row[index]).join(','))].join('\n'))
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.routeWebSocket(/.*/, socket => socket.close())
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Supply-chain automatic models')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'supply-week1.csv', mimeType: 'text/csv', buffer: csv('w1') })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of oracle.names) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Supply-chain reference graph')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  for (const [cause, effect] of oracle.edges) {
    for (const [label, name] of [['Proposed cause', oracle.names[cause]], ['Proposed effect', oracle.names[effect]]]) {
      await page.getByRole('combobox', { name: label }).click()
      await page.getByRole('option', { name, exact: true }).click()
    }
    await page.getByRole('textbox', { name: /Rationale/ }).first().fill('Relationship specified in the preserved DoWhy 0.14 supply-chain notebook.')
    await page.getByRole('button', { name: 'Add the arrow' }).click()
  }
  await page.getByRole('button', { name: 'Use for causal model analysis', exact: true }).click()
  await page.getByRole('radio', { name: 'Distribution change', exact: true }).check()
  await page.getByRole('combobox', { name: /Conditional models/ }).click()
  await page.getByRole('option', { name: 'Automatic selection', exact: true }).click()
  await page.getByRole('combobox', { name: 'Target variable', exact: true }).click()
  await page.getByRole('option', { name: 'received', exact: true }).click()
  await page.getByLabel('Comparison dataset', { exact: true }).setInputFiles({ name: 'supply-week2.csv', mimeType: 'text/csv', buffer: csv('w2') })
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByLabel('Refitted estimates', { exact: true }).fill('5')
  await page.getByLabel('Random seed', { exact: true }).fill('10')
  await page.getByRole('checkbox', { name: /same variable definitions/ }).check()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toBeVisible({ timeout: 900_000 })
  const saved = () => page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const project = await store.loadProject((await store.listProjects())[0].id)
    return project.ok ? project.value.rootCause.runs.at(-1) : null
  })
  await expect.poll(async () => Boolean(await saved())).toBe(true)
  const record = await saved()
  const result = record.evidence.outcome
  expect(record.model.query.fitting.kind).toBe('automaticFull')
  expect(record.model.fraction).toBe(1)
  expect(result.summary.quantiles).toEqual([1 - 0.95, 0.95])
  for (let column = 0; column < result.nodes.length; column++) {
    const name = record.model.names[result.nodes[column]]
    const sourceColumn = oracle.summaryNames.indexOf(name)
    expect(Math.abs(result.summary.estimates[column] - oracle.estimates[name]), name).toBeLessThanOrEqual(1e-8)
    for (let k = 0; k < 2; k++) expect(Math.abs(result.summary.bounds[column][k] - oracle.bounds[name][k])).toBeLessThanOrEqual(1e-8)
    for (let row = 0; row < 5; row++) expect(Math.abs(result.summary.replicates[row][column] - oracle.replicates[row][sourceColumn]), `${name}, repeat ${row}`).toBeLessThanOrEqual(1e-8)
  }
  expect(record.evidence.random).toEqual(oracle.random)
  expect(result.means.kind).toBe('recorded')
  for (const entry of result.means.values) {
    for (const [label, key] of [['w1', 'baseline'], ['w2', 'comparison']] as const) {
      const values = lines.slice(1).map(line => line.split(',')).filter(row => row[week] === label).map(row => Number(row[indices[entry.node]]))
      const expected = values.reduce((sum, value) => sum + value, 0) / values.length
      expect(Math.abs(entry[key] - expected)).toBeLessThan(1e-10)
    }
  }
  await info.attach('supply-chain-ui.json', { body: JSON.stringify(record, null, 2), contentType: 'application/json' })
  await page.getByTestId('root-cause-bars').scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('supply-chain-desktop.png'), fullPage: true })
  await page.getByText('Compare observed means', { exact: true }).click()
  await expect(page.getByTestId('root-cause-means')).toBeVisible()
  await page.getByRole('combobox', { name: 'Observed mean variable', exact: true }).click()
  await page.getByRole('option', { name: 'demand', exact: true }).click()
  await page.getByTestId('root-cause-means').scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('supply-chain-means.png'), fullPage: true })
  await page.setViewportSize({ width: 390, height: 844 })
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('supply-chain-mobile.png'), fullPage: true })
  expect(errors).toEqual([])
})
