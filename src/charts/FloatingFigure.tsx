import { useEffect, useId, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { Rnd } from 'react-rnd'
import { Icon } from '@/components/Icon'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useFloatingRect } from '@/lib/floatingRect'
import { useIsMobile } from '@/lib/useMediaQuery'
import { panelTitle } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'

/**
 * A figure that can be lifted into a floating window.
 *
 * A figure is read at two scales: a glance inside the panel, and a close look where one lag, node or
 * time point matters. The lifted window is draggable by its header and resizable, and carries no
 * backdrop, so the table it is being compared against stays visible and clickable. Its rect persists
 * per figure. The header takes a row of actions, exports as a rule, beside the close button.
 *
 * The window is portalled to the body because the workbench panes declare `container-type: size`,
 * which makes them the containing block for fixed-position descendants: rendered in place it would
 * be inset from the pane and clipped by its neighbours rather than floating over the window.
 */
export function FloatingFigure({
  label,
  defaultWidth = 900,
  defaultHeight = 560,
  actions,
  notice = null,
  figure,
  children,
}: {
  readonly label: string
  readonly defaultWidth?: number
  readonly defaultHeight?: number
  /** Controls for the window's header, left of the close button. */
  readonly actions?: ReactNode
  /** A line shown above the lifted figure, for a failed export. */
  readonly notice?: string | null
  /** The figure as drawn in the window, sized to fill it. */
  readonly figure: ReactNode
  /** The figure in place, given the button that opens the window, or null while it is open. */
  readonly children: (openButton: ReactNode) => ReactNode
}) {
  const [lifted, setLifted] = useState(false)
  const isMobile = useIsMobile()
  const layerId = `figure-${useId().replaceAll(':', '')}`
  const { position, size, onDragStop, onResizeStop } = useFloatingRect(
    `hirmos_panel_${label}`,
    () => ({
      x: Math.max(8, (window.innerWidth - defaultWidth) / 2),
      y: Math.max(16, (window.innerHeight - defaultHeight) / 2 - 16),
      width: defaultWidth,
      height: Math.min(defaultHeight, window.innerHeight - 32),
    }),
  )

  useEffect(() => (lifted ? pushLayer(layerId) : undefined), [layerId, lifted])
  useEffect(() => {
    if (!lifted) return undefined
    const key = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && escapeFor(layerId, event)) setLifted(false)
    }
    window.addEventListener('keydown', key)
    return () => window.removeEventListener('keydown', key)
  }, [layerId, lifted])

  const openButton = lifted ? null : (
    <button
      type="button"
      className="grid h-6 w-6 shrink-0 place-items-center rounded-md border border-hair bg-well text-faint transition-colors hover:border-edge hover:text-ink"
      aria-label={`Open ${label} in a floating window`}
      title="Open in a floating window"
      onClick={() => setLifted(true)}
    >
      <Icon name="open_in_full" size={13} />
    </button>
  )

  const header = (
    <div
      className={`${layerId}-drag flex shrink-0 select-none items-center justify-between gap-3 border-b border-hair px-3 py-2 ${isMobile ? '' : 'cursor-grab active:cursor-grabbing'}`}
    >
      <span className={cn(panelTitle, 'min-w-0 truncate text-nowrap')}>{label}</span>
      <div
        className="flex shrink-0 items-center gap-1.5"
        onPointerDown={(event) => event.stopPropagation()}
      >
        {actions}
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
      {notice !== null && (
        <p role="alert" className="mb-2 mt-0 text-body text-danger">
          {notice}
        </p>
      )}
      {figure}
    </div>
  )

  return (
    <>
      {children(openButton)}
      {lifted &&
        createPortal(
          // Drag and resize do not suit touch, so a phone gets a plain inset layer instead.
          isMobile ? (
            <div
              className="pop float fixed inset-3 z-(--z-dialog) flex flex-col overflow-hidden rounded-xl border border-edge bg-panel"
              role="dialog"
              aria-label={label}
            >
              {header}
              {body}
            </div>
          ) : (
            <Rnd
              size={size}
              position={position}
              onDragStop={(_, data) => onDragStop(data.x, data.y)}
              onResizeStop={(_, __, ref, ___, point) =>
                onResizeStop(ref.offsetWidth, ref.offsetHeight, point.x, point.y)
              }
              minWidth={Math.min(420, window.innerWidth - 16)}
              minHeight={260}
              maxWidth={window.innerWidth - 24}
              maxHeight={window.innerHeight - 24}
              bounds="window"
              dragHandleClassName={`${layerId}-drag`}
              enableResizing={{
                left: true,
                right: true,
                bottom: true,
                bottomLeft: true,
                bottomRight: true,
              }}
              className="z-(--z-panel)"
              style={{ position: 'fixed' }}
            >
              <div
                role="dialog"
                aria-label={label}
                className="pop float flex h-full flex-col overflow-hidden rounded-xl border border-edge bg-panel"
              >
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
