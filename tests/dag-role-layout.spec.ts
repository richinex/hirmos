import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

/**
 * Once a treatment and an outcome are chosen, cards are placed by role (roleLayout.ts) and then
 * separated where they overlap (separateCards in elkLayout.ts). On dagitty's
 * published example graphs, with their own exposure and outcome, in both orientations: no two cards
 * overlap, the longest causal path lies on one line, confounders sit on one side of it and colliders
 * on the other.
 */

const corpus = JSON.parse(readFileSync(new URL('./fixtures/dagitty/examples.json', import.meta.url), 'utf8'))

test('role placement keeps the causal line straight and the roles on their sides', async ({ page }) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  const failures = await page.evaluate(async (fixtures) => {
    const modules = { roles: '/src/components/dag/roleLayout.ts', flow: '/src/domain/dagFlow.ts', elk: '/src/components/dag/elkLayout.ts' }
    const { placeByRole } = await import(/* @vite-ignore */ modules.roles)
    const { analyseDagCausalFlow } = await import(/* @vite-ignore */ modules.flow)
    const { separateCards } = await import(/* @vite-ignore */ modules.elk)
    const size = { width: 164, height: 58, nameLines: 1 }
    const failures: string[] = []
    for (const orientation of ['across', 'down'] as const) {
      for (const fixture of fixtures) {
        const label = `${fixture.label}/${orientation}`
        const graph = {
          nodes: fixture.nodes.map((n: { name: string; kind: string }) => ({ id: n.name, name: n.name, kind: n.kind, column: n.name })),
          edges: fixture.edges.map((e: { from: string; to: string }, i: number) => ({ id: `e${i}`, cause: e.from, effect: e.to, timing: { kind: 'contemporaneous' } })),
        }
        const flow = analyseDagCausalFlow(graph, fixture.exposure, fixture.outcome)
        const byRoles = placeByRole(graph, flow, orientation, size)
        // The canvas separates any overlapping cards after placing them by role.
        const placed = byRoles.ok ? await separateCards(byRoles.value, size) : byRoles
        if (!placed.ok) { failures.push(`${label}: ${placed.error.kind}`); continue }
        const at = placed.value as Map<string, { x: number; y: number }>
        const ids = [...at.keys()]
        for (let i = 0; i < ids.length; i++) for (let j = i + 1; j < ids.length; j++) {
          const a = at.get(ids[i])!, b = at.get(ids[j])!
          if (Math.abs(a.x - b.x) < size.width && Math.abs(a.y - b.y) < size.height) failures.push(`${label}: ${ids[i]} overlaps ${ids[j]}`)
        }
        // Positions across the line: y when the line runs across, x when it runs down.
        const across = (id: string) => (orientation === 'across' ? at.get(id)!.y : at.get(id)!.x)
        const line = across(fixture.exposure)
        const causal = flow.paths.filter((path: { type: string }) => path.type === 'causal')
          .reduce((longest: { nodes: string[] }, path: { nodes: string[] }) => (path.nodes.length > longest.nodes.length ? path : longest), { nodes: [fixture.exposure, fixture.outcome] })
        for (const id of causal.nodes) if (Math.abs(across(id) - line) > 1) failures.push(`${label}: ${id} is off the causal line`)
        for (const [id, role] of flow.roles as Map<string, { kind: string }>) {
          if (role.kind === 'confounder' && across(id) >= line) failures.push(`${label}: confounder ${id} is not on the confounder side`)
          if (role.kind === 'collider' && across(id) <= line) failures.push(`${label}: collider ${id} is not on the collider side`)
        }
      }
    }
    return failures
  }, corpus)
  expect(failures).toEqual([])
})
