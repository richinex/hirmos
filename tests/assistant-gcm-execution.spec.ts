import { expect, test } from '@playwright/test'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'

for (const analysis of [
  'effects',
  'arrows',
  'intrinsic',
  'anomaly',
  'change',
  'intervention',
  'checks',
] as const)
  test(`Causal model ${analysis}: reviewed settings reach the saved run`, async ({ page }) => {
    test.setTimeout(150000)
    const name = `Assistant GCM ${analysis}`
    const csv = [
      'X,Y',
      ...Array.from({ length: 100 }, (_, i) => `${i % 2},${2 + 3 * (i % 2) + Math.sin(i)}`),
    ].join('\n')
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill(name)
    await page.getByRole('button', { name: 'Create project', exact: true }).click()
    await page
      .locator('input[type=file]')
      .setInputFiles({ name: 'graph.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
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
    await page.getByRole('button', { name: 'Use for causal model analysis', exact: true }).click()
    const modify =
      analysis === 'effects'
        ? 'c["trees"] = 10\nc["repetitions"] = 3'
        : analysis === 'arrows'
          ? 'c["target"] = next(v["id"] for v in s["columns"] if v["name"] == "Y")\nc["query"] = {"kind":"arrows","conditional":20,"maxRuns":10,"tolerance":0.1}'
          : analysis === 'intrinsic'
            ? 'c["target"] = next(v["id"] for v in s["columns"] if v["name"] == "Y")\nc["query"] = {"kind":"intrinsic","training":50,"randomization":10,"baseline":10}'
            : 'c["target"] = str(next(i for i,v in enumerate(s["columns"]) if v["name"] == "Y"))\nc["repetitions"] = 2\nc["samples"] = 20\nc["observationDraft"] = {v["id"]: ("1" if v["name"] == "X" else "7") for v in s["columns"]}'
    const attribution = ['anomaly', 'change', 'intervention', 'checks'].includes(analysis)
    const key = attribution ? 'gcm-attribution' : `gcm-${analysis}`
    const requests = await connectAssistant(page, [
      `r = await tool("propose_workspace", {"key":"causal-model","configuration":{"analysis":"${analysis === 'checks' ? 'anomaly' : analysis}"},"reason":"Select the requested causal model analysis."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the analysis choice.", ["user-question"])`,
      `s = await tool("workspace_settings", {"key":"${key}"})
c = s["current"]
${modify}
${analysis === 'intervention' ? 'c["shifts"] = {v["id"]:"1" for v in s["columns"] if v["name"] == "X"}' : ''}
r = await tool("propose_workspace", {"key":"${key}","configuration":c,"reason":"Use the requested small simulation for the wiring check."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the model settings.", ["user-question"])`,
      `r = await tool("run_analysis", {"key":"${analysis === 'checks' ? 'gcm-checks' : key}","reason":"Run the approved causal model."})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed " + r["runId"], ["approved-run"])`,
      `p = await tool("project")
item = p["savedCausalModelRuns"][-1]
r = await tool("run", {"workspace":"causal-model","id":item["id"]})
s = await tool("run", {"workspace":"causal-model","id":item["id"],"view":"specification"})
e = await tool("run", {"workspace":"causal-model","id":item["id"],"view":"evidence"})
assert r["kind"] == s["kind"] == e["kind"] == "saved-run"
assert r["provenance"]["graph"] == "current", r
assert s["graphRevision"]["id"] == item["graph"]["dagRevision"]
assert r["summary"] and e["evidence"]
await FINAL_VAR("Read the saved causal model without another fit.", ["saved-run"])`,
    ])
    for (const question of ['Select the requested analysis.', 'Propose the requested settings.']) {
      await ask(page, question)
      const card = page.getByRole('region', { name: 'Workspace configuration proposal' }).last()
      await expect(card.getByRole('button', { name: 'Apply settings', exact: true })).toBeVisible({
        timeout: 30000,
      })
      await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
      await expect(card).toContainText('Settings applied. No analysis was run.')
    }
    if (analysis === 'anomaly') {
      const confirmation = page.getByRole('checkbox', {
        name: 'These values use the same units and transformations as the prepared data.',
      })
      await expect(confirmation).not.toBeChecked()
      await confirmation.check()
    }
    if (analysis === 'change' || analysis === 'intervention') {
      const comparison = [
        'X,Y',
        ...Array.from({ length: 100 }, (_, i) => `${i % 2},${3 + 3 * (i % 2) + Math.sin(i)}`),
      ].join('\n')
      await page
        .getByLabel(analysis === 'change' ? 'Comparison dataset' : 'Intervention model data', {
          exact: true,
        })
        .setInputFiles({
          name: 'comparison.csv',
          mimeType: 'text/csv',
          buffer: Buffer.from(comparison),
        })
      const confirmation = page.getByRole('checkbox', {
        name: 'The selected file uses the same variable definitions, units and transformations as the prepared data.',
      })
      await expect(confirmation).not.toBeChecked()
      await confirmation.check()
    }
    const collection =
      analysis === 'effects'
        ? 'effects'
        : analysis === 'checks'
          ? 'checks'
          : attribution
            ? 'runs'
            : 'influences'
    expect((await snapshot(page, name)).rootCause[collection]).toHaveLength(0)
    await approveRun(page, 'Run the approved model.')
    await expect.poll(async () => (await snapshot(page, name)).rootCause[collection].length).toBe(1)
    const saved = (await snapshot(page, name)).rootCause[collection][0]
    if (analysis === 'effects') {
      expect(saved.model.trees).toBe(10)
      expect(saved.evidence.estimates[0]).toBeGreaterThan(2)
    } else if (analysis === 'checks') expect(saved.model.scope).toBe('fitted')
    else expect(saved.model.query.kind).toBe(analysis)
    if (analysis === 'change' || analysis === 'intervention') {
      expect(saved.comparison.name).toBe('comparison.csv')
      expect(saved.comparison.rows).toBe(100)
    }
    await ask(page, 'Read the saved causal model.')
    await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
      'without another fit',
      { timeout: 30000 },
    )
    expect((await snapshot(page, name)).rootCause[collection]).toHaveLength(1)
    expect(requests()).toBe(4)
    await page.screenshot({ path: `test-results/assistant-gcm-${analysis}.png` })
  })
