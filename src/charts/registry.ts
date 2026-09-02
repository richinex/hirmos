import { BarChart, CustomChart, GraphChart, HeatmapChart, LineChart, ScatterChart } from 'echarts/charts'
import {
  AriaComponent,
  AxisPointerComponent,
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
import { SVGRenderer } from 'echarts/renderers'

use([
  AriaComponent,
  AxisPointerComponent,
  BarChart,
  BrushComponent,
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

/** SVG only: every chart stays crisp, themeable, and exportable as text, and the canvas renderer stays out of the bundle. */
export const createChart = (element: HTMLElement): EChartsType =>
  init(element, undefined, { renderer: 'svg' })
