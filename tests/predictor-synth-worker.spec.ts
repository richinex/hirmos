import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

const oracle = JSON.parse(readFileSync('crates/analysis-wasm/test-data/predictor-synth-texas.json', 'utf8'))

test('predictor-based synthetic-control worker reproduces Texas without replacing optimizer status', async ({ page }) => {
  test.setTimeout(180_000)
  await page.goto('/app')
  const evidence = await page.evaluate(async (source) => {
    const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
    const data = new DataView(new Uint8Array(source.exact.rows).buffer)
    const rows = source.data.rows.length
    const columns = source.data.columns.length
    const values = new Float64Array(rows * columns)
    for (let row = 0; row < rows; row++) {
      for (let column = 0; column < columns; column++) values[column * rows + row] = data.getFloat64((row * columns + column) * 8, true)
    }
    const r = source.request
    const model = {
      rows, columnNames: source.data.columns,
      units: source.data.rows.map((row: { unit: number }) => row.unit),
      times: source.data.rows.map((row: { period: number }) => row.period),
      treated: r.treated, donors: r.donors, outcome: r.outcome, predictors: r.predictors,
      summary: 'mean', special: r.special.map((p: { column: number; periods: number[] }) => ({ ...p, summary: 'mean' })),
      predictorPeriods: r.predictor_periods, fitPeriods: r.fit_periods, plotPeriods: r.plot_periods,
      selection: { kind: 'automatic' },
    }
    const result = await analysis.runPredictorSyntheticControl(values, model)
    if (!result.ok) throw new Error(JSON.stringify(result.error))
    return result.value
  }, oracle)
  const close = (actual: readonly number[], expected: readonly number[]) => {
    expect(actual).toHaveLength(expected.length)
    actual.forEach((value, i) => expect(Math.abs(value - expected[i]!)).toBeLessThanOrEqual(1e-10 * Math.max(1, Math.abs(expected[i]!))))
  }
  close(evidence.donorWeights, oracle.fit.w)
  close(evidence.balance.map((row: { weight: number }) => row.weight), oracle.fit.v)
  close(evidence.synthetic, oracle.synthetic)
  close(evidence.gaps, oracle.gaps)
  expect(evidence.selection).toEqual({ kind: 'optimized', start: 'regression', method: 'nelder-mead', evaluations: 501, convergenceCode: 1 })
  expect(evidence.attempts.filter((attempt: { kind: string }) => attempt.kind === 'failed')).toHaveLength(2)
})

test('predictor-based synthetic-control contract refuses inconsistent axes and forged result metadata', async ({ page }) => {
  await page.goto('/app')
  const checks = await page.evaluate(async () => {
    const domain = await import(new URL('/src/domain/predictorSyntheticControl.ts', location.href).href)
    const base = {
      rows: 3, columnNames: ['outcome', 'predictor'], units: [0, 1, 2], times: [0, 0, 0],
      treated: 0, donors: [1, 2], outcome: 0, predictors: [1], summary: 'mean', special: [],
      predictorPeriods: [0], fitPeriods: [0], plotPeriods: [0], selection: { kind: 'automatic' },
    }
    const malformed = [
      { ...base, units: [0] }, { ...base, donors: [0, 1] }, { ...base, donors: [1, 1] },
      { ...base, predictors: [4] }, { ...base, fitPeriods: [0, 0] },
      { ...base, selection: { kind: 'supplied', weights: [0] } },
    ]
    const evidence = {
      request: base, treatedUnit: 0, donors: [1, 2], donorWeights: [0.5, 0.5], fitPeriods: [0], plotPeriods: [0],
      observed: [2], synthetic: [1], gaps: [1], balance: [{ predictor: 'predictor', treated: 2, synthetic: 1, donorMean: 1, weight: 1 }],
      predictorLoss: 1, outcomeMspe: 1, donorIterations: 3, selection: { kind: 'single-predictor' }, attempts: [], preparationNotes: [],
    }
    return {
      valid: domain.predictorSyntheticRequestSchema.safeParse(base).success && domain.predictorSyntheticEvidenceSchema.safeParse(evidence).success,
      refused: malformed.every(value => !domain.predictorSyntheticRequestSchema.safeParse(value).success),
      forged: !domain.predictorSyntheticEvidenceSchema.safeParse({ ...evidence, donors: [1, 3] }).success,
      path: !domain.predictorSyntheticEvidenceSchema.safeParse({ ...evidence, gaps: [] }).success,
    }
  })
  expect(checks).toEqual({ valid: true, refused: true, forged: true, path: true })
})
