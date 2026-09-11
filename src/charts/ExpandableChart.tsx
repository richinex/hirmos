import { useEffect, useState } from 'react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import type { VisibleWindow } from './window'
import { EChart } from './EChart'
import { FloatingFigure } from './FloatingFigure'
import { useChartExport } from './useChartExport'

/**
 * A diagnostic chart that can be lifted into a floating window, where it fills the window and can be
 * exported as it stands. Builders that declare `dataZoom` gain wheel zoom and drag panning there.
 */
export function ExpandableChart({ option, label, className = 'h-[260px]', style, testId, defaultWidth, defaultHeight, window: wanted, onWindow }: {
  readonly option: EChartsCoreOption
  readonly label: string
  readonly className?: string
  /** An explicit pixel size for a chart whose height follows its rows, in the panel; the lifted copy fills its window. */
  readonly style?: React.CSSProperties
  /** The figure's short name: its test hook, and the stem of any file exported from it. */
  readonly testId?: string
  readonly defaultWidth?: number
  readonly defaultHeight?: number
  /** A window to show and the window shown, for charts that share an axis and follow each other. */
  readonly window?: VisibleWindow | null
  readonly onWindow?: (window: VisibleWindow | null) => void
}) {
  const exporter = useChartExport(label, testId ?? null)
  const [lifted, setLifted] = useState<EChartsType | null>(null)
  useEffect(() => { if (lifted !== null) exporter.register(lifted, option) }, [exporter, lifted, option])
  return (
    <FloatingFigure
      label={label}
      defaultWidth={defaultWidth}
      defaultHeight={defaultHeight}
      actions={exporter.buttons}
      notice={exporter.problem}
      figure={<EChart option={option} label={label} className="min-h-0 flex-1" onReady={setLifted} window={wanted} onWindow={onWindow} />}
    >
      {(openButton) => (
        <div className="relative">
          <div className="absolute right-1 top-1 z-10">{openButton}</div>
          <EChart option={option} label={label} className={className} style={style} testId={testId} window={wanted} onWindow={onWindow} />
        </div>
      )}
    </FloatingFigure>
  )
}
