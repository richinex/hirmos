import { expect, test } from '@playwright/test'

test('worker keeps validity, recommendation and optimality separate', async ({ page }) => {
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const { validateAdjustmentSets } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const cases = [
      {
        name: '7',
        nodes: 5,
        edges: [
          [3, 0],
          [3, 2],
          [4, 2],
          [4, 1],
          [0, 1],
        ],
        unobserved: [3, 4],
      },
      {
        name: '7v',
        nodes: 5,
        edges: [
          [3, 0],
          [3, 2],
          [4, 2],
          [4, 1],
          [0, 1],
          [2, 1],
        ],
        unobserved: [3, 4],
      },
      {
        name: '10',
        nodes: 4,
        edges: [
          [2, 0],
          [3, 0],
          [3, 1],
          [0, 1],
        ],
        unobserved: [3],
      },
      {
        name: '15',
        nodes: 5,
        edges: [
          [0, 1],
          [0, 2],
          [2, 3],
          [4, 3],
          [4, 1],
        ],
        unobserved: [4],
      },
      {
        name: '16',
        nodes: 4,
        edges: [
          [0, 1],
          [0, 2],
          [3, 2],
          [3, 1],
        ],
        unobserved: [3],
      },
    ]
    return Promise.all(
      cases.map(async ({ name, ...graph }) => {
        const result = await validateAdjustmentSets({
          ...graph,
          treatment: 0,
          outcome: 1,
          sets: [[], [2]],
        })
        if (!result.ok) throw Error(JSON.stringify(result.error))
        return { name, ...result.value }
      }),
    )
  })
  const byName = Object.fromEntries(results.map((r) => [r.name, r]))
  for (const name of ['7v', '10']) expect(byName[name].analysis.kind).toBe('notIdentified')
  expect(byName['15'].analysis.recommendation).toEqual({
    kind: 'available',
    nodes: [],
    guarantee: 'notEstablished',
  })
  expect(byName['15'].checks).toEqual([{ kind: 'valid' }, { kind: 'valid' }])
  for (const name of ['7', '16']) {
    expect(byName[name].analysis.recommendation.guarantee).toBe('established')
    expect(byName[name].checks[1].kind).not.toBe('valid')
  }
})
