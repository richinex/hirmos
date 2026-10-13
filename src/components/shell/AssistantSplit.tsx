import { lazy, Suspense, useLayoutEffect, useRef, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { AssistantFloat } from '@/assistant/AssistantFloat'
import { AssistantPane } from '@/assistant/AssistantLauncher'
import {
  useAssistantExpanded,
  useAssistantOpen,
  useAssistantPlacement,
} from '@/assistant/assistantStore'
import { useIsMobile } from '@/lib/useMediaQuery'

const AssistantDock = lazy(() => import('./AssistantDock'))
const AssistantSheet = lazy(() => import('./AssistantSheet'))

const hostElement = (className: string) => {
  const element = document.createElement('div')
  element.className = className
  return element
}

/** Places a host node in the layout. A host keeps its React tree while it moves between slots. */
export function Slot({ host }: { readonly host: HTMLElement }) {
  const ref = useRef<HTMLDivElement>(null)
  useLayoutEffect(() => {
    ref.current?.appendChild(host)
    return () => host.remove()
  }, [host])
  return <div ref={ref} className="flex min-h-0 min-w-0 flex-1" />
}

/**
 * The chapter and the assistant, so the screen the user asks about stays in view. The chapter and the
 * assistant each render once into a host node that moves between layouts, so opening, docking,
 * floating and the first load of the docked layout never reset either.
 */
export function AssistantSplit({ chapter }: { readonly chapter: ReactNode }) {
  const open = useAssistantOpen()
  const placement = useAssistantPlacement()
  const expanded = useAssistantExpanded()
  const phone = useIsMobile()
  const [chapterHost] = useState(() => hostElement('flex min-h-0 min-w-0 flex-1'))
  const [paneHost] = useState(() => hostElement('flex min-h-0 min-w-0 flex-1 flex-col'))
  const docked = open && placement === 'docked'
  const [dockLoaded, setDockLoaded] = useState(docked)
  if (docked && !dockLoaded) setDockLoaded(true)
  const plain = (
    <div className="flex min-w-0 flex-1">
      <Slot host={chapterHost} />
    </div>
  )
  return (
    <>
      {phone ? (
        open ? (
          <Suspense fallback={plain}>
            <AssistantSheet chapterHost={chapterHost} paneHost={paneHost} expanded={expanded} />
          </Suspense>
        ) : (
          plain
        )
      ) : (
        <>
          {dockLoaded ? (
            <Suspense fallback={plain}>
              <AssistantDock chapterHost={chapterHost} paneHost={paneHost} docked={docked} />
            </Suspense>
          ) : (
            plain
          )}
          {open && placement === 'floating' ? (
            <AssistantFloat>
              <Slot host={paneHost} />
            </AssistantFloat>
          ) : null}
        </>
      )}
      {createPortal(chapter, chapterHost)}
      {open ? createPortal(<AssistantPane />, paneHost) : null}
    </>
  )
}
