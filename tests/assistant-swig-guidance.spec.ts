import { test, expect } from '@playwright/test'
import { ask, connectAssistant, snapshot } from './assistantTestFlow'
import { prepare } from './examples/support'

test('SWIG navigation selects an exact DAG and carries a kernel refusal to the model', async ({
  page,
}) => {
  test.setTimeout(150000)
  const projectName = 'SWIG guide targets'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(projectName)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'did.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from('D,Y0,Y1\n0,1,2\n1,2,4\n0,2,3\n1,3,5\n'),
  })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await prepare(page, { structure: 'cross-section', columns: 'all' })
  await page.getByRole('button', { name: /Build a DAG/ }).click()
  for (let i = 0; i < 2; i++) {
    if (i) {
      await page.getByRole('radio', { name: 'DAGs', exact: true }).click()
      await page.getByRole('button', { name: 'New DAG', exact: true }).click()
    }
    await page.getByRole('button', { name: /Substantive knowledge/ }).click()
    await page.getByLabel('DAG name').fill('DiD model')
    await page.getByRole('button', { name: /Create DAG/ }).click()
    await page.getByRole('button', { name: 'From text', exact: true }).click()
    await page
      .getByRole('textbox', { name: 'Graph text' })
      .fill(
        'dag { U [latent] e0 [latent] e1 [latent] U -> D U -> Y0 U -> Y1 D -> Y1 e0 -> Y0 e1 -> Y1 Y0 -> Y1 }',
      )
    await page.getByRole('button', { name: 'Convert to DAG' }).click()
  }
  const requests: string[] = []
  page.on('request', (request) => {
    if (request.url().includes('/chat/completions')) requests.push(request.postData() ?? '')
  })
  await connectAssistant(page, [
    `p = await tool("project")
s = await tool("workspace_settings", {"key":"dag-workspace"})
assert s["current"]["document"] == p["graphs"][1]["id"]
r = await tool("propose_workspace", {"key":"dag-workspace", "configuration":{"document":p["graphs"][0]["id"], "inspector":"swig"}, "reason":"Open the first DiD model for its graphical check."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the source graph and SWIG selection.", ["project-summary"])`,
    `p = await tool("project")
s = await tool("workspace_settings", {"key":"swig-settings"})
assert s["graph"]["document"] == p["graphs"][0]["id"], s
assert s["graph"]["name"] == "DiD model (DAG 1)", s
roles = {"U":{"kind":"confounder"}, "D":{"kind":"treatment","period":1}, "Y0":{"kind":"outcome","period":0}, "Y1":{"kind":"outcome","period":1}, "e0":{"kind":"disturbance"}, "e1":{"kind":"disturbance"}}
c = {"purpose":"did", "draft":{"roles":[roles[v["name"]] for v in s["columns"]], "adoption":1, "outcome":1, "comparison":{"kind":"neverTreated"}, "selected":[], "rationale":"Check the declared two-period graph."}}
r = await tool("propose_workspace", {"key":"swig-settings", "graph":{"document":s["graph"]["document"],"revision":s["graph"]["revision"]}, "configuration":c, "reason":"Assess DiD adjustment in the untreated world."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the DiD roles.", ["project-summary"])`,
    `p = await tool("project")
g = p["graphs"][1]
r = await tool("run_analysis", {"key":"swig", "subject":{"kind":"dag", "document":g["id"], "revision":g["revision"]}, "reason":"Check the other graph."})
assert r["kind"] == "refused" and "selected source DAG" in r["reason"], r
await FINAL_VAR("The wrong graph was refused before approval.", ["project-summary"])`,
    `g = (await tool("workspace_settings", {"key":"swig-settings"}))["graph"]
r = await tool("run_analysis", {"key":"swig", "subject":{"kind":"dag", "document":g["document"], "revision":g["revision"]}, "reason":"Assess the reviewed DiD graph."})
assert r["kind"] == "refused", r
assert "R-Y" in r["reason"], r
print(r["kind"])`,
    `await FINAL_VAR("The DiD check was refused because Y0 has children. Repeating the same check will not change that.", ["user-question"])`,
  ])
  const cards = page.getByRole('region', { name: 'Workspace configuration proposal' })
  await ask(page, 'Open SWIG for the first DiD model.')
  await expect(cards.last()).toContainText('DiD model (DAG 1)', { timeout: 30000 })
  await cards.last().getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(
    page.getByRole('button', { name: 'Return to source DAG', exact: true }),
  ).toBeVisible()
  await ask(page, 'Set up the DiD adjustment assessment.')
  await expect(cards.last()).toContainText('DiD adjustment', { timeout: 30000 })
  await expect(cards.last()).toContainText('DiD model (DAG 1)')
  await cards.last().getByRole('button', { name: 'Apply settings', exact: true }).click()
  await ask(page, 'Try a mismatched source graph.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'wrong graph was refused',
    { timeout: 30000 },
  )
  await expect(page.getByRole('region', { name: 'Analysis approval' })).toHaveCount(0)
  await ask(page, 'Run the reviewed DiD check.')
  const approval = page.getByRole('region', { name: 'Analysis approval' }).last()
  await expect(approval).toContainText('Source DAG: DiD model (DAG 1)', { timeout: 30000 })
  await approval.getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(approval).toContainText('The outcome Y0 has children.', { timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Repeating the same check will not change that.',
    { timeout: 30000 },
  )
  expect(requests.at(-1)).toContain('Analysis outcome recorded by Hirmos:')
  expect(requests.at(-1)).toContain('This refusal does not establish nonidentification.')
  expect((await snapshot(page, projectName)).swigAnalyses).toHaveLength(0)
  await page.screenshot({ path: '/tmp/hirmos-swig-guidance.png', fullPage: true })
})

test('graph approval rejects missing, mismatched and stale graph targets', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { runSubjectProblem } = await import(
      new URL('/src/assistant/analysisRun.ts', location.href).href
    )
    const inspection = { graph: { document: 'one', revision: 'latest', name: 'DiD model' } }
    return {
      missing: runSubjectProblem(inspection),
      wrong: runSubjectProblem(inspection, { kind: 'dag', document: 'two', revision: 'latest' }),
      stale: runSubjectProblem(inspection, { kind: 'dag', document: 'one', revision: 'old' }),
      correct: runSubjectProblem(inspection, { kind: 'dag', document: 'one', revision: 'latest' }),
    }
  })
  expect(result.missing).toContain('intended source DAG')
  expect(result.wrong).toContain('selected source DAG')
  expect(result.stale).toContain('revision has changed')
  expect(result.correct).toBeNull()
})
