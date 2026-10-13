import { expect, test } from '@playwright/test'

test('preparation proposals preserve review checks, reject malformed reviews and do not execute', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    // @ts-expect-error Vite serves application modules in the browser.
    const tools = await import('/src/assistant/applicationTools.tsx')
    const { proposeSql, createAssistantBridge } = tools
    // @ts-expect-error Vite serves application modules in the browser.
    const { proposePipeline } = await import('/src/assistant/preparation.ts')
    let loaded = 0
    let applied = 0
    const sql = {
      revision: 'sql-1',
      inspection: { inputs: [], outputView: 'output' },
      load: async () => {
        loaded++
      },
    }
    const graph = {
      nodes: [{ id: 'script', block: { kind: 'output' }, position: { x: 0, y: 0 } }],
      edges: [],
    }
    const pipeline = {
      kind: 'pipeline',
      revision: 'pipeline-1',
      inputs: [],
      inspection: { kind: 'pipeline', graph, tables: [], outputs: [] },
      apply: () => {
        applied++
      },
      profile: async () => {
        throw new Error('Not used')
      },
    }
    const review = {
      summary: '  Add a derived column.  ',
      checks: ['  Check missing values.  ', 'Compare row counts.'],
    }
    const accepted = proposeSql(sql, { statement: 'SELECT 1', ...review })
    const invalidReviews = [
      { reason: 'Legacy prose' },
      { summary: 'Change' },
      { summary: 'Change', checks: [] },
      { summary: 'Change', checks: ['   '] },
      { summary: 'Change\nMore', checks: ['Check rows.'] },
      { summary: 'Change', checks: [{ text: 'Check rows.', checked: true }] },
      { summary: 'Change', checks: ['Check rows.'], checked: true },
      { summary: 'Change', checks: Array(13).fill('Check rows.') },
      { summary: 'Change', checks: ['x'.repeat(401)] },
    ]
    const refused = invalidReviews.map((item) => ({
      sql: proposeSql(sql, { statement: 'SELECT 1', ...item }).ok,
      pipeline: proposePipeline(pipeline, { graph, ...item }).ok,
    }))
    const beforeApproval = { loaded, applied }
    const bridge = createAssistantBridge()
    bridge.publishSql(sql)
    const load = accepted.ok ? await bridge.loadSql(accepted.value) : null
    return { accepted, refused, beforeApproval, load, loaded, applied }
  })
  expect(result.accepted).toMatchObject({
    ok: true,
    value: {
      summary: 'Add a derived column.',
      checks: ['Check missing values.', 'Compare row counts.'],
    },
  })
  expect(result.refused).toEqual(Array(9).fill({ sql: false, pipeline: false }))
  expect(result.beforeApproval).toEqual({ loaded: 0, applied: 0 })
  expect(result.load).toEqual({ ok: true, value: undefined })
  expect(result.loaded).toBe(1)
  expect(result.applied).toBe(0)
})
