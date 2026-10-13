import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('planning settings, calculation approval and saved reading are separate', async ({ page }) => {
  test.setTimeout(90000)
  let request = 0
  const codes = [
    's = await tool("workspace_settings", {"key":"planning"})\nc = s["current"]\nc["target"] = {"kind":"power", "n":80, "effect":0.5}\np = await tool("propose_workspace", {"key":"planning", "configuration":c, "reason":"Check power for 80 observations in the first group."})\nassert p["kind"] == "awaiting-user-review"\nawait FINAL_VAR("Review the planning settings.", ["user-question"])',
    'r = await tool("run_analysis", {"key":"planning", "reason":"Calculate power with the reviewed settings."})\nassert r["kind"] == "calculated", r\nawait FINAL_VAR("Planning calculation saved.", ["approved-run"])',
    'r = await tool("run", {"workspace":"planning", "id":"current", "view":"evidence"})\nassert r["kind"] == "saved-calculation", r\nassert r["evidence"]["n"] == 80\nawait FINAL_VAR("Read the saved calculation for 80 observations in group 1.", ["saved-run"])',
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
  await page.getByRole('button', { name: /^Data studio/ }).click()
  const planning = page.getByTestId('power-planning')
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  const ask = async (question: string) => {
    await page.getByLabel('Ask a question').fill(question)
    await page.getByRole('button', { name: 'Send', exact: true }).click()
  }
  await ask('Propose power for n 80 and effect 0.5.')
  const card = page.getByRole('region', { name: 'Workspace configuration proposal' })
  await expect(card).toBeVisible({ timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  await expect(planning.getByTestId('power-result')).toHaveCount(0)
  await ask('Calculate the reviewed plan.')
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval).toBeVisible({ timeout: 30000 })
  await expect(planning.getByTestId('power-result')).toHaveCount(0)
  await approval.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(approval).toContainText('Calculation saved.', { timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Planning calculation saved.',
  )
  await expect(planning.getByTestId('power-result')).toBeVisible()
  await ask('Read the saved planning result.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Read the saved calculation',
    { timeout: 30000 },
  )
  await page.screenshot({ path: '/tmp/hirmos-assistant-planning.png', fullPage: true })
})
