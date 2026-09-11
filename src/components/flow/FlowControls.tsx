import type { ReactNode } from 'react'
import { Panel, useReactFlow, type FitViewOptions } from '@xyflow/react'
import { Icon } from '@/components/Icon'
import { iconControl } from '@/components/ui/recipes'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'

/** One square button of the strip: borderless, so the strip's own rounded border clips it. */
export const flowControl = iconControl('quiet', 'rounded-none border-0')

/**
 * The control strip every React Flow canvas carries: zoom in, zoom out and fit, then whatever the
 * canvas adds after them. A column at the canvas's right on a desktop; on a phone a centred row
 * along its foot, where a column would sit over the cards. Rendered inside the ReactFlow element,
 * so it reads the canvas's own viewport.
 */
export function FlowControls({ fit, fitLabel = 'Fit the canvas', children }: {
  readonly fit: FitViewOptions
  readonly fitLabel?: string
  readonly children?: ReactNode
}) {
  const { fitView, zoomIn, zoomOut } = useReactFlow()
  const isMobile = useIsMobile()
  return (
    <Panel position={isMobile ? 'bottom-center' : 'bottom-right'} className="!m-2">
      <div className={cn('flex overflow-hidden rounded-lg border border-hair bg-panel/95 backdrop-blur', isMobile ? 'flex-row' : 'flex-col')} role="toolbar" aria-label="Canvas">
        <button type="button" className={flowControl} title="Zoom in" aria-label="Zoom in" onClick={() => void zoomIn({ duration: 160 })}><Icon name="add" size={14} /></button>
        <button type="button" className={flowControl} title="Zoom out" aria-label="Zoom out" onClick={() => void zoomOut({ duration: 160 })}><Icon name="remove" size={14} /></button>
        <button type="button" className={flowControl} title={fitLabel} aria-label={fitLabel} onClick={() => void fitView({ ...fit, duration: 220 })}><Icon name="fit_screen" size={14} /></button>
        {children}
      </div>
    </Panel>
  )
}
