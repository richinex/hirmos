import { useMemo, useRef, useState, type ReactNode } from 'react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import { CHART_EXPORTS, exportChart, type ChartExport } from './export'
import { useChartExportContext } from './exportContext'
import { useChartTheme } from './theme'
import { downloadBlob } from '@/data/bundleFiles'
import { button } from '@/components/ui/recipes'

/**
 * Export buttons for the chart a floating window shows. Exports are of what is on screen: the
 * drawing at its size, the numbers at their zoomed range, so the lifted chart registers itself with
 * the option it was drawn from and the buttons read that pair when pressed.
 */
export function useChartExport(label: string, figure: string | null = null): {
  readonly register: (chart: EChartsType, option: EChartsCoreOption) => void
  readonly buttons: ReactNode
  readonly problem: string | null
} {
  const current = useRef<{ readonly chart: EChartsType; readonly option: EChartsCoreOption } | null>(null)
  const [problem, setProblem] = useState<string | null>(null)
  const theme = useChartTheme()
  const { project } = useChartExportContext()
  const background = theme.panel
  return useMemo(() => {
    const download = async (request: ChartExport) => {
      const drawn = current.current
      if (drawn === null) return
      setProblem(null)
      try {
        const file = await exportChart(drawn.chart, drawn.option, { project, figure, label }, request, background)
        downloadBlob(file.name, file.body instanceof Blob ? file.body : new Blob([file.body], { type: file.mediaType }))
      } catch {
        setProblem('The export could not be produced.')
      }
    }
    const buttons = (
      <>
        <span className="text-label text-faint">Export</span>
        {CHART_EXPORTS.map((request) => (
          <button key={request.kind} type="button" className={button('quiet', 'px-2 py-1 text-label')} aria-label={`Export ${label} as ${request.kind.toUpperCase()}`} onClick={() => void download(request)}>
            {request.kind.toUpperCase()}
          </button>
        ))}
      </>
    )
    return { register: (chart: EChartsType, option: EChartsCoreOption) => { current.current = { chart, option } }, buttons, problem }
  }, [background, figure, label, problem, project])
}
