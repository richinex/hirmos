import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import { visibleWindow, type VisibleWindow } from './window'

/**
 * What a reader takes away from a chart: the drawing as it stands, or the numbers behind it. Every
 * export is of what is on screen, so a zoomed chart exports its zoomed range and nothing outside it.
 */
export type ChartExport = { readonly kind: 'svg' } | { readonly kind: 'png' } | { readonly kind: 'csv' }

export const CHART_EXPORTS: readonly ChartExport[] = [{ kind: 'svg' }, { kind: 'png' }, { kind: 'csv' }]

export interface ExportedFile {
  readonly name: string
  readonly mediaType: string
  readonly body: string | Blob
}

/** A table of the numbers a chart draws: a header row, then one row per x value or per matrix row. */
export interface SeriesTable {
  readonly columns: readonly string[]
  readonly rows: readonly (readonly (string | number | null)[])[]
}

type Series = Record<string, unknown>

const rootOf = (option: EChartsCoreOption): Record<string, unknown> => {
  const base = Reflect.get(option, 'baseOption')
  return (base !== null && typeof base === 'object' ? base : option) as Record<string, unknown>
}

const firstOf = (value: unknown): Record<string, unknown> | null => {
  const single = Array.isArray(value) ? value[0] : value
  return single !== null && typeof single === 'object' ? (single as Record<string, unknown>) : null
}

const labelsOf = (axis: Record<string, unknown> | null): readonly (string | number)[] | null =>
  axis !== null && Array.isArray(axis.data) ? axis.data.map((entry) => (typeof entry === 'number' ? entry : String(entry))) : null

/** The value a datum carries, whether bare or wrapped in an object with styling. */
const valueOf = (datum: unknown): unknown =>
  datum !== null && typeof datum === 'object' && !Array.isArray(datum) ? Reflect.get(datum, 'value') : datum

const numberOr = (value: unknown): number | null => (typeof value === 'number' && Number.isFinite(value) ? value : null)

/** The x and y of one datum: a bare y against the category at its index, or an [x, y] pair. */
const pointOf = (datum: unknown, index: number, categories: readonly (string | number)[] | null): readonly [string | number, number | null] => {
  const value = valueOf(datum)
  if (Array.isArray(value) && value.length >= 2) {
    const x = value[0]
    return [typeof x === 'number' || typeof x === 'string' ? x : String(x), numberOr(value[1])]
  }
  return [categories?.[index] ?? index + 1, numberOr(value)]
}

const seriesOf = (root: Record<string, unknown>): readonly Series[] =>
  (Array.isArray(root.series) ? root.series : [root.series])
    .filter((entry): entry is Series => entry !== null && typeof entry === 'object')
    // Helper series a builder silenced, such as the invisible floor of a band, are not numbers a reader asked for.
    .filter((entry) => entry.silent !== true && Array.isArray(entry.data))

const nameOf = (entry: Series, index: number): string => (typeof entry.name === 'string' && entry.name.length > 0 ? entry.name : `series ${index + 1}`)

/** A heatmap is a matrix: its rows are the y categories, top to bottom, its columns the x categories. */
const matrixTable = (root: Record<string, unknown>, entry: Series): SeriesTable => {
  const columns = labelsOf(firstOf(root.xAxis)) ?? []
  const rows = labelsOf(firstOf(root.yAxis)) ?? []
  const cells = new Map<string, number | null>()
  for (const datum of entry.data as readonly unknown[]) {
    const value = valueOf(datum)
    if (!Array.isArray(value) || value.length < 3) continue
    cells.set(`${String(value[0])}:${String(value[1])}`, numberOr(value[2]))
  }
  // The y axis lists categories bottom to top; a table reads top to bottom.
  const order = rows.map((_, index) => index).reverse()
  return {
    columns: ['', ...columns.map(String)],
    rows: order.map((y) => [rows[y], ...columns.map((_, x) => cells.get(`${x}:${y}`) ?? null)]),
  }
}

/** One column per series, one row per x value inside the window. */
const pointsTable = (root: Record<string, unknown>, series: readonly Series[], window: VisibleWindow | null): SeriesTable => {
  const xAxis = firstOf(root.xAxis)
  const categories = labelsOf(xAxis)
  // An axis name may carry a note after a separator; a column header keeps the word alone.
  const xName = xAxis !== null && typeof xAxis.name === 'string' && xAxis.name.length > 0 ? xAxis.name.split(' · ')[0] : 'x'
  const byX = new Map<string | number, (number | null)[]>()
  const order: (string | number)[] = []
  series.forEach((entry, column) => {
    (entry.data as readonly unknown[]).forEach((datum, index) => {
      const [x, y] = pointOf(datum, index, categories)
      const position = typeof x === 'number' ? x : index
      if (window !== null && (position < window.start || position > window.end)) return
      if (!byX.has(x)) { byX.set(x, Array.from({ length: series.length }, () => null)); order.push(x) }
      const row = byX.get(x)
      if (row !== undefined) row[column] = y
    })
  })
  return { columns: [xName, ...series.map(nameOf)], rows: order.map((x) => [x, ...(byX.get(x) ?? [])]) }
}

/** The table behind an option, in the shape its series call for. */
export function seriesTable(option: EChartsCoreOption, window: VisibleWindow | null): SeriesTable {
  const root = rootOf(option)
  const series = seriesOf(root)
  const heatmap = series.find((entry) => entry.type === 'heatmap')
  return heatmap !== undefined ? matrixTable(root, heatmap) : pointsTable(root, series, window)
}

const csvCell = (value: string | number | null): string => {
  if (value === null) return ''
  const text = typeof value === 'number' ? String(value) : value
  return /[",\n]/.test(text) ? `"${text.replaceAll('"', '""')}"` : text
}

export const csvOf = (table: SeriesTable): string =>
  [table.columns.map(csvCell).join(','), ...table.rows.map((row) => row.map(csvCell).join(','))].join('\n') + '\n'

/** What names an export: the project it belongs to, when one is open, and the chart's own label. */
export interface ExportName {
  readonly project: string | null
  readonly label: string
}

const slug = (text: string): string => text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')

/**
 * A file name a folder of downloads can still read: project, chart and the day, joined so each part
 * stays a word. `seat-belt-law--impact-path--2026-09-05.csv`.
 */
export const exportName = (name: ExportName, extension: string, day: Date = new Date()): string => {
  const parts = [name.project === null ? null : slug(name.project), slug(name.label) || 'chart', day.toISOString().slice(0, 10)]
  return `${parts.filter((part): part is string => part !== null && part.length > 0).join('--')}.${extension}`
}

/**
 * The drawing as it stands, serialised from the live SVG so every attribute is well-formed XML,
 * on the panel colour so a dark theme's chart is not lost on a white page.
 */
const svgOf = (chart: EChartsType, background: string): string => {
  const element = chart.getDom().querySelector('svg')
  if (element === null) throw new Error('the chart has no SVG to export')
  const markup = new XMLSerializer().serializeToString(element)
  return markup.replace(/<svg([^>]*)>/, `<svg$1><rect width="100%" height="100%" fill="${background}"/>`)
}

const pngOf = (svg: string, width: number, height: number, scale: number): Promise<Blob> => new Promise((resolve, reject) => {
  const image = new Image()
  const url = URL.createObjectURL(new Blob([svg], { type: 'image/svg+xml;charset=utf-8' }))
  image.onload = () => {
    const canvas = document.createElement('canvas')
    canvas.width = Math.round(width * scale)
    canvas.height = Math.round(height * scale)
    const context = canvas.getContext('2d')
    if (context === null) { URL.revokeObjectURL(url); reject(new Error('no 2d context')); return }
    context.scale(scale, scale)
    context.drawImage(image, 0, 0, width, height)
    URL.revokeObjectURL(url)
    canvas.toBlob((blob) => (blob === null ? reject(new Error('no image')) : resolve(blob)), 'image/png')
  }
  image.onerror = () => { URL.revokeObjectURL(url); reject(new Error('the drawing could not be rasterised')) }
  image.src = url
})

/** Produce the requested file from a live chart; what is on screen, at the range on screen. */
export async function exportChart(chart: EChartsType, option: EChartsCoreOption, name: ExportName, request: ChartExport, background: string): Promise<ExportedFile> {
  switch (request.kind) {
    case 'svg': return { name: exportName(name, 'svg'), mediaType: 'image/svg+xml', body: svgOf(chart, background) }
    case 'png': return { name: exportName(name, 'png'), mediaType: 'image/png', body: await pngOf(svgOf(chart, background), chart.getWidth(), chart.getHeight(), 2) }
    case 'csv': return { name: exportName(name, 'csv'), mediaType: 'text/csv', body: csvOf(seriesTable(option, visibleWindow(chart))) }
  }
}
