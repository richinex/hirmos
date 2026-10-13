import { expect, test } from '@playwright/test'
import { chapter, prepare } from './examples/support'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'

for (const kind of [
  'aalen',
  'survival-forest',
  'right-censored',
  'cox-regression',
  'penalized-aft',
  'start-stop',
  'two-group',
  'multi-state',
] as const) {
  test(`${kind}: reviewed survival configuration saves its own result`, async ({ page }) => {
    test.setTimeout(120000)
    const name = `Assistant survival ${kind}`
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill(name)
    await page.getByRole('button', { name: 'Create project', exact: true }).click()
    await page
      .locator('input[type=file]')
      .setInputFiles(
        kind === 'two-group'
          ? 'tests/fixtures/ab-survival.csv'
          : kind === 'multi-state'
            ? 'public/examples/data/survival/flexsurv_bosms3.csv'
            : kind === 'start-stop'
              ? 'tests/fixtures/survival/panel.csv'
              : 'tests/fixtures/survival/subjects.csv',
      )
    await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
    await prepare(page, {
      structure: 'cross-section',
      columns:
        kind === 'two-group'
          ? ['tstatus', 'status', 'group']
          : kind === 'multi-state'
            ? ['from', 'to', 'Tstart', 'Tstop', 'status']
            : kind === 'start-stop'
              ? ['start', 'stop', 'event', 'experience']
              : ['time', 'event', 'experience'],
    })
    await chapter(page, /Survival/)
    const configuration = await page.evaluate(async (kind) => {
      const { draftFor } = await import(new URL('/src/domain/survivalDraft.ts', location.href).href)
      return draftFor(kind, [])
    }, kind)
    const requests = await connectAssistant(page, [
      `import json
s = await tool("workspace_settings", {"key":"survival"})
c = json.loads(${JSON.stringify(JSON.stringify(configuration))})
columns = {v["name"]:v["id"] for v in s["columns"]}
r = c.get("observation", c.get("input", c))
mapping = {"duration":"time", "event":"event", "start":"start", "stop":"stop", "from":"from", "to":"to"}
if c["kind"] == "two-group":
    mapping.update({"duration":"tstatus", "event":"status"})
    c["group"] = columns["group"]
    c["truncationTime"] = 1
    c["permutations"] = 20
if c["kind"] == "multi-state":
    mapping.update({"start":"Tstart", "stop":"Tstop", "event":"status"})
for field, name in mapping.items():
    if field in r:
        r[field] = columns[name]
if "covariates" in c:
    c["covariates"] = [columns["experience"]]
if c["kind"] == "survival-forest":
    c["settings"]["trees"] = 20
p = await tool("propose_workspace", {"key":"survival","configuration":c,"reason":"Fit the requested survival family using the selected time and event columns."})
assert p["kind"] == "awaiting-user-review", p
await FINAL_VAR("Review the settings.", ["user-question"])`,
      `r = await tool("run_analysis", {"key":"survival","reason":"Fit the approved survival settings."})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed " + r["runId"], ["approved-run"])`,
    ])
    await ask(page, 'Propose the requested survival settings.')
    const card = page.getByRole('region', { name: 'Workspace configuration proposal' })
    await expect(card).toBeVisible({ timeout: 30000 })
    await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
    await expect(card).toContainText('Settings applied. No analysis was run.')
    expect((await snapshot(page, name)).survivalRuns).toHaveLength(0)
    await approveRun(page, 'Fit the approved survival settings.')
    await expect.poll(async () => (await snapshot(page, name)).survivalRuns.length).toBe(1)
    const saved = (await snapshot(page, name)).survivalRuns[0]
    const expectedKind = ['right-censored', 'start-stop', 'two-group', 'multi-state'].includes(kind)
      ? `${kind}-survival-run`
      : `${kind}-run`
    expect(saved.kind).toBe(expectedKind)
    if ('covariates' in saved.configuration) expect(saved.configuration.covariates).toHaveLength(1)
    expect(requests()).toBe(2)
    await page.screenshot({ path: `test-results/assistant-survival-${kind}.png` })
  })
}
