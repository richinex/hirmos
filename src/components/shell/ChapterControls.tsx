import { useLayoutEffect, useState, type ReactNode } from 'react'
import { createPortal } from 'react-dom'

const SLOT = 'chapter-controls'

/** The place in the shell header where a full-stage chapter puts its own controls. */
export function ChapterControlsSlot() {
  return (
    <div
      id={SLOT}
      data-pane-neutral
      className="flex min-w-0 flex-1 items-center justify-end max-md:hidden"
    />
  )
}

export function ChapterControls({ children }: { readonly children: ReactNode }) {
  const [host, setHost] = useState<HTMLElement | null>(null)
  useLayoutEffect(() => setHost(document.getElementById(SLOT)), [])
  return host === null ? null : createPortal(children, host)
}
