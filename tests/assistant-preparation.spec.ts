import { openRouterStream } from './openRouterStream'
import { test, expect, type Page } from '@playwright/test'

test.describe.configure({ timeout: 90000 })

async function connect(page: Page) {
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
}
async function ask(page: Page, question: string) {
  await page.getByLabel('Ask a question').fill(question)
  await page.getByRole('button', { name: 'Send', exact: true }).click()
}
const fixture = {
  name: 'monthly.csv',
  mimeType: 'text/csv',
  buffer: Buffer.from(
    [
      'period,DriversKilled,kms',
      ...Array.from({ length: 192 }, (_, i) => `${i + 1},${100 + i},${1000 + i * 5}`),
    ].join('\n'),
  ),
}

test('local profiling handles quoted names, bounded categories, missing values and empty tables', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    // @ts-expect-error Vite browser module.
    const { isolatedDuckDbEngine } = await import('/src/data/duckdb.ts')
    // @ts-expect-error Vite browser module.
    const { profileTable, compareProfiles } = await import('/src/assistant/preparation.ts')
    const engine = await isolatedDuckDbEngine()
    const c = await engine.db.connect()
    try {
      await c.query(
        `CREATE TABLE sample AS SELECT i AS "a\"\"b", CASE WHEN i=0 THEN NULL ELSE i END AS x, 'group-' || i AS category FROM range(25) t(i)`,
      )
      const signal = AbortSignal.timeout(15000)
      const before = await profileTable(
        engine.db,
        ['sample'],
        { table: 'sample', columns: ['a"b', 'x', 'category'], categoryColumn: 'category' },
        signal,
      )
      await c.query('CREATE TABLE empty AS SELECT * FROM sample WHERE false')
      const empty = await profileTable(
        engine.db,
        ['empty'],
        { table: 'empty', columns: ['x'] },
        signal,
      )
      let refused = false
      try {
        await profileTable(
          engine.db,
          ['sample'],
          { table: 'sample; DROP TABLE sample', columns: [] },
          signal,
        )
      } catch {
        refused = true
      }
      return { before, empty, refused, comparison: compareProfiles(before, empty) }
    } finally {
      await c.close()
      await engine.db.terminate()
    }
  })
  expect(result.before.rows).toBe(25)
  expect(result.before.summaries[1].nulls).toBe(1)
  expect(result.before.summaries[2].categories).toHaveLength(20)
  expect(result.before.summaries[2].formats).toHaveLength(2)
  expect(result.empty.rows).toBe(0)
  expect(result.empty.summaries[0].minimum).toBeNull()
  expect(result.refused).toBe(true)
  expect(result.comparison.rowDifference).toBe(-25)
})

test('SQL CodeAct profiles, proposes and verifies a reviewed log and lag without repeated consent', async ({
  page,
}) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Assistant preparation')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Assistant preparation; open Projects' })).toBeVisible()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).check()
  await expect(page.getByRole('radio', { name: 'Prepare with SQL' })).toBeChecked()
  await page.locator('input[type="file"][multiple]:not([webkitdirectory])').setInputFiles(fixture)
  await expect(page.getByRole('button', { name: 'Open SQL file' })).toBeEnabled({ timeout: 30000 })
  let calls = 0
  const sql =
    'CREATE OR REPLACE VIEW hirmos_prepared AS SELECT *, ln(DriversKilled) AS log_deaths, lag(kms, 12) OVER (ORDER BY period) AS kms_lag12 FROM monthly; SELECT count(*) FROM hirmos_prepared;'
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    const code =
      calls++ === 0
        ? `print(await tool("project"))
baseline = await tool("profile_table", {"table":"monthly","columns":["period","DriversKilled","kms"]})
print(baseline)
print(await tool("propose_sql", {"statement":${JSON.stringify(sql)},"summary":"Log the outcome and lag kms by 12 ordered periods.","checks":["Check that 192 rows remain.","Check for 12 leading nulls in the lag column."]}))
await tool("finish", {"answer":"Baseline profile " + baseline["receipt"]["id"] + ": 192 rows. Review the SQL before execution.","evidence":["data-summary"]})`
        : `print(await tool("project"))
result = await tool("profile_table", {"table":"hirmos_prepared","columns":["period","DriversKilled","kms","log_deaths","kms_lag12"],"compareTo":"BASELINE"})
print(result)
assert result["kind"] == "profiled", result
assert result["comparison"]["rowDifference"] == 0
assert result["receipt"]["profile"]["summaries"][-1]["nulls"] == 12
await tool("finish", {"answer":"Verified 192 rows before and after, with 12 expected leading lag nulls. Counts alone do not verify row identity.","evidence":["data-summary"]})`
    const question = route.request().postDataJSON().messages.at(-1).content as string
    const id = question.match(/Baseline receipt: ([0-9a-f-]{36})/)?.[1] ?? ''
    return route.fulfill(
      openRouterStream({
        choices: [{ message: { content: '```python\n' + code.replace('BASELINE', id) + '\n```' } }],
      }),
    )
  })
  await connect(page)
  await ask(page, 'Profile the input and propose log outcome and a 12-period lag.')
  const answer = page.getByRole('region', { name: 'Assistant answer' })
  await expect(answer).toContainText('Baseline profile', { timeout: 45000 })
  const id = (await answer.innerText()).match(/[0-9a-f]{8}-[0-9a-f-]{27}/)?.[0]
  expect(id).toBeTruthy()
  await page.getByRole('button', { name: 'Load SQL for review', exact: true }).click()
  await expect(page.getByRole('textbox', { name: 'SQL file contents' })).toHaveValue(sql)
  await expect(page.getByText('Final statement result', { exact: true })).toHaveCount(0)
  await page.getByRole('button', { name: 'Run SQL', exact: true }).click()
  await expect(page.getByText('Final statement result', { exact: true })).toBeVisible()
  await page.keyboard.press('Escape')
  await ask(page, `Compare the output. Baseline receipt: ${id}`)
  await expect(answer.last()).toContainText('Verified 192 rows', { timeout: 45000 })
  expect(calls).toBe(2)
  await page.screenshot({ path: '/tmp/hirmos-profile-comparison.png', fullPage: true })
})

test('pipeline proposal uses existing blocks and only evaluates after approval', async ({
  page,
}) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Assistant pipeline')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.getByRole('button', { name: 'Open the editor', exact: true }).click()
  const input = page.getByTestId('block-input-1')
  await input.click({ position: { x: 20, y: 8 } })
  await page.getByLabel('File for this card').setInputFiles(fixture)
  await expect(input).toContainText('192 rows, 3 columns', { timeout: 30000 })
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({
        choices: [
          {
            message: {
              content: `\`\`\`python
p = await tool("project")
schema = await tool("pipeline_schema")
assert schema["expressions"]["dialect"] == "DuckDB SQL"
print(schema)
g = p["preparation"]["graph"]
source = next(n for n in g["nodes"] if n["block"]["kind"] == "input")
outputs = [n for n in g["nodes"] if n["block"]["kind"] == "output"]
out = outputs[0] if outputs else {"id":"out","block":{"kind":"output"},"position":{"x":500,"y":100}}
derive = {"id":"assistant-log","block":{"kind":"derive-columns","columns":[{"name":"log_deaths","expression":"ln(DriversKilled)"},{"name":"lag_kms","expression":"LAG(kms, 12) OVER (ORDER BY period)"}]},"position":{"x":260,"y":100}}
graph = {"nodes":[source, derive, out],"edges":[{"from":source["id"],"to":derive["id"],"port":0},{"from":derive["id"],"to":out["id"],"port":0}]}
print(await tool("propose_pipeline", {"graph":graph,"summary":"Add log deaths without dropping input rows.","checks":["Compare the output row count with the input."]}))
await tool("finish", {"answer":"Review the proposed derive block before evaluating it.","evidence":["project-summary"]})
\`\`\``,
            },
          },
        ],
      }),
    ),
  )
  await connect(page)
  await ask(page, 'Add a log deaths block.')
  await expect(
    page.getByRole('button', { name: 'Apply and evaluate pipeline', exact: true }),
  ).toBeVisible({
    timeout: 45000,
  })
  await expect(page.getByTestId('block-assistant-log')).toHaveCount(0)
  await page.getByRole('button', { name: 'Apply and evaluate pipeline', exact: true }).click()
  await expect(page.getByTestId('block-assistant-log')).toContainText('192 rows, 5 columns', {
    timeout: 30000,
  })
  await expect(page.getByRole('heading', { name: 'Source selected', exact: true })).toHaveCount(0)
  await page.screenshot({ path: '/tmp/hirmos-pipeline-proposal.png', fullPage: true })
})

test('workspace revisions and project scopes invalidate profiles and pipeline approval', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    // @ts-expect-error Vite browser module.
    const { createAssistantBridge } = await import('/src/assistant/applicationTools.tsx')
    // @ts-expect-error Vite browser module.
    const { proposePipeline, pipelineSchema } = await import('/src/assistant/preparation.ts')
    const bridge = createAssistantBridge()
    const profile = {
      table: 'input',
      rows: 2,
      columns: [{ name: 'x', type: 'DOUBLE' }],
      summaries: [],
    }
    const capability = {
      kind: 'sql',
      revision: 'a',
      inspection: { kind: 'sql', tables: ['input'] },
      profile: async () => profile,
    }
    bridge.scopeProfiles('a')
    bridge.publishPreparation(capability)
    const baseline = await bridge.profile(
      'a',
      { table: 'input', columns: [] },
      new AbortController().signal,
    )
    bridge.publishPreparation({ ...capability, revision: 'b' })
    const stale = await bridge.profile(
      'a',
      { table: 'input', columns: [] },
      new AbortController().signal,
    )
    bridge.scopeProfiles('b')
    const other = await bridge.profile(
      'b',
      { table: 'input', columns: [], compareTo: baseline.receipt.id },
      new AbortController().signal,
    )
    let resolve: (() => void) | undefined
    const gate = new Promise<void>((done) => {
      resolve = done
    })
    bridge.publishPreparation({
      ...capability,
      profile: async () => {
        await gate
        return profile
      },
    })
    const pending = bridge.profile(
      'a',
      { table: 'input', columns: [] },
      new AbortController().signal,
    )
    bridge.publishPreparation({ ...capability, revision: 'c' })
    resolve?.()
    const changed = await pending
    let applied = 0
    bridge.publishPreparation({
      kind: 'pipeline',
      revision: 'new',
      inspection: { kind: 'pipeline', graph: { nodes: [], edges: [] }, tables: [], outputs: [] },
      inputs: [],
      apply: () => {
        applied++
      },
      profile: async () => profile,
    })
    const proposal = bridge.applyPipeline({
      kind: 'pipeline-draft',
      revision: 'old',
      graph: { nodes: [], edges: [] },
      summary: 'stale',
      checks: ['Compare row counts.'],
    })
    const invalid = proposePipeline(bridge.store.getState().preparation, {
      graph: {
        nodes: [
          {
            id: 'input',
            block: { kind: 'input', file: { kind: 'chosen', alias: 'unknown' } },
            position: { x: 0, y: 0 },
          },
          { id: 'out', block: { kind: 'output' }, position: { x: 0, y: 100 } },
        ],
        edges: [{ from: 'input', to: 'out', port: 0 }],
      },
      summary: 'unknown file',
      checks: ['Check the input file.'],
    })
    return {
      baseline: baseline.kind,
      stale: stale.kind,
      other: other.kind,
      changed: changed.kind,
      proposal,
      applied,
      invalid: invalid.ok,
      schema: pipelineSchema({ blocks: ['derive-columns'] }),
    }
  })
  expect(result).toMatchObject({
    baseline: 'profiled',
    stale: 'refused',
    other: 'refused',
    changed: 'refused',
    proposal: { ok: false, error: 'stale' },
    applied: 0,
    invalid: false,
  })
  expect(result.schema.blocks).toHaveLength(1)
})
