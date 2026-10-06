import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('NLSY two-way fixed effects use fixest clustered inference through WASM', async ({ page }) => {
  const csv = readFileSync('docs/more-did/bad-controls/data/nlsy_job_displacement.csv', 'utf8')
  const rows = csv.trim().split(/\r?\n/).slice(1).map(line => {
    const [id, year, y, occupation, group] = line.split(',').map(Number)
    return [id!, year!, y!, occupation!, Number(group! > 0 && year! >= group!)]
  })
  await page.goto('/app')
  for (const includeOccupation of [true, false]) {
    const result = await page.evaluate(async ({ rows, includeOccupation }) => {
      // @ts-expect-error Vite resolves the browser source URL.
      const { runBackdoorLinear } = await import('/src/analysis/client.ts')
      const values = new Float64Array(rows.length * 5)
      for (let j = 0; j < 5; j++) for (let i = 0; i < rows.length; i++) values[j * rows.length + i] = rows[i][j]
      return runBackdoorLinear(values, rows.length, 5, {
        treatment: 4, outcome: 2, adjustment: includeOccupation ? [3] : [],
        hacMaxLags: null, level: 0.95, errorModel: { kind: 'cluster', column: 0 },
        fixedEffects: { kind: 'unitAndTime', unit: 0, time: 1 },
      })
    }, { rows, includeOccupation })
    expect(result.ok, JSON.stringify(result)).toBe(true)
    if (!result.ok) throw new Error(JSON.stringify(result.error))
    const reference = includeOccupation
      ? { estimate: -0.095602023833995506, se: 0.023859444547947587, interval: [-0.14238320584808825, -0.048820841819902761], k: 8 }
      : { estimate: -0.097744677301602426, se: 0.024026325359280058, interval: [-0.14485306230627254, -0.050636292296932298], k: 7 }
    expect(result.value.estimate).toBeCloseTo(reference.estimate, 9)
    const errors = result.value.errorModel
    expect(errors.kind).toBe('cluster')
    if (errors.kind !== 'cluster') throw new Error('Expected clustered inference')
    expect(errors.standardError).toBeCloseTo(reference.se, 9)
    expect(errors.interval[0]).toBeCloseTo(reference.interval[0]!, 9)
    expect(errors.interval[1]).toBeCloseTo(reference.interval[1]!, 9)
    expect(errors.correction).toEqual({ kind: 'fixestNonNested', parameters: reference.k, degreesOfFreedom: 3230 })
  }
})
