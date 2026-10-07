import { expect, test } from '@playwright/test'
import { roleDetail } from '../src/domain/dagFlow'

test('role explanations qualify precision and post-treatment adjustment claims', () => {
  expect(roleDetail({ kind: 'pre-treatment' })).toBe(
    'A cause of treatment whose directed paths to the outcome, if any, pass through treatment. Adjusting may reduce precision. In linear models, it can amplify bias from remaining unmeasured confounding.',
  )
  expect(
    roleDetail({
      kind: 'post-treatment',
      relationship: { kind: 'other', adjustment: 'unchecked' },
    }),
  ).toBe(
    'A consequence of treatment outside its directed paths to the outcome. This relationship alone does not determine whether adjustment introduces bias.',
  )
})

/**
 * The causal flow analysis on the corpus from Octopus's test suite and the Mixtape DAGs the scott-c
 * notes teach with. Runs in the browser against the real domain module, as the method catalogue does.
 */
test('derives roles, flows, paths, and adjustment from treatment-to-outcome paths', async ({
  page,
}, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain corpus runs once')
  await page.goto('/app')
  const results: unknown = await page.evaluate(async () => {
    const flow = await import(new URL('/src/domain/dagFlow.ts', window.location.href).href)
    const { validateAdjustmentSets } = await import(
      new URL('/src/analysis/client.ts', window.location.href).href
    )
    const analyse = async (g: any, treatment: string, outcome: string) => {
      const index = (id: string) => g.nodes.findIndex((node: any) => node.id === id)
      const result = await validateAdjustmentSets({
        nodes: g.nodes.length,
        edges: g.edges.map((edge: any) => [index(edge.cause), index(edge.effect)]),
        treatment: index(treatment),
        outcome: index(outcome),
        unobserved: g.nodes.flatMap((node: any, i: number) => (node.kind === 'latent' ? [i] : [])),
        sets: [],
      })
      if (!result.ok) throw Error(JSON.stringify(result.error))
      const a = result.value.analysis
      const adjustment =
        a.kind === 'notIdentified'
          ? { kind: 'none' }
          : a.emptyValid
            ? { kind: 'unnecessary' }
            : { kind: 'sufficient', variables: a.canonicalSet.map((i: number) => g.nodes[i].id) }
      return flow.analyseDagCausalFlow(g, treatment, outcome, adjustment)
    }

    const graph = (
      nodes: readonly string[],
      edges: readonly (readonly [string, string])[],
      latent: readonly string[] = [],
    ) => ({
      kind: 'editable-dag',
      nodes: nodes.map((id) =>
        latent.includes(id)
          ? { kind: 'latent', id, name: id }
          : { kind: 'observed', id, column: `0:${id}`, name: id },
      ),
      edges: edges.map(([cause, effect], index) => ({
        kind: 'directed',
        id: `e${index}`,
        cause,
        effect,
        timing: { kind: 'contemporaneous' },
        support: { kind: 'unstated' },
        evidence: [],
      })),
    })
    const summarise = async (g: unknown, treatment: string, outcome: string) => {
      const analysis = await analyse(g, treatment, outcome)
      return {
        adjustment: analysis.adjustment,
        roles: Object.fromEntries(
          [...analysis.roles.entries()].map(
            ([id, role]: [string, { kind: string; alsoCollider?: boolean }]) => [
              id,
              role.kind === 'mediator' && role.alsoCollider === true
                ? 'mediator-collider'
                : role.kind,
            ],
          ),
        ),
        edges: Object.fromEntries([...analysis.edges.entries()]),
        paths: analysis.paths.map(
          (path: { nodes: readonly string[]; status: { kind: string } }) =>
            `${path.nodes.join('-')}:${path.status.kind}`,
        ),
      }
    }
    return {
      triangle: await summarise(
        graph(
          ['health', 'outreach', 'revenue'],
          [
            ['health', 'outreach'],
            ['health', 'revenue'],
            ['outreach', 'revenue'],
          ],
        ),
        'outreach',
        'revenue',
      ),
      chain: await summarise(
        graph(
          ['outreach', 'health', 'revenue'],
          [
            ['outreach', 'health'],
            ['health', 'revenue'],
          ],
        ),
        'outreach',
        'revenue',
      ),
      latent: await summarise(
        graph(
          ['u', 'outreach', 'revenue'],
          [
            ['u', 'outreach'],
            ['u', 'revenue'],
            ['outreach', 'revenue'],
          ],
          ['u'],
        ),
        'outreach',
        'revenue',
      ),
      collider: await summarise(
        graph(
          ['outreach', 'revenue', 'adoption'],
          [
            ['outreach', 'revenue'],
            ['outreach', 'adoption'],
            ['revenue', 'adoption'],
          ],
        ),
        'outreach',
        'revenue',
      ),
      humanCapital: await summarise(
        graph(
          ['D', 'Y', 'I', 'B'],
          [
            ['D', 'Y'],
            ['I', 'D'],
            ['I', 'Y'],
            ['B', 'I'],
            ['B', 'D'],
          ],
          ['B'],
        ),
        'D',
        'Y',
      ),
      multiple: await summarise(
        graph(
          ['D', 'Y', 'X1', 'X2', 'X3'],
          [
            ['D', 'Y'],
            ['X1', 'D'],
            ['X1', 'X3'],
            ['X2', 'D'],
            ['X2', 'X3'],
            ['X3', 'Y'],
          ],
        ),
        'D',
        'Y',
      ),
      movieStar: await summarise(
        graph(
          ['Beauty', 'Talent', 'Star'],
          [
            ['Beauty', 'Star'],
            ['Talent', 'Star'],
          ],
        ),
        'Beauty',
        'Talent',
      ),
      discrimination: await summarise(
        graph(
          ['D', 'O', 'Y', 'A'],
          [
            ['D', 'O'],
            ['D', 'Y'],
            ['O', 'Y'],
            ['A', 'O'],
            ['A', 'Y'],
          ],
          ['A'],
        ),
        'D',
        'Y',
      ),
    }
  })
  const r = results as Record<
    string,
    {
      adjustment: { kind: string; variables?: string[] }
      roles: Record<string, string>
      edges: Record<string, { causal: boolean; biasing: boolean }>
      paths: string[]
    }
  >

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

/**
 * Cinelli, Forney and Pearl (2022), "A Crash Course in Good and Bad Controls": the role and the
 * adjustment verdict for Z in each of the paper's graphs, Models 1 to 18 and the two variations.
 */
test('classifies Z in every graph of the good and bad controls crash course', async ({
  page,
}, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain corpus runs once')
  await page.goto('/app')
  const models: Record<string, readonly [string, readonly string[], string, string]> = {
    '1': ['Z>X Z>Y X>Y', [], 'confounder', 'sufficient'],
    '2': ['U>Z Z>X U>Y X>Y', ['U'], 'backdoor-variable', 'sufficient'],
    '3': ['U>X U>Z Z>Y X>Y', ['U'], 'backdoor-variable', 'sufficient'],
    '4': ['Z>X Z>M X>M M>Y', [], 'confounder', 'sufficient'],
    '5': ['U>Z Z>X U>M X>M M>Y', ['U'], 'backdoor-variable', 'sufficient'],
    '6': ['U>X U>Z Z>M X>M M>Y', ['U'], 'backdoor-variable', 'sufficient'],
    '7': ['U1>X U1>Z U2>Z U2>Y X>Y', ['U1', 'U2'], 'collider', 'unnecessary'],
    '7, variation': ['U1>X U1>Z U2>Z U2>Y Z>Y X>Y', ['U1', 'U2'], 'backdoor-variable', 'none'],
    '8': ['Z>Y X>Y', [], 'outcome-predictor', 'unnecessary'],
    '9': ['Z>X X>Y', [], 'pre-treatment', 'unnecessary'],
    '10': ['Z>X U>X U>Y X>Y', ['U'], 'pre-treatment', 'none'],
    '11': ['X>Z Z>Y', [], 'mediator', 'unnecessary'],
    '11, variation': ['X>Z Z>Y U>Z U>Y', ['U'], 'mediator-collider', 'unnecessary'],
    '12': ['X>M M>Y M>Z', [], 'post-treatment', 'unnecessary'],
    '13': ['X>M Z>M M>Y', [], 'outcome-predictor', 'unnecessary'],
    '14': ['X>Y X>Z', [], 'post-treatment', 'unnecessary'],
    '15': ['X>Y X>Z Z>W U>W U>Y', ['U'], 'post-treatment', 'unnecessary'],
    '16': ['X>Y X>Z U>Z U>Y', ['U'], 'collider', 'unnecessary'],
    '17': ['X>Y X>Z Y>Z', [], 'collider', 'unnecessary'],
    '18': ['X>Y Y>Z', [], 'post-treatment', 'unnecessary'],
  }
  const results: Record<
    string,
    { role: string; adjustment: { kind: string; variables?: string[] } }
  > = await page.evaluate(async (models) => {
    const flow = await import(new URL('/src/domain/dagFlow.ts', window.location.href).href)
    const { validateAdjustmentSets } = await import(
      new URL('/src/analysis/client.ts', window.location.href).href
    )
    const analyse = async (g: any, treatment: string, outcome: string) => {
      const index = (id: string) => g.nodes.findIndex((node: any) => node.id === id)
      const result = await validateAdjustmentSets({
        nodes: g.nodes.length,
        edges: g.edges.map((edge: any) => [index(edge.cause), index(edge.effect)]),
        treatment: index(treatment),
        outcome: index(outcome),
        unobserved: g.nodes.flatMap((node: any, i: number) => (node.kind === 'latent' ? [i] : [])),
        sets: [],
      })
      if (!result.ok) throw Error(JSON.stringify(result.error))
      const a = result.value.analysis
      const adjustment =
        a.kind === 'notIdentified'
          ? { kind: 'none' }
          : a.emptyValid
            ? { kind: 'unnecessary' }
            : { kind: 'sufficient', variables: a.canonicalSet.map((i: number) => g.nodes[i].id) }
      return flow.analyseDagCausalFlow(g, treatment, outcome, adjustment)
    }

    return Object.fromEntries(
      await Promise.all(
        Object.entries(models).map(async ([model, [arrows, latent]]) => {
          const edges = arrows.split(' ').map((arrow) => arrow.split('>'))
          const names = [...new Set(edges.flat())]
          const analysis = await analyse(
            {
              kind: 'editable-dag',
              nodes: names.map((id) =>
                latent.includes(id)
                  ? { kind: 'latent', id, name: id }
                  : { kind: 'observed', id, column: `0:${id}`, name: id },
              ),
              edges: edges.map(([cause, effect], index) => ({
                kind: 'directed',
                id: `e${index}`,
                cause,
                effect,
                timing: { kind: 'contemporaneous' },
                support: { kind: 'unstated' },
                evidence: [],
              })),
            },
            'X',
            'Y',
          )
          const role = analysis.roles.get('Z')
          return [
            model,
            {
              role: role.kind === 'mediator' && role.alsoCollider ? 'mediator-collider' : role.kind,
              adjustment: analysis.adjustment,
            },
          ]
        }),
      ),
    )
  }, models)
  for (const [model, [, , role, adjustment]] of Object.entries(models)) {
    expect(results[model].role, `Model ${model}`).toBe(role)
    expect(results[model].adjustment.kind, `Model ${model}`).toBe(adjustment)
    if (adjustment === 'sufficient')
      expect(results[model].adjustment.variables, `Model ${model}`).toEqual(['Z'])
  }
})
