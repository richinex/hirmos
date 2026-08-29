import { expect, test } from '@playwright/test'

/**
 * The causal flow analysis on the corpus from Octopus's test suite and the Mixtape DAGs the scott-c
 * notes teach with. Runs in the browser against the real domain module, as the method catalogue does.
 */
test('derives roles, flows, paths, and adjustment from treatment-to-outcome paths', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain corpus runs once')
  await page.goto('/app')
  const results: unknown = await page.evaluate(async () => {
    const flow = await import(new URL('/src/domain/dagFlow.ts', window.location.href).href)
    const graph = (nodes: readonly string[], edges: readonly (readonly [string, string])[], latent: readonly string[] = []) => ({
      kind: 'editable-dag',
      nodes: nodes.map((id) => (latent.includes(id) ? { kind: 'latent', id, name: id } : { kind: 'observed', id, column: `0:${id}`, name: id })),
      edges: edges.map(([cause, effect], index) => ({ kind: 'directed', id: `e${index}`, cause, effect, timing: { kind: 'contemporaneous' }, support: { kind: 'unstated' }, evidence: [] })),
    })
    const summarise = (g: unknown, treatment: string, outcome: string) => {
      const analysis = flow.analyseDagCausalFlow(g, treatment, outcome)
      return {
        adjustment: analysis.adjustment,
        roles: Object.fromEntries([...analysis.roles.entries()].map(([id, role]: [string, { kind: string; alsoCollider?: boolean }]) => [id, role.kind === 'mediator' && role.alsoCollider === true ? 'mediator-collider' : role.kind])),
        edges: Object.fromEntries([...analysis.edges.entries()]),
        paths: analysis.paths.map((path: { nodes: readonly string[]; status: { kind: string } }) => `${path.nodes.join('-')}:${path.status.kind}`),
      }
    }
    return {
      triangle: summarise(graph(['health', 'outreach', 'revenue'], [['health', 'outreach'], ['health', 'revenue'], ['outreach', 'revenue']]), 'outreach', 'revenue'),
      chain: summarise(graph(['outreach', 'health', 'revenue'], [['outreach', 'health'], ['health', 'revenue']]), 'outreach', 'revenue'),
      latent: summarise(graph(['u', 'outreach', 'revenue'], [['u', 'outreach'], ['u', 'revenue'], ['outreach', 'revenue']], ['u']), 'outreach', 'revenue'),
      collider: summarise(graph(['outreach', 'revenue', 'adoption'], [['outreach', 'revenue'], ['outreach', 'adoption'], ['revenue', 'adoption']]), 'outreach', 'revenue'),
      humanCapital: summarise(graph(['D', 'Y', 'I', 'B'], [['D', 'Y'], ['I', 'D'], ['I', 'Y'], ['B', 'I'], ['B', 'D']], ['B']), 'D', 'Y'),
      multiple: summarise(graph(['D', 'Y', 'X1', 'X2', 'X3'], [['D', 'Y'], ['X1', 'D'], ['X1', 'X3'], ['X2', 'D'], ['X2', 'X3'], ['X3', 'Y']]), 'D', 'Y'),
      movieStar: summarise(graph(['Beauty', 'Talent', 'Star'], [['Beauty', 'Star'], ['Talent', 'Star']]), 'Beauty', 'Talent'),
      discrimination: summarise(graph(['D', 'O', 'Y', 'A'], [['D', 'O'], ['D', 'Y'], ['O', 'Y'], ['A', 'O'], ['A', 'Y']], ['A']), 'D', 'Y'),
    }
  })
  const r = results as Record<string, { adjustment: { kind: string; variables?: string[] }; roles: Record<string, string>; edges: Record<string, { causal: boolean; biasing: boolean }>; paths: string[] }>

  expect(r.triangle.edges.e2).toEqual({ causal: true, biasing: false })
  expect(r.triangle.edges.e0).toEqual({ causal: false, biasing: true })
  expect(r.triangle.edges.e1.biasing).toBe(true)
  expect(r.triangle.adjustment).toEqual({ kind: 'sufficient', variables: ['health'] })
  expect(r.triangle.roles.health).toBe('confounder')
  expect(r.triangle.paths).toContain('outreach-health-revenue:closed-by-adjustment')

  expect(Object.values(r.chain.edges).every((edge) => edge.causal && !edge.biasing)).toBe(true)
  expect(r.chain.adjustment.kind).toBe('unnecessary')
  expect(r.chain.roles.health).toBe('mediator')

  expect(r.latent.adjustment.kind).toBe('none')
  expect(r.latent.edges.e0.biasing).toBe(true)
  expect(r.latent.roles.u).toBe('unmeasured')
  expect(r.latent.paths).toContain('outreach-u-revenue:open-through-unmeasured')

  expect(r.collider.edges.e1).toEqual({ causal: false, biasing: false })
  expect(r.collider.adjustment.kind).toBe('unnecessary')
  expect(r.collider.roles.adoption).toBe('collider')

  // Mixtape figure: family income I closes the back door; unmeasured background B is blocked by I.
  expect(r.humanCapital.adjustment).toEqual({ kind: 'sufficient', variables: ['I'] })
  expect(r.humanCapital.roles.I).toBe('confounder')

  // Two sufficient sets exist; the canonical set is the union of all three.
  expect(r.multiple.adjustment).toEqual({ kind: 'sufficient', variables: ['X1', 'X2', 'X3'] })
  expect(r.multiple.roles.X1).toBe('confounder')
  expect(r.multiple.roles.X3).toBe('backdoor-variable')

  // Star is a collider between beauty and talent; no adjustment is needed and adjusting would open the path.
  expect(r.movieStar.roles.Star).toBe('collider')
  expect(r.movieStar.adjustment.kind).toBe('unnecessary')

  // Occupation is a mediator and a collider; nothing points into D, so the total effect needs no
  // adjustment, and adjusting for O would open D – O – A – Y through unmeasured ability.
  expect(r.discrimination.roles.O).toBe('mediator-collider')
  expect(r.discrimination.adjustment.kind).toBe('unnecessary')
  expect(r.discrimination.paths).toContain('D-O-A-Y:closed-at-collider')
})
