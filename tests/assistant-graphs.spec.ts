import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('graph proposals validate operations atomically and refuse stale approval', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { proposeGraph, graphProposalEvent } = await import(
      new URL('/src/assistant/graphProposal.ts', location.href).href
    )
    const { stepWorkflow } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const workflow = { ...bundle.project, kind: 'profiled' }
    const [first, second] = workflow.dagDocuments[0].current.graph.nodes.filter(
      (node: { kind: string }) => node.kind === 'observed',
    )
    const target = { kind: 'create', name: 'Review graph', basis: 'domain-knowledge' }
    const review = {
      summary: 'Propose a causal assumption.',
      checks: ['Review the direction of the arrow.'],
    }
    const arrow = {
      kind: 'add-arrow',
      cause: first.name,
      effect: second.name,
      rationale: 'Proposed mechanism for review.',
      timing: { kind: 'contemporaneous' },
    }
    const make = (operations: unknown[]) =>
      proposeGraph(workflow, { target, operations, ...review })
    const initial = JSON.stringify(workflow)
    const created = make([
      arrow,
      { kind: 'select-variables', treatment: first.name, outcome: second.name },
    ])
    if (!created.ok) throw new Error(created.error)
    const unchanged = JSON.stringify(workflow) === initial
    const event = graphProposalEvent(workflow, created.value)
    if (!event.ok) throw new Error(event.error)
    const saved = stepWorkflow(workflow, event.value)
    const document = saved.dagDocuments.find((item: { name: string }) => item.name === target.name)
    const edit = proposeGraph(saved, {
      target: { kind: 'edit', document: document.id },
      ...review,
      operations: [
        { kind: 'remove-arrow', edge: document.current.graph.edges[0].id },
        { kind: 'add-latent', name: 'Unmeasured cause' },
        { kind: 'rename-latent', variable: 'Unmeasured cause', name: 'Shared cause' },
        {
          kind: 'add-arrow',
          cause: 'Shared cause',
          effect: second.name,
          rationale: null,
          timing: { kind: 'contemporaneous' },
        },
      ],
    })
    return {
      unchanged,
      savedCount: saved.dagDocuments.length - workflow.dagDocuments.length,
      selected: document.exploration,
      stale: graphProposalEvent(saved, created.value).ok,
      cycle: make([arrow, { ...arrow, cause: second.name, effect: first.name }]).ok,
      unknown: make([{ ...arrow, cause: 'Invented column' }]).ok,
      removeObserved: make([{ kind: 'remove-latent', variable: first.name }]).ok,
      latentEndpoint: make([
        { kind: 'add-latent', name: 'U' },
        { kind: 'select-variables', treatment: 'U', outcome: second.name },
      ]).ok,
      edit: edit.ok
        ? {
            edges: edit.value.document.current.graph.edges.length,
            history: edit.value.document.history.length,
            originalHistory: document.history.length,
          }
        : edit,
      oldRevision: saved.studies[0].dagRevision === workflow.studies[0].dagRevision,
    }
  })
  expect(result).toMatchObject({
    unchanged: true,
    savedCount: 1,
    stale: false,
    cycle: false,
    unknown: false,
    removeObserved: false,
    latentEndpoint: false,
    oldRevision: true,
  })
  expect(result.selected.treatment).toBeTruthy()
  expect(result.selected.outcome).toBeTruthy()
  expect(result.edit.edges).toBe(1)
  expect(result.edit.history).toBeGreaterThan(result.edit.originalHistory)
})

test('CodeAct creates a graph only after approval and can inspect the saved revision', async ({
  page,
}) => {
  test.setTimeout(90000)
  let count = 0
  let finishAnswer!: () => void
  const answerGate = new Promise<void>((resolve) => {
    finishAnswer = resolve
  })
  await page.route('https://openrouter.ai/api/v1/chat/completions', async (route) => {
    count++
    if (count === 2) {
      await answerGate
      return route.fulfill(
        openRouterStream({
          choices: [
            {
              message: {
                content:
                  '```python\nawait tool("finish", {"answer":"Review the graph proposal before saving it.","evidence":["project-summary"]})\n```',
              },
            },
          ],
        }),
      )
    }
    const code =
      count === 1
        ? [
            'p = await tool("project")',
            'print(await tool("graph_schema"))',
            'g = p["graphs"][0]["graph"]',
            'edge = g["edges"][0]',
            'names = {n["id"]: n["name"] for n in g["nodes"]}',
            'print(await tool("propose_graph", {"target":{"kind":"create","name":"Assistant reviewed graph","basis":"domain-knowledge"},"summary":"Create a graph with one proposed arrow.","checks":["Review whether the proposed direction is justified."],"operations":[{"kind":"add-arrow","cause":names[edge["cause"]],"effect":names[edge["effect"]],"rationale":"Proposed assumption for user review.","timing":{"kind":"contemporaneous"}}]}))',
          ].join('\n')
        : [
            'p = await tool("project")',
            'g = next(g for g in p["graphs"] if g["name"] == "Assistant reviewed graph")',
            'assert len(g["graph"]["edges"]) == 1',
            'await tool("finish", {"answer":"Saved graph: " + g["name"],"evidence":["project-summary"]})',
          ].join('\n')
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
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
  await page.getByLabel('Ask a question').fill('Propose a graph with one arrow for review.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const proposal = page.getByRole('region', { name: 'Graph proposal' })
  await expect(proposal).toBeVisible({ timeout: 30000 })
  await expect(proposal).not.toContainText('Graph saved.')
  await expect(
    proposal.getByRole('button', { name: 'Apply graph changes', exact: true }),
  ).toBeDisabled()
  await expect(
    proposal.getByText('Wait for the answer to finish, or stop it before applying changes.'),
  ).toBeVisible()
  finishAnswer()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
  await expect(
    proposal.getByRole('button', { name: 'Apply graph changes', exact: true }),
  ).toBeEnabled()
  await expect(proposal.getByRole('list', { name: 'Proposed graph changes' })).toContainText(
    'Proposed assumption for user review.',
  )
  await expect(proposal.getByRole('list', { name: 'Proposed graph changes' })).not.toContainText(
    'add-arrow',
  )
  await expect(proposal.getByRole('list', { name: 'Proposed graph changes' })).not.toContainText(
    'same period',
  )
  await proposal.getByRole('button', { name: 'Apply graph changes', exact: true }).click()
  await expect(proposal).toContainText('Graph saved.')
  await page.getByLabel('Ask a question').fill('Inspect the graph that I just saved.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Saved graph: Assistant reviewed graph',
    { timeout: 30000 },
  )
  const conversation = page.getByRole('region', { name: 'Conversation turn', exact: true })
  await expect(conversation).toHaveCount(2)
  await expect(conversation.first().getByRole('region', { name: 'Graph proposal' })).toBeVisible()
  await expect(conversation.last().getByRole('region', { name: 'Graph proposal' })).toHaveCount(0)
  await page.screenshot({ path: '/tmp/hirmos-graph-proposal.png', fullPage: true })
})
