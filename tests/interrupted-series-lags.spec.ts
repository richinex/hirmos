import { expect, test } from '@playwright/test'
import { parseAnalysisWorkerCommand } from '../src/workers/analysisProtocol'

function command(rows: number, ljungBoxLags: number) {
  return {
    kind: 'interrupted-series',
    request: 'a9605a9f-43c7-42fb-87b0-5ac1a59fa885',
    values: new Float64Array(rows),
    rows,
    columns: 1,
    outcome: 0,
    model: { kind: 'continuous', errors: { kind: 'neweyWest', maxLags: null } },
    interventionRow: 100,
    lag: 0,
    impact: { kind: 'level' },
    seasonal: { kind: 'harmonic', pairs: 1, period: 52 },
    ljungBoxLags,
  }
}

test('residual diagnostic lags are bounded by the sample, not a monthly cap', () => {
  for (const [rows, lags] of [
    [195, 48],
    [196, 49],
    [208, 52],
    [260, 52],
    [1464, 366],
  ]) {
    expect(parseAnalysisWorkerCommand(command(rows, lags)).ok).toBe(true)
  }
  for (const lags of [0, -1, 1.5, 208, 209]) {
    expect(parseAnalysisWorkerCommand(command(208, lags)).ok).toBe(false)
  }
})

for (const rows of [196, 208, 260]) {
  test(`weekly interrupted series retains all ${rows} observations through the worker`, async ({
    page,
  }) => {
    await page.goto('/app')
    const result = await page.evaluate(async (rows) => {
      const path = '/src/analysis/client.ts'
      const { runInterruptedSeries } = await import(path)
      const values = Float64Array.from(
        { length: rows },
        (_, i) =>
          30 +
          0.02 * i +
          (i >= 100 ? 3 : 0) +
          2 * Math.sin((2 * Math.PI * i) / 52) +
          Math.sin(i * 1.73),
      )
      return runInterruptedSeries(values, rows, 1, {
        outcome: 0,
        model: { kind: 'continuous', errors: { kind: 'neweyWest', maxLags: null } },
        interventionRow: 100,
        lag: 0,
        impact: { kind: 'level' },
        seasonal: { kind: 'harmonic', pairs: 1, period: 52 },
        ljungBoxLags: Math.min(52, Math.floor(rows / 4)),
      })
    }, rows)
    expect(result.ok, JSON.stringify(result)).toBe(true)
    if (!result.ok) return
    expect(result.value.observations).toBe(rows)
    expect(result.value.path).toHaveLength(rows)
    expect(result.value.ljungBox).toHaveLength(Math.min(52, Math.floor(rows / 4)))
    expect(
      result.value.ljungBox.every((entry: { statistic: number }) =>
        Number.isFinite(entry.statistic),
      ),
    ).toBe(true)
  })
}
