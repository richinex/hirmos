import type { EChartsCoreOption } from 'echarts/core'
import {
  describeStrength,
  strengthMagnitude,
  type LagEndpoint,
  type LagGraph,
  type LagLinkStrength,
  type SummaryGraph,
} from '@/domain/lagGraph'
import { assertNever } from '@/domain/dop'
import { baseOption, escapeHtml, tooltip } from '../grammar'
import type { ChartTheme } from '../theme'

/**
 * The two tigramite views as ECharts `graph` series with fixed positions: the lag grid (variables by
 * lag columns, links repeated across the window) and the summary graph (one curved link per ordered
 * pair on a circle, lag list as its label). Endpoint marks keep PAG semantics: tail, arrowhead, or
 * open circle at each end.
 */

const parseHex = (colour: string): readonly [number, number, number] | null => {
  const match = /^#([0-9a-f]{6})$/i.exec(colour.trim())
  if (match === null) return null
  const value = Number.parseInt(match[1], 16)
  return [(value >> 16) & 255, (value >> 8) & 255, value & 255]
}

/** Blend two hex colours; falls back to the first when a token is not a plain hex value. */
const mix = (from: string, to: string, share: number): string => {
  const a = parseHex(from)
  const b = parseHex(to)
  if (a === null || b === null) return share < 0.5 ? from : to
  const channel = (index: number) => Math.round(a[index] + (b[index] - a[index]) * share)
  return `#${[channel(0), channel(1), channel(2)].map((value) => value.toString(16).padStart(2, '0')).join('')}`
}

/** Link colour from its strength: negative pulls toward `info`, positive toward `signal`, assumptions are `bone`. */
// Weak links anchor on `muted`, not the panel: a mix toward the background disappears on light
// themes, while muted is contrast-tuned against the panel in every theme.
const strengthColour = (strength: LagLinkStrength, scale: number, theme: ChartTheme): string => {
  switch (strength.kind) {
    case 'signed-unit':
    case 'signed-weight': {
      const share = Math.min(1, Math.abs(strength.value) / Math.max(scale, 1e-9))
      return mix(theme.muted, strength.value < 0 ? theme.info : theme.signal, share)
    }
    case 'nonnegative': return mix(theme.muted, theme.signal, Math.min(1, strength.value / Math.max(scale, 1e-9)))
    case 'assumption': return theme.bone
    default: return assertNever(strength)
  }
}

const symbolFor = (endpoint: LagEndpoint): string => {
  switch (endpoint) {
    case 'tail': return 'none'
    case 'arrow': return 'arrow'
    case 'circle': return 'emptyCircle'
    case 'conflict': return 'path://M-5,-3 L-3,-5 L0,-2 L3,-5 L5,-3 L2,0 L5,3 L3,5 L0,2 L-3,5 L-5,3 L-2,0 Z'
    case 'unresolved': return 'diamond'
    default: return assertNever(endpoint)
  }
}

const widthFor = (strength: LagLinkStrength, scale: number): number =>
  strength.kind === 'assumption' ? 1.6 : 1 + 2.2 * Math.min(1, strengthMagnitude(strength) / Math.max(scale, 1e-9))

/** A hidden cartesian grid the size of the drawing, so node x/y are pixels and nothing is auto-fitted or clipped. */
const pixelFrame = (width: number, height: number) => ({
  grid: { left: 0, top: 0, width, height, containLabel: false },
  xAxis: { type: 'value' as const, min: 0, max: width, show: false },
  yAxis: { type: 'value' as const, min: 0, max: height, inverse: true, show: false },
})

/** ECharts draws no label for a `symbol: 'none'` node, so text-only nodes are invisible one-pixel points. */
const labelAnchor = { symbol: 'circle', symbolSize: 1, itemStyle: { color: 'transparent', borderWidth: 0 }, silent: true } as const

const scaleOf = (strengths: readonly LagLinkStrength[]): number => {
  const signedUnit = strengths.some((strength) => strength.kind === 'signed-unit')
  if (signedUnit) return 1
  return Math.max(1e-9, ...strengths.map(strengthMagnitude))
}

export interface LagGridMetrics {
  readonly nodeSize: number
  readonly dx: number
  readonly dy: number
  readonly left: number
  readonly top: number
}

export const DEFAULT_LAG_GRID_METRICS: LagGridMetrics = { nodeSize: 22, dx: 104, dy: 78, left: 108, top: 44 }

/** How far a link bows off the straight line between its two nodes. A repeated self-link is drawn well
 *  clear of the row it sits on; everything else takes a slight bend so parallel links stay countable. */
const curvenessOf = (link: { readonly from: number; readonly to: number; readonly lag: number }): number =>
  link.from === link.to ? (link.lag <= 1 ? 0 : 0.45) : 0.18

/** The left gutter sized to the widest row label (11px labels average about 0.58em a character), clamped to 72–160 so one long name cannot push the grid off the strip. */
export const lagGridMetrics = (graph: LagGraph, base: LagGridMetrics = DEFAULT_LAG_GRID_METRICS): LagGridMetrics => {
  const widest = Math.max(0, ...graph.variables.map((variable) => variable.name.length))
  // A curve leaves its chord by about curveness × chord ÷ 2, and only a link touching the first row can
  // leave the frame at the top. Without this the arcs over the first row are drawn and then cut off.
  const overhang = Math.max(0, ...graph.links
    .filter((link) => link.from === 0 || link.to === 0)
    .map((link) => curvenessOf(link) * Math.hypot(link.lag * base.dx, Math.abs(link.to - link.from) * base.dy) / 2))
  return {
    ...base,
    left: Math.min(160, Math.max(72, Math.round(widest * 11 * 0.58) + base.nodeSize / 2 + 20)),
    top: base.top + Math.ceil(overhang),
  }
}

export const lagGridSize = (graph: LagGraph, metrics: LagGridMetrics = lagGridMetrics(graph)): { readonly width: number; readonly height: number } => ({
  width: metrics.left + graph.tauMax * metrics.dx + 60,
  height: metrics.top + (graph.variables.length - 1) * metrics.dy + 60,
})

/** Variables as rows, lag positions as columns, every link repeated across the window like tigramite's time series graph. */
export function lagGridOption(graph: LagGraph, theme: ChartTheme, metrics: LagGridMetrics = lagGridMetrics(graph)): EChartsCoreOption {
  const scale = scaleOf(graph.links.map((link) => link.strength))
  const positions = graph.tauMax + 1
  const centre = (variable: number, position: number) => [metrics.left + position * metrics.dx, metrics.top + variable * metrics.dy] as const
  const nodes = graph.variables.flatMap((variable, index) => Array.from({ length: positions }, (_, position) => {
    const [x, y] = centre(index, position)
    return {
      id: `n-${index}-${position}`,
      name: `${variable.name} at ${position === graph.tauMax ? 't' : `t−${graph.tauMax - position}`}`,
      value: [x, y],
      symbol: 'circle',
      symbolSize: metrics.nodeSize,
      itemStyle: { color: theme.panel, borderColor: variable.latent ? theme.faint : theme.muted, borderWidth: 1.4, borderType: variable.latent ? 'dashed' : 'solid' },
      label: { show: false },
    }
  }))
  const columnLabels = Array.from({ length: positions }, (_, position) => ({
    id: `c-${position}`,
    name: position === graph.tauMax ? 't' : `t−${graph.tauMax - position}`,
    value: [centre(0, position)[0], metrics.top - 26],
    ...labelAnchor,
    label: { show: true, position: 'inside', color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize },
    tooltip: { show: false },
  }))
  const rowLabels = graph.variables.map((variable, index) => ({
    id: `r-${index}`,
    name: variable.name,
    value: [metrics.left - metrics.nodeSize / 2 - 8, centre(index, 0)[1]],
    ...labelAnchor,
    label: { show: true, position: 'left', color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, width: metrics.left - metrics.nodeSize / 2 - 16, overflow: 'truncate' },
    // A truncated name is still readable on hover.
    tooltip: { formatter: escapeHtml(variable.name) },
  }))
  const links = graph.links.flatMap((link) => Array.from({ length: positions - link.lag }, (_, offset) => {
    const position = link.lag + offset
    return {
      source: `n-${link.from}-${position - link.lag}`,
      target: `n-${link.to}-${position}`,
      symbol: [symbolFor(link.fromEndpoint), symbolFor(link.toEndpoint)],
      symbolSize: [7, 9],
      lineStyle: {
        color: strengthColour(link.strength, scale, theme),
        width: widthFor(link.strength, scale),
        curveness: curvenessOf(link),
        opacity: 1,
        type: link.fromEndpoint === 'unresolved' || link.toEndpoint === 'unresolved' ? 'dashed' : 'solid',
      },
      value: strengthMagnitude(link.strength),
      hirmos: { from: graph.variables[link.from].name, to: graph.variables[link.to].name, lag: link.lag, strength: describeStrength(link.strength), mark: link.mark },
    }
  }))
  const size = lagGridSize(graph, metrics)
  const description = `Lag grid of ${graph.variables.length} variables over ${positions} time positions with ${graph.links.length} links; ${graph.semantics.replaceAll('-', ' ')}.`
  return {
    ...baseOption(theme, description),
    tooltip: {
      // The grid scrolls inside a strip, so the tooltip is not confined to the (possibly scrolled) chart rect.
      ...tooltip(theme, 'item', false),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const data = Reflect.get(raw, 'data')
        const own = data !== null && typeof data === 'object' ? Reflect.get(data, 'tooltip') : undefined
        if (own !== null && typeof own === 'object') {
          const formatter = Reflect.get(own, 'formatter')
          return typeof formatter === 'string' ? formatter : ''
        }
        const detail = data !== null && typeof data === 'object' ? Reflect.get(data, 'hirmos') : undefined
        if (detail === undefined || detail === null || typeof detail !== 'object') return ''
        const d = detail as { from: string; to: string; lag: number; strength: string; mark: string | null }
        return `${escapeHtml(d.from)}${d.lag === 0 ? '' : ` (t−${d.lag})`} ${escapeHtml(d.mark ?? '→')} ${escapeHtml(d.to)}<br/>${escapeHtml(d.strength)}`
      },
    },
    ...pixelFrame(size.width, size.height),
    series: [{
      type: 'graph',
      layout: 'none',
      coordinateSystem: 'cartesian2d',
      roam: false,
      data: [...nodes, ...columnLabels, ...rowLabels],
      links,
      edgeSymbolSize: [7, 9],
      lineStyle: { color: theme.muted },
      emphasis: { focus: 'adjacency', lineStyle: { width: 3 } },
      silent: false,
    }],
  }
}

export interface SummaryMetrics {
  readonly width: number
  readonly height: number
  readonly nodeSize: number
}

export const DEFAULT_SUMMARY_METRICS: SummaryMetrics = { width: 440, height: 360, nodeSize: 46 }

/** Variables on a circle (networkx `circular_layout`), one curved link per ordered pair, lags as the link label. */
export function summaryGraphOption(graph: SummaryGraph, theme: ChartTheme, metrics: SummaryMetrics = DEFAULT_SUMMARY_METRICS, highlighted: readonly string[] = []): EChartsCoreOption {
  const n = graph.variables.length
  const scale = scaleOf(graph.links.map((link) => link.strength))
  const cx = metrics.width / 2
  const cy = metrics.height / 2
  // The circle leaves room for the widest node label beside the extreme nodes and one label line above and below.
  const widest = Math.max(0, ...graph.variables.map((variable) => variable.name.length))
  const labelHalf = Math.min(cx * 0.45, Math.max(metrics.nodeSize / 2, Math.round(widest * theme.labelSize * 0.58) / 2))
  const rx = Math.max(48, cx - labelHalf - 6)
  const ry = Math.max(48, cy - metrics.nodeSize / 2 - (theme.labelSize * 1.4 + 10))
  const position = (k: number) => {
    const theta = (2 * Math.PI * k) / Math.max(n, 1)
    return [cx + rx * Math.cos(theta), cy - ry * Math.sin(theta)] as const
  }
  const autoScale = Math.max(1e-9, ...graph.autos.map((auto) => (auto === null ? 0 : strengthMagnitude(auto))))
  const nodes = graph.variables.map((variable, k) => {
    const [x, y] = position(k)
    const auto = graph.autos[k]
    const fill = auto === null ? theme.panel : mix(theme.panel, theme.signal, 0.1 + 0.6 * Math.min(1, strengthMagnitude(auto) / autoScale))
    const active = highlighted.includes(variable.id)
    const outside = y < cy - 1 ? 'top' : 'bottom'
    const labelWidth = Math.floor(2 * Math.min(x, metrics.width - x)) - 4
    return {
      id: `p-${k}`,
      name: variable.name,
      value: [x, y],
      symbol: 'circle',
      symbolSize: metrics.nodeSize,
      itemStyle: { color: fill, borderColor: active ? theme.signal : variable.latent ? theme.faint : theme.muted, borderWidth: active ? 2.4 : 1.4, borderType: variable.latent ? 'dashed' : 'solid' },
      label: { show: true, position: outside, distance: 6, color: active ? theme.ink : theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, fontWeight: active ? 600 : 400, width: labelWidth, overflow: 'truncate' },
      tooltip: { formatter: auto === null ? escapeHtml(variable.name) : `${escapeHtml(variable.name)}<br/>self-dependency ${escapeHtml(describeStrength(auto))}` },
    }
  })
  const links = graph.links.map((link) => ({
    source: `p-${link.from}`,
    target: `p-${link.to}`,
    symbol: [symbolFor(link.fromEndpoint), symbolFor(link.toEndpoint)],
    symbolSize: [8, 11],
    lineStyle: {
      color: strengthColour(link.strength, scale, theme),
      width: widthFor(link.strength, scale),
      curveness: 0.22,
      type: link.fromEndpoint === 'unresolved' || link.toEndpoint === 'unresolved' ? 'dashed' : 'solid',
    },
    label: {
      show: link.lags.length > 0,
      formatter: link.lags.join(','),
      color: theme.muted,
      fontFamily: theme.mono,
      fontSize: theme.labelSize,
      backgroundColor: theme.panel,
      padding: [1, 3],
      borderRadius: 3,
    },
    hirmos: { from: graph.variables[link.from].name, to: graph.variables[link.to].name, lags: link.lags, contemporaneous: link.contemporaneous, strength: describeStrength(link.strength), mark: link.mark },
  }))
  const description = `Summary graph of ${n} variables with ${graph.links.length} links; lag lists label the links; ${graph.semantics.replaceAll('-', ' ')}.`
  return {
    ...baseOption(theme, description),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const data = Reflect.get(raw, 'data')
        if (data === null || typeof data !== 'object') return ''
        const own = Reflect.get(data, 'tooltip')
        if (own !== null && typeof own === 'object') {
          const formatter = Reflect.get(own, 'formatter')
          return typeof formatter === 'string' ? formatter : ''
        }
        const detail = Reflect.get(data, 'hirmos')
        if (detail === undefined || detail === null || typeof detail !== 'object') return ''
        const d = detail as { from: string; to: string; lags: readonly number[]; contemporaneous: boolean; strength: string; mark: string | null }
        const when = [d.contemporaneous ? 'contemporaneous' : '', d.lags.length > 0 ? `lags ${d.lags.join(', ')}` : ''].filter((part) => part.length > 0).join(' · ')
        return `${escapeHtml(d.from)} ${escapeHtml(d.mark ?? '→')} ${escapeHtml(d.to)}<br/>${escapeHtml(when)}<br/>${escapeHtml(d.strength)}`
      },
    },
    ...pixelFrame(metrics.width, metrics.height),
    series: [{
      type: 'graph',
      layout: 'none',
      coordinateSystem: 'cartesian2d',
      roam: false,
      data: nodes,
      links,
      edgeSymbolSize: [8, 11],
      lineStyle: { color: theme.muted },
      emphasis: { focus: 'adjacency', lineStyle: { width: 3 } },
      // Edge labels stay horizontal instead of following the tangent; overlapping labels shift apart, then hide, and return on hover.
      labelLayout: (params: { readonly dataType?: string }) => (params.dataType === 'edge'
        ? { rotate: 0, hideOverlap: true, moveOverlap: 'shiftY' as const }
        : { hideOverlap: true, moveOverlap: 'shiftY' as const }),
    }],
  }
}
