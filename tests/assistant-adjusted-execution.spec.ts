import { expect, test } from '@playwright/test'
import { addArrow, chapter, choose, createDag, identify, fixture } from './examples/support'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'

for (const kind of ['regression', 'doublyRobust']) test(`approved ${kind} DiD uses its selected covariate and settings`, async ({ page }) => {
  test.setTimeout(150000)
  const name = `Assistant ${kind} DiD`
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(fixture('chapter-did-covariates.csv'))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await choose(page, 'Unit column', 'id')
  await choose(page, 'Time column', 'time_points')
  for (const column of ['Y', 'treatment', 'A']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await createDag(page, 'Adjusted comparison')
  await addArrow(page, 'treatment', 'Y', 'Treatment may change the outcome after baseline.')
  await identify(page, { target: /Treated rows/, graph: 'Adjusted comparison', treatment: 'treatment', outcome: 'Y', mechanism: 'Policy change', sentence: 'Compare treated and never-treated units before and after adoption.' })
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).click()
  await page.getByRole('radio', { name: /Panel difference-in-differences/ }).click()
  const specification = kind === 'regression' ? { kind } : { kind, folds: 5, seed: 1234, trimming: 0.01, normalization: 'in-sample' }
  const count = await connectAssistant(page, [
    `import json
p = await tool("project")
print(p["estimation"])
c = p["estimation"]["configuration"]["columns"]
x = next(v["id"] for v in c if v["name"] == "A")
s = json.loads(${JSON.stringify(JSON.stringify(specification))})
r = await tool("propose_estimator", {"estimator":"panel-intervention","configuration":{"kind":"panel-intervention","primary":"adjusted","covariates":[x],"specification":s},"reason":"Use A as the baseline covariate."})
assert r["kind"] == "awaiting-user-approval", r
await FINAL_VAR("Review the settings.", ["project-summary"])`,
    `p = await tool("project")
s = p["runs"]["estimation"]["study"]["subject"]
r = await tool("run_analysis", {"key":"estimation","subject":s,"reason":"Fit the approved adjusted DiD."})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed " + r["runId"], ["approved-run"])`,
  ])
  await ask(page, 'Propose the requested adjusted DiD.')
  const card = page.getByRole('region', { name: 'Estimator proposal' })
  await expect(card).toBeVisible({ timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied.')
  await approveRun(page, 'Run the approved adjusted DiD.')
  await expect.poll(async () => (await snapshot(page, name))?.estimationRuns.length).toBe(1)
  const saved = (await snapshot(page, name)).estimationRuns[0]
  expect(saved.configuration.specification).toEqual(specification)
  expect(saved.configuration.covariates).toHaveLength(1)
  expect(saved.evidence.kind).toBe('panelAdjusted')
  expect(saved.estimate.estimand.kind).toBe('average-treatment-effect-on-treated')
  expect(count()).toBe(2)
})
