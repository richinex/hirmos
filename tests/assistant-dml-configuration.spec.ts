import { expect, test } from '@playwright/test'
import { ask, connectAssistant } from './assistantTestFlow'

test('DML proposals validate the seed and cannot change the study target', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readEstimatorConfiguration } = await import(new URL('/src/assistant/estimatorConfiguration.ts', location.href).href)
    const { proposeSelection } = await import(new URL('/src/assistant/applicationTools.tsx', location.href).href)
    const current = { kind: 'dml-irm', att: false, seed: 1234 }
    const capability = {
      revision: 'test',
      inspection: {
        choices: [{ id: 'dml-irm', name: 'DML', eligibility: { kind: 'eligible' } }],
        configuration: { current, columns: [] },
      },
      configure: () => {},
      evaluate: () => ({ kind: 'eligible' }),
    }
    return {
      valid: readEstimatorConfiguration({ ...current, seed: 7 }, []).ok,
      invalid: [-1, 1.5, Number.MAX_SAFE_INTEGER + 1].map(seed => readEstimatorConfiguration({ ...current, seed }, []).ok),
      unsupported: readEstimatorConfiguration({ ...current, folds: 10 }, []).ok,
      plrAtt: readEstimatorConfiguration({ ...current, kind: 'dml-plr', att: true }, []).ok,
      changedTarget: proposeSelection(capability, { estimator: 'dml-irm', configuration: { ...current, att: true }, reason: 'Change target' }).ok,
    }
  })
  expect(result).toEqual({ valid: true, invalid: [false, false, false], unsupported: false, plrAtt: false, changedTarget: false })
})

for (const label of ['DML partially linear', 'DML interactive']) {
  test(`${label}: approve seed settings without fitting`, async ({ page }) => {
    await page.goto('/app/projects')
    await page.getByRole('button', { name: 'Open NSW job training and 1978 earnings', exact: true }).click()
    await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Estimation/ }).click()
    await page.getByRole('radio', { name: new RegExp('^' + label) }).click()
    const before = await page.getByRole('heading', { name: /^Runs \(/ }).innerText()
    const seed = page.getByRole('spinbutton', { name: 'Fold seed', exact: true })
    const original = await seed.inputValue()
    await connectAssistant(page, [[
      'p = await tool("project")',
      'schema = await tool("learner_schema")',
      'c = p["estimation"]["configuration"]["current"]',
      'c["seed"] = 73',
      'r = await tool("propose_estimator", {"estimator": c["kind"], "configuration": c, "reason": "Review the fold seed."})',
      'assert r["kind"] == "awaiting-user-approval", r',
      'await FINAL_VAR("Seed proposed, not applied.", ["project-summary"])',
    ].join('\n')])
    await ask(page, 'Set the fold seed to 73 without running the model.')
    const proposal = page.getByRole('region', { name: 'Estimator proposal' })
    await expect(proposal).toBeVisible({ timeout: 30000 })
    await expect(seed).toHaveValue(original)
    await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
    await proposal.screenshot({ path: `/tmp/hirmos-${label.replaceAll(' ', '-')}-proposal.png` })
    await proposal.getByRole('button', { name: 'Apply settings', exact: true }).click()
    await expect(seed).toHaveValue('73')
    await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
  })
}
