import { openRouterStream } from './openRouterStream'
import { expect, test, type Page } from '@playwright/test'
import { chapter, choose, prepare, fixture } from './examples/support'

test.describe.configure({ timeout: 90000 })

test('the local key loads only on request and is not sent to a provider', async ({ page }) => {
  let providerCalls = 0
  await page.route('https://openrouter.ai/**', (route) => {
    providerCalls++
    return route.abort()
  })
  await page.goto('/app')
  const denied = await page.evaluate(
    async () => (await fetch('/__hirmos/local-openrouter-key')).status,
  )
  expect(denied).toBe(403)
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  expect((await page.getByLabel('OpenRouter API key').inputValue()).length === 0).toBe(true)
  await page.getByRole('button', { name: 'Load local key', exact: true }).click()
  await expect
    .poll(async () => (await page.getByLabel('OpenRouter API key').inputValue()).length > 0)
    .toBe(true)
  expect(providerCalls).toBe(0)
  await page
    .getByRole('complementary', { name: 'Assistant' })
    .getByRole('button', { name: 'Close the assistant' })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  expect((await page.getByLabel('OpenRouter API key').inputValue()).length === 0).toBe(true)
})

async function connect(page: Page, code: string) {
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Use the current specification.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
}

const runCode = (key: string) => `p = await tool("project")
args = {"key":"${key}","reason":"Run this specification after approval."}
if "study" in p["runs"]["${key}"]: args["subject"] = p["runs"]["${key}"]["study"]["subject"]
result = await tool("run_analysis", args)
print(result)
await tool("finish", {"answer":"Run status: " + result["kind"], "evidence":["approved-run"]})`

test('the assistant runs an ARDL through the existing controls and saves its result', async ({
  page,
}) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Assistant ARDL')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  const csv = [
    'date,y',
    ...Array.from(
      { length: 80 },
      (_, i) =>
        `${new Date(Date.UTC(2020, 0, i + 1)).toISOString().slice(0, 10)},${10 + Math.sin(i / 4) + Math.cos(i * 1.7) * 0.2}`,
    ),
  ].join('\n')
  await page
    .locator('input[type=file]')
    .first()
    .setInputFiles({ name: 'series.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await prepare(page, {
    structure: 'time series',
    time: 'date',
    frequency: 'Daily',
    columns: ['y'],
  })
  await chapter(page, /Time-series analysis/)
  await page.getByRole('radio', { name: 'ARDL', exact: true }).click()
  await choose(page, 'Outcome series', 'y')
  await page.getByRole('radio', { name: 'Specify lags', exact: true }).click()
  await connect(page, runCode('ardl'))
  await expect(page.getByRole('region', { name: 'Analysis approval' })).toBeVisible({
    timeout: 30000,
  })
  await expect(page.getByRole('region', { name: 'Time-series result', exact: true })).toHaveCount(0)
  await page.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Run status: completed',
    { timeout: 30000 },
  )
  await expect(page.getByRole('region', { name: 'Time-series result', exact: true })).toBeVisible()
  await page.screenshot({ path: '/tmp/hirmos-assistant-ardl.png' })
})

test('the assistant runs the existing survival comparison after approval', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Assistant survival')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').first().setInputFiles(fixture('ab-survival.csv'))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await prepare(page, { structure: 'cross-section', columns: ['tstatus', 'status', 'group'] })
  await chapter(page, /Survival analysis/)
  await page.getByRole('radio', { name: 'Compare groups' }).click()
  await choose(page, 'Duration', 'tstatus')
  await choose(page, 'Event 1 observed, 0 censored', 'status')
  await choose(page, 'Group 0 or 1', 'group')
  await page.getByRole('spinbutton', { name: 'Compare through time' }).fill('7')
  await connect(page, runCode('survival'))
  await expect(page.getByRole('region', { name: 'Analysis approval' })).toBeVisible({
    timeout: 30000,
  })
  await page.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Run status: completed',
    { timeout: 30000 },
  )
  await expect(
    page.getByRole('heading', { name: 'Group 1 compared with group 0' }).first(),
  ).toBeVisible()
  await page.screenshot({ path: '/tmp/hirmos-assistant-survival.png' })
})

test('a changed specification cancels approval without running', async ({ page }) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await chapter(page, /Estimation/)
  await expect(page.getByRole('button', { name: /^Run adjusted linear regression/ })).toBeEnabled({
    timeout: 30000,
  })
  const before = await page.getByRole('heading', { name: /^Runs \(/ }).innerText()
  await connect(page, runCode('estimation'))
  await expect(page.getByRole('button', { name: 'Run and share summary' })).toBeVisible({
    timeout: 30000,
  })
  await page.getByRole('radio', { name: /^DML partially linear/ }).click()
  await expect(page.getByRole('button', { name: 'Run and share summary' })).toHaveCount(0)
  await expect(page.getByRole('region', { name: 'Analysis approval' })).toContainText(
    'Run cancelled.',
  )
  await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
})

test('SQL proposals load without execution and use the existing reviewed SQL path', async ({
  page,
}) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Assistant SQL')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
  await page.getByRole('button', { name: 'Open empty SQL editor' }).click()
  await expect(page.getByRole('button', { name: 'Open SQL file' })).toBeEnabled({ timeout: 30000 })
  const sql =
    'CREATE OR REPLACE VIEW hirmos_prepared AS SELECT 40 AS doubled; SELECT * FROM hirmos_prepared;'
  await connect(
    page,
    `print(await tool("propose_sql", {"statement":${JSON.stringify(sql)},"summary":"Create a small demonstration table.","checks":["Check the output columns and row count."]}))\nawait tool("finish", {"answer":"Review the SQL before running it.","evidence":["user-question"]})`,
  )
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible({
    timeout: 30000,
  })
  await page.getByRole('button', { name: 'Load SQL for review', exact: true }).click()
  await expect(page.getByRole('textbox', { name: 'SQL file contents' })).toHaveValue(sql)
  await expect(page.getByText('Final statement result', { exact: true })).toHaveCount(0)
  await page.getByRole('button', { name: 'Run SQL', exact: true }).click()
  await expect(
    page
      .getByRole('dialog', { name: 'SQL file', exact: true })
      .getByRole('cell', { name: '40', exact: true }),
  ).toBeVisible()
})
