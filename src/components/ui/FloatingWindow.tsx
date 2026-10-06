import { useEffect, useId, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { Rnd } from 'react-rnd'
import { Icon } from '@/components/Icon'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useFloatingRect } from '@/lib/floatingRect'
import { useIsMobile } from '@/lib/useMediaQuery'
import { panelTitle } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'

/**
 * A window over the workbench, draggable by its header and resizable from its edges, with no
 * backdrop, so the panels beside it stay visible and clickable. Its rect persists per label.
 *
 * The window is portalled to the body because the workbench panes declare `container-type: size`,
 * which makes them the containing block for fixed-position descendants: rendered in place it would
 * be inset from the pane and clipped by its neighbours rather than floating over the window.
 */
export function FloatingWindow({
  label,
  onClose,
  defaultWidth = 900,
  defaultHeight = 560,
  actions,
  children,
}: {
  readonly label: string
  readonly onClose: () => void
  readonly defaultWidth?: number
  readonly defaultHeight?: number
  /** Controls for the header, left of the close button. */
  readonly actions?: ReactNode
  readonly children: ReactNode
}) {
  const isMobile = useIsMobile()
  const layerId = `floating-${useId().replaceAll(':', '')}`
  const { position, size, onDragStop, onResizeStop } = useFloatingRect(
    `hirmos_panel_${label}`,
    () => ({
      x: Math.max(8, (window.innerWidth - defaultWidth) / 2),
      y: Math.max(16, (window.innerHeight - defaultHeight) / 2 - 16),
      width: defaultWidth,
      height: Math.min(defaultHeight, window.innerHeight - 32),
    }),
  )

  useEffect(() => pushLayer(layerId), [layerId])
  useEffect(() => {
    const key = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && escapeFor(layerId, event)) onClose()
    }
    window.addEventListener('keydown', key)
    return () => window.removeEventListener('keydown', key)
  }, [layerId, onClose])

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
          onClick={onClose}
        >
          <Icon name="close_fullscreen" size={14} />
        </button>
      </div>
    </div>
  )
  const body = <div className="flex min-h-0 flex-1 flex-col p-3">{children}</div>

  return createPortal(
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
  )
}
