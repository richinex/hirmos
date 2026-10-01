import dagre from '@dagrejs/dagre'

/** One card size for every node in a graph: the width the longest name needs, up to a cap, and the
 *  height its lines need. Measured before layout (dagCardSize.ts) so the engine places cards
 *  of the size that will be drawn, instead of a fixed box a long name is then clipped to. */
export interface DagCardSize {
  readonly width: number
  readonly height: number
  /** Lines the longest name takes at this width; every card leaves room for that many. */
  readonly nameLines: number
}

export const DEFAULT_CARD_SIZE: DagCardSize = { width: 164, height: 58, nameLines: 1 }

const COLUMN_GAP = 86
const ROW_GAP = 46
const MARGIN = 34

/**
 * The synchronous node layout for the ECharts causal-influence chart. The editable DAG canvas uses
 * ELK's joint placement and edge routing in elkLayout.ts instead.
 */
export type DagLayoutOrientation = 'across' | 'down'

export function layoutDirectedGraph<Id extends string>(nodes: readonly { readonly id: Id }[], edges: readonly { readonly cause: Id; readonly effect: Id }[], orientation: DagLayoutOrientation = 'across', size: DagCardSize = DEFAULT_CARD_SIZE): readonly { readonly id: Id; readonly x: number; readonly y: number }[] {
  const layout = new dagre.graphlib.Graph()
  layout.setGraph({ rankdir: orientation === 'across' ? 'LR' : 'TB', nodesep: ROW_GAP, ranksep: COLUMN_GAP, marginx: MARGIN, marginy: MARGIN })
  layout.setDefaultEdgeLabel(() => ({}))
  for (const node of nodes) layout.setNode(node.id, { width: size.width, height: size.height })
  for (const edge of edges) {
    if (edge.cause !== edge.effect) layout.setEdge(edge.cause, edge.effect)
  }
  dagre.layout(layout)
  return nodes.map((node) => {
    const placed = layout.node(node.id)
    return { id: node.id, x: placed.x - size.width / 2, y: placed.y - size.height / 2 }
  })
}
