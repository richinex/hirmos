import { expect, test } from '@playwright/test'
import { surrogateRunSchema } from '../src/domain/surrogateRun'

test('saved surrogate runs reject changed estimators, uncertainty and sample ownership', () => {
  const record = { kind: 'surrogate-run', id: '313803fd-9883-4d57-9b14-8d28c0fce2d5', createdAt: '2026-10-03T12:00:00.000Z', sourceFingerprint: 'a'.repeat(64), sourceRows: 5,
    selection: { sample: { column: 'sample', experimental: 1, observational: 0 }, treatment: 'treatment', outcome: 'outcome', surrogates: ['s'], adjustment: { kind: 'none' }, estimator: 'index', uncertainty: { kind: 'none' } },
    rationale: 'Synthetic test specification.', rows: { experimental: [0, 2], observational: [1, 3], excluded: [4] },
    evidence: { estimator: 'index', experimentalRows: 2, observationalRows: 2, surrogateColumns: 1, baselineColumns: 0, estimate: 2, uncertainty: { kind: 'none' } } }
  expect(surrogateRunSchema.safeParse(record).success).toBe(true)
  const restriction = {kind:'boundedDirectEffect',maximum:0.1}
  const bounded = {...record,selection:{...record.selection,checks:{validation:'none',biasBounds:restriction}},diagnostics:[{
    experimentalRows:2,observationalRows:2,surrogateColumns:1,baselineColumns:0,analysis:{kind:'biasBounds',restriction,lower:-0.1,upper:0.1}}]}
  expect(surrogateRunSchema.safeParse(bounded).success).toBe(true)
  expect(surrogateRunSchema.safeParse({...bounded,selection:record.selection}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...bounded,diagnostics:[]}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...bounded,diagnostics:[...bounded.diagnostics,...bounded.diagnostics]}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...bounded,selection:{...bounded.selection,checks:{validation:'none',biasBounds:{...restriction,maximum:0.2}}}}).success).toBe(false)
  expect(surrogateRunSchema.safeParse({...record,selection:{...record.selection,checks:{validation:'observedOutcome',biasBounds:{kind:'none'}}}}).success).toBe(false)
  for (const evidence of [{ ...record.evidence, estimator: 'score' }, { ...record.evidence, experimentalRows: 3 },
    { ...record.evidence, baselineColumns: 1 }, { ...record.evidence, uncertainty: { kind: 'bootstrapStandardError', standardError: 0.2, repetitions: 20, seed: 1 } }])
    expect(surrogateRunSchema.safeParse({ ...record, evidence }).success).toBe(false)
  for (const rows of [{ experimental: [0, 2], observational: [2, 3], excluded: [4] },
    { experimental: [0, 2], observational: [1, 3], excluded: [] },
    { experimental: [0, 2], observational: [1, 3], excluded: [5] }])
    expect(surrogateRunSchema.safeParse({ ...record, rows }).success).toBe(false)
})
