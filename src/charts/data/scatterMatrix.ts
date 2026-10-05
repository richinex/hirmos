import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, escapeHtml, tooltip } from '../grammar'
import { variableColour, type ChartTheme } from '../theme'

export interface MatrixColumn {
  readonly name: string
  readonly values: readonly number[]
}

/** Pandas scatter_matrix: observations off the diagonal, ten-bin histograms on it. */
export function scatterMatrixOption(
  columns: readonly MatrixColumn[],
  theme: ChartTheme,
): EChartsCoreOption {
  const count = columns.length
  const ranges = columns.map(({ values }) => {
    let low = Infinity,
      high = -Infinity
    for (const value of values) {
      low = Math.min(low, value)
      high = Math.max(high, value)
    }
    return low === high ? ([low - 0.5, high + 0.5] as const) : ([low, high] as const)
  })
  const cells = columns.flatMap((y, row) => columns.map((x, column) => ({ x, y, row, column })))
  const axis = {
    type: 'value' as const,
    axisLine: { lineStyle: { color: theme.hair } },
    axisTick: { show: false },
    splitLine: { show: false },
    axisLabel: {
      color: theme.muted,
      fontSize: 9,
      hideOverlap: true,
      formatter: (value: number) => formatStatistic('raw', value).text,
    },
    nameTextStyle: { color: theme.muted, fontSize: 10, fontFamily: theme.font },
  }
  return {
    ...baseOption(
      theme,
      `Scatter matrix of ${count} variables. Every observation is plotted; diagonal panels show histograms.`,
    ),
    animation: false,
    grid: cells.map(({ row, column }) => ({
      left: `${7 + (column * 90) / count}%`,
      top: `${2 + (row * 90) / count}%`,
      width: `${84 / count}%`,
      height: `${84 / count}%`,
    })),
    xAxis: cells.map(({ x, row, column }, index) => ({
      ...axis,
      gridIndex: index,
      min: ranges[column]![0],
      max: ranges[column]![1],
      name: row === count - 1 ? x.name : '',
      nameLocation: 'middle',
      nameGap: 26,
      axisLabel: { ...axis.axisLabel, show: row === count - 1 },
    })),
    yAxis: cells.map(({ y, row, column }, index) => ({
      ...axis,
      gridIndex: index,
      min: row === column ? 0 : ranges[row]![0],
      max: row === column ? undefined : ranges[row]![1],
      name: column === 0 ? y.name : '',
      nameLocation: 'middle',
      nameGap: 36,
      axisLabel: { ...axis.axisLabel, show: column === 0 && row !== column },
    })),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const cell = cells[Number(Reflect.get(raw, 'seriesIndex'))]
        const value: unknown = Reflect.get(raw, 'value')
        if (cell === undefined || !Array.isArray(value)) return ''
        return cell.row === cell.column
          ? `${escapeHtml(cell.x.name)}<br/>Bin centre: ${formatStatistic('raw', Number(value[0])).text}<br/>Observations: ${formatCount(Number(value[1])).text}`
          : `${escapeHtml(cell.x.name)}: ${formatStatistic('raw', Number(value[0])).text}<br/>${escapeHtml(cell.y.name)}: ${formatStatistic('raw', Number(value[1])).text}`
      },
    },
    series: cells.map(({ x, y, row, column }, index) => {
      const shared = {
        xAxisIndex: index,
        yAxisIndex: index,
        itemStyle: { color: variableColour(theme, y.name), opacity: 0.55 },
      }
      if (row !== column)
        return {
          ...shared,
          type: 'scatter',
          symbolSize: 2,
          large: true,
          largeThreshold: 2000,
          data: x.values.map((value, index) => [value, y.values[index]]),
        }
      const [low, high] = ranges[column]!
      const width = (high - low) / 10
      const bins = Array<number>(10).fill(0)
      for (const value of x.values)
        bins[Math.min(9, Math.max(0, Math.floor((value - low) / width)))]! += 1
      return {
        ...shared,
        type: 'bar',
        data: bins.map((size, bin) => [low + (bin + 0.5) * width, size]),
        barCategoryGap: 0,
      }
    }),
  }
}
