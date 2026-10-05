import { z } from 'zod'
import type { DagDocument } from './dag'
import { err, ok, type Result } from './dop'
export const swigProjectionSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('explicit') }).strict(),
  z
    .object({
      kind: z.literal('temporal'),
      start: z.number().int().min(-32).max(32),
      end: z.number().int().min(-32).max(32),
      invariant: z.array(z.string()).max(256),
      boundary: z.literal('closed-history'),
    })
    .strict(),
])
export type SwigProjection = z.infer<typeof swigProjectionSchema>
export type SwigGraphProblem =
  | { readonly kind: 'invalid-graph' }
  | { readonly kind: 'time-expansion-required' }
  | { readonly kind: 'too-many-variables' }
  | { readonly kind: 'invalid-periods' }
  | { readonly kind: 'invalid-invariance' }
export interface ProjectedSwigGraph {
  readonly names: readonly string[]
  readonly edges: [number, number][]
  readonly measured: readonly number[]
  readonly origins: readonly { readonly source: string; readonly period: number | null }[]
  readonly boundaryArrows: number
}
/** Finite time expansion. Initial-history closure is a recorded assumption, not inferred independence. */
export function projectSwigGraph(
  document: DagDocument,
  projection: SwigProjection,
): Result<ProjectedSwigGraph, SwigGraphProblem> {
  const graph = document.current.graph
  if (document.current.validation.structure.kind === 'invalid')
    return err({ kind: 'invalid-graph' })
  if (projection.kind === 'explicit') {
    if (graph.edges.some((edge) => edge.timing.kind === 'lagged'))
      return err({ kind: 'time-expansion-required' })
    if (graph.nodes.length > 256) return err({ kind: 'too-many-variables' })
    const positions = new Map(graph.nodes.map((node, i) => [node.id, i]))
    const edges: [number, number][] = []
    for (const edge of graph.edges) {
      const a = positions.get(edge.cause),
        b = positions.get(edge.effect)
      if (a === undefined || b === undefined) return err({ kind: 'invalid-graph' })
      edges.push([a, b])
    }
    return ok({
      names: graph.nodes.map((n) => n.name),
      edges,
      measured: graph.nodes.flatMap((n, i) => (n.kind === 'observed' ? [i] : [])),
      origins: graph.nodes.map((n) => ({ source: n.id, period: null })),
      boundaryArrows: 0,
    })
  }
  if (
    !Number.isInteger(projection.start) ||
    !Number.isInteger(projection.end) ||
    projection.start < -32 ||
    projection.end > 32 ||
    projection.end <= projection.start ||
    projection.end - projection.start > 32
  )
    return err({ kind: 'invalid-periods' })
  const invariant = new Set(projection.invariant)
  if (
    invariant.size !== projection.invariant.length ||
    [...invariant].some((id) => !graph.nodes.some((n) => n.id === id)) ||
    graph.edges.some(
      (e) => invariant.has(e.effect) && (!invariant.has(e.cause) || e.timing.kind === 'lagged'),
    )
  )
    return err({ kind: 'invalid-invariance' })
  const names: string[] = [],
    origins: { source: string; period: number | null }[] = [],
    measured: number[] = []
  const positions = new Map<string, number>()
  const key = (id: string, period: number | null) => JSON.stringify([id, period])
  for (const node of graph.nodes) {
    const periods: (number | null)[] = invariant.has(node.id)
      ? [null]
      : Array.from(
          { length: projection.end - projection.start + 1 },
          (_, i) => projection.start + i,
        )
    for (const period of periods) {
      positions.set(key(node.id, period), names.length)
      if (node.kind === 'observed') measured.push(names.length)
      origins.push({ source: node.id, period })
      names.push(period === null ? node.name : `${node.name} [t=${period}]`)
    }
  }
  if (names.length > 256) return err({ kind: 'too-many-variables' })
  const edges: [number, number][] = [],
    seen = new Set<string>()
  let boundaryArrows = 0
  for (const edge of graph.edges) {
    for (let t = projection.start; t <= projection.end; t++) {
      const causeTime = invariant.has(edge.cause)
        ? null
        : t - (edge.timing.kind === 'lagged' ? edge.timing.lag : 0)
      const effectTime = invariant.has(edge.effect) ? null : t
      if (causeTime !== null && causeTime < projection.start) {
        boundaryArrows++
        continue
      }
      const a = positions.get(key(edge.cause, causeTime)),
        b = positions.get(key(edge.effect, effectTime))
      if (a === undefined || b === undefined) return err({ kind: 'invalid-graph' })
      const identity = `${a}:${b}`
      if (!seen.has(identity)) {
        seen.add(identity)
        edges.push([a, b])
      }
    }
  }
  return ok({ names, edges, origins, measured, boundaryArrows })
}
