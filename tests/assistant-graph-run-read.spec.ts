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
    swig.names = ['X0', 'X1']
    const conclusion = {
      kind: 'did',
      assessment: {
        decision: {
          kind: 'supportedSubjectToOverlap',
          required: [0, 1],
          available: [0, 1],
          selectedSupported: false,
        },
        noWithinPeriodTreatmentCovariateEffect: true,
        noFutureTreatmentCovariateEffect: true,
        noDirectCovariateOutcomeDynamics: false,
        noWithinPeriodCovariateOutcomeEffect: false,
      },
    }
    const review = readRun(
      { ...workflow, swigAnalyses: [{ ...swig, result: { conclusion } }] },
      { workspace: 'swig', id: swig.id, view: 'review' },
    )
    workflow.dagDocuments[0].audit = []
    return {
      specification,
      evidence,
      summary,
      unchanged,
      review,
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
  expect(result.review.review.requiredControls).toEqual(['X0', 'X1'])
  expect(result.review.review.selectedSupported).toBe(false)
  expect(result.review.review.reading).toContain('graph supports an adjustment family')
  expect(result.review.review.interpretation).toContain('not automatically excluded')
  expect(result.review.provenance.graph).toBe('earlier')
})

test('graph settings refuse missing, wrong and stale targets before checking or proposing changes', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { proposeWorkspaceConfiguration } = await import(
      new URL('/src/assistant/workspaceConfiguration.tsx', location.href).href
    )
    let checked = 0
    const graph = { document: 'six', revision: 'current', name: 'Figure 6b' }
    const capabilities = {
      'swig-settings': {
        graph,
        revision: 'settings',
        title: 'SWIG',
        columns: [],
        describe: null,
        check: (value: unknown) => {
          checked++
          return { ok: true, value }
        },
      },
    }
    const request = {
      key: 'swig-settings',
      configuration: {},
      reason: 'Test the selected controls.',
    }
    const missing = proposeWorkspaceConfiguration(capabilities, request)
    const wrong = proposeWorkspaceConfiguration(capabilities, {
      ...request,
      graph: { document: 'five', revision: 'current' },
    })
    const stale = proposeWorkspaceConfiguration(capabilities, {
      ...request,
      graph: { document: 'six', revision: 'old' },
    })
    const before = checked
    const matched = proposeWorkspaceConfiguration(capabilities, {
      ...request,
      graph: { document: 'six', revision: 'current' },
    })
    return { missing, wrong, stale, before, matched, checked }
  })
  for (const refused of [result.missing, result.wrong, result.stale]) {
    expect(refused.ok).toBe(false)
    expect(refused.error).toContain('Figure 6b')
  }
  expect(result.before).toBe(0)
  expect(result.matched.ok).toBe(true)
  expect(result.checked).toBe(1)
})
