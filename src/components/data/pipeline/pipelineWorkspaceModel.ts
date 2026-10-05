import {
  blockArity,
  emptyBlock,
  pipelineBlockId,
  type AddableBlockKind,
  type PipelineBlock,
  type PipelineBlockId,
  type PipelineEdge,
  type PipelineGraph,
  type PipelineNode,
} from '@/domain/pipeline'
import type { SqlPreparationInput } from '@/domain/sourceInputs'

export const CARD_WIDTH = 192
export const CARD_HEIGHT = 86
const COLUMN_GAP = 40
const ROW_GAP = 120

/** A fresh canvas: one input card for each file given, or one empty card when none is, and the output card below. */
export function initialGraph(inputs: readonly SqlPreparationInput[]): PipelineGraph {
  const nodes: PipelineNode[] =
    inputs.length === 0
      ? [
          {
            id: pipelineBlockId('input-1'),
            block: { kind: 'input', file: { kind: 'empty' } },
            position: { x: 0, y: 0 },
          },
        ]
      : inputs.map((input, index) => ({
          id: pipelineBlockId(`input-${input.alias}`),
          block: { kind: 'input', file: { kind: 'chosen', alias: input.alias } },
          position: { x: index * (CARD_WIDTH + COLUMN_GAP), y: 0 },
        }))
  nodes.push({
    id: pipelineBlockId('output'),
    block: { kind: 'output' },
    position: { x: 0, y: ROW_GAP * 3 },
  })
  return { nodes, edges: [] }
}

/** Where a new input card goes: on the first row, to the right of the last input card. */
export function placeForInput(graph: PipelineGraph): { readonly x: number; readonly y: number } {
  const inputs = graph.nodes.filter((node) => node.block.kind === 'input')
  const rightmost = inputs.reduce(
    (best, node) => (node.position.x > best ? node.position.x : best),
    -(CARD_WIDTH + COLUMN_GAP),
  )
  return { x: rightmost + CARD_WIDTH + COLUMN_GAP, y: inputs[0]?.position.y ?? 0 }
}

export function addInputBlock(graph: PipelineGraph): {
  readonly graph: PipelineGraph
  readonly id: PipelineBlockId
} {
  const id = pipelineBlockId(`input-${crypto.randomUUID().slice(0, 8)}`)
  return {
    graph: {
      ...graph,
      nodes: [
        ...graph.nodes,
        { id, block: { kind: 'input', file: { kind: 'empty' } }, position: placeForInput(graph) },
      ],
    },
    id,
  }
}

/** Under the selected block, or under the lowest block that is not the output, so new blocks stack towards the output. */
export function placeFor(
  graph: PipelineGraph,
  selected: PipelineBlockId | null,
): { readonly x: number; readonly y: number } {
  const anchor =
    selected === null
      ? null
      : (graph.nodes.find((node) => node.id === selected && node.block.kind !== 'output') ?? null)
  if (anchor !== null) return { x: anchor.position.x, y: anchor.position.y + ROW_GAP }
  const lowest = graph.nodes
    .filter((node) => node.block.kind !== 'output')
    .reduce((best, node) => (node.position.y > best ? node.position.y : best), 0)
  return { x: 0, y: lowest + ROW_GAP }
}

/** Adds the block and, when it would land on or below the output block, moves the output down out of the way. */
export function addBlock(
  graph: PipelineGraph,
  kind: AddableBlockKind,
  position: { readonly x: number; readonly y: number },
): { readonly graph: PipelineGraph; readonly id: PipelineBlockId } {
  const id = pipelineBlockId(`${kind}-${crypto.randomUUID().slice(0, 8)}`)
  const nodes = graph.nodes.map((node) =>
    node.block.kind === 'output' &&
    node.position.y <= position.y + ROW_GAP / 2 &&
    Math.abs(node.position.x - position.x) < CARD_WIDTH
      ? { ...node, position: { x: node.position.x, y: position.y + ROW_GAP } }
      : node,
  )
  return { graph: { ...graph, nodes: [...nodes, { id, block: emptyBlock(kind), position }] }, id }
}

export function removeBlock(graph: PipelineGraph, id: PipelineBlockId): PipelineGraph {
  return {
    nodes: graph.nodes.filter((node) => node.id !== id),
    edges: graph.edges.filter((edge) => edge.from !== id && edge.to !== id),
  }
}

export function moveBlock(
  graph: PipelineGraph,
  id: PipelineBlockId,
  position: { readonly x: number; readonly y: number },
): PipelineGraph {
  return {
    ...graph,
    nodes: graph.nodes.map((node) => (node.id === id ? { ...node, position } : node)),
  }
}

export function configureBlock(
  graph: PipelineGraph,
  id: PipelineBlockId,
  block: PipelineBlock,
): PipelineGraph {
  return {
    ...graph,
    nodes: graph.nodes.map((node) => (node.id === id ? { ...node, block } : node)),
  }
}

export function removeEdge(graph: PipelineGraph, edge: PipelineEdge): PipelineGraph {
  return {
    ...graph,
    edges: graph.edges.filter(
      (candidate) =>
        !(candidate.from === edge.from && candidate.to === edge.to && candidate.port === edge.port),
    ),
  }
}

export type ConnectionRefusal =
  | { readonly kind: 'same-block' }
  | { readonly kind: 'unknown-block' }
  | { readonly kind: 'into-input' }
  | { readonly kind: 'out-of-output' }
  | { readonly kind: 'port-taken' }
  | { readonly kind: 'no-more-inputs' }
  | { readonly kind: 'would-loop' }

/** The graph indexed for the checks the canvas makes on every hover: blocks by id, arrows by the block they enter. */
export interface GraphIndex {
  readonly nodes: ReadonlyMap<PipelineBlockId, PipelineNode>
  readonly incoming: ReadonlyMap<PipelineBlockId, readonly PipelineEdge[]>
}

export function indexGraph(graph: PipelineGraph): GraphIndex {
  const incoming = new Map<PipelineBlockId, PipelineEdge[]>()
  for (const edge of graph.edges) {
    const own = incoming.get(edge.to)
    if (own === undefined) incoming.set(edge.to, [edge])
    else own.push(edge)
  }
  return { nodes: new Map(graph.nodes.map((node) => [node.id, node])), incoming }
}

export function connectionRefusal(index: GraphIndex, edge: PipelineEdge): ConnectionRefusal | null {
  if (edge.from === edge.to) return { kind: 'same-block' }
  const from = index.nodes.get(edge.from)
  const to = index.nodes.get(edge.to)
  if (from === undefined || to === undefined) return { kind: 'unknown-block' }
  if (to.block.kind === 'input') return { kind: 'into-input' }
  if (from.block.kind === 'output') return { kind: 'out-of-output' }
  const wired = index.incoming.get(edge.to) ?? []
  if (wired.some((candidate) => candidate.port === edge.port)) return { kind: 'port-taken' }
  const arity = blockArity(to.block.kind)
  if (arity.kind === 'exactly' && wired.length >= arity.count) return { kind: 'no-more-inputs' }
  // Walk upstream from the source: if the target is reached, the arrow would close a loop.
  const upstream = new Set<PipelineBlockId>()
  const pending = [edge.from]
  while (pending.length > 0) {
    const current = pending.pop()!
    if (current === edge.to) return { kind: 'would-loop' }
    if (upstream.has(current)) continue
    upstream.add(current)
    for (const candidate of index.incoming.get(current) ?? []) pending.push(candidate.from)
  }
  return null
}

export function describeConnectionRefusal(refusal: ConnectionRefusal): string {
  switch (refusal.kind) {
    case 'same-block':
      return 'A block cannot feed itself.'
    case 'unknown-block':
      return 'That block is no longer on the canvas.'
    case 'into-input':
      return 'An input file has no inputs of its own.'
    case 'out-of-output':
      return 'The output block is the end of the pipeline.'
    case 'port-taken':
      return 'This input already has an arrow. Remove it first.'
    case 'no-more-inputs':
      return 'This block takes no more inputs.'
    case 'would-loop':
      return 'This arrow would create a loop. Pipelines must flow in one direction.'
    default: {
      const exhaustive: never = refusal
      return exhaustive
    }
  }
}

export function connect(graph: PipelineGraph, edge: PipelineEdge): PipelineGraph {
  return { ...graph, edges: [...graph.edges, edge] }
}

/** A block that takes any number of inputs shows one port more than is wired, so there is always one to drop on. */
export function inputPorts(index: GraphIndex, node: PipelineNode): number {
  const arity = blockArity(node.block.kind)
  if (arity.kind === 'exactly') return arity.count
  return Math.max(arity.count, (index.incoming.get(node.id)?.length ?? 0) + 1)
}

export function inputsOf(index: GraphIndex, id: PipelineBlockId): readonly PipelineNode[] {
  return (index.incoming.get(id) ?? [])
    .slice()
    .sort((a, b) => a.port - b.port)
    .flatMap((edge) => {
      const node = index.nodes.get(edge.from)
      return node === undefined ? [] : [node]
    })
}

/** Tidy: the blocks laid out top to bottom in the order the arrows run, crossings minimised, inputs on the first row. dagre loads on the first use. */
export async function tidyGraph(graph: PipelineGraph): Promise<PipelineGraph> {
  const { default: dagre } = await import('@dagrejs/dagre')
  const layout = new dagre.graphlib.Graph()
  layout.setGraph({
    rankdir: 'TB',
    nodesep: COLUMN_GAP,
    ranksep: ROW_GAP - CARD_HEIGHT + 34,
    marginx: 0,
    marginy: 0,
  })
  layout.setDefaultEdgeLabel(() => ({}))
  for (const node of graph.nodes)
    layout.setNode(node.id, { width: CARD_WIDTH, height: CARD_HEIGHT })
  for (const edge of graph.edges) layout.setEdge(edge.from, edge.to)
  dagre.layout(layout)
  return {
    ...graph,
    nodes: graph.nodes.map((node) => {
      const placed = layout.node(node.id)
      return { ...node, position: { x: placed.x - CARD_WIDTH / 2, y: placed.y - CARD_HEIGHT / 2 } }
    }),
  }
}
