import { test, expect } from '@playwright/test'
import { columnId } from '../src/domain/dataset'
import { selectFixedEffects, backdoorLinearConfigurationSchema } from '../src/domain/estimation'
import { adjustedRegressionPlan } from '../src/analysis/adjustedRegressionPlan'

const unit = { id: columnId(0, 'unit'), name: 'unit' }
const time = { id: columnId(1, 'time'), name: 'time' }
test('fixed-effect selection requires its own columns and distinct two-way identities', () => {
  expect(selectFixedEffects('none', null, null)).toEqual({ ok: true, value: { kind: 'none' } })
  expect(selectFixedEffects('unit', null, time).ok).toBe(false)
  expect(selectFixedEffects('time', time, null).ok).toBe(true)
  expect(selectFixedEffects('unit-and-time', unit, unit).ok).toBe(false)
  expect(selectFixedEffects('unit-and-time', unit, time).ok).toBe(true)
})

test('worker planning never substitutes another uncertainty model for missing cluster labels', () => {
  const configuration = { kind: 'backdoor-linear-regression', level: 0.95, fixedEffects: { kind: 'time', column: time.id, name: time.name }, errors: { kind: 'cluster', column: unit.id, name: unit.name } } as const
  expect(adjustedRegressionPlan(configuration, new Map([[time.id, 3]]))).toEqual({ ok: false, error: { kind: 'missing-grouping-column', column: unit.id } })
  expect(adjustedRegressionPlan(configuration, new Map([[time.id, 3], [unit.id, 4]]))).toEqual({ ok: true, value: { fixedEffects: { kind: 'time', column: 3 }, errorModel: { kind: 'cluster', column: 4 } } })
  expect(backdoorLinearConfigurationSchema.safeParse({ ...configuration, errors: { kind: 'hac' } }).success).toBe(false)
  expect(backdoorLinearConfigurationSchema.safeParse({ ...configuration, fixedEffects: { kind: 'time', column: '', name: '' } }).success).toBe(false)
})
