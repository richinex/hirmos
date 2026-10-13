import { expect, test } from '@playwright/test'
import { chapter, prepare } from './examples/support'
import { ask, connectAssistant, snapshot } from './assistantTestFlow'

test('Survival distinguishes text covariates from excluded and unknown columns without changing settings', async ({
  page,
}) => {
  const name = 'Survival column refusals'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({
    name: 'survival.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(
      'time,event,group,age\n1,1,control,30\n2,0,treated,40\n3,1,control,35\n4,0,treated,45\n',
    ),
  })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await prepare(page, { structure: 'cross-section', columns: ['time', 'event'] })
  await chapter(page, /Survival/)
  const count = await connectAssistant(page, [
    `s = await tool("workspace_settings", {"key":"survival"})
ids = {v["name"]:v["id"] for v in s["columns"]}
c = {"kind":"right-censored","duration":ids["time"],"event":ids["event"],"rowFrequency":{"kind":"one-observation-per-row"},"covariates":[],"family":"logNormal","horizon":5}
for column, expected in [(ids["group"], "not numeric"), (ids["age"], "not included in the prepared dataset"), ("missing", "Choose a column in this dataset")]:
    r = await tool("propose_workspace", {"key":"survival","configuration":dict(c, covariates=[column]),"reason":"Check the requested covariate."})
    assert r["kind"] == "refused", r
    assert expected in str(r), r
await FINAL_VAR("Text, excluded and unknown columns were refused for their actual reasons.", ["user-question"])`,
  ])
  await ask(page, 'Check the three invalid covariate selections.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'actual reasons',
    { timeout: 30000 },
  )
  await expect(page.getByRole('region', { name: 'Workspace configuration proposal' })).toHaveCount(
    0,
  )
  expect((await snapshot(page, name)).survivalRuns).toHaveLength(0)
  expect(count()).toBe(1)
})
