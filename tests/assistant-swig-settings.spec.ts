import { expect, test } from '@playwright/test'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'
import { chapter } from './examples/support'

test('SWIG settings, construction and separation use separate approvals', async ({ page }) => {
  test.setTimeout(120000)
  const name = 'Severity, dose and recovery'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
  await chapter(page, /DAG workspace/)
  await page.getByRole('radio', { name: 'SWIG', exact: true }).click()
  await connectAssistant(page, [
    `s = await tool("workspace_settings", {"key":"swig-settings"})
c = s["current"]
dose = next(v["id"] for v in s["columns"] if v["name"] == "dose")
c["interventions"] = [{"key":"dose", "variable":dose, "value":"0"}]
c["rationale"] = "Severity causes both dose and recovery. Set dose to zero while retaining natural assignment."
p = await tool("propose_workspace", {"key":"swig-settings", "graph":{"document":s["graph"]["document"],"revision":s["graph"]["revision"]}, "configuration":c, "reason":"Review the intervention graph."})
assert p["kind"] == "awaiting-user-review", p
await FINAL_VAR("Review the intervention settings.", ["user-question"])`,
    `g = (await tool("workspace_settings", {"key":"swig-settings"}))["graph"]
r = await tool("run_analysis", {"key":"swig", "reason":"Construct the reviewed graph.", "subject":{"kind":"dag", "document":g["document"], "revision":g["revision"]}})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed", ["approved-run"])`,
    `s = await tool("workspace_settings", {"key":"swig-separation"})
a = next(v["id"] for v in s["columns"] if v["name"] == "dose")
b = next(v["id"] for v in s["columns"] if v["name"] == "recovery (dose = 0)")
c = {"left":a, "right":b, "given":[]}
p = await tool("propose_workspace", {"key":"swig-separation", "graph":{"document":s["graph"]["document"],"revision":s["graph"]["revision"]}, "configuration":c, "reason":"Check unconditional separation."})
assert p["kind"] == "awaiting-user-review", p
await FINAL_VAR("Review the separation variables.", ["user-question"])`,
    `g = (await tool("workspace_settings", {"key":"swig-separation"}))["graph"]
r = await tool("run_analysis", {"key":"swig-separation", "reason":"Check the reviewed variables.", "subject":{"kind":"dag", "document":g["document"], "revision":g["revision"]}})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed", ["approved-run"])`,
  ])
  const count = async () => (await snapshot(page, name)).swigAnalyses.length
  const before = await count()
  await ask(page, 'Set dose to zero in an intervention graph.')
  const cards = page.getByRole('region', { name: 'Workspace configuration proposal' })
  await expect(cards.last()).toContainText('dose = 0', { timeout: 30000 })
  await cards.last().getByRole('button', { name: 'Apply settings', exact: true }).click()
  expect(await count()).toBe(before)
  await approveRun(page, 'Construct this graph.')
  await expect.poll(count).toBe(before + 1)
  await ask(page, 'Check dose and recovery for separation.')
  await expect(cards.last()).toContainText('recovery (dose = 0)', { timeout: 30000 })
  await cards.last().getByRole('button', { name: 'Apply settings', exact: true }).click()
  expect(await count()).toBe(before + 1)
  await approveRun(page, 'Run the separation check.')
  await expect.poll(count).toBe(before + 2)
  await expect(page.getByTestId('swig-conclusion')).toContainText(
    'd-connected without conditioning',
  )
  await page.screenshot({ path: '/tmp/hirmos-assistant-swig.png', fullPage: true })
})

test('covariate encoding proposals update only eligible covariates and do not fit', async ({
  page,
}) => {
  test.setTimeout(90000)
  const name = 'NSW job training and 1978 earnings'
  await page.goto('/app/projects')
  await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
  await chapter(page, /Estimation/)
  await connectAssistant(page, [
    `s = await tool("workspace_settings", {"key":"covariate-encoding"})
c = s["current"]
c["categorical"] = [next(v["id"] for v in s["columns"] if v["name"] == "married")]
p = await tool("propose_workspace", {"key":"covariate-encoding", "configuration":c, "reason":"Treat marital status as categorical."})
assert p["kind"] == "awaiting-user-review", p
await FINAL_VAR("Review the encoding.", ["user-question"])`,
    `p = await tool("propose_workspace", {"key":"covariate-encoding", "configuration":{"categorical":["not-a-column"]}, "reason":"Check the column boundary."})
assert p["kind"] == "refused", p
await FINAL_VAR("Unknown column refused.", ["user-question"])`,
  ])
  const before = (await snapshot(page, name)).estimationRuns.length
  await ask(page, 'Encode married as categorical.')
  const card = page.getByRole('region', { name: 'Workspace configuration proposal' })
  await expect(card).toContainText('Categorical', { timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(
    page
      .getByRole('group', { name: 'Categorical covariates' })
      .getByRole('checkbox', { name: 'married', exact: true }),
  ).toBeChecked()
  expect((await snapshot(page, name)).estimationRuns.length).toBe(before)
  await ask(page, 'Try an unavailable column.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Unknown column refused.',
    { timeout: 30000 },
  )
  await page.screenshot({ path: '/tmp/hirmos-assistant-encoding.png', fullPage: true })
})
