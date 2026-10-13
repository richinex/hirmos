import { expect, test } from '@playwright/test'

test('sensitivity and surrogate reads retain saved evidence and refuse unrelated run IDs', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const source = { id: 'original-fit' }
    const sensitivity = {
      id: 'sensitivity',
      kind: 'honest-did-run',
      preparedDataset: 'original-data',
      createdAt: '2026-01-01T00:00:00Z',
      source: { kind: 'estimation', run: source.id },
      evidence: {
        estimate: 2,
        standardError: 1,
        conventional: [0, 4],
        results: [{ bound: 0.5, interval: { kind: 'fixedLength', lower: -1, upper: 5 } }],
        request: { configuration: { bounds: [0.5] } },
      },
    }
    const surrogate = {
      id: 'surrogate',
      kind: 'surrogate-run',
      createdAt: sensitivity.createdAt,
      sourceFingerprint: 'original-fingerprint',
      sourceRows: 10,
      selection: { estimator: 'index' },
      rationale: 'Recorded assumptions',
      rows: { experimental: [0], observational: [1], excluded: [] },
      evidence: {
        estimator: 'index',
        estimate: 0.2,
        uncertainty: { kind: 'none' },
        experimentalRows: 1,
        observationalRows: 1,
      },
      diagnostics: [{ result: 'saved-diagnostic' }],
      paths: { results: [{ result: 'saved-horizon' }] },
    }
    const workflow = {
      kind: 'profiled',
      prepared: { id: 'original-data' },
      profile: { source: { fingerprint: 'original-fingerprint' } },
      sensitivityRuns: [sensitivity],
      surrogateRuns: [surrogate],
      estimationRuns: [source],
      timeSeriesRuns: [],
    }
    const before = JSON.stringify(workflow)
    const sensitive = readRun(workflow, { workspace: 'sensitivity', id: sensitivity.id })
    const evidence = readRun(workflow, {
      workspace: 'surrogate',
      id: surrogate.id,
      view: 'evidence',
    })
    const specification = readRun(workflow, {
      workspace: 'surrogate',
      id: surrogate.id,
      view: 'specification',
    })
    const unchanged = before === JSON.stringify(workflow)
    workflow.estimationRuns = []
    workflow.prepared.id = 'new-data'
    workflow.profile.source.fingerprint = 'new-fingerprint'
    return {
      sensitive,
      evidence,
      specification,
      unchanged,
      missing: readRun(workflow, { workspace: 'sensitivity', id: sensitivity.id }),
      changed: readRun(workflow, { workspace: 'surrogate', id: surrogate.id }),
      wrongWorkspace: readRun(workflow, { workspace: 'sensitivity', id: surrogate.id }),
      unknown: readRun(workflow, { workspace: 'surrogate', id: 'absent' }),
    }
  })
  expect(result.sensitive.summary.estimate).toBe(2)
  expect(result.sensitive.source.id).toBe('original-fit')
  expect(result.sensitive.provenance.sourceRun).toBe('available')
  expect(result.evidence.evidence.estimate).toBe(0.2)
  expect(result.evidence.diagnostics).toEqual([{ result: 'saved-diagnostic' }])
  expect(result.evidence.paths.results).toEqual([{ result: 'saved-horizon' }])
  expect(result.specification.rationale).toBe('Recorded assumptions')
  expect(result.missing.provenance).toEqual({
    sourceRun: 'missing',
    preparedDataset: 'unavailable',
  })
  expect(result.missing.summary).toEqual(result.sensitive.summary)
  expect(result.changed.provenance.source).toBe('different')
  expect(result.wrongWorkspace.kind).toBe('refused')
  expect(result.unknown.kind).toBe('refused')
  expect(result.unchanged).toBe(true)
})
