import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('scoped reads keep assumptions with their study and refuse absent IDs', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readStudy } = await import(new URL('/src/assistant/studyRead.ts', location.href).href)
    const { assistantContext } = await import(
      new URL('/src/assistant/context.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
    const source = parsed.value
    const base = source.studies[0]
    const original = source.identifications[0]
    const studies = ['law', 'kms'].map((name) => ({
      ...base,
      id: name,
      treatment: { ...base.treatment, name },
      outcome: { ...base.outcome, name: 'DriversKilled' },
    }))
    const identifications = studies.map((study) => ({
      ...original,
      id: `${study.id}-identification`,
      study: study.id,
      result: {
        ...original.result,
        basis: [
          {
            kind: 'graph-assumption',
            id: 'no-unmeasured-confounding',
            statement: `No unmeasured common cause of ${study.treatment.name} and DriversKilled remains.`,
          },
        ],
      },
    }))
    const workflow = { ...source, kind: 'profiled', studies, identifications }
    const before = JSON.stringify(workflow)
    const law = readStudy(workflow, { id: 'law', view: 'full' })
    const kms = readStudy(workflow, { id: 'kms', view: 'full' })
    const overview = JSON.parse(assistantContext(workflow))
    return {
      law,
      kms,
      unknown: readStudy(workflow, { id: 'missing' }).kind,
      invalid: readStudy(workflow, { id: ['law', 'kms'] }).kind,
      unchanged: before === JSON.stringify(workflow),
      overviewHasAssumptions: JSON.stringify(overview.studies).includes(
        'No unmeasured common cause',
      ),
      overviewHasAdjustment: overview.studies.every(
        (study: { adjustment: unknown }) => study.adjustment !== null,
      ),
      savedRevision: base.dagRevision,
    }
  })
  expect(result.law.study).toBe('law')
  expect(result.law.specification.dagRevision).toBe(result.savedRevision)
  expect(result.law.identifications).toHaveLength(1)
  expect(result.law.identifications[0].record.study).toBe('law')
  expect(result.law.assumptions).toEqual([
    {
      ...result.law.identifications[0].record.result.basis[0],
      study: 'law',
      identification: 'law-identification',
    },
  ])
  expect(result.law.identifications[0].record.result.basis[0].statement).toContain(
    'of law and DriversKilled',
  )
  expect(result.kms.identifications[0].record.result.basis[0].statement).toContain(
    'of kms and DriversKilled',
  )
  expect(result.unknown).toBe('refused')
  expect(result.invalid).toBe('refused')
  expect(result.unchanged).toBe(true)
  expect(result.overviewHasAssumptions).toBe(false)
  expect(result.overviewHasAdjustment).toBe(true)
})

test('CodeAct requires a successful study read before citing study-record evidence', async ({
  page,
}) => {
  test.setTimeout(90000)
  let requests = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    requests++
    const code =
      requests === 1
        ? 'r = await tool("study", {"id":"missing"})\nassert r["kind"] == "refused"\nawait FINAL_VAR("Unread assumption", ["study-record"])'
        : 'p = await tool("project")\ns = await tool("study", {"id":p["studies"][0]["id"],"view":"full"})\nassert s["kind"] == "study-record"\nassert all(r["record"]["study"] == s["study"] for r in s["identifications"])\nawait FINAL_VAR("Read study " + s["study"], ["study-record"])'
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  const answer = await page.evaluate(async () => {
    const { runAssistant } = await import(new URL('/src/assistant/agent.ts', location.href).href)
    const { readStudy } = await import(new URL('/src/assistant/studyRead.ts', location.href).href)
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
      question: 'Read the first saved study.',
      context: assistantContext(workflow),
      readStudy: (raw: unknown) => readStudy(workflow, raw),
      signal: new AbortController().signal,
      trace: () => {},
      propose: null,
      proposeSql: null,
      requestRun: async () => {
        throw new Error('No run should be requested')
      },
    })
  })
  expect(requests).toBe(2)
  expect(answer.answer).toMatch(/^Read study /)
  expect(answer.evidence).toEqual(['study-record'])
})
