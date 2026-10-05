import { canvasMotion } from '@/lib/motion'
import type { ReactNode } from 'react'
import { Panel, useReactFlow, type FitViewOptions } from '@xyflow/react'
import { ZoomButtons, zoomControl } from '@/components/ui/ZoomButtons'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'

/** One square button of the strip: borderless, so the strip's own rounded border clips it. */
export const flowControl = zoomControl

/**
 * The control strip every React Flow canvas carries: zoom in, zoom out and, unless the canvas fits
 * its view another way, fit; then whatever the canvas adds after them. A column at the canvas's right on a desktop; on a phone a centred row
 * along its foot, where a column would sit over the cards. The row takes its own width, since a panel
 * centred by `left: 50%` would otherwise shrink to half the canvas, and scrolls rather than wraps. Rendered inside the ReactFlow element,
 * so it reads the canvas's own viewport.
 */
export function FlowControls({
  fit,
  fitLabel = 'Fit the canvas',
  children,
}: {
  readonly fit?: FitViewOptions
  readonly fitLabel?: string
  readonly children?: ReactNode
}) {
  const { fitView, zoomIn, zoomOut } = useReactFlow()
  const isMobile = useIsMobile()
  return (
    <Panel
      position={isMobile ? 'bottom-center' : 'bottom-right'}
      className={isMobile ? '!mx-0 !my-2' : '!m-2'}
      style={
        isMobile
          ? { transform: 'translateX(-50%)', width: 'max-content', maxWidth: 'calc(100vw - 2rem)' }
          : undefined
      }
    >
      <div
        className={cn(
          'flex overflow-hidden rounded-lg border border-hair bg-panel/95 backdrop-blur',
          isMobile ? 'flex-row overflow-x-auto [scrollbar-width:none]' : 'flex-col',
        )}
        role="toolbar"
        aria-label="Canvas"
      >
        <ZoomButtons
          onIn={() => void zoomIn(canvasMotion('zoom'))}
          onOut={() => void zoomOut(canvasMotion('zoom'))}
          onFit={
            fit === undefined ? undefined : () => void fitView({ ...fit, ...canvasMotion('fit') })
          }
          fitLabel={fitLabel}
        />
        {children}
      </div>
    </Panel>
  )
}
