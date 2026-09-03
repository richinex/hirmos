import { expect, test } from '@playwright/test'
import { INITIAL_DISCOVERY_DRAFT, stepDiscovery } from '../src/domain/discovery'

test('GRACE and CD-NOTS share their skeleton defaults', () => {
  const cdnots = stepDiscovery(INITIAL_DISCOVERY_DRAFT, { type: 'method-selected', method: 'cdnots' }).configuration
  const grace = stepDiscovery(INITIAL_DISCOVERY_DRAFT, { type: 'method-selected', method: 'grace' }).configuration

  expect(cdnots.kind).toBe('cdnots')
  expect(grace.kind).toBe('grace')
  if (cdnots.kind !== 'cdnots' || grace.kind !== 'grace') return

  expect({ maxLag: grace.maxLag, alpha: grace.alpha, context: grace.context }).toEqual({
    maxLag: cdnots.maxLag,
    alpha: cdnots.alpha,
    context: cdnots.context,
  })
})
