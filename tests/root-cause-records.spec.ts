import { expect, test } from '@playwright/test'

test('all root-cause records share graph binding and preserve their own history', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { bindRootCauseRecord, appendRootCauseRecord } = await import(new URL('/src/domain/rootCauseRecords.ts', location.href).href)
    const { EMPTY_ROOT_CAUSE } = await import(new URL('/src/domain/rootCauseAnalysis.ts', location.href).href)
    const revision = { id: 'revision', graph: { kind: 'editable-dag', nodes: [
      { kind: 'observed', id: 'a', column: '0:A', name: 'A' },
      { kind: 'observed', id: 'b', column: '1:B', name: 'B' },
    ], edges: [{ kind: 'directed', id: 'a-b', cause: 'a', effect: 'b', timing: { kind: 'contemporaneous' }, support: { kind: 'user-assumption', rationale: 'A affects B.' }, evidence: [] }] } }
    const document = { id: 'document', preparedDataset: 'prepared', dataset: { kind: 'cross-section', observations: 100 }, current: revision, audit: [revision] }
    const prepared = { id: 'prepared', columns: ['0:A', '1:B'] }
    const common = { id: 'record', createdAt: '2026-09-15T12:00:00.000Z', graph: { dagDocument: 'document', dagRevision: 'revision', preparedDataset: 'prepared' } }
    const model = { names: ['A', 'B'], edges: [[0, 1]], rows: 100 }
    const random = { keys: Array(624).fill(0), position: 0, normal: null }
    const candidates = [
      { kind: 'influence', record: { ...common, model: { ...model, target: 1, random: { kind: 'seed', seed: 0 }, query: { kind: 'intrinsic', training: 100, randomization: 20, baseline: 20 } }, evidence: { random, outcome: { kind: 'intrinsic', nodes: [0, 1], values: [2, 1] }, mechanisms: [{ kind: 'empirical', node: 0 }, { kind: 'additive', node: 1, predictor: 'linear', noise: 'continuous' }] } } },
      { kind: 'run', record: { ...common, model: { ...model, target: 1, repetitions: 1, upperQuantile: 0.95, fraction: 1, random: { kind: 'seed', seed: 47 }, query: { kind: 'anomaly', samples: 10 } }, comparison: { name: 'observation.csv', fingerprint: 'test', rows: 1 }, evidence: { random, outcome: { kind: 'anomaly', nodes: [0, 1], summary: { estimates: [1, 2], bounds: [[1, 1], [2, 2]], quantiles: [0.05, 0.95], replicates: [[1, 2]], optimizerStatus: 0 } } } } },
      { kind: 'effects', record: { ...common, model: { ...model, treatment: 0, outcome: 1, mechanisms: [{ kind: 'empirical' }, { kind: 'regression' }], trees: 10, minLeaf: 1, fitSeed: 0, simulationSeed: 47, repetitions: 1, upperQuantile: 0.95, grouping: { kind: 'none' } }, evidence: { estimates: [2], bounds: [[2, 2]], quantiles: [0.05, 0.95], replicates: [[2]], optimizer: { status: 'precisionLoss', iterations: 17 }, grouping: { kind: 'none' } } } },
      { kind: 'checks', record: { ...common, model: { ...model, seed: 47, scope: 'fitted' }, evidence: { scope: 'fitted', random, mechanisms: [{ kind: 'root', node: 0, klDivergence: 0 }, { kind: 'conditional', node: 1, crps: 0, mse: 0, nmse: 0, r2: 1 }], invertibility: [], overallDivergence: 0 } } },
    ]
    let workspace = EMPTY_ROOT_CAUSE
    const cases = candidates.map(candidate => {
      const bind = (value: unknown) => bindRootCauseRecord(value, [document], prepared)
      const accepted = bind(candidate)
      if (accepted.ok) workspace = appendRootCauseRecord(workspace, accepted.value)
      return {
        kind: candidate.kind, accepted: accepted.ok,
        malformed: bind({ ...candidate, record: { ...candidate.record, evidence: {} } }),
        missingData: bindRootCauseRecord(candidate, [document], null),
        stale: bindRootCauseRecord(candidate, [document], { ...prepared, id: 'replaced' }),
        missingRevision: bindRootCauseRecord(candidate, [{ ...document, audit: [] }], prepared),
        reordered: bind({ ...candidate, record: { ...candidate.record, model: { ...candidate.record.model, names: ['B', 'A'] } } }),
      }
    })
    return { cases, counts: [workspace.runs.length, workspace.effects.length, workspace.checks.length, workspace.influences.length], optimizer: workspace.effects[0]?.evidence.optimizer.status }
  })
  for (const item of result.cases) {
    expect(item.accepted, item.kind).toBe(true)
    expect(item.malformed).toMatchObject({ ok: false, error: { kind: 'invalid-record' } })
    expect(item.missingData).toMatchObject({ ok: false, error: { kind: 'no-preparation' } })
    expect(item.stale).toMatchObject({ ok: false, error: { kind: 'graph', problem: { kind: 'different-preparation' } } })
    expect(item.missingRevision).toMatchObject({ ok: false, error: { kind: 'graph', problem: { kind: 'missing-revision' } } })
    expect(item.reordered).toMatchObject({ ok: false, error: { kind: 'model-mismatch' } })
  }
  expect(result.counts).toEqual([1, 1, 1, 1])
  expect(result.optimizer).toBe('precisionLoss')
})
