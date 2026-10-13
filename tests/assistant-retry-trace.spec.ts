import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('trace accounts for replies without code and output-limit retries', async ({ page }) => {
  test.setTimeout(90000)
  let requests = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    requests++
    const code = [
      'p = await tool("project")',
      's = await tool("study", {"id":p["studies"][0]["id"]})',
      'a = next(a for a in s["assumptions"] if a["id"] == "no-unmeasured-confounding")',
      'assert a["study"] == s["study"]',
      'await FINAL_VAR(a["statement"] + " (study " + a["study"] + ")", ["study-record"])',
    ].join('\n')
    return route.fulfill(
      openRouterStream({
        choices: [
          {
            message: {
              content:
                requests === 1
                  ? 'I will inspect the saved study.'
                  : requests === 2
                    ? '```python\np = await tool('
                    : '```python\n' + code + '\n```',
            },
            finish_reason: requests === 2 ? 'length' : 'stop',
          },
        ],
      }),
    )
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Quote the saved no-confounding assumption.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'No unmeasured common cause',
    { timeout: 30000 },
  )
  expect(requests).toBe(3)
  await page.getByRole('button', { name: /1 block, 3 requests/ }).click()
  await expect(page.getByText(/Request 1: The reply contained no Python block/)).toBeVisible()
  await expect(page.getByText(/Request 2: Reply reached the output-token limit/)).toBeVisible()
  await page
    .getByText(/Request 1: The reply contained no Python block/)
    .screenshot({ path: '/tmp/hirmos-retry-trace.png' })
})
