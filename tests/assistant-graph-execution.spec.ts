import { expect, test } from '@playwright/test'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'

for (const key of ['distribution', 'gaussian-distribution', 'intervention']) test(`${key}: reviewed settings and approved computation use the current graph`, async ({ page }) => {
  test.setTimeout(150000)
  const name = `Assistant ${key}`
  const csv = ['X,Y', ...Array.from({ length: 100 }, (_, i) => `${i % 2},${2 + 3 * (i % 2) + Math.sin(i)}`)].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'graph.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /Independent observations/ }).check()
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Measured process')
  await page.getByRole('button', { name: /Create DAG/ }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill('dag { X -> Y; }')
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  const inspector = page.getByRole('button', { name: 'Inspector', exact: true })
  if (await inspector.isVisible()) await inspector.click()
  await page.getByRole('radio', { name: 'Intervene', exact: true }).check()
  if (key !== 'intervention') await page.getByRole('radio', { name: 'Distribution', exact: true }).check()
  if (key === 'gaussian-distribution') await page.getByRole('radio', { name: 'Conditional Gaussian', exact: true }).check()
  const config = key === 'distribution'
    ? '{"outcomes":[y],"observations":[],"interventions":[{"variable":x,"state":1}],"bins":3,"equivalentSampleSize":5}'
    : key === 'intervention'
      ? '{"set":x,"read":y,"condition":{"kind":"none"},"bins":3,"equivalentSampleSize":5}'
      : '{"variables":["continuous"] * len(s["columns"]),"outcome":y,"observations":[],"interventions":[{"variable":x,"value":1}]}'
  const index = key === 'intervention' ? 'v["id"]' : 'int(v["id"])'
  const blocks = [`s = await tool("workspace_settings", {"key":"${key}"})
x = next(${index} for v in s["columns"] if v["name"] == "X")
y = next(${index} for v in s["columns"] if v["name"] == "Y")
c = ${config}
r = await tool("propose_workspace", {"key":"${key}","configuration":c,"reason":"Read Y under intervention on X."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the query.", ["user-question"])`]
  if (key === 'gaussian-distribution') blocks.push(`r = await tool("run_analysis", {"key":"gaussian-review","reason":"Prepare the configured variable values."})
assert r["kind"] == "review-required", r
await FINAL_VAR("Prepared", ["approved-run"])`)
  blocks.push(`r = await tool("run_analysis", {"key":"${key}","reason":"Evaluate the approved query."})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed " + r["runId"], ["approved-run"])`)
  const count = await connectAssistant(page, blocks)
  await ask(page, 'Propose the query settings.')
  const card = page.getByRole('region', { name: 'Workspace configuration proposal' })
  await expect(card).toBeVisible({ timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  expect((await snapshot(page, name))?.interventionQueries ?? []).toHaveLength(0)
  if (key === 'gaussian-distribution') await approveRun(page, 'Prepare the Gaussian values.', 'Prepared')
  await approveRun(page, 'Evaluate the approved query.')
  await expect.poll(async () => (await snapshot(page, name))?.interventionQueries.length).toBe(1)
  const saved = (await snapshot(page, name)).interventionQueries[0]
  expect(saved.kind).toBe(key === 'distribution' ? 'network-query' : key === 'intervention' ? 'intervention-query' : 'conditional-gaussian-query')
  if (key === 'distribution') expect(saved.result.distribution.reduce((sum: number, row: [string[], number]) => sum + row[1], 0)).toBeCloseTo(1, 12)
  if (key === 'gaussian-distribution') expect(saved.result.mean).toBeGreaterThan(4.5)
  expect(count()).toBe(blocks.length)
  await page.screenshot({ path: `test-results/assistant-${key}.png` })
})
