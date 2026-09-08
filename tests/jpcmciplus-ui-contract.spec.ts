import { expect, test } from '@playwright/test'
import { brand } from '../src/domain/dop'
import { orderJointPanelMatrix } from '../src/domain/panel'
import { parseAnalysisWorkerCommand } from '../src/workers/analysisProtocol'

const column = (id: string, name: string) => ({ id: brand<string, 'ColumnId'>(id), name })

test('orders a shuffled balanced panel dataset-major and then period-major', () => {
  const x = column('x', 'X')
  const y = column('y', 'Y')
  const result = orderJointPanelMatrix({
    rowCount: 4,
    columns: [x, y],
    values: Float64Array.from([21, 10, 20, 11, 121, 110, 120, 111]),
  }, {
    kind: 'panel-key-matrix',
    sourceFingerprint: brand<string, 'SourceFingerprint'>('a'.repeat(64)),
    rowCount: 4,
    units: ['B', 'A', 'B', 'A'],
    periodCodes: [1, 0, 0, 1],
    periods: [{ code: 0, label: '2025' }, { code: 1, label: '2026' }],
  })

  expect(result.ok).toBe(true)
  if (!result.ok) return
  expect(result.value.units).toEqual(['A', 'B'])
  expect(result.value.periodCatalog).toEqual([{ code: 0, label: '2025' }, { code: 1, label: '2026' }])
  expect([...result.value.values]).toEqual([10, 11, 20, 21, 110, 111, 120, 121])
})

test('refuses a missing panel cell instead of silently shortening one dataset', () => {
  const x = column('x', 'X')
  const result = orderJointPanelMatrix({ rowCount: 3, columns: [x], values: Float64Array.from([10, 11, 20]) }, {
    kind: 'panel-key-matrix',
    sourceFingerprint: brand<string, 'SourceFingerprint'>('b'.repeat(64)),
    rowCount: 3,
    units: ['A', 'A', 'B'],
    periodCodes: [0, 1, 0],
    periods: [{ code: 0, label: '2025' }, { code: 1, label: '2026' }],
  })
  expect(result).toEqual({ ok: false, error: { kind: 'missing-panel-cell', unit: 'B', period: { code: 1, label: '2026' } } })
})

test('accepts only a complete J-PCMCI+ worker command', () => {
  const request = crypto.randomUUID()
  const command = parseAnalysisWorkerCommand({
    kind: 'jpcmci-plus', request, values: new Float64Array(96), rows: 48,
    datasets: 2, periods: 24, observedColumns: 2, classes: ['system', 'system'],
    timeDummy: true, spaceDummy: true, tauMax: 1, pcAlpha: 0.05,
  })
  expect(command.ok).toBe(true)

  const invalid = parseAnalysisWorkerCommand({
    kind: 'jpcmci-plus', request, values: new Float64Array(96), rows: 48,
    datasets: 2, periods: 24, observedColumns: 2, classes: ['system', 'timeContext'],
    timeDummy: true, spaceDummy: true, tauMax: 1, pcAlpha: 0.05,
  })
  expect(invalid.ok).toBe(false)
})
