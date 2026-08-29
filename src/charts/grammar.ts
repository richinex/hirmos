import type { ChartTheme } from './theme'

/**
 * What every Hirmos chart shares: axis lines on `hair`, tick labels on `faint`, axis names on `muted`,
 * tooltips on the panel surface, no animation on evidence charts, and an aria description built from
 * the same view the chart draws.
 */

export const axisLabelStyle = (theme: ChartTheme) => ({
  color: theme.faint,
  fontFamily: theme.font,
  fontSize: theme.labelSize,
  hideOverlap: true,
})

/**
 * A small uniform margin. ECharts 6 grows the grid's outer bounds to fit axis labels and names, so
 * the plot adapts to the pane instead of sitting inside a fixed gutter; the clamp keeps a long label
 * from squeezing the plot below 60 per cent of the width or height.
 */
export const gridAuto = (extra: Record<string, unknown> = {}) => ({
  left: 8,
  right: 12,
  top: 12,
  bottom: 8,
  outerBoundsClampWidth: '40%',
  outerBoundsClampHeight: '40%',
  ...extra,
})

/**
 * The stage layout, then narrower layouts for the inspector: ECharts evaluates the queries against the
 * chart's own width and re-evaluates them on every resize. Later matching entries win.
 */
export const responsive = (
  base: Record<string, unknown>,
  variants: { readonly wide?: Record<string, unknown>; readonly narrow?: Record<string, unknown> },
) => ({
  baseOption: base,
  media: [
    ...(variants.wide === undefined ? [] : [{ query: { maxWidth: 480 }, option: variants.wide }]),
    ...(variants.narrow === undefined ? [] : [{ query: { maxWidth: 320 }, option: variants.narrow }]),
  ],
})

export const axisNameStyle = (theme: ChartTheme) => ({
  color: theme.muted,
  fontFamily: theme.font,
  fontSize: theme.labelSize,
})

export const valueAxis = (theme: ChartTheme, name?: string) => ({
  type: 'value' as const,
  scale: false,
  ...(name === undefined ? {} : { name, nameLocation: 'middle' as const, nameGap: 40, nameTextStyle: axisNameStyle(theme) }),
  axisLine: { show: false },
  axisTick: { show: false },
  axisLabel: axisLabelStyle(theme),
  splitLine: { lineStyle: { color: theme.hair } },
})

export const categoryAxis = (theme: ChartTheme, data: readonly string[], name?: string) => ({
  type: 'category' as const,
  data: [...data],
  ...(name === undefined ? {} : { name, nameLocation: 'middle' as const, nameGap: 28, nameTextStyle: axisNameStyle(theme) }),
  axisLine: { lineStyle: { color: theme.hair } },
  axisTick: { show: false },
  axisLabel: axisLabelStyle(theme),
})

export const tooltip = (theme: ChartTheme, trigger: 'item' | 'axis' = 'item', confine = true) => ({
  trigger,
  borderColor: theme.hair,
  backgroundColor: theme.panel,
  textStyle: { color: theme.ink, fontFamily: theme.font, fontSize: theme.bodySize },
  appendTo: 'body' as const,
  // Kept inside the chart's own rect, so a tooltip near the pane edge cannot escape the panel; charts in a scrolling strip pass false.
  confine,
})

/** A paging legend under the plot for charts with more than one named series; the grid must reserve its height itself. */
export const legend = (theme: ChartTheme, data?: readonly string[]) => ({
  type: 'scroll' as const,
  bottom: 0,
  itemWidth: 14,
  itemHeight: 8,
  itemGap: 10,
  icon: 'roundRect',
  textStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
  pageTextStyle: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize },
  pageIconColor: theme.muted,
  pageIconInactiveColor: theme.hair,
  ...(data === undefined ? {} : { data: [...data] }),
})

/** A diagonal hatch for the cells or bars that carry a verdict, so significance is not colour alone. */
export const hatch = { symbol: 'rect', dashArrayX: [1, 0], dashArrayY: [2, 4], rotation: Math.PI / 4, color: 'rgba(255,255,255,0.35)' } as const

/** Every evidence chart starts here: deterministic (no animation), described for assistive technology. */
export const baseOption = (theme: ChartTheme, description: string) => ({
  animation: false,
  useUTC: true,
  textStyle: { fontFamily: theme.font },
  // The host names the chart (aria-label) and describes it (aria-description) from this text; ECharts's own label writer is off so the two cannot compete for one attribute.
  aria: { enabled: true, label: { enabled: false, description } },
  backgroundColor: 'transparent',
})

export const escapeHtml = (value: string): string => value
  .replaceAll('&', '&amp;')
  .replaceAll('<', '&lt;')
  .replaceAll('>', '&gt;')
  .replaceAll('"', '&quot;')
  .replaceAll("'", '&#039;')
