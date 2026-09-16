import { expect, test } from '@playwright/test'

test('diagnostic transitions reject inapplicable settings and stale dataset events', async ({ page }) => {
  await page.goto('/app')
  const actual = await page.evaluate(async () => {
    const { initialDiagnosticDraft, stepDiagnosticDraft } = await import(new URL('/src/domain/diagnosticDraft.ts', location.href).href)
    const series = { kind: 'prepared-time-series', id: 'series', columns: ['x', 'y'] }
    const observations = { kind: 'prepared-cross-section', id: 'observations', columns: ['x'] }
    const initial = initialDiagnosticDraft(series)
    const independent = initialDiagnosticDraft(observations)
    const selection = stepDiagnosticDraft(initial, { type: 'stationarity', columns: ['x', 'unknown', 'x'] }, series)
    const invalid = [
      { type: 'min-size', value: 0 }, { type: 'min-size', value: 2.5 },
      { type: 'max-lag', value: 401 }, { type: 'correlation', value: NaN },
      { type: 'vif', value: Infinity }, { type: 'pair', value: [0, 2] },
      { type: 'granger-column', role: 'cause', column: 'unknown' }, { type: 'granger-lag', value: 999 },
    ]
    return {
      selection: selection.stationarity,
      rejected: invalid.every(event => stepDiagnosticDraft(initial, event, series) === initial),
      stale: stepDiagnosticDraft(initial, { type: 'view', view: 'stationarity' }, { ...series, id: 'replaced' }) === initial,
      independent: stepDiagnosticDraft(independent, { type: 'view', view: 'stationarity' }, observations) === independent,
      noTemporalFields: !('temporal' in independent) && !('stationarity' in independent) && !('granger' in independent),
      independentPair: independent.redundancy.pair,
      unchanged: initial.stationarity,
    }
  })
  expect(actual).toEqual({ selection: ['x'], rejected: true, stale: true, independent: true, noTemporalFields: true, independentPair: [0, 0], unchanged: [] })
})
