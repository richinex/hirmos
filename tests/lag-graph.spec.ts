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
