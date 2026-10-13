import type { PipelineBlockId, PipelineGraph, PipelineNode } from '@/domain/pipeline'

export type BlockChange = 'added' | 'changed' | 'kept'

export interface FlowStep {
  readonly node: PipelineNode
  /** Null when there was no open pipeline to compare with. */
  readonly change: BlockChange | null
  /** The blocks wired into this one, in port order; shown only where there is more than one. */
  readonly inputs: readonly PipelineNode[]
}

/** Upstream before downstream, ties kept in the proposal's own order, so the chain reads from file to source. */
function ordered(graph: PipelineGraph): readonly PipelineNode[] {
  const waiting = new Map<PipelineBlockId, number>(graph.nodes.map((node) => [node.id, 0]))
  for (const edge of graph.edges) waiting.set(edge.to, (waiting.get(edge.to) ?? 0) + 1)
  const order: PipelineNode[] = []
  const placed = new Set<PipelineBlockId>()
  while (order.length < graph.nodes.length) {
    const next = graph.nodes.find((node) => !placed.has(node.id) && waiting.get(node.id) === 0)
    if (next === undefined) return [...order, ...graph.nodes.filter((node) => !placed.has(node.id))]
    order.push(next)
    placed.add(next.id)
    for (const edge of graph.edges)
      if (edge.from === next.id) waiting.set(edge.to, (waiting.get(edge.to) ?? 1) - 1)
  }
  return order
}

export function proposalFlow(
  proposed: PipelineGraph,
  current: PipelineGraph | null,
): { readonly steps: readonly FlowStep[]; readonly removed: readonly PipelineNode[] } {
  const before = new Map(current?.nodes.map((node) => [node.id, node]) ?? [])
  const byId = new Map(proposed.nodes.map((node) => [node.id, node]))
  const steps = ordered(proposed).map((node): FlowStep => {
    const previous = before.get(node.id)
    return {
      node,
      change:
        current === null
          ? null
          : previous === undefined
            ? 'added'
            : JSON.stringify(previous.block) === JSON.stringify(node.block)
              ? 'kept'
              : 'changed',
      inputs: proposed.edges
        .filter((edge) => edge.to === node.id)
        .sort((a, b) => a.port - b.port)
        .flatMap((edge) => byId.get(edge.from) ?? []),
    }
  })
  return {
    steps,
    removed: current?.nodes.filter((node) => !byId.has(node.id)) ?? [],
  }
}
