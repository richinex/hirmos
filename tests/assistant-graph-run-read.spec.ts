import { expect, test } from '@playwright/test'

test('graph readers retain recorded revisions and distinguish SWIG assessments from queries', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const identity = {
      dagDocument: 'dag',
      dagRevision: 'original',
      preparedDataset: 'data',
      createdAt: '2026-01-01T00:00:00Z',
    }
    const original = { id: 'original', graph: { nodes: ['original-node'], edges: [] } }
    const query = {
      ...identity,
      id: 'query',
      kind: 'network-query',
      specification: { names: ['Outcome'] },
      result: { probabilities: [0.2, 0.8] },
    }
    const swig = {
      ...identity,
      id: 'swig',
      kind: 'swig-analysis',
      names: ['Outcome'],
      rationale: 'Saved assumptions',
      specification: { construction: { kind: 'swig' } },
      result: { conclusion: { kind: 'notRequested' } },
    }
    const workflow = {
      kind: 'profiled',
      prepared: { id: 'new-data' },
      dagDocuments: [{ id: 'dag', current: { id: 'later' }, audit: [original] }],
      interventionQueries: [query],
      swigAnalyses: [swig],
    }
    const before = JSON.stringify(workflow)
    const specification = readRun(workflow, {
      workspace: 'dag-query',
      id: query.id,
      view: 'specification',
    })
    const evidence = readRun(workflow, { workspace: 'dag-query', id: query.id, view: 'evidence' })
    const summary = readRun(workflow, { workspace: 'swig', id: swig.id })
    const unchanged = before === JSON.stringify(workflow)
    workflow.dagDocuments[0].audit = []
    return {
      specification,
      evidence,
      summary,
      unchanged,
      original,
      missing: readRun(workflow, { workspace: 'swig', id: swig.id, view: 'specification' }),
      wrong: readRun(workflow, { workspace: 'swig', id: query.id }),
    }
  })
  expect(result.specification.graphRevision).toEqual(result.original)
  expect(result.specification.provenance.graph).toBe('earlier')
  expect(result.specification.preparedDatasetRecord).toBeNull()
  expect(result.evidence.evidence.probabilities).toEqual([0.2, 0.8])
  expect(result.summary.conclusion.kind).toBe('notRequested')
  expect(result.missing.provenance.graph).toBe('missing-revision')
  expect(result.missing.graphRevision).toBeNull()
  expect(result.wrong.kind).toBe('refused')
  expect(result.unchanged).toBe(true)
})
