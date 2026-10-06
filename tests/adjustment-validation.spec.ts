import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('supplied adjustment sets match Dagitty through the worker', async ({ page }) => {
  const fixture = JSON.parse(
    readFileSync('crates/causal-core/oracle/fixtures/dagitty_supplied_sets.json', 'utf8'),
  )
  await page.goto('/app')
  const actual = await page.evaluate(async (cases) => {
    const { validateAdjustmentSets } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const results = []
    for (const c of cases) {
      const result = await validateAdjustmentSets({
        nodes: c.nodes,
        edges: c.edges,
        treatment: c.treatment,
        outcome: c.outcome,
        unobserved: c.unobserved,
        sets: [c.set],
      })
      if (!result.ok) throw Error(JSON.stringify(result.error))
      results.push(result.value[0].kind === 'valid')
    }
    return results
  }, fixture.cases)
  expect(actual).toEqual(fixture.cases.map((c: { valid: boolean }) => c.valid))
})
