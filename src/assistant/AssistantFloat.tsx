import { useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { Rnd, type DraggableData } from 'react-rnd'
import { useFloatingRect } from '@/lib/floatingRect'
import { label } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { dockAssistant } from './assistantStore'

/** Released this close to the right edge of the window, the assistant docks again. */
const DOCK_ZONE = 72
const WIDTH = 420
const HEIGHT = 620

export const ASSISTANT_DRAG = 'assistant-drag'

const pointerX = (event: MouseEvent | TouchEvent): number =>
  'touches' in event ? (event.changedTouches[0]?.clientX ?? 0) : event.clientX

/**
 * The assistant as a window over the workbench, dragged by its own header. While it is dragged, a
 * target along the right edge shows where it will dock; releasing over it docks the assistant.
 */
export function AssistantFloat({ children }: { readonly children: ReactNode }) {
  const [docking, setDocking] = useState(false)
  const { position, size, onDragStop, onResizeStop } = useFloatingRect(
    'hirmos_panel_assistant',
    () => ({
      x: window.innerWidth - WIDTH - 24,
      y: Math.max(16, window.innerHeight - HEIGHT - 56),
      width: WIDTH,
      height: Math.min(HEIGHT, window.innerHeight - 32),
    }),
  )
  const near = (event: MouseEvent | TouchEvent) => pointerX(event) > window.innerWidth - DOCK_ZONE
  return createPortal(
    <>
      {docking ? (
        <div
          aria-hidden
          className="pop fixed bottom-10 right-2 top-14 z-(--z-panel) grid w-[400px] place-items-center rounded-xl border-2 border-dashed border-signal/60 bg-signal/[0.06]"
        >
          <span className={label('text-signal-text')}>Release to dock</span>
        </div>
      ) : null}
      <Rnd
        size={size}
        position={position}
        onDrag={(event) => setDocking(near(event as MouseEvent | TouchEvent))}
        onDragStop={(event, data: DraggableData) => {
          setDocking(false)
          if (near(event as MouseEvent | TouchEvent)) dockAssistant()
          else onDragStop(data.x, data.y)
        }}
        onResizeStop={(_, __, ref, ___, point) =>
          onResizeStop(ref.offsetWidth, ref.offsetHeight, point.x, point.y)
        }
        minWidth={340}
        minHeight={320}
        maxWidth={window.innerWidth - 24}
        maxHeight={window.innerHeight - 24}
        bounds="window"
        dragHandleClassName={ASSISTANT_DRAG}
        cancel="button, input, textarea, a"
        enableResizing={{
          left: true,
          right: true,
          top: true,
          bottom: true,
          topLeft: true,
          bottomLeft: true,
          bottomRight: true,
        }}
        className="z-(--z-panel)"
        style={{ position: 'fixed' }}
      >
        <div
          className={cn(
            'pop float flex h-full flex-col overflow-hidden rounded-xl border border-edge bg-panel transition-opacity duration-(--motion-fast)',
            docking && 'opacity-70',
          )}
        >
          {children}
        </div>
      </Rnd>
    </>,
    document.body,
  )
}
