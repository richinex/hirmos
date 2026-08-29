import { BarChart, CustomChart, GraphChart, HeatmapChart, LineChart, ScatterChart } from 'echarts/charts'
import {
  AriaComponent,
  BrushComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  DatasetComponent,
  GridComponent,
  LegendComponent,
  MarkAreaComponent,
  MarkLineComponent,
  MarkPointComponent,
  TimelineComponent,
  TooltipComponent,
  TransformComponent,
  VisualMapComponent,
} from 'echarts/components'
import { init, use, type EChartsType } from 'echarts/core'
import { CanvasRenderer, SVGRenderer } from 'echarts/renderers'

use([
  AriaComponent,
  BarChart,
  BrushComponent,
  CanvasRenderer,
  CustomChart,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  DatasetComponent,
  GraphChart,
  GridComponent,
  HeatmapChart,
  LegendComponent,
  LineChart,
  MarkAreaComponent,
  MarkLineComponent,
  MarkPointComponent,
  ScatterChart,
  SVGRenderer,
  TimelineComponent,
  TooltipComponent,
  TransformComponent,
  VisualMapComponent,
])

export type ChartRenderer = 'svg' | 'canvas'

/** SVG by default so every chart is crisp, themeable, and exportable as text; canvas for very large series. */
export const createChart = (element: HTMLElement, renderer: ChartRenderer = 'svg'): EChartsType =>
  init(element, undefined, { renderer })
