import { expect, test } from '@playwright/test'
import { swigSpecificationSchema, type SwigSpecification } from '../src/domain/swig'

const experiment: SwigSpecification = {
  roles: ['endogenous', 'endogenous'],
  edges: [[0, 1]],
  interventions: [[0, 1]],
  construction: { kind: 'swig' },
  query: { kind: 'separation', left: 0, right: 1, given: [] },
}

test('SWIG worker reproduces the randomized and confounded notebook cases', async ({ page }) => {
  await page.goto('/app')
  const results = await page.evaluate(async (specification) => {
    const { runSwigAnalysis } = await import(
      new URL('/src/analysis/client.ts', window.location.href).href
    )
    const randomized = await runSwigAnalysis(specification, ['A', 'Y'])
    const confounded = {
      ...specification,
      roles: ['endogenous', 'endogenous', 'exogenous'],
      edges: [
        [0, 1],
        [2, 0],
        [2, 1],
      ],
    }
    const uncontrolled = await runSwigAnalysis(confounded, ['A', 'Y', 'U'])
    const controlled = await runSwigAnalysis(
      {
        ...confounded,
        query: { ...specification.query, given: [2] },
      },
      ['A', 'Y', 'U'],
    )
    return { randomized, uncontrolled, controlled }
  }, experiment)
  expect(results.randomized).toMatchObject({
    ok: true,
    value: { conclusion: { kind: 'separated' }, edges: [[2, 1]] },
  })
  expect(results.uncontrolled).toMatchObject({
    ok: true,
    value: { conclusion: { kind: 'connected' } },
  })
  expect(results.controlled).toMatchObject({
    ok: true,
    value: { conclusion: { kind: 'separated' } },
  })
})

test('delta worker cancels only the declared shared function and retains both disturbances', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { runSwigAnalysis } = await import(
      new URL('/src/analysis/client.ts', window.location.href).href
    )
    return runSwigAnalysis(
      {
        roles: ['exogenous', 'endogenous', 'endogenous', 'endogenous', 'exogenous', 'exogenous'],
        edges: [
          [0, 1],
          [0, 2],
          [0, 3],
          [1, 3],
          [4, 2],
          [5, 3],
        ],
        interventions: [[1, 0]],
        construction: {
          kind: 'difference',
          earlier: {
            outcome: 2,
            terms: [
              { identity: 'unit', arguments: [0] },
              { identity: 'noise0', arguments: [4] },
            ],
          },
          later: {
            outcome: 3,
            terms: [
              { identity: 'unit', arguments: [0] },
              { identity: 'noise1', arguments: [5] },
            ],
          },
        },
        query: { kind: 'separation', left: 1, right: 7, given: [] },
      },
      ['U', 'D', 'Y0', 'Y1', 'e0', 'e1'],
    )
  })
  expect(result).toMatchObject({ ok: true, value: { conclusion: { kind: 'separated' } } })
  if (!result.ok) throw new Error('The worker refused the reference case')
  expect(result.value.nodes[7]).toEqual({
    kind: 'difference',
    earlier: 2,
    later: 3,
    cancelled: ['unit'],
  })
  expect(result.value.edges).toEqual(
    expect.arrayContaining([
      [4, 7],
      [5, 7],
    ]),
  )
  expect(result.value.edges).not.toContainEqual([0, 7])
})

test('worker refuses a cycle instead of constructing an alternative graph', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async (specification) => {
    const { runSwigAnalysis } = await import(
      new URL('/src/analysis/client.ts', window.location.href).href
    )
    return runSwigAnalysis(
      {
        ...specification,
        edges: [
          [0, 1],
          [1, 0],
        ],
      },
      ['A', 'Y'],
    )
  }, experiment)
  expect(result.ok).toBe(false)
})

test('the browser protocol rejects fields from another construction variant', () => {
  expect(
    swigSpecificationSchema.safeParse({ ...experiment, construction: { kind: 'swig', earlier: 0 } })
      .success,
  ).toBe(false)
})

test('DiD availability distinguishes untreated controls from treatment-affected controls', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { prepareDidSwig } = await import(
      new URL('/src/domain/swigDidDraft.ts', location.href).href
    )
    const { runSwigAnalysis } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const names = ['U', 'D', 'Y0', 'Y1', 'e0', 'e1', 'X0', 'X1']
    const graph = {
      names,
      origins: names.map((source) => ({ source, period: null })),
      measured: [1, 2, 3, 6, 7],
      boundaryArrows: 0,
      edges: [
        [0, 1],
        [0, 2],
        [0, 3],
        [1, 3],
        [4, 2],
        [5, 3],
        [6, 2],
        [7, 3],
      ],
    }
    const roles = [
      { kind: 'confounder' },
      { kind: 'treatment', period: 1 },
      { kind: 'outcome', period: 0 },
      { kind: 'outcome', period: 1 },
      { kind: 'disturbance' },
      { kind: 'disturbance' },
      { kind: 'covariate', period: 0 },
      { kind: 'covariate', period: 1 },
    ]
    const options = {
      adoption: 1,
      outcome: 1,
      comparison: { kind: 'neverTreated' },
      selected: [6, 7],
    }
    const ordinary = prepareDidSwig(graph, roles, options)
    const feedback = prepareDidSwig({ ...graph, edges: [...graph.edges, [1, 7]] }, roles, options)
    if (!ordinary.ok || !feedback.ok) throw Error('Invalid test specification')
    return {
      ordinary: await runSwigAnalysis(ordinary.value, names),
      feedback: await runSwigAnalysis(feedback.value, names),
    }
  })
  expect(result.ordinary).toMatchObject({
    ok: true,
    value: {
      conclusion: {
        kind: 'did',
        assessment: {
          decision: {
            kind: 'supportedSubjectToOverlap',
            required: [6, 7],
            selectedSupported: true,
          },
        },
      },
    },
  })
  expect(result.feedback).toMatchObject({
    ok: true,
    value: {
      conclusion: {
        kind: 'did',
        assessment: {
          decision: {
            kind: 'notEstablished',
            treatedBlockers: [
              { kind: 'conflictingAssignment', variable: 7, treatment: 1, required: 0, actual: 1 },
            ],
            comparisonBlockers: [],
          },
        },
      },
    },
  })
})
test('time expansion preserves lag direction, invariant causes and the explicit boundary count', async ({
  page,
}) => {
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const { projectSwigGraph } = await import(
      new URL('/src/domain/swigProjection.ts', location.href).href
    )
    const document = {
      current: {
        validation: { structure: { kind: 'valid' } },
        graph: {
          nodes: [
            { id: 'u', name: 'U', kind: 'latent' },
            { id: 'x', name: 'X', kind: 'observed' },
          ],
          edges: [
            { cause: 'u', effect: 'x', timing: { kind: 'same-period' } },
            { cause: 'x', effect: 'x', timing: { kind: 'lagged', lag: 1 } },
          ],
        },
      },
    }
    const projection = {
      kind: 'temporal',
      start: 0,
      end: 2,
      invariant: ['u'],
      boundary: 'closed-history',
    }
    return {
      expanded: projectSwigGraph(document, projection),
      unexpanded: projectSwigGraph(document, { kind: 'explicit' }),
      invalid: projectSwigGraph(document, { ...projection, start: NaN }),
    }
  })
  expect(results.expanded).toMatchObject({
    ok: true,
    value: {
      names: ['U', 'X [t=0]', 'X [t=1]', 'X [t=2]'],
      measured: [1, 2, 3],
      edges: [
        [0, 1],
        [0, 2],
        [0, 3],
        [1, 2],
        [2, 3],
      ],
      boundaryArrows: 1,
    },
  })
  expect(results.unexpanded).toMatchObject({
    ok: false,
    error: { kind: 'time-expansion-required' },
  })
  expect(results.invalid).toMatchObject({ ok: false, error: { kind: 'invalid-periods' } })
})
