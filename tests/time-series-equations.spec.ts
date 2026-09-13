import { expect, test } from '@playwright/test'
import katex from 'katex'
import { timeSeriesEquations } from '../src/domain/timeSeriesEquations'
import { parseTimeSeriesRun } from '../src/domain/timeSeries'
import type { CountSeriesModelArtifact } from '../src/domain/countSeries'

const ardl = (trend: 'c' | 'ct', testCase: number, value: number) => {
  const parsed = parseTimeSeriesRun({ kind: 'ardl', id: 'r', preparedDataset: 'p', createdAt: '2026-09-13T10:00:00.000Z', outcome: { id: 'y', name: 'Output % {long name}' }, predictor: { id: 'x', name: 'Predictor' }, specification: { maxLag: 3, terms: ({ 2: 'restricted-constant', 3: 'constant', 4: 'restricted-trend', 5: 'trend' } as Record<number, string>)[testCase] }, evidence: { kind: 'ardlPss', observations: 100, trend, case: testCase, arLag: 2, dlLag: 3, grid: [], longRunEffect: value, pValue: 0.01, interval: [value - 1, value + 1], level: 0.95, boundsStatistic: 8, boundsCritical: [[1, 2], [2, 3], [3, 4], [4, 5]], boundsPLower: 0.01, boundsPUpper: 0.03 } })
  if (!parsed.ok) throw Error(parsed.error)
  return parsed.value
}

test('ARDL equations retain signs, selected lags and deterministic terms without inventing coefficients', () => {
  for (const testCase of [2, 3, 4, 5]) for (const value of [-2, 0, 2, 1e-10]) {
    const equations = timeSeriesEquations(ardl(testCase > 3 ? 'ct' : 'c', testCase, value))
    expect(equations.general[0]!.tex.includes('\\tau t')).toBe(testCase > 3)
    expect(equations.general[0]!.tex).toContain('\\sum_{j=0}^{3}')
    expect(equations.general[0]!.tex).not.toContain('Output')
    expect(equations.fitted.kind).toBe('available')
    if (equations.fitted.kind !== 'available') throw Error('Missing coefficient')
    expect(equations.fitted.expressions).toHaveLength(1)
    expect(equations.fitted.expressions[0]!.plain).toContain(String(value))
    for (const f of [...equations.general, ...equations.fitted.expressions]) expect(() => katex.renderToString(f.tex, { throwOnError: true, strict: 'error' })).not.toThrow()
  }
})

test('VECM equations distinguish rank and deterministic-term placement for multiple series', () => {
  for (const deterministic of ['n', 'ci', 'co', 'coli'] as const) for (const rank of [0, 1, 2, 3]) {
    const matrix = rank === 0 ? [] : Array.from({ length: 3 }, () => Array.from({ length: rank }, () => 0.2))
    const parsed = parseTimeSeriesRun({ kind: 'vecm', id: 'v', preparedDataset: 'p', createdAt: '2026-09-13T10:00:00.000Z', variables: ['a', 'b', 'c'].map(id => ({ id, name: id })), specification: { maxLags: 2, deterministic, significance: 95 }, evidence: { kind: 'vecm', observations: 150, deterministic, kArDiff: 1, rank, significance: 1, longRunEffect: null, alpha: matrix, beta: matrix, pvaluesAlpha: matrix, gamma: [], chow: null } })
    if (!parsed.ok) throw Error(parsed.error)
    const equations = timeSeriesEquations(parsed.value)
    const tex = equations.general[0]!.tex
    expect(tex.includes('\\alpha(')).toBe(rank > 0)
    expect(tex.includes('\\eta(t-1)')).toBe(rank > 0 && deterministic === 'coli')
    expect(tex.includes('+c+')).toBe(deterministic === 'co' || deterministic === 'coli')
    expect(equations.fitted.kind).toBe('unavailable')
    expect(() => katex.renderToString(tex, { throwOnError: true, strict: 'error' })).not.toThrow()
  }
})

test('count equations respect parameter order and log1p, and refuse incomplete coefficient vectors', () => {
  // Only the fields consumed by the equation builder are needed for this isolated test.
  const run = (link: 'identity' | 'log', parameters: number[]) => ({ kind: 'count-series-model', outcome: { name: 'cases' }, result: { link, parameters, pastObservationLags: [1, 7], pastMeanLags: [3], size: 8 } }) as CountSeriesModelArtifact
  for (const link of ['identity', 'log'] as const) {
    const equations = timeSeriesEquations(run(link, [2, 0.4, 0.2, 0.1]))
    if (equations.fitted.kind !== 'available') throw Error('Missing count equation')
    const tex = equations.fitted.expressions[0]!.tex
    expect(tex).toContain(link === 'log' ? '(0.2)\\log(1+Y_{t-7})' : '(0.2)Y_{t-7}')
    expect(tex).toContain(link === 'log' ? '(0.1)\\log\\lambda_{t-3}' : '(0.1)\\lambda_{t-3}')
    for (const f of [...equations.general, ...equations.fitted.expressions]) expect(() => katex.renderToString(f.tex, { throwOnError: true, strict: 'error' })).not.toThrow()
  }
  expect(timeSeriesEquations(run('identity', [2])).fitted.kind).toBe('unavailable')
})
