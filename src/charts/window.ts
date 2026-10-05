import type { EChartsType } from 'echarts/core'

/**
 * The part of the x axis a chart is showing: a pair of axis values once a zoom is drawn, or the
 * whole axis. Figures beside a zoomable chart follow this window, so narrowing the chart narrows
 * the numbers with it.
 */
export interface VisibleWindow {
  readonly start: number
  readonly end: number
}

/** The first x-axis zoom's current range, or null when the chart shows everything. */
export const visibleWindow = (chart: EChartsType): VisibleWindow | null => {
  const option = chart.getOption() as {
    readonly dataZoom?: readonly {
      readonly startValue?: unknown
      readonly endValue?: unknown
      readonly start?: unknown
      readonly end?: unknown
    }[]
  }
  const zoom = option.dataZoom?.[0]
  if (zoom === undefined) return null
  const start = typeof zoom.startValue === 'number' ? zoom.startValue : null
  const end = typeof zoom.endValue === 'number' ? zoom.endValue : null
  if (start === null || end === null) return null
  // ECharts reports the whole axis as 0 to 100 percent; a range that reaches both ends is no window at all.
  if (zoom.start === 0 && zoom.end === 100) return null
  return { start, end }
}

export interface WindowSummary {
  readonly observed: number
  readonly missing: number
  readonly min: number
  readonly mean: number
  readonly max: number
}

/**
 * Summary of the values whose one-based step lies inside the window; the whole series when there is
 * none. NaN marks a missing cell and is counted, not averaged.
 */
export const summariseWindow = (
  values: Float64Array,
  window: VisibleWindow | null,
): WindowSummary => {
  const first = window === null ? 1 : Math.max(1, Math.ceil(window.start))
  const last = window === null ? values.length : Math.min(values.length, Math.floor(window.end))
  let observed = 0
  let missing = 0
  let min = Number.POSITIVE_INFINITY
  let max = Number.NEGATIVE_INFINITY
  let sum = 0
  for (let step = first; step <= last; step += 1) {
    const value = values[step - 1]
    if (Number.isNaN(value)) {
      missing += 1
      continue
    }
    observed += 1
    sum += value
    if (value < min) min = value
    if (value > max) max = value
  }
  return {
    observed,
    missing,
    min: observed === 0 ? Number.NaN : min,
    mean: observed === 0 ? Number.NaN : sum / observed,
    max: observed === 0 ? Number.NaN : max,
  }
}
