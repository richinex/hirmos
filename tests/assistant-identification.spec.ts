import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('approved identification returns adjustment evidence without silently recording a set', async ({
  page,
  isMobile,
}) => {
  test.setTimeout(90000)
  const code = [
    'project = await tool("project")',
    'assert len(project["graphs"]) > 0',
    'result = await tool("run_analysis", {"key":"identification", "subject":project["runs"]["identification"]["study"]["subject"], "reason":"Identify the current question, leaving adjustment selection to me."})',
    'print(result)',
    'import json',
    'summary = json.loads(result["summary"])',
    'assert result["summary"].index("adjustmentAnalysis") < result["summary"].index("evidence")',
    'assert summary["adjustmentAnalysis"]["kind"] == "ready"',
    'recommendation = summary["adjustmentAnalysis"]["analysis"]["recommendation"]',
    'assert recommendation["kind"] == "available"',
    'assert recommendation["guarantee"] in ["established", "notEstablished"]',
    'assert set(summary["nodeNames"][i] for i in recommendation["nodes"]) == {"team_experience", "project_complexity", "release_pressure"}',
    'await tool("finish", {"answer":result["kind"] + ": " + result.get("reason", "Saved identification"),"evidence":["approved-run"]})',
  ].join('\n')
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  if (isMobile) await page.getByRole('button', { name: 'Expand section list', exact: true }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Study design/ })
    .click()
  await expect(page.getByRole('button', { name: 'Identify the effect', exact: true })).toBeEnabled()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page
    .getByLabel('Ask a question')
    .fill('Identify the effect and let me choose the adjustment set.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('heading', { name: 'Choose a valid adjustment set' })).toHaveCount(0)
  await approval.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'review-required',
    { timeout: 30000 },
  )
  await expect(approval).toContainText('the study has not been saved yet')
  await expect(page.getByRole('heading', { name: 'Choose a valid adjustment set' })).toBeVisible()
  await page.screenshot({ path: '/tmp/hirmos-identification-review.png', fullPage: true })
})
