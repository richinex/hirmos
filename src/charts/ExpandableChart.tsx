import { useEffect, useId, useState } from 'react'
import { createPortal } from 'react-dom'
import { Rnd } from 'react-rnd'
import type { EChartsCoreOption } from 'echarts/core'
import { EChart } from './EChart'
import { Icon } from '@/components/Icon'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useFloatingRect } from '@/lib/floatingRect'
import { useIsMobile } from '@/lib/useMediaQuery'
import { panelTitle } from '@/components/ui/recipes'

/**
 * A diagnostic figure that can be lifted into a floating window.
 *
 * A correlogram or decomposition is read at two scales: a glance inside the panel, and a close look
 * where one lag or time point matters. The lifted window is draggable by its header and resizable,
 * and carries no backdrop, so the table it is being compared against stays visible and clickable.
 * Its rect persists per figure. Builders that declare `dataZoom` gain wheel zoom and drag panning.
 *
 * The window is portalled to the body because the workbench panes declare `container-type: size`,
 * which makes them the containing block for fixed-position descendants: rendered in place it would
 * be inset from the pane and clipped by its neighbours rather than floating over the window.
 */
export function ExpandableChart({ option, label, className = 'h-[260px]', testId, defaultWidth = 900, defaultHeight = 560 }: {
  readonly option: EChartsCoreOption
  readonly label: string
  readonly className?: string
  readonly testId?: string
  readonly defaultWidth?: number
  readonly defaultHeight?: number
}) {
  const [lifted, setLifted] = useState(false)
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
    <div className={`${layerId}-drag flex shrink-0 select-none items-center justify-between border-b border-hair px-3 py-2 ${isMobile ? '' : 'cursor-grab active:cursor-grabbing'}`}>
      <span className={panelTitle}>{label}</span>
      <button
        type="button"
        onPointerDown={(event) => event.stopPropagation()}
        className="text-faint transition-colors hover:text-ink"
        aria-label="Close the floating window"
        title="Close (Esc)"
        onClick={() => setLifted(false)}
      >
        <Icon name="close_fullscreen" size={14} />
      </button>
    </div>
  )

  const body = <div className="min-h-0 flex-1 p-3"><EChart option={option} label={label} className="h-full" /></div>

  return (
    <>
      <div className="relative">
        {!lifted && openButton}
        <EChart option={option} label={label} className={className} testId={testId} />
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
