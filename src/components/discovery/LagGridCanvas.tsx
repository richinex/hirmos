import { memo, useMemo } from 'react'
import '@xyflow/react/dist/style.css'
import { ReactFlow, type Node, type NodeProps } from '@xyflow/react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import { EChart } from '@/charts/EChart'
import { FlowControls } from '@/components/flow/FlowControls'

type GridNode = Node<{
  option: EChartsCoreOption
  label: string
  width: number
  height: number
  testId?: string
  onReady: (chart: EChartsType) => void
}, 'lag-grid'>

const Grid = memo(function Grid({ data }: NodeProps<GridNode>) {
  return <EChart option={data.option} label={data.label} className="block" style={{ width: data.width, height: data.height }} testId={data.testId} onReady={data.onReady} />
})

const NODE_TYPES = { 'lag-grid': Grid }
const FIT = { padding: 0.15, minZoom: 0.01, maxZoom: 1 }

/** Keep ECharts' fixed-coordinate drawing intact; the shared canvas owns pan and zoom. */
export function LagGridCanvas(props: GridNode['data']) {
  const { option, label, width, height, testId, onReady } = props
  const nodes = useMemo<GridNode[]>(() => [{
    id: 'lag-grid', type: 'lag-grid', position: { x: 0, y: 0 }, width, height,
    style: { pointerEvents: 'all' },
    data: { option, label, width, height, testId, onReady },
  }], [option, label, width, height, testId, onReady])

  return <ReactFlow<GridNode>
    nodes={nodes}
    nodeTypes={NODE_TYPES}
    nodesDraggable={false}
    nodesConnectable={false}
    nodesFocusable={false}
    elementsSelectable={false}
    fitView
    fitViewOptions={FIT}
    minZoom={0.01}
    maxZoom={3}
    zoomOnDoubleClick={false}
    panOnDrag
    zoomOnPinch
    deleteKeyCode={null}
    proOptions={{ hideAttribution: true }}
  >
    <FlowControls fit={FIT} fitLabel="Fit lag grid" />
  </ReactFlow>
}
