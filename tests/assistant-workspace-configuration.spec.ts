import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('Survival schemas cover every draft family and reject missing columns', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { survivalConfigurationSchema } = await import(
      new URL('/src/assistant/survivalConfiguration.ts', location.href).href
    )
    const { draftFor } = await import(new URL('/src/domain/survivalDraft.ts', location.href).href)
    const columns = [
      { id: 'time', name: 'time' },
      { id: 'event', name: 'event' },
    ]
    const schema = survivalConfigurationSchema(columns)
    return {
      valid: [
        'aalen',
        'survival-forest',
        'right-censored',
        'nonparametric',
        'start-stop',
        'cox-regression',
        'penalized-aft',
        'two-group',
        'multi-state',
      ].map((kind) => schema.safeParse(draftFor(kind, columns)).success),
      unknown: schema.safeParse({ ...draftFor('nonparametric', columns), duration: 'missing' })
        .success,
      extra: schema.safeParse({ ...draftFor('nonparametric', columns), run: true }).success,
    }
  })
  expect(result.valid).toEqual(Array(9).fill(true))
  expect(result.unknown).toBe(false)
  expect(result.extra).toBe(false)
})

test('Survival settings require review, then a separate approved fit saves the requested model', async ({
  page,
}) => {
  test.setTimeout(90000)
  let requests = 0
  const propose = `s = await tool("workspace_settings", {"key":"survival"})
c = s["current"]
r = c.get("observation", c)
d = {"kind":"nonparametric","duration":r["duration"],"event":r["event"],"rowFrequency":{"kind":"one-observation-per-row"},"horizon":5,"ties":"discrete"}
bad = await tool("propose_workspace", {"key":"survival","configuration":dict(d, duration="missing"),"reason":"Invalid column check"})
assert bad["kind"] == "refused", bad
p = await tool("propose_workspace", {"key":"survival","configuration":d,"reason":"Estimate the survival curve through time 5."})
assert p["kind"] == "awaiting-user-review", p
p2 = await tool("propose_workspace", {"key":"survival","configuration":dict(d, horizon=6),"reason":"Alternative horizon for review."})
assert p2["kind"] == "awaiting-user-review", p2
await FINAL_VAR("Review the proposed settings.", ["user-question"])`
  const run = `p = await tool("project")
r = await tool("run_analysis", {"key":"survival","reason":"Fit the approved survival configuration."})
assert r["kind"] == "completed", r
await FINAL_VAR("Saved survival run " + r["runId"], ["approved-run"])`
  await page.route('https://openrouter.ai/api/v1/key', (route) =>
    route.fulfill({ json: { data: { limit: null, limit_remaining: null } } }),
  )
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({
        choices: [
          { message: { content: '```python\n' + (++requests === 1 ? propose : run) + '\n```' } },
        ],
      }),
    ),
  )
  await page.goto('/app/projects')
  await page
    .getByRole('button', { name: 'Open Breast cancer survival, German study', exact: true })
    .click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Survival/ })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  const snapshot = () =>
    page.evaluate(async () => {
      const { listProjects, loadProject } = await import(
        new URL('/src/data/projectStore.ts', location.href).href
      )
      const header = (await listProjects()).find(
        (h: { name: string }) => h.name === 'Breast cancer survival, German study',
      )
      const saved = await loadProject(header.id)
      if (!saved.ok) throw new Error('Missing project')
      return saved.value.survivalRuns
    })
  const before = await snapshot()
  await page
    .getByLabel('Ask a question')
    .fill('Propose a nonparametric survival curve through time 5.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const card = page.getByRole('region', { name: 'Workspace configuration proposal' }).first()
  await expect(card).toBeVisible({ timeout: 30000 })
  expect(await snapshot()).toEqual(before)
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  const other = page.getByRole('region', { name: 'Workspace configuration proposal' }).last()
  await expect(other.getByRole('button', { name: 'Apply settings', exact: true })).toHaveCount(0)
  await expect(other).toContainText('The workspace or settings changed.')
  expect(await snapshot()).toEqual(before)
  await page.screenshot({ path: 'test-results/survival-settings.png' })
  await page.getByLabel('Ask a question').fill('Fit the approved survival model.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval).toBeVisible({ timeout: 30000 })
  await approval.getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Saved survival run',
    { timeout: 45000 },
  )
  await expect.poll(async () => (await snapshot()).length).toBe(before.length + 1)
  const saved = (await snapshot()).at(-1)
  expect(saved.configuration.kind).toBe('right-censored-nonparametric')
  expect(saved.configuration.predictionTimes.at(-1)).toBe(5)
  expect(saved.configuration.ties).toBe('discrete')
  expect(requests).toBe(2)
  await page.screenshot({ path: 'test-results/survival-approved-run.png' })
})
