import { expect, test } from '@playwright/test'
import type { LagLink } from '@/domain/lagGraph'

test('projects conflict endpoints and keeps links with unsupported marks', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Pure lag-graph projection contract runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const { lagGraphFromMarkedMatrices } = await import(new URL('/src/domain/lagGraph.ts', window.location.href).href)
    const projection = lagGraphFromMarkedMatrices(
      [
        { id: 'v1', name: 'v1', latent: false },
        { id: 'v2', name: 'v2', latent: false },
        { id: 'v3', name: 'v3', latent: false },
      ],
      [
        [[''], ['x-x'], ['+->']],
        [['x-x'], [''], ['-->']],
        [['<-+'], ['<--'], ['']],
      ],
      [
        [[0], [0.2], [0.3]],
        [[0.2], [0], [0.4]],
        [[0.3], [0.4], [0]],
      ],
      0,
      'stationary-lag-graph',
    )
    return {
      links: projection.graph.links.map((link: LagLink) => ({
        from: link.from,
        to: link.to,
        fromEndpoint: link.fromEndpoint,
        toEndpoint: link.toEndpoint,
        mark: link.mark,
      })),
      warnings: projection.warnings,
    }
  })

  expect(result.links).toEqual([
    { from: 0, to: 1, fromEndpoint: 'conflict', toEndpoint: 'conflict', mark: 'x-x' },
    { from: 0, to: 2, fromEndpoint: 'unresolved', toEndpoint: 'arrow', mark: '+->' },
    { from: 1, to: 2, fromEndpoint: 'tail', toEndpoint: 'arrow', mark: '-->' },
  ])
  expect(result.warnings).toEqual([
    { kind: 'unsupported-mark', source: 0, target: 2, lag: 0, mark: '+->' },
  ])
})

test('keeps CD-NOTS context nodes and GRACE gates in the shared lag graph', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Pure lag-graph projection contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { lagGraphFromRun } = await import(new URL('/src/domain/lagGraph.ts', window.location.href).href)
    const stringCube = () => Array.from({ length: 3 }, () => Array.from({ length: 3 }, () => ['', '']))
    const numberCube = () => Array.from({ length: 3 }, () => Array.from({ length: 3 }, () => [0, 0]))
    const graph = stringCube()
    graph[0][1][1] = '-->'
    graph[0][2][0] = '<--'
    const values = numberCube()
    values[0][1][1] = 0.72
    values[0][2][0] = -0.41
    const cdnots = lagGraphFromRun({
      kind: 'cdnots-run', id: 'run-cdn', variables: [{ id: 'a', name: 'A' }, { id: 'b', name: 'B' }],
      result: { observedVariables: 2, contextVariables: ['C_lin'], maxLag: 1, graph, valMatrix: values },
    })
    const active = Array.from({ length: 2 }, () => Array.from({ length: 2 }, () => [false, false]))
    const gates = Array.from({ length: 2 }, () => Array.from({ length: 2 }, () => [0, 0]))
    active[1][0][1] = true
    gates[1][0][1] = 0.83
    const grace = lagGraphFromRun({
      kind: 'grace-run', variables: [{ id: 'a', name: 'A' }, { id: 'b', name: 'B' }],
      result: { variables: 2, maxLag: 1, graph: active, gateValues: gates },
    })
    return {
      cdnots: {
        semantics: cdnots.graph.semantics,
        variables: cdnots.graph.variables.map((variable: { readonly id: string; readonly name: string }) => ({ id: variable.id, name: variable.name })),
        links: cdnots.graph.links.map((link: LagLink) => ({ from: link.from, to: link.to, lag: link.lag, mark: link.mark })),
      },
      grace: {
        semantics: grace.graph.semantics,
        links: grace.graph.links.map((link: LagLink) => ({ from: link.from, to: link.to, lag: link.lag, strength: link.strength })),
      },
    }
  })
  expect(result.cdnots).toEqual({
    semantics: 'nonstationary-lag-graph',
    variables: [{ id: 'a', name: 'A' }, { id: 'b', name: 'B' }, { id: 'context:run-cdn:C_lin', name: 'C_lin' }],
    links: [{ from: 0, to: 1, lag: 1, mark: '-->' }, { from: 0, to: 2, lag: 0, mark: '<--' }],
  })
  expect(result.grace).toEqual({
    semantics: 'neural-lagged-granger',
    links: [{ from: 1, to: 0, lag: 1, strength: { kind: 'nonnegative', value: 0.83 } }],
  })
})

test('keeps every PAG endpoint pair unresolved unless FCI reports a directed edge', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Pure PAG projection contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { lagGraphFromRun } = await import(new URL('/src/domain/lagGraph.ts', window.location.href).href)
    const { discoveryEvidenceView } = await import(new URL('/src/domain/dagEvidence.ts', window.location.href).href)
    const marks = ['---', '-->', '--o', '<--', '<->', '<-o', 'o--', 'o->', 'o-o']
    const variables = Array.from({ length: marks.length + 1 }, (_, index) => ({ id: `v${index}`, name: `V${index}` }))
    const graph = Array.from(
      { length: variables.length },
      () => Array.from({ length: variables.length }, () => ['']),
    )
    marks.forEach((mark, index) => {
      graph[0][index + 1][0] = mark
      graph[index + 1][0][0] = `${mark[2]}-${mark[0]}`
    })
    const run = {
      kind: 'fci-run', id: 'fci-endpoints', preparedDataset: 'prepared', createdAt: '2026-09-04T00:00:00.000Z',
      method: 'fci', variables, eligibility: { kind: 'eligible', satisfied: [] },
      result: {
        kind: 'fci', observations: 200, variables: variables.length, alpha: 0.05, maxDepth: null,
        maxPathLength: null, ciTest: 'fisherZ', graph, separatingSets: [], ciTests: [], edgeProperties: [],
      },
    }
    const projected = lagGraphFromRun(run)
    const evidence = discoveryEvidenceView(run)
    return {
      semantics: projected.graph.semantics,
      marks: projected.graph.links.map((link: LagLink) => link.mark),
      endpoints: projected.graph.links.map((link: LagLink) => [link.fromEndpoint, link.toEndpoint]),
      matches: evidence.candidates.map((candidate: { readonly relationMatch: { readonly kind: string } }) => candidate.relationMatch.kind),
    }
  })
  expect(result.semantics).toBe('pag')
  expect(result.marks).toEqual(['---', '-->', '--o', '<--', '<->', '<-o', 'o--', 'o->', 'o-o'])
  expect(result.endpoints).toEqual([
    ['tail', 'tail'], ['tail', 'arrow'], ['tail', 'circle'],
    ['arrow', 'tail'], ['arrow', 'arrow'], ['arrow', 'circle'],
    ['circle', 'tail'], ['circle', 'arrow'], ['circle', 'circle'],
  ])
  expect(result.matches).toEqual([
    'orientation-unresolved', 'directed-candidate', 'orientation-unresolved',
    'directed-candidate', 'orientation-unresolved', 'orientation-unresolved',
    'orientation-unresolved', 'orientation-unresolved', 'orientation-unresolved',
  ])
})
