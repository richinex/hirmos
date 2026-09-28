import { expect, test } from '@playwright/test'
import { levelEvidence } from '../src/domain/levelEvidence'
import { describeLevelIssueGroup, groupLevelIssues, type StationarityAssessment } from '../src/domain/stationarityAssessment'

/**
 * Series that share one stationarity issue are reported together: a few by name, many by count, with
 * refusals first. The wording is the battery's own, continued after a colon.
 */

const ref = { test: 'adf', specification: 'c', series: 'levels', pValue: 0.5 } as const
const evidence = [ref] as unknown as StationarityAssessment extends { evidence: infer E } ? E : never
const inconclusive = (...conflicts: readonly ({ kind: 'difference-battery-missing' } | { kind: 'both-reject'; specification: 'c' })[]): StationarityAssessment =>
  ({ kind: 'inconclusive', conflicts, evidence } as unknown as StationarityAssessment)
const stationary: StationarityAssessment = { kind: 'levelStationary', evidence }
const integrated: StationarityAssessment = { kind: 'differenceStationary', order: 1, evidence }
const unresolved: StationarityAssessment = { kind: 'higherOrderOrUnresolved', minimumSuspectedOrder: 2, evidence }
const broken = (row: number): StationarityAssessment => ({ kind: 'breakStationary', break: row - 1, model: 'level', evidence })
const wording = { integrated: 'the fit can be spurious.' }
const names = (count: number) => Array.from({ length: count }, (_, index) => `s${index + 1}`)

test('series with the same issue form one group, named up to five and counted beyond', () => {
  const four = groupLevelIssues(['a', 'b', 'c', 'd'].map((name) => ({ name, assessment: null })))
  expect(four).toHaveLength(1)
  expect(describeLevelIssueGroup(four[0]!, wording)).toBe('Stationarity not tested for 4 series (a, b, c, d).')
  const one = groupLevelIssues([{ name: 'kms', assessment: inconclusive({ kind: 'difference-battery-missing' }) }])
  expect(describeLevelIssueGroup(one[0]!, wording)).toBe('Stationarity inconclusive for kms: the first difference was not tested, so the series cannot be called I(1).')
  const many = groupLevelIssues(names(58).map((name) => ({ name, assessment: inconclusive({ kind: 'difference-battery-missing' }) })))
  expect(describeLevelIssueGroup(many[0]!, wording)).toBe('Stationarity inconclusive for 58 series: the first difference was not tested, so the series cannot be called I(1).')
})

test('a stationary series raises nothing, and an acronym keeps its capitals after the colon', () => {
  expect(groupLevelIssues([{ name: 'a', assessment: stationary }, { name: 'b', assessment: { kind: 'trendStationary', evidence } }])).toHaveLength(0)
  const [group] = groupLevelIssues([{ name: 'a', assessment: inconclusive({ kind: 'both-reject', specification: 'c' }) }])
  expect(describeLevelIssueGroup(group!, wording)).toBe('Stationarity inconclusive for a: ADF and KPSS both reject under c: conflicting evidence.')
})

test('refusals come first, breaks keep their row, and the missing test names its action', () => {
  const { refused, cautions } = levelEvidence([
    { name: 'a', assessment: null },
    { name: 'b', assessment: broken(41) },
    { name: 'c', assessment: unresolved },
    { name: 'd', assessment: integrated },
    { name: 'e', assessment: broken(7) },
  ], wording)
  expect(refused.map((group) => group.summary)).toEqual(['I(2) or unresolved for c: no I(0)/I(1) method applies until the order is settled.'])
  expect(cautions.map((group) => group.summary)).toEqual([
    'I(1) on the prepared scale for d: the fit can be spurious.',
    'Stationary only around a break for 2 series (b, e): the model does not include it.',
    'Stationarity not tested for a.',
  ])
  expect(cautions[1]!.series).toEqual([{ name: 'b', detail: 'break at row 41' }, { name: 'e', detail: 'break at row 7' }])
  expect(cautions.map((group) => group.action)).toEqual([null, null, 'run-stationarity-tests'])
  expect(levelEvidence([{ name: 'a', assessment: inconclusive({ kind: 'difference-battery-missing' }) }], wording).cautions[0]!.action).toBe('test-first-difference')
})
