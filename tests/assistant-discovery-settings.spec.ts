import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('every discovery family exports reviewed settings and applies them without changing method', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const d = await import(new URL('/src/domain/discovery.ts', location.href).href)
    const s = await import(new URL('/src/domain/discoverySettings.ts', location.href).href)
    const { z } = await import(new URL('/node_modules/.vite/deps/zod.js', location.href).href)
    const methods = d.DISCOVERY_METHOD_GROUPS.flatMap((g: { methods: string[] }) => g.methods)
    const defaults = methods.map((m: string) => d.initialConfigurationFor(m))
    const pc = d.initialConfigurationFor('pc-stable')
    return {
      valid: defaults.map((v: unknown) => s.discoverySettingsSchema.safeParse(v).success),
      labels: defaults.map((v: unknown) =>
        s
          .discoverySettings(v, 'Method', [
            { id: 'x', name: 'X' },
            { id: 'y', name: 'Y' },
          ])
          .every(
            (r: { label: string; value: string }) =>
              typeof r.label === 'string' && typeof r.value === 'string',
          ),
      ),
      applied: defaults.map(
        (v: unknown) =>
          JSON.stringify(
            d.stepDiscovery({ configuration: v }, { type: 'settings-applied', configuration: v })
              .configuration,
          ) === JSON.stringify(v),
      ),
      exported: !!z.toJSONSchema(s.discoverySettingsSchema, { io: 'input' }),
      wrongMethod: d.stepDiscovery(
        { configuration: pc },
        { type: 'settings-applied', configuration: d.initialConfigurationFor('cmlp') },
      ).configuration.kind,
      invalidNeural: s.discoverySettingsSchema.safeParse({
        ...d.initialConfigurationFor('cmlp'),
        checkEvery: 100,
        maxIter: 10,
      }).success,
      invalidGrace: s.discoverySettingsSchema.safeParse({
        ...d.initialConfigurationFor('grace'),
        epochs: 1,
        patience: 2,
      }).success,
      invalidLag: s.discoverySettingsSchema.safeParse({
        ...d.initialConfigurationFor('rpcmci'),
        tauMin: 3,
        tauMax: 1,
      }).success,
      unknownNode: s.discoveryBackgroundProblem(
        { ...pc, background: { ...pc.background, required: [[0, 10]] } },
        2,
      ),
      conflict: s.discoveryBackgroundProblem(
        { ...pc, background: { ...pc.background, required: [[0, 1]], forbidden: [[0, 1]] } },
        2,
      ),
    }
  })
  expect(result.valid).toEqual(Array(15).fill(true))
  expect(result.labels).toEqual(Array(15).fill(true))
  expect(result.applied).toEqual(Array(15).fill(true))
  expect(result.exported).toBe(true)
  expect(result.wrongMethod).toBe('pc-stable')
  expect(result.invalidNeural).toBe(false)
  expect(result.invalidGrace).toBe(false)
  expect(result.invalidLag).toBe(false)
  expect(result.unknownNode).not.toBeNull()
  expect(result.conflict).not.toBeNull()
})

test('discovery selection and parameters require review before an approved fit', async ({
  page,
}) => {
  test.setTimeout(90000)
  let request = 0
  const codes = [
    'p = await tool("propose_workspace", {"key":"discovery-method", "configuration":{"method":"pc-stable"}, "reason":"Review a constraint-based graph."})\nassert p["kind"] == "awaiting-user-review", p\nawait FINAL_VAR("Review PC-stable with its defaults.", ["user-question"])',
    's = await tool("workspace_settings", {"key":"discovery-settings"})\nc = s["current"]\nc["alpha"] = 0.1\nc["maxDepth"] = 1\np = await tool("propose_workspace", {"key":"discovery-settings", "configuration":c, "reason":"Use significance level 0.1 and depth 1."})\nassert p["kind"] == "awaiting-user-review", p\nawait FINAL_VAR("Review the significance level and depth.", ["user-question"])',
    'r = await tool("run_analysis", {"key":"discovery", "reason":"Fit the reviewed discovery settings."})\nassert r["kind"] == "completed", r\nawait FINAL_VAR("Discovery result saved.", ["approved-run"])',
  ]
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({
        choices: [
          { message: { content: '```python\n' + codes[Math.min(request++, 2)] + '\n```' } },
        ],
      }),
    ),
  )
  await page.goto('/app')
  await page
    .getByRole('button', { name: 'Open A simulated process with a collider', exact: true })
    .click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Discovery/ })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  const count = () =>
    page.evaluate(async () => {
      const { listProjects, loadProject } = await import(
        new URL('/src/data/projectStore.ts', location.href).href
      )
      const project = (await listProjects()).find(
        (p: { name: string }) => p.name === 'A simulated process with a collider',
      )
      const saved = await loadProject(project.id)
      return saved.value.discoveryRuns.length
    })
  const before = await count()
  const ask = async (text: string) => {
    await page.getByLabel('Ask a question').fill(text)
    await page.getByRole('button', { name: 'Send', exact: true }).click()
  }
  await ask('Propose PC-stable.')
  let card = page.getByRole('region', { name: 'Workspace configuration proposal' }).last()
  await expect(card).toBeVisible({ timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  expect(await count()).toBe(before)
  await ask('Propose alpha 0.1 and depth 1.')
  card = page.getByRole('region', { name: 'Workspace configuration proposal' }).last()
  await expect(card).toContainText('0.1', { timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  expect(await count()).toBe(before)
  await ask('Run the reviewed discovery settings.')
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval).toBeVisible({ timeout: 30000 })
  await approval.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Discovery result saved.',
    { timeout: 30000 },
  )
  expect(await count()).toBe(before + 1)
  await page.screenshot({ path: '/tmp/hirmos-assistant-discovery.png', fullPage: true })
})
