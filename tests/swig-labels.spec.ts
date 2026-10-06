import { expect, test } from '@playwright/test'
import { swigNodeName, type SwigAnalysis } from '../src/domain/swig'
import { brand } from '../src/domain/dop'

const record = (earlier: number[], later: number[]): SwigAnalysis => ({
  kind: 'swig-analysis',
  projection: { kind: 'explicit' },
  id: brand<string, 'SwigAnalysisId'>('10000000-0000-4000-8000-000000000001'),
  dagDocument: brand<string, 'DagDocumentId'>('dag'),
  dagRevision: brand<string, 'DagRevisionId'>('revision'),
  preparedDataset: brand<string, 'PreparedDatasetVersionId'>('prepared'),
  createdAt: '2026-10-05T12:00:00.000Z',
  rationale: 'Recorded intervention world.',
  names: ['D', 'Y0', 'Y1', 'Z'],
  specification: {
    roles: ['endogenous', 'endogenous', 'endogenous', 'endogenous'],
    edges: [[0, 2]],
    interventions: [
      [0, 0],
      [3, 2],
    ],
    construction: {
      kind: 'difference',
      earlier: { outcome: 1, terms: [] },
      later: { outcome: 2, terms: [] },
    },
    query: { kind: 'graph' },
  },
  result: {
    nodes: [
      { kind: 'random', original: 0, role: 'endogenous', interventions: [] },
      { kind: 'random', original: 1, role: 'endogenous', interventions: earlier },
      { kind: 'random', original: 2, role: 'endogenous', interventions: later },
      { kind: 'random', original: 3, role: 'endogenous', interventions: [] },
      { kind: 'fixed', original: 0, value: 0 },
      { kind: 'difference', earlier: 1, later: 2, cancelled: [] },
    ],
    edges: [],
    conclusion: { kind: 'notRequested' },
  },
})

test('the untreated difference uses the same potential-outcome label as its level node', () => {
  const analysis = record([], [0])
  expect(swigNodeName(analysis, 2)).toBe('Y1 (D = 0)')
  expect(swigNodeName(analysis, 5)).toBe('Y1 (D = 0) − Y0')
  expect(swigNodeName(analysis, 4)).toBe('D = 0')
})

test('each differenced outcome keeps its own recorded intervention world', () => {
  expect(swigNodeName(record([3], [0, 3]), 5)).toBe('Y1 (D = 0, Z = 2) − Y0 (Z = 2)')
  expect(swigNodeName(record([], []), 5)).toBe('Y1 − Y0')
})

test('labels use recorded names and values, not hard-coded untreated notation', () => {
  const analysis = record([], [0])
  expect(
    swigNodeName(
      {
        ...analysis,
        names: ['Policy', 'Before', 'After', 'Z'],
        specification: {
          ...analysis.specification,
          interventions: [
            [0, 1],
            [3, 2],
          ],
        },
      },
      5,
    ),
  ).toBe('After (Policy = 1) − Before')
})

test('a missing level node is refused instead of silently dropping its intervention label', () => {
  const analysis = record([], [0])
  expect(() =>
    swigNodeName(
      {
        ...analysis,
        result: { ...analysis.result, nodes: [analysis.result.nodes[5]!] },
      },
      0,
    ),
  ).toThrow('Missing random outcome node')
})
