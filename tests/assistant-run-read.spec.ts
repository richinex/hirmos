import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('saved reads preserve values and original provenance without changing the project', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const workflow = { ...parsed.value, kind: 'profiled' }
    const run = workflow.estimationRuns[0]
    const study = workflow.studies.find((s: { id: string }) => s.id === run.study)
    const document = workflow.dagDocuments.find((d: { id: string }) => d.id === study.dagDocument)
    const revision = document.audit.find((r: { id: string }) => r.id === study.dagRevision)
    document.current = { ...revision, id: 'later-revision', graph: { nodes: [], edges: [] } }
    const before = JSON.stringify(workflow)
    const summary = readRun(workflow, { id: run.id })
    const specification = readRun(workflow, { id: run.id, view: 'specification' })
    const evidence = readRun(workflow, { id: run.id, view: 'evidence' })
    const unchanged = before === JSON.stringify(workflow)
    document.audit = []
    workflow.prepared = { ...workflow.prepared, id: 'new-data' }
    const missing = readRun(workflow, { id: run.id, view: 'specification' })
    return {
      summary,
      specification,
      evidence,
      missing,
      unchanged,
      expected: {
        estimate: run.estimate,
        evidence: run.evidence,
        configuration: run.configuration,
        revision,
      },
      unknown: readRun(workflow, { id: 'missing' }),
      invalid: readRun(workflow, { id: run.id, view: 'current-controls' }),
    }
  })
  expect(result.summary.kind).toBe('saved-run')
  expect(result.summary.interval).toEqual(result.expected.estimate.interval)
  expect(result.summary.sample).toEqual(result.expected.estimate.sample)
  expect(result.summary.standardError).toBe(result.expected.estimate.standardError)
  expect(result.summary.provenance.graph).toBe('earlier')
  expect(result.specification.graphRevision).toEqual(result.expected.revision)
  expect(result.specification.configuration).toEqual(result.expected.configuration)
  expect(result.evidence.estimate).toEqual(result.expected.estimate)
  expect(result.evidence.evidence).toEqual(result.expected.evidence)
  expect(result.missing.graphRevision).toBeNull()
  expect(result.missing.preparedDatasetRecord).toBeNull()
  expect(result.missing.provenance.graph).toBe('missing-revision')
  expect(result.unknown.kind).toBe('refused')
  expect(result.invalid.kind).toBe('refused')
  expect(result.unchanged).toBe(true)
})

test('CodeAct cannot cite a refused saved read', async ({ page }) => {
  test.setTimeout(90000)
  let requests = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    requests++
    const code =
      requests === 1
        ? 'r = await tool("run", {"id":"missing"})\nassert r["kind"] == "refused"\nawait FINAL_VAR("Unread result", ["saved-run"])'
        : 'p = await tool("project")\nr = await tool("run", {"id":p["savedEstimationRuns"][0]["id"]})\nassert r["kind"] == "saved-run"\nawait FINAL_VAR("Read saved run " + r["run"], ["saved-run"])'
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  const answer = await page.evaluate(async () => {
    const { runAssistant } = await import(new URL('/src/assistant/agent.ts', location.href).href)
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { assistantContext } = await import(
      new URL('/src/assistant/context.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const workflow = { ...parsed.value, kind: 'profiled' }
    return runAssistant({
      key: 'test-key',
      model: 'test/model',
      question: 'Read a saved result.',
      context: assistantContext(workflow),
      readRun: (raw: unknown) => readRun(workflow, raw),
      signal: new AbortController().signal,
      trace: () => {},
      propose: null,
      proposeSql: null,
      requestRun: async () => {
        throw new Error('Reading must not execute an analysis')
      },
    })
  })
  expect(requests).toBe(2)
  expect(answer.answer).toMatch(/^Read saved run /)
  expect(answer.evidence).toEqual(['saved-run'])
})

test('a run completed during a question is immediately readable and identified in refreshed context', async ({
  page,
}) => {
  const code = [
    'before = await tool("project")',
    'assert before["columnCount"] == len(before["columns"])',
    'assert before["availableWorkspaceSettings"][0]["title"] == "Test settings"',
    'receipt = await tool("run_analysis", {"key":"test-run", "reason":"Run the reviewed test specification"})',
    'after = await tool("project")',
    'assert after["savedEstimates"] == before["savedEstimates"] + 1',
    'assert after["latestAnalysisThisQuestion"]["runId"] == receipt["runId"]',
    'result = await tool("run", {"id":receipt["runId"]})',
    'assert result["kind"] == "saved-run", result',
    'assert result["run"] == "new-run-this-question"',
    'await FINAL_VAR("Read the newly completed run.", ["saved-run"])',
  ].join('\n')
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app')
  const answer = await page.evaluate(async () => {
    const { runAssistant } = await import(new URL('/src/assistant/agent.ts', location.href).href)
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { assistantContext } = await import(
      new URL('/src/assistant/context.ts', location.href).href
    )
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    let workflow = { ...parsed.value, kind: 'profiled' }
    return runAssistant({
      key: 'test-key',
      model: 'test/model',
      question: 'Run and read the result.',
      context: assistantContext(workflow),
      readProject: () => assistantContext(workflow),
      readRun: (raw: unknown) => readRun(workflow, raw),
      workspaceSettings: () => [{ key: 'test-settings', title: 'Test settings' }],
      signal: new AbortController().signal,
      trace: () => {},
      propose: null,
      proposeSql: null,
      requestRun: async () => {
        const run = { ...workflow.estimationRuns[0], id: 'new-run-this-question' }
        workflow = { ...workflow, estimationRuns: [...workflow.estimationRuns, run] }
        return {
          kind: 'completed',
          runId: run.id,
          method: 'Test fit',
          summary: 'Test result saved.',
        }
      },
    })
  })
  expect(answer.answer).toBe('Read the newly completed run.')
})

test('the assistant reads a saved result through the UI without creating a run', async ({
  page,
}) => {
  const code = [
    'p = await tool("project")',
    'r = await tool("run", {"id":p["savedEstimationRuns"][0]["id"]})',
    'assert r["kind"] == "saved-run", r',
    'await FINAL_VAR("Saved result: " + r["subject"]["treatment"] + " on " + r["subject"]["outcome"], ["saved-run"])',
  ].join('\n')
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app/projects')
  await page
    .getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  const snapshot = () =>
    page.evaluate(async () => {
      const { listProjects, loadProject } = await import(
        new URL('/src/data/projectStore.ts', location.href).href
      )
      const headers = await listProjects()
      return JSON.stringify(
        await Promise.all(
          headers.map(async (header: { id: string }) => {
            const loaded = await loadProject(header.id)
            if (!loaded.ok) throw new Error('Could not read saved project')
            return { studies: loaded.value.studies, runs: loaded.value.estimationRuns }
          }),
        ),
      )
    })
  const before = await snapshot()
  await page
    .getByLabel('Ask a question')
    .fill('Explain my first saved result without running anything.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Saved result:',
    { timeout: 45000 },
  )
  expect(await snapshot()).toBe(before)
})
