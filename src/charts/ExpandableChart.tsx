import { useEffect, useId, useRef, useState } from 'react'
import { createPortal } from 'react-dom'
import { Rnd } from 'react-rnd'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import type { VisibleWindow } from './window'
import { EChart } from './EChart'
import { CHART_EXPORTS, exportChart, type ChartExport } from './export'
import { useChartExportContext } from './exportContext'
import { useChartTheme } from './theme'
import { Icon } from '@/components/Icon'
import { downloadBlob } from '@/data/bundleFiles'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useFloatingRect } from '@/lib/floatingRect'
import { useIsMobile } from '@/lib/useMediaQuery'
import { button, panelTitle } from '@/components/ui/recipes'

/**
 * A diagnostic figure that can be lifted into a floating window.
 *
 * A correlogram or decomposition is read at two scales: a glance inside the panel, and a close look
 * where one lag or time point matters. The lifted window is draggable by its header and resizable,
 * and carries no backdrop, so the table it is being compared against stays visible and clickable.
 * Its rect persists per figure. Builders that declare `dataZoom` gain wheel zoom and drag panning,
 * and the window's header exports the drawing or its numbers as they stand, zoom and all.
 *
 * The window is portalled to the body because the workbench panes declare `container-type: size`,
 * which makes them the containing block for fixed-position descendants: rendered in place it would
 * be inset from the pane and clipped by its neighbours rather than floating over the window.
 */
export function ExpandableChart({ option, label, className = 'h-[260px]', testId, defaultWidth = 900, defaultHeight = 560, window: wanted, onWindow }: {
  readonly option: EChartsCoreOption
  readonly label: string
  readonly className?: string
  readonly testId?: string
  readonly defaultWidth?: number
  readonly defaultHeight?: number
  /** A window to show and the window shown, for charts that share an axis and follow each other. */
  readonly window?: VisibleWindow | null
  readonly onWindow?: (window: VisibleWindow | null) => void
}) {
  const [lifted, setLifted] = useState(false)
  const [exportProblem, setExportProblem] = useState<string | null>(null)
  const liftedChart = useRef<EChartsType | null>(null)
  const theme = useChartTheme()
  const { project } = useChartExportContext()
  const isMobile = useIsMobile()
  const layerId = `chart-${useId().replaceAll(':', '')}`
  const { position, size, onDragStop, onResizeStop } = useFloatingRect(`hirmos_panel_${label}`, () => ({
    x: Math.max(8, (window.innerWidth - defaultWidth) / 2),
    y: Math.max(16, (window.innerHeight - defaultHeight) / 2 - 16),
    width: defaultWidth,
    height: Math.min(defaultHeight, window.innerHeight - 32),
  }))

  useEffect(() => (lifted ? pushLayer(layerId) : undefined), [layerId, lifted])
  useEffect(() => {
    if (!lifted) return undefined
    const key = (event: KeyboardEvent) => { if (event.key === 'Escape' && escapeFor(layerId, event)) setLifted(false) }
    window.addEventListener('keydown', key)
    return () => window.removeEventListener('keydown', key)
  }, [layerId, lifted])

  const download = async (request: ChartExport) => {
    const chart = liftedChart.current
    if (chart === null) return
    setExportProblem(null)
    try {
      const file = await exportChart(chart, option, { project, label }, request, theme.panel)
      downloadBlob(file.name, file.body instanceof Blob ? file.body : new Blob([file.body], { type: file.mediaType }))
    } catch {
      setExportProblem('The export could not be produced.')
    }
  }

  const openButton = (
    <button
      type="button"
      className="absolute right-1 top-1 z-10 grid h-6 w-6 place-items-center rounded-md border border-hair bg-well text-faint transition-colors hover:border-edge hover:text-ink"
      aria-label={`Open ${label} in a floating window`}
      title="Open in a floating window"
      onClick={() => setLifted(true)}
    >
      <Icon name="open_in_full" size={13} />
    </button>
  )

  const header = (
    <div className={`${layerId}-drag flex shrink-0 select-none items-center justify-between gap-3 border-b border-hair px-3 py-2 ${isMobile ? '' : 'cursor-grab active:cursor-grabbing'}`}>
      <span className={`${panelTitle} min-w-0 truncate`}>{label}</span>
      <div className="flex shrink-0 items-center gap-1.5" onPointerDown={(event) => event.stopPropagation()}>
        {/* Exports are of what is on screen: the drawing at its size, the numbers at their zoomed range. */}
        <span className="text-label text-faint">Export</span>
        {CHART_EXPORTS.map((request) => (
          <button key={request.kind} type="button" className={button('quiet', 'px-2 py-1 text-label')} aria-label={`Export ${label} as ${request.kind.toUpperCase()}`} onClick={() => void download(request)}>
            {request.kind.toUpperCase()}
          </button>
        ))}
        <button
          type="button"
          className="ml-1 text-faint transition-colors hover:text-ink"
          aria-label="Close the floating window"
          title="Close (Esc)"
          onClick={() => setLifted(false)}
        >
          <Icon name="close_fullscreen" size={14} />
        </button>
      </div>
    </div>
  )

  const body = (
    <div className="flex min-h-0 flex-1 flex-col p-3">
      {exportProblem !== null && <p role="alert" className="mb-2 mt-0 text-body text-danger">{exportProblem}</p>}
      <EChart option={option} label={label} className="min-h-0 flex-1" onReady={(chart) => { liftedChart.current = chart }} window={wanted} onWindow={onWindow} />
    </div>
  )

  return (
    <>
      <div className="relative">
        {!lifted && openButton}
        <EChart option={option} label={label} className={className} testId={testId} window={wanted} onWindow={onWindow} />
      </div>
      {lifted && createPortal(
        // Drag and resize do not suit touch, so a phone gets a plain inset layer instead.
        isMobile ? (
          <div className="pop float fixed inset-3 z-(--z-dialog) flex flex-col overflow-hidden rounded-xl border border-edge bg-panel" role="dialog" aria-label={label}>
            {header}
            {body}
          </div>
        ) : (
          <Rnd
            size={size}
            position={position}
            onDragStop={(_, data) => onDragStop(data.x, data.y)}
            onResizeStop={(_, __, ref, ___, point) => onResizeStop(ref.offsetWidth, ref.offsetHeight, point.x, point.y)}
            minWidth={Math.min(420, window.innerWidth - 16)}
            minHeight={260}
            maxWidth={window.innerWidth - 24}
            maxHeight={window.innerHeight - 24}
            bounds="window"
            dragHandleClassName={`${layerId}-drag`}
            enableResizing={{ left: true, right: true, bottom: true, bottomLeft: true, bottomRight: true }}
            className="z-(--z-panel)"
            style={{ position: 'fixed' }}
          >
            <div role="dialog" aria-label={label} className="pop float flex h-full flex-col overflow-hidden rounded-xl border border-edge bg-panel">
              {header}
              {body}
            </div>
          </Rnd>
        ),
        document.body,
      )}
    </>
  )
}
