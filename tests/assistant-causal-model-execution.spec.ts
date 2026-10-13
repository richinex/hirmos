import { expect, test } from '@playwright/test'
import { chapter, identify, runEstimator } from './examples/support'
import { ask, approveRun, connectAssistant, snapshot } from './assistantTestFlow'

for (const [name, configuration] of [
  [
    'Severity, dose and recovery',
    { kind: 'linear-scm', interventions: [0, 1], observationNoise: null },
  ],
  [
    'Deploys and incidents at a lag',
    {
      kind: 'dynamic-linear-scm',
      interventions: [0, 1],
      schedule: { kind: 'point', row: 20 },
      steps: 10,
      uncertainty: { kind: 'none' },
    },
  ],
] as const)
  test(`${configuration.kind}: reviewed configuration reaches the saved causal model`, async ({
    page,
  }) => {
    test.setTimeout(90000)
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
    if (configuration.kind === 'linear-scm')
      await identify(page, {
        graph: 'Severity confounds dose',
        treatment: 'dose',
        outcome: 'recovery',
        mechanism: 'Observed choice',
        sentence: 'Severity affects the assigned dose.',
      })
    if (configuration.kind === 'linear-scm')
      await runEstimator(page, { run: /^Run adjusted linear regression/ })
    await chapter(page, /Counterfactuals/)
    const before = (await snapshot(page, name)).counterfactualRuns.length
    const count = await connectAssistant(page, [
      `import json
s = await tool("workspace_settings", {"key":"counterfactual"})
c = json.loads(${JSON.stringify(JSON.stringify(configuration))})
r = await tool("propose_workspace", {"key":"counterfactual","configuration":c,"reason":"Compare intervention values zero and one."})
assert r["kind"] == "awaiting-user-review", r
await FINAL_VAR("Review the model settings.", ["user-question"])`,
      `r = await tool("run_analysis", {"key":"counterfactual","reason":"Fit the approved causal model."})
assert r["kind"] == "completed", r
await FINAL_VAR("Completed " + r["runId"], ["approved-run"])`,
    ])
    await ask(page, 'Propose the requested causal model.')
    const card = page.getByRole('region', { name: 'Workspace configuration proposal' })
    await expect(card).toBeVisible({ timeout: 30000 })
    await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
    await expect(card).toContainText('Settings applied. No analysis was run.')
    expect((await snapshot(page, name)).counterfactualRuns.length).toBe(before)
    await approveRun(page, 'Fit the approved causal model.')
    await expect
      .poll(async () => (await snapshot(page, name)).counterfactualRuns.length)
      .toBe(before + 1)
    expect((await snapshot(page, name)).counterfactualRuns.at(-1).configuration).toEqual(
      configuration,
    )
    expect(count()).toBe(2)
    await page.screenshot({ path: `test-results/assistant-${configuration.kind}.png` })
  })
