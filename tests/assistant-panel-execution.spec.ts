import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

const configurations = [
  {
    kind: 'panel-intervention',
    primary: 'did',
    placeboReplications: 20,
    seed: 17,
    inference: { kind: 'bootstrap', replications: 20, seed: 17 },
  },
  { kind: 'panel-intervention', primary: 'syntheticDid', placeboReplications: 20, seed: 17 },
  {
    kind: 'panel-intervention',
    primary: 'staggered',
    covariates: [],
    clustering: { kind: 'unit' },
    headline: 'simple',
    specification: {
      adjustment: { kind: 'outcomeRegression' },
      controls: 'never-treated',
      baseline: 'varying',
      anticipation: 0,
      firstEvent: null,
      lastEvent: null,
      balance: null,
      confidence: 0.95,
      inference: { kind: 'analytical' },
    },
  },
  {
    kind: 'panel-intervention',
    primary: 'sunAbraham',
    referencePeriods: [-1],
    referenceCohorts: [],
    confidence: 0.95,
  },
]

const cases = configurations.flatMap((configuration) =>
  configuration.primary === 'staggered'
    ? [
        configuration,
        {
          ...configuration,
          specification: { ...configuration.specification!, controls: 'not-yet-treated' },
        },
      ]
    : [configuration],
)

for (const configuration of cases) {
  const refuses =
    configuration.primary === 'staggered' &&
    configuration.specification?.controls === 'never-treated'
  test(`${configuration.primary} ${configuration.specification?.controls ?? ''}: ${refuses ? 'unsupported comparisons fail without saving a run' : 'approved configuration reaches the saved run unchanged'}`, async ({
    page,
  }) => {
    test.setTimeout(90000)
    let requests = 0
    const propose = `import json\np = await tool("project")\nc = json.loads(${JSON.stringify(JSON.stringify(configuration))})\nr = await tool("propose_estimator", {"estimator":"panel-intervention","configuration":c,"reason":"Review these settings before execution."})\nassert r["kind"] == "awaiting-user-approval", r\nawait FINAL_VAR("Settings proposed.", ["project-summary"])`
    const run = refuses
      ? 'p = await tool("project")\ns = p["runs"]["estimation"]["study"]["subject"]\nr = await tool("run_analysis", {"key":"estimation","subject":s,"reason":"Check the requested staggered specification."})\nassert r["kind"] == "failed", r\nawait FINAL_VAR("Fit refused: " + r["reason"], ["project-summary"])'
      : 'p = await tool("project")\ns = p["runs"]["estimation"]["study"]["subject"]\nr = await tool("run_analysis", {"key":"estimation","subject":s,"reason":"Run the approved DiD specification."})\nassert r["kind"] == "completed", r\nassert r["subject"] == s\nawait FINAL_VAR("Saved run " + r["runId"], ["approved-run"])'
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
    await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
    await page
      .getByRole('navigation', { name: 'Workspace sections' })
      .getByRole('button', { name: /Estimation/ })
      .click()
    await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
    await page.getByLabel('OpenRouter API key').fill('test-key')
    await page.getByLabel('OpenRouter model ID').fill('test/model')
    await page.getByRole('button', { name: 'Privacy', exact: true }).click()
    await page.getByRole('button', { name: 'I agree', exact: true }).click()
    await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
    await page.getByLabel('Ask a question').fill('Propose the requested DiD settings.')
    await page.getByRole('button', { name: 'Send', exact: true }).click()
    const proposal = page.getByRole('region', { name: 'Estimator proposal' })
    await expect(proposal).toBeVisible({ timeout: 30000 })
    await proposal.getByRole('button', { name: 'Apply settings', exact: true }).click()
    await expect(proposal).toContainText('Settings applied.')
    await expect(page.getByText('Runs (2)', { exact: true })).toBeVisible()
    await page.getByLabel('Ask a question').fill('Run these settings after I approve.')
    await page.getByRole('button', { name: 'Send', exact: true }).click()
    const approval = page.getByRole('region', { name: 'Analysis approval' })
    await expect(approval).toBeVisible({ timeout: 30000 })
    await approval.getByRole('button', { name: 'Run and share summary', exact: true }).click()
    if (refuses) {
      await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
        'requires at least 5',
        { timeout: 45000 },
      )
      await expect(page.getByText('Runs (2)', { exact: true })).toBeVisible()
      expect(requests).toBe(2)
      return
    }
    await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
      'Saved run ',
      { timeout: 45000 },
    )
    const answer = await page.getByRole('region', { name: 'Assistant answer' }).last().innerText()
    const runId = answer.match(/Saved run ([\w-]+)/)?.[1]
    expect(runId).toBeTruthy()
    const readSaved = () =>
      page.evaluate(async (id) => {
        const { listProjects, loadProject } = await import(
          new URL('/src/data/projectStore.ts', location.href).href
        )
        const headers = await listProjects()
        const header = headers.find((h: { name: string }) => h.name === 'AI adoption, March cohort')
        const result = await loadProject(header.id)
        if (!result.ok) throw new Error('Missing saved project')
        return result.value.estimationRuns.find((run: { id: string }) => run.id === id) ?? null
      }, runId)
    await expect.poll(readSaved).not.toBeNull()
    const saved = await readSaved()
    expect(saved.configuration).toEqual(configuration)
    expect(saved.estimate.sample.observations).toBeGreaterThan(0)
    expect(saved.id).toBeTruthy()
    expect(requests).toBe(2)
  })
}
